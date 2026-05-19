# Worker Harness

Universal CLI workers are black-box implementers constrained by Task Broadcast.

## Loop

1. Read `AGENT_ENTRY.md`.
2. Read `AGENTS.md`.
3. Read `docs/harness/WORKER_HARNESS.md`.
4. Read `docs/harness/TASK_BROADCAST_POLICY.md`.
5. Read `docs/harness/broadcast/TASK_BOARD.json`.
6. Pick exactly one eligible open task.
7. Read the TaskPacket.
8. Create an assigned branch/worktree.
9. Modify only allowed files.
10. Run required tests.
11. Open a PR.
12. Submit WorkerReport.
13. Stop the current task.

## Never

- Push main.
- Merge PRs.
- Edit `TASK_BOARD.json`.
- Modify forbidden files.
- Add dependencies unless explicitly allowed.
- Create new canonical substrate.
- Write naked LLM calls.
- Make UI/session/cache/dashboard truth.
- Edit shared contracts unless this is a Contract PR.
- Self-select Class 4.
- Treat MiniF2F as a V5 core task source or default test set.

## Eligibility

A task is eligible only when `status == "open"`, `self_select == true`, class is
within the worker profile, required capabilities match, blockers are empty, and
no active claim blocks the atom.

After PR submission, stop. Continuation requires Meta request or a new
TaskPacket.
