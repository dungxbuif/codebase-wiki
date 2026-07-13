---
artifact_type: ticket
id: TICKET-001
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
  backlog_item: BL-007
  requirement: REQ-006
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-001-rust-cli-workspace.md
  test_verification: docs/work/tickets/TICKET-001-rust-cli-workspace.md#verification-results
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/tickets/TICKET-001-rust-cli-workspace.md#docs-review
  adrs: [docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-001 Rust CLI Workspace

## Field Ownership

- Human fills intent, priority, acceptance criteria, scope, and approval.
- AI fills impact analysis, test expectations, verification evidence, docs review, and context/backlog updates.
- Shared fields include status, trace links, and small-task exemption.

## Status

- ID: TICKET-001
- Status: done
- Type: task
- Priority: high
- Phase: PHASE-001
- Owner: human

## Trace Links

- Backlog item: `BL-007`
- Requirement: `REQ-006`
- Phase: `PHASE-001`
- Detail design: `docs/work/designs/DESIGN-001-rust-cli-workspace.md`
- Test verification: this ticket
- Validation matrix: `docs/work/VALIDATION_MATRIX.md`
- Docs review: this ticket
- ADRs: `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- Release notes: `docs/releases/CHANGELOG.md`

## Context

Human fill:

- User/business/system problem: CodeWiki should learn from the two reference submodules and keep Rust available as companion tooling.
- Source prompt or requirement: "Tôi sẽ dựa trên kỹ thuật của 2 submodule. CLI tôi muốn viết lại bằng rust".
- Out of scope: full semantic init, SQLite schema, provider implementation, docs generation, sync, and Q&A.
- Approval: approved by standing instruction to continue without additional approval.

AI fill:

- Current repository context read: project context, backlog, standards, validation, active phase, detail design template, ticket template.
- Brownfield touched scope, if applicable: new Rust workspace and harness docs only.

## Acceptance Criteria

- [x] Given the repository root, when `cargo test` runs, then all Rust workspace tests pass.
- [x] Given the repository root, when `cargo run -p codewiki-cli -- status` runs, then the `codewiki` CLI responds successfully.
- [x] Given future implementation work, when an agent inspects the workspace, then CLI/core/detect/store/provider/docs boundaries are clear.
- [x] Error/failure behavior is defined for unknown commands.
- [x] UAT requirement is clear: not_required because this is runtime scaffold, not an end-user workflow.

## Small Task Exemption

- Small task exemption: no
- Reason: Creates runtime and public CLI boundaries.
- Impact checked: API=yes, DB=no, Security=no, Runtime=yes, Standards=no

## Impacted Areas

- Code: `Cargo.toml`, `crates/**`
- Requirements docs: `docs/requirements/REQUIREMENTS.md`, `docs/requirements/SPEC.md`
- Architecture docs: `docs/architecture/ARCHITECTURE.md`
- API docs: `docs/architecture/API.md`
- ERD/data docs: no persistent data model in this ticket
- Decisions: ADR-0002 already exists

## Detail Design

- Required: yes
- Link: `docs/work/designs/DESIGN-001-rust-cli-workspace.md`
- Approval: approved

## Test Expectations

- Unit: `rtk cargo test`
- Integration: not required for scaffold
- E2E: not required for scaffold
- UAT: not required because no complete user workflow yet
- Manual/platform: `rtk cargo run -p codewiki-cli -- status`
- Docs review: trace and runtime docs updated

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting check passed after running `rtk cargo fmt --all`.

- Command: `rtk cargo test`
- Result: pass
- Notes: 4 tests passed across 11 suites.

- Command: `rtk cargo run -p codewiki-cli -- status`
- Result: pass
- Notes: CLI returned scaffold status, Rust runtime, command list, planned detection, config path, local SQLite state summary, and docs root.

## Fix/Test Attempt Log

- Same-path failure attempts: 0 / 3
- Total fix/test cycles: 0 / 5
- Blocked by loop guard: no
- Human/design input needed: none

## UAT

- Required: no
- Reason if not required: runtime scaffold only; no complete end-user workflow yet.
- Expected behavior: CLI binary can be built and basic commands respond.
- Verified behavior: `codewiki status` builds and returns deterministic scaffold status.
- Sign-off: not_required

## Docs Review

- Requirements updated or not needed reason: `REQ-006` records Rust companion-tool requirement.
- Architecture updated or not needed reason: `docs/architecture/ARCHITECTURE.md` records crate boundaries and runtime flow.
- API updated or not needed reason: `docs/architecture/API.md` records initial CLI commands and unknown-command behavior.
- ERD/data updated or not needed reason: no persistent data model exists in this ticket.
- ADR created or not needed reason: ADR-0002 already records Rust CLI strategy.
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
