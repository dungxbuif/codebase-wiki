# CodeWiki Sync Workflow

Load this reference when the user asks to update, refresh, reconcile, or sync an existing CodeWiki after repository changes.

Also load `docs-structure.md`.
Also load `workspace-placement.md` when source repo and wiki workspace may differ.
Also load `source-extensions.md` when `.codewiki/sources.yml` contains non-Git sources or the user asks to include them.

## Goal

Update only the wiki content affected by repository or documentation changes. Preserve accurate docs and human-owned content. No-op when the wiki is already current.

## Procedure

1. Load current CodeWiki state.
   - Read `.codewiki/config.yml`, `.codewiki/plan.yml`, `.codewiki/AGENTS.md`, `.codewiki/sources.yml`, and `docs/index.md`.
   - Inspect the relevant existing generated pages before editing.

2. Determine change scope.
   - Use Git status/diff/log as the default source of code changes.
   - Treat non-Git sources as optional source extension evidence from `.codewiki/sources.yml`.
   - Compare changed files to `.codewiki/plan.yml` coverage and page ownership.
   - Include human docs changes as source evidence changes.

3. Build a docs impact plan.
   - Map source/doc change -> affected claim/page -> edit needed -> evidence.
   - If no claim/page is affected, do not edit generated docs.

4. Refresh evidence narrowly.
   - Read only changed files, directly related dependencies, and affected existing docs.
   - For relevant non-Git sources, invoke or ask for the user-provided source skill and consume its evidence packet.
   - Activate optional providers only when default Git/filesystem evidence is insufficient.

5. Update docs surgically.
   - Prefer replacing stale claims over rewriting whole pages.
   - Do not make formatting-only edits.
   - Keep canonical concepts in one page and link from elsewhere.
   - Preserve human-owned sections unless the user asked to overwrite them.

6. Update control state.
   - Refresh `.codewiki/plan.yml` coverage, confidence, stale areas, and open questions.
   - Update `.codewiki/AGENTS.md` only when local CodeWiki rules or provider status changed.

7. Record verification.
   - Run relevant checks or record skip reasons.
   - Note no-op outcome when docs are current.

## Diff Budget

- Small change set: update at most the directly affected page(s).
- Broad architecture/runtime change: update the canonical top-level pages plus affected area pages.
- If more than three pages need edits, re-check the impact plan before writing.
