---
artifact_type: ticket
id: TICKET-011
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-013
  requirement: REQ-011
  phase: PHASE-001
  detail_design: docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-011 Workspace Placement And Source Extension Skills

## Status

- ID: TICKET-011
- Status: done
- Type: architecture/skill
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] Skill supports repo-local and external/personal wiki workspace placement.
- [x] Skill requires confirmation before writing when placement is ambiguous.
- [x] Git is documented as the default code-change source.
- [x] Non-Git sources are supported only through user-provided source extension skills.
- [x] CodeWiki core does not define built-in Jira/Figma/provider commands.
- [x] `.codewiki/sources.yml` is documented as the source registry.

## Verification Results

- Command: `rtk cargo test -p codewiki-store`
- Result: pass
- Notes: 13 tests passed across 2 suites.

- Command: `rtk proxy python3 -c '...'`
- Result: pass
- Notes: validated `SKILL.md` references `workspace-placement.md` and `source-extensions.md`, and files exist.

## Completion Checklist

- [x] Implementation complete
- [x] Tests run and recorded
- [x] Validation matrix updated
- [x] UAT completed or explicitly not required
- [x] Master docs reconciled
- [x] Docs review completed
- [x] ADR created
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated

