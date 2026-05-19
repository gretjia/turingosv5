# Worker Entry

Use this role entry only when explicitly assigned worker work by the human
prompt, TaskPacket, or Meta continuation. During H0 smoke, a worker role session
may self-select exactly one eligible open task from the board.

Begin intake from the main checkout:

```bash
cd /home/zephryj/projects/turingosv5
```

Read in order:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/WORKER_HARNESS.md`
4. `docs/harness/TASK_BROADCAST_POLICY.md`
5. `docs/harness/broadcast/TASK_BOARD.json`
6. The selected TaskPacket

Task code must be edited only in:

```text
/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>
```

Create the task branch from latest `origin/main`:

```text
work/<atom_id>/<worker_slot>
```

Claim by draft PR titled:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

Run required tests, update the same PR with WorkerReport, run `gh pr ready`,
print `[WORKER_HALT]`, and stop. H0 smoke is single-shot; do not start another
task without a new explicit assignment.
