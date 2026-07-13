---
artifact_type: architecture_doc
id: ARCH-MASTER
status: draft
owner: shared
human_fields: [approved_boundaries, architectural_constraints, tradeoff_approval]
ai_fields: [overview, modules, diagrams, flows, dependencies, risks]
shared_fields: [status, linked_decisions]
---

# Architecture: CodeWiki

## Field Ownership

- Human owns approved boundaries, constraints, and tradeoff approvals.
- AI maintains implementation-grounded overview, modules, diagrams, flows, dependencies, and risks.

## Overview

CodeWiki is planned as a skill-first Codex system with a small core:

```text
Repository or source workspace
  -> detection and semantic exploration
  -> evidence and fact model
  -> WikiPlan
  -> generated docs in repo-local or external wiki workspace
  -> sync and Q&A
```

The core should not hard-code language/framework adapters. It should combine repository signals, Git context, filesystem reads, LLM exploration, and a replaceable code-intelligence provider boundary. The skill is the product surface; Rust is companion tooling for deterministic local operations.

## System Boundaries

- In scope: repository detection, semantic exploration, evidence modeling, wiki planning, docs generation, sync, Q&A, config, source registry, workspace placement, and durable local state.
- Out of scope for core: language/framework-specific adapters, bundled Jira/Figma/etc. providers, and broad mandatory external memory/indexing stacks.

## Modules

| Module | Responsibility | Key Files | Notes |
| --- | --- | --- | --- |
| CodeWiki skill | Own agent workflow for init, sync, Q&A, evidence, and docs | `skill/codewiki/SKILL.md` | Primary product surface |
| Skill installer | Install the skill into Codex home from this repo | `scripts/install-codewiki-skill.sh` | One-command install path |
| Rust companion tool | Provide deterministic helper commands for repo inspection/config/state when needed | `crates/codewiki-cli` | Companion surface, not the product |
| Core engine | Parse commands and orchestrate internal boundaries | `crates/codewiki-core` | Owns current `help`, `version`, and `status` behavior |
| Repo detector | Detect languages, frameworks, package managers, entrypoints, test/build tools, and docs | `crates/codewiki-detect` | Dynamic detection only; no core adapters |
| Explorer | Select files/symbols/docs to inspect and record evidence | `crates/codewiki-explore` | Deterministic semantic snapshot v1; lexical hints are evidence, not final claims |
| Evidence store | Persist facts, hypotheses, claims, source references, and claim/evidence links | `crates/codewiki-store` | SQLite local runtime state with migration and persistence helpers |
| WikiPlan generator | Produce page plan, scope, confidence, open questions, and refresh strategy | TBD | Committed summary planned under `.codewiki/` |
| Doc generator | Write human/agent-readable wiki docs | `crates/codewiki-docs` | Canonical generated docs root is `docs/**` |
| Sync engine | Detect stale docs and update safely | TBD | Must respect human-owned edits |
| Q&A engine | Answer from docs first, then evidence/source when needed | TBD | Should cite evidence |
| Provider boundary | Wrap optional code-intelligence providers | `crates/codewiki-provider` | Provider selection is target-repo specific |
| Source extension skills | User-authored skills that emit bounded evidence packets for non-Git sources | skill references/templates | Not bundled providers |

## Data Flow

```text
Git repo + files + existing docs + optional source skill evidence
  -> stack detection
  -> deterministic semantic exploration
  -> evidence/fact/hypothesis records
  -> WikiPlan
  -> generated docs
  -> sync checkpoints
  -> docs-first Q&A
```

## Wiki Workspace Documentation Structure

CodeWiki uses three distinct wiki workspace layers. The workspace may be the source repository or a separate external/personal directory:

```text
.codewiki/
  config.yml      # committed control config
  plan.yml        # committed semantic WikiPlan and sync plan
  AGENTS.md       # committed local CodeWiki agent guidance
  sources.yml     # primary Git source and optional source skill declarations

docs/
  index.md        # required generated wiki entrypoint
  ...             # canonical generated semantic docs
```

`docs/**` is the human/agent knowledge surface and the first source for Q&A. `.codewiki/**` is the committed control plane. SQLite state and rebuildable caches live outside the repository/workspace in platform app-data/cache directories.

Semantic exploration v1 records bounded file, area, symbol, import/dependency-hint, and evidence-reference snapshots. These hints seed generated docs and future claim promotion, but they are not treated as fully resolved architecture without additional evidence.

Claim persistence v1 promotes deterministic source-backed structure claims from semantic snapshots and writes repository, run, file, symbol, evidence, claim, and claim/evidence-link rows into local SQLite. Generated `docs/evidence/claims.md` mirrors those promoted claims so docs-first Q&A and SQLite-backed Q&A can share the same evidence base.

Staleness v1 compares new semantic file content hashes against existing evidence hashes. When supporting file evidence changes, linked active claims are marked `stale` before new evidence is persisted. Q&A retrieval renders active and stale SQLite claims separately so agents can answer from fresh docs/state first and inspect stale source paths narrowly when needed.

Production fixture coverage now exercises TypeScript app, Python service, and Rust workspace-shaped repositories through init, generated docs, semantic claims, SQLite Q&A context, and stale sync behavior.

Generated docs use explicit `<!-- codewiki:generated:start -->` / `<!-- codewiki:generated:end -->` regions. Sync updates only those regions and preserves human-owned text outside them. If an existing changed page has no generated markers, sync preserves it instead of overwriting it.

Synthesis pages are generated for canonical wiki slots including domains, workflows, data, interfaces, operations, testing, decisions, glossary, open questions, and observed areas. These pages are deterministic evidence summaries: when evidence is thin, they record gaps rather than claiming complete understanding.

The canonical generated docs slots are defined by `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`: `quickstart.md`, `source-map.md`, section directories such as `architecture/`, `domain/`, `workflows/`, `data-models/`, `api/`, `operations/`, `testing/`, top-level `glossary.md` and `open-questions.md`, `evidence/**`, and optional `areas/<area-slug>/overview.md`.

## Runtime Flow

```text
Skill workflow: CodeWiki init
  -> resolve repository identity
  -> load committed config if present
  -> open or create local SQLite state
  -> detect stack and repository shape
  -> explore source/docs with bounded semantic snapshot
  -> persist files, symbols, evidence, and promoted claims in SQLite
  -> mark claims stale when supporting evidence changes
  -> create WikiPlan
  -> generate docs
  -> write checkpoints for future sync
```

## Dependencies

- Git
- Filesystem access
- SQLite
- Codex skill runtime
- Rust toolchain and crates for companion tooling
- Optional code-intelligence provider boundary

## Risks And Tradeoffs

- Automatic init needs strong evidence and confidence modeling because it intentionally does not pause for approval.
- Provider lock-in is avoided by keeping the provider boundary narrow.
- Human edits in generated docs need ownership rules before sync mutates them.
- SQLite migrations must exist early because durable state is a product promise.
- The reference submodules should inform design, but CodeWiki should not inherit their runtime architecture wholesale.
- Rust companion tooling must not displace the skill as the primary UX.

## Linked Decisions

- `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`
- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
- `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`
- `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
- `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`
