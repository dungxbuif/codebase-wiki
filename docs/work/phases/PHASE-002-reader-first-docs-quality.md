---
artifact_type: phase
id: PHASE-002
status: in_review
owner: human
human_fields: [goal, scope, out_of_scope, priority, success_criteria]
ai_fields: [risks, dependencies, verification_plan, completion_summary]
shared_fields: [status, trace, tickets_and_bugs]
trace:
  backlog_items: [BL-018]
  roadmap: docs/work/ROADMAP.md
  requirements: [docs/requirements/SPEC.md, docs/requirements/REQUIREMENTS.md]
  tickets:
    - docs/work/tickets/TICKET-029-wikiplan-v2-topic-taxonomy.md
    - docs/work/tickets/TICKET-030-reader-first-synthesis-and-diagrams.md
    - docs/work/tickets/TICKET-031-onboarding-quality-evals.md
  bugs:
    - docs/work/bugs/BUG-001-companion-bypasses-reader-synthesis.md
    - docs/work/bugs/BUG-002-installed-skill-version-drift.md
  research: docs/work/research/READER-FIRST-DOCS-AUDIT.md
  reference_research: docs/work/research/REFERENCE-DOCS-QUALITY-RESEARCH.md
  grok_audit: docs/work/research/GROK-WIKI-MEZON-AUDIT.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs:
    - docs/decisions/ADR-0010-reader-first-information-architecture.md
    - docs/decisions/ADR-0011-skill-distribution-version-integrity.md
  release_notes: docs/releases/CHANGELOG.md
---

# Phase: PHASE-002 Reader-First Documentation Quality

## Status

- Status: in_review
- Priority: high
- Approval: approved by the user on 2026-07-14; implementation and pinned Mezon benchmark are complete, with human UAT and broader comparison still in review
- Created: 2026-07-14

## Goal

Generate evidence-backed documentation that enables a new developer to form an accurate system mental model, trace important runtime workflows, and identify safe change locations without reading raw source inventories first.

## Scope

- WikiPlan v2 page contracts and concept-first topic taxonomy.
- Durable repository mental model, hierarchical page ownership, and task-oriented reading order before drafting.
- Reader-facing synthesis separated from deterministic evidence inventory.
- Required LLM mental-model, WikiPlan, page-synthesis, and bounded-revision orchestration between deterministic discovery and deterministic validation/merge.
- Installed skill/companion provenance, compatibility, and drift detection so the executed product contract is reproducible.
- Architecture, component, sequence, state, data, and deployment diagram policy.
- Static quality checks plus semantic onboarding evaluations.
- Mezon Desktop as the first real benchmark, followed by TypeScript and Python/service-shaped repositories.

## Out Of Scope

- Language/framework-specific core adapters.
- Replacing evidence persistence, sync ownership, or provider policy.
- Generating API reference for every source symbol.
- Optimizing primarily for shortest output or minimum page count.

## Tickets

| ID | Title | Status | Design |
| --- | --- | --- | --- |
| TICKET-029 | WikiPlan v2 and concept-first topic taxonomy | in_review | `docs/work/designs/DESIGN-029-wikiplan-v2-topic-taxonomy.md` |
| TICKET-030 | Reader-first synthesis and diagram contracts | in_review | `docs/work/designs/DESIGN-030-reader-first-synthesis-and-diagrams.md` |
| TICKET-031 | Developer-onboarding quality evaluations | in_review | `docs/work/designs/DESIGN-031-onboarding-quality-evals.md` |

## Bugs

| ID | Title | Status | Reproduction/design |
| --- | --- | --- | --- |
| BUG-001 | Companion bypasses reader synthesis | fixed, regression_verified, in_review | `docs/work/research/GROK-WIKI-MEZON-AUDIT.md`, `docs/work/designs/DESIGN-030-reader-first-synthesis-and-diagrams.md` |
| BUG-002 | Installed skill version drift is undetectable | fixed, regression_verified, in_review | `docs/work/designs/DESIGN-032-skill-install-version-integrity.md`, `docs/decisions/ADR-0011-skill-distribution-version-integrity.md` |

