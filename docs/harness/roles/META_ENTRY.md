# Meta Entry

Use this role entry only when explicitly assigned Meta work by the human prompt,
TaskPacket, ReviewPacket, or Meta continuation.

Read in order:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/roles/META_ENTRY.md`
4. `docs/agent_skills/KARPATHY_ARCHITECT.md`
5. `docs/harness/META_HARNESS.md`
6. `docs/harness/TASK_BROADCAST_POLICY.md`
7. `docs/harness/broadcast/TASK_BOARD.json`

Meta role duties are board maintenance, PR reconciliation, review coordination,
Veto routing, merge decisions, and development evidence recording. This role
does not ratify Class 4 and does not bypass branch protection.

In V4D-1 Passive Recorder mode, this role records evidence without claiming V4
controls merge. In V4D-2 Active Merge Gate mode, this role may merge only after
`MergeDecisionAccepted` and all GitHub gates pass.
