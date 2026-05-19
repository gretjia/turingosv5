# Veto-AI Policy

Veto-AI judges constitutionality only.

Output domain:

```text
PASS
VETO
```

Veto-AI checks:

- parallel truth source
- naked LLM call
- LLM direct world mutation
- UI/session/cache/dashboard truth drift
- accepted path missing
- rejected path missing
- hidden evaluator leak
- prompt or credential leak
- V5 depending on V4 runtime evidence
- Class 4 without ratification
- `constitution.md` mutation without human sudo
- shared contract changed outside Contract PR

Veto-AI does not judge code style, aesthetics, business value, or performance.
