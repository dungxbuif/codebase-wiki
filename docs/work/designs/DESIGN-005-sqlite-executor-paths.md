---
artifact_type: detail_design
id: DESIGN-005
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-003
  requirement: REQ-003
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-005-sqlite-executor-paths.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md]
  master_docs_touched: [docs/architecture/ERD.md]
---

# DETAIL DESIGN: SQLite Executor And State Paths

## Status

- ID: DESIGN-005
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-005-sqlite-executor-paths.md`
- Approval: approved by autonomous continuation
- Updated: 2026-07-13

## 1. Context & Scope

### Problem Statement

CodeWiki needs a local runtime state location and a way to apply its bundled SQLite migrations before init/sync can persist evidence.

### Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/work/tickets/TICKET-004-sqlite-state-migrations.md`
- `docs/work/designs/DESIGN-004-sqlite-state-migrations.md`
- `crates/codewiki-store/src/lib.rs`

### Brownfield Scope

- Touched modules/files: `crates/codewiki-store`, ticket/design/validation/context/changelog.
- Direct dependencies inspected: root `Cargo.toml`, `crates/codewiki-store/Cargo.toml`.
- Contracts affected: local runtime state path and migration application.
- Known unknowns: future direct SQLite library binding.

## 2. Design Considerations & Trade-offs

| Consideration / Alternative | Pros | Cons | Decision |
| --- | --- | --- | --- |
| Use `rusqlite` now | Strong typed DB API | Requires dependency fetch and expands surface | Rejected for this slice |
| Use local `sqlite3` executable | No Rust dependency/network, real DB proof | Requires platform executable | Chosen |
| Keep migrations registry-only | Simple | Cannot initialize durable state | Rejected |

## 3. Architecture Overview

`codewiki-store` will provide:

- `RepositoryIdentity`
- `StatePaths`
- path-safe repository storage key generation
- `apply_migrations_with_sqlite`

The executor shells out to `sqlite3`, feeds SQL via stdin, and records each applied migration in `schema_migrations`.

## 4. Execution Flow

1. Build `RepositoryIdentity` from root path and optional Git remote.
2. Resolve app-data/cache paths from an explicit base directory.
3. Create state/cache directories.
4. Open/create the SQLite file.
5. Apply bundled migrations in registry order.
6. Record migration version/name/checksum.

## 5. API & Data Model Design

No public CLI API change in this slice.

## 6. Security & Authorization

- Do not store secrets in state paths or migration SQL.
- Quote SQL literals before inserting migration metadata.
- Use process arguments/stdin instead of shell interpolation.

## 7. Implementation & Verification Plan

- Unit: path key generation and path resolution tests.
- Integration: apply migrations to a temp SQLite DB using `sqlite3` and query `schema_migrations`.
- Docs review: ticket/design/validation/context/changelog.

## 8. Reconciliation Plan

- Requirements docs: no change.
- Architecture docs: no ERD schema change; note execution behavior in ticket/design.
- API docs: no change.
- ADR: no change; ADR-0001 covers SQLite durable state.
- Context: update.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting check passed.

- Command: `rtk cargo test -p codewiki-store`
- Result: pass
- Notes: 10 tests passed across 2 suites, including real SQLite migration application.
