# GEMINI.md

This compatibility file routes this CLI session into the shared TuringOS V5
harness. It does not grant a role, capability lane, review lane, or merge
authority.

Read first:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`

After that, follow only the role entry explicitly assigned by the human prompt,
TaskPacket, ReviewPacket, or Meta continuation. This file does not provide
default QA, test, risk review, Veto, implementation, review, or merge
capabilities.

Boundaries:

- Do not infer role from the CLI label.
- Do not proceed past intake without an explicit role assignment.
- Do not touch Class 4 surfaces without exact human ratification.
