//! Generated wiki document boundary.

use codewiki_explore::{ExplorationSnapshot, promote_claims_from_snapshot};
use std::fs;
use std::path::{Path, PathBuf};

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
    _repo_label: &str,
    detection_markdown: &str,
    exploration: Option<&ExplorationSnapshot>,
) -> Vec<GeneratedPage> {
    [
        GeneratedPage {
            path: "docs/evidence/README.md".to_string(),
            content: "# Evidence\n\nThis directory contains deterministic discovery artifacts. It is not the reader-facing wiki. The CodeWiki skill must create the repository mental model, WikiPlan, and reader pages before generation can be marked ready.\n\n- `SOURCES.md`: inspected source/docs/provider artifacts.\n- `CLAIMS.md`: durable evidence-backed structure claims.\n- `COMMANDS.md`: verification commands and summarized results.\n".to_string(),
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
    ]
        .into_iter()
        .map(|page| GeneratedPage {
            path: page.path.clone(),
            content: wrap_generated_region(&page.content),
        })
        .collect()
}

/// Result of deterministic reader-document validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReaderDocsQualityReport {
    /// Whether all deterministic and declared semantic gates passed.
    pub ready: bool,
    /// Named failures that prevent reader-doc success.
    pub errors: Vec<String>,
    /// Reader pages inspected, excluding evidence pages.
    pub reader_pages_checked: usize,
}

