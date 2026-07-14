---
artifact_type: detail_design
id: DESIGN-030
status: in_review
owner: ai
approval: approved
human_fields: [approval, constraints, scope_decisions]
ai_fields: [problem, context_loaded, brownfield_scope, proposed_approach, design_tradeoffs, architecture_overview, execution_flow, api_data_model, security, test_plan, reconciliation_plan]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-018
  requirement: REQ-014
  phase: PHASE-002
  ticket_or_bug:
    - docs/work/tickets/TICKET-030-reader-first-synthesis-and-diagrams.md
    - docs/work/bugs/BUG-001-companion-bypasses-reader-synthesis.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0010-reader-first-information-architecture.md]
  master_docs_touched: [docs/architecture/ARCHITECTURE.md, docs/standards/CODEWIKI.md]
---

# Detail Design: Reader-First Synthesis And Diagrams

## 1. Problem And Boundary

The current companion publishes evidence snapshots into reader pages and never executes the LLM planning/synthesis contract described by the skill. This is reproduced as BUG-001. The change must correct that orchestration boundary, preserve traceability, and enforce progressive disclosure: explain the system first, then map concepts to code.

## 2. Synthesis Pipeline

```text
Deterministic companion
bounded discovery -> evidence persistence -> input validation
        |
        v
Required LLM orchestration
repository mental model -> WikiPlan -> bounded evidence packets
        |
        v
reader-oriented drafts from individual page contracts
        |
        v
Deterministic + semantic quality gates
static -> evidence -> diagram -> ownership/overlap -> docs-only onboarding
        |
        +--> pass: normalize, safely merge, persist provenance/results
        |
        +--> fail: one LLM revision with named gaps, then incomplete on failure
```

Raw semantic snapshots remain available in evidence/state. The companion may create safe scaffolding and explicit evidence/debug pages, but it cannot publish those artifacts as successful reader documentation. A successful run records that mental-model, WikiPlan, page-synthesis, validation, and provenance stages completed. If the selected LLM is unavailable or synthesis fails, the result is `incomplete`, not a deterministic reader-page fallback.

Final reader-facing pages cannot contain bulk lines of file counts, symbol counts, or lexical imports. A draft may not change its own scope, page type, or concept ownership; newly discovered scope routes back to WikiPlan.

## 3. Orchestration And Result Contract

The companion and skill exchange typed run artifacts rather than assuming that file existence means success:

| Artifact/stage | Producer | Required for reader-doc success | Durable |
| --- | --- | --- | --- |
| Repository identity, commit, dirty state, visible-doc manifest | companion | yes | yes |
| Bounded semantic evidence and commands | companion/providers | yes | yes |
| Repository mental model | selected LLM | yes | yes |
| WikiPlan v2 and per-page contracts | selected LLM, validated by companion | yes | yes |
| Page drafts and claim/evidence links | selected LLM | yes | final accepted output only |
| Static/diagram/link/normalization report | companion | yes | yes |
| Semantic/onboarding/overlap report | evaluator with deterministic assertions | yes | yes |
| Revision gap list and attempt | evaluator + selected LLM | only after first failure | yes |

The result state distinguishes `evidence_ready`, `synthesis_incomplete`, `quality_failed`, and `reader_docs_ready`. Existing CLI success text and sync no-op behavior must not collapse these states.

Normalization before merge rejects duplicate frontmatter, duplicate canonical sections, unsupported renderer-specific tags, absolute temporary paths/`file://` links, malformed diagrams, and missing source provenance. Repository-relative links or stable source permalinks are emitted from normalized evidence identities, never copied from a temporary agent workspace.

## 4. Universal Reader Page Contract

Every reader-facing page follows this reading order unless its page-type contract justifies an omission:

1. Title, purpose, and the reader outcome.
2. Scope and explicit non-scope.
3. Plain-language mental model.
4. Responsibilities, boundaries, and relationships or ordered flow.
5. Implementation mapping only where it helps the reader act.
6. Change guidance: where to start, what can break, and how to verify.
7. Related pages and real open questions.
8. Optional evidence appendix.

Important claims carry citations next to the claim. A collapsed source list may appear at the end when it improves auditability, but it must not precede purpose and mental model by default. Evidence pages may remain inventory-oriented because their reader job is traceability.

Top-level pages summarize; topic pages explain. Evidence pages inventory. Glossary pages define project/domain language only.

## 5. Diagram Policy

| Information need | Format | Trigger |
| --- | --- | --- |
| System/context, architecture, component relationships | ASCII diagram | Three or more meaningful components/boundaries or non-obvious dependency direction |
| Ordered request/event/async flow | Mermaid `sequenceDiagram` | Multiple actors or a success/failure/async path |
| Lifecycle/state transitions | Mermaid `stateDiagram-v2` | State changes, guards, retries, or terminal states |
| Persistent relationships | Mermaid `erDiagram` or table | Multiple related entities and ownership/keys matter |
| Branching workflow | Mermaid `flowchart` | Decisions materially affect the outcome |

