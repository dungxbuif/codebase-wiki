---
artifact_type: project_context
id: CONTEXT
status: active
owner: shared
human_fields:
  - current_focus
  - open_questions
  - priority_override
ai_fields:
  - recently_touched_areas
  - recent_decisions
  - next_steps
  - queue_summary
shared_fields:
  - current_status
  - active_backlog
  - current_queue_focus
  - active_phase
  - active_ticket
  - active_bug
updated: 2026-07-13
---

# Project Context

## Field Ownership

- Human owns project intent, priority overrides, and unresolved product questions.
- AI owns concise state refreshes after work: touched areas, recent decisions, next steps, and queue summary.
- Shared fields can be updated by either human or AI, but AI must not silently override human priority.

## Current Status

- Status: CodeWiki foundation production baseline is complete and verified.
- Active backlog: `docs/work/BACKLOG.md`
- Current queue focus: foundation baseline complete; future work is release distribution or advanced provider-backed synthesis.
- Active phase: `docs/work/phases/PHASE-001-codewiki-foundation.md`
- Active ticket: None.
- Active bug: None.

## Current Focus

Maintain CodeWiki as a complete repo-native Codex skill for semantic wiki generation, sync, and docs-first Q&A across arbitrary repositories.

## Recently Touched Areas

- `AGENTS.md`
- `README.md`
- `.gitmodules`
- `references/openwiki`
- `references/deepwiki-open`
- `docs/`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`
- `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`
- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- `Cargo.toml`
- `Cargo.lock`
- `crates/codewiki-cli`
- `crates/codewiki-core`
- `crates/codewiki-detect`
- `crates/codewiki-store`
- `crates/codewiki-provider`
- `crates/codewiki-docs`
- `docs/work/tickets/TICKET-001-rust-cli-workspace.md`
- `docs/work/designs/DESIGN-001-rust-cli-workspace.md`
- `skill/codewiki/SKILL.md`
- `skill/codewiki/agents/openai.yaml`
- `scripts/install-codewiki-skill.sh`
- `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
- `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`
- `docs/work/tickets/TICKET-002-skill-first-installer.md`
- `docs/work/designs/DESIGN-002-skill-first-installer.md`
- `crates/codewiki-store`
- `docs/work/tickets/TICKET-003-config-storage-skeleton.md`
- `docs/work/designs/DESIGN-003-config-storage-skeleton.md`
- `docs/work/research/REFERENCE-PROMPTS.md`
- `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
- `skill/codewiki/references/`
- `docs/work/tickets/TICKET-004-sqlite-state-migrations.md`
- `docs/work/designs/DESIGN-004-sqlite-state-migrations.md`
- `crates/codewiki-store/migrations/001_initial_state.sql`
- `docs/work/tickets/TICKET-005-sqlite-executor-paths.md`
- `docs/work/designs/DESIGN-005-sqlite-executor-paths.md`
- `docs/work/tickets/TICKET-006-init-skeleton.md`
- `docs/work/designs/DESIGN-006-init-skeleton.md`
- `docs/work/tickets/TICKET-007-repo-detection-v1.md`
- `docs/work/designs/DESIGN-007-repo-detection-v1.md`
- `docs/work/tickets/TICKET-008-wikiplan-evidence-models.md`
- `docs/work/designs/DESIGN-008-wikiplan-evidence-models.md`
- `docs/work/tickets/TICKET-009-canonical-docs-generator.md`
- `docs/work/designs/DESIGN-009-canonical-docs-generator.md`
- `docs/work/tickets/TICKET-010-sync-skeleton.md`
- `docs/work/designs/DESIGN-010-sync-skeleton.md`
- `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`
- `skill/codewiki/references/workspace-placement.md`
- `skill/codewiki/references/source-extensions.md`
- `skill/codewiki/references/source-skill-template.md`
- `skill/codewiki/references/semantic-exploration.md`
- `crates/codewiki-explore`
- `docs/work/tickets/TICKET-012-semantic-exploration-v1.md`
- `docs/work/designs/DESIGN-012-semantic-exploration-v1.md`
- `docs/work/tickets/TICKET-013-claim-persistence-v1.md`
- `docs/work/designs/DESIGN-013-claim-persistence-v1.md`
- `crates/codewiki-store`
- `docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md`
- `docs/work/designs/DESIGN-014-staleness-qa-retrieval-v1.md`
- `skill/codewiki/references/qa.md`
- `docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md`
- `docs/work/designs/DESIGN-015-production-fixtures-eval-suite.md`
- `crates/codewiki-core/tests/production_fixtures.rs`
- `docs/work/tickets/TICKET-016-sync-safety-generated-regions.md`
- `docs/work/designs/DESIGN-016-sync-safety-generated-regions.md`
- `docs/work/tickets/TICKET-017-synthesis-pages-v1.md`
- `docs/work/designs/DESIGN-017-synthesis-pages-v1.md`
- `skill/codewiki/references/docs-structure.md`
- `docs/work/tickets/TICKET-018-release-readiness.md`
- `docs/work/ROADMAP.md`
- `docs/work/TRACEABILITY.md`
- `README.md`
- `docs/work/tickets/TICKET-019-codewiki-standards-and-status.md`
- `docs/standards/CODEWIKI.md`

## Recent Decisions

- Keep `AGENTS.md` at the repository root for agent discovery and include the local RTK rule.
- Use Harness markdown docs as the project control framework.
- Track OpenWiki and deepwiki-open as reference submodules under `references/`.
- Treat CodeWiki as a complete product, not an MVP.
- Build for all repositories without core language/framework adapters.
- Keep the default tool surface small: Git, filesystem, SQLite, Codex reasoning, and Rust companion helpers.
- Use Octocode as the default first-choice code-intelligence provider when a provider is needed.
- Use codebase-memory-mcp only for shared cross-session memory beyond CodeWiki SQLite state.
- Use CocoIndex only when repo scale or repeated refresh/query workload justifies an indexing pipeline.
- Use docs-first lazy activation: generated docs, `.codewiki` plan/instructions, SQLite evidence, and source/Git are checked before activating external tools.
- Add deterministic defaults for `.codewiki/config.yml`, `.codewiki/plan.yml`, and `.codewiki/AGENTS.md` in the Rust companion store crate.
- Preserve durable state across LLM changes and new sessions via committed config plus local SQLite state.
- Treat the CodeWiki skill as the final product.
- Keep Rust as a companion tool for deterministic local operations, not the primary user experience.
- Use OpenWiki and deepwiki-open as technical references, not as inherited runtime foundations.
- Scaffold the Rust workspace with separate CLI, core, detection, store, provider, and docs crates.
- Verify the scaffold with `rtk cargo fmt --all --check`, `rtk cargo test`, and `rtk cargo run -p codewiki-cli -- status`.
- Add a repository installer script that installs `skill/codewiki` into `$CODEX_HOME/skills/codewiki`.
- Add `origin` remote: `git@github.com:dungxbuif/harness.git`.
- Use OpenWiki as the reference for evidence, write-boundary, docs-first, and sync/no-op discipline.
- Use DeepWiki as the reference for mode-separated prompts, structured RAG context packets, same-language answers, and focused deep research.
- Do not copy either reference prompt wholesale; CodeWiki should use a small skill entry plus mode-specific prompt modules.
- Standardize target-repo generated docs under `docs/codewiki/**`; keep `.codewiki/**` for committed control-plane files.
- Keep `skill/codewiki/SKILL.md` as a compact router and put mode-specific workflow detail in `skill/codewiki/references/`.
- Start SQLite durable state with an executor-agnostic migration registry in `codewiki-store`.
- Resolve local state/cache paths from repository identity and apply migrations through the local `sqlite3` executable.
- Add `codewiki init [path]` to create target `.codewiki/**`, `docs/codewiki/index.md`, and initialize local SQLite state.
- Add repository detection v1 for languages, package managers, framework hints, entrypoints, tests, and docs signals.
- Add typed WikiPlan v1, planned pages, confidence, evidence, and claim models.
- Generate canonical starter docs for index, map, architecture, and evidence pages during init.
- Add `codewiki sync [path]` compare/update/no-op skeleton and ignore generated CodeWiki files during detection.
- Support repo-local and external/personal wiki workspace placement; ask before writing when ambiguous.
- Treat Git as the default source and support non-Git sources only through user-provided source extension skills.
- Write `.codewiki/sources.yml` during initialization with Git as the primary source, including when docs are placed in an external/personal workspace.
- Provide a copyable source-skill template so users can add Jira/Figma/fix-note style sources as separate skills instead of CodeWiki core providers.
- Add semantic exploration v1 as a deterministic companion boundary for bounded file, area, symbol, import/dependency-hint, and evidence-reference snapshots.
- Persist semantic files, symbols, evidence items, promoted claims, and claim/evidence links into local SQLite during init/sync.
- Mark claims stale when supporting file evidence changes and render SQLite-backed Q&A context with active/stale claim separation.
- Verify init, docs, semantic claims, SQLite Q&A context, and stale sync across TypeScript, Python, and Rust-shaped production fixtures.
- Add generated-region markers and sync merge behavior so human-owned text around generated docs is preserved.
- Generate canonical synthesis pages and area pages from semantic evidence while recording gaps explicitly.
- Reconcile release-readiness docs, roadmap, traceability, README, installer syntax, companion status, and validation evidence.
- Finalize CodeWiki-specific product/evidence/sync/tool standards and implemented requirement statuses.

## Next Steps

- Create execution tickets from `PHASE-001`.
- Write detail design for config/storage and repository exploration before implementation.
- Implement prompt module behavior in the actual CodeWiki init/sync/Q&A runtime.
- Add fixture repositories and production-readiness quality tests.
- Optional next work: push to remote, tag/release, or add advanced provider-backed synthesis.
- Build the skill init/sync/Q&A workflows first; use Rust only where deterministic helper behavior is needed.
- Keep source integrations skill-based: no built-in non-Git providers unless a later design explicitly changes that boundary.

## Open Questions

- Exact Rust crate/workspace layout has an initial scaffold for companion tooling; future crate splits may change if skill implementation pressure proves it.
- Octocode is the default provider when code-intelligence is needed; codebase-memory-mcp and CocoIndex remain conditional by trigger.
- Initial Codex skill packaging exists under `skill/codewiki`; marketplace/distribution details beyond direct install remain open.
