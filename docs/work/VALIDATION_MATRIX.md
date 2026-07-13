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
| REQ-002 | PHASE-001 | TBD | CodeWiki detects languages, libraries, frameworks, package managers, entrypoints, and architecture signals dynamically | no | no | no | not_required | no | no | planned | none |
| REQ-003 | PHASE-001 | TICKET-003, TICKET-004, TICKET-005 | CodeWiki has deterministic committed config/plan/AGENTS skeleton and versioned durable SQLite state migrations with a local executor/path resolver | yes | yes | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk cargo test -p codewiki-store`; `docs/work/tickets/TICKET-003-config-storage-skeleton.md`; `docs/work/tickets/TICKET-004-sqlite-state-migrations.md`; `docs/work/tickets/TICKET-005-sqlite-executor-paths.md` |
| REQ-004 | PHASE-001 | TBD | Generated wiki claims trace to source evidence, command evidence, or explicit hypotheses | no | no | no | no | no | no | planned | none |
| REQ-005 | PHASE-001 | TBD | CodeWiki uses a minimal replaceable provider boundary for optional runtime code-intelligence tools | no | no | no | not_required | no | no | planned | none |
| REQ-006 | PHASE-001 | TICKET-001 | CodeWiki Rust companion-tool scaffold exists while reference repos remain study inputs | yes | not_required | not_required | not_required | yes | yes | implemented | `rtk cargo test`; `rtk cargo run -p codewiki-cli -- status`; `docs/work/tickets/TICKET-001-rust-cli-workspace.md` |
| REQ-007 | PHASE-001 | TICKET-002 | CodeWiki is installable as a Codex skill from this repository with a single install command | not_required | not_required | not_required | not_required | yes | yes | implemented | `rtk bash -n scripts/install-codewiki-skill.sh`; `rtk git remote -v`; `docs/work/tickets/TICKET-002-skill-first-installer.md` |
| REQ-008 | PHASE-001 | TBD | Runtime provider policy selects Octocode first when needed and gates codebase-memory-mcp/CocoIndex behind specific triggers | not_required | not_required | not_required | not_required | not_required | yes | implemented | `skill/codewiki/SKILL.md`; `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` |
| REQ-009 | PHASE-001 | TBD | CodeWiki uses `docs/codewiki/**` as the generated knowledge surface and `.codewiki/**` as the committed control plane | not_required | not_required | not_required | not_required | not_required | yes | implemented | `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`; `skill/codewiki/SKILL.md`; `skill/codewiki/references/docs-structure.md`; `docs/architecture/ARCHITECTURE.md` |

## Rules

- Add or update a row when a requirement, ticket, bug, public contract, or accepted behavior is created or changed.
- Mark proof columns `yes`, `no`, or `not_required`.
- Link evidence to `docs/templates/TEST_VERIFICATION.md`, ticket verification sections, UAT, docs review, release notes, or command output summaries.
- Do not set `implemented` until required proof has evidence.
- If a proof type is `not_required`, record the reason in the linked ticket, bug, or verification artifact.
