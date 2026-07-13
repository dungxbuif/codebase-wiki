---
artifact_type: validation_matrix
id: VALIDATION_MATRIX
status: active
owner: shared
human_fields:
  - proof_override
  - acceptance_signoff
ai_fields:
  - proof_recommendation
  - evidence_links
  - status_updates
shared_fields:
  - matrix_rows
  - validation_status
updated: 2026-07-13
---

# Validation Matrix

## Field Ownership

- Human owns proof overrides and acceptance sign-off.
- AI recommends proof types, links evidence, and updates status from verification.

This file maps accepted behavior and work items to proof.

Policy lives in `docs/standards/VALIDATION.md`. This matrix is runtime project state and should change as work is planned, implemented, changed, or retired.

## Status Values

| Status | Meaning |
| --- | --- |
| planned | Accepted as intended behavior, not implemented |
| in_progress | Actively being built or verified |
| implemented | Implemented and evidence exists |
| changed | Contract or expected proof changed after earlier implementation |
| retired | No longer part of the accepted project contract |

## Matrix

| Requirement | Phase | Ticket/Bug | Contract/Behavior | Unit | Integration | E2E | UAT | Platform/Manual | Docs Review | Status | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| REQ-001 | PHASE-001 | TICKET-006 | `codewiki init` creates the initial CodeWiki file/state skeleton without approval gates | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test -p codewiki-core -p codewiki-docs -p codewiki-store`; `rtk proxy env CODEWIKI_APP_DATA_DIR=/tmp/codewiki-state-smoke CODEWIKI_CACHE_DIR=/tmp/codewiki-cache-smoke cargo run -p codewiki-cli -- init /tmp/codewiki-init-smoke`; `docs/work/tickets/TICKET-006-init-skeleton.md` |
| REQ-002 | PHASE-001 | TICKET-007 | CodeWiki detects languages, package managers, framework/library hints, entrypoints, tests, and docs signals dynamically | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test -p codewiki-detect -p codewiki-core -p codewiki-docs -p codewiki-store`; `docs/work/tickets/TICKET-007-repo-detection-v1.md` |
| REQ-003 | PHASE-001 | TICKET-003, TICKET-004, TICKET-005, TICKET-013 | CodeWiki has deterministic committed config/plan/AGENTS skeleton, versioned durable SQLite state migrations, local executor/path resolver, and persisted semantic claims/evidence | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-docs -p codewiki-core`; `rtk cargo test -p codewiki-store`; `docs/work/tickets/TICKET-003-config-storage-skeleton.md`; `docs/work/tickets/TICKET-004-sqlite-state-migrations.md`; `docs/work/tickets/TICKET-005-sqlite-executor-paths.md`; `docs/work/tickets/TICKET-013-claim-persistence-v1.md` |
| REQ-004 | PHASE-001 | TICKET-008, TICKET-009, TICKET-012, TICKET-013 | Generated wiki claims have typed evidence, confidence, WikiPlan page ownership models, canonical starter evidence pages, semantic source evidence snapshots, and durable SQLite claim/evidence links | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-docs -p codewiki-core`; `rtk cargo test -p codewiki-store -p codewiki-core -p codewiki-detect -p codewiki-docs`; `rtk cargo test -p codewiki-docs -p codewiki-core -p codewiki-store -p codewiki-detect`; `docs/work/tickets/TICKET-008-wikiplan-evidence-models.md`; `docs/work/tickets/TICKET-009-canonical-docs-generator.md`; `docs/work/tickets/TICKET-012-semantic-exploration-v1.md`; `docs/work/tickets/TICKET-013-claim-persistence-v1.md` |
| REQ-004 | PHASE-001 | TICKET-014 | CodeWiki marks claims stale when supporting evidence changes and renders SQLite-backed Q&A context with active/stale claim separation | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-core`; `docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md`; `skill/codewiki/references/qa.md` |
| REQ-001, REQ-002, REQ-004 | PHASE-001 | TICKET-015 | CodeWiki production fixture suite verifies init, semantic docs, claims, SQLite Q&A context, and stale sync across TypeScript, Python, and Rust-shaped repos | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test -p codewiki-core --test production_fixtures`; `rtk cargo test`; `docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md` |
| REQ-004 | PHASE-001 | TICKET-016 | CodeWiki sync preserves human-owned text by updating only marked generated regions and preserving unmarked changed docs | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test -p codewiki-core -p codewiki-docs`; `rtk cargo test`; `docs/work/tickets/TICKET-016-sync-safety-generated-regions.md` |
| REQ-004 | PHASE-001 | TICKET-017 | CodeWiki generates evidence-bound synthesis pages for canonical wiki slots and observed areas | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test -p codewiki-docs -p codewiki-core`; `rtk cargo test`; `docs/work/tickets/TICKET-017-synthesis-pages-v1.md`; `skill/codewiki/references/docs-structure.md` |
| REQ-002 | PHASE-001 | TICKET-012 | CodeWiki emits bounded semantic exploration snapshots with files, roles, symbols, imports/dependency hints, areas, and evidence references | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk cargo test -p codewiki-explore -p codewiki-docs -p codewiki-core`; `docs/work/tickets/TICKET-012-semantic-exploration-v1.md` |
| REQ-010 | PHASE-001 | TICKET-010 | `codewiki sync` compares generated outputs, updates stale generated files, and no-ops when current | yes | yes | not_required | not_required | no | yes | implemented | `rtk cargo test -p codewiki-core -p codewiki-docs -p codewiki-store -p codewiki-detect`; `docs/work/tickets/TICKET-010-sync-skeleton.md` |
| REQ-011 | PHASE-001 | TICKET-011 | CodeWiki supports repo-local/external wiki placement and user-provided non-Git source extension skills while Git remains the default source | yes | yes | not_required | not_required | not_required | yes | implemented | `rtk cargo test`; `rtk cargo test -p codewiki-core -p codewiki-store`; `skill/codewiki/references/workspace-placement.md`; `skill/codewiki/references/source-extensions.md`; `skill/codewiki/references/source-skill-template.md`; `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md` |
| REQ-005 | PHASE-001 | TICKET-018, TICKET-019 | CodeWiki uses a minimal replaceable provider boundary for optional runtime code-intelligence tools | not_required | not_required | not_required | not_required | not_required | yes | implemented | `skill/codewiki/SKILL.md`; `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`; `docs/standards/CODEWIKI.md` |
| Standards | PHASE-001 | TICKET-019 | CodeWiki-specific product, evidence, sync-safety, and provider standards are documented | not_required | not_required | not_required | not_required | not_required | yes | implemented | `docs/standards/CODEWIKI.md`; `docs/work/tickets/TICKET-019-codewiki-standards-and-status.md` |
| REQ-006 | PHASE-001 | TICKET-001, TICKET-021 | CodeWiki Rust companion-tool scaffold exists while reference repos remain study inputs and installed skills prefer a built companion binary | yes | not_required | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk cargo run -p codewiki-cli -- status`; `rtk bash /private/tmp/codewiki-install-binary-test/skills/codewiki/scripts/codewiki-helper.sh status`; `docs/work/tickets/TICKET-001-rust-cli-workspace.md`; `docs/work/tickets/TICKET-021-binary-first-companion-install.md` |
| REQ-007 | PHASE-001 | TICKET-002, TICKET-020, TICKET-021 | CodeWiki is installable as a Codex skill from this repository with a single install command and bundled Rust companion binary/helper/source fallback | not_required | not_required | not_required | not_required | yes | yes | implemented | `rtk bash -n scripts/install-codewiki-skill.sh`; `rtk bash -n skill/codewiki/scripts/codewiki-helper.sh`; `rtk env CODEWIKI_REPO=/Users/dungxbuif/Documents/codebase-wiki bash skill/codewiki/scripts/codewiki-helper.sh status`; `rtk env CODEWIKI_REPO_URL=/Users/dungxbuif/Documents/codebase-wiki CODEX_HOME=/private/tmp/codewiki-install-binary-test bash scripts/install-codewiki-skill.sh`; `rtk git remote -v`; `docs/work/tickets/TICKET-002-skill-first-installer.md`; `docs/work/tickets/TICKET-020-direct-docs-root-and-skill-helper.md`; `docs/work/tickets/TICKET-021-binary-first-companion-install.md` |
| REQ-008 | PHASE-001 | TBD | Runtime provider policy selects Octocode first when needed and gates codebase-memory-mcp/CocoIndex behind specific triggers | not_required | not_required | not_required | not_required | not_required | yes | implemented | `skill/codewiki/SKILL.md`; `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` |
| REQ-009 | PHASE-001 | TICKET-020, TICKET-022 | CodeWiki writes generated wiki pages directly under `docs/` using OpenWiki-style quickstart/section directories and DeepWiki-style relevant-source page context while `.codewiki/**` remains the committed control plane | yes | yes | not_required | not_required | not_required | yes | implemented | `rtk cargo test`; `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`; `skill/codewiki/SKILL.md`; `skill/codewiki/references/docs-structure.md`; `docs/architecture/ARCHITECTURE.md`; `docs/work/tickets/TICKET-020-direct-docs-root-and-skill-helper.md`; `docs/work/tickets/TICKET-022-openwiki-deepwiki-docs-patterns.md` |
| Release readiness | PHASE-001 | TICKET-018 | Top-level docs, roadmap, traceability, installer syntax, companion status, and full workspace tests are reconciled for the completed foundation | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk bash -n scripts/install-codewiki-skill.sh`; `rtk cargo run -p codewiki-cli -- status`; `docs/work/tickets/TICKET-018-release-readiness.md` |

## Rules

- Add or update a row when a requirement, ticket, bug, public contract, or accepted behavior is created or changed.
- Mark proof columns `yes`, `no`, or `not_required`.
- Link evidence to `docs/templates/TEST_VERIFICATION.md`, ticket verification sections, UAT, docs review, release notes, or command output summaries.
- Do not set `implemented` until required proof has evidence.
- If a proof type is `not_required`, record the reason in the linked ticket, bug, or verification artifact.
