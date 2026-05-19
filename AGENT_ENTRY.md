# AGENT_ENTRY.md

You are inside TuringOS V5.

This file is the shared entry point for any CLI session.

At this point you are not assigned a role by this file. A role becomes active
only when the human prompt, TaskPacket, ReviewPacket, or Meta continuation
explicitly assigns it.

Start intake from the main checkout:

```bash
cd /home/zephryj/projects/turingosv5
```

The main checkout is for intake and control view. Task code edits happen only
in an isolated task worktree after the worker role is explicitly active.

## Read Order

1. Read `AGENTS.md`.
2. Read this file.
3. Identify whether an explicit role assignment exists.
4. If a role is assigned, read the matching role entry below.
5. If no role is assigned, stop after intake and ask for an assignment.

## Role Routing

CLI labels do not grant duties, capabilities, audit authority, or merge
authority.

- For explicitly assigned Meta work, read `docs/harness/roles/META_ENTRY.md`.
- For explicitly assigned worker work or task self-selection, read
  `docs/harness/roles/WORKER_ENTRY.md`.
- For explicitly assigned independent audit, read
  `docs/harness/roles/AUDITOR_ENTRY.md`.
- For explicitly assigned Veto work, read `docs/harness/roles/VETO_ENTRY.md`.

Worker role input includes `docs/harness/broadcast/TASK_BOARD.json`; the board
is not read as runtime truth or as a universal role assignment.

## Absolute Rules

- Worker output is Candidate.
- PR is Candidate.
- CI/Veto/Meta decide accepted state.
- Never push to main.
- Never merge PR.
- Never modify forbidden files.
- Never edit `TASK_BOARD.json`.
- Never add dependencies unless the task explicitly allows it.
- Never create new canonical substrate.
- Never write naked LLM calls.
- Never make UI/session/cache/dashboard canonical truth.
- Never edit shared contracts unless this is a Contract PR.
- Never self-select Class 4.
- Never let V5 runtime depend on V4 evidence/genesis/local paths.
- Never treat MiniF2F as a V5 product asset or default test corpus.

## Task Board Boundary

`docs/harness/broadcast/TASK_BOARD.json` is a development control plane.

It is not V5 runtime truth.

V5 product code must never read:

- `AGENT_ENTRY.md`
- `docs/harness/broadcast/**`
- `docs/harness/tasks/**`
- V4 `handover/evidence/**`

## Single-shot Smoke Lifecycle

H0 smoke worker role sessions run one task and then stop. Do not run a
`while true` worker loop, automatic re-entry, or background task scanner during
this phase.

## Worker Profile

`worker_slot` is declared by the CLI launch prompt. If absent, use
`worker-unknown-<timestamp>`.

Default profile:

- `allowed_class = 1`
- `capabilities = ["docs", "harness"]`

CLI adapter files are compatibility shims only. They do not grant capabilities,
duties, audit lanes, or merge authority. Task selection is controlled by
`required_capabilities`, `preferred_capabilities`, and explicit role
assignment.

## Draft PR Claim

Before claiming:

```bash
git fetch origin
jq . docs/harness/broadcast/TASK_BOARD.json
gh pr list --state open
```

Skip any atom with an active valid claim.

Claim branch:

```text
work/<atom_id>/<worker_slot>
```

Claim worktree:

```text
/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>
```

Create the worktree from latest `origin/main`, not from a local stale branch.
Open a draft PR with this title:

```text
[CLAIM][<atom_id>][ClassX] <task title>
```

The claim PR body must include ClaimRecord, board version/hash, TaskPacket
path/hash, allowed files, forbidden files, worker profile, and claim timestamp.

Do not open a separate implementation PR. Continue work in the same draft PR,
then run `gh pr ready` when the WorkerReport is complete.

The end of a worker task is:

1. Draft PR claimed.
2. Same PR marked ready with `gh pr ready`.
3. WorkerReport submitted.
4. `[WORKER_HALT]` printed.
5. Process stopped.

Continuation requires Meta to publish a continuation or repair TaskPacket.

## Eligibility

A task is eligible only if:

- `status == "open"`
- `self_select == true`
- `claim_required == true`
- `claim_method == "draft_pr"`
- `class <= worker_allowed_class`
- required capabilities match your worker profile
- blockers are empty
- no active PR/claim exists for the same atom, unless duplicate policy allows it
- task packet exists and validates

## Class Rules

- Class 0/1: open pool; duplicate work allowed; first valid PR wins.
- Class 2: soft lease required; open draft PR early.
- Class 3: only if `self_select == true` and `meta_opened == true`.
- Class 4: never self-select.
