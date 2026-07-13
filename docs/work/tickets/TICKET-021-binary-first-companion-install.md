---
artifact_type: ticket
id: TICKET-021
status: done
owner: human
priority: high
lane: normal
trace:
  backlog_item: BL-008
  requirement: REQ-006, REQ-007
  phase: PHASE-001
  detail_design: not_required
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-021 Binary-First Companion Install

## Status

- ID: TICKET-021
- Status: done
- Type: install/runtime
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] Installer builds a release `codewiki` companion binary when Cargo is available.
- [x] Installed skill stores the built binary at `bin/codewiki`.
- [x] `scripts/codewiki-helper.sh` prefers the installed binary before PATH or Cargo/source fallback.
- [x] Companion source remains available as fallback when binary build is unavailable or intentionally replaced.

## Verification Results

- Command: `rtk bash -n scripts/install-codewiki-skill.sh`
- Result: pass
- Notes: installer syntax is valid.

- Command: `rtk bash -n skill/codewiki/scripts/codewiki-helper.sh`
- Result: pass
- Notes: helper syntax is valid.

- Command: `rtk env CODEWIKI_REPO_URL=/Users/dungxbuif/Documents/codebase-wiki CODEX_HOME=/private/tmp/codewiki-install-binary-test2 bash scripts/install-codewiki-skill.sh`
- Result: pass
- Notes: installer built release companion and installed `skills/codewiki/bin/codewiki`.

- Command: `rtk bash /private/tmp/codewiki-install-binary-test2/skills/codewiki/scripts/codewiki-helper.sh status`
- Result: pass
- Notes: helper used installed binary directly; output had no Cargo compile/run fallback logs and reported `docs root: docs`.

- Command: `rtk cargo test`
- Result: pass
- Notes: 36 tests passed across 14 suites.

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
