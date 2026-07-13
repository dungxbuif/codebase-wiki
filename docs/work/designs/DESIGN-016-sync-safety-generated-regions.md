---
artifact_type: detail_design
id: DESIGN-016
status: ready
owner: ai
trace:
  ticket: docs/work/tickets/TICKET-016-sync-safety-generated-regions.md
  phase: PHASE-001
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  master_docs: [docs/architecture/ARCHITECTURE.md]
---

# Design: Sync Safety Generated Regions

## Proposed Approach

Generated docs are wrapped with:

```text
<!-- codewiki:generated:start -->
...
<!-- codewiki:generated:end -->
```

During sync:

- if a page has markers, replace only the generated region;
- preserve all text before and after the generated region;
- if a changed existing page has no markers, preserve it and record `preserved-human-owned-unmarked`;
- new pages are still created normally.

This keeps CodeWiki safe by default while allowing generated content to refresh.
