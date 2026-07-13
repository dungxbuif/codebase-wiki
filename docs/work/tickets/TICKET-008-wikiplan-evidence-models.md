---
artifact_type: ticket
id: TICKET-008
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-004
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-008-wikiplan-evidence-models.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-008 WikiPlan And Evidence Models

## Status

- ID: TICKET-008
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] Rust models define confidence, evidence kinds, claims, planned pages, and WikiPlan v1.
- [x] Plan rendering includes detected stack, canonical pages, evidence policy, confidence, and open questions.
- [x] Generated plan is deterministic and suitable for committed `.agents/skills/codewiki/project/plan.yml`.
- [x] Tests cover plan rendering and evidence/claim model defaults.

## Verification Results

- Command: `rtk cargo test -p codewiki-store -p codewiki-core -p codewiki-detect -p codewiki-docs`
- Result: pass
- Notes: 23 tests passed across 8 suites.
