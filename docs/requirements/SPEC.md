---
artifact_type: requirement_spec
id: REQ-MASTER
status: active
owner: human
human_fields: [product_summary, goals, non_goals, users_and_stakeholders, acceptance_criteria]
ai_fields: [functional_requirements, non_functional_requirements, constraints, linked_decisions]
shared_fields: [status, trace]
---

# Product Specification: CodeWiki

## Field Ownership

- Human owns product intent, goals, non-goals, users, and acceptance criteria.
- AI may draft requirements, constraints, and linked decisions for review.

## Product Summary

CodeWiki is a Codex skill that builds and maintains a semantic wiki for software repositories and personal/external code knowledge workspaces. It should autonomously explore source code, detect the repository's languages and frameworks, create an evidence-backed WikiPlan, generate docs in a user-confirmed workspace, preserve durable state, and support docs-first Q&A and future sync runs. Rust code in this repository is companion tooling for deterministic local operations, not the primary product surface.

## Goals

- Generate high-quality semantic wiki docs for arbitrary repositories.
- Support repo-local docs and external/personal wiki workspaces.
- Make `init` automatic: no approval gate should be required for the LLM to explore and create the first wiki plan.
- Preserve useful config, storage, evidence, and checkpoints across sessions and LLM/model changes.
- Keep generated documentation trustworthy by linking claims to evidence.
- Keep the core small and portable, with replaceable provider boundaries.
- Provide a repository-installable Codex skill as the primary execution surface.
- Provide Rust companion tooling only where deterministic local behavior is useful.

## Non-Goals

- A limited MVP that cannot grow into the complete CodeWiki product.
- Core language/framework-specific adapters.
- A broad mandatory tool stack with overlapping code-intelligence systems.
- Treating chat history or chain-of-thought as durable project memory.
- Bundling Jira, Figma, or other non-Git source providers into CodeWiki core.

## Users And Stakeholders

- Developers who want fast, accurate onboarding to an unfamiliar repository.
- AI coding agents that need durable, repo-local context before changing code.
- Maintainers who want generated docs that can be refreshed and audited.
- Developers who want personal docs outside the source repository.

## Functional Requirements

- `init` explores the repository, detects stack signals, creates a WikiPlan, and generates initial docs.
- The primary product is installable as a Codex skill from this repository.
- Rust helper tooling may support deterministic local operations for the skill.
- Detection covers languages, libraries, frameworks, package managers, entrypoints, tests, build tools, service boundaries, and documentation sources where possible.
- WikiPlan records pages, scope, evidence needs, open questions, confidence, and update strategy.
- Generated docs live in committed project space under the canonical `docs/**` structure.
- Generated docs may live in the source repository or in an external/personal wiki workspace confirmed with the user.
- `docs/QUICKSTART.md` is the required generated entrypoint after successful init.
- Generated Markdown filenames are uppercase while generated wiki directories remain lowercase.
- Canonical generated docs slots include quickstart, source map, architecture, domain, workflows, data models, API/interfaces, operations, testing, decisions, glossary, open questions, evidence, and optional area pages.
- Canonical generated docs include `docs/conventions/OVERVIEW.md`, where the LLM discovers project, language, framework/library, and area conventions from explicit configuration plus repeated source evidence.
- Convention claims distinguish explicit rules, inferred patterns, hypotheses, and exceptions; inferred conventions require multiple independent examples or remain hypotheses.
- Project config lives in committed `.agents/skills/codewiki/project/config.yml`.
- Plan/state summaries that should travel with the repo live in committed `.agents/skills/codewiki/project/plan.yml` or equivalent.
- Target-repo CodeWiki agent guidance lives in committed `.agents/skills/codewiki/project/AGENTS.md`.
- Source declarations live in `.agents/skills/codewiki/project/sources.yml`.
- Git is the default source for code changes.
- Non-Git sources are supported through user-provided source extension skills, not bundled providers.
- Local runtime state lives outside the repo in SQLite and is keyed by repository identity.
- Rebuildable cache is separated from durable runtime state.
- Q&A should answer from generated docs first, then use `codewiki query`/`codewiki claims` for repository-scoped local SQLite context before source evidence fallback.
- Local retrieval separates active/stale claims and can match bounded file, symbol, and evidence metadata without dumping source contents or requiring agents to author SQL.
- Q&A should activate external runtime tools only after docs, plan, agent instructions, local SQLite evidence, and source/Git inspection are insufficient for the requested answer.
- Sync detects stale docs, records portable generated-body integrity, refreshes only verified-unchanged generated bodies automatically, and preserves manual edits for semantic reconciliation.
- WikiPlan records an evidence-backed repository mental model before creating a hierarchy, canonical concept ownership, reading order, and per-page contracts.
- WikiPlan page contracts record reader job, audience, prerequisites, reader questions, semantic scope/non-scope, required sections, question-driven diagram slots, evidence anchors with relevance reasons, related pages, open questions, refresh triggers, and acceptance checks.
- Canonical pages remain stable entrypoints while dynamic pages may be created for evidence-backed systems, components, workflows, platform boundaries, and framework concepts.
- Reader-facing docs explain system concepts, responsibilities, boundaries, interactions, workflows, change risks, and verification guidance before presenting source symbols or evidence details.
- Raw file/symbol/import inventories remain in evidence or planning layers rather than becoming the primary reader-facing content.
- Reader-facing pages begin with purpose, scope, and mental model; claim-local citations follow the explanation, while source inventories are optional appendices or evidence-page content.
- Generated documentation passes static quality checks and docs-only developer-onboarding evaluation before a high-quality init or sync is considered complete.
- Quality evaluation includes contract coverage, evidence relevance, diagram validity/information gain, cross-page ownership/overlap, docs-only onboarding tasks, and human UAT; arbitrary file/page/citation/diagram counts are not quality proxies.
- The installed skill, references, helper, and companion expose versioned package/install provenance, managed-content integrity, and compatibility status.
- Every generation and benchmark records the resolved skill root, installed content identity, contract versions, and companion compatibility; legacy, drifted, or incompatible installations cannot report high-quality reader-doc completion.

