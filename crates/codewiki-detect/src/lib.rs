//! Repository stack detection boundary.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Capabilities planned for repository detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionCapabilities {
    /// Whether language detection is part of this boundary.
    pub languages: bool,
    /// Whether package manager detection is part of this boundary.
    pub package_managers: bool,
    /// Whether framework/library signals are part of this boundary.
    pub frameworks: bool,
    /// Whether entrypoint and test/build discovery are part of this boundary.
    pub entrypoints: bool,
}

impl DetectionCapabilities {
    /// Return the scaffold capability set.
    pub fn scaffold() -> Self {
        Self {
            languages: true,
            package_managers: true,
            frameworks: true,
            entrypoints: true,
        }
    }

    /// Human-readable summary for CLI status output.
    pub fn summary(&self) -> &'static str {
        "languages, package managers, frameworks, entrypoints"
    }
}

/// Detected repository signals.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepositoryDetection {
    /// Languages detected from source file extensions.
    pub languages: Vec<String>,
    /// Package managers and build tools detected from config files.
    pub package_managers: Vec<String>,
    /// Framework/library hints detected from manifests and configs.
    pub frameworks: Vec<String>,
    /// Entrypoint-like files.
    pub entrypoints: Vec<String>,
    /// Test-related files/directories.
    pub tests: Vec<String>,
    /// Documentation files/directories.
    pub docs: Vec<String>,
}

#[derive(Debug, Default)]
struct DetectionSignals {
    package_managers: BTreeSet<String>,
    frameworks: BTreeSet<String>,
    entrypoints: BTreeSet<String>,
    tests: BTreeSet<String>,
    docs: BTreeSet<String>,
}

impl RepositoryDetection {
    /// Render a stable Markdown summary.
    pub fn to_markdown(&self) -> String {
        [
            render_list("Languages", &self.languages),
            render_list("Package/build tools", &self.package_managers),
            render_list("Framework/library signals", &self.frameworks),
            render_list("Entrypoints", &self.entrypoints),
            render_list("Tests", &self.tests),
            render_list("Docs", &self.docs),
        ]
        .join("\n")
    }
}

/// Detect repository signals with bounded filesystem traversal.
pub fn detect_repository(root: impl AsRef<Path>) -> std::io::Result<RepositoryDetection> {
    let root = root.as_ref();
    let mut files = Vec::new();
    collect_files(root, root, 0, &mut files)?;

    let mut languages = BTreeSet::new();
    let mut signals = DetectionSignals::default();

    for path in &files {
        let rel = path.to_string_lossy();
        if let Some(language) = language_for_path(path) {
            languages.insert(language.to_string());
        }
        detect_file_signals(root, path, &rel, &mut signals);
    }

    Ok(RepositoryDetection {
        languages: languages.into_iter().collect(),
        package_managers: signals.package_managers.into_iter().collect(),
        frameworks: signals.frameworks.into_iter().collect(),
        entrypoints: signals.entrypoints.into_iter().collect(),
        tests: signals.tests.into_iter().collect(),
        docs: signals.docs.into_iter().collect(),
    })
}

fn collect_files(
    root: &Path,
    current: &Path,
    depth: usize,
    files: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    if depth > 4 {
        return Ok(());
    }

    for entry in fs::read_dir(current)? {
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
            collect_files(root, &path, depth + 1, files)?;
        } else if path.is_file() {
            files.push(relative);
        }
        if files.len() >= 2_000 {
            break;
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
        ".git" | "node_modules" | "target" | "dist" | "build" | ".next" | "__pycache__"
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
        _ => None,
    }
}

fn detect_file_signals(root: &Path, path: &Path, rel: &str, signals: &mut DetectionSignals) {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    match file_name {
        "Cargo.toml" => {
            signals.package_managers.insert("Cargo".to_string());
            signals.entrypoints.insert(rel.to_string());
        }
        "package.json" => {
            signals
                .package_managers
                .insert("npm-compatible".to_string());
            signals.entrypoints.insert(rel.to_string());
            inspect_package_json(root.join(path), &mut signals.frameworks);
        }
        "pnpm-lock.yaml" => {
            signals.package_managers.insert("pnpm".to_string());
        }
        "yarn.lock" => {
            signals.package_managers.insert("Yarn".to_string());
        }
        "bun.lockb" | "bun.lock" => {
            signals.package_managers.insert("Bun".to_string());
        }
        "pyproject.toml" => {
            signals
                .package_managers
                .insert("Python packaging".to_string());
            inspect_text_file(root.join(path), &mut signals.frameworks);
        }
        "requirements.txt" => {
            signals.package_managers.insert("pip".to_string());
            inspect_text_file(root.join(path), &mut signals.frameworks);
        }
        "go.mod" => {
            signals.package_managers.insert("Go modules".to_string());
        }
        "pom.xml" => {
            signals.package_managers.insert("Maven".to_string());
        }
        "build.gradle" | "build.gradle.kts" => {
            signals.package_managers.insert("Gradle".to_string());
        }
        "next.config.js" | "next.config.mjs" | "next.config.ts" => {
            signals.frameworks.insert("Next.js".to_string());
        }
        "vite.config.ts" | "vite.config.js" => {
            signals.frameworks.insert("Vite".to_string());
        }
        "README.md" | "README" | "CONTRIBUTING.md" => {
            signals.docs.insert(rel.to_string());
        }
        _ => {}
    }

    if rel.starts_with("docs/") || rel.contains("/docs/") {
        signals.docs.insert(rel.to_string());
    }
    if rel.contains("test") || rel.contains("spec") || rel.starts_with("tests/") {
        signals.tests.insert(rel.to_string());
    }
    if matches!(
        rel,
        "src/main.rs" | "src/lib.rs" | "main.py" | "app.py" | "src/index.ts" | "src/index.js"
    ) {
        signals.entrypoints.insert(rel.to_string());
    }
}

