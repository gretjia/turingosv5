# AGENTS.md

You are operating inside TuringOS V5.

Default user-facing language is Chinese. Technical terms may remain in English.

## Read Order

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. Role entry under `docs/harness/roles/**` only after explicit assignment
4. TaskPacket or ReviewPacket
5. Relevant contract
6. Role-specific harness document

## Global Invariants

- Worker output is Candidate.
- PR is Candidate.
- CI/Veto/Meta decide whether PR becomes accepted state.
- No direct main push.
- No naked LLM call.
- No parallel substrate.
- No UI/session/cache/dashboard as truth.
- Accepted changes must have accepted path.
- Rejected changes must have rejection evidence.
- V5 runtime must not depend on V4 evidence/genesis/local path.
- Class 4 requires exact human ratification.

## Forbidden Unless Explicitly Assigned

- `constitution.md`
- `genesis_payload.toml`
- `src/state/**`
- `src/bottom_white/**`
- `src/runtime/**`
- `src/cas.rs`
- `src/hash.rs`
- `src/versioned_state.rs`
- `docs/harness/broadcast/TASK_BOARD.json`
- RMCP
- Wasmtime
- gix migration
- Next.js
- Tauri
- new dependency

## Worker Boundary

Workers read `AGENT_ENTRY.md`, choose exactly one eligible task from
`docs/harness/broadcast/TASK_BOARD.json` only after the worker role is active,
implement only the allowed files in that TaskPacket, open a PR, submit
WorkerReport, then stop that task.

Workers must not edit `TASK_BOARD.json`. The board is Meta-only development
control plane, not V5 runtime truth.

## Meta Boundary

Meta maintains the task board and TaskPackets, reconciles PRs with claims,
coordinates CI/review/Veto, records development evidence, and merges only after
required gates pass. Meta may not ratify Class 4 and may not merge around
branch protection.

## Risk Classes

- Class 0: docs, plans, charters, non-authoritative handoff.
- Class 1: additive harness docs, schemas, templates, isolated non-runtime gates.
- Class 2: production wire-up, CI, replay/verifier/userland implementation.
- Class 3: auth, money, CAS integrity, production evidence, capability gates.
- Class 4: constitution, genesis/trust root, sequencer admission, typed tx wire
  schema, canonical signing payload, kernel authority.

Class 4 is never self-selected.

## Runtime Boundary

V5 product/runtime code must not read:

- `AGENT_ENTRY.md`
- `docs/harness/broadcast/**`
- V4 `handover/evidence/**` as runtime truth

Docs and harness tests may reference those paths only to enforce the boundary.

## MiniF2F Boundary

MiniF2F is a V4 development/evaluation corpus, not a V5 product asset. V5 must
not carry `experiments/minif2f_v4` as a default package, test problem set, or
core CI path.
