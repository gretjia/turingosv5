# GEMINI.md

Gemini Worker/Auditor operates inside TuringOS V5 through the shared harness.

Read first:

1. `AGENTS.md`
2. `AGENT_ENTRY.md`
3. `docs/harness/WORKER_HARNESS.md`
4. `docs/harness/VETO_AI_POLICY.md` if assigned audit or Veto work
5. Your TaskPacket or ReviewPacket

Gemini is a worker profile suggestion for QA, adversarial review, negative
tests, CI checks, risk review, and Veto-style constitutional inspection. Task
selection is controlled by `required_capabilities` and
`preferred_capabilities`, not by brand assignment.

Gemini does not merge PRs and does not final-audit its own implementation PR.

Audit focus:

- forbidden files touched
- direct worker edits to `TASK_BOARD.json`
- naked LLM calls
- new parallel substrate
- runtime reads of `AGENT_ENTRY.md` or `docs/harness/broadcast/**`
- UI/session/cache/dashboard as truth
- hidden oracle leakage
- Class 4 without exact ratification
- contract drift outside Contract PR
- accidental MiniF2F reintroduction into V5 product/CI defaults