fn inspect_package_json(path: PathBuf, frameworks: &mut BTreeSet<String>) {
    inspect_text_file(path, frameworks);
}

fn inspect_text_file(path: PathBuf, frameworks: &mut BTreeSet<String>) {
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    let signals = [
        ("next", "Next.js"),
        ("react", "React"),
        ("vue", "Vue"),
        ("svelte", "Svelte"),
        ("express", "Express"),
        ("fastify", "Fastify"),
        ("django", "Django"),
        ("flask", "Flask"),
        ("fastapi", "FastAPI"),
        ("pytest", "pytest"),
    ];
    for (needle, label) in signals {
        if text.contains(needle) {
            frameworks.insert(label.to_string());
        }
    }
}

fn render_list(title: &str, items: &[String]) -> String {
    if items.is_empty() {
        return format!("### {title}\n\n- none detected\n");
    }
    let mut rendered = format!("### {title}\n\n");
    for item in items {
        rendered.push_str("- ");
        rendered.push_str(item);
        rendered.push('\n');
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_rust_repo() {
        let root = temp_fixture("rust");
        write_file(&root, "Cargo.toml", "[package]\nname = \"demo\"\n");
        write_file(&root, "src/main.rs", "fn main() {}\n");

        let detected = detect_repository(&root).expect("detect");

        assert!(detected.languages.contains(&"Rust".to_string()));
        assert!(detected.package_managers.contains(&"Cargo".to_string()));
        assert!(detected.entrypoints.contains(&"src/main.rs".to_string()));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn detects_typescript_next_repo() {
        let root = temp_fixture("next");
        write_file(
            &root,
            "package.json",
            "{\"dependencies\":{\"next\":\"latest\",\"react\":\"latest\"}}",
        );
        write_file(&root, "next.config.js", "module.exports = {}\n");
        write_file(&root, "src/index.ts", "export {}\n");

        let detected = detect_repository(&root).expect("detect");

        assert!(detected.languages.contains(&"TypeScript".to_string()));
        assert!(detected.frameworks.contains(&"Next.js".to_string()));
        assert!(detected.frameworks.contains(&"React".to_string()));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn detects_python_repo() {
        let root = temp_fixture("python");
        write_file(
            &root,
            "pyproject.toml",
            "[project]\ndependencies = [\"fastapi\"]\n",
        );
        write_file(&root, "app.py", "print('hi')\n");
        write_file(&root, "tests/test_app.py", "def test_x(): pass\n");

        let detected = detect_repository(&root).expect("detect");

        assert!(detected.languages.contains(&"Python".to_string()));
        assert!(detected.frameworks.contains(&"FastAPI".to_string()));
        assert!(detected.tests.contains(&"tests/test_app.py".to_string()));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn ignores_generated_codewiki_files() {
        let root = temp_fixture("generated");
        write_file(&root, "docs/QUICKSTART.md", "# generated\n");
        write_file(&root, "docs/conventions/OVERVIEW.md", "# generated\n");
        write_file(
            &root,
            ".agents/skills/codewiki/project/plan.yml",
            "schema_version: 1\n",
        );
        write_file(
            &root,
            ".agents/skills/codewiki/SKILL.md",
            "# installed skill\n",
        );
        write_file(
            &root,
            ".agents/skills/codewiki/companion/Cargo.toml",
            "[workspace]\nmembers = []\n",
        );
        write_file(
            &root,
            ".agents/skills/codewiki/companion/crates/runtime/src/lib.rs",
            "pub fn codewiki_runtime() {}\n",
        );
        write_file(&root, "README.md", "# source doc\n");

        let detected = detect_repository(&root).expect("detect");

        assert_eq!(detected.docs, vec!["README.md".to_string()]);
        assert!(!detected.languages.contains(&"Rust".to_string()));
        assert!(!detected.package_managers.contains(&"Cargo".to_string()));
        assert!(
            detected
                .entrypoints
                .iter()
                .all(|path| !path.starts_with(".agents/skills/codewiki"))
        );
        let _ = fs::remove_dir_all(root);
    }

    fn temp_fixture(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "codewiki-detect-{name}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time")
        ))
    }

    fn write_file(root: &Path, relative: &str, content: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        fs::write(path, content).expect("write");
    }
}
