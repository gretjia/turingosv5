# TuringOS V5 Harness

V5 uses a two-layer development harness.

## Role Entries

All CLI sessions start at `AGENT_ENTRY.md` without a role. Explicit role
assignment then routes to:

- `docs/harness/roles/META_ENTRY.md`
- `docs/harness/roles/WORKER_ENTRY.md`
- `docs/harness/roles/AUDITOR_ENTRY.md`
- `docs/harness/roles/VETO_ENTRY.md`

## Meta Harness

Assigned Meta owns Task Broadcast, PR reconciliation, review coordination, Veto
requests, merge decisions, and development evidence recording.

Meta does not bypass PR/CI/branch protection and cannot ratify Class 4.

## Universal Worker Harness

All CLI sessions start from the same `AGENT_ENTRY.md`, choose one eligible
TaskPacket, implement only allowed files, open a PR, submit WorkerReport, and
stop.

## Boundary

- `docs/harness/broadcast/TASK_BOARD.json` is development control plane only.
- V5 runtime must not read `AGENT_ENTRY.md` or `docs/harness/broadcast/**`.
- V4 evidence is development evidence only, not V5 production truth.
- MiniF2F is not a V5 product asset or core test set.

Operational detail lives in `docs/harness/**`.
