---
artifact_type: user_stories
id: USER_STORIES
status: active
owner: shared
human_fields: [role, need, benefit, acceptance_criteria]
ai_fields: [story_rows, trace_links, status_updates]
shared_fields: [stories]
---

# User Stories

## Field Ownership

- Human owns user intent and acceptance criteria.
- AI maintains story rows, trace links, and status updates.

| ID | Role | Need | Benefit | Acceptance Criteria | Status |
| --- | --- | --- | --- | --- | --- |
| US-001 | Maintainer using Codex | Install CodeWiki from the repository as a skill | Reuse the same documentation workflow across repositories | One install command copies the skill, builds or falls back to the companion helper, and documents the installed shape | accepted |
| US-002 | Agent running the CodeWiki skill | Explore a target repository automatically | Produce a wiki plan and docs without manual approval gates during init | The skill directs the agent to detect languages, frameworks, entrypoints, docs, tests, and architecture evidence dynamically | accepted |
| US-003 | Developer reading generated docs | Trust claims in the wiki | Understand what is fact, hypothesis, stale, or unresolved | Generated content links claims to evidence items, source files, commands, or explicit hypotheses | accepted |
| US-004 | Developer maintaining docs over time | Sync generated pages safely | Preserve human edits while refreshing generated regions | Sync updates only CodeWiki generated regions and marks stale content when evidence changes | accepted |
| US-005 | Team with private workflow sources | Extend source-change context without hardcoding providers into CodeWiki | Add local source skills for JIRA, internal incident systems, or other change sources | Git remains default; user-provided source skills follow the source extension template and are recorded as evidence | accepted |
| US-006 | Developer or agent changing an unfamiliar repository | Learn the conventions the codebase actually follows | Make changes that fit project, language, framework, and subsystem expectations | `docs/conventions/OVERVIEW.md` distinguishes explicit rules, evidence-backed inferred patterns, hypotheses, and exceptions, with at least two independent examples for inferred conventions | accepted |
