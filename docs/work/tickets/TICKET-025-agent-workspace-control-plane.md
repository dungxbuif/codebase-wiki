---
artifact_type: ticket
id: TICKET-025
status: done
owner: human
priority: high
lane: normal
trace:
  backlog_item: BL-014
  requirement: REQ-007, REQ-009, REQ-011
  phase: PHASE-001
  detail_design: not_required
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-025 Agent Workspace Control Plane

## Status

- ID: TICKET-025
- Status: done
- Type: contract/reconciliation
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] Project-local skill installation uses `.agents/skills/codewiki`.
- [x] CodeWiki project config/control files live under `.agents/skills/codewiki/project`.
- [x] `codewiki init` writes `docs/**` plus `.agents/skills/codewiki/project/{config.yml,plan.yml,AGENTS.md,sources.yml}` and does not create `.codewiki`.
- [x] Installer preserves existing `.agents/skills/codewiki/project` when updating the skill package.
- [x] Skill prompts and master docs no longer reference `.codewiki`.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass.

- Command: `rtk cargo test`
- Result: pass — 36 tests passed across 14 suites.

- Command: `rtk bash -n scripts/install-codewiki-skill.sh`
- Result: pass.

- Command: `rtk env CODEWIKI_APP_DATA_DIR=/private/tmp/codewiki-agent-control-state CODEWIKI_CACHE_DIR=/private/tmp/codewiki-agent-control-cache cargo run -p codewiki-cli -- init /private/tmp/codewiki-agent-control-smoke`
- Result: pass — created `.agents/skills/codewiki/project/**` and `docs/**`.

- Command: `rtk proxy test ! -e /private/tmp/codewiki-agent-control-smoke/.codewiki`
- Result: pass.

- Command: `rtk env CODEWIKI_TARGET_DIR=/private/tmp/codewiki-agent-control-smoke bash scripts/install-codewiki-skill.sh`
- Result: pass — installer updated the skill while preserving `.agents/skills/codewiki/project/config.yml`.

## Docs Review

- Updated runtime paths, skill instructions, references, ADRs, requirements, architecture docs, validation matrix, backlog, changelog, and context.

## Completion Checklist

- [x] Implementation complete
- [x] Tests run and recorded
- [x] Validation matrix updated
- [x] UAT completed or explicitly not required
- [x] Master docs reconciled
- [x] Docs review completed
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
