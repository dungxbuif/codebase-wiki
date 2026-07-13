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
updated: 2026-07-13
---

# Traceability

## Field Ownership

- Human owns requirement source and release scope.
- AI maintains trace links, evidence links, and ADR links during reconciliation.

Use this file to map requirements to execution, verification, decisions, and releases.

## Trace Matrix

| Requirement | Phase | Ticket/Bug | Detail Design | Test Verification | Docs Review | ADR/Docs | Release |
| --- | --- | --- | --- | --- | --- | --- | --- |
| REQ-001 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | TBD | TBD | `docs/work/VALIDATION_MATRIX.md` | TBD | `docs/requirements/SPEC.md` | TBD |
| REQ-002 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | TBD | TBD | `docs/work/VALIDATION_MATRIX.md` | TBD | `docs/architecture/ARCHITECTURE.md` | TBD |
| REQ-003 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-003-config-storage-skeleton.md` | `docs/work/designs/DESIGN-003-config-storage-skeleton.md` | `docs/work/tickets/TICKET-003-config-storage-skeleton.md#verification-results` | `docs/work/tickets/TICKET-003-config-storage-skeleton.md#docs-review` | `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`, `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md` | `docs/releases/CHANGELOG.md` |
| REQ-004 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | TBD | TBD | `docs/work/VALIDATION_MATRIX.md` | TBD | `docs/requirements/SPEC.md` | TBD |
| REQ-005 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | TBD | TBD | `docs/work/VALIDATION_MATRIX.md` | TBD | `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md` | TBD |
| REQ-006 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-001-rust-cli-workspace.md` | `docs/work/designs/DESIGN-001-rust-cli-workspace.md` | `docs/work/tickets/TICKET-001-rust-cli-workspace.md#verification-results` | `docs/work/tickets/TICKET-001-rust-cli-workspace.md#docs-review` | `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`, `docs/architecture/API.md` | `docs/releases/CHANGELOG.md` |
| REQ-007 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | `docs/work/tickets/TICKET-002-skill-first-installer.md` | `docs/work/designs/DESIGN-002-skill-first-installer.md` | `docs/work/tickets/TICKET-002-skill-first-installer.md#verification-results` | `docs/work/tickets/TICKET-002-skill-first-installer.md#docs-review` | `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`, `docs/architecture/API.md` | `docs/releases/CHANGELOG.md` |
| REQ-008 | `docs/work/phases/PHASE-001-codewiki-foundation.md` | TBD | TBD | `docs/work/VALIDATION_MATRIX.md` | `skill/codewiki/SKILL.md` | `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`, `docs/architecture/INTEGRATIONS.md` | `docs/releases/CHANGELOG.md` |

## Rules

- Every phase should link to at least one requirement, ticket, or bug.
- Every ticket or bug should link to test evidence.
- Every durable decision should link to an ADR.
- Every release should link to completed phases, tickets, bugs, and verification.
