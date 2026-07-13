---
artifact_type: ticket
id: TICKET-023
status: done
owner: human
priority: medium
lane: normal
trace:
  backlog_item: BL-002, BL-013
  requirement: REQ-006, REQ-011
  phase: PHASE-001
  detail_design: not_required
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-023 Reference Baseline And Source Provider Status

## Status

- ID: TICKET-023
- Status: done
- Type: research/reconciliation
- Priority: medium
- Phase: PHASE-001

## Acceptance Criteria

- [x] Current OpenWiki and deepwiki-open reference commits are recorded for future update comparison.
- [x] CodeWiki source-provider/change-source implementation status is documented.
- [x] Docs clarify that CodeWiki adopts OpenWiki's deterministic-fetch-then-synthesis pattern but does not bundle OpenWiki-style connectors.

## Recorded Reference Baselines

- OpenWiki: `2fb44a876db8cca461ad1c0767931d95495763a3` (`references/openwiki`, `0.1.1-9-g2fb44a8`)
- deepwiki-open: `16f35a0fc0284e99b7963bbf4e8585e9957e2fe1` (`references/deepwiki-open`, `heads/main`)

## Implementation Status

- Implemented: `.codewiki/sources.yml` registry with primary Git source.
- Implemented: external/personal workspace mode, so source repo and wiki workspace can differ.
- Implemented: source extension reference and source-skill template.
- Implemented: sync/init guidance that treats Git as default code truth and non-Git source skills as bounded evidence producers.
- Not implemented by design: built-in Jira/Figma/fix-note/chat/issue-tracker providers in CodeWiki core.

## Verification Results

- Command: `rtk git submodule status --recursive`
- Result: pass
- Notes: recorded current OpenWiki and deepwiki-open commits above.

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
