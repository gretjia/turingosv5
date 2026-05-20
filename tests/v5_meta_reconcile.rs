use serde_json::json;
use turingosv5::devtool::meta_reconcile_report;

#[test]
fn meta_reconcile_finds_claims_reports_duplicates_and_open_tasks() {
    let board = json!({
        "tasks": [
            {
                "atom_id": "V5-K0-C1-PATH-DECISION-CHRONOLOGY-001",
                "status": "open"
            },
            {
                "atom_id": "V5-K1-C2-NO-NEW-SUBSTRATE-REGRESSION-001",
                "status": "pr_open",
                "pr_number": 10
            },
            {
                "atom_id": "V5-K0-C0-REALITY-MAP-HARD-GATE-001",
                "status": "merged",
                "pr_number": 15
            }
        ]
    });
    let prs = json!([
        {
            "number": 21,
            "title": "[CLAIM][V5-K0-C1-PATH-DECISION-CHRONOLOGY-001][Class1] Path decision",
            "isDraft": false,
            "createdAt": "2026-05-20T03:00:00Z",
            "url": "https://github.com/gretjia/turingosv5/pull/21",
            "body": "ClaimRecord\nWorkerReport\nworker_halt_confirmation: \"[WORKER_HALT]\"",
            "mergeStateStatus": "CLEAN",
            "statusCheckRollup": [
                {"name": "ci-basic", "conclusion": "SUCCESS"},
                {"name": "ci-constitution-light", "conclusion": "SUCCESS"}
            ]
        },
        {
            "number": 22,
            "title": "[CLAIM][V5-K0-C1-PATH-DECISION-CHRONOLOGY-001][Class1] Duplicate",
            "isDraft": true,
            "createdAt": "2026-05-20T03:05:00Z",
            "url": "https://github.com/gretjia/turingosv5/pull/22",
            "body": "ClaimRecord",
            "mergeStateStatus": "CLEAN"
        },
        {
            "number": 23,
            "title": "[CLAIM][V5-K0-C0-REALITY-MAP-HARD-GATE-001][Class1] Stale merged task",
            "isDraft": true,
            "createdAt": "2026-05-20T03:10:00Z",
            "url": "https://github.com/gretjia/turingosv5/pull/23",
            "body": "ClaimRecord",
            "mergeStateStatus": "CLEAN"
        },
        {
            "number": 24,
            "title": "unstructured change",
            "isDraft": true,
            "createdAt": "2026-05-20T03:15:00Z",
            "url": "https://github.com/gretjia/turingosv5/pull/24",
            "body": "",
            "mergeStateStatus": "CLEAN"
        }
    ]);

    let report = meta_reconcile_report(&board, &prs).expect("report should build");
    assert_eq!(report["scanned_prs"], 4);
    assert_eq!(
        report["open_task_atoms"],
        json!(["V5-K0-C1-PATH-DECISION-CHRONOLOGY-001"])
    );
    assert!(report["actions"]
        .as_array()
        .expect("actions")
        .iter()
        .any(|action| action["pr_number"] == 21
            && action["atom_id"] == "V5-K0-C1-PATH-DECISION-CHRONOLOGY-001"
            && action["action"] == "record_worker_report"));
    assert!(
        report["actions"]
            .as_array()
            .expect("actions")
            .iter()
            .any(|action| action["pr_number"] == 22
                && action["action"] == "supersede_duplicate_claim")
    );
    assert!(report["actions"]
        .as_array()
        .expect("actions")
        .iter()
        .any(
            |action| action["pr_number"] == 23 && action["action"] == "supersede_closed_task_claim"
        ));
    assert!(report["actions"]
        .as_array()
        .expect("actions")
        .iter()
        .any(|action| action["pr_number"] == 24 && action["action"] == "orphan_pr"));
}

