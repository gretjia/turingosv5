# Worker Entry

Use this role entry only when explicitly assigned worker work by the human
prompt, TaskPacket, or Meta continuation. During H0 smoke, a worker role session
may self-select exactly one eligible open task from the board.

For local maintainer-side workers, begin intake from the main checkout:

```bash
cd /home/zephryj/projects/turingosv5
```

For external GitHub-only workers that cannot access the maintainer checkout,
use `docs/harness/boot_prompts/remote_worker_market.md`. External workers still
self-select from the public board; they are not assigned a single task package
by MetaAI.

Read in order:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/roles/WORKER_ENTRY.md`
4. `docs/harness/WORKER_HARNESS.md`
5. `docs/harness/TASK_BROADCAST_POLICY.md`
6. `docs/harness/broadcast/TASK_BOARD.json`
7. The selected TaskPacket or generated sandbox `TASK.md`

## Worker Profile

`worker_slot` is declared by the CLI launch prompt. If absent, use
`worker-unknown-<timestamp>`.

If the launch prompt does not declare capabilities, use the neutral
`default_worker_profile` published in `docs/harness/broadcast/TASK_BOARD.json`.
It is a smoke harness profile, not tied to any CLI label.

## Claim Before Code

Workers must claim before implementation. The primary claim path is now
DevTape-backed sandbox intake:

```bash
turingos-dev worker claim next \
  --store .turingos_system/devtape/turingosv5/events.jsonl \
  --repo /home/zephryj/projects/turingosv5 \
  --out-root /home/zephryj/projects/turingosv5-sandboxes \
  --worker <worker_slot>
```

This appends `TaskClaimed` evidence and creates a soft sandbox. If no eligible
task exists, stop with `[WORKER_HALT]`.

Legacy draft PR claims remain a compatibility fallback only when a TaskPacket or
Meta continuation explicitly says `claim_method: "draft_pr"`.

## Single-Shot Lifecycle

H0 smoke worker role sessions run one task and then stop. Do not run a
`while true` worker loop, automatic re-entry, or background task scanner during
this phase.

Primary sandbox task edits happen only through the generated sandbox:

```text
/home/zephryj/projects/turingosv5-sandboxes/<worker_slot>/<atom_id>
```

Read only the sandbox files and write:

```text
submit/candidate.patch
submit/WorkerReport.json
```

Then submit through TuringOS:

```bash
turingos-dev worker sandbox submit \
  --dir /home/zephryj/projects/turingosv5-sandboxes/<worker_slot>/<atom_id> \
  --store .turingos_system/devtape/turingosv5/events.jsonl \
  --repo /home/zephryj/projects/turingosv5 \
  --worktree-root /home/zephryj/projects/turingosv5-worktrees \
  --worker <worker_slot> \
  --create-pr
```

`sandbox submit` validates allowed files, requires `[WORKER_HALT]`, applies the
patch in an isolated worktree, runs local acceptance gates, commits the result,
opens the external GitHub backup PR for the current development wave, and
records `WorkerReportSubmitted`. Omit `--create-pr` only for a local dry-run
where no PR should be created.

Legacy draft PR fallback uses branch:

```text
work/<atom_id>/<worker_slot>
```

and PR title:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

Before legacy draft PR claiming, run `git fetch origin`, validate the board, and
check open PRs. Skip any atom with an active valid claim.

For legacy draft PR fallback, use a two-check claim sequence:

1. Check open PRs before creating the worktree. If any valid claim already
   exists for the atom, skip it.
2. Create the branch and worktree from latest `origin/main`.
3. Before making any implementation edit, check open PRs for the same atom
   again.
4. If another valid claim appeared, stop immediately and output
   `[WORKER_HALT]`.
5. Open the draft claim PR before implementation work.
6. After the draft PR exists, refresh open PRs. If an earlier valid claim exists
   by `createdAt`, mark the current PR as duplicate/superseded when possible,
   output `[WORKER_HALT]`, and stop.

Only the earliest valid claim may proceed to implementation. Duplicate claims
are evidence; they are not accepted state and must not continue coding.

The claim PR body must include ClaimRecord, board version/hash, TaskPacket
path/hash, allowed files, forbidden files, worker profile, and claim timestamp.
Do not open a separate implementation PR.

For sandbox intake, do not hand-edit the board or main checkout. Submit the
sandbox, print `[WORKER_HALT]`, and stop. H0 smoke is single-shot; do not start
another task without a new explicit assignment.

## Eligibility

A task is eligible only if:

- `status == "open"`
- `self_select == true`
- `claim_required == true`
- `claim_method` is compatible with sandbox intake, or explicitly declares
  legacy `draft_pr` fallback
- `class <= worker_allowed_class`
- required capabilities match the worker profile
- blockers are empty
- no active PR/claim exists for the same atom, unless duplicate policy allows it
- task packet exists and validates

## Class Rules

- Class 0/1: open pool; duplicate work allowed; first valid PR wins.
- Class 2: soft lease required; open draft PR early.
- Class 3: only if `self_select == true` and `meta_opened == true`.
- Class 4: never self-select.
