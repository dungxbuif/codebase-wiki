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

- Status: CodeWiki repository initialized with Harness control docs, reference submodules, a verified Rust companion-tool scaffold, and a skill package installer.
- Active backlog: `docs/work/BACKLOG.md`
- Current queue focus: `BL-001` through `BL-005`; `BL-007` and `BL-008` completed
- Active phase: `docs/work/phases/PHASE-001-codewiki-foundation.md`
- Active ticket: None.
- Active bug: None.

## Current Focus

Build CodeWiki as a complete repo-native Codex skill for semantic wiki generation, sync, and docs-first Q&A across arbitrary repositories.

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

## Next Steps

- Create execution tickets from `PHASE-001`.
- Write detail design for config/storage and repository exploration before implementation.
- Implement prompt module behavior in the actual CodeWiki init/sync/Q&A runtime.
- Implement SQLite executor/path resolver for the migration registry.
- Add target-repo `.codewiki/AGENTS.md` writing during CodeWiki init.
- Build the skill init/sync/Q&A workflows first; use Rust only where deterministic helper behavior is needed.
- Design SQLite schema and migrations next.

## Open Questions

- Exact Rust crate/workspace layout has an initial scaffold for companion tooling; future crate splits may change if skill implementation pressure proves it.
- Octocode is the default provider when code-intelligence is needed; codebase-memory-mcp and CocoIndex remain conditional by trigger.
- Initial Codex skill packaging exists under `skill/codewiki`; marketplace/distribution details beyond direct install remain open.
