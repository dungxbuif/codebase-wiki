---
artifact_type: bug
id: BUG-003
status: in_review
owner: human
severity: high
priority: high
bug_markers: [reproduced, fixed, regression_verified]
human_fields: [symptoms, expected_behavior, priority, severity]
ai_fields: [reproduction, actual_behavior, root_cause, impact_scope, fix_strategy, regression_tests, verification_results]
shared_fields: [status, trace, docs_review]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  detail_design: docs/work/designs/DESIGN-033-mandatory-skill-execution-gate.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Bug: Skill Can Bypass The Init And Validation Gates

## Symptoms

The latest locally installed CodeWiki skill generated six Mezon reader pages directly, without creating `.agents/skills/codewiki/project/plan.yml`, `run.yml`, or `quality-report.yml`. The pages were crate/command summaries with no diagrams, no reader navigation, and one source contradiction, yet the generating agent presented them as the requested docs.

## Reproduction

1. Install CodeWiki package `0.2.0` from source revision `9f968de0410925e0b4edc28aebbffe568ab86bed` into Mezon Desktop.
2. Invoke the installed skill to generate docs.
3. Inspect `/Users/dungxbuif/workspace/mezon-desktop/docs.zip` and the repository.
4. Observe six reader pages and no project control directory.
5. Run `codewiki validate /Users/dungxbuif/workspace/mezon-desktop`; it fails for missing WikiPlan/run/quality artifacts, missing purpose/mental model, missing page contracts, and orphan pages.

Reproducibility: observed on the user's first forward run of the verified package.

## Expected Behavior

The first mutating action for init/sync must invoke the installed companion through a deterministic preflight. Reader docs must not be written until the control plane and evidence state exist. Completion must always end with companion validation and `reader_docs_ready`.

## Actual Behavior

`SKILL.md` describes the workflow but leaves the companion call in the middle of a high-freedom checklist. The UI default prompt asks only to “Generate or sync” and does not name the mandatory first/last commands. A model can therefore synthesize Markdown directly and never reach the validator.

## Root Cause

The runtime validator is correct but optional at the agent execution boundary. The skill provides no low-freedom preflight entrypoint and its default prompt does not force the command sequence.

Single hypothesis: making companion preflight the explicit first write gate in `SKILL.md`, the UI prompt, and a bundled script will prevent ordinary skill runs from treating direct Markdown generation as a valid CodeWiki execution.

## Impact Scope

- Skill: entry workflow, UI default prompt, bundled scripts, init/sync references.
- Companion: no behavior change required; existing init and validation remain authoritative.
- Users: outputs may look complete while lacking reproducibility, evidence, and onboarding quality.
- Source/data/security: no source-code or database migration impact.

## Fix Strategy

- Add `scripts/codewiki-preflight.sh init|sync <repo>` as the mandatory first mutating action.
- Put a concise mandatory execution gate near the top of `SKILL.md`.
- Make the UI default prompt name `$codewiki`, preflight, reader-first synthesis, and final validation.
- Require init/sync references to prohibit reader-doc writes before preflight.
- Add regression coverage for the packaged execution contract and smoke-test the preflight script.

## Regression Tests

- Red/green contract test requires the mandatory gate, bundled preflight, and explicit UI prompt.
- Shell syntax check for the new script.
- Clean-fixture preflight must create project/evidence artifacts and stop at `synthesis_incomplete` without reader pages.
- Existing full workspace, installer, and pinned validation tests remain green.

## Verification Results

- Red: `rtk cargo test -p codewiki-core --test skill_execution_contract` failed because the mandatory gate was absent.
- Green: the same contract test passes after adding the gate, preflight, and explicit UI prompt.
- Clean-fixture init preflight created six control files and four evidence pages, recorded source provenance, and stopped at `synthesis_incomplete` without reader pages.
- Clean-fixture sync preflight completed as a no-op while preserving the incomplete state.
- Package version advanced to `0.2.1` so installations can distinguish the execution-gate fix.

## Attempt Log

- Attempt 1: contract test red, then mandatory gate/preflight/UI prompt implemented; targeted test and preflight smoke pass.
- Same-path attempts: 1 / 3
- Total fix/test cycles: 1 / 5
