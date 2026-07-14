---
artifact_type: adr
id: ADR-0010
status: accepted
owner: shared
human_fields: [decision_approval, final_status]
ai_fields: [context, alternatives_considered, consequences, linked_work]
shared_fields: [decision, trace]
trace:
  requirement: REQ-014
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  tickets_or_bugs: [BUG-001, TICKET-029, TICKET-030, TICKET-031]
  detail_design:
    - docs/work/designs/DESIGN-029-wikiplan-v2-topic-taxonomy.md
    - docs/work/designs/DESIGN-030-reader-first-synthesis-and-diagrams.md
    - docs/work/designs/DESIGN-031-onboarding-quality-evals.md
  master_docs:
    - docs/requirements/SPEC.md
    - docs/architecture/ARCHITECTURE.md
    - docs/standards/CODEWIKI.md
  related_decisions:
    - docs/decisions/ADR-0005-codewiki-generated-docs-structure.md
  research:
    - docs/work/research/READER-FIRST-DOCS-AUDIT.md
    - docs/work/research/REFERENCE-DOCS-QUALITY-RESEARCH.md
    - docs/work/research/GROK-WIKI-MEZON-AUDIT.md
  release_notes: docs/releases/CHANGELOG.md
---

# ADR-0010: Reader-First Information Architecture

## Status

Accepted on 2026-07-14 after the user authorized implementation with “sửa luôn đi”.

## Context

Generation trials on Mezon Desktop show that evidence-rich output can remain unsuitable for onboarding. Raw file, symbol, and lexical-import inventories dominate reader-facing pages; filesystem paths become false subsystem boundaries; diagrams are absent or inconsistent; and tests accept page presence without proving comprehension.

ADR-0005 remains valid as a stable canonical landing structure, but it does not define a sufficient page-content contract or dynamic topic model.

Reference research adds a second warning. OpenWiki has strong exploration and sync discipline but no onboarding-quality gate. deepwiki-open has an explicit hierarchy and per-page generation pipeline, but relies on file/page/diagram quotas and minimal completion checks. The live DeepWiki Mezon output is reader-friendly, but it indexed an older commit containing strong human-authored architecture docs. CodeWiki must therefore adopt mechanisms, not copy visible formatting or attribute the result to model strength.

A supplied Grok-Wiki export generated with Gemini from the local Mezon repository provides a stronger comparison. Its 11 concept-first pages use small per-topic evidence sets and explain runtime mechanisms with diagrams; source spot checks were consistent with the local checkout. Although its missing commit metadata and export defects prevent treating it as a gold artifact, it demonstrates that the repository and model class can support substantially better onboarding docs. Control-flow inspection then confirmed BUG-001: CodeWiki init/sync writes deterministic semantic summaries as final docs without executing the LLM WikiPlan or reader-synthesis stages promised by the skill.

## Decision

Adopt a reader-first, concept-first information architecture:

1. Canonical pages remain stable landing slots for init, sync, and docs-first Q&A.
2. WikiPlan first records a repository mental model, then creates a hierarchical information architecture, then creates typed per-page contracts. Drafting does not begin from a raw repository snapshot.
3. The runtime boundary is mandatory: the deterministic companion discovers and persists evidence; an LLM creates the mental model, WikiPlan, page drafts, and at most one bounded revision; the deterministic companion then validates, normalizes, safely merges, and records provenance/results. A run that did not execute the required LLM stages cannot report reader-documentation success.
4. WikiPlan may add dynamic topic pages for evidence-backed systems, components, workflows, platform boundaries, and framework concepts. Dynamic pages live in the semantic section that owns the concept; `areas/**` is compatibility-only and is not populated from top-level paths.
5. Dynamic pages are named after semantic concepts, never created solely from a file or top-level path.
6. Deterministic file/symbol/import inventories belong in evidence and planning layers, not as the main body of reader-facing pages. Deterministic summaries may be published only as explicitly incomplete evidence/debug artifacts.
7. Every planned reader-facing page declares its hierarchy and reading order, reader job, audience, questions, scope/non-scope, required sections, relationship/diagram needs, evidence anchors with relevance reasons, related pages, refresh triggers, and acceptance checks.
8. Evidence stays traceable but follows the explanation: purpose and mental model first; claim-local source anchors second; optional source inventory last or in `docs/evidence/**`. This supersedes ADR-0005's default that every source-backed page begins with a `<details>` source-file list.
9. File count, source count, page count, symbol count, and diagram count are not accepted as proxies for coverage or quality. Evidence and diagrams must be relevant to a named reader question.
10. Draft pages pass static, evidence, diagram, cross-page ownership/overlap, artifact-hygiene, and docs-only onboarding evaluation. A failed draft receives at most one bounded revision with named gaps before the run reports incomplete quality.
11. Benchmark comparisons pin repository commit, dirty state, visible existing docs, evidence scope, generator contract, model/provider metadata, and export-normalization results.

This decision refines ADR-0005; it does not supersede the canonical docs root, ownership model, or sync-safety rules.

## Alternatives Considered

- Improve prompts only: rejected because the deterministic generator, WikiPlan schema, and tests would continue rewarding raw inventories.
- Keep deterministic reader-page generation as a fallback when no LLM runs: rejected because it silently violates the skill/product contract and labels evidence summaries as onboarding docs. The fallback may create evidence/debug artifacts but must report synthesis incomplete.
- Generate one page per package or directory: rejected because repository layout does not reliably express runtime or domain boundaries.
- Let every model invent an unconstrained wiki tree: rejected because stable anchors, sync ownership, and cross-model durability would degrade.
- Require a fixed page count and fixed template: rejected because repository complexity and reader needs vary.
- Keep evidence-heavy pages and add a better Quickstart: rejected because onboarding questions cross architecture, workflows, state, API, operations, and testing.
- Copy deepwiki-open's five-file minimum and extensive Mermaid policy: rejected because quotas reward irrelevant evidence and decorative diagrams.
- Treat the live DeepWiki Mezon output as a controlled benchmark: rejected because it indexed a different commit with stronger human-authored input docs.

## Consequences

- Positive: generated docs optimize for developer comprehension and change safety while retaining evidence traceability.
- Positive: output quality becomes more model-independent because planning and evaluation contracts are explicit.
- Positive: canonical anchors and dynamic topic pages can coexist.
- Negative: init and sync require an additional synthesis/evaluation loop and may use more model/runtime budget.
- Negative: companion CLI callers that currently expect self-contained final docs need an explicit orchestration/result contract or must accept evidence-only incomplete output.
- Negative: WikiPlan serialization and generated-page tests require a compatibility update.
- Negative: fair benchmarks require pinned source fixtures and cannot rely only on live third-party output.
- Neutral: raw semantic evidence remains useful but moves to its intended evidence/control-plane role.
- Neutral: ADR-0005 remains accepted for storage/ownership/canonical slots, while its source-list-first rule and path-derived `areas/**` generation are superseded by this decision after approval.

## Linked Work

- Research: `docs/work/research/READER-FIRST-DOCS-AUDIT.md`, `docs/work/research/REFERENCE-DOCS-QUALITY-RESEARCH.md`, `docs/work/research/GROK-WIKI-MEZON-AUDIT.md`
- Bug: `docs/work/bugs/BUG-001-companion-bypasses-reader-synthesis.md`
- Phase: `docs/work/phases/PHASE-002-reader-first-docs-quality.md`
- Tickets: `TICKET-029`, `TICKET-030`, `TICKET-031`
- Related ADR: `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
