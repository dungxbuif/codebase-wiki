//! Generated wiki document boundary.

/// Planned generated docs layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiDocsLayout {
    /// Root folder for generated CodeWiki pages.
    pub generated_docs_root: &'static str,
}

/// Render the initial generated CodeWiki entrypoint.
pub fn render_initial_index(repo_label: &str) -> String {
    format!(
        "# CodeWiki: {repo_label}\n\n\
         This is the generated CodeWiki entrypoint for this repository.\n\n\
         ## Status\n\n\
         - State: initialized\n\
         - Semantic exploration: pending\n\
         - Full WikiPlan: pending\n\n\
         ## How To Use This Wiki\n\n\
         Start here, then follow links to generated pages as they are added. \
         CodeWiki answers should use `docs/codewiki/**` first, then `.codewiki/plan.yml`, \
         `.codewiki/AGENTS.md`, local SQLite evidence, source files, Git history, and optional providers only when needed.\n\n\
         ## Current Coverage\n\n\
         - Initial control files are present.\n\
         - Durable local SQLite state is initialized.\n\
         - Repository detection and semantic documentation are not complete yet.\n\n\
         ## Next Pages\n\n\
         - `map.md`\n\
         - `architecture.md`\n\
         - `evidence/claims.md`\n\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_index_mentions_docs_first_order() {
        let index = render_initial_index("example");

        assert!(index.contains("# CodeWiki: example"));
        assert!(index.contains("docs/codewiki/**"));
        assert!(index.contains("Semantic exploration: pending"));
    }
}

impl Default for WikiDocsLayout {
    fn default() -> Self {
        Self {
            generated_docs_root: "docs/codewiki",
        }
    }
}
