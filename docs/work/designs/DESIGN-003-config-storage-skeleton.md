---
artifact_type: detail_design
id: DESIGN-003
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-003
  requirement: REQ-003
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-003-config-storage-skeleton.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md]
---

# DETAIL DESIGN: Config And Storage Skeleton

## Status

- ID: DESIGN-003
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-003-config-storage-skeleton.md`
- Approval: approved by current autonomous continuation
- Updated: 2026-07-13

## Context & Scope

Problem: CodeWiki needs deterministic defaults for committed config, plan, local agent guidance, and durable state layout before implementing `init`.

Scope:

- Add Rust companion models for `.codewiki/config.yml`, `.codewiki/plan.yml`, and `.codewiki/AGENTS.md`.
- Include docs-first lazy activation and runtime provider selection policy in generated guidance.
- Keep actual filesystem writes out of this ticket.

Out of scope:

- SQLite migrations.
- Real repo identity resolution.
- Full `codewiki init` implementation.

## Design

`codewiki-store` owns storage and config schema defaults:

- `StoreLayout`: committed config, plan, local AGENTS path, state/cache summaries.
- `CodeWikiConfig`: schema version, docs root, plan path, agents path, tool policy.
- `WikiPlan`: initial plan skeleton and evidence policy.
- `render_target_agents_md`: target-repository `.codewiki/AGENTS.md` content.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test`
- `rtk cargo run -p codewiki-cli -- status`

## Reconciliation

- Requirements: update `REQ-003` status.
- Architecture: update storage component notes if needed.
- API: no public command change.
- ADR: no new ADR; ADR-0001 and ADR-0004 cover the decision.

