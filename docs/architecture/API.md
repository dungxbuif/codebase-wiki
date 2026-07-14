---
artifact_type: api_contract_index
id: API-MASTER
status: active
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
| `curl -fsSL https://raw.githubusercontent.com/dungxbuif/codebase-wiki/master/scripts/install-codewiki-skill.sh \| bash` | install command | GitHub/repo access plus optional Cargo build | implemented | Installs `skill/codewiki` into the current target repository at `.agents/skills/codewiki` by default, builds `bin/codewiki` when Cargo is available, and keeps copied Rust companion source as fallback. Set `CODEWIKI_INSTALL_SCOPE=global` to install into `$CODEX_HOME/skills/codewiki`. |
| `skill/codewiki/scripts/codewiki-helper.sh [args...]` | skill helper script | optional Rust binary/source | implemented | Resolves the Rust companion through `CODEWIKI_COMPANION_BIN`, installed `bin/codewiki`, `PATH`, `CODEWIKI_REPO`, installed companion source, or source checkout. |
| `codewiki help` / `codewiki --help` | companion command | none | implemented | Prints scaffold usage and companion-tool status. |
| `codewiki version` / `codewiki --version` | companion command | none | implemented | Prints current package version. |
| `codewiki status` | companion command | none | implemented | Prints Rust companion status, command list, detection capabilities, config path, local state summary, and docs root. |
| `codewiki init [path]` | companion command | local filesystem | implemented | Creates missing `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, `.agents/skills/codewiki/project/AGENTS.md`, canonical uppercase Markdown pages beginning with `docs/QUICKSTART.md` and `docs/conventions/OVERVIEW.md`, and applies local SQLite migrations. Marker-owned legacy lowercase generated pages are migrated safely. Defaults to the current directory when `path` is omitted. |
| `codewiki sync [path]` | companion command | local filesystem | implemented | Re-detects repository signals, compares generated plan/docs to desired output, refreshes only generated bodies whose integrity hash proves they are unchanged, preserves manual or legacy-unverified content for LLM reconciliation, creates missing pages, and no-ops when current. |

## Errors

| Error | Meaning | Consumer Impact |
| --- | --- | --- |
| Unknown command | Companion command is not recognized | Companion binary exits with code 2 and suggests `codewiki help`. |
| Invalid init usage | `codewiki init` receives too many arguments | CLI exits with code 2 and prints `codewiki init [path]` usage. |
| Invalid sync usage | `codewiki sync` receives too many arguments or runs before init | CLI exits with code 1 or 2 and prints the relevant message. |
| Init write failure | Target repo files, state dirs, or SQLite migrations cannot be created | CLI exits with code 1 and prints the failing path or migration error. |

## Versioning

Pre-1.0 skill and companion-tool surfaces are unstable. The skill install command is the primary install surface; Rust commands are companion helpers.

## Linked Decisions

- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
- `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
- `docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md`
