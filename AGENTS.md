---
artifact_type: agent_gateway
version: 1
status: active
owner: human
parse_hint: yaml_frontmatter_plus_markdown_body
---

# Agent Gateway

This file is the mandatory gateway for every AI agent working in this repository.

## Rule -1: Local Runtime

Also follow the local instruction file:

```text
@/Users/dungxbuif/.codex/RTK.md
```

All shell commands MUST be prefixed with `rtk`.

## Rule 0: Response Format

For substantial repository work, every agent response should start with:

```text
Current Phase: <Hydration|Detail Design|Execution & Test|Reconciliation|Dehydration|Blocked> | Next action: <one concrete next action>
```

If a higher-priority system or product instruction requires a different response shape, keep the lifecycle discipline but follow the higher-priority instruction.

## Rule 1: Required Reading Order

Before changing code or docs, read:

1. `docs/CONTEXT.md`
2. `docs/work/BACKLOG.md`
3. `docs/standards/README.md`
4. `docs/standards/QUALITY_BAR.md`
5. `docs/standards/VALIDATION.md`
6. Active phase in `docs/work/phases/`, if one exists
7. Active ticket or bug in `docs/work/tickets/` or `docs/work/bugs/`
8. Relevant master docs only: `docs/requirements/`, `docs/architecture/`, `docs/decisions/`

For bug, failure, or unexpected behavior work, also read `docs/standards/DEBUGGING.md` before proposing fixes.

## Rule 2: Do Not Over-Scan

Do not scan the whole repository by default.

For brownfield work:

- Inspect only the touched module and direct dependencies.
- Record known unknowns.
- Expand scope only when evidence requires it.

For CodeWiki itself, repository exploration is a product capability. When implementing that capability, keep exploration rules explicit and evidence-bound in the design.

## Rule 3: Lifecycle

Every non-trivial task follows this lifecycle:

1. Hydration: load context, backlog queue, standards, and active work.
2. Detail Design: write or update a design for non-small work.
3. Execution & Test: implement and verify with real commands.
4. Reconciliation: update master docs and decisions when behavior changes.
5. Dehydration: update `docs/CONTEXT.md`, backlog, validation matrix, and task status.

## Rule 4: Backlog Queue

`docs/work/BACKLOG.md` is the runtime scheduler. It may reorder requirements, tickets, bugs, maintenance, and framework work based on priority and risk.

Backlog order decides what to consider next. Non-tiny work still requires an execution artifact: requirement slice, phase, ticket, or bug.

## Rule 5: Status Model

Use this status flow for phases, tickets, and bugs:

```text
draft -> ready -> in_progress -> blocked -> in_review -> verified -> done
```

Bug work may additionally record `reproduced`, `fixed`, and `regression_verified`, but the primary status must still use the shared flow.

## Rule 6: CodeWiki Product Bar

CodeWiki is not an MVP exercise. Treat it as a complete repo-native skill intended to work across repositories.

Core product constraints:

- Fully automatic `init`: the LLM explores source code and creates the initial wiki plan without approval gates.
- No language/framework-specific adapters in the core. Detect languages, libraries, frameworks, entrypoints, and architecture dynamically.
- Minimal tool surface: prefer Git, filesystem, SQLite, and one strong code-intelligence provider before adding more tools.
- Persist durable project state across model changes, session resets, and future sync runs.
- Separate committed config/docs from local runtime state and rebuildable cache.
- Evidence-first output: claims in generated wiki docs must trace back to files, symbols, commands, or explicit hypotheses.

## Rule 7: Human Vs AI Field Ownership

Templates use YAML frontmatter and body sections to separate responsibilities.

- Human fields define intent, priority, approval, acceptance, and business context.
- AI fields define analysis, proposed approach, evidence, docs reconciliation, and context updates.
- Shared fields may be updated by either human or AI, but AI must not overwrite human decisions silently.

If a human-owned field is missing and blocks execution, ask or mark the item `blocked`.

## Rule 8: Required Trace Links

Every work artifact must link backward and forward:

- Phase links to backlog, roadmap, tickets, bugs, verification, validation matrix, release notes, and ADRs.
- Ticket links to backlog, phase, detail design, tests, validation matrix, docs review, ADRs, and release notes.
- Bug links to backlog, phase, reproduction, detail design, regression tests, validation matrix, docs review, ADRs, and release notes.
- Detail design links back to ticket or bug and forward to tests, docs review, ADRs, and touched master docs.
- ADR links back to triggering ticket, bug, phase, and master docs.

## Rule 9: Completion Rules

A task is not complete until:

- Relevant tests were run and results were recorded, or the skip reason is explicit.
- `docs/work/VALIDATION_MATRIX.md` was updated when behavior proof changed.
- Acceptance/UAT was completed or explicitly marked not required.
- Master docs were reconciled with actual changes.
- Docs review checklist was completed.
- New decisions were logged as ADRs when needed.
- `docs/CONTEXT.md` reflects current state and next steps.
- `docs/work/BACKLOG.md` and work item status were updated.
- `docs/releases/CHANGELOG.md` or release notes were updated with user-facing or framework changes.

## Rule 10: Loop Guard

Agents MUST stop and ask for human/design review instead of continuing when:

- The same test or verification path fails after 3 fix attempts.
- The agent has run 5 total fix/test cycles on one work item.
- A fix attempt changes architecture, API, data, security, runtime, or standards beyond approved scope.
- Each fix reveals a new unrelated failure or broader design problem.

When this happens, set the work item to `blocked`, record the attempt log, summarize the best current root-cause hypothesis, and identify the decision or input needed.

