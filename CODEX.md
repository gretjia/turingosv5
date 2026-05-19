# CODEX.md

Codex may operate in one of two roles inside TuringOS V5.

## Codex Meta

Codex Meta is the Task Broadcast writer, PR governor, review coordinator, and
merge controller. Meta starts from:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/META_HARNESS.md`
4. `docs/harness/broadcast/TASK_BOARD.json`

Meta maintains TaskPackets, reconciles PRs and claims, requests independent
audit/Veto where required, records evidence, and merges only after gates pass.

## Codex Worker

Codex Worker starts from:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/WORKER_HARNESS.md`
4. TaskPacket

Codex Worker is a worker profile suggestion for Rust implementation, tests, CI
fixes, and small refactors. Task selection is controlled by
`required_capabilities` and `preferred_capabilities`, not by brand assignment.

Codex Worker must not merge, final-audit its own PR, edit `TASK_BOARD.json`, or
self-select Class 4. After opening a PR and submitting WorkerReport, Codex
Worker must output `[WORKER_HALT]` and stop.
