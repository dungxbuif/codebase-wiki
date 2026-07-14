---
artifact_type: roadmap
id: ROADMAP
status: active
owner: human
human_fields:
  - milestones
  - priority
  - phase_order
ai_fields:
  - phase_links
  - status_summaries
shared_fields:
  - milestone_status
updated: 2026-07-13
---

# Roadmap

## Field Ownership

- Human owns milestones, priority, and phase order.
- AI maintains phase links and status summaries.

Use this file to group work into milestones or major phases.

## Roadmap Rules

- A roadmap item becomes executable only after it has a phase file in `docs/work/phases/`.
- Each phase should contain one or more tickets or bugs.
- Completed phases should link to release notes or changelog entries when relevant.

## Milestones

| Milestone | Goal | Status | Phase Files |
| --- | --- | --- | --- |
| M0 | Establish repository control harness and reference submodules | done | [PHASE-001-codewiki-foundation.md](phases/PHASE-001-codewiki-foundation.md) |
| M1 | Implement CodeWiki foundation: config, storage, detection, WikiPlan, and docs generation | done | [PHASE-001-codewiki-foundation.md](phases/PHASE-001-codewiki-foundation.md) |
| M2 | Implement sync, stale evidence detection, and docs-first Q&A context | done | [PHASE-001-codewiki-foundation.md](phases/PHASE-001-codewiki-foundation.md) |
| M3 | Package CodeWiki as a reusable Codex skill for arbitrary repositories | done | [PHASE-001-codewiki-foundation.md](phases/PHASE-001-codewiki-foundation.md) |
| M4 | Make generated docs sufficient and reproducible for developer onboarding through versioned skill integrity, reader-first planning, real LLM synthesis, diagrams, and quality evaluation | in_review | [PHASE-002-reader-first-docs-quality.md](phases/PHASE-002-reader-first-docs-quality.md) |
