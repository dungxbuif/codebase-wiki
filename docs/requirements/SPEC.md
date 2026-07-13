---
artifact_type: requirement_spec
id: REQ-MASTER
status: draft
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
- `docs/quickstart.md` is the required generated entrypoint after successful init.
- Canonical generated docs slots include map, architecture, domains, workflows, data, interfaces, operations, testing, decisions, glossary, open questions, evidence, and optional area pages.
- Project config lives in committed `.codewiki/config.yml`.
- Plan/state summaries that should travel with the repo live in committed `.codewiki/plan.yml` or equivalent.
- Target-repo CodeWiki agent guidance lives in committed `.codewiki/AGENTS.md`.
- Source declarations live in `.codewiki/sources.yml`.
- Git is the default source for code changes.
- Non-Git sources are supported through user-provided source extension skills, not bundled providers.
- Local runtime state lives outside the repo in SQLite and is keyed by repository identity.
- Rebuildable cache is separated from durable runtime state.
- Q&A should answer from generated docs first, then source evidence when docs are insufficient.
- Q&A should activate external runtime tools only after docs, plan, agent instructions, local SQLite evidence, and source/Git inspection are insufficient for the requested answer.
- Sync should detect stale docs and avoid silently overwriting human-owned content.

## Non-Functional Requirements

- Works across repositories without core adapters per language/framework.
- Rust companion crates should use typed schemas, explicit errors, deterministic filesystem behavior, and testable provider boundaries.
- Provider boundaries are replaceable.
- Durable state has migrations from the start.
- Outputs are evidence-first and auditable.
- The tool remains usable after session reset or model change.
- The generated wiki should be readable by humans and useful to agents.

## Constraints

- Keep tool count intentionally small.
- Do not make Octocode, codebase-memory-mcp, or CocoIndex bundled skill dependencies.
- Use Octocode as the default first-choice code-intelligence provider when a provider is needed.
- Use codebase-memory-mcp only when cross-session shared memory beyond CodeWiki's SQLite state is needed.
- Use CocoIndex only when repo size or repeated refresh/query workload justifies an indexing pipeline.
- Provide install/activation guidance for runtime tools in the skill and target-repo CodeWiki agent instructions.
- Keep committed config/docs separate from persistent local runtime state and rebuildable cache.
- Treat `docs/**` as the generated knowledge surface and `.codewiki/**` as the committed control plane.
- Ask for/confirm output workspace placement before writing when repo-local versus external/personal storage is ambiguous.
- Use OpenWiki and deepwiki-open as technical references only; do not inherit their runtime architecture wholesale.
- Install command should copy/install `skill/codewiki` into `$CODEX_HOME/skills/codewiki`.

## Acceptance Criteria

- A fixture repository can be initialized into a wiki with a structured WikiPlan and generated docs.
- Generated claims include evidence links or explicit hypothesis markers.
- Re-running with an existing config/state reuses prior facts and checkpoints.
- Changing model/provider does not require discarding durable state.
- Detection behavior is validated on at least several different repository shapes before release.

## Linked Decisions

- `docs/decisions/ADR-0001-codewiki-core-tooling-and-state.md`
- `docs/decisions/ADR-0002-rust-cli-and-reference-submodule-strategy.md`
- `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
- `docs/decisions/ADR-0004-runtime-optional-code-intelligence-tools.md`
- `docs/decisions/ADR-0005-codewiki-generated-docs-structure.md`
- `docs/decisions/ADR-0006-workspace-placement-and-source-extension-skills.md`
