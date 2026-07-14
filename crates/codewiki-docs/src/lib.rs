//! Generated wiki document boundary.

use codewiki_explore::{ExplorationSnapshot, promote_claims_from_snapshot};
use std::collections::BTreeMap;

/// Start marker for generated page regions.
pub const GENERATED_REGION_START: &str = "<!-- codewiki:generated:start -->";
/// Prefix for the portable integrity hash of the last CodeWiki-owned body.
pub const GENERATED_REGION_HASH_PREFIX: &str = "<!-- codewiki:generated:hash ";
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
        "# {repo_label} quickstart\n\n\
         This is the generated CodeWiki entrypoint for this repository. \
         It is the human and future-agent starting point for understanding what the project does, how the wiki is organized, and where to go next.\n\n\
         ## Status\n\n\
         - State: initialized\n\
         - Semantic exploration: pending\n\
         - Full WikiPlan: pending\n\n\
         ## Start Here\n\n\
         CodeWiki answers should use `docs/**` first, then `.agents/skills/codewiki/project/plan.yml`, \
         `.agents/skills/codewiki/project/AGENTS.md`, local SQLite evidence, source files, Git history, and optional providers only when needed.\n\n\
         - [Source map](./SOURCE-MAP.md)\n\
         - [Architecture overview](./architecture/OVERVIEW.md)\n\
         - [Domain overview](./domain/OVERVIEW.md)\n\
         - [Workflows](./workflows/OVERVIEW.md)\n\
         - [Data models](./data-models/OVERVIEW.md)\n\
         - [API and interfaces](./api/OVERVIEW.md)\n\
         - [Operations runbook](./operations/RUNBOOK.md)\n\
         - [Testing strategy](./testing/STRATEGY.md)\n\n\
         - [Code conventions](./conventions/OVERVIEW.md)\n\n\
         ## Current Coverage\n\n\
         - Initial control files are present.\n\
         - Durable local SQLite state is initialized.\n\
         - Repository detection and semantic documentation are not complete yet.\n\n\
         ## Notes For Future Agents\n\n\
         - Treat this wiki as a synthesis layer, not a raw file inventory.\n\
         - Prefer updating the canonical page for a concept instead of duplicating the same explanation elsewhere.\n\
         - Preserve human-owned content around CodeWiki generated regions.\n\
         - If a page would be a stub, keep the item in the backlog instead of creating a thin file.\n\n\
         ## Backlog\n\n\
         - Deeper semantic synthesis is pending first full exploration.\n\n"
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
            path: "docs/QUICKSTART.md".to_string(),
            content: render_initial_index_with_detection(
                repo_label,
                detection_markdown,
                semantic_markdown,
            ),
        },
        GeneratedPage {
            path: "docs/SOURCE-MAP.md".to_string(),
            content: render_map_page(detection_markdown, semantic_markdown),
        },
        GeneratedPage {
            path: "docs/architecture/OVERVIEW.md".to_string(),
            content: render_architecture_page(semantic_markdown),
        },
        GeneratedPage {
            path: "docs/domain/OVERVIEW.md".to_string(),
            content: render_domains_page(exploration),
        },
        GeneratedPage {
            path: "docs/workflows/OVERVIEW.md".to_string(),
            content: render_workflows_page(exploration),
        },
        GeneratedPage {
            path: "docs/data-models/OVERVIEW.md".to_string(),
            content: render_data_page(exploration),
        },
        GeneratedPage {
            path: "docs/api/OVERVIEW.md".to_string(),
            content: render_interfaces_page(exploration),
        },
        GeneratedPage {
            path: "docs/operations/RUNBOOK.md".to_string(),
            content: render_operations_page(exploration),
        },
        GeneratedPage {
            path: "docs/testing/STRATEGY.md".to_string(),
            content: render_testing_page(exploration),
        },
        GeneratedPage {
            path: "docs/conventions/OVERVIEW.md".to_string(),
            content: render_conventions_page(exploration),
        },
        GeneratedPage {
            path: "docs/architecture/DECISIONS.md".to_string(),
            content: render_decisions_page(exploration),
        },
        GeneratedPage {
            path: "docs/GLOSSARY.md".to_string(),
            content: render_glossary_page(exploration),
        },
        GeneratedPage {
            path: "docs/OPEN-QUESTIONS.md".to_string(),
            content: render_open_questions_page(exploration),
        },
        GeneratedPage {
            path: "docs/evidence/README.md".to_string(),
            content: "# Evidence\n\nThis directory records how generated CodeWiki claims are supported.\n\n- `SOURCES.md`: inspected source/docs/provider artifacts.\n- `CLAIMS.md`: durable claims and confidence.\n- `COMMANDS.md`: verification commands and summarized results.\n".to_string(),
        },
        GeneratedPage {
            path: "docs/evidence/SOURCES.md".to_string(),
            content: render_sources_page(detection_markdown, exploration),
        },
        GeneratedPage {
            path: "docs/evidence/CLAIMS.md".to_string(),
            content: render_claims_page(exploration),
        },
        GeneratedPage {
            path: "docs/evidence/COMMANDS.md".to_string(),
            content: "# Commands\n\nNo repository verification commands have been recorded yet.\n".to_string(),
        },
    ];
    let mut pages = pages;
    if let Some(snapshot) = exploration {
        for area in &snapshot.areas {
            pages.push(GeneratedPage {
                path: format!("docs/areas/{}/OVERVIEW.md", slugify(&area.name)),
                content: render_area_page(snapshot, &area.name),
            });
        }
    }
    pages
        .into_iter()
        .map(|page| GeneratedPage {
            path: page.path.clone(),
            content: wrap_generated_region(&add_relevant_source_files(
                page.path.as_str(),
                &page.content,
                exploration,
            )),
        })
        .collect()
}

