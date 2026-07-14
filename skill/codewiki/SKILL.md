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
- Treat current documentation as durable user input. Never silently overwrite manual edits, including edits inside a generated region; preserve and semantically reconcile them with refreshed evidence.
- Keep committed project config/docs separate from local persistent state and rebuildable cache.
- Git is the default source for code changes. Do not bundle Jira/Figma/etc. providers in CodeWiki core.
- If the output location is ambiguous, confirm whether to write docs in the source repo or in an external/personal workspace before writing files.
- Support non-Git sources through `.agents/skills/codewiki/project/sources.yml` and user-provided source extension skills.
- Do not install every provider by default. Use Octocode as the first-choice code-intelligence provider when filesystem/Git exploration is insufficient; use codebase-memory-mcp and CocoIndex only under their specific triggers.

## Mandatory Execution Gate

For every init or sync request, the first filesystem mutation MUST be the installed preflight command:

```text
<resolved-skill-root>/scripts/codewiki-preflight.sh init <repository-path>
<resolved-skill-root>/scripts/codewiki-preflight.sh sync <repository-path>
```

Resolve the active skill root from the skill that loaded these instructions; prefer the project-local installation when present. Do not create or modify reader-facing Markdown before preflight succeeds and proves the control plane, evidence pages, and source provenance exist. Init must return `synthesis_incomplete`; sync may return `reader_docs_ready` only for a verified no-op, in which case preserve the docs. Do not replace this command with an improvised repository summary.

After model planning/synthesis and quality review, the final mutating workflow command MUST be:

```text
<resolved-skill-root>/scripts/codewiki-helper.sh validate <repository-path>
```

Never call generation complete, successful, ready, or onboarding-quality unless validation returns `generation_status: reader_docs_ready`. If preflight or validation fails, report the exact failure and leave the run incomplete.

## Always-Active Reader Contract

These invariants apply even before detailed references are loaded:

1. Treat the current filesystem working tree as init's source of truth; use Git for identity, provenance, change context, and history, not as a substitute for reading current code.
2. Complete the evidence-backed repository mental model and WikiPlan before drafting any reader page.
3. Put purpose, reader outcome, and plain-language explanation before source inventory or implementation mapping.
4. Keep important claims source-backed and label hypotheses explicitly; never turn lexical symbol/import hints into architecture facts by themselves.
5. Make every page and diagram answer a named reader question, then run isolated docs-only evaluation, source audit, and final companion validation before reporting success.

## Always-Active Anti-Patterns

Reject output that exhibits any of these patterns before loading any reference:

- **File-list opener**: starting any reader page with a directory tree, file list, or symbol inventory before explaining purpose and reader outcome.
- **Unanchored claim**: asserting an architecture or behavior fact without citing at least one file path, symbol name, command, or existing doc as evidence.
- **Uncaptioned diagram**: creating a diagram with no caption that explicitly names the reader question the diagram answers.
- **False completion**: declaring init or sync complete, successful, or onboarding-ready before both `codewiki-preflight.sh` and `codewiki-helper.sh validate` return success.
- **Unlabeled hypothesis**: presenting speculative content as confirmed fact without an explicit hypothesis marker, evidence gap, and confidence note.

## Reference Loading

Keep this file as the compact router. Load bundled references only when needed:

- `references/docs-structure.md`: load for every init/sync run and when repairing or explaining the wiki layout.
- `references/workspace-placement.md`: load before writing files when repo-local vs external/personal wiki placement is not explicit.
- `references/source-extensions.md`: load when adding or using non-Git evidence/change sources.
- `references/source-skill-template.md`: load when the user wants to create a custom source skill.
- `references/semantic-exploration.md`: load during init/sync before writing docs, and when Q&A requires source evidence beyond generated docs.
- `references/conventions.md`: load during every init/sync to discover project, language, framework, and area conventions from repository evidence.
- `references/reader-first.md`: load during every init/sync to build the mental model and typed WikiPlan, synthesize reader pages, run isolated quality gates, and complete companion validation.
- `references/init.md`: load when initializing or generating a new CodeWiki.
- `references/sync.md`: load when updating, refreshing, or reconciling an existing CodeWiki.
- `references/qa.md`: load when answering questions from existing CodeWiki docs/state.
- `references/deep-research.md`: load for broad, risky, weakly documented, or semantically complex questions.

## Target Repository Layout

When initializing a target repository, prefer:

