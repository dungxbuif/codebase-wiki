//! Core command orchestration for CodeWiki.

use codewiki_detect::{DetectionCapabilities, detect_repository};
use codewiki_docs::{WikiDocsLayout, render_semantic_pages};
use codewiki_explore::explore_repository;
use codewiki_store::{
    CodeWikiConfig, DetectedStack, RepositoryIdentity, SourceRecord, StatePaths, StoreLayout,
    WikiPlan, apply_migrations_with_sqlite, persist_exploration_with_sqlite, render_sources_yaml,
    render_target_agents_md,
};
use std::fs;
use std::path::{Path, PathBuf};

/// Result of executing a CLI command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliOutput {
    /// Process exit code expected by the CLI entrypoint.
    pub exit_code: i32,
    /// Text written to stdout.
    pub stdout: String,
    /// Text written to stderr.
    pub stderr: String,
}

impl CliOutput {
    fn ok(stdout: impl Into<String>) -> Self {
        Self {
            exit_code: 0,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }

    fn error(exit_code: i32, stderr: impl Into<String>) -> Self {
        Self {
            exit_code,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Help,
    Version,
    Status,
    Init { repo_root: PathBuf },
    Sync { repo_root: PathBuf },
}

/// Runtime context used by commands that touch the filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContext {
    /// Current working directory for relative repository paths.
    pub cwd: PathBuf,
    /// Base app-data directory for CodeWiki state.
    pub app_data_base: PathBuf,
    /// Base cache directory for CodeWiki cache.
    pub cache_base: PathBuf,
    /// SQLite executable path.
    pub sqlite_executable: PathBuf,
}

impl RuntimeContext {
    /// Build a runtime context from the current process environment.
    pub fn from_process() -> Result<Self, String> {
        Ok(Self {
            cwd: std::env::current_dir()
                .map_err(|error| format!("failed to resolve current directory: {error}"))?,
            app_data_base: default_app_data_base()?,
            cache_base: default_cache_base()?,
            sqlite_executable: PathBuf::from("sqlite3"),
        })
    }
}

/// Parse and execute a CodeWiki command.
pub fn run<I, S>(args: I) -> CliOutput
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let context = match RuntimeContext::from_process() {
        Ok(context) => context,
        Err(message) => return CliOutput::error(2, format!("error: {message}\n")),
    };

    run_with_context(args, &context)
}

/// Parse and execute a CodeWiki command with an explicit runtime context.
pub fn run_with_context<I, S>(args: I, context: &RuntimeContext) -> CliOutput
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    match parse_command(args, &context.cwd) {
        Ok(Command::Help) => CliOutput::ok(help_text()),
        Ok(Command::Version) => CliOutput::ok(format!("codewiki {}\n", env!("CARGO_PKG_VERSION"))),
        Ok(Command::Status) => CliOutput::ok(status_text()),
        Ok(Command::Init { repo_root }) => match init_repo(&repo_root, context) {
            Ok(summary) => CliOutput::ok(summary),
            Err(message) => CliOutput::error(1, format!("error: {message}\n")),
        },
        Ok(Command::Sync { repo_root }) => match sync_repo(&repo_root, context) {
            Ok(summary) => CliOutput::ok(summary),
            Err(message) => CliOutput::error(1, format!("error: {message}\n")),
        },
        Err(message) => CliOutput::error(2, message),
    }
}

fn parse_command<I, S>(args: I, cwd: &Path) -> Result<Command, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Ok(Command::Help);
    };

    match first.as_ref() {
        "help" | "-h" | "--help" => Ok(Command::Help),
        "version" | "-V" | "--version" => Ok(Command::Version),
        "status" => Ok(Command::Status),
        "init" => {
            let repo_root = match args.next() {
                Some(path) => resolve_repo_path(cwd, path.as_ref()),
                None => cwd.to_path_buf(),
            };
            if args.next().is_some() {
                return Err("error: usage is `codewiki init [path]`\n".to_string());
            }
            Ok(Command::Init { repo_root })
        }
        "sync" => {
            let repo_root = match args.next() {
                Some(path) => resolve_repo_path(cwd, path.as_ref()),
                None => cwd.to_path_buf(),
            };
            if args.next().is_some() {
                return Err("error: usage is `codewiki sync [path]`\n".to_string());
            }
            Ok(Command::Sync { repo_root })
        }
        unknown => Err(format!(
            "error: unknown command `{unknown}`\n\nRun `codewiki help` for available commands.\n"
        )),
    }
}

