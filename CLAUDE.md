# CLAUDE.md

This compatibility file routes a Claude CLI session into the shared TuringOS V5
harness. It does not create a separate capability lane, review lane, or merge
authority.

Read first:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/WORKER_HARNESS.md` for worker tasks
4. `docs/harness/TASK_BROADCAST_POLICY.md`
5. Your TaskPacket or ReviewPacket

Task eligibility is determined by `docs/harness/broadcast/TASK_BOARD.json`, the
selected TaskPacket, the declared `worker_slot`, and explicit assignment. This
file does not provide default documentation, design, implementation, audit, or
review capabilities for Claude.

Absolute boundary:

- Modify only files allowed by the TaskPacket.
- Do not edit `docs/harness/broadcast/TASK_BOARD.json`.
- Do not add dependencies unless the TaskPacket explicitly allows it.
- Do not touch Class 4 surfaces.
- Do not merge or accept your own candidate work.
- Do not treat MiniF2F as a V5 core task source or default test set.
- After opening a PR and submitting WorkerReport, output `[WORKER_HALT]` and
  stop the current task.
