# Task Board

Canonical machine-readable board:

```text
docs/harness/broadcast/TASK_BOARD.json
```

This Markdown file is a derived human view only. Workers must read the JSON
board and then the referenced TaskPacket.

## H0 Open Smoke Tasks

- `V5-H0-HARNESS-JSON-001`: JSON/schema/QA hardening.
- `V5-H0-HARNESS-SINGLESHOT-001`: single-shot lifecycle and `[WORKER_HALT]`.
- `V5-H0-HARNESS-REPAIR-001`: repair fuse and dirty conflict quarantine.

## Retired R0 Tasks

- `V5-R0-DOCS-001`
- `V5-R0-HARNESS-001`
- `V5-R0-QA-001`

## Guards

- `max_repair_attempts: 3`
- `worker_halt_required: true`
- `conflict_policy: "supersede_on_dirty"`
