# CodeWiki Deep Research Workflow

Load this reference when a question is too broad, risky, weakly documented, or semantically complex for ordinary Q&A.

## Goal

Perform focused, iterative repository research without drifting into a generic repo summary. Save durable findings back into CodeWiki docs/state when they affect future understanding.

## Procedure

1. Restate the exact research topic.
   - Keep the scope narrow.
   - If the user asked about a file, feature, behavior, or risk, stay on that topic.

2. Load current docs and plan.
   - Start with `docs/codewiki/**` and `.codewiki/plan.yml`.
   - Identify what is known, stale, unknown, or contradicted.

3. Create a research plan.
   - List the specific code/docs areas to inspect.
   - List expected evidence types: files, symbols, tests, commands, Git history, provider results.

4. Investigate iteratively.
   - Each iteration should add new evidence, not repeat prior findings.
   - Expand to direct dependencies only when evidence requires it.
   - Use optional providers only when the semantic gap justifies them.

5. Synthesize findings.
   - Separate confirmed facts, likely interpretations, hypotheses, and open questions.
   - Cite evidence for durable claims.
   - Identify docs/state updates needed.

6. Reconcile if authorized by the user's request.
   - For documentation/update requests, update `docs/codewiki/**` and `.codewiki/plan.yml`.
   - For pure Q&A, answer and recommend sync if docs are stale.

## Research Output Shape

- Topic
- Confirmed findings
- Evidence
- Risks/unknowns
- Impact on docs or future changes
- Recommended update or next action

