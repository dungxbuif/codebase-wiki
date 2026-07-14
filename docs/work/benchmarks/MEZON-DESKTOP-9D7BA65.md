---
artifact_type: onboarding_benchmark
id: MEZON-DESKTOP-9D7BA65
status: in_review
owner: shared
human_fields: [uat_sign_off, reviewer_notes]
ai_fields: [source_manifest, generated_pages, evaluator_results, source_audit]
shared_fields: [status, critical_questions, result]
trace:
  phase: docs/work/phases/PHASE-002-reader-first-docs-quality.md
  ticket: docs/work/tickets/TICKET-031-onboarding-quality-evals.md
  verification: docs/work/verifications/TEST-PHASE-002-IMPLEMENTATION.md
  validation_matrix: docs/work/VALIDATION_MATRIX.md
---

# Mezon Desktop Onboarding Benchmark At 9d7ba65

## Reproducibility Manifest

- Source repository: `mezonai/mezon-desktop`
- Source commit: `9d7ba654830c0e6278d0eb413a0eb9a992f01a55`
- Source dirty before initialization: `false`
- Source checkout mode: local shared clone from the user-provided repository
- Visible human docs: `README.md`, root `Cargo.toml`, and detected repository READMEs
- CodeWiki package: `0.2.0`
- Companion interface: `2`
- Skill/reference contract: `2`
- WikiPlan schema: `2`
- Installed managed digest: `fnv1a64:86d285e28caa1b4a`
- Generation model: OpenAI Codex GPT-5
- Evaluation model: OpenAI Codex GPT-5, explicit docs-only pass plus source-and-evidence audit
- Independence caveat: generation and evaluation used the same model family; this is weaker than a genuinely separate evaluator and must remain visible in comparisons.

## Generated Reader Surface

The synthesis produced six reader pages with one canonical owner per topic:

1. `docs/QUICKSTART.md` — first-hour setup, mental model, reading paths, and change map.
2. `docs/architecture/OVERVIEW.md` — five primary crate boundaries, runtime flow, and state ownership.
3. `docs/workflows/AUTHENTICATION.md` — auth state machine, deep-link sequence, session persistence, and logout invariant.
4. `docs/components/REALTIME.md` — transport/store layers, request/push sequence, reconnect, and session refresh.
5. `docs/components/NATIVE-INTEGRATION.md` — OS capability map, callback boundary, platform differences, and graceful failure policy.
6. `docs/conventions/OVERVIEW.md` — crate ownership decision tree, lifecycle patterns, verification ladder, and review checklist.

The pages contain four material diagrams: system context, component boundary, auth state, and request/push or callback sequences. No reader page begins with a file/symbol inventory.

## Docs-Only Critical Questions

| Question | Docs-only answer | Owning page | Result |
| --- | --- | --- | --- |
| What is the process composition root? | `mezon-app`; it wires runtimes, clients, stores, native callbacks, and the main window. | Quickstart, Architecture | pass |
| Where should observable domain state live? | In GPUI entities under `mezon-store`; transport-internal state remains in GPUI-free `mezon-client`. | Architecture | pass |
| When does authenticated UI become visible? | After a session enters `Connecting` and the realtime connection is confirmed, promoting it to `Authenticated`. | Authentication | pass |
| What must logout clear? | Credentials, cached account, transport/session state, and every user-scoped store, including voice teardown. | Authentication | pass |
| Who owns reconnect policy versus wire mechanics? | `ConnectionStore` owns policy/backoff/probes; `MezonTransport` owns adapter, correlation, timeout, and decode mechanics. | Realtime | pass |
| How does a server push reach the UI? | Transport decodes a typed event, `RealtimeDispatch` updates the owning store, and views observe that store. | Realtime | pass |
| Where does deep-link platform code end? | `mezon-native` translates/forwards the OS event; `mezon-app` applies process meaning and updates auth/store behavior. | Native Integration | pass |
| What are the handoff gates? | `just lint` and `just test`; use targeted tests and `just check` earlier, and `just safety` for dependency changes. | Quickstart, Conventions | pass |
| How should a developer choose the owning crate? | Follow the UI → store → client → native → app decision tree and keep `mezon-app` as composition only. | Conventions | pass |

## Source Audit

Important claims were checked against `crates/mezon-app/src/main.rs`, `crates/mezon-ui/src/app/root.rs`, `crates/mezon-store/src/login.rs`, `crates/mezon-store/src/connection.rs`, `crates/mezon-store/src/realtime.rs`, `crates/mezon-client/src/transport.rs`, `crates/mezon-native/src/*.rs`, `README.md`, `Cargo.toml`, and `.github/workflows/ci.yml`.

- Critical hallucinations: 0
- Broken local links: 0
- Orphan reader pages: 0
- Duplicate canonical topic owners: 0
- WikiPlan hierarchy/prerequisite cycles: 0
- Bounded revision attempts: 0

## Validation Result

The installed companion reported:

```text
CodeWiki reader docs ready
reader_pages_checked: 6
generation_status: reader_docs_ready
```

Deterministic validation passed plan completeness, source provenance, installed-skill identity, quality declarations, page contracts, local links, cross-page navigation, and diagram fence checks.

## Human UAT

- Required: yes
- Status: pending maintainer review
- Review task: read the six-page surface without source and confirm that it is sufficient to choose a safe first change location.

