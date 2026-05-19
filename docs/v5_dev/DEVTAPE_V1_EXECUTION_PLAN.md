# TuringOS V5 DevTape v1.0 Orchestrator Multi-Agent Execution Plan

Status: final preflight execution plan.

## Summary

Core Illusion: TuringOS V5 multi-agent development governance is
`DevEvent[] -> projections -> decisions`.

The harness, board, PRs, and GitHub checks are not the system. They are intake
and evidence surfaces around a small append-only development tape.

## Data Flow Layout

```text
DevEvent[] -> BoardProjection
DevEvent[] -> MergeDecisionCandidate
TaskPacket + ClaimRecord + WorkerReport -> DevEvent payloads
GitHubSnapshot -> DevEvent payload
```

## Execution Order

```text
O0 preflight
-> A0 architecture pin
-> A1 baseline + semantic close
-> A2 workbench boundary and A3 characterization seeds
-> A4 micro DevTape proof
-> A5 derived board projector
-> A6 CLI intake wrapper
-> A7 reuse port contract and A8 GitHub evidence snapshot
-> A9 active merge gate
-> A10 harness thinning
-> A11 audit + veto
```

## Task Index

- A0: Architecture Pin
- A1: Baseline + Semantic Close
- A2: Workbench Boundary
- A3: Characterization Seeds
- A4: Micro DevTape Proof
- A5: Derived Board Projector
- A6: CLI Intake Wrapper
- A7: Reuse Port Contract
- A8: GitHub Evidence Snapshot
- A9: Active Merge Gate
- A10: Harness Thinning
- A11: Orchestrator Audit + Veto

## Worker Model

Spark WorkerAI may execute narrow implementation tasks. WorkerAI must read
`docs/agent_skills/KARPATHY_SIMPLE_CODE.md`, stay inside allowed files, avoid
new dependencies unless explicitly allowed, submit WorkerReport, and stop with
`[WORKER_HALT]`.

MetaAI/Orchestrator uses `docs/agent_skills/KARPATHY_ARCHITECT.md` for
architecture, Spec, task-wave, and boundary decisions. WorkerAI must not read
or apply the architect skill unless explicitly assigned MetaAI duties.

## Release Gate

Every implementation PR must pass:

```text
jq board/tasks/schemas checks
cargo fmt --check
git diff --check
cargo check --workspace
cargo test --workspace
allowed/forbidden file inspection
WorkerReport with [WORKER_HALT]
AuditorAI PASS when required
VetoAI PASS when required
```

## Success Criteria

The wave is complete when:

```text
Human says: enter the TuringOS system and develop TuringOS V5
MetaAI can create DevTaskCreated
DevTape can derive board
WorkerAI can claim one task from derived board
WorkerAI can submit WorkerReport
MetaAI can derive MergeDecisionCandidate
GitHub snapshot can be recorded as evidence
V5 runtime does not depend on old workbench, board, harness, or local paths
```
