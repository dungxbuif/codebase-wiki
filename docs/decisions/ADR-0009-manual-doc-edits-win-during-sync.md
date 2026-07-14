---
artifact_type: adr
id: ADR-0009
status: accepted
owner: human
trace:
  phase: not_applicable
  requirements: [docs/requirements/SPEC.md]
  backlog_items: [BL-017]
  triggering_ticket: docs/work/tickets/TICKET-028-preserve-manual-doc-edits.md
  related:
    - docs/decisions/ADR-0005-codewiki-generated-docs-structure.md
    - docs/architecture/ARCHITECTURE.md
---

# ADR-0009: Manual Documentation Edits Win During Sync

## Status

Accepted on 2026-07-14 by direct human decision.

## Context

Generated documentation is a durable shared knowledge surface, not disposable output. Users may correct or enrich content between CodeWiki runs, including content inside a generated region. Region boundaries alone distinguish surrounding human text but cannot detect edits made within the region.

## Decision

Embed a deterministic integrity hash with each newly generated region. Sync may replace a marked body only when its current content matches the recorded generated baseline. If the body changed or no trustworthy hash exists, preserve the current page and route the conflict to LLM semantic reconciliation. Unmarked pages remain human-owned.

Human contributions take precedence over automatic regeneration. The LLM must merge current docs with refreshed evidence instead of restoring the previous machine wording.

## Alternatives

- Let marked regions always be machine-owned: rejected because it silently destroys manual corrections.
- Never update existing docs: rejected because it makes sync ineffective.
- Keep ownership history only in local state: rejected because ownership must survive machine, session, and model changes with the committed docs.

## Consequences

- New generated pages carry self-describing ownership integrity.
- Safe unchanged pages still refresh automatically.
- Manual edits become explicit reconciliation work instead of data loss.
- Legacy hashless regions require conservative preservation until reconciled once.
