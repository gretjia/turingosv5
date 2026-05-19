# Passive Recorder MVP

Status: V4D-1 Passive Recorder contract.

V4D-1 records events, stores development evidence, and derives audit views. It
does not block merge and does not claim V4-native governance.

V4D-1 must not state that V4 controls merge. GitHub branch protection, CI,
reviews, and human-controlled repository policy remain the active enforcement
surface in this phase.

## Behavior

- records events
- records board snapshots
- records WorkerReport evidence
- records CI, review, Veto, and MergeDecision evidence
- does not block merge
- does not claim V4-native governance
- does not treat TASK_BOARD.json as runtime truth

## Minimal Command Targets

```bash
turingos dev event append --envelope event.json
turingos dev audit --project turingosv5
turingos dev board derive
```

These command names are targets for the V4 adapter. This document does not
claim the commands are implemented yet.

## Active Gate Boundary

Active Merge Gate requires MergeDecisionAccepted. The active gate is V4D-2, not
V4D-1.

In V4D-2, an accepted `MergeDecisionAccepted` DevEvent becomes a necessary input
to merge review. It is still not sufficient by itself because GitHub branch
protection, CI, review, conversation resolution, and Class 4 ratification rules
remain mandatory.
