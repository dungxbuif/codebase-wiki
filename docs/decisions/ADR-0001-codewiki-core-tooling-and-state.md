---
artifact_type: adr
id: ADR-0001
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
  backlog_items: [BL-003, BL-004]
---

# ADR-0001: CodeWiki Core Tooling And Durable State

## Status

Accepted on 2026-07-13.

Provider selection details are superseded by `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`. Git, filesystem, SQLite, evidence modeling, and durable state decisions remain active.

## Context

CodeWiki must work across repositories and across Codex sessions. The user prefers fewer tools when they are the right tools, and wants semantic analysis quality without building language/framework-specific adapters into the core.

The system also needs to reuse prior config and storage when switching LLMs or starting a new session.

## Decision

Start with a minimal core tool surface:

- Git for repository identity, diffs, history, and sync boundaries.
- Filesystem for source and generated docs.
- SQLite for durable local runtime state.
- A narrow optional code-intelligence provider boundary. Provider choice is runtime and target-repo specific.

Do not build core language/framework adapters. CodeWiki should detect languages, libraries, frameworks, entrypoints, package managers, and architecture signals dynamically, then let the LLM explore with evidence.

Separate state into:

- Committed project config and generated docs, such as `.codewiki/config.yml`, `.codewiki/plan.yml`, and `docs/codewiki/**`.
- Persistent local runtime state in platform app data, keyed by repository identity.
- Rebuildable cache for embeddings, indexes, parsed symbols, and provider-specific derived data.

Persist structured facts, hypotheses, evidence, claims, work items, checkpoints, and migrations. Do not rely on chain-of-thought or chat history as durable state.

## Options Considered

- Use codebase-memory-mcp as core memory: useful, but too broad as a mandatory dependency before the state model is proven.
- Use CocoIndex as core indexing: useful for indexing pipelines, but heavier than needed for the first durable state boundary.
- Use Octocode as an optional code-intelligence candidate: promising for some repositories, but not a mandatory dependency.
- Build adapters per language/framework: rejected for core because it conflicts with universal repo support and would grow maintenance cost quickly.

## Consequences

- Provider interfaces must be narrow and replaceable from the beginning.
- SQLite schema and migrations are first-class product work, not an afterthought.
- Detection quality and evidence modeling become central to semantic quality.
- Additional tools can still be added later, but only when a concrete gap is proven.
