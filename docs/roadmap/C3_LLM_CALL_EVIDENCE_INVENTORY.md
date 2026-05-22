# C3 LLM Call Evidence Inventory

This inventory is a Class 3 contract note for LLM call evidence. It is based
only on the sandbox task context exported for
`V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001`; it does not claim to inspect
runtime implementation.

## Contract Terms

Every accepted LLM gateway path must preserve these evidence terms:

- `PromptCapsule`: the bounded prompt input package submitted to the gateway.
- `AttemptTelemetry`: per-attempt metadata needed to audit retries, failures,
  and provider interaction shape.
- `EvidenceTuple`: the durable record tying prompt, attempt telemetry, result,
  and acceptance/rejection evidence together.
- no naked LLM call: product or harness code must not call an LLM provider
  outside the gateway contract and evidence path.

## Inventory

| Anchor | Classification | Evidence requirement | Notes |
| --- | --- | --- | --- |
| `PromptCapsule` | gateway | Required input evidence | A gateway call must receive a prompt capsule rather than loose prompt text. |
| `AttemptTelemetry` | gateway | Required attempt evidence | Each provider attempt must produce telemetry before the result can be accepted. |
| `EvidenceTuple` | gateway | Required durable evidence | The gateway result must be connectable to the prompt capsule and attempt telemetry. |
| `no naked LLM call` invariant | gateway | Required negative constraint | Any direct provider call that bypasses the evidence tuple is a violation candidate. |
| Runtime LLM implementation | unknown | Not inventoried in this sandbox | The task context forbids implementing runtime and exports no runtime source. |
| Fake/test LLM surfaces | fake/test | Must stay explicitly test-only | No concrete fake/test source was exported in this sandbox. |
| Unanchored provider calls | violation candidate | Must be rejected unless routed through evidence | No concrete violation was exported in this sandbox. |

## Acceptance Rule

A future LLM gateway is acceptable only when it can show all of the following:

1. The call starts from a `PromptCapsule`.
2. Each provider attempt records `AttemptTelemetry`.
3. The accepted or rejected outcome is represented as an `EvidenceTuple`.
4. Review can verify there is no naked LLM call outside that evidence path.

## Non-Goals

This document does not implement an LLM runtime, choose a provider, define
