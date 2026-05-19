# V5 DevKernel Architecture Plan v0.8

Status: Canonical local development plan for V4-governed V5 development.

Scope: development kernel only. This document does not define V5 product runtime
truth, typed transaction wire schema, canonical signing payload, trust root, or
sequencer admission. Those are Class 4 or production-runtime decisions and need
the accepted path required by the harness.

## Purpose

V5 development begins under V4 governance while avoiding V4 as V5 runtime truth.
The DevKernel records how agents are assigned, how work events are emitted, how
candidate changes are judged, and how development evidence is retained until an
accepted V5 substrate exists.

The canonical development rule is:

- Workers produce Candidate state.
- PRs are Candidate state.
- CI, Veto, and Meta decide whether a Candidate can become accepted
  development state.
- No V5 runtime code may depend on V4 local paths, V4 genesis, V4 handover
  evidence, dashboards, caches, or sessions.

## Architecture Overview

The DevKernel is a narrow development control plane made of five surfaces:

1. Human assignment: explicit role and task authorization.
2. Agent identity: stable worker identity plus role assignment evidence.
3. DevEventEnvelope: append-only development event records.
4. Candidate evidence: PR, tests, reports, review, veto, and merge decision.
5. Bootstrap exception ledger: temporary V4-governed evidence accepted only for
   development bootstrapping.

None of these surfaces is V5 runtime truth. They are scaffolding that lets V5
reach its own accepted substrate without pretending it already exists.

## DevEventEnvelope

Every material development event SHOULD be representable as a
`DevEventEnvelope`. The envelope is a development evidence schema, not a V5
runtime transaction.

Required fields:

```text
event_id: deterministic content hash or accepted event identifier
event_type: DevTaskCreated, WorkerReportSubmitted, PRCreated, CIResultRecorded,
  ReviewVerdictSubmitted, VetoVerdictSubmitted, MergeDecisionAccepted,
  MergeDecisionRejected, PRMerged, or another accepted DevKernel event type
project_id: "turingosv5"
actor_identity_cid: AgentIdentity or HumanArchitect evidence CID
payload_cid: event-specific payload CID
previous_event_cid: previous development event CID or null for the first event
observed_at: RFC3339 timestamp
source: human_prompt, local_cli, github, github_actions, or v4_devkernel
subject:
  repo: repository name or absolute checkout path for development evidence
  branch: branch name when applicable
  pr: PR id or URL when applicable
  files: declared affected files
payload: event-specific structured object
evidence:
  commands: read-only or validation commands that produced the event
  artifacts: paths, PR URLs, logs, hashes, reports, or review IDs
  source_anchors: code/test/doc anchors used for claims
classification:
  risk_class: Class 0, 1, 2, 3, or 4
  candidate: true unless accepted by Meta after gates
  runtime_truth: false
integrity:
  parent_event_ids: prior development events this depends on
  payload_hash: canonical hash of payload
  envelope_hash: canonical hash of the envelope excluding envelope_hash
```

Event kinds:

- `role_assigned`: human or accepted board evidence activates a role.
- `task_claimed`: worker claim against an eligible task.
- `candidate_changed`: file changes in allowed scope.
- `tests_run`: commands and results used as verification evidence.
- `review_submitted`: audit or reviewer evidence.
- `veto_submitted`: Veto verdict and reason.
- `meta_decision`: accepted/rejected decision, including gate evidence.
- `bootstrap_exception`: temporary exception under the policy below.

## AgentIdentity and Role Assignment Evidence

An agent identity is not inferred from a CLI label. It is bound by evidence.

Required fields:

```text
schema_id: "turingos.v5.agent_identity.v0.8"
agent_id: stable session or worker identifier
worker_slot: worker slot declared by launch prompt or neutral fallback
declared_capabilities: capabilities from the assignment source
active_role: Worker, Meta, Auditor, Veto, or none
role_assignment_evidence:
  source_kind: human_prompt, task_packet, review_packet, meta_continuation,
    accepted_board, or explicit_role_file
  source_ref: path, PR URL, prompt excerpt hash, or packet id
  assigned_at: RFC3339 timestamp or session intake timestamp
  allowed_files: exact list for the task, if worker
  forbidden_files: exact list or inherited harness rule
  risk_class_limit: maximum allowed class
  evidence_hash: hash of the assignment source where available
```

Rules:

- A role becomes active only after explicit assignment evidence exists.
- Workers must edit only files allowed by the active task.
- Workers must not edit task board files, role entries, schemas, runtime code, or
  any forbidden path unless explicitly assigned.
- Class 4 cannot be self-selected and requires exact human ratification.

## BootstrapException Policy

V5 needs a temporary bridge while its native accepted substrate is incomplete.
That bridge is a BootstrapException, not runtime truth.

Allowed bootstrap exceptions:

- V4-governed development evidence may be cited to justify V5 harness design.
- V4 code and tests may be read to produce a Reality Map.
- V4 patterns may inform adapters, replay/verifier plans, and acceptance gates.

Forbidden bootstrap uses:

- V5 product/runtime code must not read V4 `handover/evidence/**`,
  `genesis_payload.toml`, local V4 paths, sessions, caches, dashboards, or V4
  runtime state as truth.
