---
artifact_type: traceability_matrix
id: TRACEABILITY
status: active
owner: shared
human_fields:
  - requirement_source
  - release_scope
ai_fields:
  - trace_links
  - evidence_links
  - adr_links
shared_fields:
  - matrix_rows
updated: 2026-07-14
---

# Traceability

## Field Ownership

- Human owns requirement source and release scope.
- AI maintains trace links, evidence links, and ADR links during reconciliation.

Use this file to map requirements to execution, verification, decisions, and releases.

## Trace Matrix

| Requirement | Phase | Ticket/Bug | Detail Design | Test Verification | Docs Review | ADR/Docs | Release |
| --- | --- | --- | --- | --- | --- | --- | --- |
| REQ-001 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-006-init-skeleton.md`, `docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md` | `docs/work/designs/DESIGN-006-init-skeleton.md`, `docs/work/designs/DESIGN-015-production-fixtures-eval-suite.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-018-release-readiness.md` | `docs/requirements/SPEC.md`, `docs/architecture/ARCHITECTURE.md` | `docs/releases/CHANGELOG.md` |
| REQ-002 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-007-repo-detection-v1.md`, `docs/work/tickets/TICKET-012-semantic-exploration-v1.md`, `docs/work/tickets/TICKET-015-production-fixtures-eval-suite.md` | `docs/work/designs/DESIGN-007-repo-detection-v1.md`, `docs/work/designs/DESIGN-012-semantic-exploration-v1.md`, `docs/work/designs/DESIGN-015-production-fixtures-eval-suite.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-018-release-readiness.md` | `docs/architecture/ARCHITECTURE.md` | `docs/releases/CHANGELOG.md` |
| REQ-003 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-003-config-storage-skeleton.md` | `docs/work/designs/DESIGN-003-config-storage-skeleton.md` | `docs/work/tickets/TICKET-003-config-storage-skeleton.md#verification-results` | `docs/work/tickets/TICKET-003-config-storage-skeleton.md#docs-review` | `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` | `docs/releases/CHANGELOG.md` |
| REQ-004 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-008-wikiplan-evidence-models.md`, `docs/work/tickets/TICKET-009-canonical-docs-generator.md`, `docs/work/tickets/TICKET-013-claim-persistence-v1.md`, `docs/work/tickets/TICKET-014-staleness-qa-retrieval-v1.md`, `docs/work/tickets/TICKET-016-sync-safety-generated-regions.md`, `docs/work/tickets/TICKET-017-synthesis-pages-v1.md` | `docs/work/designs/DESIGN-008-wikiplan-evidence-models.md`, `docs/work/designs/DESIGN-009-canonical-docs-generator.md`, `docs/work/designs/DESIGN-013-claim-persistence-v1.md`, `docs/work/designs/DESIGN-014-staleness-qa-retrieval-v1.md`, `docs/work/designs/DESIGN-016-sync-safety-generated-regions.md`, `docs/work/designs/DESIGN-017-synthesis-pages-v1.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-018-release-readiness.md` | `docs/requirements/SPEC.md`, `docs/architecture/ARCHITECTURE.md`, `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md` | `docs/releases/CHANGELOG.md` |
| REQ-005 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-018-release-readiness.md` | not_required | `docs/work/VALIDATION_MATRIX.md` | `skill/codewiki/SKILL.md` | `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` | `docs/releases/CHANGELOG.md` |
| REQ-006 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-001-rust-cli-workspace.md`, `docs/work/tickets/TICKET-021-binary-first-companion-install.md`, `docs/work/tickets/TICKET-023-reference-baseline-and-source-provider-status.md`, `docs/work/tickets/TICKET-024-final-foundation-closure.md` | `docs/work/designs/DESIGN-001-rust-cli-workspace.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-024-final-foundation-closure.md` | `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`, `docs/architecture/API.md` | `docs/releases/CHANGELOG.md` |
| REQ-007 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-002-skill-first-installer.md`, `docs/work/tickets/TICKET-020-direct-docs-root-and-skill-helper.md`, `docs/work/tickets/TICKET-021-binary-first-companion-install.md`, `docs/work/tickets/TICKET-024-final-foundation-closure.md`, `docs/work/tickets/TICKET-025-agent-workspace-control-plane.md` | `docs/work/designs/DESIGN-002-skill-first-installer.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-025-agent-workspace-control-plane.md` | `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`, `docs/architecture/API.md` | `docs/releases/CHANGELOG.md` |
| REQ-008 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-018-release-readiness.md` | not_required | `docs/work/VALIDATION_MATRIX.md` | `skill/codewiki/SKILL.md` | `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`, `docs/architecture/ARCHITECTURE.md` | `docs/releases/CHANGELOG.md` |
| REQ-009 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-020-direct-docs-root-and-skill-helper.md`, `docs/work/tickets/TICKET-022-openwiki-deepwiki-docs-patterns.md`, `docs/work/tickets/TICKET-024-final-foundation-closure.md`, `docs/work/tickets/TICKET-025-agent-workspace-control-plane.md`, `docs/work/tickets/TICKET-026-uppercase-generated-markdown-filenames.md` | `docs/work/designs/DESIGN-026-uppercase-generated-markdown-filenames.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-026-uppercase-generated-markdown-filenames.md` | `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`, `docs/decisions/ADR-0007-uppercase-generated-markdown-filenames.md`, `skill/codewiki/references/docs-structure.md` | `docs/releases/CHANGELOG.md` |
| REQ-010 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-010-sync-skeleton.md`, `docs/work/tickets/TICKET-016-sync-safety-generated-regions.md`, `docs/work/tickets/TICKET-024-final-foundation-closure.md` | `docs/work/designs/DESIGN-010-sync-skeleton.md`, `docs/work/designs/DESIGN-016-sync-safety-generated-regions.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-024-final-foundation-closure.md` | `docs/architecture/ARCHITECTURE.md`, `skill/codewiki/references/sync.md` | `docs/releases/CHANGELOG.md` |
| REQ-011 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-011-workspace-source-extensions.md`, `docs/work/tickets/TICKET-023-reference-baseline-and-source-provider-status.md`, `docs/work/tickets/TICKET-024-final-foundation-closure.md`, `docs/work/tickets/TICKET-025-agent-workspace-control-plane.md` | `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-025-agent-workspace-control-plane.md` | `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`, `skill/codewiki/references/source-extensions.md` | `docs/releases/CHANGELOG.md` |
| REQ-012 | not_applicable | `docs/work/tickets/TICKET-027-code-conventions-documentation.md` | `docs/work/designs/DESIGN-027-code-conventions-documentation.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-027-code-conventions-documentation.md` | `docs/decisions/ADR-0008-code-conventions-documentation.md`, `skill/codewiki/references/conventions.md` | `docs/releases/CHANGELOG.md` |
| REQ-013 | not_applicable | `docs/work/tickets/TICKET-028-preserve-manual-doc-edits.md` | `docs/work/designs/DESIGN-028-preserve-manual-doc-edits.md` | `docs/work/VALIDATION_MATRIX.md` | `docs/work/tickets/TICKET-028-preserve-manual-doc-edits.md` | `docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md`, `skill/codewiki/references/sync.md` | `docs/releases/CHANGELOG.md` |

## Rules

- Every phase should link to at least one requirement, ticket, or bug.
- Every ticket or bug should link to test evidence.
- Every durable decision should link to an ADR.
- Every release should link to completed phases, tickets, bugs, and verification.
