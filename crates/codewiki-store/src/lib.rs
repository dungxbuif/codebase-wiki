//! Durable state and cache boundary for CodeWiki.

use codewiki_explore::{ExplorationSnapshot, promote_claims_from_snapshot};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Default CodeWiki config schema version.
pub const CODEWIKI_SCHEMA_VERSION: u32 = 1;
/// Current committed WikiPlan schema version.
pub const WIKIPLAN_SCHEMA_VERSION: u32 = 2;
/// Current reader-planning contract version.
pub const PLANNER_CONTRACT_VERSION: &str = "reader-first-v2";

/// First durable local-state migration SQL.
pub const INITIAL_STATE_MIGRATION_SQL: &str = include_str!("../migrations/001_initial_state.sql");

/// A versioned SQLite migration bundled with CodeWiki.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Migration {
    /// Monotonic migration version.
    pub version: u32,
    /// Stable human-readable migration name.
    pub name: &'static str,
    /// SQLite SQL body.
    pub sql: &'static str,
}

/// Return all bundled migrations in apply order.
pub fn available_migrations() -> &'static [Migration] {
    &[Migration {
        version: 1,
        name: "initial_state",
        sql: INITIAL_STATE_MIGRATION_SQL,
    }]
}

/// Return the latest bundled migration version.
pub fn latest_migration_version() -> u32 {
    available_migrations()
        .last()
        .map(|migration| migration.version)
        .unwrap_or(0)
}

/// Stable identity for a target repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryIdentity {
    /// Absolute or caller-stable repository root path.
    pub root_path: PathBuf,
    /// Optional Git remote URL.
    pub git_remote: Option<String>,
}

impl RepositoryIdentity {
    /// Create a repository identity from a root path and optional Git remote.
    pub fn new(root_path: impl Into<PathBuf>, git_remote: Option<String>) -> Self {
        Self {
            root_path: root_path.into(),
            git_remote,
        }
    }

    /// Return a stable key safe for use as a path component.
    pub fn storage_key(&self) -> String {
        let root_path = self.root_path.to_string_lossy();
        let source = match self.git_remote.as_deref() {
            Some(remote) if !remote.trim().is_empty() => remote.trim(),
            _ => root_path.as_ref(),
        };

        slug_with_hash(source)
    }
}

/// Local runtime paths for a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatePaths {
    /// Repository-specific base state directory.
    pub state_dir: PathBuf,
    /// Repository-specific SQLite database path.
    pub sqlite_path: PathBuf,
    /// Repository-specific rebuildable cache directory.
    pub cache_dir: PathBuf,
}

impl StatePaths {
    /// Resolve repository state/cache paths from explicit app-data and cache bases.
    pub fn resolve(
        app_data_base: impl AsRef<Path>,
        cache_base: impl AsRef<Path>,
        identity: &RepositoryIdentity,
    ) -> Self {
        let key = identity.storage_key();
        let state_dir = app_data_base.as_ref().join("codewiki").join(&key);
        let cache_dir = cache_base.as_ref().join("codewiki").join(&key);

        Self {
            sqlite_path: state_dir.join("state.sqlite3"),
            state_dir,
            cache_dir,
        }
    }

    /// Create the state and cache directories if they do not exist.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.state_dir)?;
        fs::create_dir_all(&self.cache_dir)
    }
}

/// Result of applying migrations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    /// SQLite database path migrated.
    pub sqlite_path: PathBuf,
    /// Latest migration version after application.
    pub latest_version: u32,
    /// Number of bundled migrations applied or verified.
    pub migrations_seen: usize,
}

/// Result of persisting semantic exploration state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceReport {
    /// Repository id used in SQLite.
    pub repository_id: String,
    /// Sync run id used for this persistence pass.
    pub run_id: String,
    /// Files persisted.
    pub files_seen: usize,
    /// Symbols persisted.
    pub symbols_seen: usize,
    /// Evidence items persisted.
    pub evidence_seen: usize,
    /// Claims persisted.
    pub claims_seen: usize,
    /// Claims marked stale because supporting evidence changed.
    pub stale_claims_seen: usize,
}

/// SQLite-backed Q&A context packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QaContext {
    /// Rendered Markdown context.
    pub markdown: String,
    /// Active claim rows included.
    pub active_claims_seen: usize,
    /// Stale claim rows included.
    pub stale_claims_seen: usize,
}

/// Apply all bundled migrations to a SQLite database through a local `sqlite3` executable.
pub fn apply_migrations_with_sqlite(
    sqlite_executable: impl AsRef<Path>,
    sqlite_path: impl AsRef<Path>,
) -> Result<MigrationReport, String> {
    let sqlite_path = sqlite_path.as_ref();
    if let Some(parent) = sqlite_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create SQLite state directory: {error}"))?;
    }

    for migration in available_migrations() {
        let sql = format!(
            "{}\nINSERT OR REPLACE INTO schema_migrations (version, name, checksum) VALUES ({}, '{}', '{}');\n",
            migration.sql,
            migration.version,
            sql_quote(migration.name),
            sql_quote(&migration_checksum(migration.sql)),
        );
        run_sqlite(sqlite_executable.as_ref(), sqlite_path, &sql)?;
    }

    Ok(MigrationReport {
        sqlite_path: sqlite_path.to_path_buf(),
        latest_version: latest_migration_version(),
        migrations_seen: available_migrations().len(),
    })
}

