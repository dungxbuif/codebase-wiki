---
artifact_type: ticket
id: TICKET-022
status: done
owner: human
priority: high
lane: normal
trace:
  backlog_item: BL-002, BL-010
  requirement: REQ-002, REQ-004, REQ-009
  phase: PHASE-001
  detail_design: not_required
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-022 OpenWiki And DeepWiki Docs Patterns

## Status

- ID: TICKET-022
- Status: done
- Type: research/contract/implementation
- Priority: high
- Phase: PHASE-001

## Research Findings

- OpenWiki's generated repo docs use `quickstart.md` as the required entrypoint, then focused section directories such as `architecture/overview.md`, `agent/workflow.md`, `operations/...`, and `cli/...`.
- OpenWiki explicitly avoids thin pages and low-value one-file directories; stubs should be merged into quickstart or a broader page, while deferred areas go into a `## Backlog` section.
- OpenWiki update discipline is surgical: read existing docs first, build a docs impact plan, edit only affected pages, and avoid formatting-only churn.
- DeepWiki first generates a wiki structure with pages, importance, relevant source files, related pages, and optional parent sections.
- DeepWiki page content starts with a `<details>` block listing relevant source files, then uses focused H2/H3 sections, diagrams/tables where useful, and source citations/anchors.

## Implementation Notes

- CodeWiki now uses `docs/quickstart.md` as the generated entrypoint instead of `docs/index.md`.
- Flat canonical pages were replaced with OpenWiki-style section paths:
  - `docs/source-map.md`
  - `docs/architecture/overview.md`
  - `docs/architecture/decisions.md`
  - `docs/domain/overview.md`
  - `docs/workflows/overview.md`
  - `docs/data-models/overview.md`
  - `docs/api/overview.md`
  - `docs/operations/runbook.md`
  - `docs/testing/strategy.md`
  - `docs/areas/<area>/overview.md`
- Semantic pages now prepend DeepWiki-style `<details><summary>Relevant source files</summary>...</details>` blocks when source evidence exists.
- Docs-structure skill guidance now requires quickstart-first navigation, section directories only for real areas, backlog instead of stubs, and relevant source-file blocks.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass

- Command: `rtk cargo test`
- Result: pass
- Notes: 36 tests passed across 14 suites.

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
