//! Durable state and cache boundary for CodeWiki.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Default CodeWiki schema version for committed project files.
pub const CODEWIKI_SCHEMA_VERSION: u32 = 1;

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

/// Planned storage layout for committed and local state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreLayout {
    /// Repo-committed project configuration path.
    pub committed_config_path: &'static str,
    /// Repo-committed plan summary path.
    pub committed_plan_path: &'static str,
    /// Repo-committed target-repository agent guidance path.
    pub committed_agents_path: &'static str,
    /// Human-readable summary of local state storage.
    pub local_state_summary: &'static str,
    /// Human-readable summary of rebuildable cache storage.
    pub cache_summary: &'static str,
}

impl Default for StoreLayout {
    fn default() -> Self {
        Self {
            committed_config_path: ".codewiki/config.yml",
            committed_plan_path: ".codewiki/plan.yml",
            committed_agents_path: ".codewiki/AGENTS.md",
            local_state_summary: "platform app data SQLite, keyed by repository identity",
            cache_summary: ".codewiki/cache is rebuildable and may be ignored",
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
    /// Runtime tool policy.
    pub tool_policy: ToolSelectionPolicy,
}

impl Default for CodeWikiConfig {
    fn default() -> Self {
        let layout = StoreLayout::default();

        Self {
            schema_version: CODEWIKI_SCHEMA_VERSION,
            docs_root: "docs/codewiki",
            plan_path: layout.committed_plan_path,
            agents_path: layout.committed_agents_path,
            tool_policy: ToolSelectionPolicy::default(),
        }
    }
}

impl CodeWikiConfig {
    /// Render the default config as stable YAML.
    pub fn to_yaml(&self) -> String {
        format!(
            "schema_version: {}\ndocs_root: {}\nplan_path: {}\nagents_path: {}\ntool_policy:\n  lazy_activation: {}\n  code_intelligence_default: {}\n  codebase_memory_mcp_trigger: {}\n  cocoindex_trigger: {}\n",
            self.schema_version,
            self.docs_root,
            self.plan_path,
            self.agents_path,
            self.tool_policy.lazy_activation,
            self.tool_policy.code_intelligence_default,
            self.tool_policy.memory_trigger,
            self.tool_policy.indexing_trigger,
        )
    }
}

/// Initial committed WikiPlan skeleton.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiPlan {
    /// Plan schema version.
    pub schema_version: u32,
    /// Current plan status.
    pub status: &'static str,
    /// Evidence policy for generated docs.
    pub evidence_policy: &'static str,
}

impl Default for WikiPlan {
    fn default() -> Self {
        Self {
            schema_version: CODEWIKI_SCHEMA_VERSION,
            status: "draft",
            evidence_policy: "claims must cite files, symbols, commands, docs, or explicit hypotheses",
        }
    }
}

impl WikiPlan {
    /// Render the plan skeleton as stable YAML.
    pub fn to_yaml(&self) -> String {
        format!(
            "schema_version: {}\nstatus: {}\nevidence_policy: {}\npages: []\nopen_questions: []\nstale_claims: []\n",
            self.schema_version, self.status, self.evidence_policy
        )
    }
}

/// Render target-repository CodeWiki agent guidance.
pub fn render_target_agents_md() -> String {
    [
        "# CodeWiki Local Agent Guidance",
        "",
        "Use docs-first lazy activation:",
        "",
        "1. Read `docs/codewiki/**`.",
        "2. Read `.codewiki/plan.yml`.",
        "3. Read `.codewiki/AGENTS.md`.",
        "4. Query local CodeWiki SQLite facts/evidence/claims when available.",
        "5. Inspect source files and Git history when docs are missing or stale.",
        "6. Activate external runtime tools only when evidence quality requires it.",
        "",
        "Runtime tool policy:",
        "",
        "- Octocode is the first-choice code-intelligence provider when a provider is needed.",
        "- codebase-memory-mcp is only for shared cross-session memory beyond CodeWiki SQLite state.",
        "- CocoIndex is only for repo scale or repeated refresh/query indexing workloads.",
        "",
        "Record selected tools and trigger reasons in `.codewiki/config.yml` and local runtime state.",
        "",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_layout_includes_committed_paths() {
        let layout = StoreLayout::default();

        assert_eq!(layout.committed_config_path, ".codewiki/config.yml");
        assert_eq!(layout.committed_plan_path, ".codewiki/plan.yml");
        assert_eq!(layout.committed_agents_path, ".codewiki/AGENTS.md");
    }

    #[test]
    fn config_yaml_records_lazy_provider_policy() {
        let yaml = CodeWikiConfig::default().to_yaml();

        assert!(yaml.contains("lazy_activation: true"));
        assert!(yaml.contains("code_intelligence_default: octocode"));
        assert!(yaml.contains("codebase_memory_mcp_trigger: shared cross-session memory"));
        assert!(yaml.contains("cocoindex_trigger: repo scale"));
    }

    #[test]
    fn plan_yaml_records_evidence_policy() {
        let yaml = WikiPlan::default().to_yaml();

        assert!(yaml.contains("schema_version: 1"));
        assert!(yaml.contains("claims must cite files"));
        assert!(yaml.contains("pages: []"));
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
}
