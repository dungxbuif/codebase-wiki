---
artifact_type: detail_design
id: DESIGN-004
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-003
  requirement: REQ-003
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-004-sqlite-state-migrations.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md]
  master_docs_touched: [docs/architecture/ERD.md]
---

# DETAIL DESIGN: SQLite State Migrations

## Status

- ID: DESIGN-004
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-004-sqlite-state-migrations.md`
- Approval: approved by autonomous continuation
- Updated: 2026-07-13

## 1. Context & Scope

### Problem Statement

CodeWiki needs durable local runtime state that survives model changes and session resets. The first implementation slice should define a schema and migration boundary before runtime init/sync begins writing state.

### Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`
- `docs/standards/QUALITY_BAR.md`
- `docs/standards/VALIDATION.md`
- `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`
- `docs/architecture/ERD.md`
- `crates/codewiki-store/src/lib.rs`

### Brownfield Scope

- Touched modules/files: `crates/codewiki-store`, `docs/architecture/ERD.md`, ticket/design/validation/context/changelog.
- Direct dependencies inspected: root `Cargo.toml`, `crates/codewiki-store/Cargo.toml`.
- Contracts affected: local durable state schema.
- Known unknowns: final SQLite executor crate/tool; exact app-data path resolver.
- Scope expansion reason: none.

## 2. Design Considerations & Trade-offs

| Consideration / Alternative | Pros | Cons | Decision |
| --- | --- | --- | --- |
| Add `rusqlite` now | Real execution tests immediately | Requires dependency fetch and locks executor choice early | Rejected for this slice |
| Store versioned SQL resources plus Rust registry | Deterministic, no network, executor-agnostic | Does not execute SQLite yet | Chosen |
| Single JSON state file | Simple | Weak queryability and migration story | Rejected |
| Minimal schema only for pages | Fast | Fails evidence/semantic quality requirement | Rejected |

## 3. Architecture Overview

`codewiki-store` will expose:

- a `Migration` struct;
- `available_migrations()`;
- `latest_migration_version()`;
- embedded SQL migration resources.

The first migration defines durable tables for:

- repository identity;
- sync runs;
- source/generated files;
- symbols;
- generated pages;
- evidence items;
- claims and claim-evidence links;
- provider snapshots;
- open questions.

## 4. Execution Flow

1. Runtime opens local SQLite DB for repository identity.
2. Runtime reads `available_migrations()`.
3. Runtime checks `schema_migrations`.
4. Runtime applies unapplied migrations in version order.
5. Runtime records applied version/checksum.

Executor implementation is out of scope for this slice.

## 5. API & Data Model Design

### API Changes

No public CLI command change.

### Data Model Changes

Schema migration needed: yes.

Core entities:

- `repositories`
- `sync_runs`
- `files`
- `symbols`
- `pages`
- `evidence_items`
- `claims`
- `claim_evidence`
- `provider_snapshots`
- `open_questions`

## 6. Security & Authorization

- Authentication changes: none.
- Authorization / Permissions: local filesystem/database only.
- Data Privacy / PII impact: evidence summaries may reference source paths and command summaries; do not store secrets.
- Input Validation: future executor must parameterize writes; this slice only defines schema resources.

## 7. Implementation & Verification Plan

### Impacted Areas

- Code/modules: `crates/codewiki-store`
- Product behavior: durable state contract
- API/contracts: internal Rust store API
- Data/schema: SQLite migration
- Security/auth: no auth changes
- Deployment/runtime: no runtime execution change yet
- Docs: ERD, validation, ticket/design, context/changelog

### Test Plan

- Unit: `rtk cargo test -p codewiki-store`
- Integration: not required
- E2E: not required
- UAT: not required because internal schema contract
- Manual/platform: not required
- Docs review: required

### Validation Matrix Impact

- Update required: yes
- Row(s): `REQ-003`

### Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting check passed.

- Command: `rtk cargo test -p codewiki-store`
- Result: pass
- Notes: 7 tests passed across 2 suites.

## 8. Reconciliation Plan

- Requirements docs: no change unless wording proves stale.
- Architecture docs: update ERD.
- API docs: no change.
- ERD/data docs: update.
- ADR: no change; ADR-0001 covers SQLite durable state.
- Context: update.
