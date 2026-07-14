//! Deterministic semantic exploration for CodeWiki.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum number of files inspected by semantic exploration v1.
pub const DEFAULT_FILE_LIMIT: usize = 3_000;

/// Maximum file size read for lexical symbol/import hints.
pub const MAX_TEXT_FILE_BYTES: u64 = 512 * 1024;

/// Semantic snapshot produced from repository files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplorationSnapshot {
    /// Snapshot schema version.
    pub schema_version: u32,
    /// Explored source/config/doc files.
    pub files: Vec<ExploredFile>,
    /// Top-level area summaries.
    pub areas: Vec<AreaSummary>,
    /// Dependency/import hints discovered from source text.
    pub dependency_hints: Vec<DependencyHint>,
    /// Evidence references for inspected files.
    pub evidence: Vec<EvidenceRef>,
    /// Whether traversal hit the configured file limit.
    pub truncated: bool,
    /// File limit used for this snapshot.
    pub file_limit: usize,
}

impl ExplorationSnapshot {
    /// Return a concise Markdown summary for generated docs.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("### Semantic Exploration\n\n");
        out.push_str(&format!(
            "- Files inspected: {}\n- Areas: {}\n- Dependency hints: {}\n- Truncated: {}\n\n",
            self.files.len(),
            self.areas.len(),
            self.dependency_hints.len(),
            self.truncated,
        ));

        out.push_str("#### Areas\n\n");
        if self.areas.is_empty() {
            out.push_str("- none detected\n\n");
        } else {
            for area in &self.areas {
                out.push_str(&format!(
                    "- `{}`: {} files, {} symbols\n",
                    area.name, area.file_count, area.symbol_count
                ));
            }
            out.push('\n');
        }

        out.push_str("#### Notable Files\n\n");
        for file in self.files.iter().take(20) {
            out.push_str(&format!(
                "- `{}` ({}, {}): {} lines, {} symbols, {} imports\n",
                file.path,
                file.language.as_deref().unwrap_or("unknown"),
                file.role.as_str(),
                file.line_count,
                file.symbols.len(),
                file.imports.len()
            ));
        }
        if self.files.is_empty() {
            out.push_str("- none detected\n");
        }
        out.push('\n');

        out.push_str("#### Dependency Hints\n\n");
        if self.dependency_hints.is_empty() {
            out.push_str("- none detected\n");
        } else {
            for hint in self.dependency_hints.iter().take(30) {
                out.push_str(&format!(
                    "- `{}` -> `{}` ({})\n",
                    hint.from_path, hint.target, hint.kind
                ));
            }
        }

        out
    }
}

/// Deterministic claim promoted from a semantic snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotedClaim {
    /// Stable claim id.
    pub id: String,
    /// Human-readable claim statement.
    pub statement: String,
    /// Confidence label.
    pub confidence: String,
    /// Evidence ids supporting the claim.
    pub evidence_ids: Vec<String>,
}

/// Promote deterministic source-backed claims from a semantic snapshot.
pub fn promote_claims_from_snapshot(snapshot: &ExplorationSnapshot) -> Vec<PromotedClaim> {
    let mut claims = Vec::new();

    for file in snapshot.files.iter().take(100) {
        let statement = format!(
            "File `{}` is a {} file with {} discovered symbols and {} import/dependency hints.",
            file.path,
            file.role.as_str(),
            file.symbols.len(),
            file.imports.len(),
        );
        claims.push(PromotedClaim {
            id: stable_claim_id(&statement),
            statement,
            confidence: "source-backed".to_string(),
            evidence_ids: vec![file.evidence_id.clone()],
        });
    }

    claims
}

/// A file inspected during semantic exploration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExploredFile {
    /// Repo-relative path.
    pub path: String,
    /// Detected language from extension or filename.
    pub language: Option<String>,
    /// File role inferred from path/name.
    pub role: FileRole,
    /// Number of lines read.
    pub line_count: usize,
    /// Stable content hash for the inspected text.
    pub content_hash: String,
    /// Symbols discovered from generic lexical patterns.
    pub symbols: Vec<ExploredSymbol>,
    /// Import/use/include/require hints.
    pub imports: Vec<String>,
    /// Stable evidence id for this file.
    pub evidence_id: String,
}