fn add_relevant_source_files(
    page_path: &str,
    content: &str,
    exploration: Option<&ExplorationSnapshot>,
) -> String {
    if page_path.starts_with("docs/evidence/") {
        return content.to_string();
    }

    let Some(snapshot) = exploration else {
        return content.to_string();
    };

    let paths = relevant_paths_for_page(page_path, snapshot);
    if paths.is_empty() {
        return content.to_string();
    }

    let mut out = "<details>\n<summary>Relevant source files</summary>\n\n".to_string();
    out.push_str("The following files were used as context for generating this wiki page:\n\n");
    for path in paths {
        out.push_str(&format!("- `{path}`\n"));
    }
    out.push_str("</details>\n\n");
    out.push_str(content);
    out
}

fn relevant_paths_for_page(page_path: &str, snapshot: &ExplorationSnapshot) -> Vec<String> {
    let mut paths: Vec<String> = snapshot
        .files
        .iter()
        .filter(|file| matches_page_focus(page_path, file))
        .take(12)
        .map(|file| file.path.clone())
        .collect();

    if paths.len() < 5 {
        for file in snapshot.files.iter().take(12) {
            if !paths.contains(&file.path) {
                paths.push(file.path.clone());
            }
            if paths.len() >= 5 {
                break;
            }
        }
    }

    paths
}

fn matches_page_focus(page_path: &str, file: &codewiki_explore::ExploredFile) -> bool {
    let lower = file.path.to_lowercase();
    match page_path {
        "docs/QUICKSTART.md" | "docs/SOURCE-MAP.md" => true,
        "docs/architecture/OVERVIEW.md" => {
            file.role.as_str() == "source" || file.role.as_str() == "config"
        }
        "docs/domain/OVERVIEW.md" => {
            lower.contains("domain") || lower.contains("model") || lower.contains("service")
        }
        "docs/workflows/OVERVIEW.md" => {
            lower.contains("workflow")
                || lower.contains("job")
                || lower.contains("event")
                || lower.contains("main")
                || lower.contains("app")
                || lower.contains("index")
        }
        "docs/data-models/OVERVIEW.md" => {
            lower.contains("schema")
                || lower.contains("model")
                || lower.contains("migration")
                || lower.ends_with(".sql")
                || lower.contains("data")
        }
        "docs/api/OVERVIEW.md" => {
            !file.symbols.is_empty() || lower.contains("api") || lower.contains("route")
        }
        "docs/operations/RUNBOOK.md" => {
            file.role.as_str() == "config" || file.role.as_str() == "documentation"
        }
        "docs/testing/STRATEGY.md" => file.role.as_str() == "test",
        "docs/conventions/OVERVIEW.md" => is_convention_source(file),
        "docs/architecture/DECISIONS.md" => {
            file.role.as_str() == "documentation" || lower.contains("adr")
        }
        "docs/GLOSSARY.md" | "docs/OPEN-QUESTIONS.md" => true,
        path if path.starts_with("docs/areas/") => {
            let area = path
                .trim_start_matches("docs/areas/")
                .trim_end_matches("/OVERVIEW.md");
            slugify(file.path.split('/').next().unwrap_or_default()) == area
        }
        _ => true,
    }
}

