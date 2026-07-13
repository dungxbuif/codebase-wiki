---
artifact_type: detail_design
id: DESIGN-010
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-001
  requirement: REQ-004
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-010-sync-skeleton.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
---

# DETAIL DESIGN: Sync Skeleton

## Status

- ID: DESIGN-010
- Status: done

## Scope

Add deterministic `codewiki sync [path]` for the generated files currently owned by the companion skeleton. This is not full claim-level surgical sync yet, but it establishes compare/update/no-op behavior.

## Verification

- Core tests initialize a temp repo, sync no-op, mutate a generated page, and sync updates it.

## Verification Results

- Command: `rtk cargo test -p codewiki-core -p codewiki-docs -p codewiki-store -p codewiki-detect`
- Result: pass
- Notes: 26 tests passed across 8 suites.