fn help_text() -> String {
    [
        "CodeWiki",
        "",
        "This binary is a companion tool for the CodeWiki skill.",
        "",
        "Usage:",
        "  codewiki help",
        "  codewiki version",
        "  codewiki status",
        "  codewiki init [path]",
        "  codewiki sync [path]",
        "",
        "Planned companion commands:",
        "  codewiki doctor",
        "  codewiki inspect",
        "  codewiki cache",
        "",
    ]
    .join("\n")
}

fn sync_repo(repo_root: &Path, context: &RuntimeContext) -> Result<String, String> {
    sync_workspace(repo_root, repo_root, context)
}

fn sync_workspace(
    source_root: &Path,
    workspace_root: &Path,
    context: &RuntimeContext,
) -> Result<String, String> {
    if !workspace_root.join(".codewiki").exists() {
        return Err("CodeWiki is not initialized; run `codewiki init` first".to_string());
    }
    let detection = detect_repository(source_root)
        .map_err(|error| format!("failed to detect repository signals: {error}"))?;
    let exploration = explore_repository(source_root)
        .map_err(|error| format!("failed to explore repository semantics: {error}"))?;
    let identity = RepositoryIdentity::new(source_root, None);
    let state_paths = StatePaths::resolve(&context.app_data_base, &context.cache_base, &identity);
    state_paths
        .ensure_dirs()
        .map_err(|error| format!("failed to create CodeWiki state directories: {error}"))?;
    let migration_report =
        apply_migrations_with_sqlite(&context.sqlite_executable, &state_paths.sqlite_path)?;
    let persistence_report = persist_exploration_with_sqlite(
        &context.sqlite_executable,
        &state_paths.sqlite_path,
        &identity,
        "sync",
        &exploration,
    )?;
    let repo_label = source_root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("repository");
    let mut actions = Vec::new();
    write_if_changed(
        &workspace_root.join(".codewiki/plan.yml"),
        &render_plan_with_detection(&detection),
        &mut actions,
    )?;
    for page in render_semantic_pages(repo_label, &detection.to_markdown(), &exploration) {
        write_if_changed(&workspace_root.join(page.path), &page.content, &mut actions)?;
    }

    if actions.is_empty() {
        Ok(format!(
            "CodeWiki sync no-op\nsource: {}\nworkspace: {}\nstate_db: {}\nmigration_version: {}\nclaims_persisted: {}\nstale_claims: {}\n",
            source_root.display(),
            workspace_root.display(),
            migration_report.sqlite_path.display(),
            migration_report.latest_version,
            persistence_report.claims_seen,
            persistence_report.stale_claims_seen,
        ))
    } else {
        Ok(format!(
            "CodeWiki synced\nsource: {}\nworkspace: {}\nstate_db: {}\nmigration_version: {}\nclaims_persisted: {}\nstale_claims: {}\n{}\n",
            source_root.display(),
            workspace_root.display(),
            migration_report.sqlite_path.display(),
            migration_report.latest_version,
            persistence_report.claims_seen,
            persistence_report.stale_claims_seen,
            actions.join("\n")
        ))
    }
}

fn status_text() -> String {
    let detection = DetectionCapabilities::scaffold();
    let store = StoreLayout::default();
    let docs = WikiDocsLayout::default();

    format!(
        "CodeWiki companion tool scaffold ready\nruntime: rust\ncommands: help, version, status\nplanned detection: {}\ncommitted config: {}\ncommitted plan: {}\nlocal agents: {}\nlocal state: {}\ndocs root: {}\n",
        detection.summary(),
        store.committed_config_path,
        store.committed_plan_path,
        store.committed_agents_path,
        store.local_state_summary,
        docs.generated_docs_root,
    )
}

fn init_repo(repo_root: &Path, context: &RuntimeContext) -> Result<String, String> {
    init_workspace(repo_root, repo_root, context)
}

