use codewiki_core::{RuntimeContext, run_with_context};
use codewiki_store::render_qa_context_with_sqlite;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn production_fixtures_initialize_docs_state_and_qa_context() {
    for fixture in [
        FixtureKind::TypeScriptApp,
        FixtureKind::PythonService,
        FixtureKind::RustWorkspace,
    ] {
        let base = temp_path(fixture.name());
        let repo = base.join("repo");
        fixture.write(&repo);
        let context = context_for(&base, &repo);

        let output = run_with_context(["init"], &context);

        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert!(output.stdout.contains("claims_persisted:"));
        assert!(
            repo.join(".agents/skills/codewiki/project/config.yml")
                .exists()
        );
        assert!(
            repo.join(".agents/skills/codewiki/project/sources.yml")
                .exists()
        );
        assert!(repo.join("docs/QUICKSTART.md").exists());
        assert!(repo.join("docs/SOURCE-MAP.md").exists());
        assert!(repo.join("docs/domain/OVERVIEW.md").exists());
        assert!(repo.join("docs/api/OVERVIEW.md").exists());
        assert!(repo.join("docs/conventions/OVERVIEW.md").exists());
        assert!(repo.join("docs/OPEN-QUESTIONS.md").exists());
        assert!(repo.join("docs/evidence/CLAIMS.md").exists());

        let map = fs::read_to_string(repo.join("docs/SOURCE-MAP.md")).expect("read map");
        assert!(map.contains("Semantic Structure"));
        assert!(map.contains("Dependency Hints"));
        assert!(map.contains(fixture.expected_signal()));
        let interfaces = fs::read_to_string(repo.join("docs/api/OVERVIEW.md")).expect("interfaces");
        assert!(interfaces.contains(fixture.expected_interface_signal()));
        let conventions =
            fs::read_to_string(repo.join("docs/conventions/OVERVIEW.md")).expect("conventions");
        assert!(conventions.contains("## Evidence Standard"));
        assert!(conventions.contains("Required LLM Synthesis"));

        let claims = fs::read_to_string(repo.join("docs/evidence/CLAIMS.md")).expect("claims");
        assert!(claims.contains("claim:"));
        assert!(claims.contains("evidence:"));

        let qa = render_qa_context_with_sqlite(
            &context.sqlite_executable,
            find_state_db(&context),
            fixture.qa_query(),
            10,
        )
        .expect("qa context");
        assert!(qa.active_claims_seen >= 1, "{}", qa.markdown);
        assert!(qa.markdown.contains("Active Claims"));

        let _ = fs::remove_dir_all(base);
    }
}

#[test]
fn production_fixture_sync_marks_changed_evidence_stale() {
    let base = temp_path("codewiki-fixture-stale");
    let repo = base.join("repo");
    FixtureKind::TypeScriptApp.write(&repo);
    let context = context_for(&base, &repo);

    let init = run_with_context(["init"], &context);
    assert_eq!(init.exit_code, 0, "{}", init.stderr);

    fs::write(
        repo.join("src/App.tsx"),
        "import React from 'react';\nexport function App() { return <main>changed</main>; }\nexport function extra() { return null; }\n",
    )
    .expect("mutate source");
    let sync = run_with_context(["sync"], &context);

    assert_eq!(sync.exit_code, 0, "{}", sync.stderr);
    assert!(sync.stdout.contains("stale_claims:"));

    let qa = render_qa_context_with_sqlite(
        &context.sqlite_executable,
        find_state_db(&context),
        "src/App.tsx",
        10,
    )
    .expect("qa context");
    assert!(qa.stale_claims_seen >= 1, "{}", qa.markdown);
    assert!(qa.markdown.contains("Stale Claims"));

    let _ = fs::remove_dir_all(base);
}

#[derive(Debug, Clone, Copy)]
enum FixtureKind {
    TypeScriptApp,
    PythonService,
    RustWorkspace,
}

impl FixtureKind {
    fn name(self) -> &'static str {
        match self {
            Self::TypeScriptApp => "codewiki-fixture-ts",
            Self::PythonService => "codewiki-fixture-python",
            Self::RustWorkspace => "codewiki-fixture-rust",
        }
    }

    fn expected_signal(self) -> &'static str {
        match self {
            Self::TypeScriptApp => "src/App.tsx",
            Self::PythonService => "app/main.py",
            Self::RustWorkspace => "crates/api/src/lib.rs",
        }
    }

    fn qa_query(self) -> &'static str {
        self.expected_signal()
    }

    fn expected_interface_signal(self) -> &'static str {
        match self {
            Self::TypeScriptApp => "App",
            Self::PythonService => "health",
            Self::RustWorkspace => "serve",
        }
    }

    fn write(self, repo: &Path) {
        match self {
            Self::TypeScriptApp => {
                fs::create_dir_all(repo.join("src")).expect("mkdir ts src");
                fs::write(
                    repo.join("package.json"),
                    r#"{"dependencies":{"@vitejs/plugin-react":"latest","react":"latest","typescript":"latest"},"devDependencies":{"vite":"latest"}}"#,
                )
                .expect("package");
                fs::write(
                    repo.join("src/App.tsx"),
                    "import React from 'react';\nexport interface AppProps { title: string }\nexport function App(props: AppProps) { return <main>{props.title}</main>; }\n",
                )
                .expect("app");
                fs::write(repo.join("README.md"), "# TS fixture\n").expect("readme");
            }
            Self::PythonService => {
                fs::create_dir_all(repo.join("app")).expect("mkdir app");
                fs::create_dir_all(repo.join("tests")).expect("mkdir tests");
                fs::write(
                    repo.join("pyproject.toml"),
                    "[project]\ndependencies = ['fastapi', 'pytest']\n",
                )
                .expect("pyproject");
                fs::write(
                    repo.join("app/main.py"),
                    "from fastapi import FastAPI\napp = FastAPI()\ndef health():\n    return {'ok': True}\n",
                )
                .expect("main");
                fs::write(
                    repo.join("tests/test_health.py"),
                    "def test_health(): assert True\n",
                )
                .expect("test");
            }
            Self::RustWorkspace => {
                fs::create_dir_all(repo.join("crates/api/src")).expect("mkdir rust crate");
                fs::write(
                    repo.join("Cargo.toml"),
                    "[workspace]\nmembers = ['crates/api']\n",
                )
                .expect("cargo workspace");
                fs::write(
                    repo.join("crates/api/src/lib.rs"),
                    "use std::sync::Arc;\npub struct Api;\npub fn serve() -> Arc<Api> { Arc::new(Api) }\n",
                )
                .expect("lib");
            }
        }
    }
}

fn context_for(base: &Path, repo: &Path) -> RuntimeContext {
    RuntimeContext {
        cwd: repo.to_path_buf(),
        app_data_base: base.join("app-data"),
        cache_base: base.join("cache"),
        sqlite_executable: sqlite_path(),
    }
}

fn sqlite_path() -> PathBuf {
    if Path::new("/usr/bin/sqlite3").exists() {
        PathBuf::from("/usr/bin/sqlite3")
    } else {
        PathBuf::from("sqlite3")
    }
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

fn temp_path(prefix: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{suffix}"))
}
