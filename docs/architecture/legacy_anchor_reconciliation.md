# Legacy Anchor Reconciliation

Status: K0/C1 candidate chronology seal.
Date: 2026-05-19.
Commit: `dfa002fc1e506ce72406e549abac3420a8ec5805`.

This document reconciles older V4/constitution-era claims with current V5
`main`. It is development evidence only and must not become runtime truth.

## Chronology

| Date / phase | Meaning for current V5 |
| --- | --- |
| 2026-04-26 constitution-era audit, as referenced by the Human Architect | Historical governance context. It does not prove current `main` contains a V5-native substrate. |
| V4 Native Reality Map | Useful anchor set for old-system concepts. It remains docs-only evidence unless a V5 task creates a native adapter or fixture. |
| V5 DevKernel v0.8 | Development harness plan. It explicitly avoids treating V4 as V5 runtime truth. |
| 2026-05-19 K0 Reality Proof | Current-machine inspection of this checkout. It supersedes narrative claims for what current `main` can execute. |

## Current Main Findings

- `constitution.md` is not present in this checkout.
- `genesis_payload.toml` is not present in this checkout.
- `handover/` is not present in this checkout.
- V5 `src/` contains only the small DevTape CLI/devtool surface.
- No `src/ledger.rs`, `src/wal.rs`, or `bus.graveyard` legacy control surface is
  present.
- No git2 ChainTape/CAS/Sequencer/HEAD_t implementation is present in current
  `src` or `tests`.

## V4 Anchors

The following old-system concepts remain useful as design anchors only:

- ChainTape / transition ledger
- CAS
- L4.E / rejection evidence
- PromptCapsule
- AttemptTelemetry
- EvidenceCapsule
- Veto
- `head_t_witness`

They may inform small V5-native contracts, fixtures, and adapters. They may not
be imported as V5 runtime truth, accepted production evidence, genesis, or local
path authority.

## Legacy Surfaces

| Surface | Current classification |
| --- | --- |
| V4 ChainTape/CAS objects | do_not_use_as_v5_truth |
| V4 `genesis_payload.toml` | do_not_use_as_v5_truth |
| V4 `handover/evidence/**` | do_not_use_as_v5_truth |
| V4 dashboards, sessions, caches, reports | derived_or_historical_only |
| V4 WAL / ledger / bus concepts | adapter_or_fixture_only |
| gix/gitoxide migration | future_research_only |

## Seal

The current K0 result does not ratify a production substrate. It seals only this
development fact:

```text
Current main has a semantic DevTape development-governance MVP.
Current main does not prove git2 ChainTape/CAS/Sequencer/HEAD_t.
```

Any future task that changes canonical substrate, typed transaction wire schema,
canonical signing payload, sequencer admission, trust root, or constitution
semantics remains Class 4 and requires exact Human Architect ratification.
