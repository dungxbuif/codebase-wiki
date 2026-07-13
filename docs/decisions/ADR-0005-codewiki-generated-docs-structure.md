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
.codewiki/
  config.yml
  plan.yml
  AGENTS.md

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

Only create pages that have real content. `index.md` is always required. Other top-level pages are canonical slots: if a repository does not have a meaningful data model, public interface, operations story, or testing surface, the page may be omitted and the gap recorded in `open-questions.md` or `.codewiki/plan.yml`.

The role of each layer is:

| Layer | Path | Ownership | Purpose |
| --- | --- | --- | --- |
| Control plane | `.codewiki/config.yml` | AI-generated, human-reviewable | Stable config, docs root, provider policy, sync settings. |
| Wiki plan | `.codewiki/plan.yml` | AI-generated, human-reviewable | Page plan, coverage, confidence, stale areas, refresh strategy. |
| Agent guidance | `.codewiki/AGENTS.md` | AI-generated, human-reviewable | Local CodeWiki run rules and optional provider activation notes. |
| Knowledge surface | `docs/codewiki/**` | AI-generated docs with human-owned override sections where needed | Human/agent readable semantic wiki and docs-first Q&A source. |
| Durable local state | platform app data SQLite | AI/runtime-owned | Facts, evidence, claims, symbols, sync runs, provider snapshots. |
| Rebuildable cache | platform cache directory | runtime-owned | Derived indexes, embeddings, parsed symbol cache, provider cache. |

## Canonical Page Semantics

- `index.md`: entrypoint, repository overview, how to use this wiki, major sections, freshness, and known coverage gaps.
- `map.md`: semantic navigation map: systems, packages, services, apps, bounded contexts, and where to start.
- `architecture.md`: runtime architecture, major components, dependency direction, architectural constraints, and change risks.
- `domains.md`: product/business/domain concepts and invariants, when the repository has meaningful domain logic.
- `workflows.md`: important user/system flows, jobs, event flows, lifecycle transitions, and operational sequences.
- `data.md`: persistence, schemas, migrations, models, storage boundaries, and data ownership.
- `interfaces.md`: public APIs, CLIs, events, RPC, package/library interfaces, integration contracts.
- `operations.md`: setup, build/run/deploy, environment, observability, troubleshooting, and runtime risks.
- `testing.md`: test strategy, verification commands, fixtures, coverage gaps, and safe-change checks.
- `decisions.md`: durable architectural/product decisions inferred from docs, code, and Git history; link to existing ADRs when present.
- `glossary.md`: project-specific terms, acronyms, domain language, and aliases.
- `open-questions.md`: real uncertainties that affect future understanding, sync quality, or safe changes.
- `evidence/README.md`: how evidence is recorded and how to verify claims.
- `evidence/sources.md`: source files, docs, Git history, provider outputs, and inspected artifacts.
- `evidence/commands.md`: commands run or recommended for verification, with result summaries when available.
- `evidence/claims.md`: durable claims with evidence links, confidence, status, and owning page.
- `areas/<area-slug>.md`: optional deeper pages for substantial areas. Do not create stub area pages.

## Generation Rules

- `docs/codewiki/index.md` must always exist after successful init.
- Prefer canonical top-level pages before creating many area pages.
- Do not create a page just because a page slot exists.
- Do not split concepts into separate pages until there is enough evidence-backed content to justify the split.
- Each durable claim must be connected to file, symbol, command, existing doc, Git evidence, provider evidence, or explicit hypothesis.
- Human-authored docs outside `docs/codewiki/**` remain source evidence, not generated output.
- Sync should preserve human-owned sections and avoid formatting-only rewrites.
- Q&A must read `docs/codewiki/**` before falling back to `.codewiki/plan.yml`, `.codewiki/AGENTS.md`, local SQLite evidence, source/Git, or optional providers.

## Options Considered

- Put all generated docs directly under `.codewiki/`: rejected because dotdir content reads like control/runtime state and is less natural as committed human documentation.
- Use a single `docs/codewiki.md`: rejected because it becomes too large for medium/large repositories and weakens page-level sync.
- Generate arbitrary pages from the LLM's first plan: rejected because sync and Q&A need stable canonical anchors across sessions.
- Copy OpenWiki's `openwiki/` layout: rejected because CodeWiki is repo-native and should fit common `docs/` conventions.

## Consequences

- `docs/codewiki/**` becomes the canonical generated documentation root.
- `.codewiki/**` becomes the committed control plane, not the primary reading surface.
- The Rust companion defaults and skill prompt should keep `docs_root: docs/codewiki`.
- Future prompt modules must target this structure during init, sync, and Q&A.
- Large repositories can add `areas/<area-slug>.md` pages, but only when the area is substantial.
- Config may eventually allow custom docs roots, but the default and documented contract remain `docs/codewiki/**`.

