---
artifact_type: research_note
id: GROK-WIKI-MEZON-AUDIT
status: in_review
owner: ai
human_fields: [review_notes, acceptance]
ai_fields: [scope, provenance, reproduction, analysis, findings, recommendations]
shared_fields: [status, trace]
trace:
  requirement: REQ-014
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  bug: docs/work/bugs/BUG-001-companion-bypasses-reader-synthesis.md
  audit: docs/work/research/READER-FIRST-DOCS-AUDIT.md
  reference_research: docs/work/research/REFERENCE-DOCS-QUALITY-RESEARCH.md
  adr: docs/decisions/ADR-0010-reader-first-information-architecture.md
  source_artifact: /Users/dungxbuif/Downloads/wiki-local-mezon-desktop-d2680e1538f5-wiki.zip
  source_repository: /Users/dungxbuif/workspace/mezon-desktop
  product_reference: https://grok-wiki.com/
---

# Grok-Wiki Mezon Desktop Output Audit

## Research Question

Does a Gemini-generated Grok-Wiki output from the same Mezon Desktop repository show that CodeWiki's onboarding failure is primarily caused by the repository/model, or by CodeWiki's planning, synthesis, and quality-control contracts?

## Artifact Provenance And Limits

The supplied archive exported an Obsidian vault at `2026-07-14T02:44:32.315Z`; its manifest says generation completed at `2026-07-14T02:43:57.804Z`. It identifies the repository as `local/mezon-desktop`, but records `branch: null` and no source commit.

At audit time, `/Users/dungxbuif/workspace/mezon-desktop` was at `9d7ba654830c0e6278d0eb413a0eb9a992f01a55` with only an untracked `.codewiki/` directory. The archive's source paths and checked claims match that checkout. This is strong same-repository evidence, but not a cryptographically controlled same-commit benchmark. The `d2680e1538f5` archive-name suffix is not accepted as a Git revision.

Grok-Wiki's current product description independently documents this workflow: read the repository, build an outline from evidence, write pages, then attach exact file/line sources. It also states that the selected local CLI agent performs the model work while Grok-Wiki supplies durable structure and citations. These are product claims, not proof that every exported artifact satisfies them.

## Reproduction And Measurements

The archive was extracted read-only under `/private/tmp/codewiki-grok-audit-d2680e/` and inspected as Markdown plus `manifest.json`.

| Measure | Observed result |
| --- | --- |
| Reader topic pages | 11 |
| Page words | 15,956 |
| Page lines | 2,890 |
| Mermaid diagrams | 11 across 9 pages; authentication contains 2 |
| Planned source files per page | 3–6, usually 4–6 |
| Duplicate frontmatter | Present on all 11 topic pages |
| Duplicate `Related pages` section | Present on all 11 topic pages |
| Non-portable absolute `file://` links | Present in at least 7 topic pages |
| Source commit in manifest | Missing |

The page tree is concept-first and ordered: overview, setup, development workflow, state, network/API, theme, authentication, native OS integration, voice/audio/video, configuration, and build/package/update. No page is named after `Cargo.toml`, `.github`, or an arbitrary top-level directory.

## What Is Materially Better

1. **The model receives a page job, not a repository dump.** Each manifest page has a title, a reader-facing description, and a small source set selected for that topic.
2. **Pages synthesize mechanisms.** The network page explains the dedicated Tokio runtime, transport choices, framing, reachability, reconnect behavior, realtime events, and upload paths as one system. The authentication page connects login variants, session persistence, refresh, socket credentials, and logout cleanup.
3. **Diagrams answer local questions.** Architecture/flow diagrams appear next to the mechanism they clarify instead of being required as a global count.
4. **Implementation detail follows a recognizable concept.** Symbols and files support the explanation; they do not become the information architecture.
5. **The result is deep enough to support onboarding conversations.** Most pages are roughly 1,000–2,000 words and include boundaries, operational detail, failure behavior, tables, and code mapping.

## Source-Grounding Spot Checks

The following high-information claims were checked against the current Mezon source checkout and were consistent with the inspected implementation:

