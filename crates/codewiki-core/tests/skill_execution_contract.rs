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
    assert!(prompt.contains("reader_docs_ready"));
    assert!(preflight.exists(), "missing bundled preflight entrypoint");
}
