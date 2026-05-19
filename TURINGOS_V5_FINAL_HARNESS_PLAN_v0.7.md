# TuringOS V5 Final Harness Plan v0.7

## Meta Governance x Universal Worker x Task Broadcast x PR/CI Constitutional Development

This file is the local highest guidance file for the V5 bootstrap requested on
2026-05-19.

Core mapping:

```text
V4 = development constitutional harness
V5 = clean product repo
AGENT_ENTRY.md = all CLI workers' unified entry
TASK_BOARD.json = Meta AI task broadcast market
TaskPacket = single-task boundary
WorkerReport = worker evidence output
PR = Candidate<StateTransition>
CI / Veto / Meta Review = Predicate Mesh
Merge = wtool accepted commit
Closed / rejected PR = audit trail / repair source
```

The controlling sentence:

```text
Worker can freely choose tasks, but cannot freely define rules.
```

## Verdict

```text
PASS

Use Task Broadcast: yes
Use worker self-selection: yes, class-gated
Use Meta AI review/merge: yes, never bypass PR/CI/Veto/branch protection
Use V4 as V5 development harness: yes
Use V5 clean repo: yes
Class 4 self-selection: never
Worker edits to TASK_BOARD: forbidden
Worker auto-next-task after PR: allowed only after current task ends and entry is re-read
```

## Global Invariants

1. Worker output is Candidate.
2. PR is Candidate.
3. CI / Veto / Meta decide acceptance.
4. `main` is accepted world line.
5. Closed / rejected PR is audit history.
6. V4 evidence rail records V5 development process.
7. V4 evidence is not V5 production truth.
8. V5 product code must never read V4 handover/evidence.
9. `TASK_BOARD.json` is development control plane, not runtime truth.
10. V5 runtime must never read `docs/harness/broadcast/**`.
11. UI/session/cache/dashboard are derived views only.
12. No naked LLM calls.
13. No new parallel substrate.
14. Accepted changes must have accepted path.
15. Rejected changes must have rejection evidence.
16. Class 4 is never self-selected.
17. Class 4 requires exact human ratification.
18. Worker cannot final-audit its own PR.
19. Meta AI cannot merge without CI/review/Veto gates.
20. Shared interfaces require Contract PR before implementation.

Additional V5 bootstrap ruling:

```text
MiniF2F is a V4 development/evaluation corpus, not a V5 product asset.
V5 must not carry experiments/minif2f_v4 as default package, test problem set,
or core CI path.
```

## Architecture

```text
Human Architect
  -> V4 Harness development evidence rail
  -> Codex Meta AI
  -> Task Broadcast / TaskPackets
  -> Universal CLI Workers and Reviewer Agents
  -> GitHub PR / CI / review / merge queue
  -> TuringOS V5 clean product repo
```

This development mechanism is CAK-shaped: LLM/worker produces candidate state
transitions; CI/Veto/Meta decide whether a candidate enters `main`.

## Meta Harness

Codex Meta AI maintains task broadcast, PR governance, review coordination, and
merge decisions.

Duties:

- maintain `AGENT_ENTRY.md`
- maintain `docs/harness/broadcast/TASK_BOARD.json`
- publish TaskPackets
- reconcile open PRs with task board
- detect duplicate claims
- retire/supersede tasks
- convert failed reviews into repair tasks
- inspect PR diffs, CI, tests, and WorkerReport
- request independent audit
- request Veto-AI where required
- merge only after gates pass
- record V4 development evidence

Meta must not:

- push directly to `main`
- bypass branch protection
- merge failed/pending required checks
- merge its own authored PR without independent audit
- let workers edit `TASK_BOARD.json`
- allow Class 4 self-selection
- treat `go` / `ok` / `continue` / `继续` / `可以` as Class 4 authorization
- let V5 runtime depend on V4 evidence/genesis/local paths
- accept WorkerReport without checking diff and CI

## Universal Worker Harness