/// Persist a semantic exploration snapshot into the local SQLite state.
pub fn persist_exploration_with_sqlite(
    sqlite_executable: impl AsRef<Path>,
    sqlite_path: impl AsRef<Path>,
    identity: &RepositoryIdentity,
    mode: &str,
    snapshot: &ExplorationSnapshot,
) -> Result<PersistenceReport, String> {
    let sqlite_path = sqlite_path.as_ref();
    let repository_id = identity.storage_key();
    let run_id = format!(
        "run:{:016x}",
        fnv1a64(format!("{repository_id}:{mode}:{}", snapshot.files.len()).as_bytes())
    );
    let claims = promote_claims_from_snapshot(snapshot);
    let stale_claims_seen = count_stale_candidates(
        sqlite_executable.as_ref(),
        sqlite_path,
        &repository_id,
        snapshot,
    )?;
    let mut sql = String::new();
    sql.push_str("PRAGMA foreign_keys = ON;\nBEGIN;\n");
    sql.push_str(&format!(
        "INSERT INTO repositories (id, root_path, git_remote, updated_at) VALUES ('{}', '{}', {}, CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET root_path=excluded.root_path, git_remote=excluded.git_remote, updated_at=CURRENT_TIMESTAMP;\n",
        sql_quote(&repository_id),
        sql_quote(&identity.root_path.to_string_lossy()),
        sql_optional(identity.git_remote.as_deref()),
    ));
    sql.push_str(&format!(
        "INSERT OR REPLACE INTO sync_runs (id, repository_id, mode, status, finished_at, notes) VALUES ('{}', '{}', '{}', 'completed', CURRENT_TIMESTAMP, '{}');\n",
        sql_quote(&run_id),
        sql_quote(&repository_id),
        sql_quote(mode),
        sql_quote("semantic exploration persisted"),
    ));

    for file in &snapshot.files {
        let file_id = file_id(&repository_id, &file.path);
        sql.push_str(&format!(
            "UPDATE claims SET status='stale', updated_at=CURRENT_TIMESTAMP WHERE repository_id='{}' AND status='active' AND id IN (SELECT ce.claim_id FROM claim_evidence ce JOIN evidence_items ei ON ei.id = ce.evidence_id WHERE ei.repository_id='{}' AND ei.source_path='{}' AND ei.content_hash IS NOT NULL AND ei.content_hash <> '{}');\n",
            sql_quote(&repository_id),
            sql_quote(&repository_id),
            sql_quote(&file.path),
            sql_quote(&file.content_hash),
        ));
        sql.push_str(&format!(
            "INSERT INTO files (id, repository_id, path, content_hash, language, role, is_generated, last_seen_run_id, updated_at) VALUES ('{}', '{}', '{}', '{}', {}, '{}', 0, '{}', CURRENT_TIMESTAMP) ON CONFLICT(repository_id, path) DO UPDATE SET content_hash=excluded.content_hash, language=excluded.language, role=excluded.role, last_seen_run_id=excluded.last_seen_run_id, updated_at=CURRENT_TIMESTAMP;\n",
            sql_quote(&file_id),
            sql_quote(&repository_id),
            sql_quote(&file.path),
            sql_quote(&file.content_hash),
            sql_optional(file.language.as_deref()),
            sql_quote(file.role.as_str()),
            sql_quote(&run_id),
        ));

        for symbol in &file.symbols {
            let symbol_id = symbol_id(&repository_id, &file.path, &symbol.name, symbol.line);
            sql.push_str(&format!(
                "INSERT OR REPLACE INTO symbols (id, repository_id, file_id, name, kind, start_line, confidence) VALUES ('{}', '{}', '{}', '{}', '{}', {}, 'source-backed');\n",
                sql_quote(&symbol_id),
                sql_quote(&repository_id),
                sql_quote(&file_id),
                sql_quote(&symbol.name),
                sql_quote(&symbol.kind),
                symbol.line,
            ));
        }

        sql.push_str(&format!(
            "INSERT OR REPLACE INTO evidence_items (id, repository_id, kind, source_path, summary, content_hash, confidence, run_id) VALUES ('{}', '{}', 'file', '{}', '{}', '{}', 'source-backed', '{}');\n",
            sql_quote(&file.evidence_id),
            sql_quote(&repository_id),
            sql_quote(&file.path),
            sql_quote(&format!(
                "{} file with {} discovered symbols and {} import/dependency hints",
                file.role.as_str(),
                file.symbols.len(),
                file.imports.len()
            )),
            sql_quote(&file.content_hash),
            sql_quote(&run_id),
        ));
    }

    for claim in &claims {
        sql.push_str(&format!(
            "INSERT INTO claims (id, repository_id, statement, status, confidence, owner, first_seen_run_id, last_verified_run_id, updated_at) VALUES ('{}', '{}', '{}', 'active', '{}', 'ai', '{}', '{}', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET statement=excluded.statement, confidence=excluded.confidence, last_verified_run_id=excluded.last_verified_run_id, status=CASE WHEN claims.status='stale' THEN 'stale' ELSE excluded.status END, updated_at=CURRENT_TIMESTAMP;\n",
            sql_quote(&claim.id),
            sql_quote(&repository_id),
            sql_quote(&claim.statement),
            sql_quote(&claim.confidence),
            sql_quote(&run_id),
            sql_quote(&run_id),
        ));
        for evidence_id in &claim.evidence_ids {
            sql.push_str(&format!(
                "INSERT OR REPLACE INTO claim_evidence (claim_id, evidence_id, relationship) VALUES ('{}', '{}', 'supports');\n",
                sql_quote(&claim.id),
                sql_quote(evidence_id),
            ));
        }
    }

    sql.push_str("COMMIT;\n");
    run_sqlite(sqlite_executable.as_ref(), sqlite_path, &sql)?;

    Ok(PersistenceReport {
        repository_id,
        run_id,
        files_seen: snapshot.files.len(),
        symbols_seen: snapshot.files.iter().map(|file| file.symbols.len()).sum(),
        evidence_seen: snapshot.evidence.len(),
        claims_seen: claims.len(),
        stale_claims_seen,
    })
}

/// Render a SQLite-backed Q&A context packet for a query.
pub fn render_qa_context_with_sqlite(
    sqlite_executable: impl AsRef<Path>,
    sqlite_path: impl AsRef<Path>,
    query: &str,
    limit: usize,
) -> Result<QaContext, String> {
    let limit = limit.clamp(1, 50);
    let like = format!("%{}%", sql_like_escape(query));
    let sql = format!(
        "SELECT 'active|' || c.id || '|' || c.confidence || '|' || replace(c.statement, char(10), ' ') || '|' || IFNULL(group_concat(ce.evidence_id || '@' || IFNULL(ei.source_path,''), ','), '') FROM claims c LEFT JOIN claim_evidence ce ON ce.claim_id = c.id LEFT JOIN evidence_items ei ON ei.id = ce.evidence_id WHERE c.status='active' AND (c.statement LIKE '{}' ESCAPE '\\' OR IFNULL(ei.source_path,'') LIKE '{}' ESCAPE '\\') GROUP BY c.id ORDER BY c.updated_at DESC LIMIT {};\nSELECT 'stale|' || c.id || '|' || c.confidence || '|' || replace(c.statement, char(10), ' ') || '|' || IFNULL(group_concat(ce.evidence_id || '@' || IFNULL(ei.source_path,''), ','), '') FROM claims c LEFT JOIN claim_evidence ce ON ce.claim_id = c.id LEFT JOIN evidence_items ei ON ei.id = ce.evidence_id WHERE c.status='stale' AND (c.statement LIKE '{}' ESCAPE '\\' OR IFNULL(ei.source_path,'') LIKE '{}' ESCAPE '\\') GROUP BY c.id ORDER BY c.updated_at DESC LIMIT {};",
        sql_quote(&like),
        sql_quote(&like),
        limit,
        sql_quote(&like),
        sql_quote(&like),
        limit,
    );
    let stdout = run_sqlite_capture(sqlite_executable.as_ref(), sqlite_path.as_ref(), &sql)?;
    let mut active = Vec::new();
    let mut stale = Vec::new();
    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        let mut parts = line.splitn(5, '|');
        let status = parts.next().unwrap_or_default();
        let id = parts.next().unwrap_or_default();
        let confidence = parts.next().unwrap_or_default();
        let statement = parts.next().unwrap_or_default();
        let evidence = parts.next().unwrap_or_default();
        let rendered = render_qa_claim_line(id, confidence, statement, evidence);
        match status {
            "active" => active.push(rendered),
            "stale" => stale.push(rendered),
            _ => {}
        }
    }

    let mut markdown = format!("# CodeWiki Q&A Context\n\nQuery: `{}`\n\n", query);
    markdown.push_str("## Active Claims\n\n");
    if active.is_empty() {
        markdown.push_str("- none matched\n");
    } else {
        for row in &active {
            markdown.push_str(row);
        }
    }
    markdown.push_str("\n## Stale Claims\n\n");
    if stale.is_empty() {
        markdown.push_str("- none matched\n");
    } else {
        for row in &stale {
            markdown.push_str(row);
        }
    }

    Ok(QaContext {
        markdown,
        active_claims_seen: active.len(),
        stale_claims_seen: stale.len(),
    })
}

