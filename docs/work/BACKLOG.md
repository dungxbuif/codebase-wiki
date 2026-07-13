---
artifact_type: backlog
id: BACKLOG
status: active
owner: shared
human_fields:
  - priority_override
  - rank
  - blocker_decisions
ai_fields:
  - risk_flags
  - lane_recommendation
  - next_artifact
  - notes
shared_fields:
  - queue_items
  - status
updated: 2026-07-13
---

# Backlog

## Field Ownership

- Human owns priority overrides, rank changes, and blocker decisions.
- AI may recommend lane, risk flags, next artifact, and notes.
- Shared fields include queue rows and item status.

Use this file as the runtime work queue.

The backlog decides what should be worked on next. It does not replace tickets, bugs, requirements, phases, detail designs, or verification artifacts.

## Queue Rules

- Backlog order MAY change at runtime based on priority, severity, dependency, or new information.
- Bugs MAY preempt feature tickets when severity or user impact is higher.
- New requirements MAY enter the backlog before they become requirements docs, phases, or tickets.
- Maintenance and framework work MAY appear in the same queue as product work.
- The active queue focus MUST be reflected in `docs/CONTEXT.md`.
- Do not process all tickets before bugs by default; process the highest-priority queue item that is ready and appropriately scoped.

## Intake Rules

- Keep each item short and actionable.
- Link to source context, requirement, phase, ticket, bug, or decision when available.
- Classify each item by lane: `tiny`, `normal`, `high-risk`, or `blocked`.
- Promote non-tiny backlog items into a requirement slice, phase, ticket, or bug before execution.
- Do not execute directly from backlog unless the item is clearly tiny and records a small-task exemption.
- High-risk items require detail design approval before implementation.
- Blocked items must record the missing decision, dependency, or input.

## Lane Guide

| Lane | Use When | Required Next Artifact |
| --- | --- | --- |
| tiny | Low-risk docs/copy/naming/narrow edits with no contract or runtime impact | Backlog row may be enough with small-task exemption |
| normal | Bounded story-sized work, bug fix, or maintenance task | Ticket or bug |
| high-risk | Auth, authorization, data, security, public contract, external provider, migration, major dependency, or multi-domain impact | Ticket/bug plus detail design and approval |
| blocked | Work cannot proceed because input, decision, dependency, or environment is missing | Blocker note and owner |

## Risk Flags

Mark risk flags in the `Risk Flags` column when relevant:

- Auth
- Authorization
- Data model
- Migration/data loss
- Audit/security/privacy
- External system/provider
- Public contract/API
- Existing behavior
- Weak proof
- Multi-domain
- Deployment/runtime
- Standards change

## Items

| Rank | ID | Type | Lane | Title | Priority | Status | Links | Risk Flags | Next Artifact | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | BL-001 | phase | high-risk | Design complete CodeWiki foundation | High | ready | `docs/work/phases/PHASE-001-codewiki-foundation.md` | Public contract/API, Data model, External system/provider, Multi-domain | Detail design + tickets | Define the product architecture before implementation. |
| 2 | BL-002 | research | normal | Compare OpenWiki and deepwiki-open reference patterns | High | done | `references/openwiki`, `references/deepwiki-open`, `docs/work/research/REFERENCE-PROMPTS.md`, `skill/codewiki/references/` | Existing behavior, Weak proof | done | Prompt pattern analysis was folded into CodeWiki mode references. |
| 3 | BL-003 | architecture | high-risk | Specify config, storage, cache, and migration model | High | done | `docs/work/tickets/TICKET-003-config-storage-skeleton.md`, `docs/work/tickets/TICKET-004-sqlite-state-migrations.md`, `docs/work/tickets/TICKET-005-sqlite-executor-paths.md`, `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md` | Data model, Runtime, Existing behavior | done | Config/plan/AGENTS skeleton, SQLite migration registry, and sqlite3-backed executor/path resolver exist. |
| 4 | BL-004 | architecture | high-risk | Define semantic exploration and evidence model | High | open | `docs/requirements/SPEC.md` | Public contract/API, External system/provider, Multi-domain | Detail design | Includes detection, WikiPlan, claims, evidence, hypotheses, confidence. |
| 5 | BL-005 | integration | normal | Define runtime provider boundary | Medium | open | `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` | External system/provider | Detail design | Octocode is first-choice when needed; codebase-memory-mcp and CocoIndex are trigger-gated. |
| 6 | BL-006 | framework | normal | Finalize CodeWiki-specific Harness standards | Medium | open | `docs/standards/` | Standards change | Ticket | Add project-specific rules once implementation stack is selected. |
| 7 | BL-007 | architecture | high-risk | Design Rust CLI and crate workspace | High | done | `docs/work/tickets/TICKET-001-rust-cli-workspace.md`, `docs/work/designs/DESIGN-001-rust-cli-workspace.md`, `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md` | Public contract/API, Runtime, Multi-domain | done | Rust workspace scaffold exists and verifies with cargo. |
| 8 | BL-008 | architecture | high-risk | Make CodeWiki skill-first and add repository installer | High | done | `skill/codewiki/SKILL.md`, `scripts/install-codewiki-skill.sh`, `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md` | Public contract/API, Runtime, Standards change | done | Skill is the product; Rust is a companion tool. |
| 9 | BL-009 | architecture | normal | Document runtime provider selection policy | High | done | `skill/codewiki/SKILL.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` | External system/provider, Runtime | done | Octocode is first-choice when needed; memory/indexing tools are trigger-gated. |
| 10 | BL-010 | architecture | normal | Standardize generated CodeWiki docs structure | High | done | `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`, `skill/codewiki/SKILL.md`, `docs/architecture/ARCHITECTURE.md` | Public contract/API, Data model, Standards change | done | `docs/codewiki/**` is the knowledge surface; `.codewiki/**` is the committed control plane. |
| 11 | BL-011 | skill | normal | Add CodeWiki mode prompt references | High | done | `skill/codewiki/SKILL.md`, `skill/codewiki/references/` | Public contract/API, Runtime | done | `SKILL.md` is now a compact router for docs-structure, init, sync, Q&A, and deep-research references. |