- V4 ChainTape/CAS may not become V5 ChainTape/CAS by import.
- V4 trust-root, signing payload, typed transaction schema, or sequencer
  admission may not be copied into V5 without Class 4 ratification.
- MiniF2F V4 development/evaluation corpus must not become a V5 default package,
  test problem set, or core CI path.

Exception record requirements:

- reason for the exception
- exact V4 anchors read
- V5 files affected
- risk class
- expiration condition
- replacement path to native V5 evidence
- Meta decision that accepts or rejects the exception

Default expiration:

- A BootstrapException expires when an accepted V5-native schema, verifier,
  replay path, or runtime truth surface covers the same need.

## V4D Phases

V4D means V4-governed development, not V4-dependent runtime.

### V4D-0 Intake and Map

Goal: document V4 reality without importing V4 truth.

Outputs:

- V4 Native Reality Map.
- DevKernel v0.8 plan.
- Explicit assumptions list.

Acceptance:

- Each `usable_now` item has both a code anchor and a test anchor.
- Items without anchors are marked assumption, not fact.

### V4D-1 Candidate Harness

Goal: establish development evidence flow.

Outputs:

- WorkerReport, ClaimRecord, ReviewPacket, VetoVerdict, and MergeDecision usage.
- DevEventEnvelope draft records for claims, changes, tests, reviews, vetoes,
  and Meta decisions.

Acceptance:

- Candidate state is never treated as accepted state.
- Rejected changes have rejection evidence.
- Accepted changes have an accepted path.

### V4D-2 Adapter and Replay Prototypes

Goal: prototype V5-native interfaces around V4-proven concepts without runtime
dependency on V4 files.

Outputs:

- Adapter specs for ChainTape-like event log, CAS-like object store,
  rejection-evidence lane, and HEAD_t-like witness.
- Replay/verifier tests that use V5 fixtures, not V4 local state.

Acceptance:

- No V5 runtime reads V4 paths.
- Adapter fixtures are explicit and checked in only through accepted paths.

### V4D-3 V5 Native Substrate

Goal: replace development exceptions with accepted V5-native evidence.

Outputs:

- V5-native schemas and verifiers through proper risk-class gates.
- CI gates that prove no V4 runtime dependency exists.
- Migration notes that retire bootstrap exceptions.

Acceptance:

- V5 accepted substrate can replay its own development evidence.
- V4D exceptions are closed or explicitly renewed by Meta.

### V4D-4 Ratified Kernel Boundaries

Goal: ratify any Class 4 surfaces.

Outputs:

- Human-ratified constitution/trust-root/signing/wire-schema decisions.
- Evidence that Class 4 was not self-selected by agents.

Acceptance:

- Exact human ratification exists.
- Meta does not ratify Class 4 by itself.

## Test Plan

Required documentation checks:

- `rg -n "DevEventEnvelope|AgentIdentity|BootstrapException|V4D|assumptions" docs/v5_dev`
- `rg -n "usable_now|usable_with_adapter|not_usable_yet|do_not_use" docs/v5_dev/V4_NATIVE_REALITY_MAP.md`

V4D-0 worker boundary checks:

These checks applied to the V4D-R0 plan-save and Reality Map worker task, not
to the full v0.8 implementation wave. Verify that the delegated V4D-R0 worker
changed only:
  - `docs/v5_dev/V5_DEVKERNEL_PLAN_v0.8.md`
  - `docs/v5_dev/README.md`
  - `docs/v5_dev/V4_NATIVE_REALITY_MAP.md`

Full v0.8 implementation wave boundary checks:

- Verify no edits to `TASK_BOARD.json`, `src/**`, `constitution.md`, or
  `genesis_payload.toml`.
- Verify harness entry changes are limited to read-order and gated MetaAI merge
  language.
- Verify schemas, templates, docs, and tests remain provider-neutral.

Required V4 anchor checks:

- `rg -n "ChainTape|CAS|L4.E|PromptCapsule|AttemptTelemetry|EvidenceCapsule|Veto|transition_ledger|rejection_evidence|head_t_witness" /home/zephryj/projects/turingosv4`
- For every `usable_now` Reality Map entry, confirm at least one V4 `src`
  anchor and one V4 `tests` anchor.

Future CI checks:

- V5 runtime boundary test: product/runtime code cannot reference
  `AGENT_ENTRY.md`, `docs/harness/broadcast/**`, `docs/harness/tasks/**`, V4
  `handover/evidence/**`, or absolute V4 local paths.
- Development evidence shape test: sample DevEventEnvelope records validate
  required fields.
- Bootstrap exception expiry test: every open exception has owner, reason,
  expiration condition, and replacement path.

## Assumptions

- The current task is Class 0 or Class 1 documentation-only work because it adds
  non-runtime development planning documents.
- The V5 accepted substrate is not yet complete, so development evidence remains
  Candidate until Meta accepts it.
- V4 anchors are evidence for a Reality Map only. They do not grant V5 runtime
  authority.
- Any missing code or test anchor means the item cannot be labeled
  `usable_now`.
- The canonical V5 runtime truth surfaces will be defined later through the
  appropriate risk-class process.