fn count_stale_candidates(
    sqlite_executable: &Path,
    sqlite_path: &Path,
    repository_id: &str,
    snapshot: &ExplorationSnapshot,
) -> Result<usize, String> {
    if !sqlite_path.exists() {
        return Ok(0);
    }
    let mut total = 0;
    for file in &snapshot.files {
        let sql = format!(
            "SELECT COUNT(DISTINCT ce.claim_id) FROM claim_evidence ce JOIN evidence_items ei ON ei.id = ce.evidence_id JOIN claims c ON c.id = ce.claim_id WHERE c.repository_id='{}' AND c.status='active' AND ei.repository_id='{}' AND ei.source_path='{}' AND ei.content_hash IS NOT NULL AND ei.content_hash <> '{}';",
            sql_quote(repository_id),
            sql_quote(repository_id),
            sql_quote(&file.path),
            sql_quote(&file.content_hash),
        );
        let stdout = run_sqlite_capture(sqlite_executable, sqlite_path, &sql)?;
        total += stdout.trim().parse::<usize>().unwrap_or(0);
    }
    Ok(total)
}

fn run_sqlite(sqlite_executable: &Path, sqlite_path: &Path, sql: &str) -> Result<(), String> {
    let mut child = Command::new(sqlite_executable)
        .arg(sqlite_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            format!(
                "failed to start sqlite executable `{}`: {error}",
                sqlite_executable.display()
            )
        })?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open sqlite stdin".to_string())?;
        stdin
            .write_all(sql.as_bytes())
            .map_err(|error| format!("failed to write sqlite SQL: {error}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|error| format!("failed to wait for sqlite process: {error}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "sqlite migration failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn run_sqlite_capture(
    sqlite_executable: &Path,
    sqlite_path: &Path,
    sql: &str,
) -> Result<String, String> {
    let output = Command::new(sqlite_executable)
        .arg(sqlite_path)
        .arg(sql)
        .output()
        .map_err(|error| {
            format!(
                "failed to start sqlite executable `{}`: {error}",
                sqlite_executable.display()
            )
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!(
            "sqlite query failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn slug_with_hash(source: &str) -> String {
    let mut slug = String::new();
    for ch in source.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    let slug = if slug.is_empty() { "repo" } else { slug };
    format!("{slug}-{:016x}", fnv1a64(source.as_bytes()))
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn migration_checksum(sql: &str) -> String {
    format!("{:016x}", fnv1a64(sql.as_bytes()))
}

fn sql_quote(value: &str) -> String {
    let mut quoted = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch == '\'' {
            quoted.push('\'');
            quoted.push('\'');
        } else {
            quoted.push(ch);
        }
    }
    quoted
}

fn sql_optional(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("'{}'", sql_quote(value)),
        None => "NULL".to_string(),
    }
}

fn file_id(repository_id: &str, path: &str) -> String {
    format!(
        "file-row:{:016x}",
        fnv1a64(format!("{repository_id}:{path}").as_bytes())
    )
}

fn symbol_id(repository_id: &str, path: &str, name: &str, line: usize) -> String {
    format!(
        "symbol:{:016x}",
        fnv1a64(format!("{repository_id}:{path}:{name}:{line}").as_bytes())
    )
}

fn sql_like_escape(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '\\' | '%' | '_' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            '\'' => {
                escaped.push('\'');
                escaped.push('\'');
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn render_qa_claim_line(id: &str, confidence: &str, statement: &str, evidence: &str) -> String {
    let mut line = format!("- `{id}` [{confidence}]: {statement}\n");
    for item in evidence.split(',').filter(|item| !item.trim().is_empty()) {
        let mut parts = item.splitn(2, '@');
        let evidence_id = parts.next().unwrap_or_default();
        let source_path = parts.next().unwrap_or_default();
        if source_path.is_empty() {
            line.push_str(&format!("  - evidence: `{evidence_id}`\n"));
        } else {
            line.push_str(&format!(
                "  - evidence: `{evidence_id}` from `{source_path}`\n"
            ));
        }
    }
    line
}

/// Planned storage layout for committed and local state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreLayout {
    /// Repo-committed project configuration path.
    pub committed_config_path: &'static str,
    /// Repo-committed plan summary path.
    pub committed_plan_path: &'static str,
    /// Repo-committed target-repository agent guidance path.
    pub committed_agents_path: &'static str,
    /// Committed source registry path.
    pub committed_sources_path: &'static str,
    /// Human-readable summary of local state storage.
    pub local_state_summary: &'static str,
    /// Human-readable summary of rebuildable cache storage.
    pub cache_summary: &'static str,
}

impl Default for StoreLayout {
    fn default() -> Self {
        Self {
            committed_config_path: ".agents/skills/codewiki/project/config.yml",
            committed_plan_path: ".agents/skills/codewiki/project/plan.yml",
            committed_agents_path: ".agents/skills/codewiki/project/AGENTS.md",
            committed_sources_path: ".agents/skills/codewiki/project/sources.yml",
            local_state_summary: "platform app data SQLite, keyed by repository identity",
            cache_summary: ".agents/skills/codewiki/project/cache is rebuildable and may be ignored",
        }
    }
}

/// Runtime tool activation policy for a target repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSelectionPolicy {
    /// Whether external tools are lazy-activated after docs/state/source are insufficient.
    pub lazy_activation: bool,
    /// First-choice code-intelligence provider when one is needed.
    pub code_intelligence_default: &'static str,
    /// Trigger for codebase-memory-mcp.
    pub memory_trigger: &'static str,
    /// Trigger for CocoIndex.
    pub indexing_trigger: &'static str,
}

impl Default for ToolSelectionPolicy {
    fn default() -> Self {
        Self {
            lazy_activation: true,
            code_intelligence_default: "octocode",
            memory_trigger: "shared cross-session memory beyond CodeWiki SQLite state",
            indexing_trigger: "repo scale or repeated refresh/query workload justifies an indexing pipeline",
        }
    }
}

/// Default committed CodeWiki configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeWikiConfig {
    /// CodeWiki config schema version.
    pub schema_version: u32,
    /// Preferred generated docs root.
    pub docs_root: &'static str,
    /// Preferred committed plan path.
    pub plan_path: &'static str,
    /// Preferred committed target-repo agent guidance path.
    pub agents_path: &'static str,
    /// Preferred committed source registry path.
    pub sources_path: &'static str,
    /// Runtime tool policy.
    pub tool_policy: ToolSelectionPolicy,
}

impl Default for CodeWikiConfig {
    fn default() -> Self {
        let layout = StoreLayout::default();

        Self {
            schema_version: CODEWIKI_SCHEMA_VERSION,
            docs_root: "docs",
            plan_path: layout.committed_plan_path,
            agents_path: layout.committed_agents_path,
            sources_path: layout.committed_sources_path,
            tool_policy: ToolSelectionPolicy::default(),
        }
    }
}

impl CodeWikiConfig {
    /// Render the default config as stable YAML.
    pub fn to_yaml(&self) -> String {
        format!(
            "schema_version: {}\ndocs_root: {}\nplan_path: {}\nagents_path: {}\nsources_path: {}\ntool_policy:\n  lazy_activation: {}\n  code_intelligence_default: {}\n  codebase_memory_mcp_trigger: {}\n  cocoindex_trigger: {}\n",
            self.schema_version,
            self.docs_root,
            self.plan_path,
            self.agents_path,
            self.sources_path,
            self.tool_policy.lazy_activation,
            self.tool_policy.code_intelligence_default,
            self.tool_policy.memory_trigger,
            self.tool_policy.indexing_trigger,
        )
    }
}

/// External or primary knowledge source tracked by a CodeWiki workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRecord {
    /// Source kind, e.g. `git`, `jira`, `figma`, `fix-note`, `local-docs`.
    pub kind: String,
    /// Stable human-readable source name.
    pub name: String,
    /// URL, path, issue key, or other source reference.
    pub reference: String,
}

impl SourceRecord {
    /// Create a source record.
    pub fn new(
        kind: impl Into<String>,
        name: impl Into<String>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            kind: kind.into(),
            name: name.into(),
            reference: reference.into(),
        }
    }
}

