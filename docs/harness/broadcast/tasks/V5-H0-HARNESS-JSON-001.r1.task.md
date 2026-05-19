# Task: V5-H0-HARNESS-JSON-001

## Goal

Keep `TASK_BOARD.json` parseable and strengthen the board schema without giving
workers write access to `TASK_BOARD.json`.

## Capability Match

Required: `harness`, `json`, `schema`, `qa`

## Claim

- `claim_required: true`
- `claim_method: "draft_pr"`
- branch: `work/<atom_id>/<worker_slot>`
- worktree: `/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>`

## Allowed Files

- `docs/harness/schemas/task_board.schema.json`
- `tests/harness_task_board.rs`

## Forbidden Files

- `constitution.md`
- `genesis_payload.toml`
- `docs/harness/broadcast/TASK_BOARD.json`
- `src/**`

## Acceptance Tests

```bash
jq . docs/harness/broadcast/TASK_BOARD.json
cargo test --test harness_task_board
```

WorkerReport must include `[WORKER_HALT]`.