Workers read one entry, pick one eligible task, work only inside the TaskPacket,
open a PR, submit WorkerReport, and stop.

Workers must not:

- push main
- merge PR
- modify `TASK_BOARD.json`
- modify forbidden files
- add dependencies unless explicitly allowed
- create new canonical substrate
- write naked LLM calls
- make UI/session/cache/dashboard truth
- modify shared contracts unless this is a Contract PR
- touch Class 4 surfaces without ratification
- reintroduce MiniF2F as V5 core/default test corpus

## V5 File Layout

```text
AGENT_ENTRY.md
AGENTS.md
CODEX.md
CLAUDE.md
GEMINI.md
CHARTER.md
HARNESS.md
GENESIS_PLAN.md
TURINGOS_V5_FINAL_HARNESS_PLAN_v0.7.md

docs/lineage/V4_SNAPSHOT.md
docs/harness/**
docs/harness/broadcast/TASK_BOARD.json
docs/harness/broadcast/tasks/*.task.json
docs/harness/schemas/*.schema.json
docs/harness/templates/*
docs/harness/boot_prompts/*
docs/contracts/README.md
docs/ci/CI_POLICY.md
docs/pr/PR_POLICY.md
docs/classes/RISK_CLASSES.md
.github/pull_request_template.md
.github/ISSUE_TEMPLATE/atom_task.md
.github/CODEOWNERS
.github/workflows/ci-basic.yml
.github/workflows/ci-constitution-light.yml
.github/workflows/ci-web.yml
.github/workflows/ci-evidence.yml
```

## Task Board Top-Level Shape

```json
{
  "board_version": "v0.7",
  "generated_at": "2026-05-19T00:00:00Z",
  "generated_by": "codex-meta",
  "source": "V4 harness",
  "board_writer": "meta-only",
  "runtime_boundary": {
    "development_control_plane_only": true,
    "v5_runtime_must_not_read": [
      "AGENT_ENTRY.md",
      "docs/harness/broadcast/**",
      "handover/evidence/**"
    ]
  },
  "default_duplicate_policy": "first_valid_pr_wins",
  "tasks": []
}
```

Board rules:

- Task board is Meta broadcast development control plane.
- Task board is not V5 runtime truth.
- Workers can read the board.
- Workers cannot modify the board.
- V5 runtime cannot read the board.
- PR / CI / `main` are the accepted development facts.

## Task State Machine

```text
open
claimed
pr_open
needs_repair
merged
superseded
blocked
retired
```

Workers do not directly mutate board state. Claims are expressed by PR:

```text
[CLAIM][ATOM][ClassX] Title
```

## Class and Claim Rules

Class 0/1:

```text
claim_mode = open_pool
duplicate_policy = first_valid_pr_wins
```

Class 2:

```text
claim_mode = soft_lease
draft PR claim required/preferred
```

Class 3:

```text
self_select only if meta_opened == true
draft PR early
independent audit
Veto review
```

Class 4:

```text
self_select = false
direct assignment only
exact human ratification
broad tests
Veto PASS
```

## Worker Task Selection Algorithm

1. Priority: P0 > P1 > P2 > P3.
2. Lower class first unless worker is QA and task explicitly requires QA.
3. Required capability match.
4. Blockers empty.
5. Skip active claim unless duplicate policy allows.
6. Prefer smaller allowed file surface.
7. Prefer shorter targeted tests.
8. Older task first.

Workers do not choose based on "interestingness."

## Worktree Strategy

```text
/home/zephryj/projects/turingosv5
/home/zephryj/projects/turingosv5-claude
/home/zephryj/projects/turingosv5-gemini
/home/zephryj/projects/turingosv5-codex-worker
```

One worker, one branch, one worktree. Meta may inspect all worktrees but must
not let workers mix directories.

## Worker Profiles

Claude:

```json
{
  "allowed_class": 2,
  "capabilities": ["docs", "contracts", "ux", "markdown", "long-context", "rust-read"],
  "can_merge": false
}
```