/// Render a source registry YAML document.
pub fn render_sources_yaml(primary: &SourceRecord, imports: &[SourceRecord]) -> String {
    let mut yaml = "schema_version: 1\nsources:\n".to_string();
    push_source(&mut yaml, primary, true);
    for source in imports {
        push_source(&mut yaml, source, false);
    }
    yaml
}

/// Render a single source item suitable for appending under `sources:`.
pub fn render_source_item_yaml(source: &SourceRecord, primary: bool) -> String {
    let mut yaml = String::new();
    push_source(&mut yaml, source, primary);
    yaml
}

fn push_source(yaml: &mut String, source: &SourceRecord, primary: bool) {
    yaml.push_str(&format!(
        "  - kind: {}\n    name: \"{}\"\n    ref: \"{}\"\n    primary: {}\n",
        yaml_atom(&source.kind),
        yaml_escape(&source.name),
        yaml_escape(&source.reference),
        primary,
    ));
}

/// Initial committed WikiPlan skeleton.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiPlan {
    /// Plan schema version.
    pub schema_version: u32,
    /// Current plan status.
    pub status: String,
    /// Evidence policy for generated docs.
    pub evidence_policy: String,
    /// Contract used to create and validate the page plan.
    pub planner_contract_version: String,
    /// Source revision captured for reproducibility.
    pub source_commit: String,
    /// Whether the source worktree was dirty at planning time.
    pub source_dirty: bool,
    /// Existing documentation visible to the planner.
    pub visible_docs: Vec<String>,
    /// Detected stack signals.
    pub detected: DetectedStack,
    /// Evidence-backed repository mental model created before page drafting.
    pub repository_mental_model: RepositoryMentalModel,
    /// Planned generated pages.
    pub pages: Vec<PlannedPage>,
    /// Open questions that affect future understanding.
    pub open_questions: Vec<String>,
    /// Stale claim IDs or summaries.
    pub stale_claims: Vec<String>,
}

impl Default for WikiPlan {
    fn default() -> Self {
        Self {
            schema_version: WIKIPLAN_SCHEMA_VERSION,
            status: "evidence_ready".to_string(),
            evidence_policy:
                "claims must cite files, symbols, commands, docs, or explicit hypotheses"
                    .to_string(),
            planner_contract_version: PLANNER_CONTRACT_VERSION.to_string(),
            source_commit: "unknown".to_string(),
            source_dirty: false,
            visible_docs: Vec::new(),
            detected: DetectedStack::default(),
            repository_mental_model: RepositoryMentalModel::default(),
            pages: PlannedPage::canonical_defaults(),
            open_questions: Vec::new(),
            stale_claims: Vec::new(),
        }
    }
}

impl WikiPlan {
    /// Create a plan from detected stack signals.
    pub fn from_detected(detected: DetectedStack) -> Self {
        Self {
            detected,
            ..Self::default()
        }
    }

    /// Attach immutable source provenance used by planning and benchmarks.
    pub fn with_provenance(
        mut self,
        source_commit: impl Into<String>,
        source_dirty: bool,
        visible_docs: Vec<String>,
    ) -> Self {
        self.source_commit = source_commit.into();
        self.source_dirty = source_dirty;
        self.visible_docs = visible_docs;
        self
    }

    /// Record a planning question that must survive into model enrichment.
    pub fn with_open_question(mut self, question: impl Into<String>) -> Self {
        self.open_questions.push(question.into());
        self
    }

