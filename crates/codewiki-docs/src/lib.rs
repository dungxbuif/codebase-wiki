//! Generated wiki document boundary.

use codewiki_explore::{ExplorationSnapshot, promote_claims_from_snapshot};

/// Start marker for generated page regions.
pub const GENERATED_REGION_START: &str = "<!-- codewiki:generated:start -->";
/// End marker for generated page regions.
pub const GENERATED_REGION_END: &str = "<!-- codewiki:generated:end -->";

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
    render_initial_pages_with_exploration(repo_label, detection_markdown, None)
}

/// Render the canonical initial page set with semantic exploration evidence.
pub fn render_semantic_pages(
    repo_label: &str,
    detection_markdown: &str,
    exploration: &ExplorationSnapshot,
) -> Vec<GeneratedPage> {
    render_initial_pages_with_exploration(repo_label, detection_markdown, Some(exploration))
}

fn render_initial_pages_with_exploration(
    repo_label: &str,
    detection_markdown: &str,
    exploration: Option<&ExplorationSnapshot>,
) -> Vec<GeneratedPage> {
    let semantic_markdown = exploration.map(ExplorationSnapshot::to_markdown);
    let semantic_markdown = semantic_markdown.as_deref();

    let pages = vec![
        GeneratedPage {
            path: "docs/codewiki/index.md".to_string(),
            content: render_initial_index_with_detection(
                repo_label,
                detection_markdown,
                semantic_markdown,
            ),
        },
        GeneratedPage {
            path: "docs/codewiki/map.md".to_string(),
            content: render_map_page(detection_markdown, semantic_markdown),
        },
        GeneratedPage {
            path: "docs/codewiki/architecture.md".to_string(),
            content: render_architecture_page(semantic_markdown),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/README.md".to_string(),
            content: "# Evidence\n\nThis directory records how generated CodeWiki claims are supported.\n\n- `sources.md`: inspected source/docs/provider artifacts.\n- `claims.md`: durable claims and confidence.\n- `commands.md`: verification commands and summarized results.\n".to_string(),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/sources.md".to_string(),
            content: render_sources_page(detection_markdown, exploration),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/claims.md".to_string(),
            content: render_claims_page(exploration),
        },
        GeneratedPage {
            path: "docs/codewiki/evidence/commands.md".to_string(),
            content: "# Commands\n\nNo repository verification commands have been recorded yet.\n".to_string(),
        },
    ];
    pages
        .into_iter()
        .map(|page| GeneratedPage {
            path: page.path,
            content: wrap_generated_region(&page.content),
        })
        .collect()
}

/// Wrap generated content in markers so sync can preserve human-owned text around it.
pub fn wrap_generated_region(content: &str) -> String {
    format!("{GENERATED_REGION_START}\n{content}\n{GENERATED_REGION_END}\n")
}

fn render_initial_index_with_detection(
    repo_label: &str,
    detection_markdown: &str,
    semantic_markdown: Option<&str>,
) -> String {
    let mut index = render_initial_index(repo_label);
    if semantic_markdown.is_some() {
        index = index
            .replace("- Semantic exploration: pending", "- Semantic exploration: initialized")
            .replace(
                "- Repository detection and semantic documentation are not complete yet.",
                "- Repository detection and semantic snapshot are initialized; deeper synthesis remains evidence-gated.",
            );
    }
    index.push_str("## Detected Repository Signals\n\n");
    index.push_str(detection_markdown);
    if let Some(semantic_markdown) = semantic_markdown {
        index.push_str("\n## Semantic Snapshot\n\n");
        index.push_str(semantic_markdown);
    }
    index
}

fn render_map_page(detection_markdown: &str, semantic_markdown: Option<&str>) -> String {
    let mut content = format!("# Repository Map\n\n## Detected Signals\n\n{detection_markdown}");
    if let Some(semantic_markdown) = semantic_markdown {
        content.push_str("\n## Semantic Structure\n\n");
        content.push_str(semantic_markdown);
        content.push_str("\n\n## Notes\n\nThis page is generated from bounded filesystem exploration. Treat dependency rows as hints until promoted into durable claims.\n");
    } else {
        content.push_str("\n## Notes\n\nFull semantic area mapping is pending. This page is the canonical home for package, service, app, and bounded-context navigation.\n");
    }
    content
}

