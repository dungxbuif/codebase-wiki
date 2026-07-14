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

## Mental Model Completion Checklist

Complete the persisted repository mental model before page architecture. Every item must cite relevant evidence or remain an explicit known unknown:

- `systems`: major products, services, applications, or libraries;
- `actors`: people, external systems, scheduled work, and runtime initiators;
- `boundaries`: responsibility, ownership, trust, and dependency direction;
- `runtimes`: processes, executors, threads, workers, and deployment contexts;
- `workflows`: critical end-to-end success, failure, retry, or recovery paths;
- `state ownership`: durable and in-memory state, authoritative writers, and observers;
- `integrations`: external services, protocols, platform/native boundaries, and generated contracts;
- `change risks`: high-blast-radius surfaces and their verification paths;
- `known unknowns`: unresolved questions, inspected evidence, impact, and next evidence needed.

Do not start page drafting while a critical runtime boundary, state owner, or workflow is silently unknown. Expand source exploration, activate the justified optional provider, or preserve the uncertainty explicitly.

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

## Automatic Failure Patterns

Fail the relevant quality gate when a reader page:

- opens with a file list, directory tree, symbol table, or evidence inventory instead of purpose and explanation, except for an approved evidence/reference page;
- treats a directory, file, class, function, lexical import, or framework name as a system boundary without explaining responsibility and runtime evidence;
- says what a component contains but not why it exists, what it owns, who collaborates with it, or how a developer changes it safely;
- presents an architecture, ownership, workflow, or convention claim without a local source/command/doc anchor or explicit hypothesis marker;
- includes a diagram without stating the reader question, purpose, scope, or meaning of its important edges;
- duplicates detailed ownership across pages instead of selecting one canonical home;
- says "see the source" without a specific starting anchor and relevance reason;
- reports an unverified command, start path, or safety check as verified;
- leaves a declared reader question or acceptance check unanswered without recording a known unknown.

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

## Docs-Only Onboarding Evaluation

Evaluate two complementary question sets without source access:

1. Portable questions: what the repository does and for whom; how to reach a verified start path; which major boundaries own what; how one critical workflow succeeds and fails; where a representative change should begin; what can break; and how to verify it.
2. Capability-specific questions derived from the mental model and page contracts. Ask about API, persistence, authentication, deployment, eventing, native integration, media, or similar capabilities only when evidence shows that the repository has them.

For every critical question, record the expected concepts, canonical owning page, required evidence, critical misconceptions, answer result, and any explicit known unknown. Page-contract `reader_questions` and `acceptance_checks` remain authoritative; a fixed generic question must not force an irrelevant page into the wiki.

If a critical gate fails, revise the owning page once using named gaps and rerun the gates. After that, report `quality_failed`; do not loop or lower the gate.

Finally run:

```text
codewiki validate <wiki-workspace>
```

Only `generation_status: reader_docs_ready` means the reader wiki is complete.
