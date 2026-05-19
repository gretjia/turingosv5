# Active Merge Gate Contract

Status: V4D-G1 contract.

MergeDecisionAccepted is necessary but not sufficient. GitHub branch protection remains mandatory.

This contract defines the active merge gate rules without implementing automatic
merge logic.

## Inputs

The gate requires:

- merge_decision_cid
- pr_head_sha
- required_checks
- review_decision
- conversation_resolution
- branch_protection_snapshot_cid

Optional but common inputs:

- veto_verdict_cid
- worker_report_cid
- class4_ratification_cid
- bootstrap_exception

## Rejection Rules

- accepted decision with failed CI is rejected
- accepted decision with unresolved conversations is rejected
- accepted decision with missing branch protection snapshot is rejected
- bootstrap exception without restoration evidence is rejected
- author final-audit is rejected
- unratified Class 4 mutation is rejected
- dirty merge state is superseded, not manually rebased

## Bootstrap Exception Flow

Bootstrap exceptions are exact and temporary. They exist only to handle
development bootstrapping such as a one-time protected-branch review setting
change.

Required event sequence:

1. BootstrapExceptionRequested
2. BootstrapExceptionAccepted
3. BranchProtectionSnapshotRecorded
4. BootstrapExceptionRestored

The restored snapshot must prove that branch protection returned to the expected
policy after the temporary exception.

## Decision

MetaAI may proceed only when:

- MergeDecisionAccepted exists for the PR head SHA
- required checks pass
- review requirements pass
- conversations are resolved
- branch protection snapshot exists
- no forbidden files were touched
- Class 4 was ratified when touched
- no bootstrap exception remains unrestored
