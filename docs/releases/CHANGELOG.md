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
