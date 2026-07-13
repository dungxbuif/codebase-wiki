---
artifact_type: ticket
id: TICKET-003
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-003
  requirement: REQ-003
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-003-config-storage-skeleton.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-003 Config And Storage Skeleton

## Status

- ID: TICKET-003
- Status: done
- Type: task
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] Store layout names `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, and `.agents/skills/codewiki/project/AGENTS.md`.
- [x] Config skeleton records docs root, plan path, agents path, and lazy runtime tool policy.
- [x] Plan skeleton records schema version and evidence policy.
- [x] Target `.agents/skills/codewiki/project/AGENTS.md` guidance is docs-first and includes provider trigger rules.
- [x] Rust tests verify the config/storage defaults.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting check passed.

- Command: `rtk cargo test`
- Result: pass
- Notes: 8 tests passed across 11 suites.

- Command: `rtk cargo run -p codewiki-cli -- status`
- Result: pass
- Notes: status output includes `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, `.agents/skills/codewiki/project/AGENTS.md`, local SQLite state summary, and docs root.

## Docs Review

- Requirements updated or not needed reason: `REQ-003` now has storage skeleton progress.
- Architecture updated or not needed reason: storage component already points to `crates/codewiki-store`.
- API updated or not needed reason: no public command change.
- ERD/data updated or not needed reason: SQLite schema not implemented yet.
- ADR created or not needed reason: existing ADR-0001 and ADR-0004 cover this slice.
- `docs/CONTEXT.md` updated: yes.

## Completion Checklist

- [x] Implementation complete
- [x] Tests run and recorded
- [x] Fix/test loop guard respected
- [x] Validation matrix updated or explicitly not affected
- [x] UAT completed or explicitly not required
- [x] Master docs reconciled
- [x] Docs review completed
- [x] ADR created or explicitly not needed
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
