# CodeWiki Standards

These standards apply to CodeWiki product work in addition to the general Harness standards.

## Product Surface

- CodeWiki is a Codex skill first.
- Rust is companion tooling for deterministic local operations, not the primary UX.
- Do not add CLI-first workflows for source/workspace management.
- Do not add bundled non-Git providers for Jira, Figma, or similar systems.

## Evidence Quality

- Durable claims must cite file, symbol, command, documentation, Git, provider, or explicit hypothesis evidence.
- Generated docs must distinguish evidence-backed facts from hypotheses and open questions.
- Lexical dependency hints are hints until verified by deeper source analysis or provider evidence.
- Stale claims must not be used as facts without narrow source re-check.
- Project-local discovery must exclude CodeWiki's managed `.agents/skills/codewiki/**` runtime while continuing to inspect relevant current-working-tree source, including uncommitted and untracked files.

## Reader Documentation Quality

- Deterministic discovery artifacts belong in `docs/evidence/**`; they must not be promoted as reader prose.
- Reader-doc success requires completed model mental-model, WikiPlan v2, page-synthesis, source-audit, diagram, cross-page, and docs-only onboarding stages.
- Reader pages explain purpose, scope, mental model, relationships/flow, change guidance, risks, and verification before optional source inventories.
- Dynamic topics are semantic systems, components, workflows, state/data boundaries, integrations, or high-risk change surfaces—not files or top-level directories.
- Reject local absolute links, duplicate frontmatter/headings, renderer-specific MDX, raw symbol/import inventories, malformed diagrams, broken links, and orphan pages.
- A critical onboarding failure cannot be averaged away. Route named gaps through at most one bounded revision, then report incomplete.

## Distribution Integrity

- Skill instructions, references, helper, and companion interface form one versioned package.
- Installed managed content must match `INSTALLATION.yml` and be compatible with `package.yml` before status/init/sync/validate through the helper.
- Generation run metadata records the resolved skill root, package and contract versions, managed digest, and source revision.
- `legacy_unverified`, `content_drift`, and `incompatible` installations cannot report `reader_docs_ready`.
- Doctor is read-only and never auto-updates an installation.

## Convention Quality

- Discover project, language, framework/library, and area conventions from repository configuration, docs, repeated source patterns, tests, commands, or Git evidence.
- Do not present generic ecosystem best practices as repository conventions without adoption evidence.
- Label conventions as explicit, inferred, hypothesis, or exception.
- Require at least two independent examples for an inferred convention; otherwise keep it as a hypothesis.
- Record scope, evidence, confidence, counterexamples/exceptions, and change impact in `docs/conventions/OVERVIEW.md`.

## Sync Safety

- Generated docs must use CodeWiki generated-region markers.
- Newly generated regions must carry a portable integrity hash for the last CodeWiki-owned body.
- Sync may update a marked generated region automatically only when its current body matches the recorded integrity hash.
- Sync must preserve human-owned text outside generated regions.
- Sync must preserve manual edits inside generated regions and route them to LLM semantic reconciliation.
- Legacy hashless generated regions must be preserved until reconciled; marker presence alone does not prove current machine ownership.
- If an existing changed page has no generated-region markers, preserve it rather than overwrite it.

## Tool Policy

- Baseline tools are Git, filesystem, SQLite, Codex reasoning, and optional Rust helpers.
- Octocode is the first-choice optional code-intelligence provider when default evidence is insufficient.
- codebase-memory-mcp is only for shared cross-session memory beyond CodeWiki SQLite state.
- CocoIndex is only for repo scale or repeated refresh/query workloads that justify indexing.
