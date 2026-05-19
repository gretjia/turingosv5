# Worker Harness

Universal CLI workers are black-box implementers constrained by Task Broadcast.

Workers start from the main checkout for intake only:

```bash
cd /home/zephryj/projects/turingosv5
```

Task code must be edited in:

```text
/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>
```

## Single-Shot Loop

1. Read `AGENT_ENTRY.md`.
2. Read `AGENTS.md`.
3. Read `docs/harness/WORKER_HARNESS.md`.
4. Read `docs/harness/TASK_BROADCAST_POLICY.md`.
5. Read `docs/harness/broadcast/TASK_BOARD.json`.
6. Pick exactly one eligible open task.
7. Read the TaskPacket.
8. Run `git fetch origin` and `gh pr list --state open`.
9. Create branch `work/<atom_id>/<worker_slot>` from `origin/main`.
10. Create the isolated task worktree.
11. Create a claim commit.
12. Open a draft PR titled `[CLAIM][<atom_id>][ClassX] <task title>`.
13. Modify only allowed files.
14. Run required tests.
15. Update the same PR with WorkerReport.
16. Run `gh pr ready`.
17. Output `[WORKER_HALT]`.
18. Stop the current task.

H0 smoke workers must not run an automatic task loop. One worker process handles
one selected TaskPacket, opens one PR, reports once, prints `[WORKER_HALT]`, and
stops.

## ClaimRecord

The draft claim PR body must include:

- board version/hash
- TaskPacket path/hash
- allowed files
- forbidden files
- worker profile
- claim timestamp

The same PR becomes the ready implementation PR; do not open a second PR.

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

A task is eligible only when `status == "open"`, `self_select == true`,
`claim_required == true`, `claim_method == "draft_pr"`, class is within the
worker profile, required capabilities match, blockers are empty, and no active
claim blocks the atom.

After PR submission, stop. Continuation requires Meta request or a new
TaskPacket.
