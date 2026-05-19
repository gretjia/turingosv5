# AgentIdentity

schema:
actor_id:
declared_role:
role_assignment_source:
role_assignment_ref:
role_assignment_cid:
assigned_by_actor_id:
provider_label:
runtime_label:
capabilities:
started_at:

## Notes

- `provider_label` is provenance only.
- `provider_label` is not authority.
- `role_assignment_cid` is required before a declared role can authorize work.
- Capabilities guide task eligibility but do not override TaskPacket limits.
