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

## Convention Quality

- Discover project, language, framework/library, and area conventions from repository configuration, docs, repeated source patterns, tests, commands, or Git evidence.
- Do not present generic ecosystem best practices as repository conventions without adoption evidence.
- Label conventions as explicit, inferred, hypothesis, or exception.
- Require at least two independent examples for an inferred convention; otherwise keep it as a hypothesis.
- Record scope, evidence, confidence, counterexamples/exceptions, and change impact in `docs/conventions/OVERVIEW.md`.

## Sync Safety

- Generated docs must use CodeWiki generated-region markers.
- Sync may update marked generated regions.
- Sync must preserve human-owned text outside generated regions.
- If an existing changed page has no generated-region markers, preserve it rather than overwrite it.

## Tool Policy

- Baseline tools are Git, filesystem, SQLite, Codex reasoning, and optional Rust helpers.
- Octocode is the first-choice optional code-intelligence provider when default evidence is insufficient.
- codebase-memory-mcp is only for shared cross-session memory beyond CodeWiki SQLite state.
- CocoIndex is only for repo scale or repeated refresh/query workloads that justify indexing.
