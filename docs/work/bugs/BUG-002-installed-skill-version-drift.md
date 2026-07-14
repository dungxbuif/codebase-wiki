---
artifact_type: bug
id: BUG-002
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
  requirements: [REQ-007, REQ-014, REQ-015]
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  detail_design: docs/work/designs/DESIGN-032-skill-install-version-integrity.md
  research: docs/work/research/GROK-WIKI-MEZON-AUDIT.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs:
    - docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md
    - docs/decisions/ADR-0011-skill-distribution-version-integrity.md
  release_notes: docs/releases/CHANGELOG.md
---

# Bug: Installed Skill Version Drift Is Undetectable

## Status

- Status: in_review
- Bug markers: reproduced, fixed, regression_verified
- Severity: high
- Priority: high
- Fix approval: approved by the user on 2026-07-14

## Symptoms

User-observed quality varies between runs even when frontier models are used, and the invoked CodeWiki skill cannot be assumed to match the current repository contract.

## Reproduction

1. Compare repository source `skill/codewiki/**` with the active global installation `/Users/dungxbuif/.codex/skills/codewiki/**`.
2. Observe differences in `SKILL.md` and most references.
3. Inspect key contract lines:
   - installed skill uses `.codewiki/**`; source uses `.agents/skills/codewiki/project/**`;
   - installed skill describes lowercase generated files; source requires uppercase Markdown basenames;
   - installed skill lacks `references/conventions.md`;
   - both inspected copies historically retained the source-list-first rule later superseded by ADR-0010.
4. Inspect `scripts/install-codewiki-skill.sh`: it copies/builds the package but writes no installed source revision, content digest, skill contract version, or companion compatibility metadata.
5. Reproducibility: always for the inspected installation.

## Expected Behavior

The active CodeWiki skill and bundled companion should have inspectable provenance and compatible contract versions. A generation run and benchmark should record exactly which installed skill content and companion build were used. Drift should be reported before generation, not inferred from output after failure.

## Historical Actual Behavior

The installed skill is materially older than repository source, but neither the installer nor runtime reports that fact. The package contains additional expected `bin/` and `companion/` payloads, yet there is no manifest distinguishing managed source files, generated payloads, project state, or their versions.

## Root Cause

- The installer is copy-based and has no package/install manifest.
- The skill contract, prompt/reference contract, companion build, and repository revision are not versioned together.
- Runtime status does not record the resolved skill root or verify compatibility/content integrity.
- Benchmarks record model/source variables but cannot currently pin the actual skill artifact.

Single hypothesis: missing package provenance and compatibility checks allow stale skill instructions to participate in generation unnoticed, causing output and storage contracts to vary independently of the selected model.

## Impact Scope

- Skill: all mode references can drift from repository source.
- Companion: bundled binary/source can drift from the instructions that invoke it.
- Users: results are not reproducible across machines, install scopes, or reinstall times.
- Benchmarks: model comparisons can accidentally compare different product contracts.
- Project state: old `.codewiki/**` and new `.agents/**` control planes can coexist or be selected inconsistently.

## Fix Strategy

- Add a source package manifest and install-time provenance manifest with content digests and contract versions.
- Make installation atomic and preserve only explicitly user-owned project state.
- Make helper/status/doctor report active skill root, source revision, skill/reference contract versions, companion version, and compatibility result.
- Record the same provenance in every WikiPlan/generation/benchmark run.
- Detect and clearly report source-vs-installed drift when a source checkout is available; never auto-update without user authorization.
- Add explicit legacy-control-plane detection/migration guidance.

## Regression Tests

- Install from a pinned local source into a temporary local/global root and verify managed skill files match the manifest.
- Mutate one installed reference and verify doctor/status reports content drift.
- Bundle an incompatible companion contract version and verify generation cannot report success.
- Preserve project-local human state while replacing managed skill/reference/bin/companion payloads.
- Record installed skill identity in a benchmark result.

## Verification Results

- Reproduction: `diff -rq` reports drift across `SKILL.md` and most references.
- Contract spot check: old/new control-plane and filename rules are confirmed by targeted search.
- Installer inspection: no package version, revision, digest, or compatibility manifest exists.
- Fix verification: package/install manifests, managed digest, interface/schema compatibility, doctor/package-digest commands, helper preflight, run provenance, staged activation, and ownership-conflict checks are implemented.
- Regression verification: doctor unit test detects a deliberate managed-reference mutation; temporary local install reports `verified` with package/contract/schema identity; installed helper init records the same verified identity in `run.yml`.

## Fix/Test Attempt Log

- Same-path failure attempts: 0 / 3
- Total fix/test cycles: 2 / 5
- Blocked by loop guard: no
- Human/design input needed: none for the implemented fix; global reinstall/push remains a delivery action.

## Docs Review

- Requirements: REQ-007 installability, REQ-014 reproducible quality, and REQ-015 distribution integrity are affected.
- Architecture/standards: reconcile package/runtime provenance after approval and implementation.
- API/ERD/data: no external API or product data change; local manifest schema is introduced.
- Context/backlog/phase/validation: linked in PHASE-002 planning artifacts.

## Completion Checklist

- [x] Root cause identified
- [x] Fix implemented
- [x] Regression test added or updated
- [x] Impacted behavior retested
- [x] Fix/test loop guard respected
- [x] Validation matrix has planned regression coverage
- [x] Master docs reconciled after implementation
- [x] Docs review completed
- [x] `docs/CONTEXT.md` updated
- [x] `docs/work/BACKLOG.md` updated
- [x] Trace links updated
