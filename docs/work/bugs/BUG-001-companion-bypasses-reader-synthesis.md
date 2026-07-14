---
artifact_type: bug
id: BUG-001
status: in_review
owner: human
severity: high
priority: high
bug_markers: [reproduced, fixed, regression_verified]
human_fields: [symptoms, expected_behavior, priority, severity]
ai_fields: [reproduction, actual_behavior, root_cause, impact_scope, fix_strategy, regression_tests, verification_results]
shared_fields: [status, trace, docs_review]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  detail_design: docs/work/designs/DESIGN-030-reader-first-synthesis-and-diagrams.md
  research: docs/work/research/GROK-WIKI-MEZON-AUDIT.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Bug: Companion Bypasses Reader Synthesis

## Status

- Status: in_review
- Bug markers: reproduced, fixed, regression_verified
- Severity: high
- Priority: high
- Fix approval: approved by the user on 2026-07-14

## Symptoms

User-observed symptoms:

- Frontier-model CodeWiki outputs remain difficult for a new developer to use for onboarding.
- Reader pages overuse file/symbol mentions and expose implementation inventory without a clear system mental model.
- Gemini through Grok-Wiki produces materially better concept pages and diagrams from the same Mezon Desktop repository.

## Reproduction

1. Generate CodeWiki docs for `/Users/dungxbuif/workspace/mezon-desktop` using the current `init`/`sync` path; inspect the four supplied CodeWiki archives.
2. Inspect `/Users/dungxbuif/Downloads/wiki-local-mezon-desktop-d2680e1538f5-wiki.zip`, generated through Grok-Wiki with Gemini.
3. Trace CodeWiki init/sync from `crates/codewiki-core/src/lib.rs` into `crates/codewiki-docs/src/lib.rs`.
4. Observe that CodeWiki invokes `render_semantic_pages` directly and writes its output as final docs. No LLM mental-model, WikiPlan, page-synthesis, or revision call occurs.
5. Reproducibility: always for the current companion path.

## Expected Behavior

Based on the user request and the existing CodeWiki product contract, automatic init should explore the repository, have the selected LLM create an evidence-backed semantic WikiPlan, synthesize reader-oriented pages, validate them, and produce docs sufficient for developer onboarding.

## Historical Actual Behavior

The companion deterministically chooses the page set and source files, pads source lists, creates path-derived area pages, and writes semantic snapshots/evidence summaries directly into reader-facing docs. It reports initialization/sync success without executing the LLM planning and synthesis stages promised by the skill.

## Root Cause

- Earliest incorrect boundary: `codewiki-core` treats `codewiki-docs::render_semantic_pages` as the final documentation producer instead of an evidence/scaffolding helper.
- `codewiki-docs` combines discovery output, page planning, source selection, prose rendering, and final publication in one deterministic layer.
- Tests prove page/token presence and stable writes, so the contract violation is accepted as success.
- This is not only a weak prompt: the runtime path never gives a frontier model control over the semantic page plan or reader prose.

Single hypothesis: the deterministic final renderer causes the onboarding failure because it bypasses the model synthesis stages that the product and skill promise.

## Pattern Comparison

The Grok-Wiki artifact uses a concept-first page manifest with a bounded source set per page, then produces mechanism-level prose and diagrams. Grok-Wiki's official workflow also separates repository reading, evidence-based outlining, page writing, and exact source attachment. The CodeWiki path collapses those stages into snapshot rendering.

## Impact Scope

- Code: `crates/codewiki-core`, `crates/codewiki-docs`, CLI init/sync orchestration.
- Skill: `skill/codewiki/SKILL.md` and init/sync/docs-structure references do not match runtime behavior.
- Users: new developers cannot rely on generated docs for onboarding or safe change location.
- Contracts: “skill is the product” and “fully automatic init” are violated in practice.
- Data: no source data loss; generated-doc quality and stored plan/provenance are affected.

## Fix Strategy

- Keep deterministic discovery, evidence persistence, static validation, normalization, safe merge, and provenance recording in the companion.
- Make LLM repository mental-model, WikiPlan, per-page synthesis, and one bounded revision required stages for reader-doc success.
- Do not publish deterministic snapshot summaries as successful reader pages. Permit them only as explicitly incomplete evidence/debug artifacts.
- Replace filename/role source padding with plan-selected evidence plus relevance reasons.
- Add contract and onboarding regression tests before implementation.

The user approved ADR-0010 and DESIGN-030; implementation now enforces this boundary.

## Regression Tests

- Red contract test: current `init` must fail reader-doc completion when no synthesis result is supplied; current behavior incorrectly passes.
- Red output test: generated Quickstart/Source Map/Architecture must reject raw semantic snapshot bodies and universal source-first blocks.
- Red orchestration test: a successful reader-doc run records mental-model, WikiPlan, synthesis, validation, and provenance stages.
- Comparative fixture: pinned Mezon Desktop commit must pass the approved docs-only onboarding questions without source fallback.
- Artifact hygiene: reject duplicate frontmatter/related sections, absolute `file://` links, unsupported renderer tags, malformed diagrams, and missing commit/dirty-state metadata.

## Verification Results

- Reproduction: confirmed by static control-flow inspection of current init/sync and renderer code.
- Comparative evidence: confirmed from the supplied Grok-Wiki archive and source-grounding spot checks.
- Fix verification: companion init/sync now write only `docs/evidence/**`, record `synthesis_incomplete`, and require WikiPlan/model/quality/provenance checks before `reader_docs_ready`.
- Regression verification: full workspace passed 43 tests across 14 suites; temporary installed-package init emitted exactly four evidence pages and no reader page.

## Fix/Test Attempt Log

- Same-path failure attempts: 1 / 3 (expected red contract test before implementation)
- Total fix/test cycles: 3 / 5
- Blocked by loop guard: no
- Human/design input needed: pinned Mezon reader-doc UAT before final verification.

## Docs Review

- Requirements: REQ-014 already captures reader-first onboarding quality.
- Architecture: must be reconciled after the orchestration boundary is approved and implemented.
- API/ERD/data: not affected.
- ADR: ADR-0010 updated with the required LLM-stage boundary.
- Context/backlog/phase/validation: linked in PHASE-002 planning artifacts.

## Completion Checklist

- [x] Root cause identified
- [x] Fix implemented
- [x] Regression test added or updated
- [x] Impacted behavior retested
- [x] Fix/test loop guard respected
- [x] Validation matrix has planned regression coverage
- [x] Master docs reconciled after implementation
- [x] Docs review completed for implemented boundary; Mezon UAT remains phase-level work
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
