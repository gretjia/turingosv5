# Worker Entry

Use this role entry only when explicitly assigned worker work by the human
prompt, TaskPacket, or Meta continuation. During H0 smoke, a worker role session
may self-select exactly one eligible open task from the board.

Begin intake from the main checkout:

```bash
cd /home/zephryj/projects/turingosv5
```

Read in order:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/roles/WORKER_ENTRY.md`
4. `docs/harness/WORKER_HARNESS.md`
5. `docs/harness/TASK_BROADCAST_POLICY.md`
6. `docs/harness/broadcast/TASK_BOARD.json`
7. The selected TaskPacket

## Worker Profile

`worker_slot` is declared by the CLI launch prompt. If absent, use
`worker-unknown-<timestamp>`.

If the launch prompt does not declare capabilities, use the neutral
`default_worker_profile` published in `docs/harness/broadcast/TASK_BOARD.json`.
It is a smoke harness profile, not tied to any CLI label.

## Claim Before Code

Workers must claim before implementation. The first durable public signal for a
task is the draft PR claim, not local edits, local branches, or private notes.

If a WorkerAI cannot create a draft PR claim, it must not implement the task.

## Single-Shot Lifecycle

H0 smoke worker role sessions run one task and then stop. Do not run a
`while true` worker loop, automatic re-entry, or background task scanner during
this phase.

Task code must be edited only in:

```text
/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>
```

Create the task branch from latest `origin/main`:

```text
work/<atom_id>/<worker_slot>
```

Claim by draft PR titled:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

Before claiming:

```bash
git fetch origin
jq . docs/harness/broadcast/TASK_BOARD.json
gh pr list --state open
```

Skip any atom with an active valid claim.

For the selected atom, use a two-check claim sequence:

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

Run required tests, update the same PR with WorkerReport, run `gh pr ready`,
print `[WORKER_HALT]`, and stop. H0 smoke is single-shot; do not start another
task without a new explicit assignment.

## Eligibility

A task is eligible only if:

- `status == "open"`
- `self_select == true`
- `claim_required == true`
- `claim_method == "draft_pr"`
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
