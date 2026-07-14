---
artifact_type: test_verification
id: TEST-035
status: verified
owner: ai
trace:
  backlog_item: BL-019
  requirement: REQ-016
  phase: PHASE-002
  ticket: docs/work/tickets/TICKET-035-sqlite-retrieval-surface-v2.md
  design: docs/work/designs/DESIGN-035-sqlite-retrieval-surface-v2.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-035-sqlite-retrieval-surface-v2.md
---

# Test Verification: SQLite Retrieval Surface V2

## Automated Proof

| Command | Result | Evidence |
| --- | --- | --- |
| `rtk cargo test -p codewiki-explore -p codewiki-store -p codewiki-core --no-fail-fast` | pass | Red run failed on missing query/inventory APIs; final scoped run passed 49 tests across 8 suites. |
| `rtk cargo fmt --all --check` | pass | Workspace formatting is clean. |
| `rtk cargo test --workspace --no-fail-fast` | pass | 58 tests passed across 15 suites. |
| `rtk cargo clippy -p codewiki-core -p codewiki-store -p codewiki-explore --all-targets -- -D warnings` | pass | No warnings or errors. |
| `rtk cargo run -p codewiki-cli -- help` | pass | Help exposes the exact `query` and `claims` contracts. |

## Regression Coverage

- Public CLI reads initialized repository-scoped state and returns matching symbols/claims.
- Missing text, invalid limit, invalid status, and missing state fail explicitly; missing-state reads do not create directories.
- Q&A packets include active claims, stale claims, files, symbols, and evidence sections.
- Claim inventory filters by status and exact evidence path.
- Changed statements leave old claims stale; identical deterministic statements can reactivate from fresh evidence.
- Complete snapshots invalidate deleted-source claims and remove current file/symbol inventory.
- Truncated snapshots do not infer deletion.
- Refreshed files replace their symbol inventory instead of retaining removed symbols.
- Durable promoted claims no longer contain top-level `Area ...` statements.

## Installed Package Smoke

Installed package `0.3.0` into `/private/tmp/codewiki-retrieval-v030-final` using the repository installer. Doctor verified companion interface 3 and skill/reference contracts 4. Installed default init persisted two file-level claims and stopped at `synthesis_incomplete`.

The installed commands then verified:

- `query --text authenticateSession --repo .` returned the `authenticateSession` symbol at `src/index.js:1`.
- `claims --repo . --status active --path src/index.js` returned the active file claim and evidence path.
- No top-level area claim appeared.

## Loop Guard

- Same-path failure attempts: 2 / 3.
- Previous audit: stopped at 5 / 5 after canonicalization changed `/var/...` to `/private/var/...` while init had persisted the original path-derived key.
- Resumed audit after human approval: 1 / 3 same-path failures and 2 / 5 fix/test cycles; strict Clippy failed once, then scoped tests and Clippy passed.
- Architecture/security expansion: none beyond the approved design.

## Resolved Identity Failure

Command: `rtk cargo test -p codewiki-core public_query_and_claim_commands_read_initialized_sqlite_state`

Historical result: query could not find state after `--repo .` canonicalized the repository to a different macOS alias. The approved fix now performs lexical normalization only: `.` and safe `..` components collapse while symlink aliases remain distinct. Focused regression, full workspace, and fresh installed-package smoke all pass.
