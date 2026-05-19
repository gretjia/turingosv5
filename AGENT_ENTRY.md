# AGENT_ENTRY.md

You are inside TuringOS V5.

This file is the single entry point for autonomous CLI workers.

## Read Order

1. Read `AGENTS.md`.
2. Read `docs/harness/WORKER_HARNESS.md`.
3. Read `docs/harness/TASK_BROADCAST_POLICY.md`.
4. Read `docs/harness/broadcast/TASK_BOARD.json`.
5. Pick exactly one eligible open task.
6. Read that task's TaskPacket.
7. Create a branch/worktree for that task.
8. Implement only allowed files.
9. Run required tests.
10. Open a PR with WorkerReport.
11. Stop the current task.
12. Return to idle polling only after the PR is opened.

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

## Polling Loop

When idle:

```bash
git fetch origin
git switch main
git pull --ff-only
cat AGENT_ENTRY.md
cat AGENTS.md
cat docs/harness/WORKER_HARNESS.md
cat docs/harness/broadcast/TASK_BOARD.json
gh pr list --state open
```

Then choose the highest-priority eligible task.

## Eligibility

A task is eligible only if:

- `status == "open"`
- `self_select == true`
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

## PR Ends Current Task

After opening PR and submitting WorkerReport, stop the current task.

Do not keep editing unless Meta publishes a continuation or repair task.
