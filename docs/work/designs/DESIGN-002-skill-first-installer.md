---
artifact_type: detail_design
id: DESIGN-002
status: done
owner: ai
approval: approved
human_fields:
  - approval
  - constraints
  - scope_decisions
ai_fields:
  - problem
  - context_loaded
  - brownfield_scope
  - proposed_approach
  - design_tradeoffs
  - architecture_overview
  - execution_flow
  - api_data_model
  - security
  - test_plan
  - reconciliation_plan
shared_fields:
  - status
  - trace
  - small_task_exemption
trace:
  backlog_item: BL-008
  requirement: REQ-007
  phase: PHASE-001
  ticket_or_bug: docs/work/tickets/TICKET-002-skill-first-installer.md
  test_verification: docs/work/tickets/TICKET-002-skill-first-installer.md#verification-results
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/tickets/TICKET-002-skill-first-installer.md#docs-review
  adrs: [docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md]
  master_docs_touched:
    - README.md
    - docs/architecture/ARCHITECTURE.md
    - docs/architecture/API.md
    - docs/requirements/SPEC.md
    - docs/requirements/REQUIREMENTS.md
---

# DETAIL DESIGN: Skill-First Installer

## Status

- ID: DESIGN-002
- Status: done
- Ticket/Bug: `docs/work/tickets/TICKET-002-skill-first-installer.md`
- Approval: approved by direct user instruction
- Author: Codex
- Updated: 2026-07-13

## 1. Context & Scope

### Problem Statement

- Problem statement: CodeWiki must be a reusable Codex skill, not primarily a CLI product.
- Why now: The user clarified that Rust is only a companion tool and requested a repository-based one-command installer.
- Success outcome: The repo contains a skill package and a script that installs it into `$CODEX_HOME/skills/codewiki`.

### Context Loaded

- `docs/CONTEXT.md`
- `docs/work/BACKLOG.md`
- `docs/standards/README.md`
- `docs/standards/QUALITY_BAR.md`
- `docs/standards/VALIDATION.md`
- `docs/work/phases/PHASE-001-codewiki-foundation.md`
- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- Skill creator guidance for Codex skills.

### Brownfield Scope

- Touched modules/files: `skill/codewiki/**`, `scripts/install-codewiki-skill.sh`, master docs, ADRs, ticket/design docs.
- Direct dependencies inspected: current docs and existing Rust scaffold.
- Contracts affected: skill install path and one-command installer.
- Known unknowns: final marketplace/distribution packaging beyond `$CODEX_HOME/skills`.
- Scope expansion reason: direct user request changed product direction.

### Small Task Exemption

- Small task exemption: no
- Reason: This changes product architecture and install contract.
- Impact checked: API=yes, DB=no, Security=no, Runtime=yes, Standards=no

## 2. Design Considerations & Trade-offs

| Consideration / Alternative | Pros | Cons | Decision |
| --- | --- | --- | --- |
| Keep Rust CLI primary | Existing scaffold stays conceptually simple | Conflicts with user goal | rejected |
| Skill-only with no Rust | Product direction is clean | Loses deterministic companion tooling | rejected |
| Skill-first with Rust companion | Matches user goal and keeps robust local helper path | Requires docs correction | chosen |

## 3. Architecture Overview

| Component | Role |
| --- | --- |
| `skill/codewiki/SKILL.md` | Primary product behavior and agent workflow |
| `skill/codewiki/agents/openai.yaml` | Skill UI metadata |
| `scripts/install-codewiki-skill.sh` | Installs the skill from the repo into Codex home |
| `crates/**` | Companion Rust tool workspace for deterministic support |

## 4. Execution Flow

1. User runs installer command from the repository or via raw GitHub script.
2. Installer clones the CodeWiki repository.
3. Installer copies `skill/codewiki` into `$CODEX_HOME/skills/codewiki`.
4. Future Codex sessions can trigger the CodeWiki skill by name or task description.

## 5. API & Data Model Design

- Install command: `scripts/install-codewiki-skill.sh`
- Remote default: `git@github.com:dungxbuif/harness.git`
- Install target: `$CODEX_HOME/skills/codewiki`
- Data model changes: none.

## 6. Security & Authorization

- Authentication changes: none.
- Authorization / Permissions: writes to `$CODEX_HOME/skills`.
- Data Privacy / PII impact: none.
- Input Validation: installer verifies `skill/codewiki/SKILL.md` exists before copying.

## 7. Implementation & Verification Plan

- Unit/static: `rtk bash -n scripts/install-codewiki-skill.sh`
- Platform/manual: `rtk git remote -v`
- Docs review: verify skill-first language and ADR supersession.

### Verification Results

Filled in ticket after execution.

## 8. Reconciliation Plan

- Requirements docs: update.
- Architecture docs: update.
- API docs: update.
- ERD docs: no change.
- ADR: create ADR-0003 and supersede the relevant part of ADR-0002.
- Context: update.