    /// Render the plan skeleton as stable YAML.
    pub fn to_yaml(&self) -> String {
        let mut yaml = format!(
            "schema_version: {}\nstatus: {}\nplanner_contract_version: {}\nevidence_policy: {}\nconfidence_default: {}\nsource_commit: \"{}\"\nsource_dirty: {}\n",
            self.schema_version,
            self.status,
            self.planner_contract_version,
            self.evidence_policy,
            Confidence::SourceBacked.as_str(),
            yaml_escape(&self.source_commit),
            self.source_dirty,
        );
        push_yaml_list(&mut yaml, "visible_docs", &self.visible_docs, 0);
        yaml.push_str("detected:\n");
        push_yaml_list(&mut yaml, "languages", &self.detected.languages, 2);
        push_yaml_list(
            &mut yaml,
            "package_managers",
            &self.detected.package_managers,
            2,
        );
        push_yaml_list(&mut yaml, "frameworks", &self.detected.frameworks, 2);
        push_yaml_list(&mut yaml, "entrypoints", &self.detected.entrypoints, 2);
        push_yaml_list(&mut yaml, "tests", &self.detected.tests, 2);
        push_yaml_list(&mut yaml, "docs", &self.detected.docs, 2);
        yaml.push_str("repository_mental_model:\n");
        push_yaml_list(
            &mut yaml,
            "systems",
            &self.repository_mental_model.systems,
            2,
        );
        push_yaml_list(&mut yaml, "actors", &self.repository_mental_model.actors, 2);
        push_yaml_list(
            &mut yaml,
            "boundaries",
            &self.repository_mental_model.boundaries,
            2,
        );
        push_yaml_list(
            &mut yaml,
            "runtimes",
            &self.repository_mental_model.runtimes,
            2,
        );
        push_yaml_list(
            &mut yaml,
            "workflows",
            &self.repository_mental_model.workflows,
            2,
        );
        push_yaml_list(
            &mut yaml,
            "state_ownership",
            &self.repository_mental_model.state_ownership,
            2,
        );
        push_yaml_list(
            &mut yaml,
            "integrations",
            &self.repository_mental_model.integrations,
            2,
        );
        push_yaml_list(
            &mut yaml,
            "change_risks",
            &self.repository_mental_model.change_risks,
            2,
        );
        push_yaml_list(
            &mut yaml,
            "known_unknowns",
            &self.repository_mental_model.known_unknowns,
            2,
        );
        yaml.push_str("pages:\n");
        for page in &self.pages {
            yaml.push_str(&format!(
                "  - path: {}\n    title: \"{}\"\n    page_type: {}\n    section_id: {}\n    parent_page: {}\n    order: {}\n    importance: {}\n    slot: {}\n    status: {}\n    confidence: {}\n    reader_job: \"{}\"\n    scope: \"{}\"\n    out_of_scope: \"{}\"\n",
                page.path,
                yaml_escape(&page.title),
                page.page_type,
                page.section_id,
                page.parent_page.as_deref().unwrap_or("null"),
                page.order,
                page.importance,
                page.slot,
                page.status,
                page.confidence.as_str(),
                yaml_escape(&page.reader_job),
                yaml_escape(&page.scope),
                yaml_escape(&page.out_of_scope),
            ));
            push_yaml_list(&mut yaml, "audiences", &page.audiences, 4);
            push_yaml_list(&mut yaml, "prerequisites", &page.prerequisites, 4);
            push_yaml_list(&mut yaml, "reader_questions", &page.reader_questions, 4);
            push_yaml_list(&mut yaml, "required_sections", &page.required_sections, 4);
            yaml.push_str("    diagram_slots:\n");
            if page.diagram_slots.is_empty() {
                yaml.push_str("      []\n");
            } else {
                for slot in &page.diagram_slots {
                    yaml.push_str(&format!(
                        "      - kind: {}\n        question: \"{}\"\n",
                        slot.kind,
                        yaml_escape(&slot.question)
                    ));
                }
            }
            push_yaml_list(&mut yaml, "topic_ids", &page.topic_ids, 4);
            yaml.push_str("    source_anchors:\n");
            if page.source_anchors.is_empty() {
                yaml.push_str("      []\n");
            } else {
                for anchor in &page.source_anchors {
                    yaml.push_str(&format!(
                        "      - selector: \"{}\"\n        reason: \"{}\"\n",
                        yaml_escape(&anchor.selector),
                        yaml_escape(&anchor.reason)
                    ));
                    push_yaml_list(&mut yaml, "expected_claims", &anchor.expected_claims, 8);
                }
            }
            push_yaml_list(&mut yaml, "evidence_gaps", &page.evidence_gaps, 4);
            push_yaml_list(&mut yaml, "related_pages", &page.related_pages, 4);
            push_yaml_list(&mut yaml, "open_questions", &page.open_questions, 4);
            push_yaml_list(&mut yaml, "refresh_triggers", &page.refresh_triggers, 4);
            push_yaml_list(&mut yaml, "acceptance_checks", &page.acceptance_checks, 4);
        }
        push_yaml_list(&mut yaml, "open_questions", &self.open_questions, 0);
        push_yaml_list(&mut yaml, "stale_claims", &self.stale_claims, 0);
        yaml
    }
}

/// Confidence label for claims, pages, and evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    /// Directly supported by authoritative evidence.
    Confirmed,
    /// Supported by source evidence but not independently confirmed.
    SourceBacked,
    /// Plausible but incomplete; must not be presented as fact.
    Hypothesis,
    /// Weak signal worth revisiting.
    Watchlist,
}

impl Confidence {
    /// Stable serialized label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::SourceBacked => "source-backed",
            Self::Hypothesis => "hypothesis",
            Self::Watchlist => "watchlist",
        }
    }
}

/// Evidence kind for durable source references.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    /// Source file evidence.
    File,
    /// Symbol evidence.
    Symbol,
    /// Command output summary.
    Command,
    /// Existing documentation evidence.
    Documentation,
    /// Git history evidence.
    Git,
    /// Optional provider evidence.
    Provider,
    /// Explicit hypothesis evidence marker.
    Hypothesis,
}

impl EvidenceKind {
    /// Stable serialized label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Command => "command",
            Self::Documentation => "documentation",
            Self::Git => "git",
            Self::Provider => "provider",
            Self::Hypothesis => "hypothesis",
        }
    }
}

/// Durable evidence item model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceItem {
    /// Evidence kind.
    pub kind: EvidenceKind,
    /// Source path when applicable.
    pub source_path: Option<String>,
    /// Human-readable evidence summary.
    pub summary: String,
    /// Evidence confidence.
    pub confidence: Confidence,
}

/// Durable claim model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    /// Claim statement.
    pub statement: String,
    /// Claim confidence.
    pub confidence: Confidence,
    /// Evidence items supporting or qualifying the claim.
    pub evidence: Vec<EvidenceItem>,
}

