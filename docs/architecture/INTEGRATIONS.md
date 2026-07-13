---
artifact_type: integrations_doc
id: INTEGRATIONS
status: draft
owner: shared
human_fields: [provider_choice, credential_approval, business_owner]
ai_fields: [systems, contracts, secrets_names, failure_modes]
shared_fields: [status, trace]
---

# Integrations

## Field Ownership

- Human owns provider choices, credential approval, and business owner.
- AI documents systems, contracts, secret names, and failure modes.

## External Systems

| System | Purpose | Contract | Owner | Status |
| --- | --- | --- | --- | --- |
| Git | Repository identity, diff, history, and sync context | Local git CLI/library boundary | CodeWiki | required |
| Filesystem | Source and generated docs access | Local paths in target repo | CodeWiki | required |
| SQLite | Durable local runtime state | Versioned schema in platform app data | CodeWiki | planned |
| Rust companion tool | Deterministic helper surface for the skill | `codewiki` binary and internal crates | CodeWiki | accepted |
| Octocode | Default first-choice semantic code-intelligence provider | Install/activate per target repo when filesystem/Git exploration is insufficient | target repo | conditional_default |
| codebase-memory-mcp | Conditional runtime memory integration | Install/activate only when extra shared cross-session memory is needed beyond CodeWiki SQLite state | target repo | conditional |
| CocoIndex | Conditional runtime indexing integration | Install/activate only when repo size or repeated refresh/query workload justifies an indexing pipeline | target repo | conditional |

## Credentials And Secrets

Document required secret names only. Do not store secret values.

- No credentials are defined yet.
- Future provider credentials must use secret names only in docs.

## Failure Modes

- Optional provider unavailable: continue with filesystem/Git exploration where possible and mark reduced confidence.
- SQLite migration failure: block sync/write operations until state is backed up or migrated.
- Stale generated docs: mark affected pages stale and require sync evidence before claiming freshness.
