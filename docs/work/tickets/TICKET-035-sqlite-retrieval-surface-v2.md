---
artifact_type: ticket
id: TICKET-035
status: done
owner: human
priority: high
lane: high-risk
human_fields: [title, priority, acceptance_criteria, scope, approval]
ai_fields: [impacted_areas, test_expectations, verification_results, docs_review, context_updates]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-019
  requirement: REQ-016
  phase: PHASE-002
  detail_design: docs/work/designs/DESIGN-035-sqlite-retrieval-surface-v2.md
  test_verification: docs/work/verifications/TEST-035-sqlite-retrieval-surface-v2.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-035-sqlite-retrieval-surface-v2.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-035 SQLite Retrieval Surface V2

## Status

- ID: TICKET-035
- Status: done
- Type: feature and correctness hardening
- Priority: high
- Phase: PHASE-002
- Owner: human

## Context

CodeWiki already persists local SQLite facts/evidence/claims and has a library-level lexical Q&A renderer, but the installed skill has no deterministic companion command that an agent can invoke. The current renderer searches only promoted claim statements and evidence paths, while file/symbol records remain inaccessible through the product surface. Deleted files are not invalidated, and durable claim promotion still emits the legacy internal `Area` abstraction.

The user approved closing these gaps on 2026-07-14 after reviewing the implementation-level assessment.

## Acceptance Criteria

- [x] `codewiki query --text <query> [--repo <path>] [--limit <1..50>]` returns a concise Markdown packet from local SQLite without mutating repository state.
- [x] Query results separate active and stale claims and include matching files, symbols, and evidence.
- [x] `codewiki claims [--repo <path>] [--status active|stale|all] [--path <source-path>] [--limit <1..200>]` provides deterministic claim inspection.
- [x] Missing state, invalid flags, invalid status, and invalid limits produce explicit non-zero errors.
- [x] A complete sync marks claims backed by deleted files stale; a truncated exploration does not infer deletion.
- [x] Re-derived deterministic claims can become active again, while superseded claim statements remain stale.
- [x] Durable claim promotion no longer emits top-level `Area ...` claims.
- [x] The Q&A skill reference requires the companion retrieval command before raw SQLite or source fallback.
- [x] Scoped tests and the full workspace suite pass, and public/master docs are reconciled.
- [x] Repository identity is stable for lexical aliases such as absolute path, `.`, and `..`; filesystem symlink aliases intentionally remain distinct to preserve existing path-derived state keys.

## Small Task Exemption

- Small task exemption: no
- Reason: this changes a public CLI contract and durable retrieval behavior.
- Impact checked: API=yes, DB=behavior-only/no migration, Security=no new trust boundary, Runtime=yes, Standards=no

## Impacted Areas

- Code: `codewiki-core`, `codewiki-store`, `codewiki-explore`, packaged skill references.
- Requirements docs: clarify deterministic local retrieval surface.
- Architecture docs: correct the snapshot/SQLite/Markdown relationship and Q&A flow.
- API docs: add query and claim inspection commands and errors.
- ERD/data docs: no schema migration; update lifecycle notes only if behavior proof requires it.
- Decisions: no new ADR; implements the existing SQLite-first and optional-provider decisions.

## Detail Design

- Required: yes
- Link: `docs/work/designs/DESIGN-035-sqlite-retrieval-surface-v2.md`
- Approval: approved by the user's 2026-07-14 instruction to update the assessed gaps.

## Test Expectations

- Parser/contract tests for success and invalid CLI forms.
- Store integration tests for files/symbols/evidence retrieval and claim inventory filters.
- Red/green deletion-staleness and same-statement re-derivation tests.
- Production fixture query through the public companion command.
- `rtk cargo fmt --all --check`, scoped tests, full workspace tests, and scoped strict Clippy.

## Verification Results

- Scoped red/green: pass, final 49 tests across 8 suites.
- Full workspace after identity normalization: pass, 58 tests across 15 suites.
- Formatting and scoped strict Clippy: pass.
- Fresh installed package `0.3.0` default-init plus `query --repo .`/`claims --repo .` smoke: pass.
- Evidence: `docs/work/verifications/TEST-035-sqlite-retrieval-surface-v2.md`.

## Fix/Test Attempt Log

- Resumed blocked audit: 1 / 3 same-path failures, 2 / 5 fix/test cycles; the only failure was strict-Clippy formatting and the second cycle was green.
- Previous audit: stopped at 5 / 5 after macOS path aliasing was reproduced.
- Blocked by loop guard: no; the user approved lexical normalization on 2026-07-14.
- Human/design input needed: none.

## UAT

- Required: no
- Reason: read-only companion CLI behavior is covered by contract and integration tests; the user will run a later package-level docs test.
- Expected behavior: installed agents can retrieve local memory deterministically without direct SQL.
- Verified behavior: installed binary resolves default init state through `--repo .`, returns the expected symbol, and returns the active path-filtered claim without direct SQL.
- Sign-off: automated and installed-package proof complete; user will perform downstream Mezon testing after push.

## Docs Review

- Requirements updated: REQ-016 added.
- Architecture/API updated: snapshot fan-out, retrieval commands, staleness/deletion semantics, and errors documented.
- ERD/data updated: no migration; lifecycle constraints clarified.
- ADR: not required; ADR-0001/0004 already decide the boundary.
- Context/backlog/traceability/validation/changelog updated.
- Evidence: `docs/work/reviews/DOCS-REVIEW-035-sqlite-retrieval-surface-v2.md`.

## Completion Checklist

- [x] Implementation complete
- [x] Tests run and recorded
- [x] Fix/test loop guard respected in the previous and resumed audits
- [x] Validation matrix updated
- [x] UAT completed or explicitly not required
- [x] Master docs reconciled after the identity decision
- [x] Docs review completed after the identity decision
- [x] ADR explicitly not needed
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
