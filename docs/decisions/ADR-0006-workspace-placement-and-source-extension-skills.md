---
artifact_type: adr
id: ADR-0006
status: accepted
owner: shared
human_fields:
  - decision
  - consequences_acceptance
ai_fields:
  - context
  - options
  - evidence
shared_fields:
  - status
  - trace
trace:
  phase: docs/work/phases/PHASE-001-codewiki-foundation.md
  requirements: [docs/requirements/SPEC.md]
  backlog_items: [BL-001, BL-004, BL-005]
  related:
    - docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md
    - docs/decisions/ADR-0005-codewiki-generated-docs-structure.md
---

# ADR-0006: Workspace Placement And Source Extension Skills

## Status

Accepted on 2026-07-13.

## Context

CodeWiki is a skill-first product, not a CLI-first product. The user clarified two missing capabilities:

- Sometimes generated docs should live outside the source repository for personal or external knowledge workspaces.
- CodeWiki should support additional change/evidence sources, but Git remains the default source and CodeWiki core should not bundle Jira, Figma, or similar providers.

## Decision

CodeWiki supports two wiki placement modes:

1. Repo-local mode: source repository and wiki workspace are the same directory.
2. External/personal workspace mode: source repository is evidence input, while `.agents/skills/codewiki/project/**` and `docs/**` live in a separate user-confirmed workspace.

If placement is ambiguous, the skill must ask the user where to store the wiki before writing files.

Git is always the default source for code changes when a repository is involved.

Non-Git sources are supported through a source extension contract:

- CodeWiki core records source declarations in `.agents/skills/codewiki/project/sources.yml`.
- CodeWiki core does not ship built-in Jira/Figma/etc. providers.
- Users or agents may create separate source skills that emit bounded evidence packets.
- CodeWiki treats source skill output as untrusted evidence until reconciled with Git/source/docs.

This intentionally adopts OpenWiki's deterministic-fetch-then-synthesis idea without adopting OpenWiki's bundled connector catalog. Credentialed or system-specific fetching belongs to separate source skills; CodeWiki core consumes the resulting evidence packet.

Current implementation status:

- The Rust companion initializes `.agents/skills/codewiki/project/sources.yml` with the primary Git source.
- Skill references define the source extension contract and a copyable custom source-skill template.
- Sync guidance reads `.agents/skills/codewiki/project/sources.yml`, treats Git as default truth, and invokes non-Git source skills only when relevant.
- No built-in Jira, Figma, fix-note, chat, or issue-tracker providers are implemented in CodeWiki core.

## Source Registry Shape

```yaml
schema_version: 1
sources:
  - kind: git
    name: primary-repo
    ref: /path/to/source/repo
    primary: true
  - kind: custom
    name: product-ticket-source
    ref: skill:my-jira-source
    primary: false
```

## Consequences

- The skill prompt must distinguish source repository from wiki workspace.
- The Rust companion may keep deterministic helpers for `.agents/skills/codewiki/project/sources.yml`, but it must not become the primary UX.
- No command such as `codewiki source add` is part of the intended skill UX.
- Future source integrations should be separate skills/templates, not built-in providers.