/// Wrap generated content in markers so sync can preserve human-owned text around it.
pub fn wrap_generated_region(content: &str) -> String {
    let body = content.trim_end();
    let hash = generated_region_hash(body);
    format!(
        "{GENERATED_REGION_START}\n{GENERATED_REGION_HASH_PREFIX}{hash} -->\n{body}\n{GENERATED_REGION_END}\n"
    )
}

/// Return a stable portable hash for a generated region body.
pub fn generated_region_hash(content: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in content.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
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
        content.push_str("Architecture synthesis is pending. Future sync runs should record runtime components, dependency direction, constraints, and change risks here.\n\n## Current Evidence\n\nSee `evidence/SOURCES.md` and `.agents/skills/codewiki/project/plan.yml` for detected repository signals.\n");
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

fn render_domains_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Domains\n\n".to_string();
    match exploration {
        Some(snapshot) if !snapshot.areas.is_empty() => {
            out.push_str("Top-level repository areas are treated as initial domain or subsystem candidates. These are structural hints until promoted by deeper analysis.\n\n");
            for area in &snapshot.areas {
                out.push_str(&format!(
                    "- `{}`: {} files, {} symbols; roles: {}\n",
                    area.name,
                    area.file_count,
                    area.symbol_count,
                    area.roles
                        .iter()
                        .map(|role| role.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
        _ => out.push_str("No domain candidates were detected yet.\n"),
    }
    out
}

fn render_workflows_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Workflows\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            out.push_str("Workflow seeds are inferred from entry-like files, tests, and dependency hints. Runtime flow remains pending until verified by source reading or commands.\n\n");
            for file in snapshot.files.iter().filter(|file| {
                file.path.contains("main")
                    || file.path.contains("app")
                    || file.path.contains("index")
                    || file.role.as_str() == "test"
            }) {
                out.push_str(&format!(
                    "- `{}`: {} symbols, {} import/dependency hints; evidence `{}`\n",
                    file.path,
                    file.symbols.len(),
                    file.imports.len(),
                    file.evidence_id
                ));
            }
        }
        None => out.push_str("Workflow synthesis is pending semantic exploration.\n"),
    }
    out
}

fn render_data_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Data\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            let mut matched = false;
            for file in snapshot.files.iter().filter(|file| {
                let path = file.path.to_lowercase();
                path.contains("schema")
                    || path.contains("model")
                    || path.contains("migration")
                    || path.ends_with(".sql")
                    || path.contains("data")
            }) {
                matched = true;
                out.push_str(&format!(
                    "- `{}`: data-related candidate; evidence `{}`\n",
                    file.path, file.evidence_id
                ));
            }
            if !matched {
                out.push_str("No explicit data/schema files were detected. Data model remains an open question.\n");
            }
        }
        None => out.push_str("Data synthesis is pending semantic exploration.\n"),
    }
    out
}

fn render_interfaces_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Interfaces\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            out.push_str(
                "Interface candidates come from exported/public symbols and dependency hints.\n\n",
            );
            for file in snapshot.files.iter().take(50) {
                let public_symbols: Vec<_> = file
                    .symbols
                    .iter()
                    .filter(|symbol| {
                        matches!(
                            symbol.kind.as_str(),
                            "function" | "class" | "interface" | "type" | "struct" | "trait"
                        )
                    })
                    .take(10)
                    .collect();
                if public_symbols.is_empty() && file.imports.is_empty() {
                    continue;
                }
                out.push_str(&format!("- `{}`\n", file.path));
                for symbol in public_symbols {
                    out.push_str(&format!(
                        "  - symbol `{}` ({}) at line {}\n",
                        symbol.name, symbol.kind, symbol.line
                    ));
                }
                for import in file.imports.iter().take(5) {
                    out.push_str(&format!("  - dependency hint `{import}`\n"));
                }
            }
        }
        None => out.push_str("Interface synthesis is pending semantic exploration.\n"),
    }
    out
}

fn render_operations_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Operations\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            out.push_str("Operational evidence is inferred from manifests, configs, build files, and docs.\n\n");
            for file in snapshot.files.iter().filter(|file| {
                file.role.as_str() == "config" || file.role.as_str() == "documentation"
            }) {
                out.push_str(&format!(
                    "- `{}`: {} evidence `{}`\n",
                    file.path,
                    file.role.as_str(),
                    file.evidence_id
                ));
            }
        }
        None => out.push_str("Operations synthesis is pending semantic exploration.\n"),
    }
    out
}

