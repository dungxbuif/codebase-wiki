# Convention Discovery

Load this reference during every init and sync before writing `docs/conventions/OVERVIEW.md`.

## Purpose

Discover how this repository expects code to be written and changed. Document repository evidence, not generic language or framework best practices.

## Evidence Order

1. Explicit formatter, linter, compiler, build, test, security, and repository instruction files.
2. Existing contribution, architecture, style, and framework documentation.
3. Repeated code patterns across independent files or areas.
4. Tests that enforce structure, behavior, naming, errors, or lifecycle rules.
5. Git history or command output when it explains an intentional convention.

Framework defaults are candidates only. Treat them as project conventions only when code or configuration shows that this repository follows them.

## Discovery Technique

1. Detect the repository's languages, frameworks, libraries, tools, and major areas.
2. Find explicit convention sources such as agent instructions, formatter/linter/compiler config, build manifests, contribution docs, and representative tests.
3. Sample representative source from multiple areas. Look for repeated approaches to:
   - file/module/package structure and dependency direction;
   - naming and public API shape;
   - error handling, validation, logging, and observability;
   - async/concurrency, lifecycle, state, and data ownership;
   - framework/library composition and extension points;
   - configuration, environment, secrets, and feature flags;
   - testing, fixtures, mocks, and safe-change checks;
   - security boundaries and documentation practices.
4. For every candidate convention, search for supporting examples and counterexamples.
5. Determine scope: repository-wide, language, framework/library, subsystem, test-only, or legacy area.
6. Record exceptions and migration boundaries. Do not hide conflicting patterns.

## Classification

- `explicit`: enforced or stated by authoritative repository config/docs.
- `inferred`: supported by at least two independent code examples with no material counterexample in the inspected scope.
- `hypothesis`: plausible but supported by only one example or incomplete coverage.
- `exception`: deliberate or legacy deviation from an otherwise supported convention.

Repeated code is not automatically a good convention. Describe what the repository consistently does and note suspected anti-patterns separately.

## Page Contract

Write `docs/conventions/OVERVIEW.md` with:

- detected scope and evidence coverage;
- explicit conventions;
- inferred conventions;
- language-specific conventions actually used by the repository;
- framework/library conventions actually used by the repository;
- subsystem-specific rules;
- exceptions, conflicting patterns, and legacy boundaries;
- change checklist for future humans and agents;
- open questions where evidence is insufficient.

For each convention include scope, classification, evidence paths/symbols or commands, confidence, exceptions, and change impact.

## Quality Gate

Do not declare convention discovery complete when:

- the page only repeats ecosystem best practices;
- an inferred convention has fewer than two independent examples;
- counterexamples were not searched for;
- explicit configs or repository instructions were ignored;
- conventions lack scope or evidence;
- framework defaults are presented as repository facts without adoption evidence;
- conflicting or legacy patterns are silently omitted.

