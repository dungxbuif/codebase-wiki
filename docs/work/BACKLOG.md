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
| 1 | BL-001 | phase | high-risk | Design complete CodeWiki foundation | High | done | `docs/work/phases/PHASE-001-codewiki-foundation.md`, `docs/work/tickets/TICKET-018-release-readiness.md` | Public contract/API, Data model, External system/provider, Multi-domain | done | Foundation production baseline is implemented and verified. |
| 2 | BL-002 | research | normal | Compare OpenWiki and deepwiki-open reference patterns | High | done | `references/openwiki`, `references/deepwiki-open`, `docs/work/research/REFERENCE-PROMPTS.md`, `skill/codewiki/references/` | Existing behavior, Weak proof | done | Prompt pattern analysis was folded into CodeWiki mode references. |
| 3 | BL-003 | architecture | high-risk | Specify config, storage, cache, and migration model | High | done | `docs/work/tickets/TICKET-003-config-storage-skeleton.md`, `docs/work/tickets/TICKET-004-sqlite-state-migrations.md`, `docs/work/tickets/TICKET-005-sqlite-executor-paths.md`, `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md` | Data model, Runtime, Existing behavior | done | Config/plan/AGENTS skeleton, SQLite migration registry, and sqlite3-backed executor/path resolver exist. |
| 4 | BL-004 | architecture | high-risk | Define semantic exploration and evidence model | High | done | `docs/requirements/SPEC.md`, `docs/work/tickets/TICKET-007-repo-detection-v1.md`, `docs/work/tickets/TICKET-008-wikiplan-evidence-models.md`, `docs/work/tickets/TICKET-009-canonical-docs-generator.md`, `docs/work/tickets/TICKET-012-semantic-exploration-v1.md`, `docs/work/tickets/TICKET-013-claim-persistence-v1.md`, `docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md`, `docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md`, `docs/work/tickets/TICKET-017-synthesis-pages-v1.md` | Public contract/API, External system/provider, Multi-domain | done | Detection, WikiPlan/evidence/claim models, canonical docs, semantic exploration, durable claim persistence, staleness, Q&A retrieval, sync safety, production evals, and synthesis pages exist. |
| 5 | BL-005 | integration | normal | Define runtime provider boundary | Medium | done | `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`, `skill/codewiki/SKILL.md` | External system/provider | done | Octocode is first-choice when needed; codebase-memory-mcp and CocoIndex are trigger-gated. |
| 6 | BL-006 | framework | normal | Finalize CodeWiki-specific Harness standards | Medium | done | `docs/standards/CODEWIKI.md`, `docs/work/tickets/TICKET-019-codewiki-standards-and-status.md` | Standards change | done | CodeWiki product/evidence/sync/tool standards documented. |
| 7 | BL-007 | architecture | high-risk | Design Rust CLI and crate workspace | High | done | `docs/work/tickets/TICKET-001-rust-cli-workspace.md`, `docs/work/designs/DESIGN-001-rust-cli-workspace.md`, `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md` | Public contract/API, Runtime, Multi-domain | done | Rust workspace scaffold exists and verifies with cargo. |
| 8 | BL-008 | architecture | high-risk | Make CodeWiki skill-first and add repository installer | High | done | `skill/codewiki/SKILL.md`, `scripts/install-codewiki-skill.sh`, `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md` | Public contract/API, Runtime, Standards change | done | Skill is the product; Rust is a companion tool. |
| 9 | BL-009 | architecture | normal | Document runtime provider selection policy | High | done | `skill/codewiki/SKILL.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` | External system/provider, Runtime | done | Octocode is first-choice when needed; memory/indexing tools are trigger-gated. |
| 10 | BL-010 | architecture | normal | Standardize generated CodeWiki docs structure | High | done | `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`, `skill/codewiki/SKILL.md`, `docs/architecture/ARCHITECTURE.md` | Public contract/API, Data model, Standards change | done | `docs/**` is the knowledge surface; `.agents/skills/codewiki/project/**` is the committed control plane. |
| 11 | BL-011 | skill | normal | Add CodeWiki mode prompt references | High | done | `skill/codewiki/SKILL.md`, `skill/codewiki/references/` | Public contract/API, Runtime | done | `SKILL.md` is now a compact router for docs-structure, init, sync, Q&A, and deep-research references. |
| 12 | BL-012 | feature | high-risk | Add sync compare/no-op skeleton | High | done | `docs/work/tickets/TICKET-010-sync-skeleton.md` | Public contract/API, Runtime | done | `codewiki sync [path]` updates stale generated outputs and no-ops when current. |
| 13 | BL-013 | architecture | high-risk | Support external workspaces and user source extension skills | High | done | `docs/work/tickets/TICKET-011-workspace-source-extensions.md`, `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`, `skill/codewiki/references/workspace-placement.md`, `skill/codewiki/references/source-extensions.md` | Public contract/API, External system/provider, Data model | done | Git remains default; non-Git sources are user-provided source skills, not bundled providers. |
| 14 | BL-014 | contract | normal | Move CodeWiki control plane into agent skill workspace | High | done | `docs/work/tickets/TICKET-025-agent-workspace-control-plane.md` | Public contract/API, Existing behavior | done | Project-local skill and CodeWiki config now live under `.agents/skills/codewiki`; `docs/**` remains the only generated docs surface. |
| 15 | BL-015 | contract | normal | Uppercase generated Markdown filenames | High | done | `docs/work/tickets/TICKET-026-uppercase-generated-markdown-filenames.md`, `docs/work/designs/DESIGN-026-uppercase-generated-markdown-filenames.md`, `docs/decisions/ADR-0007-uppercase-generated-markdown-filenames.md` | Public contract/API, Existing behavior | done | Generated Markdown basenames are uppercase; marker-owned lowercase pages migrate safely and human-owned pages remain untouched. |
| 16 | BL-016 | feature | normal | Discover and document code conventions | High | done | `docs/work/tickets/TICKET-027-code-conventions-documentation.md`, `docs/work/designs/DESIGN-027-code-conventions-documentation.md`, `docs/decisions/ADR-0008-code-conventions-documentation.md` | Public contract/API, Weak proof | done | Evidence-derived project/language/framework conventions are generated without core adapters or generic best-practice filler. |
| 17 | BL-017 | sync-safety | normal | Preserve manual documentation edits during sync | High | done | `docs/work/tickets/TICKET-028-preserve-manual-doc-edits.md`, `docs/work/designs/DESIGN-028-preserve-manual-doc-edits.md`, `docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md` | Public contract/API, Existing behavior, Data loss | done | Portable generated-body integrity detects manual edits and routes conflicts to LLM semantic reconciliation without overwriting current docs. |
