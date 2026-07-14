//! Regression coverage for the packaged skill's mandatory execution boundary.

use std::fs;
use std::path::PathBuf;

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("repository root")
        .to_path_buf()
}

#[test]
fn packaged_skill_requires_preflight_before_reader_doc_writes() {
    let root = repository_root();
    let skill = fs::read_to_string(root.join("skill/codewiki/SKILL.md")).expect("read skill");
    let prompt =
        fs::read_to_string(root.join("skill/codewiki/agents/openai.yaml")).expect("read prompt");
    let preflight = root.join("skill/codewiki/scripts/codewiki-preflight.sh");

    assert!(skill.starts_with("---\nname: codewiki\ndescription:"));
    assert!(skill.contains("## Mandatory Execution Gate"));
    assert!(skill.contains("scripts/codewiki-preflight.sh init"));
    assert!(skill.contains("Do not create or modify reader-facing Markdown"));
    assert!(prompt.starts_with("interface:\n"));
    assert!(prompt.contains("$codewiki"));
    assert!(prompt.contains("codewiki-preflight.sh"));
    assert!(prompt.contains("current working tree"));
    assert!(prompt.contains("repository mental model and WikiPlan"));
    assert!(prompt.contains("reader_docs_ready"));
    assert!(preflight.exists(), "missing bundled preflight entrypoint");
}

#[test]
fn packaged_skill_requires_reader_first_prompt_contract() {
    let root = repository_root();
    let skill = fs::read_to_string(root.join("skill/codewiki/SKILL.md")).expect("read skill");
    let init = fs::read_to_string(root.join("skill/codewiki/references/init.md"))
        .expect("read init reference");
    let reader = fs::read_to_string(root.join("skill/codewiki/references/reader-first.md"))
        .expect("read reader-first reference");
    let structure = fs::read_to_string(root.join("skill/codewiki/references/docs-structure.md"))
        .expect("read docs-structure reference");
    let sync = fs::read_to_string(root.join("skill/codewiki/references/sync.md"))
        .expect("read sync reference");
    let qa = fs::read_to_string(root.join("skill/codewiki/references/qa.md"))
        .expect("read qa reference");
    let package =
        fs::read_to_string(root.join("skill/codewiki/package.yml")).expect("read package");

    assert!(skill.contains("## Always-Active Reader Contract"));
    assert!(skill.contains("## Always-Active Anti-Patterns"));
    assert!(skill.contains("current filesystem working tree"));
    assert!(skill.contains("repository mental model and WikiPlan"));
    assert!(skill.contains("explanation before source inventory"));
    assert!(!skill.contains("  areas/"));
    assert!(skill.contains("codewiki query --text <query>"));
    assert!(skill.contains("codewiki claims --status stale"));

    assert!(init.contains("working tree is the source of truth"));
    assert!(init.contains("relevant untracked source files"));
    assert!(init.contains("Git metadata and diffs do not replace reading current source"));

    assert!(reader.contains("## Mental Model Completion Checklist"));
    for field in [
        "systems",
        "actors",
        "boundaries",
        "runtimes",
        "workflows",
        "state ownership",
        "integrations",
        "change risks",
        "known unknowns",
    ] {
        assert!(reader.contains(field), "missing mental-model field {field}");
    }
    assert!(reader.contains("## Automatic Failure Patterns"));
    assert!(reader.contains("## Docs-Only Onboarding Evaluation"));
    assert!(reader.to_lowercase().contains("capability-specific"));

    assert!(structure.contains("## Page-Type Quality Contracts"));
    assert!(structure.contains("Must not become"));
    for confidence in [
        "`confirmed`",
        "`source-backed`",
        "`hypothesis`",
        "`watchlist`",
    ] {
        assert!(
            structure.contains(confidence),
            "missing confidence label {confidence}"
        );
    }

    assert!(sync.contains("report `wiki is current`"));
    assert!(sync.contains("Do not touch timestamps or rewrite any file"));

    assert!(package.contains("package_version: \"0.3.0\""));
    assert!(package.contains("skill_contract_version: 4"));
    assert!(package.contains("reference_contract_version: 4"));
    assert!(qa.contains("codewiki query --text"));
    assert!(qa.contains("codewiki claims --repo"));

    for forbidden_quota in [
        "pages shorter than 200 words",
        "Min files to inspect",
        "3 consecutive new files",
        "at least 5 source files",
        "Minimum 3 rows",
        "3 most important components",
        "Use exactly these three labels",
        "[Inferred]",
    ] {
        assert!(
            !skill.contains(forbidden_quota)
                && !reader.contains(forbidden_quota)
                && !structure.contains(forbidden_quota)
                && !init.contains(forbidden_quota),
            "count-based quality proxy leaked into prompt contract: {forbidden_quota}"
        );
    }
}
