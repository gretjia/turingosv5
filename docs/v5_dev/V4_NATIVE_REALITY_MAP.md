# V4 Native Reality Map

Status: Candidate map produced from read-only inspection of
`/home/zephryj/projects/turingosv4`.

Anchor rule: `usable_now` requires at least one V4 code anchor and one V4 test
anchor. Items without anchors are assumptions only.

This map is development evidence. It must not be read by V5 product/runtime code
as truth.

## Classification Table

| Capability | Classification | Evidence status |
| --- | --- | --- |
| ChainTape / transition_ledger concept | usable_now | code anchors and test anchors listed |
| CAS concept | usable_now | code anchors and test anchors listed |
| L4.E / rejection_evidence concept | usable_now | code anchors and test anchors listed |
| PromptCapsule concept | usable_now | code anchors and test anchors listed |
| AttemptTelemetry concept | usable_now | code anchors and test anchors listed |
| EvidenceCapsule concept | usable_now | code anchors and test anchors listed |
| Veto concept | usable_now | code anchors and test anchors listed |
| head_t_witness concept | usable_now | code anchors and test anchors listed |
| Git-backed ChainTape storage | usable_with_adapter | V5-native adapter required |
| V4 prompt and telemetry schemas | usable_with_adapter | V5 schema and redaction adapter required |
| V4 CLI verifier patterns | usable_with_adapter | V5-native command required |
| V5 DevEventEnvelope implementation | not_usable_yet | V4D-R0 assumption at inspection time |
| V5 AgentIdentity binding implementation | not_usable_yet | V4D-R0 assumption at inspection time |
| V5 native ChainTape/CAS/rejection lane | not_usable_yet | V5-native implementation required |
| V4 runtime evidence as V5 runtime truth | do_not_use | runtime boundary violation |
| V4 Class 4 surfaces without ratification | do_not_use | exact human ratification required |
| MiniF2F V4 corpus as V5 product asset | do_not_use | V4 development corpus only |

Note: `not_usable_yet` rows describe the V4D-R0 inspection moment. Later
accepted V5 tasks may supersede them with V5-native contracts or tests.

## usable_now

### ChainTape / transition_ledger concept

Use: development architecture concept for an append-only accepted-transition
spine and replay/verifier planning.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs`
- `/home/zephryj/projects/turingosv4/src/bin/verify_chaintape.rs`
- `/home/zephryj/projects/turingosv4/src/bin/turingos/cmd_verify_chaintape.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/tb_14_chaintape_smoke.rs`
- `/home/zephryj/projects/turingosv4/tests/tb_6_verify_chaintape.rs`
- `/home/zephryj/projects/turingosv4/tests/cli_verify_chaintape_smoke.rs`

Limits:

- Usable as a model and anchor set only.
- V4 ChainTape data, refs, genesis, and local paths are not V5 runtime truth.

### CAS concept

Use: development architecture concept for content-addressed evidence objects.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs`
- `/home/zephryj/projects/turingosv4/src/bottom_white/cas/schema.rs`
- `/home/zephryj/projects/turingosv4/src/bottom_white/cas/git_chain.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/co1_7_extra_cas_payload_round_trip.rs`
- `/home/zephryj/projects/turingosv4/tests/tb_18r_lean_result_cas_resolves.rs`
- `/home/zephryj/projects/turingosv4/tests/tb_18r_cas_reload_split_brain.rs`

Limits:

- Usable as a model for V5-native CAS design.
- V4 CAS objects and roots are not V5 runtime truth.

### L4.E / rejection_evidence concept

