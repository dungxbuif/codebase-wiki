---
artifact_type: detail_design
id: DESIGN-006
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-001
  requirement: REQ-001
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-006-init-skeleton.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  master_docs_touched: [docs/architecture/API.md]
---

# DETAIL DESIGN: Init Skeleton

## Status

- ID: DESIGN-006
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-006-init-skeleton.md`
- Approval: approved by autonomous continuation
- Updated: 2026-07-13

## 1. Context & Scope

Implement the first runnable `codewiki init [path]` companion command. This is a deterministic skeleton, not full semantic exploration yet.

Scope:

- Create committed CodeWiki control files.
- Create `docs/quickstart.md`.
- Apply local SQLite migrations.
- Keep writes idempotent and non-overwriting.

Out of scope:

- Full repo detection.
- Full WikiPlan v1.
- Full semantic docs generation.
- Provider activation.

## 2. Design

`codewiki-core` will parse `init [path]` and use:

- `codewiki-store` for config/plan/AGENTS rendering, state path resolution, and SQLite migration application.
- `codewiki-docs` for the initial index page.

The init command writes missing files only. Existing files are reported as preserved.

## 3. Verification

- Unit/integration tests create temp repositories and temp state/cache roots.
- Tests assert required files exist.
- Tests assert existing config content is preserved.
- Tests assert `schema_migrations` exists by relying on successful migration application.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting check passed.

- Command: `rtk cargo test -p codewiki-core -p codewiki-docs -p codewiki-store`
- Result: pass
- Notes: 18 tests passed across 6 suites.

- Command: `rtk proxy env CODEWIKI_APP_DATA_DIR=/tmp/codewiki-state-smoke CODEWIKI_CACHE_DIR=/tmp/codewiki-cache-smoke cargo run -p codewiki-cli -- init /tmp/codewiki-init-smoke`
- Result: pass
- Notes: smoke test created target repo CodeWiki files and local SQLite DB under `/tmp`.