Every diagram slot names the reader question it answers. The rendered diagram states purpose and scope, labels nodes/edges, maps domain terms to code locations in surrounding prose, and links its evidence. Decorative diagrams and diagrams that merely restate an adjacent list fail quality review. Architecture, context, deployment, and static component diagrams follow the repository quality bar and use fenced ASCII text; Mermaid is reserved for sequence, state, flowchart, and ERD needs.

## 6. Canonical File Contracts

Only `QUICKSTART.md` and `conventions/OVERVIEW.md` are unconditional after successful init. Other files are created only when their reader job is evidence-backed.

| File or page type | Reader outcome | Required content | Diagram default | Must not become |
| --- | --- | --- | --- | --- |
| `QUICKSTART.md` | Form a five-minute project mental model and choose the next page. | Product/repository purpose, current status, prerequisites, shortest verified run path, system-at-a-glance, major concepts, recommended reading paths by task, limitations/freshness. | One context or architecture sketch only when it materially shortens the explanation. | Feature dump, full file list, symbol list, or duplicate architecture page. |
| `SOURCE-MAP.md` | Map a concept or change request to its owning subsystem and starting evidence. | Semantic subsystem map with responsibility, boundary, dependencies, entrypoints, likely change locations, and canonical doc link. | ASCII component/dependency map for three or more non-obvious boundaries. | Repository tree, exhaustive package table, or lexical-import dump. |
| `architecture/OVERVIEW.md` | Understand runtime boundaries and dependency direction. | System context, layers/components, ownership, runtime/executor boundaries, control/data paths, external systems, constraints, high-risk surfaces, links to deep dives. | ASCII architecture overview is required when three or more meaningful components exist. | Ordered call trace, every class/file, or unexplained box diagram. |
| `architecture/DECISIONS.md` | Understand why durable architectural choices exist and their consequences. | Decision, status/confidence, context, rationale, alternatives when evidenced, consequences, change impact, source/ADR/Git links. | None by default. | Commit log, speculative rationale presented as fact, or duplicate ADR text. |
| `domain/OVERVIEW.md` | Learn the product language and invariants that code implements. | Actors, concepts, relationships, invariants, lifecycle/business rules, ownership boundaries, examples, exceptions, canonical implementation areas. | ERD/state/table only when relationships or lifecycle materially matter. | Struct/enum catalog or generic business-domain essay. |
| `workflows/OVERVIEW.md` and workflow topic | Trace an important user/system flow and its failure behavior. | Goal, actors, preconditions, trigger, ordered happy path, alternatives/failures, state/data side effects, recovery/idempotency, observability, change/verification guidance. | Mermaid sequence for multi-actor/async order; state diagram for lifecycle; flowchart for material branching. | UI click list without system effects or prose that omits failure paths. |
| `components/<topic>.md` or semantic component topic | Safely change one subsystem without confusing its responsibilities with collaborators. | Responsibility, non-responsibility, public boundary, collaborators, lifecycle, state ownership, extension points, risks, tests, related flows. | ASCII component map or relevant sequence/state diagram when triggered. | One-file wrapper, method reference, or duplicate overview content. |
| `data-models/OVERVIEW.md` and data topic | Understand persistent ownership and invariants before changing data. | Stores/schemas, entity ownership, relationships/keys, invariants, lifecycle, migrations, consistency, retention/sensitivity, write/read paths. | Mermaid ERD or table when relationships matter. | Type list without ownership, invariants, or migration impact. |
| `api/OVERVIEW.md` and interface topic | Use or evolve an external/internal contract safely. | Consumers, protocol/interface families, authentication/authorization, request/event lifecycle, errors, compatibility/versioning, representative examples, change/testing guidance. | Sequence diagram when interaction order matters. | Exhaustive generated method list or undocumented signature dump. |
| `operations/RUNBOOK.md` | Set up, run, package/deploy, observe, and diagnose the system with verified steps. | Prerequisites, configuration names without secrets, verified commands, expected success, packaging/deploy topology, logs/metrics, common failure diagnosis, rollback/recovery where applicable. | ASCII deployment diagram when multiple runtime environments/services exist. | Unverified command collection or secret-bearing setup guide. |
| `testing/STRATEGY.md` | Select the right proof for a change. | Test layers and ownership, exact verified commands, fixture/environment needs, change-to-test matrix, known gaps/flakiness, interpretation of failures. | Table by default. | Test-file inventory or claims of coverage without evidence. |
| `conventions/OVERVIEW.md` | Apply repository-adopted patterns and recognize exceptions. | Convention, scope, evidence, confidence, examples, counterexample/exception, rationale when evidenced, change impact. | None by default. | Generic ecosystem best practices or single-example inference labeled as convention. |
| `GLOSSARY.md` | Translate project-specific terms and aliases into the canonical concept/page. | Term, project meaning, aliases, disambiguation, canonical page, evidence when non-obvious. | None. | Identifier/symbol index or standard programming vocabulary. |
| `OPEN-QUESTIONS.md` | See uncertainties that block understanding or safe change and how to resolve them. | Question, impact, attempted evidence, confidence, owner if known, next evidence/action, affected pages. | None. | Generic TODO backlog or unanswered questions with no impact. |
| `evidence/**` | Audit claims and reproduce evidence collection. | Source manifest, commands/results, claim/evidence links, confidence/staleness, generator/source metadata. | None by default. | Primary onboarding path or duplicated narrative docs. |