Use: development architecture concept for rejected-submission evidence that is
separate from accepted transition state.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/bottom_white/ledger/rejection_evidence.rs`
- `/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs`
- `/home/zephryj/projects/turingosv4/src/state/sequencer.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/tb_6_l4e_jsonl_persistence.rs`
- `/home/zephryj/projects/turingosv4/tests/tb_18r_lean_reject_in_l4e.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_l4e_body_integrity.rs`

Limits:

- Usable for V5 development policy: rejected changes require rejection
  evidence.
- V4 rejection records are not V5 runtime truth.

### PromptCapsule concept

Use: development architecture concept for redacted prompt provenance and CAS
anchoring.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/runtime/prompt_capsule.rs`
- `/home/zephryj/projects/turingosv4/src/bin/turingos/cmd_llm.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/constitution_prompt_capsule.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_real5_prompt_capsule_v2.rs`
- `/home/zephryj/projects/turingosv4/tests/cli_phase63_cas_wire.rs`

Limits:

- Usable for development evidence shape and anti-naked-LLM policy.
- V5 must define its own accepted prompt provenance schema before runtime use.

### AttemptTelemetry concept

Use: development architecture concept for per-attempt telemetry, redaction, and
attempt-to-chain accounting.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/runtime/attempt_telemetry.rs`
- `/home/zephryj/projects/turingosv4/src/runtime/chain_derived_run_facts.rs`
- `/home/zephryj/projects/turingosv4/src/bin/audit_dashboard.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/tb_18r_attempt_telemetry_serialize.rs`
- `/home/zephryj/projects/turingosv4/tests/tb_18r_no_raw_response_in_attempt_payload.rs`
- `/home/zephryj/projects/turingosv4/tests/tb_18r_attempt_routes_to_l4_or_l4e.rs`

Limits:

- Usable as a model for development telemetry.
- Audit dashboard materializations are not truth.

### EvidenceCapsule concept

Use: development architecture concept for compact evidence capsules and
outcome propagation.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/runtime/evidence_capsule.rs`
- `/home/zephryj/projects/turingosv4/src/runtime/persistence_evidence.rs`
- `/home/zephryj/projects/turingosv4/src/runtime/markov_capsule.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/tb_18_evidence_capsule_outcome_propagation.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_shielding_gate.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_fc3_evidence_binding.rs`

Limits:

- Usable as a development evidence pattern.
- V5 must not import V4 capsule objects as accepted runtime evidence.

### Veto concept

