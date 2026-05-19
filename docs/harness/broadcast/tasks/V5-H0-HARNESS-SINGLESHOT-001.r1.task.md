# Task: V5-H0-HARNESS-SINGLESHOT-001

## Goal

Make the H0 worker lifecycle single-shot and require exact `[WORKER_HALT]`
confirmation after PR and WorkerReport.

## Capability Match

Required: `harness`, `docs`, `worker-lifecycle`

## Claim

- `claim_required: true`
- `claim_method: "draft_pr"`
- branch: `work/<atom_id>/<worker_slot>`
- worktree: `/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>`

## Allowed Files

- `AGENT_ENTRY.md`
- `docs/harness/WORKER_HARNESS.md`
- `docs/harness/templates/WorkerReport.md`
- `docs/harness/schemas/worker_report.schema.json`
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
