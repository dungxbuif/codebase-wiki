---
artifact_type: research_note
id: REFERENCE-DOCS-QUALITY-RESEARCH
status: in_review
owner: ai
human_fields: [review_notes, acceptance]
ai_fields: [scope, baselines, analysis, findings, recommendations]
shared_fields: [status, trace]
trace:
  requirement: REQ-014
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  audit: docs/work/research/READER-FIRST-DOCS-AUDIT.md
  adr: docs/decisions/ADR-0010-reader-first-information-architecture.md
  references:
    - references/openwiki
    - references/deepwiki-open
    - https://deepwiki.com/mezonai/mezon-desktop
    - https://grok-wiki.com/
  grok_audit: docs/work/research/GROK-WIKI-MEZON-AUDIT.md
---

# Reference Documentation Quality Research

## Research Questions

1. Which mechanisms make a generated wiki easy for a new developer to navigate?
2. Which mechanisms actually prove that the wiki is accurate and useful for onboarding?
3. Which reference patterns should CodeWiki adopt, adapt, or reject?
4. Which existing CodeWiki decisions need refinement rather than a stronger model or a longer prompt?

## Baselines And Limits

| Reference | Inspected baseline | What was inspected | Important limit |
| --- | --- | --- | --- |
| OpenWiki | `2fb44a876db8cca461ad1c0767931d95495763a3`; compared with `origin/main` at `7c084f9` | Prompt, run orchestration, update/no-op tests, generated `openwiki/**` docs | Quality is mainly prompt-enforced; its own generated docs are not an onboarding benchmark. |
| deepwiki-open | `16f35a0fc0284e99b7963bbf4e8585e9957e2fe1` | Structure prompt, page prompt, XML parsing, generation queue, cache, tree UI, Markdown/Mermaid renderer, tests | This open-source project is not evidence of the proprietary DeepWiki pipeline. |
| deepwiki-open experimental branches | `challenge/best-wiki-prompt` at `c21937a`; `actually-awesome-wiki` at `3a55dd3` | Prompt and hierarchy experiments | Unmerged experiments reveal known problems but are not production contracts. |
| DeepWiki Mezon | <https://deepwiki.com/mezonai/mezon-desktop> | Reader-visible hierarchy, page taxonomy, diagrams, citations, reading flow | Indexed commit `b182aed3` contained strong human-authored docs absent from local HEAD `9d7ba65`; comparison is not source-equivalent. |
| Grok-Wiki Mezon export | Supplied Obsidian export generated 2026-07-14 with Gemini | Manifest, 11 pages, evidence allocation, diagrams, citations, artifact hygiene, source claim spot checks | Manifest omits source commit/dirty state, so same-repository evidence is strong but exact source equivalence is not proven. |
| CodeWiki trials | Four supplied archives generated from `/Users/dungxbuif/workspace/mezon-desktop` | Page trees, Quickstart, Source Map, Architecture, evidence placement, links, diagrams | Model prompts and commits differed; conclusions focus on repeated structural failures, not model ranking. |

Research conclusions distinguish observed code behavior from inference. The live DeepWiki site is treated as an output reference only; deepwiki-open is not assumed to implement the same system.

## OpenWiki: What It Actually Contributes

### Strong controls

- A temporary page/evidence plan is required before final writing.
- Initial docs start from a Quickstart and a small set of substantial pages rather than one page per source path.
- Pages must explain what an area does, why it exists, where to start, what to watch, and which checks matter when changing it.
- Existing docs are primary evidence and Git history is used selectively to recover intent.
- Update runs build a source-change-to-doc-impact plan, preserve accurate text, use a soft diff budget, and may be a deterministic no-op.
- Canonical ownership and anti-duplication rules improve maintainability across repeated runs.
- The runtime snapshots generated content and writes update metadata only when content changed.

### Missing quality proof

- Planning is temporary and untyped. The runtime does not validate that every page has a reader job, unique scope, required relationship, or complete evidence packet.
- The agent writes final pages directly; there is no explicit draft-review-revision pipeline.
- Tests assert prompt instructions and update/no-op behavior, not architecture comprehension or onboarding task completion.
- The repository's own generated architecture page is clear prose but contains no architecture diagram. The Quickstart still ends with long key-file and source-map lists.
- No cross-page overlap score, diagram validity gate, docs-only question benchmark, or human onboarding UAT was found.

OpenWiki is therefore the strongest reference for safe exploration and surgical maintenance, not for proving onboarding quality.

## deepwiki-open: What Produces The Reader Experience

### Two-stage generation

The open-source pipeline first asks for an XML wiki structure from the complete file tree and README. A planned page carries title, description, importance, relevant files, related pages, and parent section. Comprehensive mode adds sections and subsections. It then generates each page separately through repository RAG and renders the hierarchy in a tree UI.

This separation is a major improvement over publishing one repository snapshot into every canonical page. It gives each page a topic, a place in the hierarchy, a bounded source context, and related-page awareness.

### Presentation mechanisms worth adopting

