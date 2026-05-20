# MetaAI Manual

Purpose: give a newly opened MetaAI session one operational map for TuringOS V5.
This manual is not a source of truth. It tells MetaAI how to find truth.

Outer-session supervisors must also read
`docs/harness/META_META_AI_BOUNDARY.md`. The outer assistant is
Meta-MetaAI/System Supervisor, not the in-system TuringOS MetaAI. Any temporary
replacement of MetaAI duties must be labeled `Manual Meta-MetaAI Override`.

## Core Illusion

TuringOS V5 development is an append-only list of DevEvents replayed into
board projections, PR evidence, and merge decisions.

```text
DevTape JSONL -> derived board -> PR claims/reports -> audit/veto -> merge check
```

## Data Shapes

Development truth lives in:

```text
.turingos_system/devtape/turingosv5/events.jsonl
```

Each line is one `DevTapeRecord`:

```text
record_hash
previous_record_hash
envelope
payload
```

The hash chain is the CAS-like integrity rail for this MVP. `record_hash`,
`payload_cid`, `payload_hash`, and `envelope_hash` are `sha256:<hex>` values.
This is development governance evidence only; `classification.runtime_truth`
must be `false`.

Derived views:

```text
docs/harness/broadcast/TASK_BOARD.json
GitHub PRs and checks
WorkerReport text
MergeDecision evidence
```

These views are not canonical truth. The board is not a lock service. If a view
disagrees with DevTape, treat it as drift and regenerate or HOLD.

## New Session Intake

Start from the repo root:

```bash
cd /home/zephryj/projects/turingosv5
git status --short --branch
gh pr list --state open --json number,title,headRefName,isDraft,createdAt,url
```

Read in this order:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/roles/META_ENTRY.md`
4. `docs/agent_skills/KARPATHY_ARCHITECT.md`
5. `docs/harness/META_HARNESS.md`
6. `docs/harness/META_AI_MANUAL.md`
7. `docs/harness/META_META_AI_BOUNDARY.md`
8. `docs/harness/TASK_BROADCAST_POLICY.md`
9. `docs/v5_dev/CORE_DEV_FLOW.md`
10. `docs/harness/broadcast/TASK_BOARD.json`

If the worktree is dirty, first classify the dirt:

- user/manual edits: HOLD and ask before touching;
- MetaAI in-progress edits from this session: continue only if they match the
  current task;
- stale generated board drift: regenerate from DevTape after audit;
- WorkerAI task edits in main checkout: HOLD, because workers must use isolated
  worktrees.

## Find Current Progress

Use DevTape first:

```bash
STORE=.turingos_system/devtape/turingosv5/events.jsonl
test -f "$STORE" && tail -n 5 "$STORE" || true
turingos-dev board derive --store "$STORE" --out /tmp/turingosv5-board.json
turingos-dev audit --store "$STORE" --board docs/harness/broadcast/TASK_BOARD.json
```

Interpretation:

- no store file: kernel-driven mode is not bootstrapped yet; current board is
  bootstrap harness state, not DevTape truth;
- derive succeeds and audit passes: board is a faithful projection;
- derive succeeds and audit fails: board drift; do not dispatch workers from the
  drifted board;
- derive fails: HOLD and inspect the last valid record hash.

Then check GitHub evidence:

```bash
gh pr list --state open --json number,title,headRefName,isDraft,createdAt,url,mergeStateStatus,reviewDecision
```

This is the open PR claims scan.

If `turingos-dev` is not installed on `PATH`, use the cargo target form:

```bash
cargo run --bin turingos-dev -- <args>
```

Claim facts come from draft PRs whose title follows:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

The earliest valid `createdAt` claim owns the atom. Later duplicates are
superseded evidence.

## Publishing Work

Default distribution is board-first:

```text
Human spec -> MetaAI DevEvents -> derived board -> WorkerAI self-selects
```

MetaAI must not hand private worker-specific execution instructions to a
WorkerAI unless the Human Architect explicitly requests direct assignment or
continuation.

To publish a task in kernel-driven mode:

1. append `HumanIntentReceived` for the human instruction that creates or
   reconciles the work;
2. create a `DevTaskCreated` payload;
3. append it with `turingos-dev event append`;
4. append `TaskBroadcasted` for the same `atom_id`;
5. derive the board from DevTape;
6. audit the derived board;
7. run tests before PR.

The board row must be explainable from `source_event_cids`. A task that exists
only as a hand edit to `TASK_BOARD.json` is not accepted development state.

## Worker / PR Reconciliation

The minimal autonomous entrypoint is:

```bash
cargo run --bin turingos-dev -- meta run \
  --store .turingos_system/devtape/turingosv5/events.jsonl \
  --board-out docs/harness/broadcast/TASK_BOARD.json \
  --iterations 1 \
  --interval-ms 0 \
  --meta-adapter deepseek
