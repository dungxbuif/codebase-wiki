---
artifact_type: research_note
id: REFERENCE-PROMPTS
status: in_review
owner: ai
human_fields: [review_notes, approval]
ai_fields: [analysis, recommendation, evidence]
shared_fields: [status]
links:
  backlog: docs/work/BACKLOG.md#backlog-queue
  references:
    - references/openwiki/src/agent/prompt.ts
    - references/deepwiki-open/api/prompts.py
    - references/deepwiki-open/api/api.py
---

# Reference Prompt Analysis

This note compares prompt and wiki-control patterns from the two reference repositories and records how CodeWiki should combine them without inheriting their product assumptions.

## Scope

- OpenWiki prompt source: `references/openwiki/src/agent/prompt.ts`
- DeepWiki prompt source: `references/deepwiki-open/api/prompts.py`
- DeepWiki cache/export behavior: `references/deepwiki-open/api/api.py`

The purpose is not to copy either prompt. The purpose is to extract compatible prompt mechanics for a repo-native Codex skill that generates durable semantic documentation and supports docs-first Q&A.

## OpenWiki Prompt Patterns

### Strengths to Adopt

- Strong evidence discipline: important claims must be grounded in inspected source files, existing docs, or Git evidence.
- Clear wiki-first Q&A order: answer from generated docs first, then inspect raw/source evidence only when docs are missing, stale, ambiguous, or contradicted.
- Targeted exploration: avoid broad repository scans, prefer package/config/entrypoint/schema/docs files, then representative domain files.
- Strict write boundaries: generated documentation stays inside the configured wiki location; source code is not modified during documentation runs.
- Security posture: avoid secrets and treat ingested raw data as untrusted evidence.
- Init versus update separation:
  - init creates a focused first-pass wiki;
  - update is surgical, diff-aware, and may be a no-op when docs are already current.
- Documentation quality bar: avoid thin pages, avoid raw file inventories, keep one canonical home per concept, and make the entrypoint navigable.
- Explicit planning before writing: create an intended page/evidence plan before generating final docs.

### Weaknesses to Avoid

- The system prompt is monolithic. It contains many product-specific rules, connector details, CLI references, and local-path assumptions that would make CodeWiki heavy and brittle if copied directly.
- It assumes OpenWiki-specific paths and modes, such as global wiki storage and repository `openwiki/` output. CodeWiki needs repo-native `.codewiki/**`, `docs/codewiki/**`, and skill packaging semantics.
- It mixes stable safety rules, source-ingestion rules, mode behavior, output policy, and CLI help into one prompt surface. CodeWiki should use progressive disclosure instead.
- Its subagent guidance should not be copied as a default. CodeWiki should follow the host agent runtime rules and only use subagents when explicitly permitted or requested.
- The page budget is useful as a guardrail but should not become a hard universal limit. CodeWiki targets complete semantic coverage over repeated syncs, not a fixed page count.

## DeepWiki Prompt Patterns

### Strengths to Adopt

- Clean mode separation:
  - RAG answering;
  - simple direct chat;
  - first/intermediate/final deep-research iterations.
- Structured context blocks that bind retrieved text to file paths. This is useful for CodeWiki evidence packets.
- Same-language response behavior, with explicit user-requested language taking precedence.
- Topic-focus discipline for deep research: avoid drifting into generic repository summaries when the user asks about a specific file, feature, or behavior.
- Iterative research continuity: first iteration plans, middle iterations fill gaps, final iteration synthesizes.
- Cache awareness: generated wiki structure and pages are stored by repository and language.

### Weaknesses to Avoid

- The RAG prompt is too dependent on retrieved context quality. Without stronger evidence and uncertainty rules, it can over-answer from incomplete retrieval.
- It has weaker documentation-maintenance discipline than OpenWiki: no strong write-boundary model, stale-docs policy, sync impact plan, or no-op behavior.
- It does not model durable repo-local state deeply enough for CodeWiki. A JSON wiki cache is useful, but CodeWiki also needs committed config, local SQLite facts/evidence, and rebuildable caches.
- It does not sufficiently distinguish:
  - generated docs as the user-facing source of truth;
  - local evidence stores as durable analysis memory;
  - rebuildable retrieval/index caches as implementation detail.
