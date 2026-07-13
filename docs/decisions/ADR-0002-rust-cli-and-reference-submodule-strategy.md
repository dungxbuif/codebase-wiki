---
artifact_type: adr
id: ADR-0002
status: superseded
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
  requirements: [docs/requirements/SPEC.md, docs/requirements/REQUIREMENTS.md]
  backlog_items: [BL-002, BL-007]
---

# ADR-0002: Rust CLI And Reference Submodule Strategy

## Status

Superseded on 2026-07-13 by `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`.

The reference-submodule strategy remains useful. The claim that the Rust CLI is the primary product surface is superseded.

## Context

CodeWiki will use `references/openwiki` and `references/deepwiki-open` as technical references. These projects can teach useful patterns for wiki generation, repo ingestion, prompting, update flows, provider integration, UI/API boundaries, and operational concerns.

The user initially wanted the CodeWiki CLI rewritten in Rust rather than inheriting either reference project's CLI/runtime shape. The user later clarified that the final project is a Codex skill and Rust is only a companion tool.

## Decision

Implement Rust as a companion tool for the CodeWiki skill, not as the primary product surface.

Use the two submodules as references, not as vendor foundations:

- Study their techniques, data flow, prompts, update model, and UX tradeoffs.
- Extract portable lessons into CodeWiki design docs and tickets.
- Build companion Rust crates around strong typed schemas, explicit error handling, deterministic filesystem behavior, and SQLite-backed durable state.
- Keep provider integrations and LLM-facing workflows behind narrow Rust traits/interfaces so the CLI remains testable and replaceable.

## Options Considered

- Fork OpenWiki runtime: rejected because CodeWiki needs a repo-native skill and Rust CLI with its own state model.
- Fork deepwiki-open runtime: rejected because it is useful as a product/reference system, but CodeWiki should not inherit a web-app-first architecture.
- Write a Rust companion tool informed by both: superseded by ADR-0003's skill-first framing.

## Consequences

- The implementation plan may include a Rust workspace/crate layout for companion tooling.
- Early tickets should prioritize skill workflow and install behavior, while Rust tickets define config schema, SQLite migrations, and provider traits only when they support the skill.
- Reference study should happen before copying patterns into implementation.
- Any UI/server/CLI components remain secondary to the CodeWiki skill workflow.
