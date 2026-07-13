---
name: codewiki
description: Use when a user wants to generate, sync, inspect, or query a semantic wiki for a software repository or personal/external code knowledge workspace. CodeWiki explores source code, detects languages/frameworks/libraries dynamically, builds an evidence-backed WikiPlan, writes durable docs either inside the repo or in a user-confirmed external workspace, preserves reusable state across sessions/models, supports Git as the default source, and can route non-Git sources through user-provided source extension skills.
---

# CodeWiki

CodeWiki is a repo-native skill for creating and maintaining a semantic wiki for any software repository.

Use this skill when the user asks to:

- initialize or update a codebase wiki;
- create personal or external docs for a repo without writing into the source repo;
- understand a repository through generated docs;
- generate a WikiPlan from source code;
- sync docs after code changes;
- incorporate non-Git change sources through user-provided source skills;
- answer architecture/codebase questions from docs and evidence;
- preserve repo understanding across sessions or model changes.

## Core Rules

- The skill is the product. Rust code in this repo is a companion tool, not the primary user experience.
- Work inside the target repository, not inside the CodeWiki source repo, unless the user is developing CodeWiki itself.
- Do not require approval during `init`; the agent should explore source code autonomously and produce a WikiPlan.
- Do not build core language/framework adapters. Detect repository stack signals dynamically.
- Keep claims evidence-backed. Every durable claim should trace to files, symbols, commands, existing docs, or explicit hypotheses.
- Keep committed project config/docs separate from local persistent state and rebuildable cache.
- Git is the default source for code changes. Do not bundle Jira/Figma/etc. providers in CodeWiki core.
- If the output location is ambiguous, confirm whether to write docs in the source repo or in an external/personal workspace before writing files.
- Support non-Git sources through `.codewiki/sources.yml` and user-provided source extension skills.
- Do not install every provider by default. Use Octocode as the first-choice code-intelligence provider when filesystem/Git exploration is insufficient; use codebase-memory-mcp and CocoIndex only under their specific triggers.

## Reference Loading

Keep this file as the compact router. Load bundled references only when needed:

- `references/docs-structure.md`: load for every init/sync run and when repairing or explaining the wiki layout.
- `references/workspace-placement.md`: load before writing files when repo-local vs external/personal wiki placement is not explicit.
- `references/source-extensions.md`: load when adding or using non-Git evidence/change sources.
- `references/source-skill-template.md`: load when the user wants to create a custom source skill.
- `references/semantic-exploration.md`: load during init/sync before writing docs, and when Q&A requires source evidence beyond generated docs.
- `references/init.md`: load when initializing or generating a new CodeWiki.
- `references/sync.md`: load when updating, refreshing, or reconciling an existing CodeWiki.
- `references/qa.md`: load when answering questions from existing CodeWiki docs/state.
- `references/deep-research.md`: load for broad, risky, weakly documented, or semantically complex questions.

## Target Repository Layout

When initializing a target repository, prefer:

```text
.codewiki/
  config.yml
  plan.yml
  AGENTS.md
  sources.yml
docs/
  codewiki/
    index.md
    map.md
    architecture.md
    domains.md
    workflows.md
    data.md
    interfaces.md
    operations.md
    testing.md
    decisions.md
    glossary.md
    open-questions.md
    evidence/
      README.md
      sources.md
      commands.md
      claims.md
    areas/
      <area-slug>.md
```

`docs/codewiki/index.md` is required after a successful init. Other pages are canonical slots, not mandatory stubs: create them only when the repository has enough evidence-backed content. Use `areas/<area-slug>.md` only for substantial areas that deserve deeper treatment.

In external/personal workspace mode, this same structure lives in the chosen workspace directory, while the source repository remains read-only evidence unless the user asks otherwise.

Local runtime state should live outside the repo in platform app data and use SQLite, keyed by repository/workspace identity. Rebuildable cache should be separate from durable state.

## Runtime Tool Selection

Default exploration uses Git, filesystem access, Codex reasoning, and optional Rust companion helpers.

When default exploration is not enough, select tools in this order:

1. **Octocode**: default first-choice provider for semantic code intelligence. Use when symbol/dependency/repository-structure search would materially improve exploration quality.
2. **codebase-memory-mcp**: use only when the target repo needs shared, durable, cross-session agent memory beyond CodeWiki's own SQLite facts/evidence/state.
3. **CocoIndex**: use only when the target repo is large enough, or refresh/query workload repetitive enough, to justify an indexing pipeline.

Do not run all three by default. Choose the smallest tool set that satisfies the current repository and workflow.

Do not vendor these tools into the skill. Prefer target-repo runtime setup:

1. Record the selected tool and trigger reason in `.codewiki/config.yml`.
2. Add or update `.codewiki/AGENTS.md` with local install/activation notes for that repository.
3. Install or activate the tool in the target repo only after the trigger is met.
4. Record the tool version/config/evidence source in local runtime state.

If no optional tool is available, continue with Git/filesystem exploration and mark reduced confidence where appropriate.

## Docs-First Activation

After CodeWiki has generated docs, answer questions in this order:

1. `docs/codewiki/**`
2. `.codewiki/plan.yml`
3. `.codewiki/AGENTS.md`
4. local SQLite facts/evidence/claims
5. source files and Git history
6. external runtime tools, only when the earlier layers are stale, insufficient, or the user asks for graph/index/memory-heavy analysis

This means ordinary Q&A about documented architecture should not activate Octocode, codebase-memory-mcp, or CocoIndex. Activate them lazily when evidence quality requires it.

## Init Workflow

1. Resolve repository identity from Git and filesystem context.
2. Detect language, package manager, framework/library, entrypoint, test/build, and docs signals.
3. Inspect existing docs before source where useful.
4. Explore source semantically with bounded file, area, symbol, import/dependency-hint, and evidence snapshots.
5. Build a WikiPlan with pages, scope, evidence needs, confidence, open questions, and refresh strategy.
6. Generate `docs/codewiki/**` from evidence and mark uncertainty explicitly.
7. Write `.codewiki/config.yml` and `.codewiki/plan.yml`.
8. Write `.codewiki/AGENTS.md` with CodeWiki-local instructions, including optional runtime tool status.
9. Record verification commands or skip reasons.

## Sync Workflow

1. Compare Git state, known evidence, generated pages, and changed files.
2. Mark stale pages and stale claims before rewriting.
3. Preserve human-owned edits unless the generated region is explicit.
4. Refresh evidence and update pages.
5. Record what changed and why.

## Q&A Workflow

1. Answer from `docs/codewiki/**` first.
2. If docs are insufficient, inspect evidence/source and say which docs are missing or stale.
3. Cite files, symbols, commands, or generated wiki pages.
4. Do not present hypotheses as facts.

## Companion Tool

If a `codewiki` Rust binary is available, use it for deterministic repo inspection, config/state operations, validation, or cache/index maintenance. Do not treat the binary as the product surface; it supports the skill.
