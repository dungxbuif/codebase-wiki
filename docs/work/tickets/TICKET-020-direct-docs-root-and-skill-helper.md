---
artifact_type: ticket
id: TICKET-020
status: done
owner: human
priority: high
lane: normal
trace:
  backlog_item: BL-010, BL-008
  requirement: REQ-007, REQ-009
  phase: PHASE-001
  detail_design: not_required
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-020 Direct Docs Root And Skill Helper

## Status

- ID: TICKET-020
- Status: done
- Type: contract/reconciliation
- Priority: high
- Phase: PHASE-001

## Acceptance Criteria

- [x] Generated CodeWiki pages default directly under `docs/`, not `docs/codewiki/`.
- [x] Existing unmarked human-authored `docs/**` files remain evidence and are not skipped wholesale.
- [x] Installed skill includes a helper script for invoking the Rust companion.
- [x] Installer copies companion source into the installed skill when available so the helper can run through Cargo if no binary is installed.
- [x] Docs, skill references, tests, and validation records reflect the direct docs-root contract.

## Implementation Notes

- Canonical generated docs now use paths such as `docs/index.md`, `docs/map.md`, `docs/evidence/claims.md`, and `docs/areas/<area>.md`.
- Detection/exploration skip only `.codewiki/**` and known generated CodeWiki pages/directories, not all of `docs/**`.
- `skill/codewiki/scripts/codewiki-helper.sh` resolves the Rust companion through `CODEWIKI_COMPANION_BIN`, `PATH`, `CODEWIKI_REPO`, installed `companion/` source, or a source checkout.
- `scripts/install-codewiki-skill.sh` copies `skill/codewiki` and the Rust companion source into the installed skill package.

## Verification Results

- Command: `rtk bash -n scripts/install-codewiki-skill.sh`
- Result: pass
- Notes: installer syntax is valid.

- Command: `rtk bash -n skill/codewiki/scripts/codewiki-helper.sh`
- Result: pass
- Notes: helper script syntax is valid.

- Command: `rtk env CODEWIKI_REPO=/Users/dungxbuif/Documents/codebase-wiki bash skill/codewiki/scripts/codewiki-helper.sh status`
- Result: pass
- Notes: helper resolves the Rust companion from `CODEWIKI_REPO` and reports `docs root: docs`.

- Command: `rtk env CODEWIKI_REPO_URL=/Users/dungxbuif/Documents/codebase-wiki CODEX_HOME=/private/tmp/codewiki-install-test-postcommit bash scripts/install-codewiki-skill.sh`
- Result: pass
- Notes: local clone install copied `scripts/codewiki-helper.sh` and `companion/` source into the installed skill.

- Command: `rtk bash /private/tmp/codewiki-install-test-postcommit/skills/codewiki/scripts/codewiki-helper.sh status`
- Result: pass
- Notes: installed helper compiled and ran the bundled companion source, reporting `docs root: docs`.

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
