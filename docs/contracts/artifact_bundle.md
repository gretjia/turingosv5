# ArtifactBundle Contract

Status: v0.1 development/product contract seed.

Scope: ArtifactBundle describes generated application artifacts as Candidate
output. It is not runtime truth, a trust root, a signing payload, or a temporary
directory truth source.

## Required Fields

- artifact_bundle_cid
- spec_capsule_cid
- generation_attempt_cid
- files
- entrypoint

`artifact_bundle_cid` identifies the bundle record. `spec_capsule_cid` links the
bundle to the user specification capsule that caused generation.
`generation_attempt_cid` links the bundle to the attempt evidence that produced
it.

## Files

`files` is the ordered list of material files in the bundle. Each file entry
must include:

- path
- content_cid
- media_type
- role

File paths are relative paths inside the bundle. The file list is evidence for
what was produced; it is not a naked HTML string and it is not a cache snapshot.

## Entrypoint

`entrypoint` names one path from `files` as the first preview/build target. For
single-page web output this is usually an HTML file path. The entrypoint must be
inside the bundle file list so preview and audit code can resolve output through
the bundle contract instead of local workspace layout.

## Invalid Shapes

- artifact without artifact_bundle_cid
- artifact without spec_capsule_cid
- artifact without generation_attempt_cid
- artifact with empty files
- entrypoint outside files
- naked HTML string used as the artifact
- temporary directory truth used as accepted output
