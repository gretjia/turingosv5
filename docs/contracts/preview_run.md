# Preview Run Contract

Status: v0.1 development contract.

Scope: product contract seed only. This document does not define a preview
server, runtime authority, trust root, signing payload, typed transaction wire
schema, or sequencer admission.

## PreviewRunCapsule

PreviewRunCapsule records a read-only render attempt for an ArtifactBundle CID.
It is Candidate evidence for what a generated artifact bundle displayed after
redaction, not an accepted runtime state transition.

Required fields:

- preview_run_cid
- artifact_bundle_cid
- redaction_result_cid
- renderer
- rendered_at

The truth path is:

```text
ArtifactBundle CID -> redaction result CID -> PreviewRunCapsule CID
```

`artifact_bundle_cid` is the only artifact input. Preview code may inspect the
content-addressed bundle named by that CID, apply the referenced redaction
result, and emit a preview_run_cid for the read-only render evidence.

## Boundary

PreviewRunCapsule must not be treated as source truth, build truth, accepted
state, or cache truth. It never mutates ArtifactBundle content and never grants
authority to replace the bundle it renders.

Preview evidence must cite redaction before display. A preview without
`redaction_result_cid` is invalid for this contract.

Invalid shapes:

- PreviewRunCapsule used as accepted runtime truth
- PreviewRunCapsule that omits artifact_bundle_cid
- PreviewRunCapsule that omits redaction_result_cid
- preview cache used as canonical artifact content
- prompt material exposed as preview evidence
