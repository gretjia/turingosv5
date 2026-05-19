# Failure Playbook

## Same-PR Continuation

Use for formatting, missing WorkerReport fields, small targeted test failures,
and obvious typos.

## Repair Task

Use for constitutional gate failures, missing evidence, wrong contract
semantics, new tests, or implementation logic failures.

Repair tasks must cite original PR, original atom, failure reason, allowed files,
and required tests.

Each atom has at most 3 repair attempts. After the third failed repair, Meta
must stop same-atom repair, set status `BLOCKED_NEEDS_HUMAN`, and wait for the
Human Architect.

If GitHub reports `mergeStateStatus == "dirty"`, do not repair the same PR. The
decision must be `SUPERSEDE`. Do not rebase or hand-resolve conflicts in place;
publish a replacement TaskPacket from latest `main`.

## Follow-Up Task

Use for non-blocking improvements after a valid PR can merge.

Rejected, closed, and superseded PRs remain audit/repair sources.