#[test]
fn meta_reconcile_does_not_rerecord_pr_open_worker_reports() {
    let board = json!({
        "tasks": [
            {
                "atom_id": "V5-K2-C4-ARTIFACT-BUNDLE-CONTRACT-001",
                "status": "pr_open",
                "pr_number": 9
            },
            {
                "atom_id": "V5-K1-C2-NO-NEW-SUBSTRATE-REGRESSION-001",
                "status": "pr_open",
                "pr_number": 10
            }
        ]
    });
    let prs = json!([
        {
            "number": 9,
            "title": "[CLAIM][V5-K2-C4-ARTIFACT-BUNDLE-CONTRACT-001][Class1] ArtifactBundle",
            "isDraft": false,
            "createdAt": "2026-05-20T01:49:23Z",
            "url": "https://github.com/gretjia/turingosv5/pull/9",
            "body": "ClaimRecord\nWorkerReport\n[WORKER_HALT]",
            "mergeStateStatus": "BEHIND",
            "statusCheckRollup": [
                {"name": "ci-basic", "conclusion": "SUCCESS"},
                {"name": "ci-constitution-light", "conclusion": "SUCCESS"}
            ]
        },
        {
            "number": 10,
            "title": "[CLAIM][V5-K1-C2-NO-NEW-SUBSTRATE-REGRESSION-001][Class1] No substrate",
            "isDraft": false,
            "createdAt": "2026-05-20T01:49:27Z",
            "url": "https://github.com/gretjia/turingosv5/pull/10",
            "body": "ClaimRecord\nWorkerReport\n[WORKER_HALT]",
            "mergeStateStatus": "BEHIND",
            "statusCheckRollup": [
                {"name": "ci-basic", "conclusion": "FAILURE"},
                {"name": "ci-constitution-light", "conclusion": "SUCCESS"}
            ]
        }
    ]);

    let report = meta_reconcile_report(&board, &prs).expect("report should build");
    assert!(report["actions"]
        .as_array()
        .expect("actions")
        .iter()
        .any(|action| action["pr_number"] == 9 && action["action"] == "hold_until_branch_updated"));
    assert!(report["actions"]
        .as_array()
        .expect("actions")
        .iter()
        .any(|action| action["pr_number"] == 10 && action["action"] == "hold_failed_ci"));
}

#[test]
fn meta_reconcile_holds_dirty_or_unreported_claims() {
    let board = json!({
        "tasks": [
            {
                "atom_id": "V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001",
                "status": "open"
            }
        ]
    });
    let prs = json!([
        {
            "number": 31,
            "title": "[CLAIM][V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001][Class1] Evidence inventory",
            "isDraft": false,
            "createdAt": "2026-05-20T04:00:00Z",
            "url": "https://github.com/gretjia/turingosv5/pull/31",
            "body": "ClaimRecord",
            "mergeStateStatus": "DIRTY"
        }
    ]);

    let report = meta_reconcile_report(&board, &prs).expect("report should build");
    let action = &report["actions"][0];
    assert_eq!(action["action"], "hold_dirty_claim");
    assert_eq!(action["needs_worker_report"], true);
}

#[test]
fn meta_reconcile_recognizes_sandbox_worker_prs() {
    let board = json!({
        "tasks": [
            {
                "atom_id": "V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001",
                "status": "open"
            }
        ]
    });
    let prs = json!([
        {
            "number": 29,
            "title": "[WORKER][V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001] Sandbox submission",
            "isDraft": false,
            "createdAt": "2026-05-20T07:00:50Z",
            "url": "https://github.com/gretjia/turingosv5/pull/29",
            "body": "WorkerReport\n- atom_id: V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001\n- worker_halt_confirmation: [WORKER_HALT]\n",
            "mergeStateStatus": "CLEAN",
            "statusCheckRollup": [
                {"name": "ci-basic", "conclusion": "SUCCESS"},
                {"name": "ci-constitution-light", "conclusion": "SUCCESS"}
            ]
        }
    ]);

    let report = meta_reconcile_report(&board, &prs).expect("report should build");
    let action = &report["actions"][0];
    assert_eq!(action["pr_number"], 29);
    assert_eq!(
        action["atom_id"],
        "V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001"
    );
    assert_eq!(action["action"], "record_worker_report");
}

#[test]
fn meta_reconcile_treats_blocked_clean_ci_as_merge_check_candidate() {
    let board = json!({
        "tasks": [
            {
                "atom_id": "V5-K0-C1-PATH-DECISION-CHRONOLOGY-001",
                "status": "pr_open",
                "pr_number": 28
            }
        ]
    });
    let prs = json!([
        {
            "number": 28,
            "title": "[WORKER][V5-K0-C1-PATH-DECISION-CHRONOLOGY-001] Sandbox submission",
            "isDraft": false,
            "createdAt": "2026-05-20T07:00:21Z",
            "url": "https://github.com/gretjia/turingosv5/pull/28",
            "body": "WorkerReport\n- atom_id: V5-K0-C1-PATH-DECISION-CHRONOLOGY-001\n- worker_halt_confirmation: [WORKER_HALT]\n",
            "mergeStateStatus": "BLOCKED",
            "statusCheckRollup": [
                {"name": "ci-basic", "conclusion": "SUCCESS"},
                {"name": "ci-constitution-light", "conclusion": "SUCCESS"}
            ]
        }
    ]);

    let report = meta_reconcile_report(&board, &prs).expect("report should build");
    let action = &report["actions"][0];
    assert_eq!(action["pr_number"], 28);
    assert_eq!(action["action"], "run_merge_check");
}
