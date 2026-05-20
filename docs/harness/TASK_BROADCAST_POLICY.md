# Task Broadcast Policy

`TASK_BOARD.json` is the development control plane and public task market. It is
not V5 runtime truth, and it is not a worker lock service.

Workers may read it. Workers may not modify it.

## Market And Sandbox Phases

Selection phase:

- WorkerAI clients read the public task market in `TASK_BOARD.json`.
- WorkerAI clients inspect TaskPackets and self-select one eligible task.
- MetaAI does not privately assign a single task package unless the Human
  Architect explicitly requests direct assignment or continuation.
- Remote WorkerAI clients use a GitHub open PR claim overlay before choosing:
  if an open PR title or branch already contains the atom id, the worker skips
  that atom and chooses another eligible task.

Execution phase:

- After claim, context narrows to the selected TaskPacket and its
  `allowed_files`.
- Sandbox or sparse-checkout execution limits the working surface after
  self-selection; it must not replace the public task market.
- Worker output remains Candidate until MetaAI reconciles WorkerReport, CI,
  audit, Veto, and merge decision evidence.

## Status Machine

```text
open -> claimed -> submitted -> pr_open -> needs_repair -> merged
open -> superseded
open -> blocked
open -> retired
```

Claims are expressed by DevTape `TaskClaimed` events. Legacy draft PR claims are
compatibility evidence only for tasks that explicitly require
`claim_method: "draft_pr"`.

## Board Guards

- `max_repair_attempts: 3`
- `worker_halt_required: true`
- `conflict_policy: "supersede_on_dirty"`
- `claim_required: true` for smoke tasks
- `claim_method: "sandbox"` for new tasks

Workers must submit a WorkerReport containing `[WORKER_HALT]`, then stop.
Meta converts dirty merge states into `SUPERSEDE`; dirty PRs are evidence, not
accepted state.

## Sandbox Claims

Default worker intake is:

```bash
turingos-dev worker claim next --store .turingos_system/devtape/turingosv5/events.jsonl --repo /home/zephryj/projects/turingosv5 --out-root /home/zephryj/projects/turingosv5-sandboxes --worker <worker_slot>
```

The command appends `TaskClaimed` evidence and emits a generated sandbox. Worker
submissions return through `turingos-dev worker sandbox submit`, which validates
allowed files, `[WORKER_HALT]`, and local gates before recording
`WorkerReportSubmitted`.

## Draft PR Claims

Draft PR claims are legacy fallback for tasks that explicitly require
`claim_method: "draft_pr"`. Claim facts still must not come from worker edits to
`TASK_BOARD.json`.

Valid claim title:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

When multiple draft PRs claim the same atom, the earliest valid claim by
`createdAt` wins. Later duplicates become `SUPERSEDE` or duplicate evidence.

## Race Window

Two workers can read the same board before either one appends `TaskClaimed`.
This is allowed as a race condition, not as accepted parallel work.

Workers reduce duplicate waste by claiming through
`turingos-dev worker claim next`, which replays DevTape state before emitting a
sandbox. Legacy draft PR workers must still check open PRs before worktree
creation, before implementation edits, and after opening the draft claim.

The board may lag behind claims. Meta reconciliation updates the board after
observing DevTape and PR evidence; WorkerAI sessions must use DevTape
`TaskClaimed` records as the live coordination signal during the race window.

## Claim Modes

- Class 0/1: `open_pool`; duplicates allowed; first valid PR wins.
- Class 2: `soft_lease`; sandbox claim first, draft PR fallback if required.
- Class 3: Meta-opened only; independent audit and Veto required.
- Class 4: never self-select; direct assignment after exact ratification only.

## Selection Order

1. Priority: P0, P1, P2, P3.
2. Lower class first unless explicitly QA/audit.
3. Required capability match.
4. Empty blockers.
5. No active claim unless duplicate policy allows it.
6. Smaller allowed file surface.
7. Lower test cost.
8. Older task.
