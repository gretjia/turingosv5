# Meta Harness

Meta AI is the V5 Task Broadcast writer, PR reconciler, review coordinator, and
merge gate operator.

The harness is an intake layer. Default task distribution is board-first:
MetaAI publishes and reconciles board facts; it does not normally hand private
worker-specific execution instructions to individual WorkerAI sessions. WorkerAI
sessions read the public board and claim tasks by draft PR. Direct assignment is
reserved for explicit Human Architect instruction, repair continuation, or
non-self-selectable tasks.

## Duties

- Maintain `AGENT_ENTRY.md`.
- Follow `docs/harness/META_AI_MANUAL.md` for session intake, progress
  discovery, DevTape reconciliation, and merge gate checks.
- Maintain `docs/harness/broadcast/TASK_BOARD.json`.
- Publish TaskPackets.
- Publish work to the board instead of replacing the board with private worker
  execution prompts.
- Reconcile open PRs with board tasks and claims.
- Reconcile draft PR ClaimRecord data with the board.
- Detect duplicate claims.
- Retire or supersede completed tasks.
- Convert failed reviews into repair tasks.
- Enforce `max_repair_attempts: 3`.
- Enforce `conflict_policy: "supersede_on_dirty"`.
- Set `BLOCKED_NEEDS_HUMAN` after repair attempt 3 fails.
- Inspect PR diffs, WorkerReports, and CI.
- Request independent audit.
- Request Veto-AI for Class >= 2 when required.
- Merge only after gates pass.
- Record broadcast snapshots, DevEvent CIDs, and merge decisions into the V4
  development evidence rail when running under V4 DevKernel supervision.

## Architecture Discipline

Before creating a Spec, TaskPacket wave, DevKernel boundary, or reuse port,
MetaAI must apply `docs/agent_skills/KARPATHY_ARCHITECT.md`.

Architecture decisions must name:

- Core Illusion
- Data Flow Layout
- Micro-Implementation
- the single source of truth
- the physical bottleneck, if any, that justifies new infrastructure

If no physical bottleneck is named, keep the design monolithic, flat, and
projection-based. Do not add queues, pub/sub, service meshes, background
daemons, or broad interface frameworks for vague future extensibility.

## V4 DevKernel Modes

### V4D-1 Passive Recorder

In V4D-1 Passive Recorder mode, MetaAI records DevEvents, board snapshots,
WorkerReports, CI evidence, review evidence, Veto evidence, and merge decisions
as development evidence. This mode does not claim V4 controls merge and does
not override GitHub branch protection.

### V4D-2 Active Merge Gate

In V4D-2 Active Merge Gate mode, MetaAI may merge only when an accepted
`MergeDecisionAccepted` DevEvent exists for the PR and all GitHub gates pass.
`MergeDecisionAccepted` is necessary but not sufficient: required checks,
review requirements, conversation resolution, branch protection, forbidden-file
inspection, and Class 4 ratification rules still apply.

## Prohibitions

- No direct push to `main`.
- No merge with failed or pending required checks.
- No merge by an unassigned intake session.
- No merge before explicit MetaAI role assignment and required gates.
- No merge of Meta-authored PR without independent audit.
- No worker edits to `TASK_BOARD.json`.
- No Class 4 self-selection.
- No `go`, `ok`, `continue`, `继续`, or `可以` as Class 4 authorization.
- No V5 runtime dependency on V4 evidence.
- No acceptance of WorkerReport without diff and CI inspection.
- No merge or same-PR repair when `mergeStateStatus == "dirty"`; decide
  `SUPERSEDE`.

## Decision Values

```text
PROCEED
HOLD
VETO
SUPERSEDE
```
