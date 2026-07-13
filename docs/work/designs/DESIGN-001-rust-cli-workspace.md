---
artifact_type: detail_design
id: DESIGN-001
status: done
owner: ai
approval: approved
human_fields:
  - approval
  - constraints
  - scope_decisions
ai_fields:
  - problem
  - context_loaded
  - brownfield_scope
  - proposed_approach
  - design_tradeoffs
  - architecture_overview
  - execution_flow
  - api_data_model
  - security
  - test_plan
  - reconciliation_plan
shared_fields:
  - status
  - trace
  - small_task_exemption
trace:
  backlog_item: BL-007
  requirement: REQ-006
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-001-rust-cli-workspace.md
  test_verification: docs/work/tickets/TICKET-001-rust-cli-workspace.md#verification-results
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/tickets/TICKET-001-rust-cli-workspace.md#docs-review
  adrs: [docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md]
  master_docs_touched:
    - docs/architecture/ARCHITECTURE.md
    - docs/requirements/REQUIREMENTS.md
    - docs/requirements/SPEC.md
---

# DETAIL DESIGN: Rust CLI Workspace

## Status

- ID: DESIGN-001
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-001-rust-cli-workspace.md`
- Approval: approved by standing user instruction to continue autonomously
- Author: Codex
- Updated: 2026-07-13

## Trace Links

- Backlog item: `BL-007`
- Requirement: `REQ-006`
- Phase: `PHASE-001`
- Ticket/Bug: `docs/work/tickets/TICKET-001-rust-cli-workspace.md`
- Test verification: `docs/work/tickets/TICKET-001-rust-cli-workspace.md#verification-results`
- Validation matrix: `docs/work/VALIDATION_MATRIX.md`
- Docs review: `docs/work/tickets/TICKET-001-rust-cli-workspace.md#docs-review`
- ADRs: `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- Master docs touched: `docs/architecture/ARCHITECTURE.md`, `docs/requirements/REQUIREMENTS.md`, `docs/requirements/SPEC.md`

---

## 1. Context & Scope

### Problem Statement

- Problem statement: CodeWiki needs a Rust-first CLI/runtime foundation that can grow into semantic init, sync, planning, storage, and Q&A without inheriting OpenWiki or deepwiki-open runtime architecture.
- Why now: The repository has reference submodules and an accepted ADR for Rust CLI strategy; the next executable step is a crate layout that future work can build on safely.
- Success outcome: A compiling Rust workspace exists with clear crate boundaries and a minimal `codewiki` binary.

### Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/standards/README.md`
- `docs/standards/QUALITY_BAR.md`
- `docs/standards/VALIDATION.md`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`
- `docs/templates/DETAIL_DESIGN.md`
- `docs/templates/TICKET.md`
- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`

### Brownfield Scope

- Touched modules/files: new root `Cargo.toml`, `.gitignore`, `crates/**`, ticket/design docs, runtime harness docs.
- Direct dependencies inspected: current harness docs and Rust toolchain availability.
- Contracts affected: `codewiki` CLI command surface begins as a public contract.
- Known unknowns: exact provider API, final storage schema, exact Codex skill packaging layout.
- Scope expansion reason: none.

### Small Task Exemption

- Small task exemption: no
- Reason: This creates runtime architecture and public CLI boundaries.
- Impact checked: API=yes, DB=no, Security=no, Runtime=yes, Standards=no

---

## 2. Design Considerations & Trade-offs

| Consideration / Alternative | Pros | Cons | Decision |
| --- | --- | --- | --- |
| Single Rust crate | Fastest to start | Blurs CLI, core, storage, and provider boundaries | rejected |
| Large full implementation now | More immediate functionality | High risk before schemas/provider boundaries are designed | rejected |
| Workspace with small typed crates | Clear boundaries, testable, easy to extend | Slight initial ceremony | chosen |
| Pull external CLI/storage deps immediately | Faster polished CLI | Network/dependency churn before boundaries settle | rejected for scaffold |

