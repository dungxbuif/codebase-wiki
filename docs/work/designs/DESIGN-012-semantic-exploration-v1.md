---
artifact_type: detail_design
id: DESIGN-012
status: ready
owner: ai
trace:
  ticket: docs/work/tickets/TICKET-012-semantic-exploration-v1.md
  phase: PHASE-001
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  master_docs: [docs/requirements/SPEC.md, docs/architecture/ARCHITECTURE.md]
---

# Design: Semantic Exploration V1

## Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/standards/README.md`
- `docs/standards/QUALITY_BAR.md`
- `docs/standards/VALIDATION.md`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`
- `docs/requirements/SPEC.md`
- `docs/requirements/REQUIREMENTS.md`
- `docs/architecture/ARCHITECTURE.md`
- `docs/work/tickets/TICKET-007-repo-detection-v1.md`
- `docs/work/tickets/TICKET-008-wikiplan-evidence-models.md`

## Problem Statement

CodeWiki currently detects repository stack signals and emits canonical starter docs, but it does not yet produce a semantic repository snapshot. That leaves generated wiki pages in a pending state and blocks higher-quality sync/Q&A.

## Proposed Approach

Add a new Rust companion crate, `codewiki-explore`, with a deterministic `explore_repository(root)` API that returns:

- `ExplorationSnapshot`: schema version, files, areas, dependency hints, evidence refs, and limits.
- `ExploredFile`: repo-relative path, language, role, line count, symbols, imports, and evidence id.
- `ExploredSymbol`: generic symbol kind/name/line extracted from lexical source patterns.
- `DependencyHint`: file-to-import relationships from common import/use/include/require forms.
- `AreaSummary`: top-level area grouping with file and symbol counts.

Integrate this snapshot into `codewiki-docs` so init/sync renders richer:

- `docs/map.md`: areas, important files, symbols, dependency hints.
- `docs/architecture.md`: evidence-backed structure summary.
- `docs/evidence/sources.md`: explored files and evidence IDs.

## Brownfield Scope

Touched modules:

- `Cargo.toml`
- `crates/codewiki-explore`
- `crates/codewiki-docs`
- `crates/codewiki-core`
- Harness docs for ticket, validation, backlog, context, changelog, and architecture.

Direct dependencies:

- `codewiki-detect` remains stack detection.
- `codewiki-docs` consumes semantic snapshot for rendering.
- `codewiki-core` orchestrates detection + exploration.

## Alternatives Considered

- Use Octocode immediately: rejected for this slice because provider activation must remain optional/lazy and target-repo specific.
- Add language-specific AST parsers: rejected because core must not become a collection of language/framework adapters.
- Keep docs pending until LLM-only exploration: rejected because durable state and repeatable tests need deterministic evidence input.

## Risks

- Lexical extraction can miss or over-label symbols. Mitigation: mark output as hints/evidence, not final truth.
- Large repositories can be expensive. Mitigation: bounded traversal, skip generated/vendor dirs, record limits.
- Generic import detection can include external packages. Mitigation: call them dependency hints, not resolved dependency graph.

## Verification

- Unit tests in `codewiki-explore` for Rust and TypeScript-shaped fixtures.
- Integration tests in `codewiki-core` proving init emits semantic docs and external workspace still keeps generated docs outside the source repo.
- Full workspace cargo test.

## Reconciliation Plan

- Requirements: update REQ-002/REQ-004 status or evidence if tests pass.
- Architecture: replace Explorer `TBD` with `crates/codewiki-explore`.
- ADR: no new ADR required; this implements existing ADR-0001/ADR-0005 boundaries without changing provider strategy.
- Validation matrix: add/update semantic exploration row.
- Changelog/context/backlog: update after implementation.
