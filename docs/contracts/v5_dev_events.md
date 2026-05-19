# V5 DevEvent Contracts

Status: v0.8 development contract.

Scope: development governance only. These contracts do not define V5 runtime
truth, trust root, signing payload, typed transaction wire schema, or sequencer
admission.

## Authority Model

Every material development payload must be carried by a DevEventEnvelope before
it can be used as governance evidence.

Payload without DevEventEnvelope is invalid.

AgentIdentity is provider-neutral. A provider label never grants role authority.
provider_label is provenance only. declared_role without role_assignment_cid is invalid.

Role authority comes from explicit assignment evidence:

- human prompt
- TaskPacket
- ReviewPacket
- Meta continuation
- accepted DevKernel event
- explicit role entry referenced by assignment evidence

## DevEventEnvelope

Required fields:

- event_id
- event_type
- project_id
- actor_identity_cid
- payload_cid
- previous_event_cid
- observed_at
- source
- subject
- evidence
- classification
- integrity

The envelope links an actor identity to a payload and a prior event. The payload
may be a task, report, PR, CI result, Veto verdict, merge decision, merge record,
bootstrap exception, branch protection snapshot, or repair event.

`classification.runtime_truth` must be false. DevEvents are development
governance evidence until a later V5-native accepted substrate supersedes this
bootstrap layer.

## AgentIdentity

Required fields:

- actor_id
- declared_role
- role_assignment_source
- role_assignment_ref
- role_assignment_cid
- assigned_by_actor_id
- provider_label
- runtime_label
- capabilities
- started_at

`provider_label` and `runtime_label` describe provenance and execution context.
They do not authorize MetaAI, WorkerAI, AuditorAI, or VetoAI duties.

## MVP Event Payloads

The v0.8 MVP event family is:

- DevTaskCreated
- WorkerReportSubmitted
- PRCreated
- CIResultRecorded
- ReviewVerdictSubmitted
- VetoVerdictSubmitted
- MergeDecisionAccepted
- MergeDecisionRejected
- PRMerged
- BranchProtectionSnapshotRecorded
- BootstrapExceptionRequested
- BootstrapExceptionAccepted
- BootstrapExceptionRestored

Each payload is Candidate evidence until accepted by the appropriate gate.
Rejected payloads require rejection evidence instead of silent discard.

## Provider Neutrality

No contract may bind duties or authority to a model vendor, CLI brand, or account
label. A role is active only when the DevKernel can cite role assignment
evidence by CID.

Invalid examples:

- provider_label used as authority
- declared_role present without role_assignment_cid
- Veto verdict outside PASS or VETO
- merge decision without PR number
- PR merge record without a referenced merge decision

## MergeDecision

MergeDecision payloads must carry DevEvent CID references for task, role
assignment, WorkerReport, PRCreated, CIResult, review when required, Veto when
required, branch protection snapshot, bootstrap exception when used, and the
MergeDecision event itself.

`PROCEED` is valid only when CI, review, Veto, branch protection, conversation
resolution, forbidden-file checks, author-final-audit checks, and Class 4
ratification checks pass.

## BootstrapException

BootstrapException payloads must record exact V4 anchors, affected V5 files,
risk class, replacement path, Meta decision, expiration condition, and
restoration evidence. They are temporary development bridge evidence, never V5
runtime truth.
