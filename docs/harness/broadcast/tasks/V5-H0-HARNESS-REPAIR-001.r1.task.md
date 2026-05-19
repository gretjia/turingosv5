# Task: V5-H0-HARNESS-REPAIR-001

## Goal

Bound repair attempts at 3 and require dirty merge states to be superseded
instead of repaired in place.

## Capability Match

Required: `harness`, `meta-governance`, `policy`

## Claim

- `claim_required: true`
- `claim_method: "draft_pr"`
- branch: `work/<atom_id>/<worker_slot>`
- worktree: `/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>`

## Allowed Files

- `docs/harness/TASK_BROADCAST_POLICY.md`
- `docs/harness/FAILURE_PLAYBOOK.md`
- `docs/harness/DIRTY_TREE_POLICY.md`
- `docs/harness/META_HARNESS.md`
- `docs/harness/templates/MergeDecision.md`
- `tests/harness_task_board.rs`

## Forbidden Files

- `constitution.md`
- `genesis_payload.toml`
- `docs/harness/broadcast/TASK_BOARD.json`
- `src/**`

## Acceptance Tests

```bash
cargo test --test harness_task_board
```

WorkerReport must include `[WORKER_HALT]`.
