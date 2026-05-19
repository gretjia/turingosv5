# TuringOS V5

TuringOS V5 is the clean product repo bootstrapped from TuringOS V4.

V4 remains the development constitutional harness. V5 is the future product
world line and must not treat V4 evidence, V4 genesis, local handover paths, or
MiniF2F development corpora as production truth.

## Start Here

For CLI workers:

```bash
cat AGENT_ENTRY.md
```

For Meta:

```bash
cat CODEX.md
cat docs/harness/META_HARNESS.md
cat docs/harness/broadcast/TASK_BOARD.json
```

## Current Phase

V5-R0 Clean Bootstrap:

- shared `AGENTS.md`
- single `AGENT_ENTRY.md`
- brand adapters for Codex, Claude, and Gemini
- Task Broadcast board
- TaskPacket/WorkerReport/MergeDecision schemas
- PR/CI scaffolding
- harness boundary gate

No product feature work is claimed in R0.

## Gate

```bash
cargo test --test harness_task_board
```
