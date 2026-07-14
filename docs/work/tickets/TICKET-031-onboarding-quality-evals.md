---
artifact_type: ticket
id: TICKET-031
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
  detail_design: docs/work/designs/DESIGN-031-onboarding-quality-evals.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: Developer-Onboarding Quality Evaluations

## Context And Scope

Existing fixture tests prove that pages and evidence tokens exist. They do not prove that a developer can understand architecture, trace workflows, or identify safe change locations from docs alone.

## Acceptance Criteria

- [x] Static checks reject raw inventory prose, orphan pages, absolute local links, missing/broken related-page links, malformed diagrams, and export artifacts.
- [x] Contract checks prove page contracts are complete, uniquely owned, connected, and fully synthesized; the semantic evaluator checks reader-question and acceptance coverage.
- [ ] A semantic rubric scores purpose, boundaries, responsibilities, relationships, workflow clarity, change guidance, risks, tests, and evidence quality.
- [x] Mezon Desktop has a pinned onboarding question set covering bootstrap, auth, transport, state/UI, native integration, testing, and safe-change locations; media remains a non-critical follow-up surface.
- [ ] At least TypeScript application and Python/service repository shapes have equivalent fixture questions.
- [x] Critical questions are answerable from docs alone with evidence and no critical hallucinations in the pinned Mezon benchmark.
- [x] Eval result contract identifies failure and permits at most one bounded synthesis revision.
- [x] Benchmarks pin source commit, dirty state, visible existing docs, evidence scope, and generator/eval contract versions.
- [x] Quality-report contract requires separate `docs_only` reader and `source_and_evidence` auditor contexts.
- [ ] Results are comparable across model providers and do not encode a provider-specific prompt.
- [ ] Human UAT/sign-off is required before PHASE-002 completes.

## Approval

Approved by the user on 2026-07-14; implementation remains in progress.

## Impacted Areas

- Tests/evals: production fixtures, quality lint, docs-only Q&A harness, Mezon benchmark artifacts
- Skill: init/sync completion criteria and revision loop
- Docs: validation matrix, UAT, test verification, changelog
- Data/security/API: no auth or external API change expected; eval result storage may extend local state only if justified

## Detail Design

- Required: yes
- Link: `docs/work/designs/DESIGN-031-onboarding-quality-evals.md`
- Approval: approved

## Test Expectations

- Unit: static rules and rubric result parsing.
- Integration: multi-repository eval harness and revision routing.
- UAT: human reviews generated Mezon docs against the approved questions.
- Docs review: validation, testing strategy, requirements, context, and release notes.

## Verification Results

- Static/artifact-hygiene and revision-limit checks are implemented and covered by workspace tests.
- TypeScript, Python, and Rust fixtures prove evidence initialization and semantic persistence, but do not yet execute the full docs-only question rubric.
- Pinned Mezon commit `9d7ba654830c0e6278d0eb413a0eb9a992f01a55` synthesized six reader pages and reached `reader_docs_ready`; the reproducibility manifest, docs-only answers, source audit, and evaluator-independence caveat are recorded in `docs/work/benchmarks/MEZON-DESKTOP-9D7BA65.md`.
- Human UAT, cross-model comparison, full semantic rubric calibration, media-depth coverage, and executed TypeScript/Python docs-only question sets remain phase-level follow-up gates.
