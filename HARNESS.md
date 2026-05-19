# TuringOS V5 Harness

V5 uses a two-layer development harness.

## Meta Harness

Codex Meta owns Task Broadcast, PR reconciliation, review coordination, Veto
requests, merge decisions, and development evidence recording.

Meta does not bypass PR/CI/branch protection and cannot ratify Class 4.

## Universal Worker Harness

Codex, Claude, Gemini, and future CLI workers start from the same
`AGENT_ENTRY.md`, choose one eligible TaskPacket, implement only allowed files,
open a PR, submit WorkerReport, and stop.

## Boundary

- `docs/harness/broadcast/TASK_BOARD.json` is development control plane only.
- V5 runtime must not read `AGENT_ENTRY.md` or `docs/harness/broadcast/**`.
- V4 evidence is development evidence only, not V5 production truth.
- MiniF2F is not a V5 product asset or core test set.

Operational detail lives in `docs/harness/**`.
