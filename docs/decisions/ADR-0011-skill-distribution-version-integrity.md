---
artifact_type: adr
id: ADR-0011
status: accepted
owner: shared
human_fields: [decision_approval, final_status]
ai_fields: [context, alternatives_considered, consequences, linked_work]
shared_fields: [decision, trace]
trace:
  requirements: [REQ-007, REQ-014, REQ-015]
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  tickets_or_bugs: [BUG-002]
  detail_design: docs/work/designs/DESIGN-032-skill-install-version-integrity.md
  master_docs:
    - docs/requirements/SPEC.md
    - docs/architecture/ARCHITECTURE.md
    - docs/standards/CODEWIKI.md
  related_decisions:
    - docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md
  research: docs/work/research/GROK-WIKI-MEZON-AUDIT.md
  release_notes: docs/releases/CHANGELOG.md
---

# ADR-0011: Skill Distribution Version Integrity

## Status

Accepted on 2026-07-14 after the user authorized implementation with “sửa luôn đi”.

## Context

BUG-002 reproduced a material difference between repository source `skill/codewiki/**` and the active global installation under `~/.codex/skills/codewiki/**`. The installed skill uses legacy control-plane paths and filename rules and lacks newer references. The installer copies skill and companion payloads but records no source revision, content digest, or compatibility versions, so generation and benchmarks cannot identify which product contract actually ran.

ADR-0003 correctly made the skill the product and Rust the companion. It did not define how those two artifacts are versioned, installed, inspected, or proven compatible.

## Decision

1. Treat the skill references, helper scripts, companion interface, and their contract/schema versions as one versioned distribution unit.
2. Keep a committed source package manifest under `skill/codewiki/` and write an install provenance manifest under the installed skill root.
3. The install manifest records package version, source revision and dirty state when available, managed-file content digest, skill/reference contract version, companion interface/build version, install time/scope, and legacy migration compatibility.
4. Managed source, helper, binary, and companion payloads are replaced atomically. Project-local human/control state is outside the managed digest and is preserved only through explicitly declared paths.
5. Status/doctor and every generation/benchmark run record the resolved skill root and install manifest. Missing, modified, or incompatible artifacts are visible states, not silent warnings hidden from result metadata.
6. Companion/skill incompatibility prevents reader-doc success. Source-checkout drift is reported when comparable source is available, but CodeWiki never auto-updates without user authorization.
7. Benchmarks pin installed skill digest and contract versions alongside repository commit, model/provider, and evaluation contract.

This refines ADR-0003 without changing the skill-first product choice.

## Alternatives Considered

- Rely on users to reinstall manually: rejected because neither users nor benchmarks can observe when reinstall is required.
- Use only a semantic version string: rejected because local edits and mismatched bundled content can share the same version label.
- Hash the entire installed directory: rejected because build payloads and user-owned project state need separate ownership and reproducibility rules.
- Auto-update stale skills: rejected because it mutates user tooling and can invalidate active work without approval.
- Treat global/local installations as interchangeable: rejected because resolved skill root is part of the executed product identity.

## Consequences

- Positive: model/output comparisons become reproducible and skill drift is diagnosable before generation.
- Positive: skill instructions and companion behavior can evolve without silent incompatibility.
- Positive: installer ownership becomes explicit and safer for preserved project state.
- Negative: install/release workflows require manifest generation and compatibility tests.
- Negative: existing installations without manifests report legacy/unverified until reinstalled or migrated.
- Neutral: the chosen model/provider remains independent; this versions CodeWiki's orchestration contract, not model credentials.

## Linked Work

- Bug: `docs/work/bugs/BUG-002-installed-skill-version-drift.md`
- Design: `docs/work/designs/DESIGN-032-skill-install-version-integrity.md`
- Related ADR: `docs/decisions/ADR-0003-skill-first-product-and-rust-companion-tool.md`