/// Validate a synthesized CodeWiki workspace before it can report reader-doc success.
pub fn validate_reader_workspace(workspace_root: &Path) -> ReaderDocsQualityReport {
    let mut errors = Vec::new();
    let control_root = workspace_root.join(".agents/skills/codewiki/project");
    let plan_path = control_root.join("plan.yml");
    let quality_path = control_root.join("quality-report.yml");
    let run_path = control_root.join("run.yml");

    let plan = fs::read_to_string(&plan_path).unwrap_or_else(|_| {
        errors.push(format!("missing WikiPlan: {}", plan_path.display()));
        String::new()
    });
    for required in [
        "schema_version: 2",
        "planner_contract_version: reader-first-v2",
        "source_commit:",
        "source_dirty:",
        "visible_docs:",
        "repository_mental_model:",
        "page_type:",
        "reader_job:",
        "reader_questions:",
        "source_anchors:",
        "diagram_slots:",
        "acceptance_checks:",
    ] {
        if !plan.contains(required) {
            errors.push(format!(
                "WikiPlan missing required contract field `{required}`"
            ));
        }
    }
    if plan.contains("pending-llm-selection") || plan.contains("llm_semantic_planning_pending") {
        errors.push("WikiPlan still contains deterministic planning placeholders".to_string());
    }
    if plan.contains("source_commit: \"unknown\"") {
        errors.push("WikiPlan lacks a reproducible source revision".to_string());
    }
    let planned_pages = validate_wikiplan_structure(&plan, &mut errors);

    let run = fs::read_to_string(&run_path).unwrap_or_else(|_| {
        errors.push(format!("missing run provenance: {}", run_path.display()));
        String::new()
    });
    for required in [
        "companion_interface_version: 3",
        "skill_installation:\n  state: verified",
        "discovery: complete",
        "evidence_persistence: complete",
    ] {
        if !run.contains(required) {
            errors.push(format!("run provenance missing `{required}`"));
        }
    }

    let quality = fs::read_to_string(&quality_path).unwrap_or_else(|_| {
        errors.push(format!(
            "missing quality report: {}",
            quality_path.display()
        ));
        String::new()
    });
    for required in [
        "model_synthesis: pass",
        "contract_coverage: pass",
        "source_audit: pass",
        "diagram_audit: pass",
        "cross_page_review: pass",
        "docs_only_onboarding: pass",
        "reader_context: docs_only",
        "source_auditor_context: source_and_evidence",
        "critical_failures: 0",
    ] {
        if !quality.contains(required) {
            errors.push(format!("quality report missing `{required}`"));
        }
    }
    for required in [
        "generation_model:",
        "evaluation_model:",
        "revision_attempts:",
    ] {
        if !quality.contains(required) {
            errors.push(format!("quality report missing `{required}`"));
        }
    }
    if quality.contains("generation_model: \"unrecorded\"")
        || quality.contains("evaluation_model: \"unrecorded\"")
    {
        errors.push("quality report lacks model provenance".to_string());
    }
    if let Some(attempts) = yaml_scalar(&quality, "revision_attempts") {
        match attempts.parse::<u32>() {
            Ok(0 | 1) => {}
            Ok(value) => errors.push(format!(
                "quality report exceeds one bounded revision attempt: {value}"
            )),
            Err(_) => errors.push("quality report revision_attempts is not an integer".to_string()),
        }
    }

    for required in [
        workspace_root.join("docs/QUICKSTART.md"),
        workspace_root.join("docs/conventions/OVERVIEW.md"),
    ] {
        if !required.exists() {
            errors.push(format!(
                "missing required reader page: {}",
                required.display()
            ));
        }
    }

    let mut reader_pages = Vec::new();
    collect_markdown_files(&workspace_root.join("docs"), &mut reader_pages);
    reader_pages.retain(|path| !path.starts_with(workspace_root.join("docs/evidence")));
    let mut page_contents = Vec::new();
    for path in &reader_pages {
        match fs::read_to_string(path) {
            Ok(content) => {
                validate_reader_page(workspace_root, path, &content, &mut errors);
                page_contents.push((path.clone(), content));
            }
            Err(error) => errors.push(format!("cannot read {}: {error}", path.display())),
        }
    }
    validate_plan_page_coverage(workspace_root, &planned_pages, &reader_pages, &mut errors);
    validate_cross_page_navigation(workspace_root, &page_contents, &mut errors);

    ReaderDocsQualityReport {
        ready: errors.is_empty(),
        errors,
        reader_pages_checked: reader_pages.len(),
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ParsedPlanPage {
    path: String,
    page_type: String,
    parent_page: Option<String>,
    prerequisites: Vec<String>,
    topic_ids: Vec<String>,
}

fn validate_wikiplan_structure(plan: &str, errors: &mut Vec<String>) -> Vec<ParsedPlanPage> {
    let lines: Vec<_> = plan.lines().collect();
    let mental_model = section_lines(&lines, "repository_mental_model:", "pages:");
    if !mental_model
        .iter()
        .any(|line| line.trim_start().starts_with("- "))
    {
        errors.push("WikiPlan repository mental model is empty".to_string());
    }

    let mut blocks: Vec<Vec<&str>> = Vec::new();
    for line in &lines {
        if line.starts_with("  - path:") {
            blocks.push(vec![line]);
        } else if let Some(block) = blocks.last_mut()
            && (line.starts_with("    ") || line.trim().is_empty())
        {
            block.push(line);
        }
    }
    if blocks.is_empty() {
        errors.push("WikiPlan contains no page contracts".to_string());
        return Vec::new();
    }

    let mut pages = Vec::new();
    let mut paths = std::collections::BTreeSet::new();
    let mut topics = std::collections::BTreeMap::new();
    for block in blocks {
        let path = block_scalar(&block, "path").unwrap_or_default();
        let page_type = block_scalar(&block, "page_type").unwrap_or_default();
        let label = if path.is_empty() {
            "<missing-path>"
        } else {
            &path
        };
        for field in [
            "title",
            "page_type",
            "section_id",
            "parent_page",
            "order",
            "importance",
            "reader_job",
            "scope",
            "out_of_scope",
            "diagram_slots",
            "source_anchors",
            "prerequisites",
            "evidence_gaps",
            "related_pages",
            "open_questions",
        ] {
            if !block_has_field(&block, field) {
                errors.push(format!("WikiPlan page `{label}` missing `{field}`"));
            }
        }
        for field in [
            "audiences",
            "reader_questions",
            "required_sections",
            "topic_ids",
            "refresh_triggers",
            "acceptance_checks",
        ] {
            if block_list(&block, field).is_empty() {
                errors.push(format!("WikiPlan page `{label}` has empty `{field}`"));
            }
        }
        let selectors = block
            .iter()
            .filter(|line| line.trim_start().starts_with("- selector:"))
            .count();
        let reasons = block
            .iter()
            .filter(|line| line.trim_start().starts_with("reason:"))
            .count();
        if selectors == 0 || reasons < selectors {
            errors.push(format!(
                "WikiPlan page `{label}` source anchors require a relevance reason"
            ));
        }
        let diagram_kinds = block
            .iter()
            .filter(|line| line.trim_start().starts_with("- kind:"))
            .count();
        let diagram_questions = block
            .iter()
            .filter(|line| line.trim_start().starts_with("question:"))
            .count();
        if diagram_questions < diagram_kinds {
            errors.push(format!(
                "WikiPlan page `{label}` diagram slots require a reader question"
            ));
        }
        if path.is_empty() {
            continue;
        }
        if !paths.insert(path.clone()) {
            errors.push(format!("WikiPlan duplicates page path `{path}`"));
        }
        let topic_ids = block_list(&block, "topic_ids");
        for topic in &topic_ids {
            if let Some(owner) = topics.insert(topic.clone(), path.clone()) {
                errors.push(format!(
                    "WikiPlan topic `{topic}` has multiple canonical owners: `{owner}` and `{path}`"
                ));
            }
        }
        let parent_page = block_scalar(&block, "parent_page").and_then(|value| {
            if value == "null" || value.is_empty() {
                None
            } else {
                Some(value)
            }
        });
        pages.push(ParsedPlanPage {
            path,
            page_type,
            parent_page,
            prerequisites: block_list(&block, "prerequisites"),
            topic_ids,
        });
    }

    if !paths.contains("docs/QUICKSTART.md") {
        errors.push("WikiPlan lacks canonical docs/QUICKSTART.md contract".to_string());
    }
    if !paths.contains("docs/conventions/OVERVIEW.md") {
        errors.push("WikiPlan lacks canonical docs/conventions/OVERVIEW.md contract".to_string());
    }
    for page in &pages {
        if let Some(parent) = &page.parent_page
            && !paths.contains(parent)
        {
            errors.push(format!(
                "WikiPlan page `{}` references missing parent `{parent}`",
                page.path
            ));
        }
        for prerequisite in &page.prerequisites {
            if !paths.contains(prerequisite) {
                errors.push(format!(
                    "WikiPlan page `{}` references missing prerequisite `{prerequisite}`",
                    page.path
                ));
            }
        }
        if page.path != "docs/QUICKSTART.md"
            && page.page_type != "evidence"
            && page.parent_page.is_none()
        {
            errors.push(format!(
                "WikiPlan reader page `{}` is orphaned from the hierarchy",
                page.path
            ));
        }
    }
    validate_prerequisite_cycles(&pages, errors);
    pages
}

fn section_lines<'a>(lines: &'a [&'a str], start: &str, end: &str) -> Vec<&'a str> {
    let Some(start_index) = lines.iter().position(|line| *line == start) else {
        return Vec::new();
    };
    let end_index = lines[start_index + 1..]
        .iter()
        .position(|line| *line == end)
        .map_or(lines.len(), |index| start_index + 1 + index);
    lines[start_index + 1..end_index].to_vec()
}

fn block_has_field(block: &[&str], key: &str) -> bool {
    block.iter().any(|line| {
        line.trim_start()
            .strip_prefix(key)
            .is_some_and(|rest| rest.starts_with(':'))
    })
}

fn block_scalar(block: &[&str], key: &str) -> Option<String> {
    block.iter().find_map(|line| {
        let trimmed = line.trim_start();
        let prefix = if key == "path" {
            "- path:".to_string()
        } else {
            format!("{key}:")
        };
        trimmed
            .strip_prefix(&prefix)
            .map(|value| value.trim().trim_matches('"').to_string())
    })
}

fn block_list(block: &[&str], key: &str) -> Vec<String> {
    let Some(start) = block
        .iter()
        .position(|line| line.trim_start() == format!("{key}:"))
    else {
        return Vec::new();
    };
    let key_indent = block[start].len() - block[start].trim_start().len();
    let mut values = Vec::new();
    for line in &block[start + 1..] {
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent <= key_indent {
            break;
        }
        let trimmed = line.trim_start();
        if let Some(value) = trimmed.strip_prefix("- ") {
            let value = value.trim().trim_matches('"');
            if value != "[]" && !value.contains(':') {
                values.push(value.to_string());
            }
        }
    }
    values
}

fn validate_prerequisite_cycles(pages: &[ParsedPlanPage], errors: &mut Vec<String>) {
    let graph: std::collections::BTreeMap<_, _> = pages
        .iter()
        .map(|page| (page.path.as_str(), page.prerequisites.as_slice()))
        .collect();
    for page in pages {
        let mut visiting = std::collections::BTreeSet::new();
        if has_prerequisite_cycle(&page.path, &graph, &mut visiting) {
            errors.push(format!(
                "WikiPlan prerequisite cycle includes `{}`",
                page.path
            ));
        }
    }
}

fn has_prerequisite_cycle<'a>(
    path: &'a str,
    graph: &std::collections::BTreeMap<&'a str, &'a [String]>,
    visiting: &mut std::collections::BTreeSet<&'a str>,
) -> bool {
    if !visiting.insert(path) {
        return true;
    }
    let cycle = graph.get(path).is_some_and(|prerequisites| {
        prerequisites.iter().any(|prerequisite| {
            graph.contains_key(prerequisite.as_str())
                && has_prerequisite_cycle(prerequisite, graph, visiting)
        })
    });
    visiting.remove(path);
    cycle
}

fn validate_plan_page_coverage(
    workspace_root: &Path,
    planned_pages: &[ParsedPlanPage],
    reader_pages: &[PathBuf],
    errors: &mut Vec<String>,
) {
    let planned_reader_paths: std::collections::BTreeSet<_> = planned_pages
        .iter()
        .filter(|page| page.page_type != "evidence")
        .map(|page| page.path.as_str())
        .collect();
    for page in planned_pages
        .iter()
        .filter(|page| page.page_type != "evidence")
    {
        if !workspace_root.join(&page.path).exists() {
            errors.push(format!(
                "WikiPlan reader page `{}` was not synthesized",
                page.path
            ));
        }
    }
    for path in reader_pages {
        let Ok(relative) = path.strip_prefix(workspace_root) else {
            continue;
        };
        let relative = relative.to_string_lossy();
        if !planned_reader_paths.contains(relative.as_ref()) {
            errors.push(format!("reader page `{relative}` has no WikiPlan contract"));
        }
    }
}

fn collect_markdown_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, out);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            out.push(path);
        }
    }
    out.sort();
}

