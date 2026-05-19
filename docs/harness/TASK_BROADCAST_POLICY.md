# Task Broadcast Policy

`TASK_BOARD.json` is the development control plane. It is not V5 runtime truth.

Workers may read it. Workers may not modify it.

## Status Machine

```text
open -> claimed -> pr_open -> needs_repair -> merged
open -> superseded
open -> blocked
open -> retired
```

Claims are expressed by PRs, not direct board edits.

## Board Guards

- `max_repair_attempts: 3`
- `worker_halt_required: true`
- `conflict_policy: "supersede_on_dirty"`
- `claim_required: true` for smoke tasks
- `claim_method: "draft_pr"` for smoke tasks

Workers must submit a WorkerReport containing `[WORKER_HALT]`, then stop.
Meta converts dirty merge states into `SUPERSEDE`; dirty PRs are evidence, not
accepted state.

## Draft PR Claims

Smoke tasks require draft PR claims even for Class 0/1. Claim facts come from
GitHub PRs, not worker edits to `TASK_BOARD.json`.

Valid claim title:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

When multiple draft PRs claim the same atom, the earliest valid claim by
`createdAt` wins. Later duplicates become `SUPERSEDE` or duplicate evidence.

## Claim Modes

- Class 0/1: `open_pool`; duplicates allowed; first valid PR wins.
- Class 2: `soft_lease`; draft PR claim preferred.
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
