# Worker Harness

Universal CLI workers are black-box implementers constrained by Task Broadcast.
The harness gives intake rules; it does not dispatch workers or hold canonical
state.

Workers start from the main checkout for intake only:

```bash
cd /home/zephryj/projects/turingosv5
```

Task code must be edited in:

```text
/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>
```

## Single-Shot Loop

1. Read `AGENTS.md`.
2. Read `AGENT_ENTRY.md`.
3. Read `docs/harness/roles/WORKER_ENTRY.md`.
4. Read `docs/harness/WORKER_HARNESS.md`.
5. Read `docs/harness/TASK_BROADCAST_POLICY.md`.
6. Read `docs/harness/broadcast/TASK_BOARD.json`.
7. Pick exactly one eligible open task.
8. Read the TaskPacket.
9. Run `git fetch origin` and `gh pr list --state open`.
10. Skip any atom with an active valid draft PR claim.
11. Create branch `work/<atom_id>/<worker_slot>` from `origin/main`.
12. Create the isolated task worktree.
13. Re-check open PRs for the same atom before implementation edits.
14. If another valid claim appeared, output `[WORKER_HALT]` and stop.
15. Create a claim commit.
16. Open a draft PR titled `[CLAIM][<atom_id>][ClassX] <task title>`.
17. Refresh open PRs; if an earlier valid claim exists by `createdAt`, mark the
    current PR duplicate/superseded when possible, output `[WORKER_HALT]`, and
    stop.
18. Modify only allowed files.
19. Run required tests.
20. Update the same PR with WorkerReport.
21. Run `gh pr ready`.
22. Output `[WORKER_HALT]`.
23. Stop the current task.

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

Claim first, code after. A worker must not begin implementation until its draft
PR claim is visible and no earlier valid claim owns the same atom.

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