/// File role used for semantic grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileRole {
    /// Runtime/source file.
    Source,
    /// Test or spec file.
    Test,
    /// Build/config/manifest file.
    Config,
    /// Documentation file.
    Documentation,
    /// Other text file worth recording.
    Other,
}

impl FileRole {
    /// Stable string representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Test => "test",
            Self::Config => "config",
            Self::Documentation => "documentation",
            Self::Other => "other",
        }
    }
}

/// Generic symbol hint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExploredSymbol {
    /// Symbol name.
    pub name: String,
    /// Symbol kind, e.g. `function`, `class`, `struct`, `type`.
    pub kind: String,
    /// One-based line number.
    pub line: usize,
}

/// Dependency/import hint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyHint {
    /// Source file path.
    pub from_path: String,
    /// Imported target text.
    pub target: String,
    /// Import style.
    pub kind: String,
}

/// Area summary grouped by top-level path segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreaSummary {
    /// Area name.
    pub name: String,
    /// Files in the area.
    pub file_count: usize,
    /// Symbols in the area.
    pub symbol_count: usize,
    /// Roles observed in the area.
    pub roles: Vec<FileRole>,
}

/// Evidence reference for an inspected source artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRef {
    /// Stable evidence id.
    pub id: String,
    /// File path.
    pub path: String,
    /// Evidence kind.
    pub kind: String,
}

/// Explore repository semantics with default limits.
pub fn explore_repository(root: impl AsRef<Path>) -> std::io::Result<ExplorationSnapshot> {
    explore_repository_with_limit(root, DEFAULT_FILE_LIMIT)
}

/// Explore repository semantics with an explicit file limit.
pub fn explore_repository_with_limit(
    root: impl AsRef<Path>,
    file_limit: usize,
) -> std::io::Result<ExplorationSnapshot> {
    let root = root.as_ref();
    let mut relative_paths = Vec::new();
    let mut truncated = false;
    collect_candidate_files(
        root,
        root,
        0,
        file_limit,
        &mut relative_paths,
        &mut truncated,
    )?;

    let mut files = Vec::new();
    let mut dependency_hints = Vec::new();
    let mut evidence = Vec::new();

    for relative_path in relative_paths {
        let absolute_path = root.join(&relative_path);
        let metadata = fs::metadata(&absolute_path)?;
        if metadata.len() > MAX_TEXT_FILE_BYTES {
            continue;
        }
        let Ok(text) = fs::read_to_string(&absolute_path) else {
            continue;
        };
        let path = normalize_path(&relative_path);
        let role = infer_role(&relative_path);
        let language = language_for_path(&relative_path).map(str::to_string);
        let content_hash = text_hash(&text);
        let symbols = extract_symbols(&text);
        let imports = extract_imports(&text);
        let evidence_id = evidence_id_for_path(&path);

        for import in &imports {
            dependency_hints.push(DependencyHint {
                from_path: path.clone(),
                target: import.clone(),
                kind: "lexical-import".to_string(),
            });
        }

        evidence.push(EvidenceRef {
            id: evidence_id.clone(),
            path: path.clone(),
            kind: "file".to_string(),
        });

        files.push(ExploredFile {
            path,
            language,
            role,
            line_count: text.lines().count(),
            content_hash,
            symbols,
            imports,
            evidence_id,
        });
    }

    let areas = summarize_areas(&files);

    Ok(ExplorationSnapshot {
        schema_version: 1,
        files,
        areas,
        dependency_hints,
        evidence,
        truncated,
        file_limit,
    })
}

fn collect_candidate_files(
    root: &Path,
    current: &Path,
    depth: usize,
    file_limit: usize,
    files: &mut Vec<PathBuf>,
    truncated: &mut bool,
) -> std::io::Result<()> {
    if depth > 6 || files.len() >= file_limit {
        *truncated |= files.len() >= file_limit;
        return Ok(());
    }

    for entry in fs::read_dir(current)? {
        if files.len() >= file_limit {
            *truncated = true;
            break;
        }
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if should_skip(&name) {
            continue;
        }
        let relative = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
        if is_generated_codewiki_path(&relative) {
            continue;
        }

        if path.is_dir() {
            collect_candidate_files(root, &path, depth + 1, file_limit, files, truncated)?;
        } else if path.is_file() {
            if !is_semantic_candidate(&relative) {
                continue;
            }
            files.push(relative);
        }
    }

    Ok(())
}

