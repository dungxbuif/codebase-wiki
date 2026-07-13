---
artifact_type: detail_design
id: DESIGN-007
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-004
  requirement: REQ-002
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-007-repo-detection-v1.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md]
---

# DETAIL DESIGN: Repository Detection V1

## Status

- ID: DESIGN-007
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-007-repo-detection-v1.md`
- Approval: approved by autonomous continuation
- Updated: 2026-07-13

## Scope

Implement deterministic repository signal detection using targeted filesystem traversal:

- languages from extensions;
- package/build/test tools from well-known config files;
- framework/library hints from manifest/config file content;
- entrypoints/docs/tests from path patterns.

This is detection, not language-specific semantic exploration. It should stay lightweight and bounded.

## Verification

- Unit tests create fixture directories for Rust, TypeScript/Next, and Python.
- `codewiki init` smoke behavior remains covered by core tests.

## Verification Results

- Command: `rtk cargo test -p codewiki-detect -p codewiki-core -p codewiki-docs -p codewiki-store`
- Result: pass
- Notes: 21 tests passed across 8 suites.

- Command: `rtk proxy env CODEWIKI_APP_DATA_DIR=/tmp/codewiki-detect-state CODEWIKI_CACHE_DIR=/tmp/codewiki-detect-cache cargo run -p codewiki-cli -- init /tmp/codewiki-detect-smoke`
- Result: pass
- Notes: detection signals were written to plan and index.
