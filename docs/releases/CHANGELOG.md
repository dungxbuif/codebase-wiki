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

### Added

- Initialized repository control harness for CodeWiki.
- Added OpenWiki and deepwiki-open as reference submodules.
- Seeded CodeWiki product requirements, roadmap, foundation phase, validation matrix, and first ADR.
- Recorded the Rust companion-tool decision and reference-submodule strategy.
- Added the initial Rust workspace scaffold and verified `codewiki status`.
- Reframed CodeWiki as a skill-first product, added `skill/codewiki`, and added a repository-based skill installer.
- Documented runtime provider selection policy: Octocode first when needed, codebase-memory-mcp for memory trigger, CocoIndex for indexing trigger.
- Added Rust companion config/storage skeleton for `.codewiki/config.yml`, `.codewiki/plan.yml`, and `.codewiki/AGENTS.md`.
- Added reference prompt analysis comparing OpenWiki and DeepWiki patterns for CodeWiki prompt architecture.
- Standardized generated target-repo docs structure: `docs/codewiki/**` as knowledge surface and `.codewiki/**` as committed control plane.
- Added CodeWiki bundled workflow references for docs structure, init, sync, Q&A, and deep research.
- Added the first executor-agnostic SQLite migration for CodeWiki durable local state.
- Added local state/cache path resolution and sqlite3-backed migration application.
- Added `codewiki init [path]` skeleton for target repo CodeWiki files and local SQLite state initialization.
- Added repository detection v1 and wrote detection signals into init plan/index output.
- Added WikiPlan v1 plus confidence, evidence, claim, and planned page models.
- Added canonical starter docs generation for `index`, `map`, `architecture`, and `evidence/**`.
- Added `codewiki sync [path]` compare/update/no-op skeleton.
- Added skill-first support design for external/personal wiki workspaces and user-provided non-Git source extension skills.
- Added `.codewiki/sources.yml` generation, external workspace initialization support, and a source-skill template for custom non-Git evidence sources.
- Added semantic exploration v1 for bounded file, area, symbol, import/dependency-hint, and evidence-reference snapshots used by generated wiki docs.
- Added durable claim persistence v1 for semantic files, symbols, evidence items, claims, and claim/evidence links in local SQLite.
- Added staleness detection and SQLite-backed Q&A context rendering with active/stale claim separation.
- Added production fixture integration tests for TypeScript, Python, and Rust-shaped repositories.
- Added generated-region markers so sync can preserve human-owned documentation edits.
- Added canonical synthesis pages and area pages generated from semantic evidence.
