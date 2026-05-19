# Dependency Policy

Workers must not add dependencies unless the TaskPacket explicitly allows a
Dependency PR.

Dependency PRs must state package name/version, why existing code is
insufficient, license risk, supply-chain risk, runtime boundary impact, and
rollback plan.
