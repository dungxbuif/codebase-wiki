---
artifact_type: detail_design
id: DESIGN-014
status: ready
owner: ai
trace:
  ticket: docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md
  phase: PHASE-001
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  master_docs: [docs/requirements/SPEC.md, docs/architecture/ARCHITECTURE.md]
---

# Design: Staleness And Q&A Retrieval V1

## Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/standards/README.md`
- `docs/standards/QUALITY_BAR.md`
- `docs/standards/VALIDATION.md`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`
- `skill/codewiki/references/qa.md`
- `crates/codewiki-explore/src/lib.rs`
- `crates/codewiki-store/src/lib.rs`

## Proposed Approach

1. Add `content_hash` to `ExploredFile`, computed from actual file text.
2. Use that hash in SQLite `files.content_hash` and `evidence_items.content_hash`.
3. Before replacing changed evidence, mark linked active claims stale when the source path exists but content hash changed.
4. Add `render_qa_context_with_sqlite(sqlite, db, query, limit)`:
   - returns active claims first;
   - includes stale claims separately;
   - includes evidence IDs/source paths;
   - uses a simple escaped LIKE query for deterministic local context retrieval.
5. Update skill Q&A guidance to use SQLite context after docs/plan and before source fallback.

## Risks

- Hashing full text can be expensive on huge files. Existing exploration already bounds file size to 512 KiB.
- Staleness is conservative: changed evidence marks related claims stale even if the claim remains true. That is safer than false freshness.
- LIKE retrieval is not semantic search. It is a deterministic floor; optional providers can still be activated when quality requires it.

## Verification

- Unit/integration tests mutate an explored file and verify old claims become stale.
- Q&A context test verifies active and stale claim sections render from SQLite.
- Full workspace tests.
