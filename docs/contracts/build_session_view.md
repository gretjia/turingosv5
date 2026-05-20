# BuildSessionView Contract

BuildSessionView is a derived view over accepted build evidence. It is a
projection for display, inspection, and operator ergonomics only.

## Source Inputs

The view is rebuilt from accepted source inputs:

- Dev events that describe build attempts and outcomes.
- Accepted artifact metadata.
- Accepted test run references.
- Accepted review or gate decisions that attach to the build.

Those inputs remain authoritative. The view stores no independent decision,
admission, merge, or evidence state.

## Rebuild Rule

Consumers must be able to delete cache data for BuildSessionView and rebuild
the same view from the source inputs. If a cached copy disagrees with the
accepted inputs, the cached copy is discarded and the projection is rebuilt.

Regeneration must be deterministic for the same accepted inputs. Missing or
stale view data is a cache problem, not a change to build state.

## Non-Goals

BuildSessionView does not:

- Admit work into the system.
- Decide whether a PR is accepted.
- Replace DevTape, accepted artifact metadata, test evidence, or gate evidence.
- Create a new runtime substrate.
- Make UI, dashboard, session, or cache state authoritative.