## Dependencies

- Existing repository detection and semantic evidence snapshots.
- ADR-0005 canonical docs structure and ADR-0009 manual-edit ownership.
- Skill-first init/sync workflows.
- Approved package/install provenance contract from ADR-0011.

## Risks

- Quality rules can become rigid templates that produce repetitive prose.
- Topic discovery can overfit Mezon Desktop or Rust workspace structure.
- Diagram requirements can encourage decorative diagrams unless triggers and evaluation are explicit.
- Semantic evaluation can become model-dependent unless it includes deterministic checks and observable onboarding questions.
- Live third-party references can mislead model comparisons when source commits or visible human docs differ.
- Orchestration can accidentally preserve current compatibility by silently falling back to deterministic prose; success states must make synthesis completion explicit.
- Stale local/global skill installations can invalidate quality comparisons unless the resolved skill root and content/contract identity are recorded.

## Success Criteria

- Critical system boundaries and workflows are represented as concepts, not top-level filesystem paths.
- Reader-facing pages contain no raw semantic snapshot or bulk symbol/import inventory.
- Reader-doc success is impossible unless the selected LLM completed the mental-model, WikiPlan, and page-synthesis stages.
- Every planned page has a unique reader job, canonical concept ownership, evidence rationale, and page-specific acceptance checks.
- Required diagrams are present, scoped, labeled, and evidence-backed.
- A developer can answer the approved onboarding benchmark from docs alone with evidence links.
- Quality gates produce comparable results across frontier models and at least three repository shapes.
- Benchmark results pin the same source commit and visible-docs manifest; no source-equivalence caveat is hidden.
- Exported artifacts have portable links, single frontmatter/related sections, supported Markdown constructs, and recorded source provenance.
- Generation and benchmark results record an integrity-checked installed skill/companion identity; legacy, drifted, or incompatible installations cannot report reader-doc success.
- Sync and manual-edit preservation remain regression-verified.

## Verification Plan

- Unit and integration tests for WikiPlan v2 serialization, topic candidate filtering, generated-page boundaries, and static quality linting.
- Fixture tests for Rust workspace, TypeScript application, and Python/service repository shapes.
- Mezon Desktop docs generation benchmark with docs-only onboarding UAT.
- Full workspace regression, docs review, traceability, and changelog reconciliation.
- Temporary install/doctor tests for manifest integrity, drift, compatibility, rollback, and project-state preservation.

## Gate Checklist

- [x] Phase has a linked requirement and research evidence.
- [x] Tickets and detail designs are drafted.
- [x] Risks, dependencies, and verification strategy are recorded.
- [x] BUG-001 is reproduced and its architectural root cause is recorded.
- [x] BUG-002 is reproduced and its distribution/versioning root cause is recorded.
- [x] Human approved ADR-0010, ADR-0011, and the four detail designs.
- [x] Implementation began after approval.
- [x] Pinned Mezon synthesis reached `reader_docs_ready` on a clean source commit.
- [x] Full workspace tests, scoped strict Clippy, installer/helper syntax, and installed-helper validation pass.

## Completion Summary

WikiPlan v2, legacy migration, hierarchy/ownership/cycle validation, the evidence-only companion boundary, reader-doc validation, package/install manifests, doctor/helper preflight, and run provenance are implemented. A clean checkout of Mezon Desktop commit `9d7ba65` produced six concept-first reader pages and passed deterministic validation as `reader_docs_ready`; the docs-only answers and source audit are recorded in `docs/work/benchmarks/MEZON-DESKTOP-9D7BA65.md`. Phase closure remains gated by human UAT and the accepted cross-model/TypeScript/Python comparison scope; those gates are not silently waived.
