---
artifact_type: ticket
id: TICKET-013
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-003, REQ-004
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-013-claim-persistence-v1.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-013 Claim Persistence V1

## Status

- ID: TICKET-013
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Problem

Semantic exploration v1 produces useful source evidence, but the facts are only rendered into generated docs. CodeWiki needs durable claim/evidence persistence in SQLite so future sessions, model changes, sync runs, and Q&A can reuse prior evidence instead of relying on chat context or regenerated markdown alone.

## Acceptance Criteria

- [x] Promote deterministic claims from semantic exploration snapshots.
- [x] Persist repository, run, files, symbols, evidence items, claims, and claim/evidence links into the existing SQLite schema.
- [x] Init and sync write durable state without changing the skill-first UX.
- [x] Generated `docs/evidence/claims.md` lists promoted claims with evidence IDs.
- [x] Tests prove claims and evidence persist into SQLite and docs remain enriched.

## Scope

In scope:

- Deterministic v1 claim promotion from file/area evidence.
- SQLite persistence through the existing `sqlite3` executor boundary.
- Init/sync integration.
- Docs rendering for promoted claims.

Out of scope:

- LLM-authored claim synthesis.
- Claim invalidation/staleness algorithms.
- Provider-driven graph claims.
- New CLI UX.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-docs -p codewiki-core`
- `rtk cargo test`
- Grep check for disallowed source-provider CLI UX.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting verified.

- Command: `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-docs -p codewiki-core`
- Result: pass
- Notes: 29 tests passed across 8 suites.

- Command: `rtk cargo test`
- Result: pass
- Notes: 33 tests passed across 13 suites.

- Command: `rtk proxy rg -n "source add|--workspace|--output|codewiki source|codewiki init \\[source\\]|codewiki sync \\[source\\]" . --glob '!docs/work/tickets/TICKET-012-semantic-exploration-v1.md' --glob '!docs/work/tickets/TICKET-013-claim-persistence-v1.md'`
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