fn validate_reader_page(
    workspace_root: &Path,
    path: &Path,
    content: &str,
    errors: &mut Vec<String>,
) {
    let label = path.display();
    for forbidden in [
        "file://",
        "/Users/",
        "/private/tmp/",
        "/var/folders/",
        "Error generating content:",
        "## Semantic Snapshot",
        "## Dependency Hints",
        "discovered symbols",
        "lexical-import",
        "<CardGroup",
        "<ResponseField",
        "<Steps",
        "<Info",
    ] {
        if content.contains(forbidden) {
            errors.push(format!(
                "{label}: forbidden generated artifact `{forbidden}`"
            ));
        }
    }
    if content.matches("## Related pages").count() > 1 {
        errors.push(format!("{label}: duplicate Related pages sections"));
    }
    let mut headings = std::collections::BTreeSet::new();
    for heading in content.lines().filter(|line| line.starts_with("## ")) {
        let normalized = heading.trim().to_ascii_lowercase();
        if !headings.insert(normalized) {
            errors.push(format!("{label}: duplicate canonical section `{heading}`"));
        }
    }
    if has_duplicate_frontmatter(content) {
        errors.push(format!("{label}: duplicate frontmatter blocks"));
    }
    if !content.lines().any(|line| line.starts_with("# ")) {
        errors.push(format!("{label}: missing page title"));
    }
    if !content.to_ascii_lowercase().contains("## purpose") {
        errors.push(format!("{label}: missing reader purpose"));
    }
    if path.ends_with("QUICKSTART.md") && !content.to_ascii_lowercase().contains("## mental model")
    {
        errors.push(format!("{label}: Quickstart missing mental model"));
    }
    if let Some(index) = content.find("<summary>Relevant source files</summary>")
        && index < 600
    {
        errors.push(format!(
            "{label}: source inventory precedes the reader mental model"
        ));
    }
    let mermaid_starts = content.matches("```mermaid").count();
    let all_fences = content.matches("```").count();
    if mermaid_starts > 0 && (all_fences < mermaid_starts * 2 || !all_fences.is_multiple_of(2)) {
        errors.push(format!("{label}: malformed Mermaid/code fence structure"));
    }
    for target in markdown_link_targets(content) {
        if target.starts_with('#')
            || target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            continue;
        }
        let target = target.split('#').next().unwrap_or_default();
        if target.is_empty() {
            continue;
        }
        let resolved = path.parent().unwrap_or(workspace_root).join(target);
        let workspace_canonical = fs::canonicalize(workspace_root).ok();
        let resolved_canonical = fs::canonicalize(&resolved).ok();
        if !matches!(
            (&workspace_canonical, &resolved_canonical),
            (Some(root), Some(target)) if target.starts_with(root)
        ) {
            errors.push(format!("{label}: unresolved local link `{target}`"));
        }
    }
}

