---
artifact_type: ticket
id: TICKET-007
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-002
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-007-repo-detection-v1.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-007 Repository Detection V1

## Status

- ID: TICKET-007
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] Detect languages from source file extensions.
- [x] Detect package managers/build tools from common config files.
- [x] Detect framework/library signals from config/manifests without core adapters.
- [x] Detect entrypoint/test/docs signals.
- [x] `codewiki init` includes detection summary in generated plan/index.
- [x] Tests cover Rust, JS/TS, and Python-shaped fixture directories.

## Verification Results

- Command: `rtk cargo fmt --all`
- Result: pass
- Notes: formatting applied.

- Command: `rtk cargo test -p codewiki-detect -p codewiki-core -p codewiki-docs -p codewiki-store`
- Result: pass
- Notes: 21 tests passed across 8 suites.

- Command: `rtk proxy env CODEWIKI_APP_DATA_DIR=/tmp/codewiki-detect-state CODEWIKI_CACHE_DIR=/tmp/codewiki-detect-cache cargo run -p codewiki-cli -- init /tmp/codewiki-detect-smoke`
- Result: pass
- Notes: TypeScript, Next.js, and React signals appeared in `.codewiki/plan.yml` and `docs/quickstart.md`.