## 7. Dynamic Topic Rules

- A dynamic page uses the closest canonical semantic section; `areas/**` is not a default destination.
- The page title names the developer concept, not a directory, manifest, or source file.
- The page must have a unique reader job and at least one substantial boundary, workflow, invariant, integration, or high-risk change surface.
- The page must link upward to its parent and sideways only to directly related concepts.
- If its required explanation fits as one subsection of the parent, merge it instead of creating a page.
- Page-specific required sections come from WikiPlan; generators must not paste every universal heading when a section is irrelevant.

## 8. Cross-Page Review

Before acceptance, review the wiki as one reading surface:

- every important concept has exactly one canonical home;
- Quickstart links every major section and offers task-oriented reading paths;
- parent/child, prerequisite, and related-page links are valid and non-circular where cycles are forbidden;
- summaries link to deeper pages instead of repeating their implementation detail;
- no page depends on a concept that is neither explained nor linked;
- terminology is consistent across pages and glossary aliases resolve ambiguity;
- important plan gaps appear in Quickstart limitations or `OPEN-QUESTIONS.md`, not as silent omissions.

## 9. Static Guardrails

- Reject absolute local links and `file://` URLs.
- Reject reader pages dominated by `N symbols`, `N imports`, evidence IDs, or lexical-import rows.
- Reject dynamic one-file/config pages without a semantic boundary.
- Reject required diagrams that are absent, unlabeled, malformed, or unsupported by prose/evidence.
- Reject orphan topic pages and competing entrypoints.
- Reject reader pages whose first substantive block is a source inventory unless the approved page type is evidence/reference.
- Reject arbitrary minimum source/citation/page/diagram counts in acceptance logic.
- Reject page drafts that do not answer their declared reader questions or violate canonical concept ownership.
- Reject multiple frontmatter blocks, duplicate canonical sections, unsupported renderer tags, and absent source revision/dirty-state metadata.
- Reject reader-doc success when any required LLM stage or run artifact is missing.

## 10. Verification And Reconciliation

- Add unit/integration checks in docs/core crates and init/sync regression tests.
- Render and inspect representative ASCII/Mermaid output.
- Validate Mermaid syntax and fail the page rather than caching/rendering an error string as successful content.
- Test cross-page overlap/ownership and task-oriented reading paths.
- Add a red orchestration regression proving the current companion incorrectly succeeds without mental-model/WikiPlan/page-synthesis artifacts.
- Add Grok-export-inspired artifact-hygiene fixtures for duplicated frontmatter/sections, temporary absolute links, renderer tags, and missing provenance.
- Verify ADR-0009 human-edit preservation.
- Update skill references, requirements, architecture, standards, validation, context, traceability, and changelog.

## 11. Prompt-Contract Hardening Follow-Up

The user approved a follow-up on 2026-07-14 after reviewing the latest generated docs and the OpenWiki/deepwiki-open prompt patterns. This follow-up clarifies the accepted contract without introducing count-based quality proxies:

- Keep a compact always-active reader contract in `SKILL.md`: preflight before reader writes, current-working-tree source inspection, mental model and WikiPlan before drafting, explanation before source inventory, explicit hypotheses, and final isolated evaluation/validation.
- Treat the current filesystem working tree as init's source of truth. Git supplies identity, provenance, dirty/change context, and history; it does not replace reading current tracked, modified, staged, and relevant untracked source.
- Add concise failure examples to `reader-first.md` so agents can recognize inventory-first prose, unsupported architecture claims, decorative diagrams, duplicate page ownership, and implementation detail without reader outcome.
- Expose the implemented `RepositoryMentalModel` fields as the planning checklist rather than inventing a second template.
- Move the approved page-type `reader outcome / required content / must not become` contract into the always-loaded docs-structure reference.
- Expose the existing canonical claim confidence labels (`confirmed`, `source-backed`, `hypothesis`, `watchlist`) without adding `inferred` to the generic claim model.
- Keep page length, file count, source count, diagram count, and repository-size thresholds out of acceptance logic. Coverage is proven by reader questions, evidence, known unknowns, and safe-change tasks.

Regression coverage must fail when the packaged skill omits these entry/reference invariants. The package and skill/reference contract versions advance so installed copies cannot silently appear equivalent to the prior prompt contract.
