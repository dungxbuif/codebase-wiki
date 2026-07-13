---
artifact_type: ticket
id: TICKET-027
status: done
owner: human
priority: high
lane: normal
trace:
  backlog_item: BL-016
  requirement: REQ-012
  phase: not_applicable
  detail_design: docs/work/designs/DESIGN-027-code-conventions-documentation.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0008-code-conventions-documentation.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Ticket: Code Conventions Documentation

## Problem

Every repository expresses project-, language-, and framework-level conventions through configuration, repeated code patterns, tests, and exceptions. CodeWiki currently documents architecture, domains, workflows, APIs, operations, and testing but does not require the LLM to discover and preserve these conventions for future humans and agents.

## Acceptance Criteria

- [x] Add canonical `docs/conventions/OVERVIEW.md` to init, sync, WikiPlan, navigation, and docs-first Q&A.
- [x] Require the LLM to infer conventions from repository evidence rather than generic language/framework advice.
- [x] Distinguish explicit, inferred, hypothesis, and exception evidence.
- [x] Require repeated independent examples for inferred conventions or mark them as hypotheses.
- [x] Cover project structure, language idioms, framework usage, naming, errors, async/state/data, dependencies, APIs, testing, configuration, security, and documentation only when evidence exists.
- [x] Exclude generated conventions docs from future source detection/exploration.
- [x] Tests and master docs prove the new contract.

## Approval

Approved directly by the user on 2026-07-14.

## Verification Plan

- `rtk cargo fmt --all --check`
- `rtk cargo test`
- Smoke init verifies `docs/conventions/OVERVIEW.md` and WikiPlan registration.
- Skill package validation or documented fallback.

## Verification Results

- `rtk cargo fmt --all --check`: passed.
- `rtk cargo test`: passed, 38 tests across 14 suites.
- Init smoke at `/private/tmp/codewiki-conventions-smoke.5AE7jG`: passed; generated `docs/conventions/OVERVIEW.md` and registered `slot: conventions` in WikiPlan.
- Skill creator `quick_validate.py`: unavailable because the local Python runtime lacks PyYAML; equivalent frontmatter and `agents/openai.yaml` parsing with Ruby/Psych passed.

UAT is not required because this is a deterministic generated-page and skill-contract change covered by integration smoke verification. Docs review passed for requirements, architecture, API, standards, ADRs, traceability, validation, context, backlog, and changelog.
