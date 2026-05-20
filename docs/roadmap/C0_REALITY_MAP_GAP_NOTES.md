# C0 Reality Map Gap Notes

This document captures the gaps identified during the K0 reality proof phase. It follows the "No human-guessed hyperparameters" and "Science, evidence, and data over intuition" principles by only claiming proven state where code and test anchors exist.

## Reality Map Gaps

| Claim | Current Evidence | Missing Evidence | Next Atom |
| --- | --- | --- | --- |
| git2 ChainTape/CAS/Sequencer/HEAD_t | None. `Cargo.toml` lacks `git2`. | `git2` dependency, `src/substrate/git2.rs`, and integration tests. | `V5-K0-C1-PATH-DECISION-CHRONOLOGY-001` |
| Product TISR (spec -> generate -> preview) | None. No `src` anchors for capsules. | `SpecCapsule`, `ArtifactBundle`, and `PreviewRunCapsule` definitions. | `V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001` |
| Evidence Rails (PromptCapsule / AttemptTelemetry) | Docs and policy only. | Executable implementation in `src` and proof in `tests`. | `V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001` |
| ArtifactBundle CAS Wire | None. | Content-addressed storage for artifact bundles. | `V5-K2-C4-ARTIFACT-BUNDLE-CONTRACT-001` |
| Preview Truth Path | None. | Preview server that reads only from `ArtifactBundle` CID. | `V5-K3-C5-PREVIEW-TRUTH-PATH-CONTRACT-001` |
| spec-derived tests | None. | Acceptance slots to TestRun conversion logic. | `V5-K6-C10-SPEC-DERIVED-TESTRUN-001` |

## Observation Summary

1. **Path B-pragmatic (git2)** is currently a "ghost claim". It is requested by the architecture but absent in the substrate.
2. **DevTape MVP** is the only proven governance substrate, but it is not for product runtime.
3. **Product Flow** is entirely unproven; no code or test anchors exist for the core spec/generate/preview loop.

## Hard Gate Verdict

**FAIL (conditional)**. Do not proceed to K1 as if the product substrate exists. The "ghost" status of git2 must be resolved in `C1` before building product features on top of it.

[WORKER_HALT]
