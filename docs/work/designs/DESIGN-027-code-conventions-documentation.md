---
artifact_type: detail_design
id: DESIGN-027
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-016
  requirement: REQ-012
  phase: not_applicable
  ticket_or_bug: docs/work/tickets/TICKET-027-code-conventions-documentation.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0008-code-conventions-documentation.md]
  master_docs_touched:
    - docs/requirements/SPEC.md
    - docs/architecture/ARCHITECTURE.md
    - docs/architecture/API.md
---

# Detail Design: Code Conventions Documentation

## Approach

Add `docs/conventions/OVERVIEW.md` as a canonical semantic page. The Rust companion seeds explicit convention sources and repeated structural/dependency candidates; the skill requires the LLM to inspect representative code, confirm scope, find counterexamples, and synthesize actionable conventions.

## Evidence Model

Each convention records:

- scope: repository, language, framework/library, subsystem, or test code;
- status: explicit, inferred, hypothesis, or exception;
- evidence: config path, source symbol/range, test, existing docs, or command;
- confidence;
- exceptions and change-safety impact.

An explicit configuration rule may stand alone. An inferred convention needs at least two independent supporting examples; otherwise it remains a hypothesis. Framework defaults are not project conventions unless repository evidence shows adoption.

## Implementation Scope

- Add a concise `references/conventions.md` exploration contract and route it from the skill.
- Extend canonical layout, init, sync, Q&A, and docs semantics.
- Generate a convention-evidence seed page from configs/docs/tests, repeated dependency hints, and repository areas without claiming that candidates are confirmed conventions.
- Register the page in WikiPlan and relevant-source selection.
- Exclude `docs/conventions/**` from source detection/exploration.
- Add unit, fixture, and smoke proof.

## Alternatives

- Put conventions inside architecture: rejected because conventions are cross-cutting and directly guide future code changes.
- Generate generic language best practices: rejected because they may contradict the repository.
- Require language adapters: rejected by the adapter-free core constraint.

## Risks

- A repeated anti-pattern may be mistaken for an intended convention. Mitigate with counterexample search, confidence, and explicit/inferred labels.
- Small repositories may not have enough repeated evidence. Preserve hypotheses and gaps instead of inventing rules.
- Deterministic lexical candidates are not final conventions. Label them as exploration seeds only.

## Reconciliation

Update requirements, architecture, API, ADR-0005 structure, ADR-0008, skill references, validation, traceability, context, backlog, and changelog. No storage migration, provider, security, or source-extension contract changes.
