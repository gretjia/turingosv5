# Multi-Agent Duplicate Claim Canary

Task ID: V5-STRESS-DUPLICATE-CLAIM-001

Observed worker slot: worker-dup-b

Claim PR URL placeholder: https://github.com/gretjia/turingosv5/pull/7

Coordination rule: earliest_valid_claim_wins. Duplicate draft PR claims for the
same atom are evidence, but only the earliest valid claim proceeds to the
WorkerReport implementation path.

WorkerReport marker: this canary document intentionally includes
`WorkerReport` and `[WORKER_HALT]` so the acceptance check can confirm the
single-shot worker lifecycle.