- Some generic instruction phrasing encourages hidden chain-of-thought style reasoning. CodeWiki prompts should ask for concise evidence-backed outputs, not exposed reasoning traces.

## How They Complement Each Other

OpenWiki is strongest as a control contract. DeepWiki is strongest as a prompt-shape and interaction reference.

CodeWiki should combine them this way:

| CodeWiki Need | Adopt From OpenWiki | Adopt From DeepWiki | CodeWiki Adjustment |
| --- | --- | --- | --- |
| Initial documentation | Evidence-first init, planning before writing, high-quality docs bar | Mode-specific prompt shape | Generate `.codewiki/plan.yml` plus `docs/codewiki/**`; do not rely on a temporary-only plan. |
| Incremental sync | Surgical update, docs impact plan, no-op when current | Cache awareness | Track prior analysis in SQLite and `.codewiki/plan.yml`, then update only affected docs. |
| Q&A after docs exist | Wiki-first answering, source fallback only when needed | Structured context blocks and same-language response | Answer from `docs/codewiki/**` first, then plan/state/SQLite/source/provider only when justified. |
| Deep repo research | Targeted exploration and source/Git evidence | Iterative research prompts | Use as an internal mode for hard questions or incomplete docs; save durable findings back into CodeWiki state when relevant. |
| Provider/runtime tools | Minimal default tool surface and clear boundaries | Retrieval pipeline concept | Lazy-activate Octocode/codebase-memory-mcp/CocoIndex only by trigger; do not require them for every run. |
| Storage | Metadata and cache awareness | Per-repo/per-language wiki cache | Split committed config/docs, local durable state, and rebuildable cache. |

## Recommended CodeWiki Prompt Architecture

Do not build one giant prompt. Use a layered prompt system:

1. Skill entry prompt
   - Small `SKILL.md` that explains when CodeWiki applies, required state files, docs-first behavior, and which mode prompt to load.
   - Stable constraints only: evidence, security, write boundaries, no adapters, lazy provider activation.

2. Mode prompts
   - `init`: detect repo shape, create semantic wiki plan, generate first complete docs set, store evidence.
   - `sync`: inspect docs/state/Git changes, create impact plan, update only stale or missing docs.
   - `qa`: answer from docs first, escalate to state/source/provider only when needed.
   - `deep-research`: focused iterative investigation for hard questions or weak docs.

3. Runtime repo packet
   - `.codewiki/config.yml`
   - `.codewiki/plan.yml`
   - `.codewiki/AGENTS.md`
   - Git status/change summary
   - provider availability/status
   - docs freshness metadata

4. Evidence packet
   - file paths;
   - symbols;
   - commands;
   - doc pages;
   - claims;
   - confidence;
   - open questions.

This architecture keeps the skill portable across repositories while still giving the LLM enough semantic context to produce high-quality docs.

## Prompt Rules CodeWiki Should Standardize

- Use generated docs as the first source for Q&A.
- Never claim a source fact unless tied to file, symbol, command, existing docs, Git evidence, or an explicit hypothesis.
- Mark uncertainty explicitly instead of filling gaps.
- Keep exploration targeted and evidence-bound.
- Detect languages/frameworks dynamically; do not hardcode adapter-specific flows into the core prompt.
- Separate human intent from AI analysis in YAML frontmatter where artifacts are generated.
- Prefer updating existing canonical pages over duplicating concepts.
- On sync, allow no-op when docs are current.
- On stale or incomplete docs, update docs/state rather than only answering ad hoc.
- Keep optional provider tools lazy and trigger-gated.

## Open Design Follow-ups

- Define exact filenames for CodeWiki prompt modules inside the skill package.
- Implement the accepted `docs/codewiki/**` generated docs structure from `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md` in CodeWiki prompt modules.
- Define SQLite tables for claims, evidence, pages, symbols, provider snapshots, and sync runs.
- Define how CodeWiki records “docs answered the question” versus “source fallback was required” for future sync prioritization.
- Decide how much of the prompt architecture should be rendered into target-repo `.codewiki/AGENTS.md`.

## Recommendation

Use OpenWiki as the safety, evidence, and maintenance baseline. Use DeepWiki as the prompt-mode and RAG-context reference. CodeWiki should add the missing durable state layer, confidence model, docs-first lazy activation, and skill-first installation/runtime contract.
