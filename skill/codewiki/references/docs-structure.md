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
  codewiki/
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

`docs/codewiki/**` is the knowledge surface. `.codewiki/**` is the committed control plane. SQLite state and rebuildable cache live outside the repo/workspace.

This structure may live either inside the source repository or inside a separate personal/external wiki workspace. When outside the source repository, `.codewiki/sources.yml` records the source repository and any additional evidence sources.

## Page Rules

- Always create `docs/codewiki/index.md` after successful init.
- Treat all other paths as canonical slots, not mandatory stubs.
- Create a page only when it has evidence-backed content.
- Prefer canonical top-level pages before creating many `areas/<area-slug>.md` pages.
- Use `areas/<area-slug>.md` only for substantial code areas, bounded contexts, packages, apps, services, or domains.
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
