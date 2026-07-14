---
artifact_type: detail_design
id: DESIGN-035
status: verified
owner: ai
approval: approved
human_fields: [approval, constraints, scope_decisions]
ai_fields: [problem, context_loaded, brownfield_scope, proposed_approach, design_tradeoffs, api_data_model, security, test_plan, reconciliation_plan]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-019
  requirement: REQ-016
  phase: PHASE-002
  ticket_or_bug: docs/work/tickets/TICKET-035-sqlite-retrieval-surface-v2.md
  test_verification: docs/work/verifications/TEST-035-sqlite-retrieval-surface-v2.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-035-sqlite-retrieval-surface-v2.md
  adrs: [ADR-0001, ADR-0004]
  master_docs_touched: [docs/requirements/SPEC.md, docs/architecture/ARCHITECTURE.md, docs/architecture/API.md]
---

# Detail Design: SQLite Retrieval Surface V2

## Problem And Boundary

The storage crate already implements active/stale claim retrieval, but no command exposes it to the installed skill. The helper therefore relies on model-dependent direct SQLite access. Retrieval searches claims only, despite the same database containing files, symbols, and evidence. Staleness compares only files present in the latest snapshot, and legacy top-level `Area` claims leak an internal traversal abstraction into durable memory.

This slice does not add embeddings, FTS migrations, an Octocode adapter, remote/shared memory, or language-specific indexing.

## Context Loaded

- `docs/CONTEXT.md`, `docs/work/BACKLOG.md`, active PHASE-002 and TICKET-031
- `docs/standards/{README,QUALITY_BAR,VALIDATION,DEBUGGING,GIT,TESTING,DOCS,CODEWIKI}.md`
- TICKET/DESIGN-014, ADR-0001, ADR-0004
- `crates/codewiki-{core,store,explore,provider}` and `skill/codewiki/references/qa.md`

## Brownfield Scope

- Touched modules: CLI parsing/execution, SQLite read queries and persistence invalidation, deterministic claim promotion, Q&A instructions.
- Direct dependencies inspected: repository identity/state path resolution, exploration truncation flag, production fixtures, package/interface manifests.
- Contracts affected: companion CLI, Q&A answer order, active/stale semantics.
- Known unknown: semantic ranking quality remains provider-dependent and is deliberately out of scope.

## Proposed Approach

1. Add typed read filters in `codewiki-store` and extend the existing Q&A packet with matching files, symbols, and evidence while preserving active/stale claim sections.
2. Add a separate claim inventory renderer supporting `active`, `stale`, and `all`, plus optional exact source-path filtering.
3. Expose both through explicit, unambiguous flags:
   - `codewiki query --text <query> [--repo <path>] [--limit <1..50>]`
   - `codewiki claims [--repo <path>] [--status active|stale|all] [--path <source-path>] [--limit <1..200>]`
4. Resolve the local DB from the repository identity and fail read-only when init state is absent. Query commands never create migrations or mutate the repository/database.
5. During a complete, non-truncated snapshot, mark claims linked to evidence paths absent from the new snapshot stale and remove absent file/symbol inventory rows. Do not infer deletion from truncated exploration.
6. Let a deterministic claim regenerated from the current snapshot return to `active`; leave old claims with changed statements stale.
7. Make sync run IDs depend on snapshot file paths and hashes, preventing changed snapshots with the same file count from replacing the same run row.
8. Stop promoting aggregate top-level `Area` statements. Keep area detection only as internal exploration metadata until it is renamed or retired separately.
9. Update the skill Q&A contract to invoke `codewiki query` before direct SQLite/source fallback; use `codewiki claims --status stale` for freshness checks.

## Design Tradeoffs

- Direct `sqlite3` instructions: rejected because they leak schema and vary by model.
- Vector search now: rejected because deterministic lexical retrieval should be exposed and measured first.
- FTS5 migration now: rejected to avoid a data migration before the public query contract proves insufficient.
- Treat every missing file as deleted: rejected when exploration is truncated because it creates false staleness.
- Preserve sticky stale status for regenerated deterministic claims: rejected because fresh source-derived evidence has reverified the exact same deterministic statement.

## Blocked Identity Decision

The fifth fix/test cycle proved that canonicalizing only CLI paths is inconsistent with direct init/workspace calls. On macOS, an init identity under `/var/folders/...` and a query identity canonicalized to `/private/var/folders/...` produce different state keys.

Recommended next design: lexically normalize existing command paths by removing `.` and resolving safe `..` components without resolving filesystem symlinks. This fixes `--repo .` while preserving existing path-derived state keys. The alternative is canonical identity everywhere plus fallback lookup/migration for existing state; that is broader and risks orphaning pre-0.3 state.

The user approved the recommended lexical-normalization design on 2026-07-14 and authorized completion, commit, and push. The resumed audit starts with a fresh loop-guard counter.

## API And Data Model

- Public companion interface increments because two commands and their error contracts are added.
- Package version increments for a testable installation boundary.
- No SQLite schema migration. Existing tables and indexes are sufficient for bounded lexical queries.
- Output remains Markdown by default so the skill can consume it directly; machine-readable JSON is deferred.

## Security And Privacy

- Commands read only the repository-scoped local SQLite path.
- No network, credentials, or provider activation is added.
- Queries return stored summaries, paths, names, and claims; they do not dump source contents.

## Test Plan

- Add red tests for missing CLI commands, broader Q&A sections, claim filters, deleted-file staleness, truncated-snapshot safety, and removal of area claims.
- Implement until scoped tests pass.
- Run full workspace tests, formatting, and strict Clippy for touched crates.
- Record package contract/version assertions.

## Reconciliation Plan

- Requirements: clarify deterministic local retrieval and shared-memory boundary.
- Architecture: correct the false claim that `CLAIMS.md` is a SQLite export; document snapshot fan-out and CLI retrieval.
- API: add commands, flags, outputs, and errors.
- ERD: no schema change; describe deleted-evidence lifecycle only if needed.
- ADR: no new ADR because ADR-0001/0004 already select local SQLite plus lazy optional providers.
- Context/backlog/validation/changelog: update after proof.