fn markdown_link_targets(content: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut rest = content;
    while let Some(index) = rest.find("](") {
        rest = &rest[index + 2..];
        let Some(end) = rest.find(')') else {
            break;
        };
        targets.push(
            rest[..end]
                .trim()
                .trim_matches('<')
                .trim_matches('>')
                .to_string(),
        );
        rest = &rest[end + 1..];
    }
    targets
}

fn validate_cross_page_navigation(
    workspace_root: &Path,
    pages: &[(PathBuf, String)],
    errors: &mut Vec<String>,
) {
    let mut incoming = std::collections::BTreeSet::new();
    let workspace_canonical =
        fs::canonicalize(workspace_root).unwrap_or_else(|_| workspace_root.to_path_buf());
    for (source, content) in pages {
        for target in markdown_link_targets(content) {
            if target.starts_with('#')
                || target.starts_with("http://")
                || target.starts_with("https://")
                || target.starts_with("mailto:")
            {
                continue;
            }
            let target = target.split('#').next().unwrap_or_default();
            let resolved = source.parent().unwrap_or(workspace_root).join(target);
            let Ok(resolved) = fs::canonicalize(resolved) else {
                continue;
            };
            if let Ok(relative) = resolved.strip_prefix(&workspace_canonical) {
                incoming.insert(relative.to_path_buf());
            }
        }
    }
    for (page, _) in pages {
        let canonical_page = fs::canonicalize(page).unwrap_or_else(|_| page.clone());
        let Ok(relative) = canonical_page.strip_prefix(&workspace_canonical) else {
            continue;
        };
        if relative != Path::new("docs/QUICKSTART.md") && !incoming.contains(relative) {
            errors.push(format!("{}: orphan reader page", page.display()));
        }
    }
}

