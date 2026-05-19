# Core Dev Flow

Status: DevTape v1.0 architecture pin.

## Core Illusion

TuringOS V5 development governance is an append-only list of development facts
projected into human-readable views and merge decisions.

## Data Flow Layout

```text
DevEvent JSONL tape -> BoardProjection
DevEvent JSONL tape -> MergeDecisionCandidate
DevEvent {
  event_id
  event_type
  actor
  subject
  payload
  previous_event_id
  observed_at
  runtime_truth: false
}
```

Minimal derived shapes:

```text
BoardProjection {
  generated_from: DevEvent[]
  tasks: TaskProjection[]
}

MergeDecisionCandidate {
  atom_id
  required_evidence
  missing_evidence
  decision
}
```

## Micro-Implementation

```text
tape = []

append(event):
  require event.runtime_truth == false
  require event.previous_event_id == tape.last.event_id or tape is empty
  tape.push(event)

derive_board(tape):
  project DevTaskCreated, DevTaskClaimed, WorkerReportSubmitted into task rows

derive_merge_candidate(tape, atom_id):
  inspect task, claim, report, CI, review, Veto, and GitHub evidence events
```

## Kernel-Driven MVP Boundary

The first executable loop is local and append-only:

```bash
turingos-dev event append --file event.json --store .turingos_system/devtape/turingosv5/events.jsonl
turingos-dev board derive --store .turingos_system/devtape/turingosv5/events.jsonl --out docs/harness/broadcast/TASK_BOARD.json
turingos-dev audit --store .turingos_system/devtape/turingosv5/events.jsonl --board docs/harness/broadcast/TASK_BOARD.json
turingos-dev merge check --store .turingos_system/devtape/turingosv5/events.jsonl --pr <number>
```

MetaAI creates DevEvents; the board is derived from DevEvents; audit fails if a
board fact lacks source DevTape records.

## Runtime Boundary

This flow governs development evidence only. V5 runtime must not read the board,
harness, local TuringOS system workbench paths, old evidence, old genesis, or
local archive paths as runtime truth.
