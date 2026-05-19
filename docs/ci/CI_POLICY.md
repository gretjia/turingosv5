# CI Policy

Initial required checks:

- `ci-basic`
- `ci-constitution-light`

Path-filtered checks must not become required until they have always-run shims,
because skipped required workflows can remain pending and block PRs.
