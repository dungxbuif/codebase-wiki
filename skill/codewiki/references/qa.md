# CodeWiki Q&A Workflow

Load this reference when the user asks questions about a repository that already has CodeWiki docs or CodeWiki state.

## Answer Order

1. Read `docs/codewiki/**` first.
2. Read `.codewiki/plan.yml`.
3. Read `.codewiki/AGENTS.md`.
4. Use local SQLite facts/evidence/claims if available.
5. Inspect source files and Git history only when docs/state are missing, stale, ambiguous, or contradicted.
6. Activate optional external tools only when earlier layers are insufficient or the user requests graph/index/memory-heavy analysis.

## SQLite Context Packet

When local CodeWiki SQLite state exists, build a concise Q&A context packet before source fallback:

- active claims first;
- stale claims in a separate section;
- evidence IDs and source paths for citations;
- query terms matched against claim statements and evidence paths.

Treat stale claims as warnings, not answer facts. If a stale claim is relevant, inspect the cited source path narrowly or recommend sync before answering.

## Answer Rules

- Answer in the user's language unless they request another language.
- Lead with the answer, then cite the supporting wiki pages or source evidence.
- If docs are enough, do not inspect raw source just to be extra sure.
- If docs are insufficient, say what is missing or stale and inspect narrowly.
- Do not present hypotheses as facts.
- If answering required source fallback, recommend a follow-up sync when the docs should be updated.
- If SQLite contains stale claims, say which claim/evidence path is stale before relying on source fallback.

## Evidence Style

Use concise references such as:

- `docs/codewiki/architecture.md`
- `.codewiki/plan.yml`
- `src/foo/bar.ts`
- `symbol: PaymentService.authorize`
- `command: cargo test -p crate_name`

Avoid long copied code excerpts. Summarize what the evidence proves.
