---
artifact_type: architecture_doc
id: ARCH-MASTER
status: active
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

CodeWiki is a skill-first Codex system with a small core:

```text
Repository or source workspace
  -> detection and semantic exploration
  -> evidence and fact model
  -> required LLM repository mental model
  -> WikiPlan v2 and per-page contracts
  -> required LLM reader-doc synthesis
  -> deterministic and semantic quality gates
  -> reader docs in repo-local or external wiki workspace
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
| Skill installer | Atomically install a versioned skill/companion package and preserve declared project state | `scripts/install-codewiki-skill.sh`, `skill/codewiki/package.yml` | Writes `INSTALLATION.yml`; doctor verifies managed content and compatibility |
| Rust companion tool | Discover/persist evidence and validate synthesized output | `crates/codewiki-cli` | Evidence/validation boundary, never the reader-prose producer |
| Core engine | Parse commands and orchestrate deterministic boundaries | `crates/codewiki-core` | Owns `doctor`, `init`, `sync`, `validate`, read-only `query`/`claims`, run states, and provenance |
| Repo detector | Detect languages, frameworks, package managers, entrypoints, test/build tools, and docs | `crates/codewiki-detect` | Dynamic detection only; no core adapters |
| Explorer | Select files/symbols/docs to inspect and record evidence | `crates/codewiki-explore` | Deterministic semantic snapshot v1; lexical hints are evidence, not final claims |
| Evidence store | Persist facts, hypotheses, claims, source references, and claim/evidence links | `crates/codewiki-store` | SQLite local runtime state with migration and persistence helpers |
| WikiPlan contract | Persist the repository mental model, hierarchy, reader jobs/questions, anchors, diagrams, and acceptance checks | `crates/codewiki-store`, `skill/codewiki/references/reader-first.md` | Schema v2 under `.agents/skills/codewiki/project/plan.yml`; model synthesis is required |
| Docs boundary and validator | Write deterministic evidence pages and reject invalid reader-doc output | `crates/codewiki-docs` | Reader prose is synthesized by the skill/model; `validate` gates success |
| Sync engine | Detect stale docs and update safely | `crates/codewiki-core`, `crates/codewiki-docs`, `crates/codewiki-store` | Respects human-owned edits via generated-region markers |
| Q&A engine | Answer from docs first, then local SQLite evidence/source when needed | `crates/codewiki-core`, `crates/codewiki-store`, `skill/codewiki/references/qa.md` | `query` retrieves claims/files/symbols/evidence; `claims` inspects active/stale state without ad-hoc SQL |
| Provider boundary | Wrap optional code-intelligence providers | `crates/codewiki-provider` | Provider selection is target-repo specific |
| Source extension skills | User-authored skills that emit bounded evidence packets for non-Git sources | skill references/templates | Not bundled providers |

## Data Flow

```text
Git repo + files + existing docs + optional source skill evidence
  -> stack detection
  -> deterministic semantic ExplorationSnapshot
       |-> local SQLite files/symbols/evidence/active+stale claims
       |     -> codewiki query / codewiki claims
       |-> docs/evidence/** generated independently from the snapshot
  -> model mental model + WikiPlan + reader docs
  -> sync checkpoints
  -> docs-first Q&A -> SQLite retrieval -> narrow source/provider fallback
```

## Wiki Workspace Documentation Structure

CodeWiki uses three distinct wiki workspace layers. The workspace may be the source repository or a separate external/personal directory:

```text
.agents/skills/codewiki/project/
  config.yml      # committed control config
  plan.yml        # committed semantic WikiPlan and sync plan
  AGENTS.md       # committed local CodeWiki agent guidance
  sources.yml     # primary Git source and optional source skill declarations
  run.yml         # stage states plus installed skill identity
  quality-report.yml # contract/evidence/diagram/cross-page/docs-only results

docs/
  evidence/       # deterministic discovery/claim artifacts from companion init/sync
  QUICKSTART.md   # required only after successful model synthesis
  ...             # concept-first reader docs selected by WikiPlan v2
```

`docs/**` is the human/agent knowledge surface and the first source for Q&A. `.agents/skills/codewiki/project/**` is the committed control plane. SQLite state and rebuildable caches live outside the repository/workspace in platform app-data/cache directories.

Repository-scoped state identity lexically normalizes command paths so `.` and safe `..` aliases resolve the same key. It deliberately does not canonicalize filesystem symlinks: this preserves existing path-derived state keys and avoids silently migrating or merging independently addressed workspaces.

Semantic exploration v1 records bounded file, area, symbol, import/dependency-hint, and evidence-reference snapshots. These hints seed generated docs and future claim promotion, but they are not treated as fully resolved architecture without additional evidence. In project-local mode, detector/explorer traversal excludes the managed `.agents/skills/codewiki/**` runtime so installed skill references and copied companion source cannot become target-repository evidence.

Claim persistence promotes deterministic file-level source-backed structure claims from semantic snapshots and writes repository, run, file, symbol, evidence, claim, and claim/evidence-link rows into local SQLite. Top-level traversal `areas` remain internal exploration metadata and are not promoted as durable claims. `docs/evidence/CLAIMS.md` and SQLite are separate outputs derived from the same snapshot: the Markdown page presents current deterministic claims, while SQLite preserves active/stale history and queryable inventory.

Staleness compares new semantic file content hashes against existing evidence hashes. Changed evidence invalidates linked claims before fresh evidence is persisted; a regenerated identical deterministic claim returns to active, while superseded statements remain stale. A complete snapshot also invalidates claims backed by deleted files and removes their current file/symbol inventory. A truncated snapshot never infers deletion. `codewiki query` renders active/stale claims plus matching files, symbols, and evidence; `codewiki claims` supports status/path inspection.

Production fixture coverage exercises TypeScript app, Python service, and Rust workspace-shaped repositories through evidence-only init, semantic claims/symbols, SQLite Q&A context, and stale sync behavior. Reader-doc benchmark coverage is tracked separately in PHASE-002.

Generated docs use explicit `<!-- codewiki:generated:start -->` / `<!-- codewiki:generated:end -->` regions plus a portable generated-body integrity hash. Sync refreshes a region automatically only when its current body matches that baseline. Manual edits inside or outside the region are preserved; inside-region conflicts and legacy hashless regions are routed to LLM semantic reconciliation. Unmarked pages remain human-owned.

The companion no longer emits deterministic synthesis pages. `init` and `sync` refresh `docs/evidence/**`, set `generation_status: synthesis_incomplete`, and stop. The skill/model must create the repository mental model, replace the WikiPlan scaffold, synthesize reader pages, run isolated reviews, and call `validate`. Only validation may promote the run to `reader_docs_ready`.

ADR-0010 refines the canonical slots from ADR-0005: `QUICKSTART.md` and `conventions/OVERVIEW.md` are unconditional reader pages after success; other pages exist only for evidence-backed reader jobs. Dynamic pages live under semantic owners such as `components/`, `architecture/`, or `workflows/`. `areas/**` is legacy input only and is not regenerated from top-level paths.

## Runtime Flow

```text
Skill workflow: CodeWiki init
  -> resolve repository identity
  -> load committed config if present
  -> open or create local SQLite state
  -> detect stack and repository shape
  -> explore source/docs with bounded semantic snapshot
  -> discover explicit and inferred repository conventions with scope and exceptions
  -> persist files, symbols, evidence, and promoted claims in SQLite
  -> mark claims stale when supporting evidence changes
  -> write evidence pages and WikiPlan v2 scaffold
  -> report synthesis_incomplete
  -> skill/model builds mental model and complete WikiPlan
  -> skill/model synthesizes reader pages and one bounded revision if needed
  -> companion validates plan, provenance, pages, links, and declared quality reviews
  -> report reader_docs_ready only after all gates pass
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
- Missing, legacy, drifted, or incompatible installed skill identities must not report reader-doc success.

## Linked Decisions

- `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`
- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
- `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`
- `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
- `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`
- `docs/decisions/ADR-0007-uppercase-generated-markdown-filenames.md`
- `docs/decisions/ADR-0008-code-conventions-documentation.md`
- `docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md`
- `docs/decisions/ADR-0010-reader-first-information-architecture.md`
- `docs/decisions/ADR-0011-skill-distribution-version-integrity.md`
