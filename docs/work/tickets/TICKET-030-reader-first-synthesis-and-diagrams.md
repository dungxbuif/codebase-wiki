---
artifact_type: ticket
id: TICKET-030
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
  bug: docs/work/bugs/BUG-001-companion-bypasses-reader-synthesis.md
  detail_design: docs/work/designs/DESIGN-030-reader-first-synthesis-and-diagrams.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: Reader-First Synthesis And Diagram Contracts

## Context And Scope

Reader-facing pages currently expose semantic snapshots and bulk symbol/import lists. BUG-001 confirms that init/sync writes those deterministic summaries as final docs without executing the skill's LLM planning/synthesis contract. This ticket corrects the runtime boundary, separates evidence packets from explanatory docs, and makes page-type content and diagram rules executable.

## Acceptance Criteria

- [x] Raw semantic snapshots and bulk symbol/import inventories are absent from reader-facing page bodies.
- [x] A successful reader-doc run records completed LLM mental-model, WikiPlan, page-synthesis, and bounded-revision stages; missing/failed synthesis reports incomplete instead of publishing deterministic fallback prose.
- [x] The companion is limited to deterministic discovery/persistence, input/output validation, normalization, safe merge, and provenance; evidence/debug summaries are never mislabeled as completed reader docs.
- [x] Every page starts from its approved reader questions and explains concepts before code symbols.
- [x] Reader-facing pages start with purpose/scope/mental model; source inventories are optional appendices or evidence-page content, never a universal first block.
- [x] Quickstart, source map, architecture, workflow, component, domain, data, API, operations, testing, conventions, glossary, and evidence page types have explicit content contracts.
- [x] Architecture/component pages require a labeled ASCII diagram when relationships materially aid understanding.
- [x] Multi-actor or asynchronous workflows require Mermaid sequence diagrams; lifecycle pages require state diagrams; relational persistence pages require ERDs when applicable.
- [x] Generated links are repository-relative or stable source permalinks; absolute `file://` links are rejected.
- [x] Topic pages include responsibility, boundaries, collaborators, important flows, change guidance, risks, tests, evidence, and related pages when applicable.
- [x] Page generation uses relevance-based evidence with no arbitrary minimum file, citation, page, or diagram count.
- [x] Cross-page review requires one canonical home per concept, valid reading paths, consistent terminology, and bounded overlap.
- [x] Diagram failures and generation error strings fail acceptance and cannot be cached as successful pages.
- [x] Duplicate frontmatter/sections, unsupported renderer tags, temporary absolute links, and missing commit/dirty-state metadata fail artifact acceptance.
- [x] Sync preserves human edits and does not reintroduce raw evidence into reader pages.

## Approval

Approved by the user on 2026-07-14.

## Impacted Areas

- Skill: `skill/codewiki/references/docs-structure.md`, `init.md`, `sync.md`, and a new reader-first contract reference
- Code: `crates/codewiki-docs`, `crates/codewiki-core`
- Standards: CodeWiki docs quality and diagram policy
- Data/security/API: no schema, auth, or external API change expected

## Detail Design

- Required: yes
- Link: `docs/work/designs/DESIGN-030-reader-first-synthesis-and-diagrams.md`
- Approval: approved

## Test Expectations

- Unit: page contract rendering/lint rules, orchestration result states, normalization, and disallowed inventory patterns.
- Integration: init/sync must not report reader-doc success without synthesis artifacts; accepted output shape and generated-region preservation.
- UAT: Mezon Desktop docs are readable without source-first navigation.
- Manual: render/inspect diagrams and cross-links.

## Verification Results

- Unit/integration regressions prove deterministic init/sync emit evidence only and remain `synthesis_incomplete`.
- `codewiki validate` rejects incomplete plan/run/model provenance, Grok export artifacts, raw inventories, broken/orphan links, malformed Mermaid fences, and unpassed isolated review stages.
- Package `0.2.2` keeps a compact always-active reader contract, requires current-working-tree inspection, exposes qualitative failure patterns and typed mental-model fields, and ports approved page-type contracts into the always-loaded references without count quotas.
- Packaged contract regression fails if these invariants or contract versions disappear.
- Full workspace: 50 tests passed across 15 suites on 2026-07-14; scoped strict Clippy passes.
- Remaining review gap: human UAT and the broader accepted comparison scope remain phase-level gates.
