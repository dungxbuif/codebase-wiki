---
artifact_type: adr
id: ADR-0005
status: accepted
owner: shared
human_fields:
  - decision
  - consequences_acceptance
ai_fields:
  - context
  - options
  - evidence
shared_fields:
  - status
  - trace
trace:
  phase: docs/work/phases/PHASE-001-codewiki-foundation.md
  requirements: [docs/requirements/SPEC.md]
  backlog_items: [BL-001, BL-003, BL-004]
  related:
    - docs/work/research/REFERENCE-PROMPTS.md
    - docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md
    - docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md
    - docs/decisions/ADR-0007-uppercase-generated-markdown-filenames.md
    - docs/decisions/ADR-0008-code-conventions-documentation.md
    - docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md
---

# ADR-0005: CodeWiki Generated Docs Structure

## Status

Accepted on 2026-07-13.

## Context

CodeWiki needs one stable generated documentation structure so `init`, `sync`, and docs-first Q&A do not invent a different wiki layout for every repository. The structure must work across arbitrary codebases, support semantic quality, remain navigable for humans, and stay useful to future agents after session resets or model changes.

The reference prompt analysis shows that OpenWiki's strong documentation boundaries are worth adopting, while DeepWiki's mode separation and structured context handling are useful. CodeWiki needs to combine those ideas with a repo-native storage model:

- generated docs should be committed and human-readable;
- repo-local control files should be committed separately from generated docs;
- local durable state should live outside the repo;
- rebuildable cache should not be confused with durable state.

## Decision

Use this default target-repository structure:

