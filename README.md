# CodeWiki

CodeWiki is planned as a repo-native Codex skill for generating, syncing, and querying a high-quality semantic wiki for any software repository.

The intended shape is not a thin documentation generator. CodeWiki should explore a repository, detect languages/frameworks/libraries dynamically, build an evidence-backed wiki plan, generate durable docs, and preserve reusable local state across sessions, model changes, and future updates.

## Current Repository State

This repository currently contains the project control harness and upstream reference submodules:

- `references/openwiki`: LangChain OpenWiki reference implementation.
- `references/deepwiki-open`: AsyncFuncAI DeepWiki Open reference implementation.
- `skill/codewiki`: installable Codex skill package.
- `scripts/install-codewiki-skill.sh`: repository-based skill installer.
- `docs/`: Harness-based SDLC control surface for requirements, phases, decisions, validation, and durable project context.
- `crates/`: Rust companion tooling for deterministic local operations.

## Install Skill

After this repository is pushed, install the skill with:

```bash
curl -fsSL https://raw.githubusercontent.com/dungxbuif/harness/main/scripts/install-codewiki-skill.sh | bash
```

From a local checkout, install with:

```bash
scripts/install-codewiki-skill.sh
```

## Product Direction

- Automatic `init`: LLM explores source code and creates wiki output without approval gates.
- Works across repositories without core language/framework adapters.
- Uses detection and semantic exploration to understand each repository.
- Keeps committed config/docs separate from local persistent runtime state and rebuildable cache.
- Keeps evidence with claims so generated docs can be trusted, checked, and refreshed.
- Uses a small tool surface. The preferred initial code intelligence candidate is Octocode, with Git/filesystem/SQLite as baseline infrastructure.
- Treats Rust as companion tooling for the skill, not as the main product interface.

## Project Control

Start with:

- `AGENTS.md`
- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/work/ROADMAP.md`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`

Harness source reference: <https://github.com/dungxbuif/harness>
