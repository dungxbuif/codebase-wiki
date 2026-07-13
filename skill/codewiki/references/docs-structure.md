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
  index.md
  map.md
  architecture.md
  domains.md
  workflows.md
  data.md
  interfaces.md
  operations.md
  testing.md
  decisions.md
  glossary.md
  open-questions.md
  evidence/
    README.md
    sources.md
    commands.md
    claims.md
  areas/
    <area-slug>.md
```

`docs/**` is the knowledge surface. `.codewiki/**` is the committed control plane. SQLite state and rebuildable cache live outside the repo/workspace.

Because CodeWiki writes directly into `docs/`, existing unmarked human-authored files must be preserved. Generated content should be written only to canonical pages that are missing or contain CodeWiki generated-region markers.

This structure may live either inside the source repository or inside a separate personal/external wiki workspace. When outside the source repository, `.codewiki/sources.yml` records the source repository and any additional evidence sources.

## Page Rules

- Always create `docs/index.md` after successful init.
- Generate canonical top-level synthesis pages during init/sync when semantic exploration has run.
- If evidence is thin, keep the page explicit about gaps instead of pretending the section is complete.
- Prefer canonical top-level pages before creating many `areas/<area-slug>.md` pages.
- Use `areas/<area-slug>.md` for observed top-level areas and keep the page evidence-scoped.
- Keep one canonical home per concept. Link instead of duplicating.
- Preserve human-owned sections during sync.

## Canonical Page Semantics

- `index.md`: overview, freshness, wiki navigation, coverage gaps, and how to use the wiki.
- `map.md`: semantic navigation map of packages, apps, services, bounded contexts, and where to start.
- `architecture.md`: runtime architecture, major components, dependency direction, constraints, and change risks.
- `domains.md`: product/business/domain concepts and invariants.
- `workflows.md`: user/system flows, jobs, event flows, lifecycles, and important sequences.
- `data.md`: persistence, schemas, migrations, storage boundaries, and data ownership.
- `interfaces.md`: public APIs, CLIs, events, RPC, package/library surfaces, and integrations.
- `operations.md`: setup, build/run/deploy, environment, observability, troubleshooting, and runtime risks.
- `testing.md`: test strategy, commands, fixtures, coverage gaps, and safe-change checks.
- `decisions.md`: durable decisions inferred from docs, code, and Git history; link existing ADRs when present.
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
