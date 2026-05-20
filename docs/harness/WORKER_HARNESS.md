# Worker Harness

Universal CLI workers are black-box implementers constrained by Task Broadcast.
The harness gives intake rules; it does not dispatch workers or hold canonical
state.

Workers start from the main checkout for intake only:

```bash
cd /home/zephryj/projects/turingosv5
```

Primary WorkerAI task code is submitted through a generated soft sandbox:

```text
/home/zephryj/projects/turingosv5-sandboxes/<worker_slot>/<atom_id>
```

## Single-Shot Loop

1. Read `AGENTS.md`.
2. Read `AGENT_ENTRY.md`.
3. Read `docs/harness/roles/WORKER_ENTRY.md`.
4. Read `docs/harness/WORKER_HARNESS.md`.
5. Read `docs/harness/TASK_BROADCAST_POLICY.md`.
6. Read `docs/harness/broadcast/TASK_BOARD.json` as a projection only.
7. Claim exactly one eligible open task through:
   `turingos-dev worker claim next --store .turingos_system/devtape/turingosv5/events.jsonl --repo /home/zephryj/projects/turingosv5 --out-root /home/zephryj/projects/turingosv5-sandboxes --worker <worker_slot>`.
8. Read the generated sandbox `TASK.md`, `CONTEXT.md`, and exported
   `allowed_files/**`.
9. Modify no repo files directly. Write only `submit/candidate.patch` and
   `submit/WorkerReport.json` inside the sandbox.
10. Submit through:
   `turingos-dev worker sandbox submit --dir <sandbox> --store .turingos_system/devtape/turingosv5/events.jsonl --repo /home/zephryj/projects/turingosv5 --worktree-root /home/zephryj/projects/turingosv5-worktrees --worker <worker_slot>`.
11. Output `[WORKER_HALT]`.
12. Stop the current task.

H0 smoke workers must not run an automatic task loop. One worker process handles
one selected TaskPacket, submits once, prints `[WORKER_HALT]`, and stops.

## ClaimRecord

Legacy draft PR fallback is available only when a TaskPacket or Meta
continuation explicitly asks for `claim_method: "draft_pr"`.

The legacy draft claim PR body must include:

- board version/hash
- TaskPacket path/hash
- allowed files
- forbidden files
- worker profile
- claim timestamp

The same PR becomes the ready implementation PR; do not open a second PR.

Claim first, code after. A worker must not begin implementation until
`turingos-dev worker claim next` has produced a sandbox, or until a legacy draft
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
`claim_required == true`, class is within the worker profile, required
capabilities match, blockers are empty, and no active claim blocks the atom.
Sandbox intake is primary; `claim_method == "draft_pr"` is legacy fallback.

After PR submission, stop. Continuation requires Meta request or a new
TaskPacket.
