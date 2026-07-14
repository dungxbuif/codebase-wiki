# CodeWiki Docs Structure

Load this reference for every `init` and `sync` run, and when Q&A needs to explain or repair the generated wiki layout.

## Target Repository Layers

```text
.agents/skills/codewiki/project/
  config.yml
  plan.yml
  AGENTS.md
  sources.yml

docs/
  QUICKSTART.md
  SOURCE-MAP.md
  architecture/
    OVERVIEW.md
    DECISIONS.md
  domain/
    OVERVIEW.md
  workflows/
    OVERVIEW.md
  data-models/
    OVERVIEW.md
  api/
    OVERVIEW.md
  operations/
    RUNBOOK.md
  testing/
    STRATEGY.md
  conventions/
    OVERVIEW.md
  GLOSSARY.md
  OPEN-QUESTIONS.md
  evidence/
    README.md
    SOURCES.md
    COMMANDS.md
    CLAIMS.md
  components/
    <SEMANTIC-TOPIC>.md
```

`docs/**` is the knowledge surface. `.agents/skills/codewiki/project/**` is the committed control plane. SQLite state and rebuildable cache live outside the repo/workspace.

Because CodeWiki writes directly into `docs/`, existing unmarked human-authored files must be preserved. Generated content should be written only to canonical pages that are missing or contain CodeWiki generated-region markers.

This structure may live either inside the source repository or inside a separate personal/external wiki workspace. When outside the source repository, `.agents/skills/codewiki/project/sources.yml` records the source repository and any additional evidence sources.

## Page Rules

- Always create `docs/QUICKSTART.md` after successful init.
- Always create `docs/conventions/OVERVIEW.md` after successful init and derive it from repository evidence using `conventions.md`.
- Keep generated Markdown filenames uppercase and directories lowercase.
- Generate section directories only when the section has real explanatory value.
- If evidence is thin, keep the page explicit about gaps instead of pretending the section is complete.
- Prefer headings inside `QUICKSTART.md` or broader section pages before creating many small directories.
- Put dynamic topic pages under their semantic owner such as `architecture/`, `workflows/`, `components/`, or `data-models/`. Read `areas/**` only for legacy compatibility and re-plan it semantically.
- Keep one canonical home per concept. Link instead of duplicating.
- Preserve human-owned sections during sync.
- Each reader page starts with purpose, scope, and a plain-language mental model. Source inventories never precede the explanation.
- Important claims use claim-local source anchors. An optional source inventory may appear at the end; exhaustive evidence belongs under `docs/evidence/**`.
- Keep deferred areas in a concise `## Backlog` section at the end of `QUICKSTART.md`; do not create separate stub pages.

## Canonical Page Semantics

- `QUICKSTART.md`: five-minute mental model, verified start path, system at a glance, task-oriented reading paths, freshness, limitations, and backlog.
- `SOURCE-MAP.md`: semantic navigation map of packages, apps, services, bounded contexts, and where to start.
- `architecture/OVERVIEW.md`: runtime architecture, major components, dependency direction, constraints, and change risks.
- `architecture/DECISIONS.md`: durable decisions inferred from docs, code, and Git history; link existing ADRs when present.
- `domain/OVERVIEW.md`: product/business/domain concepts and invariants.
- `workflows/OVERVIEW.md`: user/system flows, jobs, event flows, lifecycles, and important sequences.
- `data-models/OVERVIEW.md`: persistence, schemas, migrations, storage boundaries, and data ownership.
- `api/OVERVIEW.md`: public APIs, CLIs, events, RPC, package/library surfaces, and integrations.
- `operations/RUNBOOK.md`: setup, build/run/deploy, environment, observability, troubleshooting, and runtime risks.
- `testing/STRATEGY.md`: test strategy, commands, fixtures, coverage gaps, and safe-change checks.
- `conventions/OVERVIEW.md`: explicit and inferred project, language, framework/library, and area conventions with evidence, scope, confidence, exceptions, and change impact.
- `GLOSSARY.md`: project-specific terms, acronyms, aliases, and domain language.
- `OPEN-QUESTIONS.md`: uncertainties that affect future understanding, sync quality, or safe changes.
- `evidence/README.md`: how evidence is recorded and verified.
- `evidence/SOURCES.md`: inspected files, docs, Git history, provider outputs, and source artifacts.
- `evidence/COMMANDS.md`: commands run or recommended, with result summaries when available.
- `evidence/CLAIMS.md`: durable claims with evidence links, confidence, status, and owning page.

## Page-Type Quality Contracts

Only `QUICKSTART.md` and `conventions/OVERVIEW.md` are unconditional after successful init. Create every other page only for an evidence-backed reader job.

| Page type | Reader outcome and required content | Must not become |
| --- | --- | --- |
| `QUICKSTART.md` | Form a five-minute project mental model: purpose, status, prerequisites, shortest verified start path, system at a glance, major concepts, task-oriented reading paths, limitations, freshness, and backlog. | Feature dump, full file/symbol list, or duplicate architecture page. |
| `SOURCE-MAP.md` | Map a concept or change request to its owning subsystem, responsibility, boundary, dependencies, entrypoints, likely change location, and canonical page. | Repository tree, exhaustive package table, or lexical-import dump. |
| Architecture/component page | Explain runtime/component boundaries, ownership, dependency direction, collaborators, control/data paths, constraints, extension points, risks, tests, and related flows. | Every class/file, unexplained boxes, one-file wrapper, or ordered trace with no ownership model. |
| Domain page | Explain actors, project language, concepts, relationships, invariants, lifecycles, ownership, examples, and exceptions. | Struct/enum catalog or generic domain essay detached from repository evidence. |
| Workflow page | Trace goal, actors, preconditions, trigger, happy path, failures, retries/recovery, state/data effects, observability, change location, risks, and verification. | UI click list, file list, or sequence that omits system effects and failure behavior. |
| Data-model page | Explain stores/schemas, entity ownership, relationships/keys, invariants, lifecycle, migrations, consistency, sensitivity, and read/write paths. | Type list without ownership, invariants, or migration impact. |
| API/interface page | Explain consumers, contract families, authentication/authorization when applicable, lifecycle, errors, compatibility, representative use, change risks, and tests. | Exhaustive generated method/signature reference without contract guidance. |
| Operations page | Provide evidenced prerequisites, configuration names without secrets, verified commands and expected results, topology, observability, diagnosis, and recovery/rollback where applicable. | Unverified command collection or secret-bearing setup guide. |
| Testing page | Map change types to test layers, exact verified commands, fixtures/environments, known gaps/flakiness, and failure interpretation. | Test-file inventory or unsupported coverage claims. |
| Conventions page | Record convention, scope, classification, evidence, confidence, examples, counterexamples/exceptions, rationale when evidenced, and change impact. | Generic ecosystem advice or a single example presented as an adopted convention. |
| Glossary/open-questions page | Define project language or record uncertainty with impact, attempted evidence, owner/next action, and affected pages. | Identifier index, generic TODO list, or questions with no impact. |
| `evidence/**` | Make source, command, claim, confidence, staleness, and provenance auditable. | Primary onboarding path or duplicate narrative documentation. |

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

## Claim Confidence Labels

Use the canonical claim/page/evidence confidence labels consistently:

- `confirmed`: directly supported by authoritative evidence.
- `source-backed`: supported by source evidence but not independently confirmed.
- `hypothesis`: plausible but incomplete; mark it explicitly and never present it as fact.
- `watchlist`: weak or transient signal worth revisiting before it influences reader guidance.

`inferred` is a convention classification governed by `conventions.md`, not a generic claim confidence label.