/// Detected repository stack signals stored in the WikiPlan.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DetectedStack {
    /// Detected languages.
    pub languages: Vec<String>,
    /// Detected package managers/build tools.
    pub package_managers: Vec<String>,
    /// Detected frameworks/libraries.
    pub frameworks: Vec<String>,
    /// Detected entrypoint files.
    pub entrypoints: Vec<String>,
    /// Detected test files/dirs.
    pub tests: Vec<String>,
    /// Detected docs files/dirs.
    pub docs: Vec<String>,
}

/// Evidence-backed system model that must precede page architecture.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepositoryMentalModel {
    /// Major systems or products.
    pub systems: Vec<String>,
    /// Human or system actors.
    pub actors: Vec<String>,
    /// Responsibility and dependency boundaries.
    pub boundaries: Vec<String>,
    /// Runtime and executor contexts.
    pub runtimes: Vec<String>,
    /// Important end-to-end workflows.
    pub workflows: Vec<String>,
    /// Persistent and in-memory state owners.
    pub state_ownership: Vec<String>,
    /// External systems and platform integrations.
    pub integrations: Vec<String>,
    /// High-risk change surfaces.
    pub change_risks: Vec<String>,
    /// Important unresolved questions.
    pub known_unknowns: Vec<String>,
}

/// A question-driven diagram requirement for a page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramSlot {
    /// Diagram kind such as component, sequence, state, flowchart, or ERD.
    pub kind: String,
    /// Reader question answered by the diagram.
    pub question: String,
}

/// A selected evidence source with an explicit relevance reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceAnchor {
    /// File, symbol, command, documentation, Git range, or hypothesis selector.
    pub selector: String,
    /// Why the source is necessary for this page.
    pub reason: String,
    /// Claims expected from this evidence source.
    pub expected_claims: Vec<String>,
}

/// Planned generated page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedPage {
    /// Generated page path.
    pub path: String,
    /// Human-readable title.
    pub title: String,
    /// Semantic page type.
    pub page_type: String,
    /// Owning semantic section.
    pub section_id: String,
    /// Parent reader page, when any.
    pub parent_page: Option<String>,
    /// Stable reading order within the section.
    pub order: u32,
    /// Reader importance.
    pub importance: String,
    /// Canonical page slot.
    pub slot: String,
    /// Intended audiences.
    pub audiences: Vec<String>,
    /// Unique job this page performs for its reader.
    pub reader_job: String,
    /// Prerequisite pages.
    pub prerequisites: Vec<String>,
    /// Questions the page must answer directly.
    pub reader_questions: Vec<String>,
    /// Semantic scope.
    pub scope: String,
    /// Explicit non-scope.
    pub out_of_scope: String,
    /// Required reader-facing sections.
    pub required_sections: Vec<String>,
    /// Question-driven diagram slots.
    pub diagram_slots: Vec<DiagramSlot>,
    /// Canonical concept ownership identifiers.
    pub topic_ids: Vec<String>,
    /// Selected evidence with relevance reasons.
    pub source_anchors: Vec<SourceAnchor>,
    /// Known missing evidence.
    pub evidence_gaps: Vec<String>,
    /// Lateral related pages.
    pub related_pages: Vec<String>,
    /// Page-local unresolved questions.
    pub open_questions: Vec<String>,
    /// Events that make this page stale.
    pub refresh_triggers: Vec<String>,
    /// Observable page acceptance checks.
    pub acceptance_checks: Vec<String>,
    /// Planning status.
    pub status: String,
    /// Page confidence.
    pub confidence: Confidence,
}

impl PlannedPage {
    /// Return canonical default page plan.
    pub fn canonical_defaults() -> Vec<Self> {
        [
            ("docs/QUICKSTART.md", "CodeWiki Quickstart", "quickstart", "overview"),
            (
                "docs/conventions/OVERVIEW.md",
                "Code Conventions",
                "conventions",
                "reference",
            ),
            ("docs/evidence/CLAIMS.md", "Claims", "evidence.claims", "evidence"),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (path, title, slot, page_type))| Self {
            path: path.to_string(),
            title: title.to_string(),
            page_type: page_type.to_string(),
            section_id: slot.split('.').next().unwrap_or(slot).to_string(),
            parent_page: None,
            order: (index as u32 + 1) * 10,
            importance: if slot == "quickstart" { "critical" } else { "supporting" }.to_string(),
            slot: slot.to_string(),
            audiences: vec!["new_developer".to_string(), "maintainer".to_string()],
            reader_job: match slot {
                "quickstart" => "Form a five-minute system mental model and choose a task-oriented reading path.",
                "conventions" => "Change the repository using its evidence-backed conventions and exceptions.",
                _ => "Audit the evidence supporting generated documentation claims.",
            }
            .to_string(),
            prerequisites: Vec::new(),
            reader_questions: vec![match slot {
                "quickstart" => "What does this repository do and where should a new developer start?",
                "conventions" => "Which repository-specific rules and exceptions govern a safe change?",
                _ => "Which evidence supports the reader-facing claims?",
            }
            .to_string()],
            scope: "Evidence-backed reader guidance for this canonical page.".to_string(),
            out_of_scope: "Raw exhaustive file, symbol, or lexical-import inventory.".to_string(),
            required_sections: vec![
                "purpose".to_string(),
                "mental_model".to_string(),
                "change_guide".to_string(),
                "evidence".to_string(),
            ],
            diagram_slots: Vec::new(),
            topic_ids: vec![slot.to_string()],
            source_anchors: vec![SourceAnchor {
                selector: "pending-llm-selection".to_string(),
                reason: "The LLM planner must replace this scaffold with relevant evidence and a reason.".to_string(),
                expected_claims: Vec::new(),
            }],
            evidence_gaps: vec!["llm_semantic_planning_pending".to_string()],
            related_pages: Vec::new(),
            open_questions: Vec::new(),
            refresh_triggers: vec!["supporting_file_changed".to_string()],
            acceptance_checks: vec![
                "Every reader question has a direct answer and claim-local evidence.".to_string(),
                "The page names a safe change starting point, risks, and verification path.".to_string(),
            ],
            status: "planned".to_string(),
            confidence: Confidence::SourceBacked,
        })
        .collect()
    }
}

