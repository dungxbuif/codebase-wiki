# CodeWiki Workspace Placement

Load this reference whenever the user initializes or syncs CodeWiki and the output location is not explicit.

## Output Modes

CodeWiki supports two output modes:

1. Repo-local wiki
   - Source repo and wiki workspace are the same directory.
   - Write control files to `.codewiki/**`.
   - Write generated docs to `docs/**`.
   - Use when the user wants docs committed with the source repository.

2. External/personal wiki workspace
   - Source repo is evidence input.
   - Wiki workspace is a separate directory chosen by the user.
   - Write `.codewiki/**` and `docs/**` inside the external workspace, not inside the source repo.
   - Use when the user wants personal docs, cross-repo knowledge, private notes, or docs outside source control.

## Confirmation Rule

Before writing files, determine whether the user clearly specified the wiki workspace.

- If explicit: proceed.
- If ambiguous: ask where to store the wiki before writing.
- If the user says “personal”, “outside source”, “not in repo”, or similar: ask for or infer a separate workspace path and confirm it.
- If the user says “repo-native”, “in this repo”, “commit with source”, or similar: use repo-local mode.

Do not silently write generated docs into the source repository when the user asked for personal/external docs.

## Workspace Files

Regardless of mode, the wiki workspace contains:

```text
.codewiki/
  config.yml
  plan.yml
  AGENTS.md
  sources.yml
docs/
  index.md
  ...
```

`sources.yml` records evidence sources. In repo-local mode, the primary Git source is the same directory as the workspace. In external mode, the primary Git source points to the source repository path or remote.
