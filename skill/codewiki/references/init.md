# CodeWiki Init Workflow

Load this reference when the user asks to initialize, generate, bootstrap, or create a CodeWiki for a repository.

Also load `docs-structure.md`.

## Goal

Create the initial repo-native semantic wiki without approval gates. The output is a useful, evidence-backed first complete wiki, not a raw file inventory and not a thin MVP.

## Procedure

1. Resolve repository identity.
   - Use Git remote, root path, branch, and HEAD when available.
   - Record dirty worktree status as context, not as an error.

2. Load existing context.
   - Read existing `.codewiki/config.yml`, `.codewiki/plan.yml`, `.codewiki/AGENTS.md`, and `docs/codewiki/**` if present.
   - Read existing human docs such as `README*`, `docs/**`, ADRs, runbooks, API docs, and contribution/setup docs.
   - Treat existing human docs as source evidence, not generated output.

3. Detect repository shape dynamically.
   - Identify languages, libraries, frameworks, package managers, entrypoints, test/build tools, service boundaries, schemas, configs, and docs.
   - Do not use core language/framework adapters.
   - Prefer targeted discovery over broad full-tree reads.

4. Explore source evidence.
   - Start with entrypoints, package/build config, routing/API/schema files, tests/evals, and representative domain files.
   - Expand scope only when evidence requires it.
   - Avoid secrets and generated/vendor/cache directories.

5. Create `.codewiki/plan.yml`.
   - Include repo identity, detected stack, proposed pages, coverage, evidence needs, confidence, stale/unknown areas, provider tool status, and refresh strategy.
   - If optional code intelligence is needed, record the trigger and selected provider.

6. Generate docs under `docs/codewiki/**`.
   - Always write `index.md`.
   - Create only evidence-backed canonical pages.
   - Use `areas/<area-slug>.md` only for substantial areas.
   - Mark uncertainties in `open-questions.md` and/or `.codewiki/plan.yml`.

7. Write `.codewiki/AGENTS.md`.
   - Explain docs-first Q&A order.
   - Record optional provider activation notes for this repository.
   - Keep it local to CodeWiki behavior; do not replace the repository root `AGENTS.md`.

8. Record verification.
   - Run safe, relevant commands when available and useful.
   - If verification is skipped, record why and residual risk in docs or plan.

## Output Quality Bar

- A new human can start at `docs/codewiki/index.md` and understand the repository.
- A future agent can answer common architecture and change-safety questions from the docs first.
- Every important claim has evidence or is marked as a hypothesis.
- The wiki explains why important code exists, not just where files are.

