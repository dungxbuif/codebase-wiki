# CodeWiki

CodeWiki is a repo-native Codex skill for generating, syncing, and querying a high-quality semantic wiki for any software repository.

The intended shape is not a thin documentation generator. CodeWiki should explore a repository, detect languages/frameworks/libraries dynamically, build an evidence-backed wiki plan, generate durable docs, and preserve reusable local state across sessions, model changes, and future updates.

## Current Repository State

This repository contains the project control harness, installable CodeWiki skill, upstream reference submodules, and Rust companion helpers:

- `references/openwiki`: LangChain OpenWiki reference implementation.
- `references/deepwiki-open`: AsyncFuncAI DeepWiki Open reference implementation.
- `skill/codewiki`: installable Codex skill package.
- `scripts/install-codewiki-skill.sh`: repository-based skill installer.
- `docs/`: Harness-based SDLC control surface for requirements, phases, decisions, validation, and durable project context.
- `crates/`: Rust companion tooling for deterministic local operations.

Implemented foundation capabilities:

- skill-first workflow and repository installer;
- repo-local or external/personal wiki workspace placement;
- Git as the default source plus user-authored source skill extension template;
- dynamic repository detection without core framework adapters;
- semantic exploration snapshots for files, areas, symbols, imports, evidence, and claims;
- SQLite durable state for files, symbols, evidence, claims, claim/evidence links, stale claims, and Q&A context;
- read-only `codewiki query` and `codewiki claims` commands so agents can consume that state without ad-hoc SQL;
- evidence-only companion init/sync plus required model-driven repository mental model, WikiPlan v2, and concept-first reader synthesis;
- deterministic reader-doc validation for provenance, contracts, portable links, artifact hygiene, navigation, and declared isolated quality reviews;
- sync safety with portable generated-body integrity that preserves manual edits inside and outside generated regions for semantic reconciliation;
- production fixture tests for TypeScript, Python, and Rust-shaped repositories.
- versioned, doctor-verified skill packages with atomic replacement, managed-content digest, run provenance, and binary/source fallback.

## Install Skill

From the target code repository, install the skill into that repository with:

```bash
curl -fsSL https://raw.githubusercontent.com/dungxbuif/codebase-wiki/master/scripts/install-codewiki-skill.sh | bash
```

By default this writes to:

```text
<target-repo>/.agents/skills/codewiki
```

From a local checkout, install with:

```bash
scripts/install-codewiki-skill.sh
```

To install globally into `$CODEX_HOME/skills/codewiki`, opt in explicitly:

```bash
curl -fsSL https://raw.githubusercontent.com/dungxbuif/codebase-wiki/master/scripts/install-codewiki-skill.sh | CODEWIKI_INSTALL_SCOPE=global bash
```

The installer stages and validates `skill/codewiki`, builds `bin/codewiki` when compatible Cargo/Rust is available, preserves declared `project/**` state, writes `INSTALLATION.yml`, and activates the package with a rollback-safe rename. The helper verifies the installed digest and interface before deterministic companion operations, including init/sync/validate/query/claims.

Useful diagnostics:

```bash
.agents/skills/codewiki/scripts/codewiki-helper.sh status
.agents/skills/codewiki/bin/codewiki doctor .agents/skills/codewiki
```

After init/sync has created local state, inspect it deterministically:

```bash
.agents/skills/codewiki/bin/codewiki query --text "authentication flow" --repo .
.agents/skills/codewiki/bin/codewiki claims --repo . --status stale
```

## Product Direction

- Automatic `init`: LLM explores source code and creates wiki output without approval gates.
- Companion `init` is intentionally evidence-only and reports `synthesis_incomplete`; the skill completes model planning/synthesis and must pass `codewiki validate` before claiming onboarding-ready docs.
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
