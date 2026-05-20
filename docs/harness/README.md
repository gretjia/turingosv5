# V5 Harness

The harness is a CLI intake layer for multi-CLI development. It is not the V5 kernel,
scheduler, runtime truth source, or worker execution manager.

CLI sessions start unassigned. They read `AGENTS.md` first, then
`AGENT_ENTRY.md`, then enter a role only when the human prompt, TaskPacket,
ReviewPacket, or Meta continuation explicitly assigns one.

Worker role sessions do not freely write code. They first enter the public task
market, self-select one eligible TaskPacket, claim it, then narrow into the
selected task's execution context. Local workers use `turingos-dev worker claim
next`; external GitHub-only workers use
`docs/harness/boot_prompts/remote_worker_market.md`.

Meta role sessions own the broadcast board, PR reconciliation, review
coordination, Veto requests, merge decisions, and development evidence
recording.

Normal distribution is DevTape/board-first. MetaAI appends DevEvents and
derives the board; WorkerAI sessions use that board as a public task market.
The board is a projection/control view, while live coordination comes from
`TaskClaimed` evidence for local sandbox workers and GitHub open PR claim
overlay for remote workers. Draft PR claim remains a legacy fallback for tasks
that explicitly require it.

## Files

- `AGENT_ENTRY.md`: shared unassigned CLI entry point.
- `AGENTS.md`: shared cross-agent contract.
- `docs/harness/roles/**`: role-specific entry points.
- `docs/harness/META_HARNESS.md`: Meta duties and prohibitions.
- `docs/harness/WORKER_HARNESS.md`: worker task loop.
- `docs/harness/TASK_BROADCAST_POLICY.md`: task market rules.
- `docs/harness/boot_prompts/remote_worker_market.md`: GitHub-only worker entry.
- `docs/harness/broadcast/TASK_BOARD.json`: Meta-only task board.
- `docs/harness/schemas/**`: JSON schema contracts.
- `docs/harness/templates/**`: report and decision templates.

## Current Gate

```bash
cargo test --test harness_task_board
```