Gemini:

```json
{
  "allowed_class": 3,
  "capabilities": ["qa", "audit", "negative-tests", "ci", "risk-review", "adversarial"],
  "can_merge": false
}
```

Codex Worker:

```json
{
  "allowed_class": 2,
  "capabilities": ["rust", "web", "tests", "ci", "implementation"],
  "can_merge": false
}
```

Codex Meta:

```json
{
  "allowed_class": 3,
  "capabilities": ["orchestration", "review", "merge-decision", "broadcast", "task-generation"],
  "can_merge": true,
  "can_ratify_class4": false
}
```

## Command Safety

Allowed by default: `git status`, `git diff`, `git diff --check`, `git log`,
`git show`, `git branch`, `git switch assigned-branch`, `rg`, read-only
`cat`/`sed`/`awk`, `cargo check`, `cargo test`, `cargo fmt --check`,
`gh pr view`, `gh pr checks`, `gh pr list`.

Conditional: assigned-branch `git push`, assigned-branch `gh pr create`,
Dependency PR `cargo update`, Contract PR shared-contract edits, CI/harness PR
CI edits.

Forbidden by default: `git push origin main`, `git reset --hard`,
`git clean -fdx`, `gh pr merge`, `sudo`, `curl | sh`, `wget | sh`, secret
printing, unratified `constitution.md` or `genesis_payload.toml` edits,
worker edits to `TASK_BOARD.json`.

## Context and Hidden Gates

Workers get WorkerContextPack, not the entire hidden oracle. Hidden gates are run
by Meta/QA. Failure feedback should disclose category and visible repair hint,
not hidden scoring logic.

## Required Formats

TaskPacket, WorkerReport, ReviewPacket, MergeDecision, VetoVerdict, and
BroadcastSnapshot live under `docs/harness/schemas/` and
`docs/harness/templates/`.

## PR Rules

PR titles:

```text
[CLAIM][V5-C0-CONTRACT-ARTIFACT-001][Class2] Define ArtifactManifest contract
[V5-C0][Class2][Contract] Define ArtifactManifest contract
[REPAIR][V5-REPAIR-V5-C0-CONTRACT-ARTIFACT-001-001][Class2] Fix invariant
[BROADCAST][Class0] Update task board 2026-05-19 AM
```

PRs must include task broadcast metadata, class/lane, allowed file list,
forbidden-file confirmation, tests, WorkerReport, risk review, and "PR ends
current task" acknowledgement.

## Veto-AI

Veto-AI output is `PASS | VETO`. It checks constitutionality only: parallel
truth, naked LLM, direct world mutation, UI/cache truth drift, accepted/rejected
path gaps, hidden oracle leak, prompt/credential leak, V5 depending on V4
runtime evidence, Class 4 without ratification, constitution mutation without
sudo, and contract drift outside Contract PR.

## Bootstrap Phases

V4-H0: harness refactor and dummy broadcast dry run.

V5-R0: clean bootstrap with PR/CI/harness; no product features.

V5-C0: contract freeze.

V5-M1: evidence-backed product skeleton.

V5-M2: foolproof MVP.

V5-M3: audited delivery.

## Immediate Queue

1. Add `docs/harness/README.md`.
2. Add `META_HARNESS.md`.
3. Add `WORKER_HARNESS.md`.
4. Add `TASK_BROADCAST_POLICY.md`.
5. Add `AGENT_ENTRY.md`.
6. Add task board schema.
7. Add task packet schema.
8. Add worker report schema.
9. Add merge decision schema.
10. Add `harness_task_board` gate.
11. Add runtime-does-not-read-agent-broadcast gate.
12. Run dummy broadcast dry run.

## Final Principle

Do not let three CLI agents freely write code. Let them enter a task broadcast
market where constitution, TaskPacket, PR, CI, Veto, and Meta AI constrain them
into candidate state transitions.