fn is_generated_codewiki_path(path: &Path) -> bool {
    path.starts_with(".agents/skills/codewiki") || is_generated_docs_path(path)
}

fn is_generated_docs_path(path: &Path) -> bool {
    path.starts_with("docs/evidence")
        || path.starts_with("docs/areas")
        || path.starts_with("docs/architecture")
        || path.starts_with("docs/domain")
        || path.starts_with("docs/workflows")
        || path.starts_with("docs/data-models")
        || path.starts_with("docs/api")
        || path.starts_with("docs/operations")
        || path.starts_with("docs/testing")
        || path.starts_with("docs/conventions")
        || matches!(
            path.to_str(),
            Some(
                "docs/QUICKSTART.md"
                    | "docs/SOURCE-MAP.md"
                    | "docs/GLOSSARY.md"
                    | "docs/OPEN-QUESTIONS.md"
                    | "docs/quickstart.md"
                    | "docs/source-map.md"
                    | "docs/glossary.md"
                    | "docs/open-questions.md"
            )
        )
}

fn should_skip(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | "node_modules"
            | "target"
            | "dist"
            | "build"
            | ".next"
            | "__pycache__"
            | ".venv"
            | "vendor"
            | "coverage"
    )
}

fn is_semantic_candidate(path: &Path) -> bool {
    if infer_role(path) != FileRole::Other {
        return true;
    }
    language_for_path(path).is_some()
}

fn infer_role(path: &Path) -> FileRole {
    let rel = normalize_path(path);
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    if rel.starts_with("docs/")
        || rel.contains("/docs/")
        || matches!(file_name, "README.md" | "README" | "CONTRIBUTING.md")
    {
        return FileRole::Documentation;
    }
    if rel.contains("test")
        || rel.contains("spec")
        || rel.starts_with("tests/")
        || rel.contains("/tests/")
    {
        return FileRole::Test;
    }
    if is_config_file(file_name) {
        return FileRole::Config;
    }
    if language_for_path(path).is_some() {
        return FileRole::Source;
    }
    FileRole::Other
}

fn is_config_file(file_name: &str) -> bool {
    matches!(
        file_name,
        "Cargo.toml"
            | "package.json"
            | "pyproject.toml"
            | "requirements.txt"
            | "go.mod"
            | "pom.xml"
            | "build.gradle"
            | "build.gradle.kts"
            | "tsconfig.json"
            | "vite.config.ts"
            | "vite.config.js"
            | "next.config.js"
            | "next.config.mjs"
            | "next.config.ts"
    )
}

fn language_for_path(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("rs") => Some("Rust"),
        Some("ts") | Some("tsx") => Some("TypeScript"),
        Some("js") | Some("jsx") | Some("mjs") | Some("cjs") => Some("JavaScript"),
        Some("py") => Some("Python"),
        Some("go") => Some("Go"),
        Some("java") => Some("Java"),
        Some("kt") | Some("kts") => Some("Kotlin"),
        Some("swift") => Some("Swift"),
        Some("rb") => Some("Ruby"),
        Some("php") => Some("PHP"),
        Some("cs") => Some("C#"),
        Some("cpp") | Some("cc") | Some("cxx") | Some("hpp") | Some("h") => Some("C/C++"),
        Some("sql") => Some("SQL"),
        Some("md") => Some("Markdown"),
        Some("toml") | Some("json") | Some("yaml") | Some("yml") => Some("Config"),
        _ => None,
    }
}

fn extract_symbols(text: &str) -> Vec<ExploredSymbol> {
    let mut symbols = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some((kind, name)) = symbol_from_line(trimmed) {
            symbols.push(ExploredSymbol {
                name: clean_symbol_name(name),
                kind: kind.to_string(),
                line: index + 1,
            });
        }
        if symbols.len() >= 200 {
            break;
        }
    }
    symbols
}

