# Friendly Error and L4.E Contract

This contract defines the user-facing behavior for rejection outcomes in
failure paths that enter `L4.E`.

## Scope

- Capture user-safe rejection evidence instead of internal panic or stack traces.
- Classify failures with `RejectClass`.
- Keep `world_head` unchanged on rejected attempts.

## User-facing error shape

When validation or policy checks reject an attempt, the system must return
friendly, non-sensitive error text and a machine-readable classification.

- `RejectClass` MUST be present.
- Error text MUST be phrased for end-user understanding.
- Internal artifacts (stack traces, prompt text, raw secrets, internal IDs)
  MUST NOT be exposed in user-facing messages.

## L4.E failure contract

- `L4.E` is the failure lane for rejected attempts.
- Evidence for `L4.E` must include:
  - attempt identity,
  - `RejectClass`,
  - user-safe message,
  - and the reason for rejection.
- `world_head` MUST NOT advance on `L4.E` paths.
- A rejection in `L4.E` MUST be auditable and replay-safe.

## Canonical requirement

Any rejected path that is recorded under this contract is a hard stop for
execution state progression and must preserve deterministic recovery from the
same rejected attempt.
