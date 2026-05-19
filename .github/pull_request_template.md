# PR

## Task Broadcast

- Atom ID:
- Task revision:
- TaskPacket:
- TaskPacket SHA-256:
- Board version:
- Board SHA-256:
- Claim mode:
- Duplicate policy:
- Worker slot:
- Worker profile:

## ClaimRecord

- Claim required:
- Claim method:
- Claim PR:
- Claim branch:
- Claim worktree:
- Claim timestamp:

## Class / Lane

- Phase:
- Lane:
- Class:

## Scope

### Allowed files changed

- ...

### Forbidden files confirmation

I did not touch:

- `constitution.md`
- `genesis_payload.toml`
- `docs/harness/broadcast/TASK_BOARD.json` unless this is a Meta broadcast PR
- `src/state/**`
- `src/bottom_white/**`
- `src/runtime/**`
- V4 handover/evidence as V5 runtime truth
- MiniF2F as a V5 product/core test dependency

## Tests

```bash
...
```

## WorkerReport

Paste or link WorkerReport.

WorkerReport must include `WORKER_HALT_CONFIRMATION: [WORKER_HALT]`.
WorkerReport must also include `CLAIM_PR_URL`, `READY_PR_URL`, and `WORKTREE`.

## Risk Review

- Naked LLM call risk:
- Parallel substrate risk:
- UI source-of-truth risk:
- Contract drift risk:
- L4/L4.E risk:
- Hidden oracle leak risk:
- Class 4 risk:

## Task Completion

After this PR is opened and WorkerReport is submitted, output `[WORKER_HALT]`
and stop unless Meta publishes a continuation or repair task.
