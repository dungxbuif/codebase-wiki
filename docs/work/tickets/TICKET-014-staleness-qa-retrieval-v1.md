---
artifact_type: ticket
id: TICKET-014
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-003, REQ-004
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-014-staleness-qa-retrieval-v1.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-014 Staleness And Q&A Retrieval V1

## Status

- ID: TICKET-014
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Problem

CodeWiki persists claims/evidence, but sync does not yet mark claims stale when evidence changes, and Q&A does not yet have a deterministic context packet from SQLite. This blocks reliable docs-first answers across sessions and model changes.

## Acceptance Criteria

- [x] Semantic exploration records stable content hashes for inspected files.
- [x] Persistence marks claims stale when supporting file evidence content changes.
- [x] Persistence returns stale claim counts.
- [x] Store exposes a SQLite-backed Q&A context renderer for active/stale claims and evidence.
- [x] Skill Q&A reference explains docs-first retrieval and SQLite context usage.
- [x] Tests prove stale claims and Q&A context behavior.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-core`
- `rtk cargo test`
- Grep check for disallowed source-provider CLI UX.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting verified.

- Command: `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-core`
- Result: pass
- Notes: 27 tests passed across 6 suites.

- Command: `rtk cargo test`
- Result: pass
- Notes: 34 tests passed across 13 suites.

- Command: `rtk proxy rg -n "source add|--workspace|--output|codewiki source|codewiki init \\[source\\]|codewiki sync \\[source\\]" . --glob '!docs/work/tickets/TICKET-012-semantic-exploration-v1.md' --glob '!docs/work/tickets/TICKET-013-claim-persistence-v1.md' --glob '!docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md'`
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
