---
artifact_type: ticket
id: TICKET-002
status: done
owner: human
priority: high
lane: high-risk
human_fields:
  - title
  - priority
  - acceptance_criteria
  - scope
  - approval
ai_fields:
  - impacted_areas
  - test_expectations
  - verification_results
  - docs_review
  - context_updates
shared_fields:
  - status
  - trace
  - small_task_exemption
trace:
  backlog_item: BL-008
  requirement: REQ-007
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-002-skill-first-installer.md
  test_verification: docs/work/tickets/TICKET-002-skill-first-installer.md#verification-results
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/tickets/TICKET-002-skill-first-installer.md#docs-review
  adrs: [docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-002 Skill-First Installer

## Status

- ID: TICKET-002
- Status: done
- Type: task
- Priority: high
- Phase: PHASE-001
- Owner: human

## Context

Human fill:

- User/business/system problem: CodeWiki should be installed and used as a Codex skill across projects.
- Source prompt or requirement: "tôi sẽ ko cần CLI, rust chỉ là công cụ đi kèm với skill. Mục đích cuối cùng dự án này là skill ... cần có một lệnh cài thẳng qua repo để cài skill vào các dự án"
- Out of scope: complete CodeWiki init/sync implementation.
- Approval: approved by direct instruction.

AI fill:

- Current repository context read: project context, backlog, standards, validation, phase, ADR-0002, skill-creator guidance.
- Brownfield touched scope: skill package, installer script, remote, and docs reconciliation.

## Acceptance Criteria

- [x] Given this repo, when an agent inspects it, then CodeWiki is documented as skill-first.
- [x] Given the install script, when syntax is checked, then it is valid Bash.
- [x] Given git remotes are listed, then `origin` points to `git@github.com:dungxbuif/harness.git`.
- [x] Given future users, when they read README/API docs, then they can see the one-command install path.
- [x] UAT requirement is clear: not_required because this is installation scaffolding and docs.

## Small Task Exemption

- Small task exemption: no
- Reason: Changes product architecture and install contract.
- Impact checked: API=yes, DB=no, Security=no, Runtime=yes, Standards=no

## Impacted Areas

- Code: `skill/codewiki/**`, `scripts/install-codewiki-skill.sh`
- Requirements docs: `docs/requirements/REQUIREMENTS.md`, `docs/requirements/SPEC.md`
- Architecture docs: `docs/architecture/ARCHITECTURE.md`
- API docs: `docs/architecture/API.md`
- ERD/data docs: no change
- Decisions: `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`

## Detail Design

- Required: yes
- Link: `docs/work/designs/DESIGN-002-skill-first-installer.md`
- Approval: approved

## Test Expectations

- Unit/static: `rtk bash -n scripts/install-codewiki-skill.sh`
- Integration: not required because cloning/copying into `$CODEX_HOME` is an external install action.
- E2E: not required yet.
- UAT: not required.
- Manual/platform: `rtk git remote -v`
- Docs review: trace and product direction updated.

## Verification Results

- Command: `rtk bash -n scripts/install-codewiki-skill.sh`
- Result: pass
- Notes: installer script syntax is valid.

- Command: `rtk git remote -v`
- Result: pass
- Notes: `origin` fetch/push points to `git@github.com:dungxbuif/harness.git`.

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: Rust companion workspace formatting remains clean.

- Command: `rtk cargo test`
- Result: pass
- Notes: 4 tests passed across 11 suites.

## UAT

- Required: no
- Reason if not required: installation scaffold and docs only.
- Expected behavior: install script can be used after repo is pushed.
- Verified behavior: syntax and remote verified locally.
- Sign-off: not_required

## Docs Review

- Requirements updated or not needed reason: updated with skill-first and install requirement.
- Architecture updated or not needed reason: updated to make skill package primary and Rust companion.
- API updated or not needed reason: updated with installer command.
- ERD/data updated or not needed reason: no data model change.
- ADR created or not needed reason: ADR-0003 created; ADR-0002 partially superseded.
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