Use: development architecture concept for veto verdicts and fail-closed review
gates.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/bus.rs`
- `/home/zephryj/projects/turingosv4/src/sdk/tool.rs`
- `/home/zephryj/projects/turingosv4/src/runtime/real5_roles.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/constitution_predicate_gate.rs`
- `/home/zephryj/projects/turingosv4/tests/fc_alignment_conformance.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_real14_e2_candidate_verifier.rs`

Limits:

- Usable for V5 development gate design.
- Veto authority in V5 must come from V5 harness assignment and accepted
  policies, not V4 role state.

### head_t_witness concept

Use: development architecture concept for a compact derived head witness over
state, L4, L4.E, CAS, economic state, and run id.

Code anchors:

- `/home/zephryj/projects/turingosv4/src/state/head_t_witness.rs`
- `/home/zephryj/projects/turingosv4/src/state/mod.rs`
- `/home/zephryj/projects/turingosv4/src/runtime/mod.rs`

Test anchors:

- `/home/zephryj/projects/turingosv4/tests/constitution_head_t_witness.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_head_t_c2_multi_ref.rs`
- `/home/zephryj/projects/turingosv4/tests/co1_7_extra_sequencer_head_t_advancement.rs`

Limits:

- Usable as a design reference for V5-native witnesses.
- V4 HEAD_t values are not V5 runtime truth.

## usable_with_adapter

### Git-backed ChainTape storage

Use with adapter only. V4 has a libgit2-backed implementation and named-ref
model, but V5 should expose a V5-native interface before any runtime use.

Anchors:

- `/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_head_t_c2_multi_ref.rs`

Adapter requirements:

- No absolute V4 path.
- No V4 genesis trust.
- V5 fixtures and verifier tests must be native.

### V4 prompt and telemetry schemas

Use with adapter only. PromptCapsule, AttemptTelemetry, and EvidenceCapsule show
useful field families, but V5 needs its own accepted schema ids and redaction
rules.

Anchors:

- `/home/zephryj/projects/turingosv4/src/runtime/prompt_capsule.rs`
- `/home/zephryj/projects/turingosv4/src/runtime/attempt_telemetry.rs`
- `/home/zephryj/projects/turingosv4/src/runtime/evidence_capsule.rs`
- `/home/zephryj/projects/turingosv4/tests/constitution_prompt_capsule.rs`
- `/home/zephryj/projects/turingosv4/tests/tb_18r_attempt_telemetry_serialize.rs`

Adapter requirements:

- V5 schema ids.
- Explicit redaction tests.
- No raw model response or hidden chain-of-thought in public evidence.

### V4 CLI verifier patterns

Use with adapter only. V4 verifier binaries demonstrate useful command shapes,
but V5 should not shell out to V4 tooling as runtime verification.

Anchors:

- `/home/zephryj/projects/turingosv4/src/bin/verify_chaintape.rs`
- `/home/zephryj/projects/turingosv4/src/bin/turingos/cmd_verify_chaintape.rs`
- `/home/zephryj/projects/turingosv4/tests/cli_verify_chaintape_smoke.rs`

Adapter requirements:

- V5-native command.
- V5-native fixtures.
- CI boundary test proving no V4 local path dependency.

## not_usable_yet

### V5 DevEventEnvelope implementation

Assumption: this plan defines the desired envelope shape, but no V5-native
schema or validator exists in the allowed task scope.

Required before use:

- accepted schema
- sample fixtures
- validation test
- Meta acceptance path

### V5 AgentIdentity binding implementation

Assumption: the harness has role documents and task board policy, but a native
AgentIdentity event stream has not been implemented in this task.

Required before use:

- accepted identity schema
- role_assignment evidence validator
- tests for explicit-role-only activation

### V5 native ChainTape/CAS/rejection lane

Assumption: V4 proves these concepts are implementable, but V5 native substrate
files were outside this task's allowed edit scope.

Required before use:

- V5-native code
- V5-native tests
- accepted risk-class path

## do_not_use

### V4 runtime evidence as V5 runtime truth

Do not use:

- V4 `handover/evidence/**`
- V4 `genesis_payload.toml`
- V4 local runtime paths
- V4 ChainTape/CAS objects as accepted V5 evidence
- V4 dashboard/session/cache state as truth

Reason:

- The V5 runtime boundary forbids V5 product/runtime dependency on V4 evidence,
  genesis, or local path truth.

### V4 Class 4 surfaces without ratification

Do not copy or activate without exact human ratification:

- constitution
- genesis/trust root
- sequencer admission
- typed transaction wire schema
- canonical signing payload
- kernel authority

Reason:

- Class 4 is never self-selected and cannot be ratified by Meta alone.

### MiniF2F V4 corpus as V5 product asset

Do not use V4 MiniF2F as:

- default V5 package
- V5 test problem set
- core CI path
- product/runtime asset

Reason:

- MiniF2F is a V4 development/evaluation corpus, not a V5 product asset.

## Anchor Check Summary

Read-only command pattern used:

```bash
rg -n "ChainTape|CAS|L4.E|PromptCapsule|AttemptTelemetry|EvidenceCapsule|Veto|transition_ledger|rejection_evidence|head_t_witness" /home/zephryj/projects/turingosv4
```

Observed exact anchor terms:

- `ChainTape`
- `CAS`
- `L4.E`
- `PromptCapsule`
- `AttemptTelemetry`
- `EvidenceCapsule`
- `Veto`
- `transition_ledger`
- `rejection_evidence`
- `head_t_witness`

## Assumptions

- This map reflects the checked-out V4 repository at inspection time, not an
  external release guarantee.
- The presence of V4 code and tests is evidence for development planning only.
- Any future V5 implementation must re-establish acceptance with V5-native code,
  tests, and risk-class gates.
