# CodeWiki Q&A Workflow

Load this reference when the user asks questions about a repository that already has CodeWiki docs or CodeWiki state.

## Answer Order

1. Read `docs/**` first.
2. Read `.agents/skills/codewiki/project/plan.yml`.
3. Read `.agents/skills/codewiki/project/AGENTS.md`.
4. Use local SQLite facts/evidence/claims if available.
5. Inspect source files and Git history only when docs/state are missing, stale, ambiguous, or contradicted.
6. Activate optional external tools only when earlier layers are insufficient or the user requests graph/index/memory-heavy analysis.

## Mandatory SQLite Retrieval

When local CodeWiki state exists, use the companion command instead of writing ad-hoc SQL:

```bash
codewiki query --text "<focused question or symbol/path>" --repo <repository-path> --limit 10
```

For freshness review or targeted invalidation checks, use:

```bash
codewiki claims --repo <repository-path> --status stale --limit 50
codewiki claims --repo <repository-path> --status all --path <repo-relative-source-path>
```

The query packet must provide:

- active claims first;
- stale claims in a separate section;
- matching file and symbol inventory;
- matching evidence summaries;
- evidence IDs and source paths for citations;
- query terms matched against claim statements and evidence paths.

Treat stale claims as warnings, not answer facts. If a stale claim is relevant, inspect the cited source path narrowly or recommend sync before answering.

If the installed companion predates `codewiki query`, report the missing retrieval surface and fall back to bounded direct SQLite inspection only when necessary. Do not pretend that `docs/evidence/CLAIMS.md` is an export of SQLite status: both are derived from exploration, but only SQLite retains active/stale history.

## Answer Rules

- Answer in the user's language unless they request another language.
- Lead with the answer, then cite the supporting wiki pages or source evidence.
- If docs are enough, do not inspect raw source just to be extra sure.
- If docs are insufficient, say what is missing or stale and inspect narrowly.
- Use `codewiki query` before source fallback; do not skip it merely because direct filesystem access is available.
- Do not present hypotheses as facts.
- If answering required source fallback, recommend a follow-up sync when the docs should be updated.
- If SQLite contains stale claims, say which claim/evidence path is stale before relying on source fallback.

## Evidence Style

Use concise references such as:

- `docs/architecture/OVERVIEW.md`
- `docs/conventions/OVERVIEW.md`
- `.agents/skills/codewiki/project/plan.yml`
- `src/foo/bar.ts`
- `symbol: PaymentService.authorize`
- `command: cargo test -p crate_name`

Avoid long copied code excerpts. Summarize what the evidence proves.
