# TuringOS Reality Map K0

Status: K0/C0 candidate reality proof.
Date: 2026-05-19.
Commit: `dfa002fc1e506ce72406e549abac3420a8ec5805`.

This document records what current `main` can prove by machine inspection. It is
development evidence only. It is not V5 runtime truth, and it must not be read
by product/runtime code as canonical state.

## Core Illusion

K0 asks whether current `main` already contains the physical substrate claimed by
the architecture story, or whether those claims are still assumptions.

## Data Flow Layout

```text
git commit + grep evidence + code anchors + test anchors
  -> RealityMap
  -> PathDecision
  -> next task wave
```

Current proven executable loop:

```text
AppendInput
  -> append_event()
  -> DevTapeRecord JSONL
  -> derive_board()
  -> audit_board_drift() / merge_check()
```

## Commands Run

```bash
git status --short
git rev-parse HEAD
cargo metadata --no-deps --format-version 1 > /tmp/turingos-cargo-metadata.json
rg -n "git2|Repository::|ChainTape|HEAD_t|C1|C2|refs/chaintape/cas|TypedTx|Sequencer|PromptCapsule|AttemptTelemetry|EvidenceCapsule|SpecCapsule|TISR|REAL-17|rejection_evidence|transition_ledger|head_t_witness" src tests handover
rg -n "git2|Repository::|ChainTape|HEAD_t|C1|C2|refs/chaintape/cas|TypedTx|Sequencer|PromptCapsule|AttemptTelemetry|EvidenceCapsule|SpecCapsule|TISR|REAL-17|rejection_evidence|transition_ledger|head_t_witness" src tests
rg -n "spec_capsule|SpecCapsule|generate|preview|PromptCapsule|AttemptTelemetry|EvidenceCapsule|Artifact|TISR|turingos_web|ChainTape|CAS|L4.E|rejection_evidence" src tests handover
rg -n "spec_capsule|SpecCapsule|generate|preview|PromptCapsule|AttemptTelemetry|EvidenceCapsule|Artifact|TISR|turingos_web|ChainTape|CAS|L4.E|rejection_evidence" src tests
cargo test --test constitution_no_parallel_ledger
cargo test --test constitution_no_new_parallel_substrate
```

Observed command notes:

- `git status --short` was clean before K0 documentation edits.
- `handover/` does not exist in this checkout, so the requested grep path that
  includes `handover` exits with an OS error.
- The two constitution test targets requested by the final task sheet do not
  exist in current `Cargo.toml`.
- `cargo metadata` completed and was written to
  `/tmp/turingos-cargo-metadata.json`.

## Kernel-Driven Boot Correction

After HOLD, MetaAI initialized the local DevTape before further K0 dispatch.
The store is ignored local development evidence at:

```text
.turingos_system/devtape/turingosv5/events.jsonl
```

Appended records:

| Event | Record hash |
| --- | --- |
| HumanIntentReceived | `sha256:661e1908c7a4279549add8674ce08637d4fbd9d9194e9bd856e21c7ee5d0fac1` |
| DevTaskCreated | `sha256:9496c571fe666b4bbd42505d90de3fa89a354cabffbc89ebd4e2bada376921fa` |
| TaskBroadcasted | `sha256:c7e3889620a60a681060cf3da1c7ca52ed41c979775f90c401bbc485dde3fec3` |

The derived local board was written to:

```text
.turingos_system/devtape/turingosv5/derived_board.json
```

Audit command:

```bash
cargo run --quiet --bin turingos-dev -- audit --store .turingos_system/devtape/turingosv5/events.jsonl --board .turingos_system/devtape/turingosv5/derived_board.json
```

Result: `AUDIT_PASS`.

## Current Main Substrate Claims

| Claim | Classification | Evidence |
| --- | --- | --- |
| git2 ChainTape/CAS/Sequencer/HEAD_t is present in current `main` | assumption_not_proven | No `src` or `tests` match for the K0 substrate grep. `Cargo.toml` has no `git2` dependency. |
| V5 has a local DevTape development-evidence MVP | usable_now_for_development_governance | `src/devtool/mod.rs` defines `DevTapeRecord`, hash chaining, board derivation, audit drift, and merge gate checks. |
| `turingos-dev` CLI exists as a cargo target | usable_now_via_cargo | `src/bin/turingos-dev.rs` exposes `event append`, `board derive`, `audit`, and `merge check`. It is not installed on `PATH` in this session. |
| `.turingos_system/devtape/turingosv5/events.jsonl` exists | not_present | The store file does not exist at K0 intake. |
| The checked-in board is DevTape-derived truth | bootstrap_projection_only | Without the store file, the checked-in board cannot be audited as a DevTape projection. |
| Product TISR/spec/generate/preview flow exists | not_usable_yet | No current `src` or `tests` anchors for `TISR`, `SpecCapsule`, `ArtifactBundle`, preview server, or `/build`. |

## Code Anchors

Development DevTape MVP:

- `src/devtool/mod.rs`: `DevTapeRecord` stores `record_hash`,
  `previous_record_hash`, `envelope`, and `payload`.
- `src/devtool/mod.rs`: `append_event()` enforces tip hash continuity,
  accepted event types, and `classification.runtime_truth == false`.
- `src/devtool/mod.rs`: `derive_board()` projects `DevTaskCreated`,
  `TaskBroadcasted`, `TaskClaimed`, `WorkerReportSubmitted`, and
  `MergeDecisionRecorded`.
