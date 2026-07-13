---
artifact_type: requirements_index
id: REQUIREMENTS
status: active
owner: shared
human_fields: [priority, acceptance, requirement_source]
ai_fields: [requirement_rows, status_updates, trace_links]
shared_fields: [functional_requirements, non_functional_requirements]
---

# Requirements

## Field Ownership

- Human owns priority, acceptance, and requirement source.
- AI maintains rows, trace links, and status updates.

## Functional Requirements

| ID | Requirement | Priority | Source | Status |
| --- | --- | --- | --- | --- |
| REQ-001 | CodeWiki init must automatically explore a repository and generate an evidence-backed WikiPlan without approval gates. | high | User direction | implemented |
| REQ-002 | CodeWiki must dynamically detect languages, libraries, frameworks, package managers, entrypoints, tests, and architecture signals. | high | User direction | implemented |
| REQ-003 | CodeWiki must preserve reusable config and runtime state across sessions and LLM/model changes. | high | User direction | implemented |
| REQ-004 | CodeWiki generated docs must trace claims to source evidence, command evidence, or explicit hypotheses. | high | User direction | implemented |
| REQ-005 | CodeWiki should start with a minimal tool surface and keep code-intelligence providers optional and replaceable. | medium | User direction | implemented |
| REQ-006 | CodeWiki's Rust helper tooling must remain companion tooling while OpenWiki and deepwiki-open are used as references. | high | User direction | implemented |
| REQ-007 | CodeWiki must be installable as a Codex skill from this repository with a single install command. | high | User direction | implemented |
| REQ-008 | CodeWiki must use Octocode as the default first-choice code-intelligence provider when needed, while gating codebase-memory-mcp and CocoIndex behind specific memory/indexing triggers. | high | User direction | implemented |
| REQ-009 | CodeWiki generated docs must use `docs/**` as the knowledge surface and `.agents/skills/codewiki/project/**` as the committed control plane. | high | User direction | implemented |
| REQ-010 | CodeWiki must synchronize generated regions safely, preserve human-owned content, and surface stale evidence. | high | User direction | implemented |
| REQ-011 | CodeWiki must use Git as the default change source while allowing user-provided source extension skills and external wiki workspaces. | high | User direction | implemented |
| REQ-012 | CodeWiki must discover and document repository-specific project, language, framework/library, and area conventions from explicit and repeated code evidence. | high | User direction | implemented |

## Non-Functional Requirements

| ID | Requirement | Category | Status |
| --- | --- | --- | --- |
| NFR-001 | The core must avoid language/framework-specific adapters. | portability | implemented |
| NFR-002 | Durable local state must be schema-versioned and migratable. | reliability | implemented |
| NFR-003 | Provider integrations must be narrow and replaceable. | maintainability | implemented |
| NFR-004 | The product target is complete quality, not an MVP-only slice. | product_quality | implemented |
| NFR-005 | Rust crate boundaries should keep CLI, core engine, storage, detection, and provider integration testable independently. | maintainability | implemented |
| NFR-006 | The skill workflow must remain the primary user experience; companion binaries must not become required for basic skill activation. | product_quality | implemented |
| NFR-007 | Optional provider guidance must live in the skill and target-repo CodeWiki agent instructions, not as mandatory bundled dependencies. | portability | implemented |
| NFR-008 | Generated documentation layout must be stable enough for sync and docs-first Q&A across model/session changes. | maintainability | implemented |
| NFR-009 | Convention documentation must describe observed repository practice, including scope and exceptions, rather than generic ecosystem best practices. | product_quality | implemented |
