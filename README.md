# TuringOS V5

TuringOS V5 is an independent product repo. The local TuringOS system workbench
may be used as a development tool, but V5 runtime truth must not depend on old
genesis files, old evidence paths, local archive paths, dashboards, sessions, or
the harness board.

## Start

Every CLI session starts from the repo root:

```bash
cd /home/zephryj/projects/turingosv5
cat AGENTS.md
cat AGENT_ENTRY.md
```

`AGENT_ENTRY.md` does not assign a role by itself. A session becomes MetaAI,
WorkerAI, AuditorAI, or VetoAI only when the Human Architect, a TaskPacket,
ReviewPacket, or Meta continuation explicitly assigns that role.

## Harness Scope

The harness is a CLI intake layer. It is not the V5 kernel, scheduler, runtime
truth source, or worker execution manager.

Normal distribution is board-first:

1. MetaAI publishes or reconciles `docs/harness/broadcast/TASK_BOARD.json` and
   TaskPackets.
2. Separate WorkerAI CLI sessions read the board.
3. Each WorkerAI self-selects exactly one eligible task.
4. Each WorkerAI claims by draft PR before implementation.
5. MetaAI reconciles PR claim evidence, CI, review, Veto, and WorkerReport.

MetaAI should not replace this flow with private worker-specific execution
instructions unless the Human Architect explicitly requests a direct assignment
or continuation.

## Parallel Workers

`TASK_BOARD.json` is a projection/control view, not a lock service. The live
claim fact is the earliest valid draft PR whose title matches:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

Workers must check open PR claims before worktree creation, re-check before
implementation edits, open the draft PR claim before coding, and stop if an
earlier valid claim owns the same atom.

If only one unblocked self-select task exists, opening three WorkerAI CLIs may
create duplicate claim races. The protocol prevents duplicate accepted state,
but efficient parallelism requires multiple unblocked tasks.

## Current Phase

The current wave is DevTape v1.0 preflight. It focuses on development tooling
and boundaries:

- shared `AGENTS.md` and `AGENT_ENTRY.md`;
- explicit role entries under `docs/harness/roles/**`;
- board and TaskPacket intake;
- WorkerReport, MergeDecision, and schema contracts;
- local workbench lineage and runtime boundary guards.

No product runtime feature is claimed by the harness itself.

## Gate

```bash
jq . docs/harness/broadcast/TASK_BOARD.json
cargo check --workspace
cargo test --workspace
```