fn symbol_from_line(line: &str) -> Option<(&'static str, &str)> {
    let line = strip_visibility(line);
    for (prefix, kind) in [
        ("async function ", "function"),
        ("function ", "function"),
        ("fn ", "function"),
        ("async fn ", "function"),
        ("def ", "function"),
        ("class ", "class"),
        ("struct ", "struct"),
        ("enum ", "enum"),
        ("interface ", "interface"),
        ("type ", "type"),
        ("const ", "constant"),
        ("let ", "binding"),
        ("var ", "binding"),
        ("package ", "package"),
        ("impl ", "impl"),
        ("trait ", "trait"),
    ] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some((kind, rest));
        }
    }
    if let Some(rest) = line.strip_prefix("export function ") {
        return Some(("function", rest));
    }
    if let Some(rest) = line.strip_prefix("export class ") {
        return Some(("class", rest));
    }
    if let Some(rest) = line.strip_prefix("export interface ") {
        return Some(("interface", rest));
    }
    if let Some(rest) = line.strip_prefix("export type ") {
        return Some(("type", rest));
    }
    if let Some(rest) = line.strip_prefix("export const ") {
        return Some(("constant", rest));
    }
    None
}

fn strip_visibility(line: &str) -> &str {
    line.strip_prefix("pub(crate) ")
        .or_else(|| line.strip_prefix("pub "))
        .or_else(|| line.strip_prefix("private "))
        .or_else(|| line.strip_prefix("public "))
        .unwrap_or(line)
}

fn clean_symbol_name(raw: &str) -> String {
    raw.chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | ':' | '.'))
        .collect::<String>()
        .trim_matches(':')
        .to_string()
}

fn extract_imports(text: &str) -> Vec<String> {
    let mut imports = BTreeSet::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(target) = import_from_line(trimmed) {
            imports.insert(target);
        }
        if imports.len() >= 300 {
            break;
        }
    }
    imports.into_iter().collect()
}

fn import_from_line(line: &str) -> Option<String> {
    if line.starts_with("//") || line.starts_with('#') {
        return None;
    }
    if let Some(rest) = line.strip_prefix("use ") {
        return Some(clean_import_target(rest.trim_end_matches(';')));
    }
    if let Some(rest) = line.strip_prefix("import ") {
        return Some(clean_import_target(rest.trim_end_matches(';')));
    }
    if let Some(rest) = line.strip_prefix("from ") {
        return Some(clean_import_target(rest.trim_end_matches(';')));
    }
    if let Some(rest) = line.strip_prefix("require(") {
        return Some(clean_import_target(rest.trim_end_matches(");")));
    }
    if let Some(rest) = line.strip_prefix("#include ") {
        return Some(clean_import_target(rest));
    }
    None
}

fn clean_import_target(raw: &str) -> String {
    raw.trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim_matches('<')
        .trim_matches('>')
        .to_string()
}

fn summarize_areas(files: &[ExploredFile]) -> Vec<AreaSummary> {
    let mut by_area: BTreeMap<String, (usize, usize, BTreeSet<FileRole>)> = BTreeMap::new();
    for file in files {
        let area = file
            .path
            .split('/')
            .next()
            .filter(|segment| !segment.is_empty())
            .unwrap_or("root")
            .to_string();
        let entry = by_area.entry(area).or_default();
        entry.0 += 1;
        entry.1 += file.symbols.len();
        entry.2.insert(file.role);
    }
    by_area
        .into_iter()
        .map(|(name, (file_count, symbol_count, roles))| AreaSummary {
            name,
            file_count,
            symbol_count,
            roles: roles.into_iter().collect(),
        })
        .collect()
}

fn evidence_id_for_path(path: &str) -> String {
    format!("file:{:016x}", fnv1a64(path.as_bytes()))
}

fn text_hash(text: &str) -> String {
    format!("{:016x}", fnv1a64(text.as_bytes()))
}

fn stable_claim_id(statement: &str) -> String {
    format!("claim:{:016x}", fnv1a64(statement.as_bytes()))
}