/// Render target-repository CodeWiki agent guidance.
pub fn render_target_agents_md() -> String {
    [
        "# CodeWiki Local Agent Guidance",
        "",
        "A companion `init` or `sync` refreshes evidence only. It leaves `run.yml` at `synthesis_incomplete`; do not present the workspace as onboarding-ready yet.",
        "",
        "Complete the reader-first synthesis workflow:",
        "",
        "1. Build and record the repository mental model from source-backed evidence.",
        "2. Replace the scaffold with a complete WikiPlan v2 whose pages name reader jobs, questions, source anchors, diagrams, and acceptance checks.",
        "3. Synthesize reader docs in purpose -> mental model -> flow -> change guidance -> evidence order. Do not copy evidence inventories into reader pages.",
        "4. Run isolated contract, source, diagram, cross-page, and docs-only onboarding reviews; record results in `quality-report.yml`.",
        "5. Run `codewiki validate <workspace>` and require `generation_status: reader_docs_ready` before claiming completion.",
        "",
        "For normal code changes after synthesis, use docs-first lazy activation:",
        "",
        "1. Read the relevant reader docs and WikiPlan page contract.",
        "2. Query local CodeWiki SQLite facts/evidence/claims when available.",
        "3. Inspect source files and Git history when docs are missing or stale.",
        "4. Activate external runtime tools only when evidence quality requires it.",
        "",
        "Before changing code, read `docs/conventions/OVERVIEW.md` and verify that cited convention evidence is current for the affected area.",
        "",
        "During CodeWiki sync, current docs are durable user input. Preserve manual edits inside or outside generated regions and semantically merge them with refreshed source evidence; never replace a conflict with a whole regenerated page.",
        "",
        "Runtime tool policy:",
        "",
        "- Octocode is the first-choice code-intelligence provider when a provider is needed.",
        "- codebase-memory-mcp is only for shared cross-session memory beyond CodeWiki SQLite state.",
        "- CocoIndex is only for repo scale or repeated refresh/query indexing workloads.",
        "",
        "Record selected tools and trigger reasons in `.agents/skills/codewiki/project/config.yml` and local runtime state.",
        "",
    ]
    .join("\n")
}

fn push_yaml_list(yaml: &mut String, key: &str, items: &[String], indent: usize) {
    let prefix = " ".repeat(indent);
    yaml.push_str(&format!("{prefix}{key}:\n"));
    if items.is_empty() {
        yaml.push_str(&format!("{prefix}  []\n"));
        return;
    }
    for item in items {
        yaml.push_str(&format!("{prefix}  - \"{}\"\n", yaml_escape(item)));
    }
}

