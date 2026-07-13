---
artifact_type: ticket
id: TICKET-017
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-004
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-017-synthesis-pages-v1.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-017 Synthesis Pages V1

## Status

- ID: TICKET-017
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Problem

CodeWiki has semantic evidence and durable claims, but only the map, architecture, and evidence pages are meaningfully populated. The canonical docs structure needs generated domain, workflow, data, interface, operations, testing, decisions, glossary, open-questions, and area pages so the wiki feels complete after init.

## Acceptance Criteria

- [x] Generate canonical synthesis pages from semantic evidence.
- [x] Generate area pages for substantial top-level areas.
- [x] Keep every synthesis page evidence-bound and explicitly mark unresolved sections.
- [x] Keep sync generated-region safe.
- [x] Production fixture tests verify synthesis pages exist and contain evidence-backed content.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test -p codewiki-docs -p codewiki-core`
- `rtk cargo test`
- Grep check for disallowed source-provider CLI UX.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting verified after applying `rtk cargo fmt --all`.

- Command: `rtk cargo test -p codewiki-docs -p codewiki-core`
- Result: pass
- Notes: 14 tests passed across 5 suites.

- Command: `rtk cargo test`
- Result: pass
- Notes: 36 tests passed across 14 suites.

- Command: `rtk proxy rg -n "source add|--workspace|--output|codewiki source|codewiki init \\[source\\]|codewiki sync \\[source\\]" . --glob '!docs/work/tickets/TICKET-012-semantic-exploration-v1.md' --glob '!docs/work/tickets/TICKET-013-claim-persistence-v1.md' --glob '!docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md' --glob '!docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md' --glob '!docs/work/tickets/TICKET-017-synthesis-pages-v1.md'`
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