- `crates/mezon-client/src/transport_runtime.rs` creates the dedicated transport runtime with two worker threads.
- `crates/mezon-client/src/app_api.rs` contains the bounded fetch/upload behavior and multipart thresholds described by the network page.
- `crates/mezon-client/src/abridged_tcp_adapter.rs` implements the custom abridged TCP framing and timeout behavior described by the network page.
- `crates/mezon-ui/src/auth/login_view.rs`, `crates/mezon-client/src/auth.rs`, `crates/mezon-client/src/session.rs`, and `crates/mezon-store/src/login.rs` support the described QR/OTP/OAuth/session/logout flow.
- `crates/mezon-native/src/instance.rs`, `deep_link.rs`, `tray.rs`, `badge.rs`, `notifications.rs`, `autostart.rs`, and `power.rs` support the native-integration page's subsystem breakdown.

These spot checks do not certify every claim. They demonstrate that the improved reader experience was not achieved by replacing source grounding with generic prose.

## Quality Defects In The Grok Export

The artifact is a useful benchmark, not a template to copy verbatim:

- Missing commit/dirty-state provenance prevents a controlled regression comparison.
- Absolute temporary-workspace `file://` URLs break after export.
- Duplicate YAML frontmatter and duplicate related-page sections indicate a composition/export merge defect.
- `<CardGroup>`, `<Info>`, `<Steps>`, and `<ResponseField>` are renderer-specific constructs that are not portable Markdown/Obsidian contracts.
- Source links are frequently file- or symbol-level rather than a normalized repository permalink contract.
- Coverage is strong but incomplete for first-day onboarding: application bootstrap/lifecycle, the full UI shell/chat composition, and a product/domain glossary do not have clear canonical homes.

CodeWiki should adopt the planning and synthesis mechanisms while exceeding this artifact on provenance, portability, page ownership, and validation.

## Root-Cause Comparison With CodeWiki

The current CodeWiki `init` and `sync` paths call `render_semantic_pages` directly. That deterministic renderer:

- creates every canonical reader page whether its reader job is evidence-backed or not;
- creates an `areas/**` page for every detected filesystem area;
- prepends a source-file details block to every non-evidence page;
- selects sources using filename/role substring rules and pads the list to at least five files;
- writes the entire semantic snapshot into Quickstart/Source Map/Architecture content;
- does not invoke an LLM WikiPlan or LLM reader-page synthesis stage.

Therefore the frontier model used outside the companion cannot repair the dominant output shape. The companion has already decided the page tree, evidence allocation, reading order, and final prose before the model can perform reader-oriented synthesis.

## Compounding Skill Distribution Drift

The active global skill under `/Users/dungxbuif/.codex/skills/codewiki` also differs materially from repository source `skill/codewiki`. The installed copy still uses `.codewiki/**`, lowercase generated filenames, and an older reference set; it has no conventions reference. The installer records no revision, digest, or skill/companion compatibility versions, so archived runs cannot prove which instruction contract executed.

This drift can worsen or vary output, but it is not the root cause of the deterministic companion's behavior: the current repository companion itself still bypasses LLM synthesis. The two defects require separate fixes—BUG-001 for orchestration and BUG-002 for distribution integrity.

## Debugging Hypothesis

I think the companion's deterministic final-page renderer causes the onboarding-quality failure because same-repository Gemini output becomes substantially better when an external pipeline gives the model a concept-first outline and bounded per-page evidence, while CodeWiki directly publishes snapshot-derived pages without executing its documented LLM planning and synthesis contract.

## Decision Impact

This evidence strengthens ADR-0010 from a presentation decision into an orchestration-boundary decision:

```text
Deterministic companion
discover -> persist evidence -> validate inputs
                    |
                    v
Required LLM stage
mental model -> WikiPlan -> page synthesis -> bounded revision
                    |
                    v
Deterministic companion
validate -> normalize -> merge safely -> persist provenance/results
```

The deterministic companion may produce evidence/debug views and safe scaffolding. It must not label snapshot summaries as complete reader documentation or silently succeed when the required model synthesis stage did not run.

## Conclusion

The new artifact materially reduces the likelihood that Mezon Desktop source quality or Gemini capability is the primary cause. CodeWiki has two product-control failures: the companion publishes deterministic evidence summaries instead of executing the skill's LLM synthesis contract, and the installed skill/companion identity can drift without detection. Fixing prompts alone cannot close either gap.
