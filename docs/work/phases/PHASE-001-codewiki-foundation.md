---
artifact_type: phase
id: PHASE-001
status: ready
owner: human
human_fields:
  - goal
  - scope
  - out_of_scope
  - priority
  - success_criteria
ai_fields:
  - risks
  - dependencies
  - verification_plan
  - completion_summary
shared_fields:
  - status
  - trace
  - tickets_and_bugs
trace:
  backlog_items: [BL-001, BL-002, BL-003, BL-004, BL-005, BL-007, BL-008, BL-009]
  roadmap: docs/work/ROADMAP.md
  requirements: [docs/requirements/SPEC.md, docs/requirements/REQUIREMENTS.md]
  tickets: [docs/work/tickets/TICKET-001-rust-cli-workspace.md, docs/work/tickets/TICKET-002-skill-first-installer.md, docs/work/tickets/TICKET-003-config-storage-skeleton.md]
  bugs: []
  test_verification: TBD
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md, docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md, docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md]
  release_notes: TBD
---

# Phase: PHASE-001 CodeWiki Foundation

## Field Ownership

- Human owns goal, scope, out of scope, priority, and success criteria.
- AI owns risks, dependencies, verification plan, implementation evidence, and completion summary.
- Shared fields include status, trace links, and ticket/bug list.

## Status

- ID: PHASE-001
- Status: ready
- Owner: human
- Created: 2026-07-13
- Updated: 2026-07-13

## Trace Links

- Backlog items: `BL-001`, `BL-002`, `BL-003`, `BL-004`, `BL-005`, `BL-007`, `BL-008`, `BL-009`
- Roadmap: `docs/work/ROADMAP.md`
- Requirements: `docs/requirements/SPEC.md`, `docs/requirements/REQUIREMENTS.md`
- Tickets: `docs/work/tickets/TICKET-001-rust-cli-workspace.md`, `docs/work/tickets/TICKET-002-skill-first-installer.md`, `docs/work/tickets/TICKET-003-config-storage-skeleton.md`
- Bugs: none
- Test verification: TBD
- Validation matrix: `docs/work/VALIDATION_MATRIX.md`
- ADRs: `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`, `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`, `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`
- Release notes: TBD

## Goal

Create the complete foundation for CodeWiki as a repo-native skill that can initialize a semantic wiki for arbitrary repositories, preserve durable state, and support future sync and Q&A workflows.

## Scope

- Define the CodeWiki product contract and durable requirements.
- Design repository detection, semantic exploration, wiki planning, generation, sync, and Q&A boundaries.
- Establish config, storage, cache, and migration rules that survive model/session changes.
- Design the skill package, installer, and Rust companion-tool boundaries.
- Decide the minimal first tool surface and integration boundaries.
- Define optional runtime provider install guidance for target repositories.
- Use `references/openwiki` and `references/deepwiki-open` as reference submodules for comparative study.
- Produce tickets and detail designs before implementation work starts.

## Out Of Scope

- Shipping a reduced MVP that cannot support the complete product direction.
- Building language/framework-specific adapters into the core.
- Adding multiple overlapping code-intelligence tools before the first provider boundary is proven.
- Treating chat history as durable project memory.

## Tickets And Bugs

| ID | Type | Title | Status | Link |
| --- | --- | --- | --- | --- |
| TICKET-001 | ticket | Rust CLI workspace scaffold | done | `docs/work/tickets/TICKET-001-rust-cli-workspace.md` |
| TICKET-002 | ticket | Skill-first installer | done | `docs/work/tickets/TICKET-002-skill-first-installer.md` |
| TICKET-003 | ticket | Config/storage skeleton | done | `docs/work/tickets/TICKET-003-config-storage-skeleton.md` |
| TICKET-004 | ticket | SQLite state migrations | done | `docs/work/tickets/TICKET-004-sqlite-state-migrations.md` |
| TICKET-005 | ticket | SQLite executor and state paths | done | `docs/work/tickets/TICKET-005-sqlite-executor-paths.md` |
| TICKET-006 | ticket | Init skeleton | done | `docs/work/tickets/TICKET-006-init-skeleton.md` |
| TICKET-007 | ticket | Repository detection v1 | done | `docs/work/tickets/TICKET-007-repo-detection-v1.md` |
| TICKET-008 | ticket | WikiPlan and evidence models | done | `docs/work/tickets/TICKET-008-wikiplan-evidence-models.md` |
| TBD | ticket | Implement docs generation and sync loop | draft | TBD |

## Dependencies

- Upstream reference repos available under `references/`.
- Codex skill packaging and installer decisions.
- Rust toolchain and crate/workspace decisions for companion tooling.
- Decision on optional code-intelligence provider boundary. Provider selection is target-repo specific.
- Optional runtime provider choices remain target-repo specific.
- Local filesystem and SQLite availability.
- Codex skill packaging requirements when implementation reaches skill installation.

## Risks

- Semantic quality can degrade if evidence, hypotheses, and confidence are not modeled explicitly.
- Over-tooling can make the skill brittle; the first provider boundary must stay narrow.
- Fully automatic init needs strong guardrails because it intentionally avoids approval gates.
- Storage reuse across LLM/model changes requires schema stability and migrations from the beginning.
- Sync can overwrite useful human edits unless docs ownership and generated regions are designed early.

## Success Criteria

- Requirements and architecture docs describe the complete product, not an MVP.
- Tickets are small enough to execute while preserving the full architecture.
- Persistent config/state/cache model is specified before implementation.
- Skill packaging and installer are specified before full init/sync implementation.
- Rust companion-tool workspace design is specified before companion implementation.
- Reference repos are available as submodules and can be studied without vendoring their runtime assumptions.
- Validation matrix contains planned proof rows for the core product behaviors.

## Verification Plan

- Verify submodules with `git submodule status`.
- Verify harness docs exist and runtime files reference CodeWiki, not the source Harness project.
- For each implementation ticket, record command-based proof or an explicit skip reason.
- Add fixture repositories later to test detection, planning, generation, sync, and Q&A.

## Gate Checklist

- [x] Phase has linked requirements or explicit discovery goal
- [x] Tickets/bugs are created or planned
- [x] Risks and dependencies are recorded
- [x] Verification plan is defined
- [x] Release/changelog need is identified

## Completion Summary

Not complete. Foundation planning is ready for ticket creation and implementation.