fn yaml_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn yaml_atom(value: &str) -> String {
    let escaped = yaml_escape(value);
    if escaped
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        escaped
    } else {
        format!("\"{escaped}\"")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_layout_includes_committed_paths() {
        let layout = StoreLayout::default();

        assert_eq!(
            layout.committed_config_path,
            ".agents/skills/codewiki/project/config.yml"
        );
        assert_eq!(
            layout.committed_plan_path,
            ".agents/skills/codewiki/project/plan.yml"
        );
        assert_eq!(
            layout.committed_agents_path,
            ".agents/skills/codewiki/project/AGENTS.md"
        );
        assert_eq!(
            layout.committed_sources_path,
            ".agents/skills/codewiki/project/sources.yml"
        );
    }

    #[test]
    fn config_yaml_records_lazy_provider_policy() {
        let yaml = CodeWikiConfig::default().to_yaml();

        assert!(yaml.contains("lazy_activation: true"));
        assert!(yaml.contains("sources_path: .agents/skills/codewiki/project/sources.yml"));
        assert!(yaml.contains("code_intelligence_default: octocode"));
        assert!(yaml.contains("codebase_memory_mcp_trigger: shared cross-session memory"));
        assert!(yaml.contains("cocoindex_trigger: repo scale"));
    }

    #[test]
    fn plan_yaml_records_evidence_policy() {
        let yaml = WikiPlan::default().to_yaml();

        assert!(yaml.contains("schema_version: 2"));
        assert!(yaml.contains("claims must cite files"));
        assert!(yaml.contains("repository_mental_model:"));
        assert!(yaml.contains("reader_questions:"));
        assert!(yaml.contains("source_anchors:"));
        assert!(yaml.contains("acceptance_checks:"));
        assert!(yaml.contains("docs/QUICKSTART.md"));
        assert!(yaml.contains("docs/conventions/OVERVIEW.md"));
        assert!(!yaml.contains("docs/quickstart.md"));
        assert!(yaml.contains("confidence_default: source-backed"));
    }

    #[test]
    fn target_agents_guidance_is_docs_first() {
        let guidance = render_target_agents_md();

        assert!(guidance.contains("docs-first lazy activation"));
        assert!(guidance.contains("Octocode is the first-choice"));
        assert!(guidance.contains("CocoIndex is only"));
    }

    #[test]
    fn migration_registry_is_ordered() {
        let migrations = available_migrations();

        assert_eq!(latest_migration_version(), 1);
        assert_eq!(migrations[0].version, 1);
        assert_eq!(migrations[0].name, "initial_state");
    }

    #[test]
    fn initial_migration_contains_core_tables() {
        let sql = INITIAL_STATE_MIGRATION_SQL;

        for table in [
            "schema_migrations",
            "repositories",
            "sync_runs",
            "files",
            "symbols",
            "pages",
            "evidence_items",
            "claims",
            "claim_evidence",
            "provider_snapshots",
            "open_questions",
        ] {
            assert!(sql.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")));
        }
    }

    #[test]
    fn initial_migration_links_claims_to_evidence() {
        let sql = INITIAL_STATE_MIGRATION_SQL;

        assert!(sql.contains("PRIMARY KEY (claim_id, evidence_id)"));
        assert!(sql.contains("FOREIGN KEY (claim_id) REFERENCES claims(id)"));
        assert!(sql.contains("FOREIGN KEY (evidence_id) REFERENCES evidence_items(id)"));
    }

    #[test]
    fn repository_identity_prefers_remote_for_storage_key() {
        let identity = RepositoryIdentity::new(
            "/Users/example/project",
            Some("git@github.com:example/project.git".to_string()),
        );

        let key = identity.storage_key();

        assert!(key.starts_with("git-github-com-example-project-git-"));
        assert_eq!(key, identity.storage_key());
    }

    #[test]
    fn state_paths_are_repo_scoped() {
        let identity = RepositoryIdentity::new("/tmp/example", None);
        let paths = StatePaths::resolve("/tmp/app-data", "/tmp/cache", &identity);

        assert!(paths.state_dir.starts_with("/tmp/app-data/codewiki"));
        assert!(paths.cache_dir.starts_with("/tmp/cache/codewiki"));
        assert_eq!(paths.sqlite_path.file_name().unwrap(), "state.sqlite3");
    }

    #[test]
    fn sqlite_executor_applies_initial_migration() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = std::env::temp_dir().join(format!(
            "codewiki-store-test-{}-{}",
            std::process::id(),
            unique_test_suffix()
        ));
        let db = base.join("state.sqlite3");

        let report = apply_migrations_with_sqlite(&sqlite, &db).expect("migrations apply");

        assert_eq!(report.latest_version, 1);
        assert_eq!(report.migrations_seen, 1);
        assert!(db.exists());

        let output = Command::new(&sqlite)
            .arg(&db)
            .arg("SELECT version || ':' || name FROM schema_migrations ORDER BY version;")
            .output()
            .expect("query schema_migrations");

        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "1:initial_state"
        );

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn sqlite_persists_exploration_claims_and_evidence() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = std::env::temp_dir().join(format!(
            "codewiki-store-persist-test-{}-{}",
            std::process::id(),
            unique_test_suffix()
        ));
        let db = base.join("state.sqlite3");
        apply_migrations_with_sqlite(&sqlite, &db).expect("migrations apply");

        let identity = RepositoryIdentity::new(base.join("repo"), None);
        let snapshot = test_snapshot("hash:test", 3, 1);

        let report = persist_exploration_with_sqlite(&sqlite, &db, &identity, "init", &snapshot)
            .expect("persist snapshot");

        assert_eq!(report.files_seen, 1);
        assert_eq!(report.symbols_seen, 1);
        assert_eq!(report.evidence_seen, 1);
        assert!(report.claims_seen >= 2);

        let output = Command::new(&sqlite)
            .arg(&db)
            .arg("SELECT (SELECT COUNT(*) FROM files) || ':' || (SELECT COUNT(*) FROM symbols) || ':' || (SELECT COUNT(*) FROM evidence_items) || ':' || (SELECT COUNT(*) FROM claims) || ':' || (SELECT COUNT(*) FROM claim_evidence);")
            .output()
            .expect("query persisted state");

        assert!(output.status.success());
        let counts = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<_> = counts.trim().split(':').collect();
        assert_eq!(parts[0], "1");
        assert_eq!(parts[1], "1");
        assert_eq!(parts[2], "1");
        assert!(parts[3].parse::<usize>().expect("claims count") >= 2);
        assert!(parts[4].parse::<usize>().expect("claim evidence count") >= 2);

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn sqlite_marks_claims_stale_and_renders_qa_context() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = std::env::temp_dir().join(format!(
            "codewiki-store-stale-test-{}-{}",
            std::process::id(),
            unique_test_suffix()
        ));
        let db = base.join("state.sqlite3");
        apply_migrations_with_sqlite(&sqlite, &db).expect("migrations apply");

        let identity = RepositoryIdentity::new(base.join("repo"), None);
        let first = test_snapshot("hash:before", 3, 1);
        persist_exploration_with_sqlite(&sqlite, &db, &identity, "init", &first)
            .expect("persist first snapshot");
        let second = test_snapshot("hash:after", 3, 1);
        let report = persist_exploration_with_sqlite(&sqlite, &db, &identity, "sync", &second)
            .expect("persist changed snapshot");

        assert!(report.stale_claims_seen >= 1);

        let context = render_qa_context_with_sqlite(&sqlite, &db, "src/lib.rs", 10)
            .expect("render qa context");

        assert!(context.stale_claims_seen >= 1);
        assert!(context.markdown.contains("## Active Claims"));
        assert!(context.markdown.contains("## Stale Claims"));
        assert!(context.markdown.contains("src/lib.rs"));

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn wikiplan_records_detected_stack() {
        let plan = WikiPlan::from_detected(DetectedStack {
            languages: vec!["Rust".to_string()],
            package_managers: vec!["Cargo".to_string()],
            frameworks: Vec::new(),
            entrypoints: vec!["src/main.rs".to_string()],
            tests: Vec::new(),
            docs: Vec::new(),
        });

        let yaml = plan.to_yaml();

        assert!(yaml.contains("- \"Rust\""));
        assert!(yaml.contains("- \"Cargo\""));
        assert!(yaml.contains("- \"src/main.rs\""));
    }

    #[test]
    fn evidence_claim_models_have_confidence_labels() {
        let claim = Claim {
            statement: "The app has a Rust entrypoint.".to_string(),
            confidence: Confidence::SourceBacked,
            evidence: vec![EvidenceItem {
                kind: EvidenceKind::File,
                source_path: Some("src/main.rs".to_string()),
                summary: "Rust main file exists.".to_string(),
                confidence: Confidence::Confirmed,
            }],
        };

        assert_eq!(claim.confidence.as_str(), "source-backed");
        assert_eq!(claim.evidence[0].kind.as_str(), "file");
        assert_eq!(claim.evidence[0].confidence.as_str(), "confirmed");
    }

    #[test]
    fn sources_yaml_records_primary_and_imports() {
        let primary = SourceRecord::new("git", "source", "/workspace/source");
        let jira = SourceRecord::new("jira", "PROJ-123", "https://jira.example/browse/PROJ-123");

        let yaml = render_sources_yaml(&primary, &[jira]);

        assert!(yaml.contains("kind: git"));
        assert!(yaml.contains("primary: true"));
        assert!(yaml.contains("kind: jira"));
        assert!(yaml.contains("primary: false"));
    }

    fn unique_test_suffix() -> String {
        use std::fmt::Write as _;

        let mut value = String::new();
        write!(
            &mut value,
            "{:?}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time")
        )
        .expect("write suffix");
        slug_with_hash(&value)
    }

    fn test_snapshot(
        content_hash: &str,
        line_count: usize,
        symbol_count: usize,
    ) -> ExplorationSnapshot {
        let symbols = (0..symbol_count)
            .map(|index| codewiki_explore::ExploredSymbol {
                name: format!("build{index}"),
                kind: "function".to_string(),
                line: index + 1,
            })
            .collect::<Vec<_>>();
        ExplorationSnapshot {
            schema_version: 1,
            files: vec![codewiki_explore::ExploredFile {
                path: "src/lib.rs".to_string(),
                language: Some("Rust".to_string()),
                role: codewiki_explore::FileRole::Source,
                line_count,
                content_hash: content_hash.to_string(),
                symbols,
                imports: vec!["std::fs".to_string()],
                evidence_id: "file:test".to_string(),
            }],
            areas: vec![codewiki_explore::AreaSummary {
                name: "src".to_string(),
                file_count: 1,
                symbol_count,
                roles: vec![codewiki_explore::FileRole::Source],
            }],
            dependency_hints: Vec::new(),
            evidence: vec![codewiki_explore::EvidenceRef {
                id: "file:test".to_string(),
                path: "src/lib.rs".to_string(),
                kind: "file".to_string(),
            }],
            truncated: false,
            file_limit: 3_000,
        }
    }
}
