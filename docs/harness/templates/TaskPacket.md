# Task: <ATOM_ID>

## Goal

<goal>

## Capability Match

Required:

- <capability>

Preferred:

- <capability>

## Required Skill

- `docs/agent_skills/KARPATHY_SIMPLE_CODE.md`

## Implementation Style

- Follow Karpathy Simple Code.
- Prefer direct input -> transform -> output data flow.
- No new dependency unless this TaskPacket explicitly allows it.
- No broad abstraction unless this TaskPacket explicitly requires it.
- No neighboring refactor outside the allowed files and goal.
- No clever compression that hurts readability.

## Claim

- `claim_required: true`
- `claim_method: "draft_pr"`
- branch: `work/<atom_id>/<worker_slot>`
- worktree: `/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>`
- PR title: `[CLAIM][<atom_id>][ClassX] <task title>`
- claim before code: open the draft PR claim before implementation edits
- duplicate race rule: earliest valid draft PR by `createdAt` owns the atom

## Allowed Files

- <path>

## Forbidden Files

- `constitution.md`
- `genesis_payload.toml`
- `docs/harness/broadcast/TASK_BOARD.json`

## Acceptance Tests

```bash
git diff --check
```

## Harness Guards

- `worker_halt_required: true`
- `max_repair_attempts: 3`
- `conflict_policy: "supersede_on_dirty"`

## Non-goals

- Do not modify runtime code unless explicitly allowed.
