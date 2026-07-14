# Reader-First Planning, Synthesis, And Evaluation

Load this reference during every init/sync after deterministic discovery and before reader-facing docs are accepted.

## Runtime Boundary

CodeWiki has three mandatory boundaries:

```text
companion discovery/persistence
        -> LLM mental model, WikiPlan, page synthesis, bounded revision
        -> companion validation, normalization, safe merge, provenance
```

The companion may write `docs/evidence/**` and control-plane scaffolding. It must not be treated as the author of the final wiki. `generation_status: synthesis_incomplete` is expected after companion init/sync and is not an error.

## Planning Passes

1. Discover bounded source, documentation, Git, test, configuration, and command evidence.
2. Model systems, actors, responsibility boundaries, runtimes, workflows, state ownership, integrations, change risks, and known unknowns.
3. Architect a concept-first hierarchy with one canonical home per concept and task-oriented reading paths.
4. Contract every page before drafting.

Every planned reader page records:

- semantic page type, parent, order, importance, and canonical topic IDs;
- audience, unique reader job, prerequisites, and reader questions;
- scope and explicit non-scope;
- required sections and question-driven diagram slots;
- source anchors with a relevance reason and expected claims;
- evidence gaps, related pages, open questions, refresh triggers, and acceptance checks.

Never keep `pending-llm-selection` or `llm_semantic_planning_pending` in a completed WikiPlan. Do not create a page from a directory, config file, or filename alone.

## Page Reading Order

1. Purpose and reader outcome.
2. Scope and non-scope.
3. Plain-language mental model.
4. Responsibilities, boundaries, relationships, or ordered flow.
5. Implementation mapping needed for action.
6. Change guidance, risks, and verification.
7. Related pages and real open questions.
8. Optional evidence appendix.

Important claims receive claim-local evidence. Do not begin reader pages with a source inventory. Raw files, symbols, imports, and deterministic claims remain under `docs/evidence/**` or the control plane.

## Diagram Rules

- Use fenced ASCII for context, architecture, deployment, and static component relationships.
- Use Mermaid `sequenceDiagram` for multi-actor or asynchronous order.
- Use Mermaid `stateDiagram-v2` for lifecycles and guarded transitions.
- Use Mermaid `erDiagram` or a table for persistent ownership/relationships.
- Use Mermaid `flowchart` for material branching.

Every diagram slot names the reader question it answers. Decorative diagrams fail review.

## Quality And Revision

Before calling companion validation, write `.agents/skills/codewiki/project/quality-report.yml` with evidence for these gates:

```yaml
schema_version: 1
generation_model: "<model used for synthesis>"
evaluation_model: "<model used for evaluation>"
model_synthesis: pass
contract_coverage: pass
source_audit: pass
diagram_audit: pass
cross_page_review: pass
docs_only_onboarding: pass
reader_context: docs_only
source_auditor_context: source_and_evidence
critical_failures: 0
revision_attempts: 0
notes: "Concise evidence and remaining non-critical gaps."
```

Run a docs-only reader against reader pages without source, plan evidence, SQLite, or reference outputs. Separately audit important claims against source. Check canonical ownership, terminology, overlap, links, diagrams, change guidance, risks, and tests.

If a critical gate fails, revise the owning page once using named gaps and rerun the gates. After that, report `quality_failed`; do not loop or lower the gate.

Finally run:

```text
codewiki validate <wiki-workspace>
```

Only `generation_status: reader_docs_ready` means the reader wiki is complete.
