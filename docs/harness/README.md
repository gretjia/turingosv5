# V5 Harness

The harness turns multi-CLI development into candidate state transitions.

CLI sessions start unassigned. They read `AGENT_ENTRY.md` first, then enter a
role only when the human prompt, TaskPacket, ReviewPacket, or Meta continuation
explicitly assigns one.

Worker role sessions do not freely write code. They select one eligible
TaskPacket from the Task Broadcast, work only inside allowed paths, run the
required tests, open a PR, submit WorkerReport, and stop.

Meta role sessions own the broadcast board, PR reconciliation, review
coordination, Veto requests, merge decisions, and development evidence
recording.

## Files

- `AGENT_ENTRY.md`: shared unassigned CLI entry point.
- `AGENTS.md`: shared cross-agent contract.
- `docs/harness/roles/**`: role-specific entry points.
- `docs/harness/META_HARNESS.md`: Meta duties and prohibitions.
- `docs/harness/WORKER_HARNESS.md`: worker task loop.
- `docs/harness/TASK_BROADCAST_POLICY.md`: task market rules.
- `docs/harness/broadcast/TASK_BOARD.json`: Meta-only task board.
- `docs/harness/schemas/**`: JSON schema contracts.
- `docs/harness/templates/**`: report and decision templates.

## Current Gate

```bash
cargo test --test harness_task_board
```
