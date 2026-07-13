---
artifact_type: detail_design
id: DESIGN-009
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-004
  requirement: REQ-004
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-009-canonical-docs-generator.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
---

# DETAIL DESIGN: Canonical Docs Generator

## Status

- ID: DESIGN-009
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-009-canonical-docs-generator.md`
- Approval: approved by autonomous continuation
- Updated: 2026-07-13

## Scope

Generate useful canonical starter docs during init:

- `docs/quickstart.md`
- `docs/source-map.md`
- `docs/architecture/overview.md`
- `docs/domain/overview.md`
- `docs/workflows/overview.md`
- `docs/data-models/overview.md`
- `docs/api/overview.md`
- `docs/operations/runbook.md`
- `docs/testing/strategy.md`
- `docs/architecture/decisions.md`
- `docs/glossary.md`
- `docs/open-questions.md`
- `docs/evidence/README.md`
- `docs/evidence/sources.md`
- `docs/evidence/claims.md`
- `docs/evidence/commands.md`

These pages must be honest about coverage: detection is done, full semantic exploration is pending. Later semantic runs should keep OpenWiki's section-directory discipline and DeepWiki's relevant-source-files block instead of flattening every concept into top-level files.

## Verification

- Unit tests for rendered page set.
- Core init tests assert canonical pages are created.

## Verification Results

- Command: `rtk cargo test -p codewiki-docs -p codewiki-core -p codewiki-store -p codewiki-detect`
- Result: pass
- Notes: 24 tests passed across 8 suites.

- Command: `rtk proxy env CODEWIKI_APP_DATA_DIR=/tmp/codewiki-docs-state CODEWIKI_CACHE_DIR=/tmp/codewiki-docs-cache cargo run -p codewiki-cli -- init /tmp/codewiki-docs-smoke`
- Result: pass
- Notes: smoke test created index, map, architecture, and evidence pages.
