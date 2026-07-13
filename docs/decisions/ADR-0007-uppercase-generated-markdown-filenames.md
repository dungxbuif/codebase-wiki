---
artifact_type: adr
id: ADR-0007
status: accepted
owner: human
trace:
  phase: not_applicable
  requirements: [docs/requirements/SPEC.md]
  backlog_items: [BL-015]
  triggering_ticket: docs/work/tickets/TICKET-026-uppercase-generated-markdown-filenames.md
  related:
    - docs/decisions/ADR-0005-codewiki-generated-docs-structure.md
    - docs/architecture/ARCHITECTURE.md
---

# ADR-0007: Uppercase Generated Markdown Filenames

## Status

Accepted on 2026-07-14 by direct human decision.

## Context

ADR-0005 standardized the generated wiki structure but used lowercase Markdown basenames and later workflow text retained an inconsistent `index.md` reference. The generated paths are shared by init, sync, WikiPlan, docs-first Q&A, source filtering, and durable links, so the naming convention must be explicit and consistent.

## Decision

Keep generated wiki directories lowercase and make every CodeWiki-generated Markdown basename uppercase. The required entrypoint is `docs/QUICKSTART.md`; `index.md` is not a canonical page. Existing lowercase pages are migrated automatically only when CodeWiki generated-region markers prove CodeWiki ownership.

This ADR supersedes only the filename casing and entrypoint spelling portions of ADR-0005. ADR-0005's directory structure, page semantics, evidence rules, and ownership boundaries remain accepted.

## Alternatives

- Retain lowercase filenames: rejected by the human naming decision.
- Uppercase directories too: rejected because the decision applies to Markdown filenames and lowercase directories remain conventional and stable.
- Force-rename all legacy files: rejected because unmarked files may be human-authored.

## Consequences

- Generated links and WikiPlan paths change case.
- Case-sensitive filesystems receive one canonical path per page.
- Marker-owned legacy pages can migrate safely.
- Unmarked lowercase pages remain preserved and may coexist until a human reconciles them.
- Tests and docs must reject reintroduction of lowercase canonical generated paths.

