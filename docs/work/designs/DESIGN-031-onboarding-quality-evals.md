---
artifact_type: detail_design
id: DESIGN-031
status: in_progress
owner: ai
approval: approved
human_fields: [approval, constraints, scope_decisions]
ai_fields: [problem, context_loaded, brownfield_scope, proposed_approach, design_tradeoffs, architecture_overview, execution_flow, api_data_model, security, test_plan, reconciliation_plan]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: PHASE-002
  ticket_or_bug: docs/work/tickets/TICKET-031-onboarding-quality-evals.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  master_docs_touched: [docs/requirements/SPEC.md, docs/standards/CODEWIKI.md]
---

# Detail Design: Developer-Onboarding Quality Evaluations

## 1. Problem

Presence tests allow unreadable inventories to pass. Quality proof must measure whether docs answer real developer questions accurately and guide safe changes without source-first exploration.

## 2. Evaluation Layers

1. Structural lint: page identity, links, sections, diagrams, orphan/stub detection, disallowed raw-inventory patterns.
2. Contract coverage: each declared reader question, required section, diagram slot, evidence gap, and acceptance check has an observable result.
3. Evidence audit: important claims have relevant source/command/doc/Git evidence or explicit uncertainty; citations resolve and support the claim.
4. Diagram audit: syntax/renderability, question answered, evidence support, labels/scope, and information gain over adjacent prose.
5. Cross-page audit: canonical ownership, terminology, overlap, prerequisites, related links, and reading-path completeness.
6. Semantic rubric: purpose, boundaries, responsibilities, interactions, workflow clarity, change guidance, risks, tests, and uncertainty handling.
7. Docs-only onboarding Q&A/tasks: answer versioned benchmark questions and choose safe change locations using generated docs without source fallback.
8. Human UAT: maintainer/new-developer review of the generated wiki and a bounded first-change exercise.

## 3. Mezon Desktop Benchmark

Critical questions include:

- What are the main runtime components and dependency direction?
- How does application bootstrap reach the first authenticated UI?
- How do authentication and session refresh work?
- How does realtime transport connect, retry, and dispatch events?
- Which layer owns domain state and how does UI observe it?
- Where should a developer change notifications, deep links, or tray behavior?
- How are GPUI, Tokio, and other async execution contexts separated?
- Where do Protobuf, media, voice/video, and localization fit?
- Which files are high-risk change surfaces?
- Which commands/tests prove a change is safe?

Each question records expected concepts, required evidence, critical misconceptions, and source-fallback status.

The benchmark also includes task cards such as “change tray notification behavior safely” or “modify reconnect policy.” A task card must identify the canonical page, owning component, starting source anchor, likely risks, and verification path from docs alone.

## 4. Fair Benchmark Protocol

Each benchmark fixture records:

- exact source commit, dirty state, and submodule state;
- all existing human/generated docs visible to the generator;
- include/exclude paths and evidence manifest;
- CodeWiki version, WikiPlan schema, planner/content/eval contract versions;
- model/provider and generation parameters for comparison metadata only;
- generated page tree and run timings/cost when available.

The Mezon baseline must be regenerated from one pinned commit. The live DeepWiki result at `b182aed3` may be reviewed as a reader-experience reference, but it cannot be scored against CodeWiki output from `9d7ba65` as if inputs were equal. Reference docs and expected benchmark answers are eval-only and are not exposed to the generator.

## 5. Evaluation Isolation

- The docs-only reader receives generated reader-facing docs, not source, WikiPlan evidence packets, SQLite, or benchmark oracle answers.
- A separate source auditor verifies claims/citations against the pinned repository evidence.
- Deterministic checks evaluate paths, links, required contract fields, citation resolution, forbidden inventory patterns, and diagram parsing.
- Semantic judgment reports rubric evidence and named gaps; it does not expose or require hidden reasoning traces.
- Human UAT remains required for the first acceptance of each repository shape and for material rubric changes.

## 6. Pass Criteria

- All critical questions are answerable docs-only with supporting evidence.
- No critical architecture or ownership hallucination is accepted.
- Every critical component and workflow has one canonical home.
- Static guardrails pass with no raw inventory or local absolute links.
- Diagram-required pages pass render/structure review.
- Every required page-contract question and acceptance check is covered or explicitly reported as an unresolved gap.
- Task cards identify a safe starting location, risks, and verification without source-first exploration.
- Human UAT signs off that the wiki is sufficient to begin a bounded first change.

Scores diagnose weak pages but do not average away a critical failure. A failed critical question routes one bounded revision to the owning page; repeated failure follows the repository loop guard.

Rubric dimensions use `0 = absent/incorrect`, `1 = partial`, `2 = sufficient`, and `3 = strong`. A critical dimension must score at least 2, but binary critical failures take precedence over averages.

## 7. Portability

The same harness runs against a TypeScript application and Python/service fixture. Questions vary by repository evidence, while rubric dimensions remain stable. Provider/model identity is recorded for comparison but never embedded in pass logic.

## 8. Verification And Reconciliation

- Unit-test lint/rubric data structures and critical-failure behavior.
- Integration-test docs-only context isolation and revision routing.
- Regression-test error-content rejection, citation resolution, diagram parse failures, and cross-page ownership conflicts.
- Record exact generation/eval commands and model/provider metadata.
- Add UAT and test-verification artifacts before phase completion.
- Update validation matrix, testing strategy, skill completion rules, context, traceability, and changelog.

## 9. Portable And Capability-Specific Questions

The docs-only evaluation combines two question sets:

1. Portable questions that apply to any repository: purpose and users, verified start path, major responsibility boundaries, one critical end-to-end workflow, safe starting location for a representative change, risks, and verification.
2. Capability-specific questions derived from the repository mental model and WikiPlan. API, persistence, authentication, deployment, eventing, native integration, or similar questions become required only when repository evidence establishes that capability.

Each critical question records expected concepts, canonical owning page, required evidence, critical misconceptions, and whether the answer is an explicit known unknown. Fixed generic questions must not force irrelevant API, data, or deployment pages into repositories that do not have those capabilities.
