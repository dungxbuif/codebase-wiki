# CodeWiki Init Workflow

## Mandatory First Write

Before creating or changing any reader-facing Markdown, run the active installed skill's `scripts/codewiki-preflight.sh init <repository-path>`. If it fails, stop reader synthesis and report the failure. Direct Markdown generation without this control/evidence gate is not a CodeWiki init.

Load this reference when the user asks to initialize, generate, bootstrap, or create a CodeWiki for a repository.

Also load `docs-structure.md`.
Also load `conventions.md`.
Also load `reader-first.md`.
Also load `workspace-placement.md` when the output location is not explicit.
Also load `source-extensions.md` when the user mentions non-Git sources.

## Goal

Create the initial repo-native semantic wiki without approval gates. The output is a useful, evidence-backed first complete wiki, not a raw file inventory and not a thin MVP.

## Procedure

1. Resolve repository identity.
   - Use Git remote, root path, branch, and HEAD when available.
   - Record dirty worktree status as context, not as an error.
   - Treat Git as the default source for code changes.
   - The current working tree is the source of truth for init. Inspect tracked, staged, modified, and relevant untracked source files; record deleted paths as provenance.
   - Git metadata and diffs do not replace reading current source. `HEAD` identifies a base revision, not necessarily the code the developer is running.

2. Resolve wiki workspace placement.
   - If the user clearly wants repo-local docs, use the source repository as the workspace.
   - If the user wants personal/external docs, use a separate confirmed workspace.
   - If unclear, ask before writing files.
   - Never silently write docs into the source repo when the user asked for personal/external docs.

3. Load existing context.
   - Read existing `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, `.agents/skills/codewiki/project/AGENTS.md`, and `docs/**` if present.
   - Read existing human docs such as `README*`, `docs/**`, ADRs, runbooks, API docs, and contribution/setup docs.
   - Treat existing human docs as source evidence, not generated output.

4. Detect repository shape dynamically.
   - Identify languages, libraries, frameworks, package managers, entrypoints, test/build tools, service boundaries, schemas, configs, and docs.
   - Do not use core language/framework adapters.
   - Prefer targeted discovery over broad full-tree reads.

5. Explore source evidence.
   - Start with entrypoints, package/build config, routing/API/schema files, tests/evals, and representative domain files.
   - Discover project, language, framework/library, and area conventions from explicit config plus repeated code evidence.
   - Expand scope only when evidence requires it.
   - Avoid secrets and generated/vendor/cache directories.
   - Trace current runtime boundaries, control/data flow, state ownership, failure paths, and safe-change verification beyond lexical symbol/import hints.
   - Continue until every repository mental-model field and planned reader question has relevant evidence or an explicit known unknown. Do not use file counts or consecutive-file counts as completion proxies.
   - If important cross-file relationships remain unresolved after bounded filesystem exploration, activate Octocode when available and record the trigger; otherwise retain reduced confidence and the named gap.

6. Run companion evidence initialization when available.
   - Treat `generation_status: synthesis_incomplete` as the correct evidence-ready state.
   - The companion writes control scaffolding and `docs/evidence/**`; it does not complete reader docs.

7. Replace the scaffold with WikiPlan v2.
   - Record source commit/dirty state, visible docs, the repository mental model, concept ownership, hierarchy, reading order, and typed page contracts from `reader-first.md`.
   - Select source anchors for relevance, never to meet a count quota.
   - If optional code intelligence is needed, record the trigger and selected provider.

8. Create or update `.agents/skills/codewiki/project/sources.yml`.
   - Always include the primary Git source when a repository is involved.
   - Add non-Git sources only as declarations or user-provided source skill refs.
   - Do not assume CodeWiki has a built-in provider for Jira, Figma, or other systems.

9. Generate docs under `docs/**`.
   - Always write `QUICKSTART.md`; do not create `index.md` as a competing entrypoint.
   - Keep every generated Markdown filename uppercase while keeping directories lowercase.
   - Create only evidence-backed canonical pages.
   - Create dynamic pages under semantic owners; do not regenerate path-derived `areas/**` pages.
   - Always write `conventions/OVERVIEW.md` from the convention discovery contract; record hypotheses and conflicts when evidence is incomplete.
   - Mark uncertainties in `OPEN-QUESTIONS.md` and/or `.agents/skills/codewiki/project/plan.yml`.

10. Write `.agents/skills/codewiki/project/AGENTS.md`.
   - Explain docs-first Q&A order.
   - Record optional provider activation notes for this repository.
   - Keep it local to CodeWiki behavior; do not replace the repository root `AGENTS.md`.

11. Evaluate, revise once at most, and validate.
   - Run isolated docs-only onboarding and a separate source audit.
   - Write `.agents/skills/codewiki/project/quality-report.yml` using `reader-first.md`.
   - Run `codewiki validate <workspace>`; do not report completion unless it returns `reader_docs_ready`.

12. Record verification.
   - Run safe, relevant commands when available and useful.
   - If verification is skipped, record why and residual risk in docs or plan.

## Output Quality Bar

- A new human can start at `docs/QUICKSTART.md` and understand the repository.
- A future agent can answer common architecture and change-safety questions from the docs first.
- Every important claim has evidence or is marked as a hypothesis.
- The wiki explains why important code exists, not just where files are.