```

Use `--iterations <n>` with a positive integer for a controlled polling run.
This is not a daemon yet; it is a repeatable one-process reconcile loop. If the
DeepSeek adapter fails, the provider error is recorded as candidate evidence and
the deterministic reconcile loop still runs.

When a WorkerAI opens a draft claim PR, MetaAI records:

```text
TaskClaimed {
  atom_id
  pr_number
  claim_pr_url
  worker_identity
  board_hash
  task_packet_hash
  createdAt
}
```

When WorkerReport appears, MetaAI records:

```text
WorkerReportSubmitted {
  atom_id
  pr_number
  files_changed
  tests_run
  forbidden_files_touched
  class4_touched
  worker_halt_confirmation: "[WORKER_HALT]"
}
```

Rules:

- PR without `TaskClaimed` is orphan.
- PR without `WorkerReportSubmitted` cannot merge.
- WorkerReport without `[WORKER_HALT]` is incomplete.
- Dirty PR merge state becomes `SUPERSEDE`; do not rebase or manually resolve
  inside the same candidate.
- Ready PRs can produce deterministic gate evidence:
  `AuditVerdictSubmitted`, `VetoVerdictSubmitted`, and
  `MergeDecisionRecorded`.
- Deterministic audit only checks mechanical PR facts available to MetaAI:
  changed files, allowed/forbidden files, CI state, review/branch state, and
  WorkerReport presence. It does not replace human Class 4 ratification or
  product-quality review.

## Merge Check

Before merge, run:

```bash
turingos-dev merge check --store .turingos_system/devtape/turingosv5/events.jsonl --pr <number>
gh pr checks <number>
gh pr view <number> --json mergeStateStatus,reviewDecision,statusCheckRollup
```

`MergeDecisionRecorded(PROCEED)` is necessary but not sufficient. GitHub branch
protection, CI, review, conversation resolution, forbidden-file checks,
independent audit, Veto when required, and Class 4 ratification still apply.

`MergeDecisionRecorded(HOLD)` is a valid accepted observation. It means the PR
has enough evidence to explain why it cannot merge yet.

Never merge when:

- required CI is failed or pending;
- `mergeStateStatus` is dirty;
- review is required but missing;
- conversations are unresolved;
- author is final auditor;
- forbidden files were touched;
- Class 4 was touched without exact Human Architect ratification;
- DevTape lacks the required evidence.

## Bootstrap And Recovery

If DevTape does not exist yet, do not pretend the board is kernel-driven. Use
the current board only as bootstrap harness state, then create DevEvents and
derive a replacement board.

If `.turingos_system/devtape/turingosv5/events.jsonl` is broken:

1. stop dispatching workers;
2. identify the first broken record;
3. preserve the broken file as local evidence outside git;
4. rebuild a new tape only from accepted events or explicit Human Architect
   recovery instruction.

If GitHub `main` contains a commit with no DevTape merge evidence, record or
plan `ExternalMutationDetected` and HOLD future merge automation until reconciled.

## MetaAI Self-Check

Before telling the Human Architect that development is moving:

```text
devtape_store_checked:
board_derived_or_bootstrap_labeled:
open_pr_claims_checked:
dirty_tree_classified:
next_task_source_event_cids:
merge_gate_status:
runtime_truth_boundary_preserved:
```
