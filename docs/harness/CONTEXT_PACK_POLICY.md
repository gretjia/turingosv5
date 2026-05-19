# Context Pack Policy

Workers read enough context to implement one TaskPacket, not the whole project.

## Packs

- MetaContextPack: roadmap, board, open PRs, CI, class policy, branch status.
- WorkerContextPack: entry files, TaskPacket, relevant contract, allowed files,
  tests.
- ReviewerContextPack: TaskPacket, diff, PR body, test output, invariants.
- VetoContextPack: TaskPacket, touched files, risk class, invariant checklist.

## Shielded From Workers

- hidden gate oracle
- Veto scoring internals
- unrelated failed attempts
- private diagnostics
- secrets and credentials
- hidden benchmark leaks
