//! Core command orchestration for CodeWiki.

use codewiki_detect::{DetectionCapabilities, detect_repository};
use codewiki_docs::{
    GENERATED_REGION_END, GENERATED_REGION_HASH_PREFIX, GENERATED_REGION_START, WikiDocsLayout,
    generated_region_hash, render_semantic_pages, validate_reader_workspace,
};
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
    Doctor { skill_root: PathBuf },
    PackageDigest { skill_root: PathBuf },
    Init { repo_root: PathBuf },
    Sync { repo_root: PathBuf },
    Validate { repo_root: PathBuf },
}

/// Version of the contract between the skill workflow and Rust companion.
pub const COMPANION_INTERFACE_VERSION: u32 = 2;

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
        Ok(Command::Doctor { skill_root }) => match diagnose_installation(&skill_root) {
            Ok(summary) => CliOutput::ok(summary),
            Err(message) => CliOutput::error(1, format!("error: {message}\n")),
        },
        Ok(Command::PackageDigest { skill_root }) => match managed_skill_digest(&skill_root) {
            Ok(digest) => CliOutput::ok(format!("{digest}\n")),
            Err(message) => CliOutput::error(1, format!("error: {message}\n")),
        },
        Ok(Command::Init { repo_root }) => match init_repo(&repo_root, context) {
            Ok(summary) => CliOutput::ok(summary),
            Err(message) => CliOutput::error(1, format!("error: {message}\n")),
        },
        Ok(Command::Sync { repo_root }) => match sync_repo(&repo_root, context) {
            Ok(summary) => CliOutput::ok(summary),
            Err(message) => CliOutput::error(1, format!("error: {message}\n")),
        },
        Ok(Command::Validate { repo_root }) => match validate_workspace(&repo_root) {
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
        "doctor" => {
            let skill_root = match args.next() {
                Some(path) => resolve_repo_path(cwd, path.as_ref()),
                None => cwd.to_path_buf(),
            };
            if args.next().is_some() {
                return Err("error: usage is `codewiki doctor [skill-root]`\n".to_string());
            }
            Ok(Command::Doctor { skill_root })
        }
        "package-digest" => {
            let skill_root = match args.next() {
                Some(path) => resolve_repo_path(cwd, path.as_ref()),
                None => cwd.to_path_buf(),
            };
            if args.next().is_some() {
                return Err("error: usage is `codewiki package-digest [skill-root]`\n".to_string());
            }
            Ok(Command::PackageDigest { skill_root })
        }
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
        "validate" => {
            let repo_root = match args.next() {
                Some(path) => resolve_repo_path(cwd, path.as_ref()),
                None => cwd.to_path_buf(),
            };
            if args.next().is_some() {
                return Err("error: usage is `codewiki validate [path]`\n".to_string());
            }
            Ok(Command::Validate { repo_root })
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
        "  codewiki doctor [skill-root]",
        "  codewiki init [path]",
        "  codewiki sync [path]",
        "  codewiki validate [path]",
        "",
        "Internal/diagnostic commands:",
        "  codewiki package-digest [skill-root]",
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
    reject_legacy_control_plane(workspace_root)?;
    let (source_commit, source_dirty) = source_provenance(source_root);
    if !workspace_root
        .join(".agents/skills/codewiki/project")
        .exists()
    {
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
    migrate_legacy_generated_docs(workspace_root, &mut actions)?;
    let plan_path = workspace_root.join(".agents/skills/codewiki/project/plan.yml");
    migrate_legacy_plan(
        workspace_root,
        &detection,
        &source_commit,
        source_dirty,
        &mut actions,
    )?;
    if !plan_path.exists() {
        write_if_missing(
            &plan_path,
            &render_plan_with_detection(&detection, &source_commit, source_dirty),
            &mut actions,
        )?;
    }
    for page in render_semantic_pages(repo_label, &detection.to_markdown(), &exploration) {
        write_if_changed(&workspace_root.join(page.path), &page.content, &mut actions)?;
    }

    let run_path = workspace_root.join(".agents/skills/codewiki/project/run.yml");
    let quality_path = workspace_root.join(".agents/skills/codewiki/project/quality-report.yml");
    let evidence_changed = !actions.is_empty() || persistence_report.stale_claims_seen > 0;
    if evidence_changed || !run_path.exists() || !quality_path.exists() {
        write_control_if_changed(
            &run_path,
            &render_run_status("synthesis_incomplete"),
            &mut actions,
        )?;
        write_control_if_changed(
            &quality_path,
            &render_quality_report_template(),
            &mut actions,
        )?;
    }

    if actions.is_empty() {
        Ok(format!(
            "CodeWiki sync no-op\nsource: {}\nworkspace: {}\nstate_db: {}\nmigration_version: {}\nclaims_persisted: {}\nstale_claims: {}\ngeneration_status: synthesis_incomplete\n",
            source_root.display(),
            workspace_root.display(),
            migration_report.sqlite_path.display(),
            migration_report.latest_version,
            persistence_report.claims_seen,
            persistence_report.stale_claims_seen,
        ))
    } else {
        Ok(format!(
            "CodeWiki evidence refreshed\nsource: {}\nworkspace: {}\nstate_db: {}\nmigration_version: {}\nclaims_persisted: {}\nstale_claims: {}\ngeneration_status: synthesis_incomplete\nnext: run the CodeWiki skill synthesis workflow, then `codewiki validate {}`\n{}\n",
            source_root.display(),
            workspace_root.display(),
            migration_report.sqlite_path.display(),
            migration_report.latest_version,
            persistence_report.claims_seen,
            persistence_report.stale_claims_seen,
            workspace_root.display(),
            actions.join("\n")
        ))
    }
}

fn status_text() -> String {
    let detection = DetectionCapabilities::scaffold();
    let store = StoreLayout::default();
    let docs = WikiDocsLayout::default();

    format!(
        "CodeWiki companion tool ready\nruntime: rust\ncompanion_interface_version: {}\n{}commands: help, version, status, doctor, init, sync, validate\nrepository detection: {}\ncommitted config: {}\ncommitted plan: {}\nlocal agents: {}\nlocal state: {}\ndocs root: {}\n",
        COMPANION_INTERFACE_VERSION,
        runtime_skill_metadata(),
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
    reject_legacy_control_plane(workspace_root)?;
    // Capture source provenance before repo-local initialization writes its own
    // control files. Otherwise CodeWiki would report a clean repository as dirty
    // because of the files it just created.
    let (source_commit, source_dirty) = source_provenance(source_root);
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
    migrate_legacy_generated_docs(workspace_root, &mut actions)?;
    write_if_missing(
        &workspace_root.join(".agents/skills/codewiki/project/config.yml"),
        &CodeWikiConfig::default().to_yaml(),
        &mut actions,
    )?;
    write_if_missing(
        &workspace_root.join(".agents/skills/codewiki/project/plan.yml"),
        &render_plan_with_detection(&detection, &source_commit, source_dirty),
        &mut actions,
    )?;
    write_if_missing(
        &workspace_root.join(".agents/skills/codewiki/project/AGENTS.md"),
        &render_target_agents_md(),
        &mut actions,
    )?;
    let primary_source = SourceRecord::new(
        "git",
        "primary-repo",
        source_root.to_string_lossy().to_string(),
    );
    write_if_missing(
        &workspace_root.join(".agents/skills/codewiki/project/sources.yml"),
        &render_sources_yaml(&primary_source, &[]),
        &mut actions,
    )?;
    write_control_if_changed(
        &workspace_root.join(".agents/skills/codewiki/project/run.yml"),
        &render_run_status("synthesis_incomplete"),
        &mut actions,
    )?;
    write_control_if_changed(
        &workspace_root.join(".agents/skills/codewiki/project/quality-report.yml"),
        &render_quality_report_template(),
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
        "CodeWiki evidence initialized\nsource: {}\nworkspace: {}\nstate_db: {}\nmigration_version: {}\nclaims_persisted: {}\nstale_claims: {}\ngeneration_status: synthesis_incomplete\nnext: run the CodeWiki skill synthesis workflow, then `codewiki validate {}`\n{}\n",
        source_root.display(),
        workspace_root.display(),
        migration_report.sqlite_path.display(),
        migration_report.latest_version,
        persistence_report.claims_seen,
        persistence_report.stale_claims_seen,
        workspace_root.display(),
        actions.join("\n"),
    ))
}

fn render_plan_with_detection(
    detection: &codewiki_detect::RepositoryDetection,
    source_commit: &str,
    source_dirty: bool,
) -> String {
    WikiPlan::from_detected(DetectedStack {
        languages: detection.languages.clone(),
        package_managers: detection.package_managers.clone(),
        frameworks: detection.frameworks.clone(),
        entrypoints: detection.entrypoints.clone(),
        tests: detection.tests.clone(),
        docs: detection.docs.clone(),
    })
    .with_provenance(
        source_commit.to_string(),
        source_dirty,
        detection.docs.clone(),
    )
    .to_yaml()
}

fn source_provenance(source_root: &Path) -> (String, bool) {
    (
        git_value(source_root, &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".to_string()),
        git_value(source_root, &["status", "--porcelain"])
            .is_some_and(|status| !status.trim().is_empty()),
    )
}

fn reject_legacy_control_plane(workspace_root: &Path) -> Result<(), String> {
    let legacy = workspace_root.join(".codewiki");
    if legacy.exists() {
        return Err(format!(
            "legacy_control_plane: `{}` exists; move or migrate it explicitly before using `.agents/skills/codewiki/project`",
            legacy.display()
        ));
    }
    Ok(())
}

fn migrate_legacy_plan(
    workspace_root: &Path,
    detection: &codewiki_detect::RepositoryDetection,
    source_commit: &str,
    source_dirty: bool,
    actions: &mut Vec<String>,
) -> Result<(), String> {
    let project_root = workspace_root.join(".agents/skills/codewiki/project");
    let plan_path = project_root.join("plan.yml");
    if !plan_path.exists() {
        return Ok(());
    }
    let legacy_plan = fs::read_to_string(&plan_path)
        .map_err(|error| format!("failed to read `{}`: {error}", plan_path.display()))?;
    let Some(schema_version) = yaml_scalar(&legacy_plan, "schema_version") else {
        return Err(format!(
            "invalid_plan: `{}` has no schema_version",
            plan_path.display()
        ));
    };
    let schema_version = schema_version.parse::<u32>().map_err(|_| {
        format!(
            "invalid_plan: `{}` has a non-integer schema_version",
            plan_path.display()
        )
    })?;
    if schema_version == codewiki_store::WIKIPLAN_SCHEMA_VERSION {
        return Ok(());
    }
    if schema_version != 1 {
        return Err(format!(
            "incompatible_plan: schema {schema_version} is not supported; expected 1 or {}",
            codewiki_store::WIKIPLAN_SCHEMA_VERSION
        ));
    }

    let backup_path = project_root.join("plan.v1.legacy.yml");
    if backup_path.exists() {
        let existing = fs::read_to_string(&backup_path)
            .map_err(|error| format!("failed to read `{}`: {error}", backup_path.display()))?;
        if existing != legacy_plan {
            return Err(format!(
                "legacy_plan_conflict: `{}` already contains a different v1 plan",
                backup_path.display()
            ));
        }
    } else {
        fs::write(&backup_path, &legacy_plan)
            .map_err(|error| format!("failed to preserve `{}`: {error}", backup_path.display()))?;
        actions.push(format!("preserved-legacy-plan: {}", backup_path.display()));
    }

    let enriched = WikiPlan::from_detected(DetectedStack {
        languages: detection.languages.clone(),
        package_managers: detection.package_managers.clone(),
        frameworks: detection.frameworks.clone(),
        entrypoints: detection.entrypoints.clone(),
        tests: detection.tests.clone(),
        docs: detection.docs.clone(),
    })
    .with_provenance(source_commit.to_string(), source_dirty, detection.docs.clone())
    .with_open_question(
        "Enrich canonical concepts and evidence anchors from preserved plan.v1.legacy.yml before reader synthesis.",
    )
    .to_yaml();
    write_control_if_changed(&plan_path, &enriched, actions)
}

fn render_run_status(status: &str) -> String {
    let (mental_model, wikiplan, synthesis, quality) = if status == "reader_docs_ready" {
        ("complete", "complete", "complete", "pass")
    } else {
        ("pending", "scaffold_only", "pending", "pending")
    };
    format!(
        "schema_version: 1\ncompanion_interface_version: {}\ngeneration_status: {}\n{}stages:\n  discovery: complete\n  evidence_persistence: complete\n  repository_mental_model: {}\n  wikiplan: {}\n  page_synthesis: {}\n  quality: {}\n",
        COMPANION_INTERFACE_VERSION,
        status,
        runtime_skill_metadata(),
        mental_model,
        wikiplan,
        synthesis,
        quality
    )
}

fn runtime_skill_metadata() -> String {
    let Ok(root) = std::env::var("CODEWIKI_SKILL_ROOT") else {
        return "skill_installation:\n  state: not_recorded\n".to_string();
    };
    let root = PathBuf::from(root);
    let package = fs::read_to_string(root.join("package.yml")).unwrap_or_default();
    let installation = fs::read_to_string(root.join("INSTALLATION.yml")).unwrap_or_default();
    let state = if diagnose_installation(&root).is_ok() {
        "verified"
    } else if installation.is_empty() {
        "legacy_unverified"
    } else {
        "invalid"
    };
    format!(
        "skill_installation:\n  state: {}\n  root: \"{}\"\n  package_version: \"{}\"\n  skill_contract_version: \"{}\"\n  reference_contract_version: \"{}\"\n  managed_digest: \"{}\"\n  source_revision: \"{}\"\n",
        state,
        yaml_string(&root.to_string_lossy()),
        yaml_string(
            &yaml_scalar(&package, "package_version").unwrap_or_else(|| "unknown".to_string())
        ),
        yaml_string(
            &yaml_scalar(&package, "skill_contract_version")
                .unwrap_or_else(|| "unknown".to_string())
        ),
        yaml_string(
            &yaml_scalar(&package, "reference_contract_version")
                .unwrap_or_else(|| "unknown".to_string())
        ),
        yaml_string(
            &yaml_scalar(&installation, "managed_digest").unwrap_or_else(|| "unknown".to_string())
        ),
        yaml_string(
            &yaml_scalar(&installation, "source_revision").unwrap_or_else(|| "unknown".to_string())
        ),
    )
}

fn yaml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn render_quality_report_template() -> String {
    "schema_version: 1\nmodel_synthesis: pending\ncontract_coverage: pending\nsource_audit: pending\ndiagram_audit: pending\ncross_page_review: pending\ndocs_only_onboarding: pending\nreader_context: docs_only\nsource_auditor_context: source_and_evidence\ncritical_failures: pending\nrevision_attempts: 0\ngeneration_model: \"unrecorded\"\nevaluation_model: \"unrecorded\"\nnotes: \"The CodeWiki skill must run isolated reader and source-auditor checks before validation.\"\n".to_string()
}

fn validate_workspace(workspace_root: &Path) -> Result<String, String> {
    let report = validate_reader_workspace(workspace_root);
    if !report.ready {
        return Err(format!(
            "reader documentation quality failed ({} pages checked):\n- {}",
            report.reader_pages_checked,
            report.errors.join("\n- ")
        ));
    }
    let run_path = workspace_root.join(".agents/skills/codewiki/project/run.yml");
    if let Some(parent) = run_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create `{}`: {error}", parent.display()))?;
    }
    let existing_run = fs::read_to_string(&run_path)
        .map_err(|error| format!("failed to read `{}`: {error}", run_path.display()))?;
    fs::write(&run_path, mark_run_reader_docs_ready(&existing_run))
        .map_err(|error| format!("failed to write `{}`: {error}", run_path.display()))?;
    Ok(format!(
        "CodeWiki reader docs ready\nworkspace: {}\nreader_pages_checked: {}\ngeneration_status: reader_docs_ready\n",
        workspace_root.display(),
        report.reader_pages_checked
    ))
}

fn mark_run_reader_docs_ready(existing: &str) -> String {
    let mut output = String::new();
    for line in existing.lines() {
        let replacement = if line.starts_with("generation_status:") {
            "generation_status: reader_docs_ready"
        } else if line.trim_start().starts_with("repository_mental_model:") {
            "  repository_mental_model: complete"
        } else if line.trim_start().starts_with("wikiplan:") {
            "  wikiplan: complete"
        } else if line.trim_start().starts_with("page_synthesis:") {
            "  page_synthesis: complete"
        } else if line.trim_start().starts_with("quality:") {
            "  quality: pass"
        } else {
            line
        };
        output.push_str(replacement);
        output.push('\n');
    }
    output
}

fn git_value(root: &Path, args: &[&str]) -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn diagnose_installation(skill_root: &Path) -> Result<String, String> {
    let package_path = skill_root.join("package.yml");
    let installation_path = skill_root.join("INSTALLATION.yml");
    let package = fs::read_to_string(&package_path).map_err(|_| {
        format!(
            "legacy_unverified: missing source package manifest `{}`; reinstall CodeWiki explicitly",
            package_path.display()
        )
    })?;
    let installation = fs::read_to_string(&installation_path).map_err(|_| {
        format!(
            "legacy_unverified: missing install provenance `{}`; reinstall CodeWiki explicitly",
            installation_path.display()
        )
    })?;
    for key in [
        "package_version",
        "skill_contract_version",
        "reference_contract_version",
        "companion_interface_version",
        "wikiplan_schema_min",
        "wikiplan_schema_max",
    ] {
        let source_value = yaml_scalar(&package, key)
            .ok_or_else(|| format!("invalid_manifest: package.yml lacks {key}"))?;
        if matches!(
            key,
            "package_version"
                | "skill_contract_version"
                | "reference_contract_version"
                | "companion_interface_version"
        ) {
            let installed_value = yaml_scalar(&installation, key)
                .ok_or_else(|| format!("legacy_unverified: INSTALLATION.yml lacks {key}"))?;
            if source_value != installed_value {
                return Err(format!(
                    "incompatible: {key} differs (package={source_value}, installed={installed_value})"
                ));
            }
        }
    }
    let schema_min = yaml_scalar(&package, "wikiplan_schema_min")
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| "invalid_manifest: wikiplan_schema_min is not an integer".to_string())?;
    let schema_max = yaml_scalar(&package, "wikiplan_schema_max")
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| "invalid_manifest: wikiplan_schema_max is not an integer".to_string())?;
    if !(schema_min..=schema_max).contains(&codewiki_store::WIKIPLAN_SCHEMA_VERSION) {
        return Err(format!(
            "incompatible: runtime WikiPlan schema {} is outside package range {schema_min}..={schema_max}",
            codewiki_store::WIKIPLAN_SCHEMA_VERSION
        ));
    }
    let expected_digest = yaml_scalar(&installation, "managed_digest")
        .ok_or_else(|| "legacy_unverified: INSTALLATION.yml lacks managed_digest".to_string())?;
    let actual_digest = managed_skill_digest(skill_root)?;
    if expected_digest != actual_digest {
        return Err(format!(
            "content_drift: managed skill digest differs (expected {expected_digest}, actual {actual_digest})"
        ));
    }
    let package_interface = yaml_scalar(&package, "companion_interface_version")
        .ok_or_else(|| "package.yml lacks companion_interface_version".to_string())?;
    let installed_interface = yaml_scalar(&installation, "companion_interface_version")
        .ok_or_else(|| "INSTALLATION.yml lacks companion_interface_version".to_string())?;
    let runtime_interface = COMPANION_INTERFACE_VERSION.to_string();
    if package_interface != runtime_interface || installed_interface != runtime_interface {
        return Err(format!(
            "incompatible: package={package_interface}, installed={installed_interface}, runtime={runtime_interface}"
        ));
    }
    Ok(format!(
        "CodeWiki installation verified\nstate: verified\nskill_root: {}\ninstall_scope: {}\nmanaged_digest: {}\ncompanion_interface_version: {}\npackage_version: {}\nskill_contract_version: {}\nreference_contract_version: {}\nwikiplan_schema_range: {}..={}\nsource_revision: {}\n",
        skill_root.display(),
        yaml_scalar(&installation, "install_scope").unwrap_or_else(|| "unknown".to_string()),
        actual_digest,
        runtime_interface,
        yaml_scalar(&package, "package_version").unwrap_or_else(|| "unknown".to_string()),
        yaml_scalar(&package, "skill_contract_version").unwrap_or_else(|| "unknown".to_string()),
        yaml_scalar(&package, "reference_contract_version")
            .unwrap_or_else(|| "unknown".to_string()),
        schema_min,
        schema_max,
        yaml_scalar(&installation, "source_revision").unwrap_or_else(|| "unknown".to_string()),
    ))
}

