# Semantic Exploration

Load this reference during init/sync before writing docs, and during Q&A when generated docs are insufficient.

## Purpose

Semantic exploration turns repository files into evidence-backed structure hints:

- files/modules with roles;
- top-level areas;
- symbols;
- imports and dependency hints;
- evidence references that docs and claims can cite.

These are hints until promoted into durable claims. Do not present lexical imports or symbol matches as complete architecture by themselves.

## Default Flow

1. Start with generated docs and `.agents/skills/codewiki/project/plan.yml` if they exist.
2. Use Git/filesystem and companion helper output where available to build a bounded semantic snapshot.
3. Prefer source files, package/config files, tests, and existing docs over generated/vendor/cache files.
4. Group findings by area and cite file paths/symbols.
5. Write generated docs only from evidence-backed statements or explicit hypotheses.
6. Promote deterministic claims only when they cite evidence IDs.
7. Persist reusable evidence/claims in local SQLite when the companion helper is available.
8. Record gaps in `docs/open-questions.md` or `.agents/skills/codewiki/project/plan.yml`.

## Evidence Rules

- Every durable architecture statement needs at least one file, symbol, command, existing doc, or explicit hypothesis marker.
- Dependency rows from lexical imports are dependency hints, not resolved runtime behavior.
- If a file or area was not inspected, say so rather than implying coverage.
- If optional providers are used, record why default exploration was insufficient.
- Generated `docs/evidence/claims.md` and SQLite claim/evidence rows should describe the same promoted evidence base.

## Provider Escalation

Use optional providers only when semantic quality needs them:

- Octocode: when symbol/dependency search or cross-file structure is hard to infer from bounded filesystem exploration.
- codebase-memory-mcp: when multiple agents/sessions need shared memory beyond CodeWiki SQLite state.
- CocoIndex: when repo size or repeated refresh/query workload justifies indexing.

Do not activate all providers by default.
