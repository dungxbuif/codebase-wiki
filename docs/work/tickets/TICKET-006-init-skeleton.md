---
artifact_type: ticket
id: TICKET-006
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-001
  requirement: REQ-001
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-006-init-skeleton.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-006 Init Skeleton

## Status

- ID: TICKET-006
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] `codewiki init [path]` creates `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, `.agents/skills/codewiki/project/AGENTS.md`, and `docs/quickstart.md`.
- [x] Init applies local SQLite migrations for the target repository.
- [x] Init is idempotent and does not overwrite existing files.
- [x] Tests prove file creation and no-overwrite behavior.
- [x] CLI help documents `init`.

## Test Expectations

- Unit: `rtk cargo test -p codewiki-core -p codewiki-docs -p codewiki-store`
- Integration: init writes fixture temp repo and initializes SQLite
- E2E: not required
- UAT: not required for skeleton
- Docs review: required

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting check passed.

- Command: `rtk cargo test -p codewiki-core -p codewiki-docs -p codewiki-store`
- Result: pass
- Notes: 18 tests passed across 6 suites.

- Command: `rtk proxy env CODEWIKI_APP_DATA_DIR=/tmp/codewiki-state-smoke CODEWIKI_CACHE_DIR=/tmp/codewiki-cache-smoke cargo run -p codewiki-cli -- init /tmp/codewiki-init-smoke`
- Result: pass
- Notes: created `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, `.agents/skills/codewiki/project/AGENTS.md`, `docs/quickstart.md`, and local SQLite state DB. Env vars were used because sandbox blocks default HOME app-data writes.

## Completion Checklist

- [x] Implementation complete
- [x] Tests run and recorded
- [x] Validation matrix updated
- [x] UAT completed or explicitly not required
- [x] Master docs reconciled
- [x] Docs review completed
- [x] ADR created or explicitly not needed
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