fn yaml_scalar(yaml: &str, key: &str) -> Option<String> {
    yaml.lines().find_map(|line| {
        let (candidate, value) = line.split_once(':')?;
        if candidate.trim() != key {
            return None;
        }
        Some(value.trim().trim_matches('"').to_string())
    })
}

fn managed_skill_digest(skill_root: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    collect_managed_skill_files(skill_root, skill_root, &mut files)?;
    files.sort();
    let mut hash = 0xcbf29ce484222325_u64;
    for path in files {
        let relative = path
            .strip_prefix(skill_root)
            .map_err(|error| format!("failed to relativize `{}`: {error}", path.display()))?;
        for byte in relative.to_string_lossy().as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0;
        hash = hash.wrapping_mul(0x100000001b3);
        let content = fs::read(&path).map_err(|error| {
            format!("failed to read managed file `{}`: {error}", path.display())
        })?;
        for byte in content {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

fn collect_managed_skill_files(
    skill_root: &Path,
    current: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let entries = fs::read_dir(current).map_err(|error| {
        format!(
            "failed to read skill directory `{}`: {error}",
            current.display()
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("failed to inspect skill entry: {error}"))?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect `{}`: {error}", path.display()))?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "managed skill content must not contain symlinks: {}",
                path.display()
            ));
        }
        let relative = path.strip_prefix(skill_root).unwrap_or(&path);
        let first = relative
            .components()
            .next()
            .and_then(|part| part.as_os_str().to_str());
        if matches!(
            first,
            Some("bin" | "companion" | "project" | ".git" | "target")
        ) || relative == Path::new("INSTALLATION.yml")
        {
            continue;
        }
        if path.is_dir() {
            collect_managed_skill_files(skill_root, &path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
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

fn write_control_if_changed(
    path: &Path,
    content: &str,
    actions: &mut Vec<String>,
) -> Result<(), String> {
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
    actions.push(format!("updated-control: {}", path.display()));
    Ok(())
}

fn write_if_changed(path: &Path, content: &str, actions: &mut Vec<String>) -> Result<(), String> {
    if path.exists() {
        let existing = fs::read_to_string(path)
            .map_err(|error| format!("failed to read `{}`: {error}", path.display()))?;
        if existing == content {
            return Ok(());
        }
        match merge_generated_region(&existing, content) {
            GeneratedRegionMerge::Merged(merged) => {
                if merged == existing {
                    return Ok(());
                }
                fs::write(path, merged)
                    .map_err(|error| format!("failed to write `{}`: {error}", path.display()))?;
                actions.push(format!("updated-generated-region: {}", path.display()));
                return Ok(());
            }
            GeneratedRegionMerge::HumanEdited => {
                actions.push(format!(
                    "preserved-human-edited-generated-region: {}",
                    path.display()
                ));
                return Ok(());
            }
            GeneratedRegionMerge::LegacyUnverified => {
                actions.push(format!(
                    "preserved-unverified-legacy-generated-region: {}",
                    path.display()
                ));
                return Ok(());
            }
            GeneratedRegionMerge::NoRegion => {}
        }
        actions.push(format!(
            "preserved-human-owned-unmarked: {}",
            path.display()
        ));
        return Ok(());
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum GeneratedRegionMerge {
    Merged(String),
    HumanEdited,
    LegacyUnverified,
    NoRegion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedGeneratedRegion<'a> {
    start: usize,
    end: usize,
    body: &'a str,
    recorded_hash: Option<&'a str>,
}

fn merge_generated_region(existing: &str, generated: &str) -> GeneratedRegionMerge {
    let Some(region) = parse_generated_region(existing) else {
        return GeneratedRegionMerge::NoRegion;
    };
    let Some(recorded_hash) = region.recorded_hash else {
        return GeneratedRegionMerge::LegacyUnverified;
    };
    if recorded_hash != generated_region_hash(region.body) {
        return GeneratedRegionMerge::HumanEdited;
    }

    let mut merged = String::new();
    merged.push_str(&existing[..region.start]);
    merged.push_str(generated.trim_end());
    merged.push('\n');
    merged.push_str(&existing[region.end..]);
    GeneratedRegionMerge::Merged(merged)
}

fn parse_generated_region(content: &str) -> Option<ParsedGeneratedRegion<'_>> {
    let start = content.find(GENERATED_REGION_START)?;
    let after_start = start + GENERATED_REGION_START.len();
    let after_start = consume_line_ending(content, after_start)?;
    let (recorded_hash, body_start) =
        if content[after_start..].starts_with(GENERATED_REGION_HASH_PREFIX) {
            let hash_end = content[after_start..].find(" -->")? + after_start;
            let hash = &content[after_start + GENERATED_REGION_HASH_PREFIX.len()..hash_end];
            let marker_end = hash_end + " -->".len();
            (Some(hash), consume_line_ending(content, marker_end)?)
        } else {
            (None, after_start)
        };
    let end_marker = content[body_start..].find(GENERATED_REGION_END)? + body_start;
    let raw_body = &content[body_start..end_marker];
    let body = raw_body
        .strip_suffix("\r\n")
        .or_else(|| raw_body.strip_suffix('\n'))
        .unwrap_or(raw_body);
    let end = end_marker + GENERATED_REGION_END.len();
    Some(ParsedGeneratedRegion {
        start,
        end,
        body,
        recorded_hash,
    })
}

fn consume_line_ending(content: &str, offset: usize) -> Option<usize> {
    if content[offset..].starts_with("\r\n") {
        Some(offset + 2)
    } else if content[offset..].starts_with('\n') {
        Some(offset + 1)
    } else {
        None
    }
}

const LEGACY_GENERATED_DOC_PATHS: &[(&str, &str)] = &[
    ("docs/quickstart.md", "docs/QUICKSTART.md"),
    ("docs/source-map.md", "docs/SOURCE-MAP.md"),
    (
        "docs/architecture/overview.md",
        "docs/architecture/OVERVIEW.md",
    ),
    (
        "docs/architecture/decisions.md",
        "docs/architecture/DECISIONS.md",
    ),
    ("docs/domain/overview.md", "docs/domain/OVERVIEW.md"),
    ("docs/workflows/overview.md", "docs/workflows/OVERVIEW.md"),
    (
        "docs/data-models/overview.md",
        "docs/data-models/OVERVIEW.md",
    ),
    ("docs/api/overview.md", "docs/api/OVERVIEW.md"),
    ("docs/operations/runbook.md", "docs/operations/RUNBOOK.md"),
    ("docs/testing/strategy.md", "docs/testing/STRATEGY.md"),
    ("docs/glossary.md", "docs/GLOSSARY.md"),
    ("docs/open-questions.md", "docs/OPEN-QUESTIONS.md"),
    ("docs/evidence/sources.md", "docs/evidence/SOURCES.md"),
    ("docs/evidence/commands.md", "docs/evidence/COMMANDS.md"),
    ("docs/evidence/claims.md", "docs/evidence/CLAIMS.md"),
];

fn migrate_legacy_generated_docs(
    workspace_root: &Path,
    actions: &mut Vec<String>,
) -> Result<(), String> {
    for (legacy_relative, canonical_relative) in LEGACY_GENERATED_DOC_PATHS {
        let legacy = workspace_root.join(legacy_relative);
        let canonical = workspace_root.join(canonical_relative);
        migrate_legacy_generated_page(&legacy, &canonical, actions)?;
    }

    let areas_root = workspace_root.join("docs/areas");
    if areas_root.exists() {
        let entries = fs::read_dir(&areas_root)
            .map_err(|error| format!("failed to inspect `{}`: {error}", areas_root.display()))?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                format!("failed to inspect `{}`: {error}", areas_root.display())
            })?;
            if entry
                .file_type()
                .map_err(|error| {
                    format!("failed to inspect `{}`: {error}", entry.path().display())
                })?
                .is_dir()
            {
                migrate_legacy_generated_page(
                    &entry.path().join("overview.md"),
                    &entry.path().join("OVERVIEW.md"),
                    actions,
                )?;
            }
        }
    }
    Ok(())
}

fn migrate_legacy_generated_page(
    legacy: &Path,
    canonical: &Path,
    actions: &mut Vec<String>,
) -> Result<(), String> {
    if !has_exact_file_name(legacy)? || has_exact_file_name(canonical)? {
        return Ok(());
    }

    let content = fs::read_to_string(legacy)
        .map_err(|error| format!("failed to read `{}`: {error}", legacy.display()))?;
    if !content.contains(GENERATED_REGION_START) || !content.contains(GENERATED_REGION_END) {
        actions.push(format!(
            "preserved-human-owned-legacy-name: {}",
            legacy.display()
        ));
        return Ok(());
    }

    if let Some(parent) = canonical.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create `{}`: {error}", parent.display()))?;
    }
    rename_case_safely(legacy, canonical)?;
    actions.push(format!(
        "migrated-generated-page: {} -> {}",
        legacy.display(),
        canonical.display()
    ));
    Ok(())
}

fn has_exact_file_name(path: &Path) -> Result<bool, String> {
    let Some(parent) = path.parent() else {
        return Ok(false);
    };
    let Some(expected) = path.file_name() else {
        return Ok(false);
    };
    if !parent.exists() {
        return Ok(false);
    }
    let entries = fs::read_dir(parent)
        .map_err(|error| format!("failed to inspect `{}`: {error}", parent.display()))?;
    for entry in entries {
        let entry =
            entry.map_err(|error| format!("failed to inspect `{}`: {error}", parent.display()))?;
        if entry.file_name() == expected {
            return Ok(true);
        }
    }
    Ok(false)
}

fn rename_case_safely(source: &Path, target: &Path) -> Result<(), String> {
    let parent = source
        .parent()
        .ok_or_else(|| format!("legacy page has no parent: {}", source.display()))?;
    let file_name = source
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("page.md");
    let temporary = parent.join(format!(".codewiki-case-migration-{file_name}"));
    if has_exact_file_name(&temporary)? {
        return Err(format!(
            "cannot migrate `{}` because temporary path already exists: {}",
            source.display(),
            temporary.display()
        ));
    }
    fs::rename(source, &temporary).map_err(|error| {
        format!(
            "failed to stage legacy page `{}` for migration: {error}",
            source.display()
        )
    })?;
    if let Err(error) = fs::rename(&temporary, target) {
        let _ = fs::rename(&temporary, source);
        return Err(format!(
            "failed to migrate legacy page `{}` to `{}`: {error}",
            source.display(),
            target.display()
        ));
    }
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
        assert!(output.stdout.contains("companion_interface_version: 2"));
        assert!(
            output
                .stdout
                .contains(".agents/skills/codewiki/project/config.yml")
        );
        assert!(
            output
                .stdout
                .contains(".agents/skills/codewiki/project/AGENTS.md")
        );
    }

    #[test]
    fn help_mentions_init() {
        let output = run(["help"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("codewiki init [path]"));
        assert!(output.stdout.contains("codewiki sync [path]"));
        assert!(output.stdout.contains("codewiki doctor"));
        assert!(output.stdout.contains("codewiki validate [path]"));
    }

    #[test]
    fn unknown_command_fails() {
        let output = run(["wat"]);

        assert_eq!(output.exit_code, 2);
        assert!(output.stderr.contains("unknown command"));
    }

    #[test]
    fn doctor_verifies_manifest_and_detects_drift() {
        let root = temp_path("codewiki-doctor");
        fs::create_dir_all(root.join("references")).expect("mkdir skill");
        fs::write(
            root.join("package.yml"),
            "schema_version: 1\npackage_version: \"0.2.0\"\nskill_contract_version: 2\nreference_contract_version: 2\ncompanion_interface_version: 2\nwikiplan_schema_min: 2\nwikiplan_schema_max: 2\n",
        )
        .expect("write package");
        fs::write(root.join("SKILL.md"), "# Skill\n").expect("write skill");
        fs::write(root.join("references/init.md"), "# Init\n").expect("write reference");
        let digest = managed_skill_digest(&root).expect("digest");
        fs::write(
            root.join("INSTALLATION.yml"),
            format!(
                "schema_version: 1\npackage_version: \"0.2.0\"\nskill_contract_version: 2\nreference_contract_version: 2\nsource_revision: \"test\"\nmanaged_digest: \"{digest}\"\ncompanion_interface_version: 2\ninstall_scope: local\n"
            ),
        )
        .expect("write installation");

        let verified = diagnose_installation(&root).expect("verified install");
        assert!(verified.contains("state: verified"));

        fs::write(root.join("references/init.md"), "# Drifted\n").expect("mutate reference");
        let drift = diagnose_installation(&root).expect_err("drift must fail");
        assert!(drift.contains("content_drift"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn validate_requires_synthesized_docs_and_quality_report() {
        let base = temp_path("codewiki-validate");
        let repo = base.join("repo");
        fs::create_dir_all(repo.join("docs/conventions")).expect("mkdir docs");
        fs::create_dir_all(repo.join(".agents/skills/codewiki/project")).expect("mkdir control");
        fs::write(
            repo.join(".agents/skills/codewiki/project/plan.yml"),
            synthesized_test_plan(),
        )
        .expect("write plan");
        fs::write(
            repo.join(".agents/skills/codewiki/project/quality-report.yml"),
            "model_synthesis: pass\ncontract_coverage: pass\nsource_audit: pass\ndiagram_audit: pass\ncross_page_review: pass\ndocs_only_onboarding: pass\nreader_context: docs_only\nsource_auditor_context: source_and_evidence\ncritical_failures: 0\nrevision_attempts: 0\ngeneration_model: \"test-generator\"\nevaluation_model: \"test-evaluator\"\n",
        )
        .expect("write report");
        fs::write(
            repo.join(".agents/skills/codewiki/project/run.yml"),
            "schema_version: 1\ncompanion_interface_version: 2\ngeneration_status: synthesis_incomplete\nskill_installation:\n  state: verified\nstages:\n  discovery: complete\n  evidence_persistence: complete\n  repository_mental_model: pending\n  wikiplan: scaffold_only\n  page_synthesis: pending\n  quality: pending\n",
        )
        .expect("write run provenance");
        fs::write(
            repo.join("docs/QUICKSTART.md"),
            "# Quickstart\n\n## Purpose\n\nUnderstand the application.\n\n## Mental model\n\nThe application owns its runtime.\n\n## Reading paths\n\nRead the [repository conventions](./conventions/OVERVIEW.md) before changing code.\n",
        )
        .expect("write quickstart");
        fs::write(
            repo.join("docs/conventions/OVERVIEW.md"),
            "# Conventions\n\n## Purpose\n\nUse repository evidence.\n",
        )
        .expect("write conventions");
        let context = RuntimeContext {
            cwd: repo.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: PathBuf::from("sqlite3"),
        };

        let output = run_with_context(["validate"], &context);

        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert!(
            output
                .stdout
                .contains("generation_status: reader_docs_ready")
        );
        let run = fs::read_to_string(repo.join(".agents/skills/codewiki/project/run.yml"))
            .expect("read run");
        assert!(run.contains("generation_status: reader_docs_ready"));
        assert!(run.contains("state: verified"));
        assert!(run.contains("repository_mental_model: complete"));
        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn init_rejects_legacy_control_plane_conflicts() {
        let base = temp_path("codewiki-legacy-control");
        let repo = base.join("repo");
        fs::create_dir_all(repo.join(".codewiki")).expect("mkdir legacy control");
        let context = RuntimeContext {
            cwd: repo.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: PathBuf::from("sqlite3"),
        };

        let output = run_with_context(["init"], &context);

        assert_eq!(output.exit_code, 1);
        assert!(output.stderr.contains("legacy_control_plane"));
        assert!(!repo.join(".agents/skills/codewiki/project").exists());
        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn sync_preserves_and_enriches_v1_plan() {
        let sqlite = if Path::new("/usr/bin/sqlite3").exists() {
            PathBuf::from("/usr/bin/sqlite3")
        } else {
            PathBuf::from("sqlite3")
        };
        let base = temp_path("codewiki-plan-v1");
        let repo = base.join("repo");
        fs::create_dir_all(repo.join("src")).expect("mkdir src");
        fs::write(repo.join("src/main.rs"), "fn main() {}\n").expect("write source");
        let context = RuntimeContext {
            cwd: repo.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: sqlite,
        };
        assert_eq!(run_with_context(["init"], &context).exit_code, 0);
        let project = repo.join(".agents/skills/codewiki/project");
        let legacy = "schema_version: 1\nstatus: planned\npages:\n  - path: docs/QUICKSTART.md\n";
        fs::write(project.join("plan.yml"), legacy).expect("write v1 plan");

        let output = run_with_context(["sync"], &context);

        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert!(output.stdout.contains("preserved-legacy-plan"));
        assert_eq!(
            fs::read_to_string(project.join("plan.v1.legacy.yml")).expect("legacy backup"),
            legacy
        );
        let plan = fs::read_to_string(project.join("plan.yml")).expect("v2 plan");
        assert!(plan.contains("schema_version: 2"));
        assert!(plan.contains("preserved plan.v1.legacy.yml"));
        let _ = fs::remove_dir_all(base);
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
        assert!(
            repo.join(".agents/skills/codewiki/project/config.yml")
                .exists()
        );
        assert!(
            repo.join(".agents/skills/codewiki/project/plan.yml")
                .exists()
        );
        assert!(
            repo.join(".agents/skills/codewiki/project/AGENTS.md")
                .exists()
        );
        assert!(
            repo.join(".agents/skills/codewiki/project/sources.yml")
                .exists()
        );
        assert!(!repo.join("docs/QUICKSTART.md").exists());
        assert!(!repo.join("docs/SOURCE-MAP.md").exists());
        assert!(!repo.join("docs/architecture/OVERVIEW.md").exists());
        assert!(!repo.join("docs/conventions/OVERVIEW.md").exists());
        assert!(repo.join("docs/evidence/CLAIMS.md").exists());
        assert!(!has_exact_file_name(&repo.join("docs/quickstart.md")).expect("inspect legacy"));
        assert!(output.stdout.contains("migration_version: 1"));
        assert!(output.stdout.contains("claims_persisted:"));
        assert!(
            output
                .stdout
                .contains("generation_status: synthesis_incomplete")
        );
        assert!(
            fs::read_to_string(repo.join(".agents/skills/codewiki/project/plan.yml"))
                .expect("read plan")
                .contains("detected:")
        );
        assert!(
            fs::read_to_string(repo.join("docs/evidence/CLAIMS.md"))
                .expect("read claims")
                .contains("claim:")
        );
        assert!(
            fs::read_to_string(repo.join("docs/evidence/SOURCES.md"))
                .expect("read sources")
                .contains("src/lib.rs")
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
        fs::create_dir_all(repo.join(".agents/skills/codewiki/project"))
            .expect("create .agents/skills/codewiki/project");
        fs::write(
            repo.join(".agents/skills/codewiki/project/config.yml"),
            "custom: true\n",
        )
        .expect("write config");
        let context = RuntimeContext {
            cwd: repo.clone(),
            app_data_base: base.join("app-data"),
            cache_base: base.join("cache"),
            sqlite_executable: sqlite,
        };

        let output = run_with_context(["init"], &context);

        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert_eq!(
            fs::read_to_string(repo.join(".agents/skills/codewiki/project/config.yml"))
                .expect("read config"),
            "custom: true\n"
        );
        assert!(output.stdout.contains("preserved:"));

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn sync_noops_when_current_and_updates_generated_region_only() {
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

        let run_path = repo.join(".agents/skills/codewiki/project/run.yml");
        let ready_run = fs::read_to_string(&run_path).expect("run status").replace(
            "generation_status: synthesis_incomplete",
            "generation_status: reader_docs_ready",
        );
        fs::write(&run_path, ready_run).expect("mark prior synthesis ready");
        let quality_path = repo.join(".agents/skills/codewiki/project/quality-report.yml");
        fs::write(&quality_path, "model_synthesis: pass\n").expect("mark prior quality pass");

        let no_op = run_with_context(["sync"], &context);
        assert_eq!(no_op.exit_code, 0, "{}", no_op.stderr);
        assert!(no_op.stdout.contains("no-op"));
        assert!(no_op.stdout.contains("claims_persisted:"));
        assert!(
            fs::read_to_string(&run_path)
                .expect("preserved ready status")
                .contains("generation_status: reader_docs_ready")
        );

        let claims_path = repo.join("docs/evidence/CLAIMS.md");
        let existing_map = fs::read_to_string(&claims_path).expect("claims");
        fs::write(
            &claims_path,
            format!("human preface\n{existing_map}\nhuman notes\n"),
        )
        .expect("edit claims");
        fs::write(repo.join("src/main.rs"), "use std::fs;\nfn main() {}\n").expect("mutate source");
        let synced = run_with_context(["sync"], &context);
        assert_eq!(synced.exit_code, 0, "{}", synced.stderr);
        assert!(synced.stdout.contains("updated-generated-region:"));
        assert!(
            fs::read_to_string(&run_path)
                .expect("downgraded status")
                .contains("generation_status: synthesis_incomplete")
        );
        assert!(
            fs::read_to_string(&quality_path)
                .expect("reset quality")
                .contains("model_synthesis: pending")
        );
        let map = fs::read_to_string(&claims_path).expect("read claims");
        assert!(map.contains("human preface"));
        assert!(map.contains("human notes"));
        assert!(map.contains("# Claims"));
        let qa = codewiki_store::render_qa_context_with_sqlite(
            &context.sqlite_executable,
            find_state_db(&context),
            "std::fs",
            10,
        )
        .expect("dependency evidence query");
        assert!(qa.markdown.contains("std::fs"), "{}", qa.markdown);
        assert!(
            sqlite_count(
                &context.sqlite_executable,
                &find_state_db(&context),
                "sync_runs"
            ) >= 2
        );

        let manually_edited = map.replace("# Claims", "# Verified Claims (human correction)");
        fs::write(&claims_path, &manually_edited).expect("write manual generated-region edit");
        fs::write(
            repo.join("src/main.rs"),
            "use std::{fs, path::Path};\nfn main() {}\n",
        )
        .expect("mutate source again");
        let conflict = run_with_context(["sync"], &context);
        assert_eq!(conflict.exit_code, 0, "{}", conflict.stderr);
        assert!(
            conflict
                .stdout
                .contains("preserved-human-edited-generated-region:")
        );
        assert_eq!(
            fs::read_to_string(&claims_path).expect("read preserved claims"),
            manually_edited
        );

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn generated_region_merge_preserves_manual_body_edits() {
        let existing_generated = codewiki_docs::wrap_generated_region(
            "# Source Map\n\nHuman-correctable generated statement.",
        );
        let refreshed =
            codewiki_docs::wrap_generated_region("# Source Map\n\nRefreshed generated statement.");
        let with_surrounding_human_text =
            format!("Human preface.\n\n{existing_generated}\nHuman notes.\n");

        let merged = merge_generated_region(&with_surrounding_human_text, &refreshed);
        let GeneratedRegionMerge::Merged(merged) = merged else {
            panic!("unchanged generated body should refresh");
        };
        assert!(merged.contains("Human preface."));
        assert!(merged.contains("Human notes."));
        assert!(merged.contains("Refreshed generated statement."));

        let manually_edited = existing_generated.replace(
            "Human-correctable generated statement.",
            "User-authored correction that must win.",
        );
        assert_eq!(
            merge_generated_region(&manually_edited, &refreshed),
            GeneratedRegionMerge::HumanEdited
        );
    }

    #[test]
    fn generated_region_merge_preserves_hashless_legacy_regions() {
        let legacy = format!("{GENERATED_REGION_START}\n# Legacy body\n{GENERATED_REGION_END}\n");
        let refreshed = codewiki_docs::wrap_generated_region("# Refreshed body");

        assert_eq!(
            merge_generated_region(&legacy, &refreshed),
            GeneratedRegionMerge::LegacyUnverified
        );
        assert_eq!(
            merge_generated_region("# Human page\n", &refreshed),
            GeneratedRegionMerge::NoRegion
        );
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
        assert!(
            workspace
                .join(".agents/skills/codewiki/project/config.yml")
                .exists()
        );
        assert!(
            workspace
                .join(".agents/skills/codewiki/project/sources.yml")
                .exists()
        );
        assert!(!workspace.join("docs/QUICKSTART.md").exists());
        assert!(!workspace.join("docs/conventions/OVERVIEW.md").exists());
        assert!(
            fs::read_to_string(workspace.join("docs/evidence/SOURCES.md"))
                .expect("read sources")
                .contains("src/main.rs")
        );
        assert!(
            fs::read_to_string(workspace.join("docs/evidence/CLAIMS.md"))
                .expect("read claims")
                .contains("claim:")
        );
        assert!(
            !source
                .join(".agents/skills/codewiki/project/config.yml")
                .exists()
        );
        assert!(!source.join("docs/QUICKSTART.md").exists());
        assert!(
            fs::read_to_string(workspace.join(".agents/skills/codewiki/project/sources.yml"))
                .expect("read sources")
                .contains("primary: true")
        );

        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn migrates_only_marker_owned_lowercase_generated_pages() {
        let root = temp_path("codewiki-core-uppercase-migration");
        fs::create_dir_all(root.join("docs/areas/client")).expect("mkdir docs");
        let generated =
            format!("{GENERATED_REGION_START}\n# Legacy generated\n{GENERATED_REGION_END}\n");
        fs::write(root.join("docs/quickstart.md"), &generated).expect("write legacy quickstart");
        fs::write(root.join("docs/areas/client/overview.md"), &generated)
            .expect("write legacy area");
        fs::write(root.join("docs/glossary.md"), "# Human glossary\n")
            .expect("write human legacy page");
        let mut actions = Vec::new();

        migrate_legacy_generated_docs(&root, &mut actions).expect("migrate pages");

        assert!(root.join("docs/QUICKSTART.md").exists());
        assert!(!has_exact_file_name(&root.join("docs/quickstart.md")).expect("inspect legacy"));
        assert!(root.join("docs/areas/client/OVERVIEW.md").exists());
        assert!(root.join("docs/glossary.md").exists());
        assert!(!has_exact_file_name(&root.join("docs/GLOSSARY.md")).expect("inspect canonical"));
        assert!(
            actions
                .iter()
                .any(|action| action.starts_with("migrated-generated-page:"))
        );
        assert!(
            actions
                .iter()
                .any(|action| action.starts_with("preserved-human-owned-legacy-name:"))
        );

        let _ = fs::remove_dir_all(root);
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

    fn synthesized_test_plan() -> String {
        r#"schema_version: 2
status: synthesized
planner_contract_version: reader-first-v2
source_commit: "test"
source_dirty: false
visible_docs:
  []
repository_mental_model:
  systems:
    - "Application runtime"
  actors:
    - "Developer"
pages:
  - path: docs/QUICKSTART.md
    title: "Quickstart"
    page_type: overview
    section_id: quickstart
    parent_page: null
    order: 10
    importance: critical
    reader_job: "Understand the application"
    scope: "System entrypoint"
    out_of_scope: "Implementation reference"
    audiences:
      - "new_developer"
    prerequisites:
      []
    reader_questions:
      - "How does it work?"
    required_sections:
      - "purpose"
      - "mental_model"
    diagram_slots:
      []
    topic_ids:
      - "quickstart"
    source_anchors:
      - selector: "src/lib.rs"
        reason: "Runtime owner"
    evidence_gaps:
      []
    related_pages:
      - "docs/conventions/OVERVIEW.md"
    open_questions:
      []
    refresh_triggers:
      - "supporting_file_changed"
    acceptance_checks:
      - "Question answered"
  - path: docs/conventions/OVERVIEW.md
    title: "Conventions"
    page_type: reference
    section_id: conventions
    parent_page: docs/QUICKSTART.md
    order: 20
    importance: supporting
    reader_job: "Change code consistently"
    scope: "Repository conventions"
    out_of_scope: "Generic language advice"
    audiences:
      - "new_developer"
    prerequisites:
      - "docs/QUICKSTART.md"
    reader_questions:
      - "Which rules govern changes?"
    required_sections:
      - "purpose"
      - "change_guide"
    diagram_slots:
      []
    topic_ids:
      - "conventions"
    source_anchors:
      - selector: "Cargo.toml"
        reason: "Explicit build policy"
    evidence_gaps:
      []
    related_pages:
      - "docs/QUICKSTART.md"
    open_questions:
      []
    refresh_triggers:
      - "supporting_file_changed"
    acceptance_checks:
      - "Rules include scope and evidence"
"#
        .to_string()
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