- Concept-oriented page titles and hierarchical navigation.
- Per-page descriptions, importance, parent section, related pages, and selected source anchors.
- Separate page generation after the information architecture is known.
- Tables and diagrams chosen for component relationships, flows, state, and data.
- A renderer that supports Mermaid, zoom/pan, and visible syntax errors.
- Inline source/line citations close to the claims they support.

### Quality shortcuts to reject

- Structure planning sees only the file tree and README, so it can miss runtime semantics and inherit stale README assumptions.
- Comprehensive mode suggests a generic fixed taxonomy and fixes output at 8–12 pages. Repository complexity, not a page quota, should determine coverage.
- The page prompt requires at least five cited files and experimental prompts raise this to 8–10. Relevance can be diluted to satisfy the quota.
- The prompt says to use Mermaid extensively. Diagram count can increase without information gain.
- Page content is stored after minimal delimiter cleanup. No claim audit, citation validation, overlap review, page-contract validation, or bounded revision was found.
- Cache readiness checks only that every page has non-placeholder content. An `Error generating content: ...` string qualifies as content and can therefore satisfy the cache condition.
- Mermaid syntax failures are shown by the renderer, but no generation retry or wiki-level failure gate was found.
- Test coverage focuses on backend/provider/embedding behavior; no generated-wiki quality test was found.

Experimental branches explicitly add related-page context and “minimal overlap” instructions. That is evidence that page duplication is a known prompt-level problem, not evidence that it has been solved by evaluation.

## Live DeepWiki Mezon: What The Output Demonstrates

The live wiki uses a reader-oriented taxonomy: project overview and getting started first, followed by workspace/layout, bootstrap and lifecycle, authentication, native integration, network transport, state management, UI architecture, framework/platform concepts, protocol/assets/localization, conventions, testing, and glossary pages. This is materially easier to scan than pages named after `Cargo.toml`, `.github`, or raw top-level directories.

Its useful characteristics are:

- progressive disclosure from system overview to components and workflows;
- one recognizable home for major concepts;
- diagrams and tables embedded near the explanation they clarify;
- source citations following claims;
- related pages that form reading paths.

However, the result cannot be attributed only to a superior generation pipeline. DeepWiki indexed `b182aed3`, whose `docs/01-architecture-overview.md` already supplied a technology table, crate dependency map, directory responsibilities, platform matrix, transport comparison, project status, and onboarding analogies. The local benchmark HEAD `9d7ba65` had removed that docs directory before CodeWiki generation.

A fair benchmark must pin the same source commit and source visibility. Human docs present in one run but absent in another are a benchmark variable, not a model-quality result.

## Grok-Wiki Mezon Export: The Stronger Controlled Signal

The user generated a second reference from the local Mezon Desktop repository through Grok-Wiki with Gemini. The export contains 11 concept-oriented pages, 15,956 words, 11 Mermaid diagrams, and a manifest assigning 3–6 relevant files to each page. High-information network, authentication, state, and native-integration claims were consistent with spot checks against local HEAD `9d7ba65`.

This artifact changes the diagnosis. Unlike the live DeepWiki comparison, it does not depend on the older human-authored `docs/01-architecture-overview.md`. Its missing commit metadata still prevents a strict benchmark claim, but it demonstrates that the current source repository contains enough evidence for Gemini to synthesize a useful developer reference when the pipeline gives it a concept-first outline and bounded per-page source context.

The artifact also has serious export defects: duplicate frontmatter and related-page sections, temporary absolute `file://` links, renderer-specific MDX tags, no source revision, and incomplete canonical coverage of bootstrap/UI/domain topics. CodeWiki should use it as a mechanism and quality-ceiling reference, not a format template. See `docs/work/research/GROK-WIKI-MEZON-AUDIT.md`.

The official Grok-Wiki product workflow matches the observed strength: repository reading, outline construction from evidence, page writing, and exact source attachment are separate stages. It explicitly positions the local CLI agent as the reasoning/writing engine and Grok-Wiki as the durable context, citation, and workspace layer. CodeWiki's current companion path does not execute an equivalent model-planning or model-synthesis stage.

## Cross-Reference Decision Matrix

