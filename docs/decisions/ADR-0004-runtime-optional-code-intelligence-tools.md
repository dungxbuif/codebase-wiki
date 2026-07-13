---
artifact_type: adr
id: ADR-0004
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
  backlog_items: [BL-005, BL-009]
---

# ADR-0004: Code Intelligence Tool Selection Policy

## Status

Accepted on 2026-07-13.

## Context

CodeWiki may benefit from Octocode, codebase-memory-mcp, and CocoIndex, but making all of them active by default would bloat the skill and make every target repository pay setup cost before a concrete need exists.

The user wants a quality-preserving decision, not an open-ended "maybe any tool" policy. The skill needs a default choice and clear trigger conditions for when additional tools are justified.

## Decision

Choose one default code-intelligence tool path and keep the others conditional.

Default policy:

- **Octocode is the default first-choice code-intelligence provider** when filesystem/Git exploration is not enough for semantic code understanding.
- **codebase-memory-mcp is not a code-intelligence default**. Use it only when the target repo needs shared, durable, cross-session agent memory beyond CodeWiki's own SQLite facts/evidence/state.
- **CocoIndex is not a default**. Use it only when the target repo is large enough, or the refresh/query workload repetitive enough, to justify an indexing pipeline.

Runtime install policy:

- Do not bundle these tools into the CodeWiki skill.
- The skill provides install/activation guidance.
- Agents install or activate tools at runtime inside the local target repository only after the trigger is met.
- Provider/tool choice is recorded in `.codewiki/config.yml`, `.codewiki/AGENTS.md`, and local runtime state.
- Do not run Octocode + codebase-memory-mcp + CocoIndex together by default.

Docs-first activation policy:

- After CodeWiki has generated docs, Q&A must use `docs/codewiki/**`, `.codewiki/plan.yml`, `.codewiki/AGENTS.md`, and local SQLite facts/evidence/claims before activating external tools.
- Source/Git inspection comes next when docs are missing, stale, or insufficient.
- External tools activate only when docs/state/source cannot answer with enough evidence or when the question explicitly requires graph/index/memory capabilities.

Default exploration remains Git + filesystem + Codex reasoning + Rust companion helpers. Escalate to Octocode first for semantic code intelligence; add codebase-memory-mcp or CocoIndex only for their specific triggers.

## Options Considered

- Bundle all three tools: rejected because it increases setup cost, coupling, and quality variance.
- Pick only one forever: rejected because memory and indexing are different problems from code intelligence.
- Use Octocode by default and gate the others by trigger: accepted because it keeps quality consistent while preserving escape hatches.

## Consequences

- `SKILL.md` must include concise tool-selection guidance instead of vendoring tool-specific setup into the skill.
- Runtime configuration must record enabled optional tools per target repo.
- Generated target-repo AGENTS guidance should explain optional tool status and installation rules.
- The Rust companion provider boundary should remain replaceable, but Octocode is the default provider to try first when a provider is needed.
