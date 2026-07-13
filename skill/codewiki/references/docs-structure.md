# CodeWiki Docs Structure

Load this reference for every `init` and `sync` run, and when Q&A needs to explain or repair the generated wiki layout.

## Target Repository Layers

```text
.codewiki/
  config.yml
  plan.yml
  AGENTS.md
  sources.yml

docs/
  quickstart.md
  source-map.md
  architecture/
    overview.md
    decisions.md
  domain/
    overview.md
  workflows/
    overview.md
  data-models/
    overview.md
  api/
    overview.md
  operations/
    runbook.md
  testing/
    strategy.md
  glossary.md
  open-questions.md
  evidence/
    README.md
    sources.md
    commands.md
    claims.md
  areas/
    <area-slug>/
      overview.md
```

`docs/**` is the knowledge surface. `.codewiki/**` is the committed control plane. SQLite state and rebuildable cache live outside the repo/workspace.

Because CodeWiki writes directly into `docs/`, existing unmarked human-authored files must be preserved. Generated content should be written only to canonical pages that are missing or contain CodeWiki generated-region markers.

This structure may live either inside the source repository or inside a separate personal/external wiki workspace. When outside the source repository, `.codewiki/sources.yml` records the source repository and any additional evidence sources.

## Page Rules

- Always create `docs/quickstart.md` after successful init.
- Generate section directories only when the section has real explanatory value.
- If evidence is thin, keep the page explicit about gaps instead of pretending the section is complete.
- Prefer headings inside `quickstart.md` or broader section pages before creating many small directories.
- Use `areas/<area-slug>/overview.md` for observed top-level areas only when the area is substantial.
- Keep one canonical home per concept. Link instead of duplicating.
- Preserve human-owned sections during sync.
- Each generated source-backed page should start with a `<details>` block listing relevant source files, following the DeepWiki page pattern.
- Each page should include source anchors inline or in a source-map section where they help future humans/agents verify claims.
- Keep deferred areas in a concise `## Backlog` section at the end of `quickstart.md`; do not create separate stub pages.

## Canonical Page Semantics

- `quickstart.md`: entrypoint, overview, freshness, navigation, key source files, notes for future agents, and backlog.
- `source-map.md`: semantic navigation map of packages, apps, services, bounded contexts, and where to start.
- `architecture/overview.md`: runtime architecture, major components, dependency direction, constraints, and change risks.
- `architecture/decisions.md`: durable decisions inferred from docs, code, and Git history; link existing ADRs when present.
- `domain/overview.md`: product/business/domain concepts and invariants.
- `workflows/overview.md`: user/system flows, jobs, event flows, lifecycles, and important sequences.
- `data-models/overview.md`: persistence, schemas, migrations, storage boundaries, and data ownership.
- `api/overview.md`: public APIs, CLIs, events, RPC, package/library surfaces, and integrations.
- `operations/runbook.md`: setup, build/run/deploy, environment, observability, troubleshooting, and runtime risks.
- `testing/strategy.md`: test strategy, commands, fixtures, coverage gaps, and safe-change checks.
- `glossary.md`: project-specific terms, acronyms, aliases, and domain language.
- `open-questions.md`: uncertainties that affect future understanding, sync quality, or safe changes.
- `evidence/README.md`: how evidence is recorded and verified.
- `evidence/sources.md`: inspected files, docs, Git history, provider outputs, and source artifacts.
- `evidence/commands.md`: commands run or recommended, with result summaries when available.
- `evidence/claims.md`: durable claims with evidence links, confidence, status, and owning page.

## Evidence Requirements

Each durable claim should point to at least one of:

- source file path;
- symbol or API name plus file path;
- command and summarized result;
- existing documentation;
- Git evidence;
- provider evidence;
- explicit hypothesis marker with confidence and open question.

Do not present hypotheses as facts.
