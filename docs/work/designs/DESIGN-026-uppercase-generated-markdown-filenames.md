---
artifact_type: detail_design
id: DESIGN-026
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-015
  requirement: REQ-009
  phase: not_applicable
  ticket_or_bug: docs/work/tickets/TICKET-026-uppercase-generated-markdown-filenames.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0007-uppercase-generated-markdown-filenames.md]
  master_docs_touched:
    - docs/requirements/SPEC.md
    - docs/architecture/ARCHITECTURE.md
    - docs/architecture/API.md
---

# Detail Design: Uppercase Generated Markdown Filenames

## Problem And Context

Generated wiki filenames are a public, persisted contract used by skill prompts, WikiPlan, Rust generation, sync safety, docs-first Q&A, and links. A case-only naming change can duplicate pages or overwrite human documentation unless migration is marker-aware.

## Proposed Contract

- Keep directories lowercase: `architecture/`, `domain/`, `evidence/`, and similar.
- Uppercase every generated Markdown basename: `QUICKSTART.md`, `SOURCE-MAP.md`, `OVERVIEW.md`, `DECISIONS.md`, `RUNBOOK.md`, `STRATEGY.md`, `GLOSSARY.md`, `OPEN-QUESTIONS.md`, `README.md`, `SOURCES.md`, `COMMANDS.md`, and `CLAIMS.md`.
- Use `areas/<area-slug>/OVERVIEW.md` for area pages.
- Use `QUICKSTART.md` as the sole canonical entrypoint. Remove the stale `index.md` instruction.

## Migration And Safety

Before init/sync writes canonical pages, inspect the known legacy lowercase path mappings. Rename a legacy page to its uppercase canonical path only when:

1. the legacy page exists;
2. the uppercase page does not exist; and
3. the legacy page contains both CodeWiki generated-region markers.

Preserve unmarked legacy files and preserve both files when the uppercase target already exists. This avoids silently mutating human-owned documentation.

## Touched Scope

- `crates/codewiki-docs`: generated paths, links, relevant-source routing, tests.
- `crates/codewiki-core`: marker-aware legacy migration and tests.
- `crates/codewiki-detect`, `crates/codewiki-explore`: generated-path exclusion.
- `crates/codewiki-store`: WikiPlan canonical paths.
- `skill/codewiki/**`: target layout and workflows.
- Current product/master/decision/validation/release docs.

## Alternatives

- Keep lowercase filenames: rejected by explicit user naming decision.
- Rename every lowercase Markdown file in a target repo: rejected because existing files may be human-owned.
- Generate uppercase files without migration: rejected because previously generated pages would be duplicated and become stale.

## Verification

- Unit tests assert uppercase paths and absence of lowercase generated outputs.
- Migration tests prove marked legacy pages are renamed and unmarked pages are preserved.
- Production fixtures and full workspace tests pass.
- Smoke init proves the on-disk layout.

## Reconciliation

- Update ADR-0005 through ADR-0007's narrower naming decision.
- Update SPEC, architecture, API, validation matrix, changelog, context, backlog, and traceability.
- No ERD, security, provider, or source-extension behavior changes.
