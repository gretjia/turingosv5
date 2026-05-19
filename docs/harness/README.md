# V5 Harness

The harness turns multi-CLI development into candidate state transitions.

Worker CLIs do not freely write code. They read `AGENT_ENTRY.md`, select one
eligible TaskPacket from the Task Broadcast, work only inside allowed paths, run
the required tests, open a PR, submit WorkerReport, and stop.

Meta AI owns the broadcast board, PR reconciliation, review coordination, Veto
requests, merge decisions, and development evidence recording.

## Files

- `AGENT_ENTRY.md`: single worker entry point.
- `AGENTS.md`: shared cross-agent contract.
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