/// Initialize CodeWiki for a source root into a wiki workspace.
///
/// When `source_root == workspace_root`, this is repo-local mode. When they differ,
/// source files are treated as evidence and generated docs/control files are written
/// into the external workspace.
pub fn init_workspace(
    source_root: &Path,
    workspace_root: &Path,
    context: &RuntimeContext,
) -> Result<String, String> {
    fs::create_dir_all(workspace_root)
        .map_err(|error| format!("failed to create wiki workspace root: {error}"))?;

    let identity = RepositoryIdentity::new(source_root, None);
    let detection = detect_repository(source_root)
        .map_err(|error| format!("failed to detect repository signals: {error}"))?;
    let exploration = explore_repository(source_root)
        .map_err(|error| format!("failed to explore repository semantics: {error}"))?;
    let state_paths = StatePaths::resolve(&context.app_data_base, &context.cache_base, &identity);
    state_paths
        .ensure_dirs()
        .map_err(|error| format!("failed to create CodeWiki state directories: {error}"))?;
    let migration_report =
        apply_migrations_with_sqlite(&context.sqlite_executable, &state_paths.sqlite_path)?;
    let persistence_report = persist_exploration_with_sqlite(
        &context.sqlite_executable,
        &state_paths.sqlite_path,
        &identity,
        "init",
        &exploration,
    )?;

    let mut actions = Vec::new();
    write_if_missing(
        &workspace_root.join(".codewiki/config.yml"),
        &CodeWikiConfig::default().to_yaml(),
        &mut actions,
    )?;
    write_if_missing(
        &workspace_root.join(".codewiki/plan.yml"),
        &render_plan_with_detection(&detection),
        &mut actions,
    )?;
    write_if_missing(
        &workspace_root.join(".codewiki/AGENTS.md"),
        &render_target_agents_md(),
        &mut actions,
    )?;
    let primary_source = SourceRecord::new(
        "git",
        "primary-repo",
        source_root.to_string_lossy().to_string(),
    );
    write_if_missing(
        &workspace_root.join(".codewiki/sources.yml"),
        &render_sources_yaml(&primary_source, &[]),
        &mut actions,
    )?;
    let repo_label = source_root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("repository");
    for page in render_semantic_pages(repo_label, &detection.to_markdown(), &exploration) {
        write_if_missing(&workspace_root.join(page.path), &page.content, &mut actions)?;
    }

    Ok(format!(
        "CodeWiki initialized\nsource: {}\nworkspace: {}\nstate_db: {}\nmigration_version: {}\nclaims_persisted: {}\nstale_claims: {}\n{}\n",
        source_root.display(),
        workspace_root.display(),
        migration_report.sqlite_path.display(),
        migration_report.latest_version,
        persistence_report.claims_seen,
        persistence_report.stale_claims_seen,
        actions.join("\n"),
    ))
}

fn render_plan_with_detection(detection: &codewiki_detect::RepositoryDetection) -> String {
    WikiPlan::from_detected(DetectedStack {
        languages: detection.languages.clone(),
        package_managers: detection.package_managers.clone(),
        frameworks: detection.frameworks.clone(),
        entrypoints: detection.entrypoints.clone(),
        tests: detection.tests.clone(),
        docs: detection.docs.clone(),
    })
    .to_yaml()
}

fn write_if_missing(path: &Path, content: &str, actions: &mut Vec<String>) -> Result<(), String> {
    if path.exists() {
        actions.push(format!("preserved: {}", path.display()));
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create `{}`: {error}", parent.display()))?;
    }
    fs::write(path, content)
        .map_err(|error| format!("failed to write `{}`: {error}", path.display()))?;
    actions.push(format!("created: {}", path.display()));
    Ok(())
}

fn write_if_changed(path: &Path, content: &str, actions: &mut Vec<String>) -> Result<(), String> {
    if path.exists() {
        let existing = fs::read_to_string(path)
            .map_err(|error| format!("failed to read `{}`: {error}", path.display()))?;
        if existing == content {
            return Ok(());
        }
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create `{}`: {error}", parent.display()))?;
    }
    fs::write(path, content)
        .map_err(|error| format!("failed to write `{}`: {error}", path.display()))?;
    actions.push(format!("updated: {}", path.display()));
    Ok(())
}

