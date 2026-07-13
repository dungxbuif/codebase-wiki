---
artifact_type: detail_design
id: DESIGN-013
status: ready
owner: ai
trace:
  ticket: docs/work/tickets/TICKET-013-claim-persistence-v1.md
  phase: PHASE-001
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  master_docs: [docs/requirements/SPEC.md, docs/requirements/REQUIREMENTS.md, docs/architecture/ARCHITECTURE.md]
---

# Design: Claim Persistence V1

## Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/standards/README.md`
- `docs/standards/QUALITY_BAR.md`
- `docs/standards/VALIDATION.md`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`
- `crates/codewiki-store/migrations/001_initial_state.sql`
- `crates/codewiki-store/src/lib.rs`
- `crates/codewiki-core/src/lib.rs`

## Problem Statement

CodeWiki has a durable SQLite schema for claims and evidence, and semantic exploration v1 can discover file/symbol/import evidence. The missing layer is a deterministic persistence path that writes these facts into SQLite during init/sync and produces a generated claims page from the same evidence.

## Proposed Approach

1. Add deterministic promoted claim records to `codewiki-explore`.
2. Render promoted claims in `codewiki-docs`.
3. Add `persist_exploration_with_sqlite` in `codewiki-store`:
   - upsert repository identity;
   - insert a sync run;
   - upsert explored files;
   - insert symbols;
   - upsert file evidence;
   - upsert promoted claims;
   - link claims to evidence.
4. Call persistence during init and sync after migrations are applied.

## Existing Schema

No schema migration is required for this slice. Migration `001_initial_state.sql` already has:

- `repositories`
- `sync_runs`
- `files`
- `symbols`
- `evidence_items`
- `claims`
- `claim_evidence`

## Alternatives Considered

- Add migration v2 immediately: rejected because the required tables already exist.
- Persist markdown only: rejected because markdown alone does not satisfy reusable local runtime state.
- Wait for LLM-authored claims: rejected because deterministic evidence claims are enough for a verified v1 persistence layer.

## Risks

- Deterministic claims can be shallow. Mitigation: label them source-backed structural claims, not deep architecture conclusions.
- Repeated syncs can duplicate rows. Mitigation: stable IDs and upserts for repository/files/evidence/claims.
- Sync persistence needs runtime context. Mitigation: route sync through the existing `RuntimeContext`.

## Verification

- Store test applies migrations, persists a synthetic exploration snapshot, and queries claim/evidence counts from SQLite.
- Core tests verify init writes generated claims docs and durable SQLite claim/evidence rows.
- Full cargo test.

## Reconciliation Plan

- Requirements: update REQ-003/REQ-004 evidence.
- Architecture: document durable claim persistence under evidence store/runtime flow.
- Validation matrix: add ticket row/evidence.
- Context/backlog/changelog/phase: update after implementation.
