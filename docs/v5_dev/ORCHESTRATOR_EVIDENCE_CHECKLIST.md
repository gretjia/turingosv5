# Orchestrator Evidence Checklist

Status: reusable MetaAI checklist.

Use this checklist before a PR can advance from Candidate toward accepted
development state.

## Required Evidence

- Task exists
- Role assignment evidence exists
- WorkerReport exists
- PRCreated exists
- CIResult exists
- ReviewVerdict exists when required
- VetoVerdict exists when required
- MergeDecisionAccepted exists before merge
- GitHub gates pass
- No author final-audit
- No forbidden files touched
- Class 4 ratified when touched

## Boundary Checks

- provider label is not authority
- branch protection satisfied
- all review threads resolved
- WorkerReport includes `[WORKER_HALT]`
- TASK_BOARD.json is control view, not runtime truth
- V5 runtime does not depend on V4 evidence, genesis, or local paths
- dirty merge state uses SUPERSEDE, not manual conflict repair

## Simplicity Checks

Use `docs/agent_skills/KARPATHY_SIMPLE_CODE.md` as the style gate.

HOLD if:

- a dependency was added without explicit task permission
- a broad abstraction was added without a real boundary
- code hides simple data flow behind manager/factory/engine naming
- the worker compressed code into unclear cleverness
- old-system code was copied instead of isolated behind a port/fixture

## Architecture Checks

Use `docs/agent_skills/KARPATHY_ARCHITECT.md` for MetaAI architecture, Spec,
task-wave, DevKernel boundary, and reuse-port decisions.

HOLD if:

- the core data shape is named nowhere
- no micro end-to-end model exists for a new subsystem
- new infrastructure is justified by a physical bottleneck nowhere, only vague
  future extensibility
- board/runtime truth boundary is preserved nowhere in the decision record
- old-system reuse bypasses a narrow port or characterization fixture

## Required CID References

The MergeDecision must reference DevEvent CIDs for:

- task event
- role assignment event
- worker report event
- PR created event
- CI result event
- review verdict event when required
- Veto verdict event when required
- branch protection snapshot
- bootstrap exception when used
- merge decision event

## Release Recommendation

`PROCEED` is allowed only when every required evidence item exists and every
boundary check passes. Otherwise use `HOLD`, `VETO`, or `SUPERSEDE`.
