# Derived Task Board Contract

Status: V4D-Q1 contract.

`TASK_BOARD.json` is a development control view. In V4D-Q1, the board becomes a
derived projection from V4 DevKernel DevEvents.

## Rules

- board is rebuildable
- manual mutation is drift
- board does not mutate DevKernel state
- source = v4_devkernel_derived
- source_event_cids are required
- manual source is invalid for V4D-Q1

Deleting the derived board must not delete development state. Mutating the board
must not mutate DevKernel state. If the board contents disagree with the
projection from source events, audit reports drift and MetaAI must regenerate or
supersede the view.

## Projection Fields

Each task projection includes:

- atom_id
- phase
- lane
- class
- status
- title
- task_packet
- claim_required
- claim_method
- source_event_cids
- pr_number when claimed or ready
- ci status when known
- review status when known
- veto status when known
- merge_decision when known

## Drift Scenario

1. DevKernel has DevTaskCreated and WorkerReportSubmitted events for an atom.
2. A local editor changes the board status by hand.
3. `turingos dev board derive` produces a different projection.
4. `turingos dev audit --project turingosv5` reports board drift.
5. The hand edit is rejected as view drift, not accepted state.
