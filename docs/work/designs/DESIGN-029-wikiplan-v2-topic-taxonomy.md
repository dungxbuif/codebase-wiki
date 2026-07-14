---
artifact_type: detail_design
id: DESIGN-029
status: in_review
owner: ai
approval: approved
human_fields: [approval, constraints, scope_decisions]
ai_fields: [problem, context_loaded, brownfield_scope, proposed_approach, design_tradeoffs, architecture_overview, execution_flow, api_data_model, security, test_plan, reconciliation_plan]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: PHASE-002
  ticket_or_bug: docs/work/tickets/TICKET-029-wikiplan-v2-topic-taxonomy.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  master_docs_touched: [docs/requirements/SPEC.md, docs/architecture/ARCHITECTURE.md]
---

# Detail Design: WikiPlan v2 And Topic Taxonomy

## 1. Context And Scope

WikiPlan v1 cannot carry the semantic and reader contract needed to generate high-quality docs. The deterministic area model also equates paths with components. This design changes the committed plan contract and candidate-selection boundary without introducing language-specific adapters.

Brownfield scope is limited to WikiPlan/store models, semantic topic candidates, core init/sync orchestration, serialization tests, and directly related skill/master docs. SQLite schema changes are not expected unless implementation proves page metadata must be queried locally.

## 2. Proposed Page Contract

WikiPlan v2 has two durable layers before page text exists:

1. Repository mental model: systems, actors, boundaries, runtimes, workflows, state/data ownership, external integrations, change risks, and known unknowns, each tied to evidence.
2. Hierarchical page plan: sections, reading order, canonical concept ownership, related pages, and a typed contract for every planned page.

Each planned page carries:

```yaml
path: docs/architecture/TRANSPORT.md
title: Network Transport
page_type: component
section_id: architecture
parent_page: docs/architecture/OVERVIEW.md
order: 30
importance: critical
audiences: [new_developer, maintainer]
reader_job: Trace a realtime request and locate safe transport changes.
prerequisites: [docs/architecture/OVERVIEW.md]
reader_questions:
  - How does a realtime request reach the backend?
  - Which component owns reconnect behavior?
scope: Transport boundary and connection lifecycle
out_of_scope: Full generated protobuf reference
required_sections: [mental_model, responsibilities, boundaries, interactions, change_guide, risks, tests]
diagram_slots:
  - kind: component
    question: Which layers own wire, protocol, client, and state responsibilities?
  - kind: sequence
    question: What happens during connect, disconnect, retry, and dispatch?
topic_ids: [transport, connection-lifecycle]
source_anchors:
  - selector: crates/mezon-client/src/transport.rs
    reason: Owns protocol request/response and event dispatch behavior.
    expected_claims: [transport_responsibility, event_dispatch]
  - selector: crates/mezon-store/src/connection.rs
    reason: Owns application-facing lifecycle and reconnect policy.
    expected_claims: [connection_owner, retry_policy]
evidence_gaps: [failure_path_tests]
related_pages: [docs/architecture/OVERVIEW.md, docs/workflows/AUTHENTICATION.md]
open_questions: []
refresh_triggers: [supporting_file_changed, dependency_edge_changed, linked_claim_stale]
acceptance_checks:
  - Each reader question has a direct answer and claim-local evidence.
  - Wire/protocol/client/store ownership is not conflated.
  - Diagrams add relationship or order information not already obvious from a list.
  - Change guidance names entrypoint, risks, and verification path.
status: planned
confidence: source-backed
```

`source_anchors` has no minimum item count. One authoritative source is better than eight tangential files. A selector may identify a file, symbol, command, existing doc, Git range, or explicit hypothesis; every anchor records why it is relevant.

Plan schema version increments with explicit compatibility behavior: v1 plans remain readable as legacy anchors, then are enriched during the next semantic init/sync rather than silently discarded.

## 3. Planning And Topic Discovery