fn has_duplicate_frontmatter(content: &str) -> bool {
    let Some(rest) = content.strip_prefix("---\n") else {
        return false;
    };
    let Some(end) = rest.find("\n---\n") else {
        return false;
    };
    rest[end + 5..].trim_start().starts_with("---\n")
}

fn yaml_scalar<'a>(yaml: &'a str, key: &str) -> Option<&'a str> {
    yaml.lines().find_map(|line| {
        let (candidate, value) = line.split_once(':')?;
        (candidate.trim() == key).then(|| value.trim().trim_matches('"'))
    })
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

impl Default for WikiDocsLayout {
    fn default() -> Self {
        Self {
            generated_docs_root: "docs",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_companion_pages_are_evidence_only() {
        let pages = render_initial_pages("example", "### Languages\n\n- Rust\n");
        let paths: Vec<_> = pages.iter().map(|page| page.path.as_str()).collect();

        assert!(!paths.contains(&"docs/QUICKSTART.md"));
        assert!(!paths.contains(&"docs/SOURCE-MAP.md"));
        assert!(!paths.contains(&"docs/architecture/OVERVIEW.md"));
        assert!(paths.contains(&"docs/evidence/CLAIMS.md"));
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
    fn semantic_companion_output_is_evidence_only() {
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

        assert!(!pages.iter().any(|page| page.path == "docs/QUICKSTART.md"));
        assert!(!pages.iter().any(|page| page.path == "docs/SOURCE-MAP.md"));
        assert!(
            !pages
                .iter()
                .any(|page| page.path == "docs/architecture/OVERVIEW.md")
        );
        assert!(pages.iter().any(|page| {
            page.path == "docs/evidence/SOURCES.md" && page.content.contains("file:test")
        }));
        assert!(pages.iter().any(|page| {
            page.path == "docs/evidence/CLAIMS.md"
                && page.content.contains("claim:")
                && page.content.contains("evidence: `file:test`")
        }));
        assert!(
            !pages
                .iter()
                .any(|page| page.path.starts_with("docs/areas/"))
        );
    }

    #[test]
    fn reader_validation_rejects_grok_export_artifacts() {
        let root = temp_path("codewiki-docs-invalid-reader");
        fs::create_dir_all(root.join("docs/conventions")).expect("mkdir docs");
        fs::create_dir_all(root.join(".agents/skills/codewiki/project")).expect("mkdir control");
        fs::write(
            root.join(".agents/skills/codewiki/project/plan.yml"),
            "schema_version: 2\nrepository_mental_model:\nreader_questions:\nsource_anchors:\nacceptance_checks:\n",
        )
        .expect("write plan");
        fs::write(
            root.join(".agents/skills/codewiki/project/quality-report.yml"),
            "model_synthesis: pass\ncontract_coverage: pass\nsource_audit: pass\ndiagram_audit: pass\ncross_page_review: pass\ndocs_only_onboarding: pass\n",
        )
        .expect("write quality");
        fs::write(
            root.join("docs/QUICKSTART.md"),
            "---\ntitle: One\n---\n---\ntitle: Two\n---\n# Quickstart\n\n[Source](file:///var/folders/tmp/repo/src.rs)\n\n[Escape](./escape.md)\n\n## Related pages\n\n## Related pages\n",
        )
        .expect("write quickstart");
        #[cfg(unix)]
        let outside = {
            let outside = root.with_extension("outside.md");
            fs::write(&outside, "# Outside\n").expect("write outside file");
            std::os::unix::fs::symlink(&outside, root.join("docs/escape.md"))
                .expect("link outside workspace");
            outside
        };
        fs::write(
            root.join("docs/conventions/OVERVIEW.md"),
            "# Conventions\n\n<CardGroup>unsupported</CardGroup>\n",
        )
        .expect("write conventions");

        let report = validate_reader_workspace(&root);

        assert!(!report.ready);
        assert!(report.errors.iter().any(|error| error.contains("file://")));
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.contains("duplicate frontmatter"))
        );
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.contains("duplicate Related"))
        );
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.contains("<CardGroup"))
        );
        #[cfg(unix)]
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.contains("unresolved local link `./escape.md`"))
        );
        let _ = fs::remove_dir_all(root);
        #[cfg(unix)]
        let _ = fs::remove_file(outside);
    }

    #[test]
    fn wikiplan_validation_rejects_duplicate_ownership_and_cycles() {
        let plan = r#"schema_version: 2
repository_mental_model:
  systems:
    - "Runtime"
pages:
  - path: docs/QUICKSTART.md
    title: "Quickstart"
    page_type: overview
    section_id: quickstart
    parent_page: docs/conventions/OVERVIEW.md
    order: 10
    importance: critical
    reader_job: "Start"
    scope: "Runtime"
    out_of_scope: "Reference"
    audiences:
      - "developer"
    prerequisites:
      - "docs/conventions/OVERVIEW.md"
    reader_questions:
      - "Where do I start?"
    required_sections:
      - "purpose"
    diagram_slots:
      []
    topic_ids:
      - "runtime"
    source_anchors:
      - selector: "src/main.rs"
        reason: "Entrypoint"
    evidence_gaps:
      []
    related_pages:
      []
    open_questions:
      []
    refresh_triggers:
      - "source_changed"
    acceptance_checks:
      - "Start is clear"
  - path: docs/conventions/OVERVIEW.md
    title: "Conventions"
    page_type: reference
    section_id: conventions
    parent_page: docs/QUICKSTART.md
    order: 20
    importance: supporting
    reader_job: "Change safely"
    scope: "Rules"
    out_of_scope: "Generic advice"
    audiences:
      - "developer"
    prerequisites:
      - "docs/QUICKSTART.md"
    reader_questions:
      - "What rules apply?"
    required_sections:
      - "purpose"
    diagram_slots:
      []
    topic_ids:
      - "runtime"
    source_anchors:
      - selector: "Cargo.toml"
        reason: "Explicit policy"
    evidence_gaps:
      []
    related_pages:
      []
    open_questions:
      []
    refresh_triggers:
      - "source_changed"
    acceptance_checks:
      - "Rules are evidenced"
"#;
        let mut errors = Vec::new();

        validate_wikiplan_structure(plan, &mut errors);

        assert!(
            errors
                .iter()
                .any(|error| error.contains("multiple canonical owners"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("prerequisite cycle"))
        );
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
