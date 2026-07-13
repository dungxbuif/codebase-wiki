---
artifact_type: adr
id: ADR-0008
status: accepted
owner: human
trace:
  phase: not_applicable
  requirements: [docs/requirements/SPEC.md]
  backlog_items: [BL-016]
  triggering_ticket: docs/work/tickets/TICKET-027-code-conventions-documentation.md
  related:
    - docs/decisions/ADR-0005-codewiki-generated-docs-structure.md
    - docs/decisions/ADR-0007-uppercase-generated-markdown-filenames.md
    - docs/architecture/ARCHITECTURE.md
---

# ADR-0008: Code Conventions Documentation

## Status

Accepted on 2026-07-14 by direct human decision.

## Context

Architecture and API docs do not fully tell a future developer or agent how code is expected to be shaped. Repositories encode conventions through formatter/linter/build configuration, recurring structures, framework usage, tests, and intentional exceptions. Generic best-practice text is not sufficient because the actual repository may diverge from ecosystem defaults.

## Decision

Add `docs/conventions/OVERVIEW.md` as a canonical CodeWiki page. Require CodeWiki's LLM workflow to discover repository-, language-, framework-, and area-level conventions from code evidence. Label each convention as explicit, inferred, hypothesis, or exception; require repeated independent examples for inferred conventions; and record scope, evidence, confidence, exceptions, and change impact.

The Rust companion may seed convention evidence candidates but must not promote lexical repetition into a confirmed convention without LLM source analysis.

## Alternatives

- Rely on external style guides: rejected because they do not describe repository-specific behavior.
- Fold conventions into every existing page: rejected because it fragments the change guidance and duplicates cross-cutting rules.
- Add language/framework adapters: rejected because CodeWiki must remain dynamically detected and adapter-free.

## Consequences

- Init and sync must inspect convention evidence and maintain a dedicated page.
- WikiPlan and navigation gain a conventions slot.
- Future agents can consult repository-specific change rules before editing code.
- Small or inconsistent repositories may produce hypotheses and explicit gaps instead of definitive conventions.

