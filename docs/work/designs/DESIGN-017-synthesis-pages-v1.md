---
artifact_type: detail_design
id: DESIGN-017
status: ready
owner: ai
trace:
  ticket: docs/work/tickets/TICKET-017-synthesis-pages-v1.md
  phase: PHASE-001
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  master_docs: [docs/architecture/ARCHITECTURE.md]
---

# Design: Synthesis Pages V1

## Proposed Approach

Use the existing `ExplorationSnapshot` to generate deterministic, evidence-bound pages for the canonical CodeWiki slots:

- `domains.md`: top-level areas and role mix.
- `workflows.md`: entrypoints/tests/import hints as workflow seeds.
- `data.md`: config/schema/data-related files and explicit gaps.
- `interfaces.md`: exported symbols, public functions/classes/interfaces, and import hints.
- `operations.md`: package/build/config files and operational gaps.
- `testing.md`: detected tests and test-related files.
- `decisions.md`: current generated decisions and unresolved decision gaps.
- `glossary.md`: symbol/area glossary.
- `open-questions.md`: missing evidence and low-confidence areas.
- `areas/<area>.md`: area-level file/symbol/evidence summary.

All pages stay inside generated-region markers and are sync-safe.

## Risks

- Deterministic synthesis may sound too shallow. Mitigation: label pages as evidence summaries and point to gaps/open questions.
- Area pages can be noisy. Mitigation: create only for observed top-level areas and cap listed files.

## Verification

- Docs tests check canonical synthesis pages and generated markers.
- Production fixtures verify several pages exist and contain fixture evidence.