fn render_testing_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Testing\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            let tests: Vec<_> = snapshot
                .files
                .iter()
                .filter(|file| file.role.as_str() == "test")
                .collect();
            if tests.is_empty() {
                out.push_str("No test files were detected in the bounded exploration snapshot.\n");
            } else {
                for file in tests {
                    out.push_str(&format!(
                        "- `{}`: {} symbols; evidence `{}`\n",
                        file.path,
                        file.symbols.len(),
                        file.evidence_id
                    ));
                }
            }
        }
        None => out.push_str("Testing synthesis is pending semantic exploration.\n"),
    }
    out
}

fn render_conventions_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Code Conventions\n\n".to_string();
    out.push_str("This page records repository-specific conventions discovered from configuration, documentation, repeated code patterns, tests, and explicit exceptions. Ecosystem defaults are not project conventions unless repository evidence shows adoption.\n\n");
    out.push_str("## Evidence Standard\n\n- `explicit`: enforced or stated by authoritative repository configuration or documentation.\n- `inferred`: supported by at least two independent examples in the inspected scope.\n- `hypothesis`: plausible but supported by one example or incomplete coverage.\n- `exception`: a deliberate or legacy deviation from a supported convention.\n\n");

    let Some(snapshot) = exploration else {
        out.push_str("## Convention Discovery Status\n\nSemantic exploration has not run yet. Inspect explicit convention sources and representative code before promoting repository conventions.\n");
        return out;
    };

    out.push_str("## Explicit Convention Sources\n\n");
    let explicit_sources: Vec<_> = snapshot
        .files
        .iter()
        .filter(|file| is_convention_source(file))
        .take(50)
        .collect();
    if explicit_sources.is_empty() {
        out.push_str("- No explicit convention source was detected in the bounded snapshot.\n");
    } else {
        for file in explicit_sources {
            out.push_str(&format!(
                "- `{}`: {} candidate; evidence `{}`\n",
                file.path,
                file.role.as_str(),
                file.evidence_id
            ));
        }
    }

    out.push_str("\n## Repeated Pattern Candidates\n\nThese candidates require LLM source inspection, counterexample search, scope classification, and confidence before they become conventions.\n\n");
    let mut dependency_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for hint in &snapshot.dependency_hints {
        *dependency_counts.entry(hint.target.as_str()).or_default() += 1;
    }
    let repeated_dependencies: Vec<_> = dependency_counts
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .take(20)
        .collect();
    if repeated_dependencies.is_empty() {
        out.push_str("- No repeated dependency hint reached the two-file candidate threshold.\n");
    } else {
        for (target, count) in repeated_dependencies {
            out.push_str(&format!(
                "- `{target}` appears in {count} inspected files; verify whether its usage pattern forms a scoped convention.\n"
            ));
        }
    }
    for area in snapshot.areas.iter().filter(|area| area.file_count >= 2) {
        out.push_str(&format!(
            "- Area `{}` contains {} inspected files and may have area-specific conventions requiring representative sampling.\n",
            area.name, area.file_count
        ));
    }

    out.push_str("\n## Required LLM Synthesis\n\nFor each confirmed convention, record scope, classification, evidence paths/symbols or commands, confidence, exceptions, and change impact. Cover project structure, language and framework usage, naming, errors, async/state/data, dependencies, APIs, tests, configuration, security, and documentation only where evidence exists. Do not convert a single example or generic best practice into a repository rule.\n");
    out
}

fn is_convention_source(file: &codewiki_explore::ExploredFile) -> bool {
    let path = file.path.to_lowercase();
    file.role.as_str() == "config"
        || file.role.as_str() == "documentation"
        || file.role.as_str() == "test"
        || path.contains("lint")
        || path.contains("format")
        || path.contains("style")
        || path.contains("convention")
        || path.contains("contributing")
        || path.contains("editorconfig")
        || path.contains("clippy")
        || path.contains("eslint")
        || path.contains("prettier")
        || path.contains("biome")
}

fn render_decisions_page(_exploration: Option<&ExplorationSnapshot>) -> String {
    "# Decisions\n\nNo repository-specific architecture decisions have been promoted yet. Record future decisions here only when they are backed by source evidence, existing docs, or explicit human input.\n".to_string()
}

fn render_glossary_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Glossary\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            for area in &snapshot.areas {
                out.push_str(&format!("- `{}`: top-level area candidate\n", area.name));
            }
            for file in snapshot.files.iter().take(50) {
                for symbol in file.symbols.iter().take(10) {
                    out.push_str(&format!(
                        "- `{}`: {} from `{}` line {}\n",
                        symbol.name, symbol.kind, file.path, symbol.line
                    ));
                }
            }
        }
        None => out.push_str("Glossary generation is pending semantic exploration.\n"),
    }
    out
}

