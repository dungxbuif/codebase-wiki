---
artifact_type: ticket
id: TICKET-012
status: done
owner: human
priority: high
lane: high-risk
trace:
  backlog_item: BL-004
  requirement: REQ-002, REQ-004
  phase: PHASE-001
  detail_design: docs/work/designs/DESIGN-012-semantic-exploration-v1.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md, docs/decisions/ADR-0005-codewiki-generated-docs-structure.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: TICKET-012 Semantic Exploration V1

## Status

- ID: TICKET-012
- Status: done
- Type: feature
- Priority: high
- Phase: PHASE-001

## Problem

Detection v1 identifies stack signals, but generated docs still cannot describe repository structure from source evidence. CodeWiki needs a deterministic semantic snapshot that can feed WikiPlan, docs generation, sync, and future Q&A without hard-coding framework adapters.

## Acceptance Criteria

- [x] Add a semantic exploration boundary that scans bounded repository files and emits modules/files, roles, symbols, imports/dependency hints, areas, and evidence references.
- [x] Keep exploration dynamic and adapter-free: use generic source/text patterns, detected paths, and evidence records rather than framework-specific providers.
- [x] Use semantic exploration output during init/sync to enrich `map.md`, `architecture.md`, and `evidence/sources.md`.
- [x] Preserve repo-local and external workspace behavior.
- [x] Tests cover at least Rust and TypeScript-shaped fixtures plus external workspace behavior.

## Scope

In scope:

- Deterministic filesystem-based semantic snapshot v1.
- Generic lexical symbol/import discovery for common code shapes.
- Docs rendering from semantic snapshot.
- Rust companion integration for init/sync helper paths.

Out of scope:

- Mandatory Octocode/codebase-memory-mcp/CocoIndex activation.
- Full AST parsing or language server integration.
- Natural-language synthesis beyond evidence-backed summaries.
- Built-in framework/provider adapters.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test -p codewiki-explore -p codewiki-docs -p codewiki-core`
- `rtk cargo test`
- Manual grep check that no new source-provider CLI UX was introduced.

## Verification Results

- Command: `rtk cargo fmt --all --check`
- Result: pass
- Notes: formatting verified.

- Command: `rtk cargo test -p codewiki-explore -p codewiki-docs -p codewiki-core`
- Result: pass
- Notes: 14 tests passed across 6 suites.

- Command: `rtk cargo test`
- Result: pass
- Notes: 31 tests passed across 13 suites.

- Command: `rtk proxy rg -n "source add|--workspace|--output|codewiki source|codewiki init \\[source\\]|codewiki sync \\[source\\]" . --glob '!docs/work/tickets/TICKET-012-semantic-exploration-v1.md'`
- Result: pass
- Notes: only ADR-0006 contains a deliberate statement rejecting `codewiki source add` UX.

## Completion Checklist

- [x] Implementation complete
- [x] Tests run and recorded
- [x] Validation matrix updated
- [x] UAT completed or explicitly not required
- [x] Master docs reconciled
- [x] Docs review completed
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
