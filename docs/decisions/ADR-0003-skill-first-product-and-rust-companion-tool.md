---
artifact_type: adr
id: ADR-0003
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
  requirements: [docs/requirements/SPEC.md, docs/requirements/REQUIREMENTS.md]
  backlog_items: [BL-008]
---

# ADR-0003: Skill-First Product And Rust Companion Tool

## Status

Accepted on 2026-07-13.

Supersedes the part of ADR-0002 that described the Rust CLI as the primary product surface.

## Context

The final purpose of this project is a Codex skill that can be installed into and used across repositories. Rust remains useful for deterministic local operations, but the user does not need a standalone CLI as the main interface.

## Decision

CodeWiki is a skill-first product.

Rust is a companion tool for the skill. It may provide deterministic helpers for repository inspection, config/state operations, validation, cache/index maintenance, and future performance-sensitive paths. The skill workflow, `SKILL.md`, installer, and repo-native docs/state model are the primary product surface.

Provide a one-command installer from the repository that installs `skill/codewiki` into `$CODEX_HOME/skills/codewiki`.

## Options Considered

- Keep Rust CLI as the primary product: rejected because the user's desired final artifact is a reusable skill.
- Remove Rust entirely: rejected because Rust remains useful for robust local companion tooling.
- Skill-first with Rust companion tool: accepted because it matches the final product goal while preserving deterministic implementation capacity.

## Consequences

- Documentation must say "skill" before "CLI".
- The Rust workspace should avoid over-owning UX and should expose helper behavior the skill can call.
- The installer script is part of the product foundation.
- Future tickets should prioritize skill init/sync/Q&A behavior and target-repo installation flow.

