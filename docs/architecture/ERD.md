---
artifact_type: erd_master
id: ERD-MASTER
status: active
owner: shared
human_fields: [data_ownership_decisions, migration_approval]
ai_fields: [entities, relationships, constraints, migrations, linked_decisions]
shared_fields: [status, trace]
---

# ERD

## Field Ownership

- Human owns data ownership and migration approval.
- AI maintains entities, relationships, constraints, migrations, and linked decisions.

## Entities

| Entity/Table | Purpose | Owner | Notes |
| --- | --- | --- | --- |
| `schema_migrations` | Records applied SQLite migrations | CodeWiki runtime | Enables durable state upgrades across versions |
| `repositories` | Stores target repository identity and current Git context | CodeWiki runtime | Keyed by stable repository ID |
| `sync_runs` | Records init/sync/Q&A research runs that mutate or verify state | CodeWiki runtime | Links evidence and generated pages to a run |
| `files` | Tracks source, docs, and generated files inspected or produced by CodeWiki | CodeWiki runtime | `is_generated` separates generated docs from source evidence |
| `symbols` | Tracks discovered symbols and source locations | CodeWiki runtime | Provider-neutral symbol cache |
| `pages` | Tracks generated `docs/**` pages and status | CodeWiki runtime | Supports stale-page detection |
| `evidence_items` | Stores durable evidence summaries from files, symbols, commands, docs, Git, or providers | CodeWiki runtime | Must not store secrets |
| `claims` | Stores durable wiki claims with status, confidence, and ownership | CodeWiki runtime | Claims can be active, stale, superseded, or hypothesis |
| `claim_evidence` | Many-to-many support/contradiction links between claims and evidence | CodeWiki runtime | Enables evidence-first Q&A and sync |
| `provider_snapshots` | Records optional provider use, version/config hash, and trigger reason | CodeWiki runtime | Supports Octocode/codebase-memory-mcp/CocoIndex auditability |
| `open_questions` | Tracks unresolved uncertainties that affect future understanding or safe changes | CodeWiki runtime | Can be resolved by later sync/research |

## Relationships

| From | To | Relationship | Notes |
| --- | --- | --- | --- |
| `sync_runs.repository_id` | `repositories.id` | many-to-one | Every run belongs to one repository |
| `files.repository_id` | `repositories.id` | many-to-one | File paths are scoped per repository |
| `files.last_seen_run_id` | `sync_runs.id` | many-to-one optional | Tracks last observed run |
| `symbols.file_id` | `files.id` | many-to-one | Symbols belong to files |
| `pages.repository_id` | `repositories.id` | many-to-one | Generated pages are scoped per repository |
| `pages.last_generated_run_id` | `sync_runs.id` | many-to-one optional | Tracks page generation/update run |
| `evidence_items.repository_id` | `repositories.id` | many-to-one | Evidence is repo-scoped |
| `evidence_items.symbol_id` | `symbols.id` | many-to-one optional | Evidence may refer to a symbol |
| `evidence_items.run_id` | `sync_runs.id` | many-to-one optional | Evidence may be captured during a run |
| `claims.page_id` | `pages.id` | many-to-one optional | Claims may be owned by generated pages |
| `claim_evidence.claim_id` | `claims.id` | many-to-one | Join table |
| `claim_evidence.evidence_id` | `evidence_items.id` | many-to-one | Join table |
| `provider_snapshots.repository_id` | `repositories.id` | many-to-one | Provider use is repo-scoped |
| `open_questions.repository_id` | `repositories.id` | many-to-one | Questions are repo-scoped |
| `open_questions.created_run_id` | `sync_runs.id` | many-to-one optional | Run that created the question |
| `open_questions.resolved_run_id` | `sync_runs.id` | many-to-one optional | Run that resolved the question |

## Constraints

- `repositories.root_path` is required; Git fields may be null for non-Git folders.
- `files.path` is unique per repository.
- `pages.path` is unique per repository.
- `claim_evidence` uses `(claim_id, evidence_id)` as a composite primary key.
- Deleting a repository cascades to all repo-scoped state.
- Evidence summaries must not store secret values.
- Confidence/status values are text in the first migration so the agent/runtime can evolve labels before hard enum constraints are introduced.

## Migrations

- `001_initial_state.sql`: creates the durable local state schema for repositories, sync runs, files, symbols, pages, evidence, claims, provider snapshots, and open questions.

## Linked Decisions

- `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`
- `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`
- `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
