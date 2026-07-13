//! Durable state and cache boundary for CodeWiki.

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
}