- `src/devtool/mod.rs`: `audit_board_drift()` rejects board drift from the
  derived projection.
- `src/devtool/mod.rs`: `merge_check()` requires claim, WorkerReport, audit,
  Veto, merge decision, CI, clean merge state, and branch protection evidence.
- `src/bin/turingos-dev.rs`: CLI wrapper for event append, board derive, audit,
  and merge check.

Dependency anchor:

- `Cargo.toml` uses `serde`, `serde_json`, and `sha2`.
- `Cargo.toml` does not include `git2`, `gix`, `reqwest`, provider SDKs, web
  frameworks, RMCP, Wasmtime, Next.js, or Tauri.

## Test Anchors

Development DevTape MVP:

- `tests/v5_devtape_kernel_mvp.rs` proves append hash chaining, broken previous
  hash rejection, `runtime_truth=true` rejection, and unknown event rejection.
- `tests/v5_devtape_kernel_mvp.rs` proves broadcast-before-board projection and
  manual board drift detection.
- `tests/v5_devtape_kernel_mvp.rs` proves merge checks require claim, report,
  audit, Veto, CI, branch protection, and clean merge state evidence.

Planning and boundary anchors:

- `tests/v5_plan_reality_map.rs` checks the earlier V4 Native Reality Map and
  DevKernel plan docs.
- `tests/v5_devtape_v1_execution_plan.rs` checks the DevTape v1 execution plan.
- `tests/v5_harness_alignment.rs` checks Meta/Worker harness role routing and
  board-first worker behavior.

## UX Endpoint Map

No product UX endpoints are present in current `src`.

| Endpoint | Status | Evidence |
| --- | --- | --- |
| `/build` | not_usable_yet | No web server module or route exists. |
| `/preview/<session_id>` | not_usable_yet | No preview server module or route exists. |
| artifact download | not_usable_yet | No ArtifactBundle download path exists. |

## LLM Call Map

No runtime LLM call sites are present in `src`.

| Call site | Classification | Evidence |
| --- | --- | --- |
| Spec interview LLM | not_usable_yet | No implementation anchor. |
| Code generation LLM | not_usable_yet | No implementation anchor. |
| Repair/regenerate LLM | not_usable_yet | No implementation anchor. |
| Naked LLM violation | none_observed_in_runtime | There are no runtime LLM call sites to classify yet. |

Policy references to prompt, model, and LLM appear in docs and tests only. They
do not prove an executable PromptCapsule or AttemptTelemetry path.

## Artifact Write Map

No product artifact write path is present.

| Write path | Classification | Evidence |
| --- | --- | --- |
| ArtifactBundle CAS write | not_usable_yet | No `ArtifactBundle` implementation anchor in `src` or `tests`. |
| Bare HTML generation return | none_observed_in_runtime | No product generator exists. |
| DevTape JSONL write | development_evidence_only | `append_event()` appends records to the configured store path. |
| Derived board write | development_projection_only | `turingos-dev board derive` writes a board projection to the requested path. |

## Preview Read Map

No product preview read path is present.

| Read path | Classification | Evidence |
| --- | --- | --- |
| Preview reads ArtifactBundle CID | not_usable_yet | No preview server exists. |
| Preview reads arbitrary temp path | none_observed_in_runtime | No preview server exists. |
| Preview console/network capture | not_usable_yet | No preview runner exists. |

## Legacy Boundary Map

| Legacy surface | Classification | Evidence |
| --- | --- | --- |
| V4 evidence/handover path | absent_in_checkout | `handover/` does not exist here. |
| V4 Native Reality Map | docs_only_reference | `docs/v5_dev/V4_NATIVE_REALITY_MAP.md` is development evidence, not runtime truth. |
| V4 lineage snapshot | docs_only_reference | `docs/lineage/V4_SNAPSHOT.md` documents lineage boundaries. |
| `src/ledger.rs`, `src/wal.rs`, `bus.graveyard` | absent_in_v5_main | No matching files are present. |
| V4 ChainTape/CAS objects | do_not_use_as_v5_truth | Existing docs explicitly prohibit importing V4 genesis, local paths, or evidence as V5 runtime truth. |

## Path Decision Summary

Path B-pragmatic is not proven for current `main` because the required git2
ChainTape/CAS/Sequencer/HEAD_t code and test anchors are absent.

Current executable fallback is the semantic DevTape MVP in `src/devtool/**`.
That fallback is development governance evidence only. It is not a product
runtime substrate, not a V5 production ChainTape, and not a license to create a
second canonical substrate.

## K0 Kill Criteria

| Criterion | Result |
| --- | --- |
| A1 cannot prove current `main` has ChainTape/CAS/HEAD_t | triggered_for_path_b |
| Naked LLM call exists but cannot be classified | not_triggered; no runtime LLM call sites found |
| Preview temp file truth exists | not_triggered; no preview server found |
| Web session canonical state exists | not_triggered; no web session implementation found |
| Competing canonical substrates exist | not_triggered in current code; only semantic DevTape MVP is implemented |

## Next Gate

Do not enter K1 as though TISR, PromptCapsule, ArtifactBundle, or preview rails
already exist. The next accepted task wave should either:

- formalize the current semantic DevTape fallback as the K0 development
  governance path, or
- implement the missing K1 product evidence rails in small atoms after
  TaskPackets name exact allowed files, forbidden files, and tests.