```text
Bounded repository signals
      |
      v
Evidence-backed repository mental model
(systems, actors, boundaries, runtimes, workflows, state, integrations, risks)
      |
      v
LLM semantic clustering
(system, component, workflow, platform, framework, reference)
      |
      v
Hierarchy + concept ownership + reading-order review
      |
      v
Per-page contracts and bounded source anchors
```

A substantial topic needs an explainable responsibility plus at least one of: a runtime boundary, dependency boundary, user/system workflow, persistent-state ownership, external integration, safety-critical change surface, or reusable framework role. File count alone is neither required nor sufficient.

Planning runs in four explicit passes:

1. Discover: collect bounded evidence and record uncertainty without proposing pages.
2. Model: identify the repository's major concepts and relationships.
3. Architect: assign each concept one canonical home, create hierarchy/reading paths, and record uncovered concepts.
4. Contract: define the reader job, questions, evidence rationale, diagrams, refresh triggers, and acceptance checks for each page.

Draft generation may request more evidence when a contract gap remains, but it may not expand scope or create a new page implicitly. New concepts route back through the plan.

## 4. Hierarchy And Ownership Rules

- `QUICKSTART.md` is the only required reader entrypoint.
- Canonical overview files are stable landing slots, not mandatory empty templates.
- Dynamic pages live under their semantic owner, such as `architecture/TRANSPORT.md`, `workflows/AUTHENTICATION.md`, `components/STATE-MANAGEMENT.md`, or `data-models/MESSAGE-MODEL.md`.
- `areas/**` is read for v1 compatibility but is not a default v2 output location.
- Each `topic_id` has exactly one canonical page. Other pages may summarize and link but may not repeat the full explanation.
- Parent/child links define navigation; prerequisites define reading order; related links define lateral exploration. These relationships are distinct.
- A page is merged into its parent when it has no unique reader job, no substantial boundary/workflow, or insufficient evidence-backed content.
- Missing but important coverage becomes an explicit plan gap or open question, never a thin placeholder page.

## 5. Compatibility

- Read v1 canonical page identities and `areas/**` entries without data loss.
- Mark v1 path-derived area entries as `needs_replanning`; do not copy them automatically into v2 topic pages.
- Enrich preserved canonical pages with v2 contracts during the next init/sync.
- Preserve generated-region ownership and human edits while page paths are reconciled.
- Store `schema_version`, `planner_contract_version`, source commit, visible-docs manifest, and planning timestamp for benchmarkability.

## 6. Alternatives And Trade-offs

| Alternative | Decision |
| --- | --- |
| Keep v1 and place contracts only in prompts | Rejected; contracts would not survive model/session changes. |
| Derive pages directly from directories/packages | Rejected; physical layout and semantic boundaries differ. |
| Fully unconstrained LLM page tree | Rejected; canonical anchors and sync stability are still required. |
| Generic evidence candidates plus LLM semantic clustering | Chosen; portable while retaining semantic judgment. |
| File/source-count minimum per page | Rejected; count does not prove relevance or reader-question coverage. |
| File-tree and README-only planning | Rejected; planning must use bounded runtime, state, workflow, test, and existing-doc evidence. |

## 7. Verification

- Unit-test v2 mental-model/page fields, stable YAML, legacy-plan enrichment, and substantial-topic rules.
- Test that config-only paths do not become dynamic topics.
- Test that multi-crate/multi-package responsibilities can become component topics without Rust/TypeScript-specific core branches.
- Test canonical concept ownership, hierarchy integrity, prerequisite cycles, orphan pages, contract gaps, and merge-to-parent decisions.
- Test that evidence anchors require a relevance reason but no arbitrary minimum count.
- Run init/sync compatibility tests and the full workspace suite.

## 8. Reconciliation

- Update requirements, architecture, skill init/sync docs, CodeWiki standards, validation matrix, context, traceability, and changelog.
- Keep ADR-0005 accepted and link ADR-0010 as its reader-first refinement.
- API/ERD/security docs require no change unless implementation adds persisted page metadata.
