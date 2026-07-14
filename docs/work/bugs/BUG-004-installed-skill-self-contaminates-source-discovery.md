---
artifact_type: bug
id: BUG-004
status: in_review
owner: shared
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
  detail_design: docs/work/designs/DESIGN-034-exclude-installed-codewiki-payload-from-source-discovery.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md, docs/decisions/ADR-0011-skill-distribution-version-integrity.md]
  release_notes: docs/releases/CHANGELOG.md
---

# Bug: Installed CodeWiki Skill Self-Contaminates Source Discovery

## Symptoms

A clean forward test installed CodeWiki project-locally into a JavaScript repository. Companion preflight then reported Cargo and `.agents/skills/codewiki/companion/Cargo.toml` as target-repository signals. `docs/evidence/SOURCES.md` and `CLAIMS.md` contained CodeWiki's own skill references, manifest, helper payload, and copied Rust companion source.

## Reproduction

1. Install package `0.2.2` project-locally under `.agents/skills/codewiki` in a non-Rust target repository.
2. Run the installed `codewiki-preflight.sh init <target>`.
3. Inspect `plan.yml`, `docs/evidence/SOURCES.md`, and `docs/evidence/CLAIMS.md`.
4. Observe false Cargo/Rust signals and evidence paths under `.agents/skills/codewiki/**`.

Expected: current target code, including relevant uncommitted/untracked source, is explored; the installed CodeWiki runtime payload is excluded.

Actual: only `.agents/skills/codewiki/project/**` is excluded, so sibling managed skill/reference/companion paths enter detection and semantic evidence.

## Root Cause

`codewiki-detect::is_generated_codewiki_path` and `codewiki-explore::is_generated_codewiki_path` exclude the project control directory instead of the complete installed skill root. The project-local installer intentionally copies Rust companion source into that root, making self-contamination deterministic.

Single hypothesis: excluding `.agents/skills/codewiki/**` at both deterministic traversal boundaries will preserve current-working-tree exploration while preventing CodeWiki from documenting itself as part of the target.

## Impact Scope

- Detection: false language, package-manager, framework, entrypoint, test, and docs signals.
- Evidence/state: irrelevant files, symbols, claims, and provider-independent snapshots are persisted.
- Reader synthesis: mental models and page plans may describe Cargo/Rust or CodeWiki internals in unrelated repositories.
- Source/data/security: target source is not modified; generated evidence and local state are polluted.

## Fix Strategy

- Exclude the full `.agents/skills/codewiki/**` subtree in both detector and semantic explorer.
- Preserve exploration of other target source, including relevant untracked files.
- Add detector/explorer regression fixtures with installed skill and companion payloads.
- Repeat a clean installed-package forward test; do not reuse contaminated evidence state.

## Regression Tests

- Detector fixture must ignore installed `SKILL.md` and companion `Cargo.toml`/Rust source.
- Semantic exploration fixture must emit no evidence path under `.agents/skills/codewiki/**`.
- Existing current-working-tree test must still persist untracked target source.
- Full workspace, strict scoped Clippy, installer doctor, and fresh forward init must pass.

## Verification Results

- Reproduction: confirmed in `/private/tmp/codewiki-forward-022`; the plan reported Cargo and evidence contained the installed package.
- Red: detector and explorer regressions failed because installed skill/companion files were discovered.
- Green: both regressions pass after excluding the full managed skill root before recursive traversal.
- Current-working-tree regression still persists modified/untracked target source and records `source_dirty: true`.
- Fresh package `0.2.2` preflight in `/private/tmp/codewiki-forward-022b` detects only JavaScript/npm, persists `src/audit.js`, and contains no `.agents/skills/codewiki/**`, Cargo, or Rust evidence.
- Full workspace: 50 tests across 15 suites pass; scoped strict Clippy for core/detect/explore passes.
- Independent package `0.2.2` forward synthesis reached `reader_docs_ready` with eight validated reader pages. Its mental model includes the dirty/untracked audit path and contains no installed-skill/Cargo/Rust target claims. Human Mezon/package UAT remains the phase completion gate.

## Fix/Test Attempt Log

- Attempt 1: added detector/explorer installed-payload regressions; both failed red, then passed after excluding the managed skill root before recursive traversal.
- Same-path failure attempts: 1 / 3.
- Total fix/test cycles: 1 / 5.
- Blocked by loop guard: no.

## Completion Checklist

- [x] Root cause identified.
- [x] Fix implemented.
- [x] Regression tests added and observed red/green.
- [x] Current-working-tree behavior retested.
- [x] Fresh installed-package preflight verified.
- [x] Validation matrix, architecture, standards, context, backlog, phase, docs review, and changelog reconciled.
- [x] Independent clean-fixture reader synthesis reached `reader_docs_ready`.
- [ ] Human Mezon/package UAT remains a phase-level gate.
