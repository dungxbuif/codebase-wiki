---
artifact_type: detail_design
id: DESIGN-032
status: in_review
owner: ai
approval: approved
human_fields: [approval, constraints, scope_decisions]
ai_fields: [problem, context_loaded, brownfield_scope, proposed_approach, design_tradeoffs, architecture_overview, execution_flow, api_data_model, security, test_plan, reconciliation_plan]
shared_fields: [status, trace, small_task_exemption]
trace:
  backlog_item: BL-018
  requirements: [REQ-007, REQ-014, REQ-015]
  phase: PHASE-002
  ticket_or_bug: docs/work/bugs/BUG-002-installed-skill-version-drift.md
  test_verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
  docs_review: docs/work/reviews/DOCS-REVIEW-PHASE-002-IMPLEMENTATION.md
  adrs: [docs/decisions/ADR-0011-skill-distribution-version-integrity.md]
  master_docs_touched: [README.md, docs/architecture/ARCHITECTURE.md, docs/standards/CODEWIKI.md]
---

# Detail Design: Skill Install Version Integrity

## 1. Problem And Boundary

The active installed skill can differ materially from repository source with no observable version or compatibility signal. This design covers package/install provenance, managed-file integrity, companion compatibility, status/doctor reporting, and benchmark recording. It does not authorize automatic updates or redesign Codex skill discovery precedence.

## 2. Distribution Model

```text
repository source
skill manifest + managed skill files + companion source
                    |
                    v
atomic installer/build
                    |
                    v
installed skill root
managed payload + install provenance + preserved declared project state
                    |
                    v
status/doctor/run provenance
resolved root + digests + contract compatibility
```

## 3. Manifests

Add a committed source manifest, for example `skill/codewiki/package.yml`:

- `schema_version`
- `package_version`
- `skill_contract_version`
- `reference_contract_version`
- `companion_interface_version`
- `wikiplan_schema_range`
- managed path/include rules
- preserved user/project path rules

The installer writes `INSTALLATION.yml` in the installed root with:

- all source contract fields;
- source Git revision and dirty state when available;
- canonical digest of managed source/reference/script content;
- companion source digest, binary digest, build version, and availability;
- install scope/root/time and installer version;
- migration/legacy state detected during installation.

The digest excludes `project/**`, caches, runtime state, and other declared user-owned paths. Binary/source/helper digests remain separate so diagnostics identify the drifting layer.

## 4. Installation And Ownership

1. Resolve source and destination.
2. Read and validate the source package manifest.
3. Stage managed content and companion build in a temporary sibling directory.
4. Compute digests and write install provenance in staging.
5. Preserve only declared project/user-owned paths.
6. Validate staged package and companion compatibility.
7. Atomically replace the managed installation where filesystem semantics permit; otherwise use a rollback-safe rename sequence.
8. Run installed status/doctor and report the resolved manifest identity.

An installation without a manifest is `legacy_unverified`. A modified managed file is `content_drift`. An incompatible skill/companion pair is `incompatible`. None of these states may report `reader_docs_ready`.

## 5. Runtime And Benchmark Contract

`status`/`doctor` and generation results expose:

- resolved skill root and install scope;
- package and contract versions;
- source revision/digest;
- managed-content integrity;
- companion source/binary version and compatibility;
- legacy control-plane detection;
- recommended explicit reinstall/migration command when needed.

WikiPlan/run metadata and benchmark results copy this identity. They do not rely on a later filesystem lookup that may have changed after generation.

## 6. Compatibility And Migration

- The skill declares the companion interface and WikiPlan schema range it accepts.
- The companion reports its interface/build version without needing repository source.
- Old `.codewiki/**` control state is detected and reported with the existing marker-aware migration path; it is never silently merged with `.agents/**` state.
- Global and local installations are separate product identities. The actually resolved root is recorded.
- Reinstall is explicit and user-authorized; doctor never mutates installed content.

## 7. Tests

- Manifest schema/required-field/unit tests.
- Local and global temporary install smokes from a pinned checkout.
- Managed-file digest equality after install.
- Deliberate reference mutation produces `content_drift`.
- Deliberate companion interface mismatch produces `incompatible` and blocks reader-doc success.
- Missing manifest produces `legacy_unverified` with an explicit remediation.
- Project-state preservation test proves only declared paths survive replacement.
- Rollback test proves a failed staged build/validation leaves the old installation intact.
- Benchmark serialization includes the installed skill identity.

## 8. Security And Safety

- Never hash or emit credentials, runtime databases, prompts, source repository contents, or user project state as part of the skill package digest.
- Do not execute an installed binary before its staged package manifest and compatibility metadata validate.
- Do not auto-update or delete undeclared user content; stop and report an ownership conflict.

## 9. Reconciliation

After implementation, update installer/README, architecture, CodeWiki standards, skill helper/status guidance, validation matrix, context, traceability, changelog, ADR-0003 relationship notes, and BUG-002 verification.
