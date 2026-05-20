# Edit/Regenerate Versioning Contract

When a `ModificationRequestCapsule` is accepted, the system must create a **new
`ArtifactBundle`** rather than mutating an existing artifact record.

Regenerate is an explicit replay action. It derives a new bundle from:

- the accepted `ModificationRequestCapsule`
- the current accepted source evidence for the related session
- existing deterministic packaging rules

The old artifact remains valid for preview and may still be served from the
accepted evidence record set. A regeneration event creates a newer artifact
version while preserving read access to previous preview artifacts for audit and
reproducibility.

## Preview behavior for old artifact

After regeneration, older artifacts must remain previewable until explicitly revoked
by governance policy. If a user opens an earlier artifact reference, the system
must keep serving that historical preview artifact and should not silently rewrite
or replace it.

Regenerate should therefore be treated as:

- a new immutable artifact production event
- a new immutable reference edge
- a non-destructive update path for previous preview artifacts

## Non-goals

- Overwriting an existing artifact in place.
- Directly patching files of old artifacts.
- Introducing a fresh LLM call path for regeneration content.
