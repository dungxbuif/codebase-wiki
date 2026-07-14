---
artifact_type: ticket
id: TICKET-028
status: done
owner: human
priority: high
lane: normal
trace:
  backlog_item: BL-017
  requirement: REQ-013
  phase: not_applicable
  detail_design: docs/work/designs/DESIGN-028-preserve-manual-doc-edits.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: Preserve Manual Documentation Edits

## Problem

CodeWiki already preserves text outside generated-region markers, but the companion sync currently replaces the entire marked region. A user may improve or correct generated content inside that region between sync runs. Silent replacement would lose authoritative human context.

## Acceptance Criteria

- [x] Newly generated regions record an integrity hash for the last CodeWiki-owned body.
- [x] Sync updates a generated region only when its current body still matches that recorded baseline.
- [x] Manual edits inside a generated region are preserved and reported as a merge conflict for LLM reconciliation.
- [x] Manual text outside an unchanged generated region remains preserved while the generated region refreshes.
- [x] Unmarked pages and legacy hashless generated regions are preserved conservatively.
- [x] Skill instructions require reading and semantically merging current docs rather than regenerating whole pages.
- [x] Tests and master docs prove the behavior.

## Approval

Approved directly by the user on 2026-07-14.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test`
- Smoke init and sync with manual edits inside and outside a generated region.
- Skill package validation or documented fallback.

## Verification Results

- `rtk cargo fmt --all --check`: passed.
- `rtk cargo test`: passed, 41 tests across 14 suites.
- Integration command-path regression proves an unchanged hashed body refreshes with surrounding human text preserved, then an inside-region manual edit produces `preserved-human-edited-generated-region` and remains byte-for-byte unchanged.
- Unit regression proves legacy hashless regions and unmarked pages are preserved.
- Init/sync smoke at `/private/tmp/codewiki-manual-docs-smoke.TCBs9h` proved a generated hash was emitted and a human preface outside the region survived refresh. The temporary workspace was removed by the environment before a second manual step; the same inside-region path is covered by the command-level integration test.
- Skill creator `quick_validate.py`: unavailable because the local Python runtime lacks PyYAML; equivalent `SKILL.md` and `agents/openai.yaml` YAML parsing with Ruby/Psych passed.

UAT passed through the command-path behavior assertions. Docs review passed for requirements, architecture, API, standards, ADRs, traceability, validation, context, backlog, and changelog. No ERD, database migration, provider, security, or source-extension change was required.
