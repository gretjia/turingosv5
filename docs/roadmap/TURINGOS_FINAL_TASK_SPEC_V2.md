# TuringOS Final Task Specification v2.0

Version: Karpathy-Style Architecture Review v2.0
Date: 2026-05-19
Status: Human Architect final execution brief.

This document is the local preserved task book for future MetaAI sessions. It
is not a vision document, a module catalog, or an enterprise architecture
blueprint. The style ruling is:

```text
Data Shapes > Logic
End-to-End Toy Model > future extensibility
monolithic and flat > premature distribution
physical facts > architecture narration
```

## 0. Core Illusion

TuringOS is not a multi-agent operating system, a generative app platform, or
an LLM workflow orchestrator.

Physically, TuringOS does one thing:

```text
write user intent, LLM attempts, candidate bytes, preview results, tests,
failures, and audits into a replayable append-only build chain.
```

The LLM proposes candidate bytes. TuringOS verifies whether those bytes may
advance the world.

Shortest data flow:

```text
UserIntent
  -> PromptCapsule
  -> LlmAttempt
  -> Candidate
  -> PredicateDecision
  -> L4 accepted or L4.E rejected
  -> ArtifactBundle / PreviewRun / TestRun
  -> Derived BuildSessionView
```

Shortest product flow:

```text
user intent
  -> spec
  -> artifact bundle
  -> CID-addressed preview
  -> spec-derived tests
  -> replayable delivery audit
```

Any term that cannot return to this loop should be deleted or postponed.

## 1. Architecture Ruling

Preserve the TuringOS constitutional physics:

```text
Q_t = <q_t, HEAD_t, tape_t>
```

- `tape_t` is physical memory.
- `HEAD_t` is the current world path pointer.
- `q_t` is recoverable internal state.
- `rtool(Q_t)` reads current world and produces model-visible context.
- `wtool(output | Q_t)` advances accepted world only when predicates pass.
- Failure must enter rejected evidence.

Preserve CAK:

```text
LLM output = Candidate
Candidate = state transition proposal
PredicateDecision = hard gate
Accepted = advances world head
Rejected = advances audit/rejection evidence, not world head
Every signal = reconstructible from canonical tape/substrate
```

Preserve Product-CAK:

```text
spec -> generate -> preview -> edit/regenerate -> spec-derived test -> audited delivery
```

UX may be smooth, but it must not create a new truth source.

## 2. Deprecated Or Downgraded

Do not start from module lists. Earlier P0-P14, A0-A15, and M0-M8 are useful
history, but the execution path is now data-shape and end-to-end-loop first.

Downgrade or reject:

- default creation of `src/cas.rs`, `src/hash.rs`, or `src/versioned_state.rs`;
- premature RMCP, Wasmtime, gix migration, or remote A2A;
- three naked API calls for Step 1 / Step 2 / Step 3;
- preview reading temporary files;
- web session memory as canonical state;
- LLM critic directly accepting artifacts;
- dashboard, wallet, market, cost, prompt registry as source of truth;
- natural-language rules as the architecture gate.

## 3. Current Path Ruling

Use:

```text
Path B-pragmatic, gated by Reality Proof
```

If Reality Map proves current `main` has git2 ChainTape/CAS/Sequencer/evidence
substrate, then:

```text
canonical substrate = existing git2 ChainTape/CAS
gix/gitoxide = future research only
legacy WAL / ledger / bus.graveyard = adapter or derived feedback, never authority
```

If proof fails:

```text
fallback = minimal semantic tape
```

No two canonical substrates may coexist.

## 4. Core Data Shapes

### CanonicalObject

```rust
struct CanonicalObject {
    id: ObjectId,
    kind: ObjectKind,
    body_cid: Cid,
    parent_ids: Vec<ObjectId>,
    created_by: ActorId,
    created_at: Timestamp,
    evidence: EvidenceRefs,
}
```

Rules:

- `ObjectId` is not a frontend session id.
- `body_cid` is not a temporary file path.
- `created_at` is not ordering truth.
- `evidence` is part of the state transition.

### HeadState

```rust
struct HeadState {
    world_head: Option<ObjectId>,
    audit_head: Option<ObjectId>,
    rejection_head: Option<ObjectId>,
}
```

Rules:

- accepted transition advances `world_head` and `audit_head`;
- rejected transition advances `audit_head` / `rejection_head`, not
  `world_head`;
- view update advances no canonical head;
- preview run advances no world head unless explicitly accepted by gate.

### Candidate

```rust
struct Candidate {
    candidate_id: Cid,
    source_attempt: Cid,
    proposed_action: Action,
    proposed_payload: Cid,
    claims: Vec<Claim>,
}
```

Candidate may be:

