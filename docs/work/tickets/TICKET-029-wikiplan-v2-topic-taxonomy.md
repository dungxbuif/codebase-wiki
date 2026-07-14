---
artifact_type: ticket
id: TICKET-029
status: in_review
owner: human
priority: high
lane: high-risk
human_fields: [title, priority, acceptance_criteria, scope, approval]
ai_fields: [impacted_areas, test_expectations, verification_results, docs_review, context_updates]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: PHASE-002
  detail_design: docs/work/designs/DESIGN-029-wikiplan-v2-topic-taxonomy.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: WikiPlan v2 And Concept-First Topic Taxonomy

## Context And Scope

WikiPlan v1 records only page identity/status fields and cannot express why a page exists, who reads it, what it must explain, which relationships need diagrams, or how it becomes stale. Top-level filesystem areas are currently promoted into docs even when they are configuration files rather than system components.

Scope includes the committed WikiPlan model, serialization, topic candidate model, generic evidence signals, compatibility behavior, and tests. Final prose generation and onboarding scoring are handled by TICKET-030 and TICKET-031.

## Acceptance Criteria

- [x] WikiPlan records an evidence-backed repository mental model before proposing pages.
- [x] Planned pages record hierarchy/order, page type, reader job, audience, prerequisites, reader questions, scope/out-of-scope, required sections, diagram slots, source anchors with relevance reasons, related pages, open questions, refresh triggers, and acceptance checks.
- [x] Canonical landing pages and dynamic concept pages use the same page contract.
- [x] Topic candidates distinguish systems, components, workflows, platform boundaries, framework concepts, and reference/evidence pages through the reader-first planning contract.
- [x] A path or file alone cannot qualify as a substantial topic in the planning contract.
- [x] Dynamic pages live under semantic owners; v1 `areas/**` entries are compatibility inputs and are not regenerated from top-level paths.
- [x] Config/docs files such as `Cargo.toml`, `README.md`, and `.github` are evidence unless a separate semantic boundary is established.
- [x] Existing plan files have explicit compatibility/migration behavior.
- [x] Topic ownership, parent/child links, prerequisites, related links, and uncovered-plan gaps are validated before drafting.
- [x] Tests cover serialization stability and Rust/TypeScript/Python-shaped topic discovery without language adapters.
- [ ] UAT is required as part of PHASE-002 onboarding evaluation.

## Approval

Approved by the user on 2026-07-14.

## Impacted Areas

- Code: `crates/codewiki-store`, `crates/codewiki-explore`, `crates/codewiki-core`
- Contracts: committed `plan.yml`, page planning, init/sync
- Docs: requirements, architecture, skill init/sync references, standards
- Data/security/API: no database, auth, or external API change expected; committed plan schema changes

## Detail Design

- Required: yes
- Link: `docs/work/designs/DESIGN-029-wikiplan-v2-topic-taxonomy.md`
- Approval: approved

## Test Expectations

- Unit: WikiPlan v2 model and stable YAML serialization.
- Integration: init/sync compatibility and topic candidate filtering.
- UAT: docs taxonomy supports approved onboarding questions.
- Docs review: requirements, architecture, ADR, skill references, context, validation, changelog.

## Verification Results

- WikiPlan serialization tests pass with schema v2 mental-model and complete page-contract fields.
- Companion init/sync tests prove no `areas/**` or reader pages are emitted from deterministic path discovery.
- Sync preserves a v1 plan as `plan.v1.legacy.yml`, creates a v2 enrichment scaffold, and refuses conflicting/unsupported legacy state.
- Deterministic validation rejects duplicate ownership, missing hierarchy/prerequisites, cycles, unsynthesized plan pages, and unplanned reader pages.
- Pinned Mezon synthesis at `9d7ba65` exercised a six-page semantic plan with clean source provenance and reached `reader_docs_ready`.
