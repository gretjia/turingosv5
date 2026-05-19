# DevEventEnvelope

schema:
event_id:
event_type:
project_id:
actor_identity_cid:
payload_cid:
previous_event_cid:
observed_at:
source:
subject:
  repo:
  branch:
  pr:
  files:
evidence:
  commands:
  artifacts:
  source_anchors:
classification:
  risk_class:
  candidate:
  runtime_truth: false
integrity:
  parent_event_ids:
  payload_hash:
  envelope_hash:

## Notes

- Payload without this envelope is not governance evidence.
- `actor_identity_cid` must reference AgentIdentity or HumanArchitect evidence.
- `payload_cid` must reference the event payload.
- `previous_event_cid` is null only for the first event in a project chain.
- `classification.runtime_truth` must remain false.
