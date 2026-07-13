---
artifact_type: ticket
id: TICKET-015
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-001, REQ-002, REQ-004
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-015-production-fixtures-eval-suite.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-015 Production Fixtures Eval Suite

## Status

- ID: TICKET-015
- Status: done
- Type: feature/quality
- Priority: high
- Phase: PHASE-001

## Problem

Unit tests prove individual helpers, but production CodeWiki needs fixture-level coverage across different repository shapes. Without this, semantic detection/exploration/docs/state can regress while unit tests still pass.

## Acceptance Criteria

- [x] Add integration fixtures for multiple repo shapes.
- [x] Verify init creates docs, plan, sources, enriched semantic map, claims, and SQLite state.
- [x] Verify Q&A context retrieves claims from persisted state.
- [x] Verify sync detects stale claims after source changes.
- [x] Keep fixtures local/generated inside tests; no new provider dependency.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test -p codewiki-core --test production_fixtures`
- `rtk cargo test`
- Grep check for disallowed source-provider CLI UX.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting verified after applying `rtk cargo fmt --all`.

- Command: `rtk cargo test -p codewiki-core --test production_fixtures`
- Result: pass
- Notes: 2 tests passed across TypeScript, Python, and Rust fixture shapes.

- Command: `rtk cargo test`
- Result: pass
- Notes: 36 tests passed across 14 suites.

- Command: `rtk proxy rg -n "source add|--workspace|--output|codewiki source|codewiki init \\[source\\]|codewiki sync \\[source\\]" . --glob '!docs/work/tickets/TICKET-012-semantic-exploration-v1.md' --glob '!docs/work/tickets/TICKET-013-claim-persistence-v1.md' --glob '!docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md' --glob '!docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md'`
- Result: pass
- Notes: only ADR-0006 contains a deliberate statement rejecting `codewiki source add` UX.

## Completion Checklist

- [x] Implementation complete
- [x] Tests run and recorded
- [x] Validation matrix updated
- [x] UAT completed or explicitly not required
- [x] Master docs reconciled
- [x] Docs review completed
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