- `SpecCandidate`
- `ArtifactCandidate`
- `ArtifactPatchCandidate`
- `RepairSignalCandidate`
- `PromptPatchCandidate`
- `ToolCallProposal`
- `DeliveryProposal`

Candidate may not directly write files, execute tools, update `HEAD`, update
canonical prompts, declare done, or deploy.

### EvidenceTuple

```rust
struct EvidenceTuple {
    prompt_capsule_cid: Cid,
    attempt_telemetry_cid: Cid,
    output_cid: Cid,
    predicate_decision_cid: Option<Cid>,
    resulting_object_id: Option<ObjectId>,
}
```

Rules:

- no `PromptCapsule` -> no LLM call;
- no `AttemptTelemetry` -> no Candidate;
- no `PredicateDecision` -> no accepted/rejected transition;
- no output CID -> no replay;
- no L4/L4.E reachability -> no audit claim.

### PredicateDecision

```rust
struct PredicateDecision {
    decision_id: Cid,
    candidate_id: Cid,
    accept: bool,
    predicate_results: Vec<PredicateResult>,
    reject_class: Option<RejectClass>,
    repair_hint_cid: Option<Cid>,
}
```

Minimum predicates:

```text
P1 Schema / parse
P2 Flow / policy
P3 Provenance / spec consistency
P4 Privacy / shielding / security
P5 Budget / resource bound
```

Model judgment is never a predicate.

### Product Capsules

`SpecCapsule` must link user inputs, prompt capsules, attempt telemetry, final
spec markdown CID, slot table, acceptance slots, and predicate decision.

`ArtifactBundle` must include artifact bundle CID, spec capsule CID, generation
attempt CID, files with path/CID/mime/hash, entrypoint, verifier results, and
timestamp. Generated output is never a naked HTML string.

`PreviewRunCapsule` is a read-only function of `ArtifactBundle`; it captures
redacted logs and never advances world head.

`ModificationRequestCapsule` turns user edits into new candidate input; it does
not patch files directly.

`TestScenarioSetCapsule` and `TestRunCapsule` come from spec acceptance slots.
Hidden oracle does not enter generator prompt.

`BuildSessionView` is a derived UI projection. Deleting cache must allow
rebuild from canonical evidence.

## 5. Micro-Implementation

Every complex implementation must map to this toy model:

```python
class TuringOSMicro:
    def __init__(self, substrate, llm, predicates):
        self.substrate = substrate
        self.llm = llm
        self.predicates = predicates

    def append_evidence(self, kind, body, parents=()):
        return self.substrate.append(kind=kind, body=body, parents=parents)

    def rtool(self, session_id):
        return derive_visible_frame(self.substrate, session_id)

    def llm_attempt(self, frame, schema=None):
        prompt = self.append_evidence("PromptCapsule", {
            "visible_context_hash": hash(frame.visible_text),
            "read_set": frame.read_set,
            "schema": schema,
        })
        raw = self.llm.complete(frame.visible_text, schema=schema)
        attempt = self.append_evidence("AttemptTelemetry", {
            "prompt": prompt.id,
            "model": raw.model,
            "latency_ms": raw.latency_ms,
            "token_usage": raw.token_usage_or_unknown,
            "output_cid": store(raw.output),
        }, parents=[prompt.id])
        return Candidate(source_attempt=attempt.id, payload=raw.output)

    def decide(self, candidate):
        result = self.predicates.evaluate(candidate, self.substrate.heads())
        decision = self.append_evidence(
            "PredicateDecision",
            result.to_json(),
            parents=[candidate.source_attempt],
        )
        if result.accept:
            return self.substrate.commit_accepted(candidate, decision)
        return self.substrate.append_rejected(candidate, decision)
```

Facts:

- LLM only completes; it does not write world.
- Intermediate objects enter evidence first.
- `decide()` is the only accepted/rejected transition point.
- UI is always `derive_build_session_view()`.

## 6. Product Goal

TuringOS lets a user open `/build`, describe a requirement, receive an
evidence-backed spec, generate an artifact bundle, preview it in sandbox,
modify it naturally, run spec-derived tests, and receive an auditable delivery.

Every delivery must answer:

- which inputs produced the spec;
- which LLM attempt produced the artifact;
- which artifact bundle the preview read;
- which acceptance slots produced tests;
- which gates passed;
- which failures were rejected;
- whether any naked LLM call occurred;
- whether temp file/session/dashboard became truth;
- whether replay can happen without another LLM call.

## 7. Invariants

- One canonical substrate.
- All signals reconstructible.
- Rejected is state.
- UX never writes world.
- LLM never judges itself.
- Small atoms, no architecture by vibes.

## 8. K0-K8 Roadmap