---

## 3. Architecture Overview

### Component Responsibilities

| Component | Role |
| --- | --- |
| `codewiki-cli` | Owns binary entrypoint, argument handoff, process exit behavior |
| `codewiki-core` | Owns command parsing/execution orchestration and stable domain contracts |
| `codewiki-detect` | Will detect repo stack signals without core language adapters |
| `codewiki-store` | Will own SQLite-backed durable state and migrations |
| `codewiki-provider` | Will define replaceable code-intelligence/LLM provider traits |
| `codewiki-docs` | Will own generated docs paths, page model, and sync-facing docs helpers |

```text
+----------------+
| codewiki binary|
+-------+--------+
        |
        v
+----------------+       +------------------+
| codewiki-core  | ----> | codewiki-detect  |
+-------+--------+       +------------------+
        |                +------------------+
        +--------------> | codewiki-store   |
        |                +------------------+
        |                +------------------+
        +--------------> | codewiki-provider|
        |                +------------------+
        |                +------------------+
        +--------------> | codewiki-docs    |
                         +------------------+
```

---

## 4. Execution Flow

1. `codewiki-cli` collects process args.
2. `codewiki-core` parses args into a typed command.
3. `codewiki-core` executes the command using internal boundaries.
4. For this scaffold, `help`, `version`, and `status` return deterministic text.
5. Future tickets replace placeholders with detection, storage, provider, docs generation, sync, and Q&A behavior.

---

## 5. API & Data Model Design

### API Changes

- CLI binary: `codewiki`
- Initial commands:
  - `codewiki help`
  - `codewiki --help`
  - `codewiki version`
  - `codewiki --version`
  - `codewiki status`

### Data Model Changes

- Table / Collection: none in this ticket.
- Added / Modified fields: none.
- Schema Migration needed: no.

---

## 6. Security & Authorization

- Authentication changes: none.
- Authorization / Permissions: none.
- Data Privacy / PII impact: none.
- Input Validation: reject unknown CLI commands with a non-zero exit code and short message.

---

## 7. Implementation & Verification Plan

### Impacted Areas

- Code/modules: root Cargo workspace and `crates/**`.
- Product behavior: initial `codewiki` CLI commands.
- API/contracts: CLI command names begin as a public surface.
- Data/schema: no persistent schema yet.
- Security/auth: no security boundary changes.
- Deployment/runtime: Rust build/test runtime.
- Docs: ticket, design, context, backlog, phase, validation, traceability, changelog.

### Test Plan

- Unit: `rtk cargo test`
- Integration: not required for scaffold; command behavior is covered by unit tests in `codewiki-core`.
- E2E: not required until CLI has repository-mutating behavior.
- UAT: not required for scaffold because no user workflow is complete yet.
- Manual/platform: `rtk cargo run -p codewiki-cli -- status`
- Docs review: verify harness links and status fields are updated.

### Validation Matrix Impact

- Update required: yes
- Row(s): `REQ-006`
- Reason if no update required: N/A

### Verification Results

- `rtk cargo fmt --all --check`: pass.
- `rtk cargo test`: pass, 4 tests passed across 11 suites.
- `rtk cargo run -p codewiki-cli -- status`: pass, CLI returned scaffold status.

---

## 8. Reconciliation Plan

- Requirements docs: update
- Architecture docs: update
- API docs: update
- ERD docs: no change
- SDD docs: no change
- ADR: no change, ADR-0002 already exists
- Context: update

### Docs Review Checklist

- Code changed but docs unchanged reason: N/A
- Requirements updated or not needed reason: update `REQ-006` implementation status after scaffold.
- Architecture updated or not needed reason: update crate boundaries and CLI command surface.
- API updated or not needed reason: update CLI API docs.
- ERD/data updated or not needed reason: no data model exists in this ticket.
- SDD updated or not needed reason: not used yet.
- ADR created or not needed reason: ADR-0002 already records the durable decision.
