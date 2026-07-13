---
artifact_type: detail_design
id: DESIGN-015
status: ready
owner: ai
trace:
  ticket: docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md
  phase: PHASE-001
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  master_docs: [docs/requirements/SPEC.md, docs/architecture/ARCHITECTURE.md]
---

# Design: Production Fixtures Eval Suite

## Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/standards/README.md`
- `docs/standards/QUALITY_BAR.md`
- `docs/standards/VALIDATION.md`
- `crates/codewiki-core/src/lib.rs`
- `crates/codewiki-store/src/lib.rs`

## Proposed Approach

Add a Rust integration test suite under `crates/codewiki-core/tests/production_fixtures.rs`. The tests generate temporary fixture repositories at runtime rather than committing bulky fixture trees.

Fixture shapes:

- TypeScript app with React/package metadata.
- Python service with FastAPI-style source and tests.
- Rust workspace-like repository.

The suite verifies the full deterministic path:

1. `run_with_context(["init"], context)`.
2. Generated `.agents/skills/codewiki/project/**` and `docs/**`.
3. Semantic map and claims page content.
4. SQLite Q&A context retrieval.
5. Source mutation followed by `sync` marks stale claims.

## Risks

- Tests may depend on `sqlite3`. Existing store/core tests already use the local executable fallback.
- Runtime-generated fixtures can become too synthetic. Mitigation: include package/config/test/docs signals for each shape.

## Reconciliation

- Validation matrix and changelog record production fixture coverage.
- No ADR required; this validates existing architecture.