## Non-Functional Requirements

- Works across repositories without core adapters per language/framework.
- Rust companion crates should use typed schemas, explicit errors, deterministic filesystem behavior, and testable provider boundaries.
- Provider boundaries are replaceable.
- Durable state has migrations from the start.
- Outputs are evidence-first and auditable.
- Convention documentation must describe the repository's actual practices rather than generic ecosystem best practices.
- The tool remains usable after session reset or model change.
- The generated wiki should be readable by humans and useful to agents.
- Manual documentation contributions remain durable across sync, machine, session, and model changes.
- Generated documentation quality is model-independent: the same reader, evidence, diagram, and onboarding contracts apply across model providers.
- New developers should be able to form a system mental model and begin a bounded first change from the generated docs without source-first repository exploration.

## Constraints

- Keep tool count intentionally small.
- Do not make Octocode, codebase-memory-mcp, or CocoIndex bundled skill dependencies.
- Use Octocode as the default first-choice code-intelligence provider when a provider is needed.
- Use codebase-memory-mcp only when cross-session shared memory beyond CodeWiki's SQLite state is needed.
- Use CocoIndex only when repo size or repeated refresh/query workload justifies an indexing pipeline.
- Provide install/activation guidance for runtime tools in the skill and target-repo CodeWiki agent instructions.
- Keep committed config/docs separate from persistent local runtime state and rebuildable cache.
- Treat `docs/**` as the generated knowledge surface and `.agents/skills/codewiki/project/**` as the committed control plane.
- Ask for/confirm output workspace placement before writing when repo-local versus external/personal storage is ambiguous.
- Use OpenWiki and deepwiki-open as technical references only; do not inherit their runtime architecture wholesale.
- Install command should copy/install `skill/codewiki` into the target workspace at `.agents/skills/codewiki` by default.

## Acceptance Criteria

- A fixture repository can be initialized into a wiki with a structured WikiPlan and generated docs.
- Generated claims include evidence links or explicit hypothesis markers.
- Re-running with an existing config/state reuses prior facts and checkpoints.
- Changing model/provider does not require discarding durable state.
- Detection behavior is validated on at least several different repository shapes before release.
- Critical onboarding questions are answerable from generated docs alone with evidence and without critical architecture or ownership hallucinations.
- Benchmark comparisons pin source commit, dirty/submodule state, visible existing docs, evidence scope, and generation/evaluation contract versions.
- Generated topic pages reflect semantic responsibilities and workflows rather than top-level filesystem paths.
- Architecture, workflow, state, and data pages include the appropriate diagram when the documented relationships materially benefit from one.
- A pinned install smoke proves managed skill/reference/helper/companion integrity, deliberate drift is detected, incompatible contracts block reader-doc success, and declared project state survives replacement.
- Benchmark results contain immutable installed-skill and companion identity captured at run time rather than inferred later.

## Linked Decisions

- `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`
- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
- `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`
- `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
- `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`
- `docs/decisions/ADR-0007-uppercase-generated-markdown-filenames.md`
- `docs/decisions/ADR-0008-code-conventions-documentation.md`
- `docs/decisions/ADR-0009-manual-doc-edits-win-during-sync.md`
- `docs/decisions/ADR-0010-reader-first-information-architecture.md`
- `docs/decisions/ADR-0011-skill-distribution-version-integrity.md`