### K0 Reality Proof / Stack Freeze

Prove current substrate before changing behavior. Produce:

- `docs/roadmap/TURINGOS_REALITY_MAP_K0.md`
- `docs/adr/0001-substrate-path-b-pragmatic-git2.md`
- `docs/architecture/legacy_anchor_reconciliation.md`

Kill criteria include unproven ChainTape/CAS/HEAD_t, naked LLM calls, preview
temp-file truth, web session canonical state, or competing substrates.

### K1 Evidence Rail Closure

Connect existing TISR `spec -> generate -> preview` to evidence rails. Ensure
PromptCapsule, AttemptTelemetry, EvidenceCapsule, L4, L4.E, replay, and audit.

### K2 ArtifactBundle

Generated software becomes content-addressed `ArtifactBundle`; preview,
download, and test read only bundles.

### K3 Preview Truth Path

Preview is `render(ArtifactBundle CID) -> PreviewRunCapsule`. It rejects
arbitrary paths and captures/redacts logs.

### K4 Single URL Micro-MVP

`/build` completes `spec -> generate -> preview`, refresh restores from
canonical evidence, and regenerate creates a modification request.

### K5 Iterative Edit / Regenerate

Natural-language edits create `ModificationRequestCapsule` and new artifact
versions. Old versions remain previewable, downloadable, and auditable.

### K6 Spec-Derived Real Scenario Audit

Acceptance slots become `TestScenarioSetCapsule` and `TestRunCapsule`. Hidden
oracle is protected. Accepted delivery requires passing test run CID.

### K7 Production Hardening As Derived Views

Session isolation, restore, cost dashboard, prompt versioning, canary
promotion, and audit packet are derived from evidence.

### K8 Self-Improvement Without Self-Corruption

Failures create patch proposals. Patch proposals do not apply directly; they
require eval-clean, Veto, and promotion receipt.

## 9. Commit Queue

```text
C0 Reality Map Hard Gate
C1 Path Decision + Chronology Seal
C2 No-New-Substrate Regression
C3 No Naked LLM Call
C4 ArtifactBundle CAS Wire
C5 Preview Truth Path
C6 BuildSession Derived View
C7 Friendly Error with L4.E
C8 Minimal Single URL MVP
C9 Edit/Regenerate Versioning
C10 Spec-Derived TestRun
C11 Audit Packet
```

Do not jump the queue.

## 10. Authorization Policy

Class 0-2 may be authorized by ordinary "go / ok / continue" language.

Class 3 requires exact atom id, allowed files, forbidden files, acceptance
commands, and negative tests.

Class 4 requires:

```text
RATIFY CLASS-4:
  Atom: <atom id>
  Files: <affected files>
  Scope: <exact scope>
  I authorize this Class 4 implementation atom.
```

`constitution.md` can only be modified by human sudo.

## 11. Forbidden Now

Do not:

- bypass PromptCapsule / AttemptTelemetry for UX speed;
- let web session memory become truth;
- let preview temp files become artifact truth;
- add parallel CAS/hash/WAL/HEAD_t substrate;
- introduce Next.js, Tauri, RMCP, Wasmtime, gix migration, remote A2A,
  LangGraph, CrewAI, or Agents SDK as kernel runtime;
- treat Structured Outputs as predicate;
- let LATEST.md claim closure before tests/evidence.

## 12. MVP Done

MVP done means:

1. user opens `/build`;
2. user describes requirements;
3. system creates evidence-backed `SpecCapsule`;
4. system creates evidence-backed `ArtifactBundle`;
5. preview reads only `ArtifactBundle` CID;
6. refresh restores `BuildSessionView`;
7. natural-language edits create new bundles;
8. failures enter L4.E with friendly errors;
9. replay/audit rebuilds without LLM;
10. no session, temp preview, dashboard cache, or LLM critic becomes source of
    truth.

Differentiated done additionally includes spec-derived tests, replayable test
runs, protected hidden oracle, passing test run CID, and audit packet.

Production done additionally includes session isolation, derived cost dashboard,
prompt promotion receipts, gated self-improvement, and machine-checkable Class
3/4 gates.

## 13. Final Compression

```text
K0 prove current substrate
  -> K1 close evidence rails
  -> K2 make artifact physical
  -> K3 make preview read-only bundle rendering
  -> K4 ship single URL MVP
  -> K5 version edits/regeneration
  -> K6 spec-derived tests
  -> K7 production hardening as derived views
  -> K8 self-improvement with gates
```

Core sentence:

```text
Do not design TuringOS as a complex system. Compress it into a replayable build
tape: LLM proposes candidate bytes, predicates decide whether world advances,
and all UX is a derived view of that tape.
```
