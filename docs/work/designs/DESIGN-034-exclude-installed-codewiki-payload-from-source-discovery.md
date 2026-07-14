---
artifact_type: detail_design
id: DESIGN-034
status: in_review
owner: ai
approval: approved
human_fields: [approval, constraints, scope_decisions]
ai_fields: [problem, context_loaded, brownfield_scope, proposed_approach, design_tradeoffs, architecture_overview, execution_flow, api_data_model, security, test_plan, reconciliation_plan]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: PHASE-002
  ticket_or_bug: docs/work/bugs/BUG-004-installed-skill-self-contaminates-source-discovery.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [ADR-0010, ADR-0011]
  master_docs_touched: [docs/architecture/ARCHITECTURE.md, docs/standards/CODEWIKI.md]
---

# Detail Design: Exclude Installed CodeWiki Payload From Source Discovery

## Problem And Boundary

Project-local installation places managed skill files and copied companion source under `.agents/skills/codewiki/**`. Those files are CodeWiki runtime, not target-repository evidence. The existing filter excludes only `project/**`, allowing the rest of the installed payload to contaminate detection and semantic exploration.

## Proposed Approach

Change the shared path rule independently in `codewiki-detect` and `codewiki-explore` from `.agents/skills/codewiki/project` to `.agents/skills/codewiki`. Keep other `.agents/**` content visible because it may be legitimate repository guidance or user-provided skill evidence; only CodeWiki's own installed root is unconditionally excluded.

No Git ignore rule, source-extension contract, SQLite schema, WikiPlan schema, provider policy, or reader-page contract changes.

## Risks And Alternatives

- Ignoring all `.agents/**`: rejected because it can hide repository-owned agent instructions and unrelated source skills.
- Ignoring only `companion/**`: rejected because SKILL/references/manifests also polluted evidence.
- Filtering after persistence: rejected because false stack/entrypoint signals already affect the mental model scaffold.

Risk is narrow: a repository intentionally using `.agents/skills/codewiki/**` as product source would now be hidden, but that path is reserved by the installed CodeWiki product contract.

## Test And Reconciliation Plan

- Red/green unit tests in detector and explorer.
- Preserve the current-working-tree/untracked-source integration regression.
- Full workspace and scoped strict Clippy.
- Fresh project-local package install, doctor, preflight, and independent forward synthesis.
- Reconcile BUG-004, phase/backlog/context, validation matrix, test verification, docs review, architecture/standards no-change notes, and changelog.