fn render_architecture_page(semantic_markdown: Option<&str>) -> String {
    let mut content = "# Architecture\n\n".to_string();
    if let Some(semantic_markdown) = semantic_markdown {
        content.push_str("Architecture synthesis starts from the semantic snapshot below. Future sync runs should promote stable structure into explicit claims with confidence and evidence.\n\n");
        content.push_str("## Current Semantic Evidence\n\n");
        content.push_str(semantic_markdown);
        content.push_str("\n\n## Interpretation Status\n\n- Component boundaries: evidence-backed hints, not final claims.\n- Dependency direction: lexical import hints only.\n- Runtime behavior: pending deeper exploration and/or provider activation when needed.\n");
    } else {
        content.push_str("Architecture synthesis is pending. Future sync runs should record runtime components, dependency direction, constraints, and change risks here.\n\n## Current Evidence\n\nSee `evidence/sources.md` and `.codewiki/plan.yml` for detected repository signals.\n");
    }
    content
}

fn render_sources_page(
    detection_markdown: &str,
    exploration: Option<&ExplorationSnapshot>,
) -> String {
    let mut content = format!("# Evidence Sources\n\n## Initial Detection\n\n{detection_markdown}");
    match exploration {
        Some(snapshot) => {
            content.push_str("\n## Explored Files\n\n");
            for evidence in snapshot.evidence.iter().take(200) {
                content.push_str(&format!(
                    "- `{}`: `{}` ({})\n",
                    evidence.id, evidence.path, evidence.kind
                ));
            }
            if snapshot.evidence.is_empty() {
                content.push_str("- none\n");
            }
            if snapshot.truncated {
                content.push_str(&format!(
                    "\nTraversal reached the configured file limit of {} files.\n",
                    snapshot.file_limit
                ));
            }
        }
        None => {
            content.push_str(
                "Additional source evidence will be recorded during semantic exploration and sync.\n",
            );
        }
    }
    content
}

fn render_claims_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut content = "# Claims\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            let claims = promote_claims_from_snapshot(snapshot);
            content.push_str("These deterministic claims were promoted from semantic exploration evidence. They are source-backed structure claims, not complete architecture conclusions.\n\n");
            if claims.is_empty() {
                content.push_str("No durable semantic claims have been promoted yet.\n");
                return content;
            }
            for claim in claims {
                content.push_str(&format!(
                    "- `{}` [{}]: {}\n",
                    claim.id, claim.confidence, claim.statement
                ));
                for evidence_id in claim.evidence_ids {
                    content.push_str(&format!("  - evidence: `{evidence_id}`\n"));
                }
            }
        }
        None => {
            content.push_str("No durable semantic claims have been promoted yet beyond initial repository detection. Future claims must include evidence and confidence.\n");
        }
    }
    content
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
        assert!(
            pages
                .iter()
                .all(|page| page.content.contains(GENERATED_REGION_START))
        );
    }

    #[test]
    fn semantic_pages_include_exploration_snapshot() {
        let snapshot = ExplorationSnapshot {
            schema_version: 1,
            files: vec![codewiki_explore::ExploredFile {
                path: "src/lib.rs".to_string(),
                language: Some("Rust".to_string()),
                role: codewiki_explore::FileRole::Source,
                line_count: 10,
                content_hash: "hash:test".to_string(),
                symbols: vec![codewiki_explore::ExploredSymbol {
                    name: "build".to_string(),
                    kind: "function".to_string(),
                    line: 1,
                }],
                imports: vec!["std::fs".to_string()],
                evidence_id: "file:test".to_string(),
            }],
            areas: vec![codewiki_explore::AreaSummary {
                name: "src".to_string(),
                file_count: 1,
                symbol_count: 1,
                roles: vec![codewiki_explore::FileRole::Source],
            }],
            dependency_hints: vec![codewiki_explore::DependencyHint {
                from_path: "src/lib.rs".to_string(),
                target: "std::fs".to_string(),
                kind: "lexical-import".to_string(),
            }],
            evidence: vec![codewiki_explore::EvidenceRef {
                id: "file:test".to_string(),
                path: "src/lib.rs".to_string(),
                kind: "file".to_string(),
            }],
            truncated: false,
            file_limit: 3_000,
        };

        let pages = render_semantic_pages("example", "### Languages\n\n- Rust\n", &snapshot);

        assert!(pages.iter().any(|page| {
            page.path == "docs/codewiki/map.md" && page.content.contains("Semantic Structure")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/codewiki/evidence/sources.md" && page.content.contains("file:test")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/codewiki/evidence/claims.md"
                && page.content.contains("claim:")
                && page.content.contains("evidence: `file:test`")
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
