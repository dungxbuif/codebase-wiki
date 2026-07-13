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
| M0 | Establish repository control harness and reference submodules | active | [PHASE-001-codewiki-foundation.md](phases/PHASE-001-codewiki-foundation.md) |
| M1 | Implement CodeWiki foundation: config, storage, detection, WikiPlan, and docs generation | planned | TBD |
| M2 | Implement sync, stale evidence detection, and docs-first Q&A | planned | TBD |
| M3 | Package CodeWiki as a reusable Codex skill for arbitrary repositories | planned | TBD |