```text
.agents/skills/codewiki/project/
  config.yml
  plan.yml
  AGENTS.md
  sources.yml
docs/
  QUICKSTART.md
  SOURCE-MAP.md
  architecture/
    OVERVIEW.md
    DECISIONS.md
  domain/
    OVERVIEW.md
  workflows/
    OVERVIEW.md
  data-models/
    OVERVIEW.md
  api/
    OVERVIEW.md
  operations/
    RUNBOOK.md
  testing/
    STRATEGY.md
  conventions/
    OVERVIEW.md
  GLOSSARY.md
  OPEN-QUESTIONS.md
  evidence/
    README.md
    SOURCES.md
    COMMANDS.md
    CLAIMS.md
  areas/
    <area-slug>/
      OVERVIEW.md
```

`docs/QUICKSTART.md` and `docs/conventions/OVERVIEW.md` are required after a successful synthesized init. Keep generated Markdown filenames uppercase while keeping directories lowercase. Other pages are canonical slots, not mandatory stubs: create them only when the repository has enough evidence-backed content. Dynamic pages live under their semantic owner; `areas/**` is legacy compatibility input, not a default generation target.

Follow the reader-first contract: start with a concise quickstart, create section directories only for real documentation areas, avoid thin one-file folders unless they have a strong boundary, explain purpose and mental model before code details, cite important claims locally, and keep source inventories at the end or under `docs/evidence/**`.

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

1. Record the selected tool and trigger reason in `.agents/skills/codewiki/project/config.yml`.
2. Add or update `.agents/skills/codewiki/project/AGENTS.md` with local install/activation notes for that repository.
3. Install or activate the tool in the target repo only after the trigger is met.
4. Record the tool version/config/evidence source in local runtime state.

If no optional tool is available, continue with Git/filesystem exploration and mark reduced confidence where appropriate.

## Docs-First Activation

After CodeWiki has generated docs, answer questions in this order:

1. `docs/**`
2. `.agents/skills/codewiki/project/plan.yml`
3. `.agents/skills/codewiki/project/AGENTS.md`
4. local SQLite facts/evidence/claims
5. source files and Git history
6. external runtime tools, only when the earlier layers are stale, insufficient, or the user asks for graph/index/memory-heavy analysis

This means ordinary Q&A about documented architecture should not activate Octocode, codebase-memory-mcp, or CocoIndex. Activate them lazily when evidence quality requires it.

## Init Workflow

1. Run the mandatory `codewiki-preflight.sh init <repository-path>` gate before writing reader docs; expect `generation_status: synthesis_incomplete`.
2. Resolve repository identity from Git and filesystem context, then inspect the current working tree, including relevant uncommitted and untracked source.
3. Inspect existing docs and bounded evidence before source where useful.
4. Explore source semantically for the mental model, conventions, and page-specific evidence gaps.
5. Build the repository mental model and WikiPlan v2 page contracts from evidence.
6. Generate reader-first `docs/**` one page contract at a time and mark uncertainty explicitly.
7. Run contract, source, diagram, cross-page, and isolated docs-only onboarding evaluation; perform at most one bounded revision.
8. Write the quality report and run the mandatory companion validation command.
9. Record verification commands or skip reasons. Init is complete only at `reader_docs_ready`.

## Sync Workflow

1. Run the mandatory `codewiki-preflight.sh sync <repository-path>` gate before modifying reader docs.
2. Compare Git state, known evidence, generated pages, and changed files.
3. Read the current affected docs and detect manual changes before generating replacements.
4. Mark stale pages and stale claims before rewriting.
5. Refresh unchanged generated regions automatically only when their recorded integrity hash still matches.
6. If a generated body was manually edited, preserve it and semantically merge refreshed evidence around the user's contribution; do not restore old machine wording.
7. Preserve unmarked and legacy hashless pages unless the user explicitly authorizes replacement.
8. Run mandatory companion validation and record what changed, what was preserved, and why.

## Q&A Workflow

1. Answer from `docs/**` first.
2. If docs are insufficient, inspect evidence/source and say which docs are missing or stale.
3. Cite files, symbols, commands, or generated wiki pages.
4. Do not present hypotheses as facts.

## Companion Tool

If a `codewiki` Rust binary is available, use it for deterministic repo inspection, config/state operations, validation, or cache/index maintenance. Do not treat the binary as the product surface; it supports the skill.

Installed skills include `scripts/codewiki-helper.sh`. Use that wrapper when available so the skill can locate the Rust companion through:

1. `CODEWIKI_COMPANION_BIN`
2. installed `bin/codewiki`
3. `codewiki` on `PATH`
4. `CODEWIKI_REPO`
5. bundled companion source or the source checkout that contains this skill

Prefer the prebuilt binary path. Use source/Cargo fallback only when the installed binary is missing or intentionally replaced. If the wrapper cannot locate the companion, continue with filesystem/Git reasoning and record the reduced deterministic-helper coverage.
