---
artifact_type: research_note
id: READER-FIRST-DOCS-AUDIT
status: in_review
owner: ai
human_fields: [review_notes, acceptance]
ai_fields: [scope, evidence, findings, recommendations]
shared_fields: [status, trace]
trace:
  requirement: REQ-014
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  tickets:
    - docs/work/tickets/TICKET-029-wikiplan-v2-topic-taxonomy.md
    - docs/work/tickets/TICKET-030-reader-first-synthesis-and-diagrams.md
    - docs/work/tickets/TICKET-031-onboarding-quality-evals.md
  adr: docs/decisions/ADR-0010-reader-first-information-architecture.md
  reference_research: docs/work/research/REFERENCE-DOCS-QUALITY-RESEARCH.md
  grok_audit: docs/work/research/GROK-WIKI-MEZON-AUDIT.md
  bug: docs/work/bugs/BUG-001-companion-bypasses-reader-synthesis.md
  distribution_bug: docs/work/bugs/BUG-002-installed-skill-version-drift.md
---

# Reader-First Generated Docs Audit

## Research Question

Why do generated CodeWiki docs remain unsuitable for developer onboarding even when frontier models are used, and which product contracts must change?

## Evidence Reviewed

- `/Users/dungxbuif/Documents/docs.zip`
- `/Users/dungxbuif/Documents/docs-gemini.zip`
- `/Users/dungxbuif/Documents/docs-gpt5.6.zip`
- `/Users/dungxbuif/Documents/docs-qwen.zip`
- Source repository: `/Users/dungxbuif/workspace/mezon-desktop`
- DeepWiki reference: <https://deepwiki.com/mezonai/mezon-desktop>
- Reference implementation research: `docs/work/research/REFERENCE-DOCS-QUALITY-RESEARCH.md`
- Grok-Wiki same-repository artifact audit: `docs/work/research/GROK-WIKI-MEZON-AUDIT.md`
- CodeWiki requirements, ADR-0005, DESIGN-017, WikiPlan models, docs generator, and production fixture assertions.

## Confirmed Findings

1. Model capability is not the primary failure. Gemini and GPT produced nearly the same raw-inventory shape because the deterministic generator and page contract expose the same file/symbol/import snapshot as reader-facing content.
2. `docs.zip` is materially better because it synthesizes responsibilities, dependency direction, and runtime diagrams, but it remains too flat for onboarding and includes non-portable absolute `file://` links.
3. Qwen avoids the largest inventory dump but under-documents the system and omits diagrams and subsystem depth.
4. Top-level filesystem paths are not reliable subsystem boundaries. Generated pages for `Cargo.toml`, `README.md`, `.github`, and `deny.toml` contradict ADR-0005's substantial-area rule.
5. The current WikiPlan implementation does not carry the page-level scope, evidence needs, open questions, reader intent, diagram needs, or refresh strategy promised by the product specification.
6. Current automated tests prove file presence and evidence tokens, not onboarding usefulness, architecture comprehension, change-location guidance, or diagram quality.
7. The current companion does not merely constrain the model with a weak template: `init` and `sync` call a deterministic final-page renderer and never execute the LLM mental-model, WikiPlan, page-synthesis, or revision stages promised by the skill.
8. The supplied Gemini/Grok-Wiki export creates 11 concept pages with bounded 3–6-file evidence sets and mechanism-level diagrams/prose from the local Mezon repository. This materially weakens the hypothesis that source quality or Gemini capability is the primary blocker.
9. The active globally installed CodeWiki skill is stale relative to repository source and no install manifest exposes the drift. This is a separate reproducibility defect that can vary prompt/storage contracts across runs.

## Reference Caveat

DeepWiki indexed commit `b182aed3`, which still contained a strong human-authored `docs/01-architecture-overview.md`. The local test commit removed that directory before regenerating docs. DeepWiki therefore had stronger source documentation, but its concept-first page taxonomy and progressive disclosure remain valid product references.

The open-source deepwiki-open repository is not assumed to be the proprietary DeepWiki implementation. Its code explains useful hierarchy, per-page generation, related-page, citation, cache, and rendering patterns, but the live Mezon wiki has a larger and more specific concept taxonomy than deepwiki-open's fixed 8–12-page comprehensive contract.

The Grok-Wiki archive is closer to a same-source comparison, but its manifest omits the exact source commit and dirty state. It is strong causal evidence, not yet a fully controlled benchmark. It also contains duplicate frontmatter/related sections, non-portable temporary links, and renderer-specific tags, so “better onboarding” must not be confused with “production-clean artifact.”

## Root Cause

CodeWiki currently collapses three layers that must remain separate:

```text
Deterministic evidence inventory
          |
          v
Reader-oriented semantic synthesis
          |
          v
Developer onboarding and change guidance
```

The implementation publishes the first layer as if it were the second, bypasses the documented LLM synthesis stage, and validates presence as if it proved the third.

## Recommended Decision

- Keep canonical pages as stable landing slots.
- Add dynamic concept-first pages for real systems, components, workflows, platform boundaries, and framework concepts.
- Move raw file/symbol/import inventory to `docs/evidence/**` and committed/runtime state.
- Introduce WikiPlan v2 page contracts with explicit reader questions, scope, diagrams, evidence, relationships, refresh triggers, and quality checks.
- Replace the ADR-0005 “source list first” default with purpose/scope first, claim-local citations, and an optional evidence appendix.
- Plan repository mental model and hierarchical page ownership before retrieving per-page evidence or drafting prose.
- Require reader-first synthesis and diagram selection before final docs are accepted.
- Add docs-only onboarding evaluations, including a Mezon Desktop benchmark and at least two additional repository shapes.
- Pin benchmark commits and record which existing docs were visible so human-authored source documentation is not mistaken for model quality.
- Enforce a three-boundary runtime: deterministic discovery/persistence, required LLM planning/synthesis, then deterministic validation/normalization/safe merge.
- Treat a missing synthesis stage as incomplete generation; never silently promote deterministic snapshot summaries to successful reader docs.
- Add artifact-hygiene gates for duplicate frontmatter/sections, unsupported tags, portable links, and source commit/dirty-state provenance.
- Version the skill/reference/companion distribution together and record the resolved installed skill identity in every generation and benchmark result.

## Known Unknowns

- The final semantic scoring threshold needs calibration across multiple repositories.
- Diagram rendering validation may need a lightweight parser or renderer beyond Markdown linting.
- The boundary between deterministic candidate discovery and LLM page planning must remain language-agnostic.