fn resolve_repo_path(cwd: &Path, path: &str) -> PathBuf {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    }
}

fn default_app_data_base() -> Result<PathBuf, String> {
    if let Ok(path) = std::env::var("CODEWIKI_APP_DATA_DIR") {
        return Ok(PathBuf::from(path));
    }
    if let Ok(path) = std::env::var("XDG_DATA_HOME") {
        return Ok(PathBuf::from(path));
    }
    let home = std::env::var("HOME").map_err(|_| "HOME is not set".to_string())?;
    if cfg!(target_os = "macos") {
        Ok(PathBuf::from(home).join("Library/Application Support"))
    } else {
        Ok(PathBuf::from(home).join(".local/share"))
    }
}

fn default_cache_base() -> Result<PathBuf, String> {
    if let Ok(path) = std::env::var("CODEWIKI_CACHE_DIR") {
        return Ok(PathBuf::from(path));
    }
    if let Ok(path) = std::env::var("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(path));
    }
    let home = std::env::var("HOME").map_err(|_| "HOME is not set".to_string())?;
    if cfg!(target_os = "macos") {
        Ok(PathBuf::from(home).join("Library/Caches"))
    } else {
        Ok(PathBuf::from(home).join(".cache"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_prints_help() {
        let output = run(std::iter::empty::<&str>());

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("Usage:"));
    }

    #[test]
    fn version_prints_package_version() {
        let output = run(["version"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.starts_with("codewiki "));
    }

    #[test]
    fn status_mentions_rust_runtime() {
        let output = run(["status"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("runtime: rust"));
        assert!(output.stdout.contains(".codewiki/config.yml"));
        assert!(output.stdout.contains(".codewiki/AGENTS.md"));
    }

    #[test]
    fn help_mentions_init() {
        let output = run(["help"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("codewiki init [path]"));
        assert!(output.stdout.contains("codewiki sync [path]"));
    }

    #[test]
    fn unknown_command_fails() {
        let output = run(["wat"]);

        assert_eq!(output.exit_code, 2);
        assert!(output.stderr.contains("unknown command"));
    }

    #[test]
    fn init_creates_required_files_and_state() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = temp_path("codewiki-core-init");
        let repo = base.join("repo");
        let context = RuntimeContext {
            cwd: repo.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: sqlite,
        };
        fs::create_dir_all(repo.join("src")).expect("mkdir src");
        fs::write(repo.join("src/lib.rs"), "pub fn build() {}\n").expect("write source");

        let output = run_with_context(["init"], &context);

        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert!(repo.join(".codewiki/config.yml").exists());
        assert!(repo.join(".codewiki/plan.yml").exists());
        assert!(repo.join(".codewiki/AGENTS.md").exists());
        assert!(repo.join(".codewiki/sources.yml").exists());
        assert!(repo.join("docs/codewiki/index.md").exists());
        assert!(repo.join("docs/codewiki/map.md").exists());
        assert!(repo.join("docs/codewiki/architecture.md").exists());
        assert!(repo.join("docs/codewiki/evidence/claims.md").exists());
        assert!(output.stdout.contains("migration_version: 1"));
        assert!(output.stdout.contains("claims_persisted:"));
        assert!(
            fs::read_to_string(repo.join(".codewiki/plan.yml"))
                .expect("read plan")
                .contains("detected:")
        );
        assert!(
            fs::read_to_string(repo.join("docs/codewiki/evidence/claims.md"))
                .expect("read claims")
                .contains("claim:")
        );
        assert!(
            fs::read_to_string(repo.join("docs/codewiki/map.md"))
                .expect("read map")
                .contains("Semantic Structure")
        );
        assert!(
            sqlite_count(
                &context.sqlite_executable,
                &find_state_db(&context),
                "claims"
            ) >= 1
        );

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn init_preserves_existing_files() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = temp_path("codewiki-core-preserve");
        let repo = base.join("repo");
        fs::create_dir_all(repo.join(".codewiki")).expect("create .codewiki");
        fs::write(repo.join(".codewiki/config.yml"), "custom: true\n").expect("write config");
        let context = RuntimeContext {
            cwd: repo.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: sqlite,
        };

        let output = run_with_context(["init"], &context);

        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert_eq!(
            fs::read_to_string(repo.join(".codewiki/config.yml")).expect("read config"),
            "custom: true\n"
        );
        assert!(output.stdout.contains("preserved:"));

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn sync_noops_when_current_and_updates_stale_page() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = temp_path("codewiki-core-sync");
        let repo = base.join("repo");
        fs::create_dir_all(repo.join("src")).expect("mkdir src");
        fs::write(repo.join("src/main.rs"), "fn main() {}\n").expect("write main");
        let context = RuntimeContext {
            cwd: repo.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: sqlite,
        };
        assert_eq!(run_with_context(["init"], &context).exit_code, 0);

        let no_op = run_with_context(["sync"], &context);
        assert_eq!(no_op.exit_code, 0, "{}", no_op.stderr);
        assert!(no_op.stdout.contains("no-op"));
        assert!(no_op.stdout.contains("claims_persisted:"));

        fs::write(repo.join("docs/codewiki/map.md"), "stale\n").expect("stale map");
        let synced = run_with_context(["sync"], &context);
        assert_eq!(synced.exit_code, 0, "{}", synced.stderr);
        assert!(synced.stdout.contains("updated:"));
        assert!(
            fs::read_to_string(repo.join("docs/codewiki/map.md"))
                .expect("read map")
                .contains("Repository Map")
        );
        assert!(
            fs::read_to_string(repo.join("docs/codewiki/map.md"))
                .expect("read map")
                .contains("Dependency Hints")
        );
        assert!(
            sqlite_count(
                &context.sqlite_executable,
                &find_state_db(&context),
                "sync_runs"
            ) >= 2
        );

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn init_workspace_keeps_docs_outside_source_repo() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = temp_path("codewiki-core-external-workspace");
        let source = base.join("source");
        let workspace = base.join("personal-wiki");
        fs::create_dir_all(source.join("src")).expect("mkdir src");
        fs::write(source.join("src/main.rs"), "fn main() {}\n").expect("write main");
        let context = RuntimeContext {
            cwd: source.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: sqlite,
        };

        let output = init_workspace(&source, &workspace, &context).expect("init workspace");

        assert!(output.contains("workspace:"));
        assert!(workspace.join(".codewiki/config.yml").exists());
        assert!(workspace.join(".codewiki/sources.yml").exists());
        assert!(workspace.join("docs/codewiki/index.md").exists());
        assert!(
            fs::read_to_string(workspace.join("docs/codewiki/map.md"))
                .expect("read map")
                .contains("src/main.rs")
        );
        assert!(
            fs::read_to_string(workspace.join("docs/codewiki/evidence/claims.md"))
                .expect("read claims")
                .contains("claim:")
        );
        assert!(!source.join(".codewiki/config.yml").exists());
        assert!(!source.join("docs/codewiki/index.md").exists());
        assert!(
            fs::read_to_string(workspace.join(".codewiki/sources.yml"))
                .expect("read sources")
                .contains("primary: true")
        );

        let _ = fs::remove_dir_all(base);
    }

    fn find_state_db(context: &RuntimeContext) -> PathBuf {
        let codewiki_dir = context.app_data_base.join("codewiki");
        let repo_dir = fs::read_dir(codewiki_dir)
            .expect("read codewiki state dir")
            .next()
            .expect("state dir exists")
            .expect("state dir entry")
            .path();
        repo_dir.join("state.sqlite3")
    }

    fn sqlite_count(sqlite: &Path, db: &Path, table: &str) -> usize {
        let output = std::process::Command::new(sqlite)
            .arg(db)
            .arg(format!("SELECT COUNT(*) FROM {table};"))
            .output()
            .expect("query sqlite count");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .expect("count")
    }

    fn temp_path(prefix: &str) -> PathBuf {
        let suffix = format!(
            "{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time")
        );
        std::env::temp_dir().join(format!("{prefix}-{suffix}"))
    }
}
