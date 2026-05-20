# ADR 0001: Substrate Path B-Pragmatic Git2 Decision

Status: Accepted for current `main` as `Path B not proven; semantic DevTape
fallback only`.
Date: 2026-05-19.
Commit: `dfa002fc1e506ce72406e549abac3420a8ec5805`.

## Context

The Human Architect's final task sheet selects Path B-pragmatic only if Reality
Proof demonstrates that current `main` already has a git2
ChainTape/CAS/Sequencer/HEAD_t substrate. The same sheet forbids two canonical
substrates, new default `src/cas.rs`, `src/hash.rs`, `src/versioned_state.rs`,
RMCP, Wasmtime, gix migration, Next.js, Tauri, and new dependencies.

K0 inspection found no current `src` or `tests` anchors for git2,
`Repository::`, ChainTape, CAS refs, Sequencer, `HEAD_t`, TypedTx,
PromptCapsule, AttemptTelemetry, EvidenceCapsule, SpecCapsule, TISR, L4.E, or
rejection evidence.

Current `main` does contain a small semantic DevTape development-governance MVP:

```text
AppendInput -> append_event -> DevTapeRecord JSONL
DevTapeRecord[] -> derive_board
DevTapeRecord[] -> audit_board_drift / merge_check
```

## Decision

Path B-pragmatic is not accepted as proven current-main reality.

For current `main`, the accepted operational path is:

```text
semantic DevTape MVP for development governance only
```

This fallback is a local append-only development evidence rail. It is not V5
product runtime truth and must not be presented as production ChainTape/CAS.

Git2-backed ChainTape/CAS remains a future adapter/research or migration path
until a later task proves or implements it through the accepted risk-class gates.
gix/gitoxide remains future research only.

## Consequences

- K1 cannot assume a git2-backed substrate exists.
- Board projection is valid only when derived from an existing DevTape store and
  audited against it.
- The checked-in board is bootstrap projection state until DevTape-backed source
  events exist.
- Product work must still preserve the final task sheet's core data shapes:
  Candidate, EvidenceTuple, PredicateDecision, SpecCapsule, ArtifactBundle,
  PreviewRunCapsule, TestRunCapsule, and derived BuildSessionView.
- No new canonical CAS/hash/WAL/HEAD_t path may be introduced without the
  required accepted task path and Class 4 ratification where applicable.

## Non-Goals

- Do not edit `constitution.md` or `genesis_payload.toml`.
- Do not migrate to gix/gitoxide in this atom.
- Do not introduce RMCP, Wasmtime, Next.js, Tauri, or a new dependency.
- Do not implement product TISR, ArtifactBundle, preview, or `/build` in this
  ADR.

## Follow-Up Gates

The next tasks should make these gates executable in current `main`:

```text
C2: no-new-substrate regression
C3: no naked LLM call
C4: ArtifactBundle CAS wire
C5: preview truth path
C6: BuildSession derived view
```

Each gate must be a small atom with exact allowed files, forbidden files,
acceptance commands, and negative tests.
