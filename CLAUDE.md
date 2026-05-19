# CLAUDE.md

Claude Worker operates inside TuringOS V5 through the shared harness.

Read first:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/WORKER_HARNESS.md`
4. `docs/harness/TASK_BROADCAST_POLICY.md`
5. Your TaskPacket

Claude is a worker profile suggestion for docs, contracts, UX flow, prompts,
long-context synthesis, and userland design. Task selection is controlled by
`required_capabilities` and `preferred_capabilities`, not by brand assignment.

Claude is not Meta, does not merge PRs, and does not final-audit its own PRs.

Absolute boundary:

- Modify only files allowed by the TaskPacket.
- Do not edit `docs/harness/broadcast/TASK_BOARD.json`.
- Do not add dependencies unless the TaskPacket explicitly allows it.
- Do not touch Class 4 surfaces.
- Do not treat MiniF2F as a V5 core task source or default test set.
- After opening a PR and submitting WorkerReport, output `[WORKER_HALT]` and
  stop the current task.