```text
.agents/skills/codewiki/project/
  config.yml
  plan.yml
  AGENTS.md

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

ADR-0007 refines this structure by requiring uppercase generated Markdown basenames while directories remain lowercase. ADR-0008 adds the required conventions page. Only create pages that have real content. `QUICKSTART.md` and `conventions/OVERVIEW.md` are always required. Other section pages are canonical slots: if a repository does not have a meaningful data model, public interface, operations story, or testing surface, the page may be omitted and the gap recorded in `QUICKSTART.md` backlog, `OPEN-QUESTIONS.md`, or `.agents/skills/codewiki/project/plan.yml`.

The same structure may live inside the source repository or inside a separate external/personal wiki workspace. In external workspace mode, the source repository remains evidence input and the generated docs/control plane are written to the selected workspace.

The role of each layer is:

| Layer | Path | Ownership | Purpose |
| --- | --- | --- | --- |
| Control plane | `.agents/skills/codewiki/project/config.yml` | AI-generated, human-reviewable | Stable config, docs root, provider policy, sync settings. |
| Wiki plan | `.agents/skills/codewiki/project/plan.yml` | AI-generated, human-reviewable | Page plan, coverage, confidence, stale areas, refresh strategy. |
| Agent guidance | `.agents/skills/codewiki/project/AGENTS.md` | AI-generated, human-reviewable | Local CodeWiki run rules and optional provider activation notes. |
| Source registry | `.agents/skills/codewiki/project/sources.yml` | AI-generated, human-reviewable | Primary Git source and optional user-provided source skill declarations. |
| Knowledge surface | `docs/**` | AI-generated docs with human-owned override sections where needed | Human/agent readable semantic wiki and docs-first Q&A source. |
| Durable local state | platform app data SQLite | AI/runtime-owned | Facts, evidence, claims, symbols, sync runs, provider snapshots. |
| Rebuildable cache | platform cache directory | runtime-owned | Derived indexes, embeddings, parsed symbol cache, provider cache. |

## Canonical Page Semantics

- `QUICKSTART.md`: entrypoint, repository overview, how to use the wiki, major sections, freshness, key source files, future-agent notes, and backlog.
- `SOURCE-MAP.md`: semantic navigation map: systems, packages, services, apps, bounded contexts, and where to start.
- `architecture/OVERVIEW.md`: runtime architecture, major components, dependency direction, architectural constraints, and change risks.
- `architecture/DECISIONS.md`: durable architectural/product decisions inferred from docs, code, and Git history; link to existing ADRs when present.
- `domain/OVERVIEW.md`: product/business/domain concepts and invariants, when the repository has meaningful domain logic.
- `workflows/OVERVIEW.md`: important user/system flows, jobs, event flows, lifecycle transitions, and operational sequences.
- `data-models/OVERVIEW.md`: persistence, schemas, migrations, models, storage boundaries, and data ownership.
- `api/OVERVIEW.md`: public APIs, CLIs, events, RPC, package/library interfaces, integration contracts.
- `operations/RUNBOOK.md`: setup, build/run/deploy, environment, observability, troubleshooting, and runtime risks.
- `testing/STRATEGY.md`: test strategy, verification commands, fixtures, coverage gaps, and safe-change checks.
- `conventions/OVERVIEW.md`: explicit and inferred project, language, framework/library, and area conventions with scope, evidence, confidence, exceptions, and change impact.
- `GLOSSARY.md`: project-specific terms, acronyms, domain language, and aliases.
- `OPEN-QUESTIONS.md`: real uncertainties that affect future understanding, sync quality, or safe changes.
- `evidence/README.md`: how evidence is recorded and how to verify claims.
- `evidence/SOURCES.md`: source files, docs, Git history, provider outputs, and inspected artifacts.
- `evidence/COMMANDS.md`: commands run or recommended for verification, with result summaries when available.
- `evidence/CLAIMS.md`: durable claims with evidence links, confidence, status, and owning page.
- `areas/<area-slug>/OVERVIEW.md`: optional deeper pages for substantial areas. Do not create stub area pages.

## Generation Rules

- `docs/QUICKSTART.md` must always exist after successful init.
- `docs/conventions/OVERVIEW.md` must always exist after successful init and must describe repository evidence rather than generic ecosystem guidance.
- Prefer `QUICKSTART.md` sections and broad section overview pages before creating many area pages.
- Do not create a page just because a page slot exists.
- Do not split concepts into separate pages until there is enough evidence-backed content to justify the split.
- Do not create low-value one-file section directories unless the boundary is substantial and likely to grow.
- Source-backed pages should begin with a `<details>` block listing relevant source files, following the DeepWiki generated-page pattern.
- Each durable claim must be connected to file, symbol, command, existing doc, Git evidence, provider evidence, or explicit hypothesis.
- Existing unmarked human-authored `docs/**` files remain source evidence and must not be overwritten by CodeWiki.
- CodeWiki may write direct `docs/` pages only when a canonical page is missing or already contains CodeWiki generated-region markers.
- New generated regions carry a portable body hash. Sync may automatically replace a region only when its current body matches that baseline.
- Sync preserves manual edits inside and outside generated regions, preserves legacy hashless regions conservatively, and routes conflicts to LLM semantic reconciliation per ADR-0009.
- Sync should avoid formatting-only rewrites.
- Q&A must read `docs/**` before falling back to `.agents/skills/codewiki/project/plan.yml`, `.agents/skills/codewiki/project/AGENTS.md`, local SQLite evidence, source/Git, or optional providers.
- If source repo and wiki workspace differ, Q&A must treat `.agents/skills/codewiki/project/sources.yml` as the map from workspace to source evidence.

## Options Considered

- Put all generated docs directly under `.agents/skills/codewiki/project/`: rejected because dotdir content reads like control/runtime state and is less natural as committed human documentation.
- Use a single `docs.md`: rejected because it becomes too large for medium/large repositories and weakens page-level sync.
- Generate arbitrary pages from the LLM's first plan: rejected because sync and Q&A need stable canonical anchors across sessions.
- Copy OpenWiki's `openwiki/` layout: rejected because CodeWiki is repo-native and should fit common `docs/` conventions.

## Consequences

- `docs/**` becomes the canonical generated documentation root.
- `.agents/skills/codewiki/project/**` becomes the committed control plane, not the primary reading surface.
- The Rust companion defaults and skill prompt should keep `docs_root: docs`.
- Future prompt modules must target this structure during init, sync, and Q&A.
- Large repositories can add `areas/<area-slug>/OVERVIEW.md` pages, but only when the area is substantial.
- Config may eventually allow custom docs roots, but the default and documented contract remain `docs/**`.
- Workspace placement is governed by `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`.
