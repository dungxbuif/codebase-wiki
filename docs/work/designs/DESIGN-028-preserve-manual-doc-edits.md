---
artifact_type: detail_design
id: DESIGN-028
status: done
owner: ai
approval: approved
trace:
  backlog_item: BL-017
  requirement: REQ-013
  phase: not_applicable
  ticket_or_bug: docs/work/tickets/TICKET-028-preserve-manual-doc-edits.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  adrs: [docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md]
  master_docs_touched:
    - docs/requirements/SPEC.md
    - docs/architecture/ARCHITECTURE.md
    - docs/architecture/API.md
---

# Detail Design: Preserve Manual Documentation Edits

## Approach

Keep the existing generated-region boundary and add a deterministic hash marker immediately inside it. The hash represents the normalized generated body written by CodeWiki. Before replacement, sync compares the current body with the recorded hash.

- Matching hash: the body remains CodeWiki-owned and may refresh; surrounding human text is preserved.
- Mismatching hash: a human or another tool changed the body; preserve the page and emit a conflict action.
- Missing hash: ownership cannot be proven; preserve the legacy region for LLM review.
- Missing region markers: preserve the whole page as human-owned.

The skill then requires the LLM to inspect current docs and source evidence, merge stale machine claims around the human contribution, and write a new generated baseline only after preserving the contribution.

## Scope

- Generated-region rendering and integrity parsing.
- Sync write/merge behavior and regression tests.
- Skill sync instructions and generated target-repo agent guidance.
- Requirements, architecture, API, standards, ADR, traceability, validation, context, backlog, and changelog reconciliation.

No new CLI command, provider, language adapter, database migration, or external merge tool is introduced.

## Alternatives

- Always preserve every existing generated page: rejected because it prevents safe automatic refresh when no human edit exists.
- Store the baseline only in SQLite: rejected because committed docs must remain portable across machines, sessions, and model changes.
- Attempt an automatic text three-way merge: rejected because semantic documentation conflicts require LLM judgment and silent textual merging can retain stale claims.

## Risks

- Existing hashless markers cannot prove ownership. Preserve them conservatively and require one reconciliation pass.
- A user may intentionally edit the hash marker. Treat that as untrusted ownership and preserve the page.
- Conflict preservation can leave a stale page temporarily. Report the conflict clearly and require semantic reconciliation before claiming sync completion.

## Verification

- Unit-test stable hashing and marker presence.
- Integration-test unchanged generated refresh, outside-region preservation, inside-region preservation, and hashless legacy preservation.
- Run the full workspace suite and a real init/sync smoke.
