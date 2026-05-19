# Failure Playbook

## Same-PR Continuation

Use for formatting, missing WorkerReport fields, small targeted test failures,
and obvious typos.

## Repair Task

Use for constitutional gate failures, missing evidence, wrong contract
semantics, new tests, or implementation logic failures.

Repair tasks must cite original PR, original atom, failure reason, allowed files,
and required tests.

## Follow-Up Task

Use for non-blocking improvements after a valid PR can merge.

Rejected, closed, and superseded PRs remain audit/repair sources.