| Mechanism | OpenWiki | deepwiki-open | Live DeepWiki | Grok-Wiki export | CodeWiki decision |
| --- | --- | --- | --- | --- | --- |
| Targeted evidence discovery | Strong | RAG after tree/README plan | Not observable | 3–6 manifest sources per topic | Adopt bounded discovery and persist selected evidence plus relevance. |
| Hierarchical information architecture | Prompt-only, shallow | Explicit sections/subsections | Strong | Ordered concept tree | Adopt typed hierarchy in WikiPlan. |
| Per-page contract | Informal prose rules | Description, importance, files, relations, parent | Strong output | Title, description, selected files; rich output | Extend with reader job, questions, scope, evidence rationale, diagrams, refresh, acceptance. |
| Concept-first naming | Required by prompt | Encouraged | Strong | Strong | Enforce semantic topic qualification; paths are evidence, not topics. |
| Evidence presentation | Source maps often accumulate | Source list first plus inline citations | Inline citations visible | Claim-local links plus manifest list, but non-portable | Cite claims inline; source inventory belongs at the end or in evidence pages, not before the mental model. |
| Diagram strategy | No executable gate | “Extensively use Mermaid” | High visual value | 11 mechanism-local diagrams | Use question-triggered diagram slots and validate information gain/renderability. |
| Cross-page uniqueness | Canonical-home prompt | Related-page prompt; experimental anti-overlap | Strong navigation | Strong topics, but duplicate export sections | Add ownership map, normalization, and overlap review before acceptance. |
| Incremental sync | Strong, surgical, no-op | Full cache refresh | Not observable | Not evaluated | Keep CodeWiki/OpenWiki-style impact planning and ownership safety. |
| Quality evaluation | Not found | Not found | Not observable | Export defects escaped | CodeWiki must add static, evidence, semantic, docs-only, artifact-hygiene, and human gates. |

## Corrections To Existing CodeWiki Decisions

### Keep

- ADR-0005's separation of committed docs, committed control plane, durable local state, and rebuildable cache.
- Stable canonical landing pages for init, sync, and docs-first Q&A.
- Evidence traceability, generated-region ownership, and human-edit preservation.
- Language/framework detection from evidence rather than core adapters.

### Refine or supersede

1. “Source-backed pages should begin with a `<details>` source list” is the wrong default. It copies a DeepWiki presentation pattern that places implementation evidence before the reader's mental model. Pages should begin with purpose and scope; claim-local citations and an optional evidence appendix remain required.
2. `areas/<path>/OVERVIEW.md` must not be populated from top-level filesystem areas. Dynamic pages should live in the semantic section that owns the concept, such as `architecture/TRANSPORT.md`, `workflows/AUTHENTICATION.md`, or `components/STATE-MANAGEMENT.md`. `areas/**` remains a compatibility slot, not a generation target.
3. Canonical filenames alone do not define quality. Each planned file needs a typed, page-specific content contract and observable acceptance checks.
4. File count, citation count, page count, symbol count, and diagram count are not quality metrics. Use relevance, coverage of reader questions, ownership clarity, evidence sufficiency, and onboarding task success.
5. Prompt-only rules are insufficient. The generation pipeline needs explicit planning, drafting, static validation, semantic/docs-only evaluation, cross-page review, and one bounded revision.
6. “Skill is the product; Rust is a companion” is currently aspirational rather than enforced. The companion may collect/persist evidence and validate/merge output, but a successful reader-doc run must execute the LLM mental-model, WikiPlan, and page-synthesis stages. Deterministic snapshot summaries are evidence/debug artifacts, not final wiki prose.
7. Quality comparisons must pin the installed skill/companion artifact, not only repository source. The currently active global skill differs from `skill/codewiki/**`, while the installer records no revision/digest/compatibility manifest.

## Required CodeWiki Pipeline

```text
Bounded repository evidence
          |
          v
Repository mental model
(systems, actors, boundaries, workflows, state, risks)
          |
          v
Hierarchical WikiPlan
(reader jobs, unique ownership, reading order, page contracts)
          |
          v
Per-page evidence packets
(relevant anchors and why each is needed)
          |
          v
Reader-first drafts
          |
          v
Static + evidence + diagram + overlap checks
          |
          v
Docs-only onboarding evaluation
          |
          +--> pass: write/reconcile generated regions
          |
          +--> fail: one bounded revision with named gaps
```

The repository mental model and page plan must be durable enough to survive session/model changes. Draft text may be transient; page ownership, reader questions, evidence anchors, open questions, and refresh triggers may not be.

## Benchmark Protocol

Every comparison run must record:

- exact repository commit and dirty state;
- which existing docs were visible;
- generator/version/model/provider and prompt-contract version;
- resolved installed skill root, package/content digest, reference contract, and companion compatibility version;
- included/excluded paths and evidence manifest;
- generated page tree and page-contract coverage;
- static/evidence/diagram/overlap results;
- docs-only onboarding answers and required concept/evidence matches;
- source fallbacks, critical misconceptions, and human UAT result.
- export hygiene: portable links, exactly one frontmatter block, exactly one related-page block, supported Markdown components, and source revision metadata.

Reference outputs must not be exposed to the generator under test. Scores may diagnose weak pages, but a critical architecture, ownership, security, or change-location error cannot be averaged away.

## Recommendation

Use OpenWiki as the exploration, write-boundary, and incremental-maintenance reference. Use deepwiki-open as the inspectable hierarchy, per-page generation, related-page, citation, and renderer reference. Use the live DeepWiki Mezon wiki as a reader-experience benchmark with an explicit source-commit caveat. Use the Grok-Wiki export as evidence that the same repository/model class can produce substantially stronger pages when outline and synthesis are real pipeline stages, while explicitly rejecting its provenance and export defects. CodeWiki's differentiator must be durable typed page contracts, an enforced model-synthesis boundary, and onboarding-quality evaluation with bounded revision.