fn normalize_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn explores_rust_symbols_and_imports() {
        let root = temp_path("codewiki-explore-rust");
        fs::create_dir_all(root.join("src")).expect("mkdir");
        fs::write(
            root.join("src/lib.rs"),
            "use std::fs;\npub struct Wiki;\npub fn build() {}\n",
        )
        .expect("write");

        let snapshot = explore_repository(&root).expect("explore");

        let file = snapshot
            .files
            .iter()
            .find(|file| file.path == "src/lib.rs")
            .expect("file");
        assert_eq!(file.language.as_deref(), Some("Rust"));
        assert!(file.symbols.iter().any(|symbol| symbol.name == "Wiki"));
        assert!(file.symbols.iter().any(|symbol| symbol.name == "build"));
        assert!(
            snapshot
                .dependency_hints
                .iter()
                .any(|hint| { hint.from_path == "src/lib.rs" && hint.target.contains("std::fs") })
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn explores_typescript_exports_and_areas() {
        let root = temp_path("codewiki-explore-ts");
        fs::create_dir_all(root.join("apps/web/src")).expect("mkdir");
        fs::write(
            root.join("apps/web/src/index.ts"),
            "import { createRoot } from 'react-dom/client';\nexport interface AppConfig {}\nexport function start() {}\n",
        )
        .expect("write");

        let snapshot = explore_repository(&root).expect("explore");

        assert!(snapshot.areas.iter().any(|area| area.name == "apps"));
        assert!(snapshot.files.iter().any(|file| {
            file.path == "apps/web/src/index.ts"
                && file.symbols.iter().any(|symbol| symbol.name == "AppConfig")
        }));
        assert!(snapshot.to_markdown().contains("Dependency Hints"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn promotes_deterministic_claims_from_snapshot() {
        let root = temp_path("codewiki-explore-claims");
        fs::create_dir_all(root.join("src")).expect("mkdir");
        fs::write(root.join("src/lib.rs"), "pub fn build() {}\n").expect("write");

        let snapshot = explore_repository(&root).expect("explore");
        let claims = promote_claims_from_snapshot(&snapshot);

        assert!(
            claims
                .iter()
                .all(|claim| !claim.statement.starts_with("Area `")),
            "top-level traversal areas must not become durable reader claims"
        );
        assert!(claims.iter().any(|claim| {
            claim.statement.contains("File `src/lib.rs`") && !claim.evidence_ids.is_empty()
        }));
        assert!(claims.iter().all(|claim| claim.id.starts_with("claim:")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ignores_generated_conventions_page() {
        let root = temp_path("codewiki-explore-generated-conventions");
        fs::create_dir_all(root.join("docs/conventions")).expect("mkdir docs");
        fs::create_dir_all(root.join("src")).expect("mkdir src");
        fs::write(root.join("docs/conventions/OVERVIEW.md"), "# generated\n")
            .expect("write generated docs");
        fs::write(root.join("src/lib.rs"), "pub fn build() {}\n").expect("write source");

        let snapshot = explore_repository(&root).expect("explore");

        assert!(snapshot.files.iter().any(|file| file.path == "src/lib.rs"));
        assert!(
            snapshot
                .files
                .iter()
                .all(|file| file.path != "docs/conventions/OVERVIEW.md")
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ignores_installed_codewiki_skill_and_companion_payload() {
        let root = temp_path("codewiki-explore-installed-skill");
        fs::create_dir_all(root.join(".agents/skills/codewiki/companion/src"))
            .expect("mkdir installed companion");
        fs::create_dir_all(root.join("src")).expect("mkdir target src");
        fs::write(
            root.join(".agents/skills/codewiki/SKILL.md"),
            "# installed skill\n",
        )
        .expect("write installed skill");
        fs::write(
            root.join(".agents/skills/codewiki/companion/Cargo.toml"),
            "[package]\nname = \"installed-codewiki\"\n",
        )
        .expect("write installed manifest");
        fs::write(
            root.join(".agents/skills/codewiki/companion/src/lib.rs"),
            "pub fn installed_runtime() {}\n",
        )
        .expect("write installed source");
        fs::write(root.join("src/app.js"), "export function app() {}\n")
            .expect("write target source");

        let snapshot = explore_repository(&root).expect("explore");

        assert!(snapshot.files.iter().any(|file| file.path == "src/app.js"));
        assert!(
            snapshot
                .files
                .iter()
                .all(|file| !file.path.starts_with(".agents/skills/codewiki"))
        );
        assert!(
            snapshot
                .evidence
                .iter()
                .all(|evidence| !evidence.path.starts_with(".agents/skills/codewiki"))
        );

        let _ = fs::remove_dir_all(root);
    }

    fn temp_path(prefix: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{suffix}"))
    }
}
