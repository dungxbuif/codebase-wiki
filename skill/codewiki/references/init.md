# CodeWiki Init Workflow

Load this reference when the user asks to initialize, generate, bootstrap, or create a CodeWiki for a repository.

Also load `docs-structure.md`.
Also load `workspace-placement.md` when the output location is not explicit.
Also load `source-extensions.md` when the user mentions non-Git sources.

## Goal

Create the initial repo-native semantic wiki without approval gates. The output is a useful, evidence-backed first complete wiki, not a raw file inventory and not a thin MVP.

## Procedure

1. Resolve repository identity.
   - Use Git remote, root path, branch, and HEAD when available.
   - Record dirty worktree status as context, not as an error.
   - Treat Git as the default source for code changes.

2. Resolve wiki workspace placement.
   - If the user clearly wants repo-local docs, use the source repository as the workspace.
   - If the user wants personal/external docs, use a separate confirmed workspace.
   - If unclear, ask before writing files.
   - Never silently write docs into the source repo when the user asked for personal/external docs.

3. Load existing context.
   - Read existing `.codewiki/config.yml`, `.codewiki/plan.yml`, `.codewiki/AGENTS.md`, and `docs/codewiki/**` if present.
   - Read existing human docs such as `README*`, `docs/**`, ADRs, runbooks, API docs, and contribution/setup docs.
   - Treat existing human docs as source evidence, not generated output.

4. Detect repository shape dynamically.
   - Identify languages, libraries, frameworks, package managers, entrypoints, test/build tools, service boundaries, schemas, configs, and docs.
   - Do not use core language/framework adapters.
   - Prefer targeted discovery over broad full-tree reads.

5. Explore source evidence.
   - Start with entrypoints, package/build config, routing/API/schema files, tests/evals, and representative domain files.
   - Expand scope only when evidence requires it.
   - Avoid secrets and generated/vendor/cache directories.

6. Create `.codewiki/plan.yml`.
   - Include repo identity, detected stack, proposed pages, coverage, evidence needs, confidence, stale/unknown areas, provider tool status, and refresh strategy.
   - If optional code intelligence is needed, record the trigger and selected provider.

7. Create or update `.codewiki/sources.yml`.
   - Always include the primary Git source when a repository is involved.
   - Add non-Git sources only as declarations or user-provided source skill refs.
   - Do not assume CodeWiki has a built-in provider for Jira, Figma, or other systems.

8. Generate docs under `docs/codewiki/**`.
   - Always write `index.md`.
   - Create only evidence-backed canonical pages.
   - Use `areas/<area-slug>.md` only for substantial areas.
   - Mark uncertainties in `open-questions.md` and/or `.codewiki/plan.yml`.

9. Write `.codewiki/AGENTS.md`.
   - Explain docs-first Q&A order.
   - Record optional provider activation notes for this repository.
   - Keep it local to CodeWiki behavior; do not replace the repository root `AGENTS.md`.

10. Record verification.
   - Run safe, relevant commands when available and useful.
   - If verification is skipped, record why and residual risk in docs or plan.

## Output Quality Bar

- A new human can start at `docs/codewiki/index.md` and understand the repository.
- A future agent can answer common architecture and change-safety questions from the docs first.
- Every important claim has evidence or is marked as a hypothesis.
- The wiki explains why important code exists, not just where files are.
