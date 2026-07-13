---
artifact_type: api_contract_index
id: API-MASTER
status: draft
owner: shared
human_fields: [contract_intent, approval]
ai_fields: [contract_rows, errors, auth_notes, versioning, linked_decisions]
shared_fields: [status, trace]
---

# API

## Field Ownership

- Human owns public contract intent and approval for contract changes.
- AI maintains contract rows, errors, auth notes, versioning, and linked decisions.

## API Surface

Document HTTP endpoints, RPC methods, events, CLI commands, or any other public contract.

| Contract | Type | Auth | Status | Notes |
| --- | --- | --- | --- | --- |
| `curl -fsSL https://raw.githubusercontent.com/dungxbuif/harness/main/scripts/install-codewiki-skill.sh \| bash` | install command | GitHub/repo access | implemented | Installs `skill/codewiki` into `$CODEX_HOME/skills/codewiki`. |
| `codewiki help` / `codewiki --help` | companion command | none | implemented | Prints scaffold usage and companion-tool status. |
| `codewiki version` / `codewiki --version` | companion command | none | implemented | Prints current package version. |
| `codewiki status` | companion command | none | implemented | Prints Rust companion scaffold status, command list, planned detection, config path, local state summary, and docs root. |
| `codewiki doctor` | companion command | none | planned | Future deterministic environment/config diagnostic helper. |
| `codewiki inspect` | companion command | none | planned | Future deterministic repository signal inspection helper. |
| `codewiki cache` | companion command | none | planned | Future deterministic cache/index helper. |

## Errors

| Error | Meaning | Consumer Impact |
| --- | --- | --- |
| Unknown command | Companion command is not recognized | Companion binary exits with code 2 and suggests `codewiki help`. |
| Extra argument | Scaffold parser receives more than one argument | CLI exits with code 2 until richer parsing is implemented. |

## Versioning

Pre-1.0 skill and companion-tool surfaces are unstable. The skill install command is the primary install surface; Rust commands are companion helpers.

## Linked Decisions

- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
