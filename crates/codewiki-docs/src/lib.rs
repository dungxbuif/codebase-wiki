//! Generated wiki document boundary.

/// Planned generated docs layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiDocsLayout {
    /// Root folder for generated CodeWiki pages.
    pub generated_docs_root: &'static str,
}

/// A generated documentation page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedPage {
    /// Repo-relative page path.
    pub path: String,
    /// Markdown content.
    pub content: String,
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

/// Render the canonical initial page set.
pub fn render_initial_pages(repo_label: &str, detection_markdown: &str) -> Vec<GeneratedPage> {
    vec![
        GeneratedPage {
            path: "docs/codewiki/index.md".to_string(),
            content: render_initial_index_with_detection(repo_label, detection_markdown),
        },
        GeneratedPage {
            path: "docs/codewiki/map.md".to_string(),
            content: format!(
                "# Repository Map\n\n## Detected Signals\n\n{detection_markdown}\n## Notes\n\nFull semantic area mapping is pending. This page is the canonical home for package, service, app, and bounded-context navigation.\n"
            ),
        },
        GeneratedPage {
            path: "docs/codewiki/architecture.md".to_string(),
            content: "# Architecture\n\nArchitecture synthesis is pending. Future sync runs should record runtime components, dependency direction, constraints, and change risks here.\n\n## Current Evidence\n\nSee `evidence/sources.md` and `.codewiki/plan.yml` for detected repository signals.\n".to_string(),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/README.md".to_string(),
            content: "# Evidence\n\nThis directory records how generated CodeWiki claims are supported.\n\n- `sources.md`: inspected source/docs/provider artifacts.\n- `claims.md`: durable claims and confidence.\n- `commands.md`: verification commands and summarized results.\n".to_string(),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/sources.md".to_string(),
            content: format!(
                "# Evidence Sources\n\n## Initial Detection\n\n{detection_markdown}\nAdditional source evidence will be recorded during semantic exploration and sync.\n"
            ),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/claims.md".to_string(),
            content: "# Claims\n\nNo durable semantic claims have been promoted yet beyond initial repository detection. Future claims must include evidence and confidence.\n".to_string(),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/commands.md".to_string(),
            content: "# Commands\n\nNo repository verification commands have been recorded yet.\n".to_string(),
        },
    ]
}

fn render_initial_index_with_detection(repo_label: &str, detection_markdown: &str) -> String {
    let mut index = render_initial_index(repo_label);
    index.push_str("## Detected Repository Signals\n\n");
    index.push_str(detection_markdown);
    index
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

    #[test]
    fn initial_pages_include_canonical_evidence_pages() {
        let pages = render_initial_pages("example", "### Languages\n\n- Rust\n");
        let paths: Vec<_> = pages.iter().map(|page| page.path.as_str()).collect();

        assert!(paths.contains(&"docs/codewiki/index.md"));
        assert!(paths.contains(&"docs/codewiki/map.md"));
        assert!(paths.contains(&"docs/codewiki/architecture.md"));
        assert!(paths.contains(&"docs/codewiki/evidence/claims.md"));
        assert!(pages.iter().any(|page| {
            page.content
                .contains("Full semantic area mapping is pending")
        }));
    }
}

impl Default for WikiDocsLayout {
    fn default() -> Self {
        Self {
            generated_docs_root: "docs/codewiki",
        }
    }
}
