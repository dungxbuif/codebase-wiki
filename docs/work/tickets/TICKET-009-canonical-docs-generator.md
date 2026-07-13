---
artifact_type: ticket
id: TICKET-009
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-004
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-009-canonical-docs-generator.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-009 Canonical Docs Generator

## Status

- ID: TICKET-009
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] `codewiki init` writes canonical starter pages beyond `index.md`.
- [x] Generated pages include repository map, architecture, and evidence pages.
- [x] Pages include detection/evidence placeholders and avoid pretending full semantic analysis is done.
- [x] Tests prove required canonical pages are created.

## Verification Results

- Command: `rtk cargo test -p codewiki-docs -p codewiki-core -p codewiki-store -p codewiki-detect`
- Result: pass
- Notes: 24 tests passed across 8 suites.

- Command: `rtk proxy env CODEWIKI_APP_DATA_DIR=/tmp/codewiki-docs-state CODEWIKI_CACHE_DIR=/tmp/codewiki-docs-cache cargo run -p codewiki-cli -- init /tmp/codewiki-docs-smoke`
- Result: pass
- Notes: created canonical docs pages under `docs/**`.
