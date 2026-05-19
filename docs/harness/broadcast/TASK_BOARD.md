# Task Board

Canonical machine-readable board:

```text
docs/harness/broadcast/TASK_BOARD.json
```

This Markdown file is a derived human view only. Workers must read the JSON
board and then the referenced TaskPacket.

## DevTape v1.0 Open Tasks

- `V5-SYS-A0-ARCH-PIN-001`: Meta-only architecture pin.
- `V5-SYS-A1-BASELINE-SEMANTIC-CLOSE-001`: baseline and semantic close.
- `V5-SYS-A2-WORKBENCH-BOUNDARY-001`: local workbench boundary.
- `V5-SYS-A3-CHARACTERIZATION-SEEDS-001`: characterization seed fixtures.
- `V5-SYS-A4-MICRO-DEVTAPE-001`: micro DevTape proof.
- `V5-SYS-A5-DERIVED-BOARD-PROJECTOR-001`: derived board projector.
- `V5-SYS-A6-CLI-INTAKE-WRAPPER-001`: CLI intake wrapper.
- `V5-SYS-A7-REUSE-PORT-CONTRACT-001`: minimal reuse port contract.
- `V5-SYS-A8-GITHUB-EVIDENCE-SNAPSHOT-001`: GitHub evidence payload.
- `V5-SYS-A9-ACTIVE-MERGE-GATE-001`: active merge gate check.
- `V5-SYS-A10-HARNESS-THINNING-001`: harness thinning.
- `V5-SYS-A11-AUDIT-VETO-001`: final read-only audit and veto.

## Retired R0 Tasks

- `V5-R0-DOCS-001`
- `V5-R0-HARNESS-001`
- `V5-R0-QA-001`
- `V5-H0-HARNESS-JSON-001`
- `V5-H0-HARNESS-SINGLESHOT-001`
- `V5-H0-HARNESS-REPAIR-001`

## Guards

- `max_repair_attempts: 3`
- `worker_halt_required: true`
- `conflict_policy: "supersede_on_dirty"`
