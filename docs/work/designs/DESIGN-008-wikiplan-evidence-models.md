---
artifact_type: detail_design
id: DESIGN-008
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-004
  requirement: REQ-004
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-008-wikiplan-evidence-models.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
---

# DETAIL DESIGN: WikiPlan And Evidence Models

## Status

- ID: DESIGN-008
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-008-wikiplan-evidence-models.md`
- Approval: approved by autonomous continuation
- Updated: 2026-07-13

## Scope

Add typed Rust models in `codewiki-store` for semantic planning and evidence-backed docs:

- confidence labels;
- evidence kinds;
- durable claims;
- planned pages;
- WikiPlan v1 rendering.

This slice does not persist model rows through SQLite yet; the schema already exists and runtime writes will follow.

## Verification

- Unit tests for rendering deterministic plan YAML.
- Unit tests for evidence/claim defaults.

## Verification Results

- Command: `rtk cargo test -p codewiki-store -p codewiki-core -p codewiki-detect -p codewiki-docs`
- Result: pass
- Notes: 23 tests passed across 8 suites.
