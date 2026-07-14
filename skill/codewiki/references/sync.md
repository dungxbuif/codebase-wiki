# CodeWiki Sync Workflow

Load this reference when the user asks to update, refresh, reconcile, or sync an existing CodeWiki after repository changes.

Also load `docs-structure.md`.
Also load `conventions.md`.
Also load `workspace-placement.md` when source repo and wiki workspace may differ.
Also load `source-extensions.md` when `.agents/skills/codewiki/project/sources.yml` contains non-Git sources or the user asks to include them.

## Goal

Update only the wiki content affected by repository or documentation changes. Preserve accurate docs and human-owned content. No-op when the wiki is already current.

## Procedure

1. Load current CodeWiki state.
   - Read `.agents/skills/codewiki/project/config.yml`, `.agents/skills/codewiki/project/plan.yml`, `.agents/skills/codewiki/project/AGENTS.md`, `.agents/skills/codewiki/project/sources.yml`, and `docs/QUICKSTART.md`.
   - Inspect the relevant existing generated pages before editing.
   - Treat their current text as input to the update, not disposable prior output.

2. Determine change scope.
   - Use Git status/diff/log as the default source of code changes.
   - Treat non-Git sources as optional source extension evidence from `.agents/skills/codewiki/project/sources.yml`.
   - Compare changed files to `.agents/skills/codewiki/project/plan.yml` coverage and page ownership.
   - Include human docs changes as source evidence changes.

3. Build a docs impact plan.
   - Map source/doc change -> affected claim/page -> edit needed -> evidence.
   - Classify affected page content as unmodified generated content, surrounding human content, manually edited generated content, or unverified legacy content.
   - If no claim/page is affected, do not edit generated docs.

4. Refresh evidence narrowly.
   - Read only changed files, directly related dependencies, and affected existing docs.
   - For relevant non-Git sources, invoke or ask for the user-provided source skill and consume its evidence packet.
   - Activate optional providers only when default Git/filesystem evidence is insufficient.
   - Refresh `conventions/OVERVIEW.md` when formatter/linter/build/test config, framework usage, representative patterns, or documented exceptions change.

5. Update docs surgically.
   - Prefer replacing stale claims over rewriting whole pages.
   - Do not make formatting-only edits.
   - Keep canonical concepts in one page and link from elsewhere.
   - Preserve human-owned sections unless the user explicitly asked to overwrite them.
   - A matching generated-body integrity hash proves only that the body was not manually edited since CodeWiki wrote it; only then may the companion refresh it automatically.
   - If the companion reports `preserved-human-edited-generated-region`, read the current page and perform a semantic merge: retain the user's contribution, refresh only stale evidence-backed claims, and preserve disagreements as explicit notes or open questions.
   - If the companion reports `preserved-unverified-legacy-generated-region`, preserve the page and reconcile it once before establishing a new integrity baseline.
   - Never resolve a docs conflict by copying the newly generated page wholesale over the current page.

6. Update control state.
   - Refresh `.agents/skills/codewiki/project/plan.yml` coverage, confidence, stale areas, and open questions.
   - Update `.agents/skills/codewiki/project/AGENTS.md` only when local CodeWiki rules or provider status changed.

7. Record verification.
   - Run relevant checks or record skip reasons.
   - Report preserved manual edits and any unresolved docs/source disagreement.
   - Note no-op outcome when docs are current.

## Diff Budget

- Small change set: update at most the directly affected page(s).
- Broad architecture/runtime change: update the canonical top-level pages plus affected area pages.
- If more than three pages need edits, re-check the impact plan before writing.
