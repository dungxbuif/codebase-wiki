---
artifact_type: detail_design
id: DESIGN-033
status: in_review
owner: ai
approval: approved
human_fields: [approval, constraints, scope_decisions]
ai_fields: [problem, context_loaded, brownfield_scope, proposed_approach, design_tradeoffs, architecture_overview, execution_flow, api_data_model, security, test_plan, reconciliation_plan]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  ticket_or_bug: docs/work/bugs/BUG-003-skill-can-bypass-init-gate.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
---

# Detail Design: Mandatory Skill Execution Gate

## Problem

The companion correctly refuses reader-doc success without synthesis artifacts, but the skill can be executed without calling the companion. The user's verified Mezon run proves prompt guidance alone is too permissive.

## Context Loaded

- User-provided `docs.zip` and generated Mezon `docs/**`.
- Installed package manifest/digest and source revision.
- Companion validation failure output.
- BUG-001, ADR-0010, DESIGN-030, reader-first reference, skill-creator guidance, and current helper behavior.

## Brownfield Scope

Touch only the CodeWiki skill entry contract, UI prompt, a bundled preflight wrapper, contract tests, and lifecycle docs. Reuse the existing companion `doctor`, `init`, `sync`, and `validate` commands.

## Proposed Approach

1. Add a low-freedom preflight wrapper accepting only `init|sync <repo>`.
2. The wrapper resolves the existing helper, invokes the companion, and verifies required control/evidence artifacts plus `synthesis_incomplete` state.
3. Put the wrapper command in a mandatory gate immediately after Core Rules.
4. State explicitly that no reader-facing Markdown may be created or modified before the gate succeeds.
5. Regenerate `agents/openai.yaml` with an explicit `$codewiki` default prompt naming preflight and validation.
6. Keep final semantic synthesis model-driven; the wrapper does not generate prose.

## Tradeoffs

- Stronger procedural wording cannot intercept an arbitrary filesystem write, but a one-command first gate materially reduces execution freedom and makes the valid path obvious.
- Adding a full autonomous prose generator to the companion would violate the skill-first/model-synthesis architecture and is rejected.
- Writing new root-level target `AGENTS.md` rules during installation is rejected because installation must not silently change a user's repository policy.

## Impact

- API/data/security: none.
- Runtime: one new shell wrapper around existing verified commands.
- Compatibility: existing helper commands remain unchanged.
- Docs: skill, init/sync references, phase, bug, validation, context, changelog.

## Verification

- Contract test red before implementation and green after.
- `bash -n` on helper, installer, and preflight.
- Clean Rust fixture preflight smoke.
- Skill `quick_validate.py` and regenerated `openai.yaml` inspection.
- Full workspace tests and scoped strict Clippy.

## Approval

Approved by the user's instruction to fix immediately on 2026-07-14.