fn render_open_questions_page(exploration: Option<&ExplorationSnapshot>) -> String {
    let mut out = "# Open Questions\n\n".to_string();
    match exploration {
        Some(snapshot) => {
            if snapshot.truncated {
                out.push_str("- Exploration hit the file limit; coverage is incomplete.\n");
            }
            if snapshot
                .files
                .iter()
                .all(|file| file.role.as_str() != "test")
            {
                out.push_str("- No tests were detected; verification strategy is unknown.\n");
            }
            out.push_str("- Runtime behavior needs command evidence or deeper source analysis before being treated as confirmed.\n");
            out.push_str("- Domain boundaries are inferred from paths and should be reviewed during deeper synthesis.\n");
        }
        None => out.push_str("- Semantic exploration has not run yet.\n"),
    }
    out
}

fn render_area_page(snapshot: &ExplorationSnapshot, area_name: &str) -> String {
    let mut out = format!("# Area: `{area_name}`\n\n");
    out.push_str("This area page is generated from bounded semantic exploration evidence.\n\n");
    for file in snapshot
        .files
        .iter()
        .filter(|file| file.path.split('/').next() == Some(area_name))
        .take(50)
    {
        out.push_str(&format!(
            "- `{}`: {}, {} symbols, {} imports; evidence `{}`\n",
            file.path,
            file.role.as_str(),
            file.symbols.len(),
            file.imports.len(),
            file.evidence_id
        ));
    }
    out
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "area".to_string()
    } else {
        slug.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_index_mentions_docs_first_order() {
        let index = render_initial_index("example");

        assert!(index.contains("# example quickstart"));
        assert!(index.contains("docs/**"));
        assert!(index.contains("Semantic exploration: pending"));
    }

    #[test]
    fn initial_pages_include_canonical_evidence_pages() {
        let pages = render_initial_pages("example", "### Languages\n\n- Rust\n");
        let paths: Vec<_> = pages.iter().map(|page| page.path.as_str()).collect();

        assert!(paths.contains(&"docs/QUICKSTART.md"));
        assert!(paths.contains(&"docs/SOURCE-MAP.md"));
        assert!(paths.contains(&"docs/architecture/OVERVIEW.md"));
        assert!(paths.contains(&"docs/domain/OVERVIEW.md"));
        assert!(paths.contains(&"docs/workflows/OVERVIEW.md"));
        assert!(paths.contains(&"docs/api/OVERVIEW.md"));
        assert!(paths.contains(&"docs/conventions/OVERVIEW.md"));
        assert!(paths.contains(&"docs/OPEN-QUESTIONS.md"));
        assert!(paths.contains(&"docs/evidence/CLAIMS.md"));
        assert!(!paths.iter().any(|path| path.ends_with("quickstart.md")));
        assert!(pages.iter().any(|page| {
            page.content
                .contains("Full semantic area mapping is pending")
        }));
        assert!(
            pages
                .iter()
                .all(|page| page.content.contains(GENERATED_REGION_START))
        );
        assert!(pages.iter().all(|page| {
            page.content
                .contains("<!-- codewiki:generated:hash fnv1a64:")
        }));
    }

    #[test]
    fn generated_region_hash_is_stable_and_tracks_body_changes() {
        let first = wrap_generated_region("# Page\n\nGenerated body.\n");
        let same = wrap_generated_region("# Page\n\nGenerated body.");
        let changed = wrap_generated_region("# Page\n\nHuman correction.");

        assert_eq!(first, same);
        assert_ne!(first, changed);
        assert!(first.contains(&generated_region_hash("# Page\n\nGenerated body.")));
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
            page.path == "docs/SOURCE-MAP.md" && page.content.contains("Semantic Structure")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/evidence/SOURCES.md" && page.content.contains("file:test")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/evidence/CLAIMS.md"
                && page.content.contains("claim:")
                && page.content.contains("evidence: `file:test`")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/areas/src/OVERVIEW.md" && page.content.contains("src/lib.rs")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/api/OVERVIEW.md"
                && page
                    .content
                    .contains("<summary>Relevant source files</summary>")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/conventions/OVERVIEW.md"
                && page.content.contains("## Evidence Standard")
                && page.content.contains("Required LLM Synthesis")
        }));
        assert!(
            pages.iter().any(|page| {
                page.path == "docs/api/OVERVIEW.md" && page.content.contains("build")
            })
        );
    }
}

impl Default for WikiDocsLayout {
    fn default() -> Self {
        Self {
            generated_docs_root: "docs",
        }
    }
}
