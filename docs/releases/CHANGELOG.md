---
artifact_type: changelog
id: CHANGELOG
status: active
owner: shared
human_fields: [release_approval]
ai_fields: [change_entries, linked_releases]
shared_fields: [status]
---

# Changelog

## Field Ownership

- Human owns release approval.
- AI maintains change entries and links to release notes.

All notable changes should be recorded here.

## [Unreleased]

### Fixed

- Released CodeWiki package `0.2.1` with a mandatory first-write preflight and final-validation execution gate after a verified `0.2.0` agent run generated reader Markdown without invoking init, WikiPlan, quality, or validation.

### Changed

- Split generation into deterministic evidence collection, required model mental-model/WikiPlan/page synthesis, and deterministic/semantic quality gates; companion init/sync now stop at `synthesis_incomplete` instead of publishing snapshot summaries as reader docs.
- Upgraded committed planning to WikiPlan v2 with repository mental-model and typed reader/evidence/diagram/refresh/acceptance page contracts; path-derived `areas/**` generation is removed.
- Added explicit v1-plan preservation/enrichment, legacy-control-plane rejection, and plan integrity checks for hierarchy, unique topic ownership, prerequisites, cycles, synthesized coverage, and unplanned reader pages.
- Capture Git source provenance before repo-local initialization writes control files, preventing clean repositories from being reported as dirty by CodeWiki's own output.
- Changed sync ownership semantics so manual edits inside generated regions win: new regions carry portable integrity hashes, unchanged bodies refresh safely, and edited or legacy-unverified bodies are preserved for LLM semantic reconciliation.
- Standardized every CodeWiki-generated Markdown basename as uppercase while keeping wiki directories lowercase, made `docs/QUICKSTART.md` the sole entrypoint, and added marker-aware migration for legacy lowercase generated pages.

### Added

- Added reader-doc validation for source/run provenance, installed-skill identity, model/evaluator metadata, bounded revision count, required reader pages, portable links, duplicate structures, raw inventory patterns, Mermaid fences, cross-page links, and orphan pages.
- Added `codewiki-preflight.sh`, packaged execution-contract regression coverage, and an explicit `$codewiki` UI prompt that names preflight and `reader_docs_ready` validation.
- Added versioned skill package/install manifests, managed-content digest, atomic installer staging, `doctor`, `package-digest`, `validate`, helper preflight, and run-level skill provenance.
- Added reader-first synthesis/diagram/quality references and Mezon/Grok-Wiki regression research covering concept-first planning and export-hygiene failures.
- Added a pinned Mezon Desktop `9d7ba65` benchmark: six reader pages passed docs-only questions, source audit, installed-skill provenance, and deterministic `reader_docs_ready` validation.
- Added evidence-derived project/language/framework convention discovery and canonical `docs/conventions/OVERVIEW.md` generation.
- Initialized repository control harness for CodeWiki.
- Added OpenWiki and deepwiki-open as reference submodules.
- Seeded CodeWiki product requirements, roadmap, foundation phase, validation matrix, and first ADR.
- Recorded the Rust companion-tool decision and reference-submodule strategy.
- Added the initial Rust workspace scaffold and verified `codewiki status`.
- Reframed CodeWiki as a skill-first product, added `skill/codewiki`, and added a repository-based skill installer.
- Documented runtime provider selection policy: Octocode first when needed, codebase-memory-mcp for memory trigger, CocoIndex for indexing trigger.
- Added Rust companion config/storage skeleton for `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, and `.agents/skills/codewiki/project/AGENTS.md`.
- Added reference prompt analysis comparing OpenWiki and DeepWiki patterns for CodeWiki prompt architecture.
- Standardized generated target-repo docs structure: `docs/**` as knowledge surface and `.agents/skills/codewiki/project/**` as committed control plane.
- Added CodeWiki bundled workflow references for docs structure, init, sync, Q&A, and deep research.
- Added the first executor-agnostic SQLite migration for CodeWiki durable local state.
- Added local state/cache path resolution and sqlite3-backed migration application.
- Added `codewiki init [path]` skeleton for target repo CodeWiki files and local SQLite state initialization.
- Added repository detection v1 and wrote detection signals into init plan/index output.
- Added WikiPlan v1 plus confidence, evidence, claim, and planned page models.
- Added canonical starter docs generation for `quickstart`, `source-map`, OpenWiki-style section directories, and `evidence/**`.
- Added `codewiki sync [path]` compare/update/no-op skeleton.
- Added skill-first support design for external/personal wiki workspaces and user-provided non-Git source extension skills.
- Added `.agents/skills/codewiki/project/sources.yml` generation, external workspace initialization support, and a source-skill template for custom non-Git evidence sources.
- Added semantic exploration v1 for bounded file, area, symbol, import/dependency-hint, and evidence-reference snapshots used by generated wiki docs.
- Added durable claim persistence v1 for semantic files, symbols, evidence items, claims, and claim/evidence links in local SQLite.
- Added staleness detection and SQLite-backed Q&A context rendering with active/stale claim separation.
- Added production fixture integration tests for TypeScript, Python, and Rust-shaped repositories.
- Added generated-region markers so sync can preserve human-owned documentation edits.
- Added canonical synthesis pages and area pages generated from semantic evidence.
- Added an installed skill helper script plus copied Rust companion source so CodeWiki skills can invoke deterministic Rust support without making the companion the primary UX.
- Changed the skill installer/helper to prefer a prebuilt installed `bin/codewiki` companion binary, with companion source retained only as fallback.
- Changed the generated docs default from nested `docs/codewiki/**` to direct `docs/**` canonical pages.
- Reworked generated docs layout/content patterns from OpenWiki and DeepWiki: quickstart-first navigation, section directories, no thin pages, backlog-in-quickstart, and relevant-source-files blocks on source-backed pages.
- Recorded current OpenWiki/deepwiki-open reference baseline commits and documented CodeWiki's source-extension status versus OpenWiki-style bundled connectors.
- Closed the foundation phase with reconciled runtime status, phase trace, master docs, validation, README, and final verification evidence.
- Changed the installer default to project-local installation under `.agents/skills/codewiki`; global installation now requires `CODEWIKI_INSTALL_SCOPE=global`.
- Reconciled README, roadmap, traceability, validation, and release-readiness docs for the completed CodeWiki foundation baseline.
- Added CodeWiki-specific standards and reconciled implemented requirement statuses.
