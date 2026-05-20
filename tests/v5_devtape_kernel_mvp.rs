use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use turingosv5::devtool::{
    append_event, audit_board_drift, derive_board, merge_check, AppendInput, MergeGateDecision,
};

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turingosv5-devtape-{name}-{}-{nanos}",
        std::process::id()
    ))
}

fn envelope(
    event_id: &str,
    event_type: &str,
    previous: Option<&str>,
    runtime_truth: bool,
) -> Value {
    json!({
        "event_id": event_id,
        "event_type": event_type,
        "project_id": "turingosv5",
        "actor_identity_cid": "sha256:actor0001",
        "payload_cid": "sha256:filled-by-append",
        "previous_event_cid": previous,
        "observed_at": "2026-05-19T00:00:00Z",
        "source": "local_cli",
        "subject": {
            "repo": "gretjia/turingosv5",
            "branch": null,
            "pr": null,
            "files": []
        },
        "evidence": {
            "commands": [],
            "artifacts": [],
            "source_anchors": []
        },
        "classification": {
            "risk_class": 0,
            "candidate": true,
            "runtime_truth": runtime_truth
        },
        "integrity": {
            "payload_hash": "sha256:filled-by-append",
            "envelope_hash": "sha256:filled-by-append"
        }
    })
}

fn input(
    event_id: &str,
    event_type: &str,
    previous: Option<String>,
    payload: Value,
) -> AppendInput {
    AppendInput {
        previous_record_hash: previous.clone(),
        envelope: envelope(event_id, event_type, previous.as_deref(), false),
        payload,
    }
}

fn task_payload(atom_id: &str) -> Value {
    json!({
        "atom_id": atom_id,
        "title": format!("{atom_id} task"),
        "phase": "V5-K0",
        "lane": "RealityProof",
        "risk_class": 0,
        "priority": "P0",
        "status": "open",
        "self_select": true,
        "meta_opened": true,
        "claim_mode": "open_pool",
        "claim_required": true,
        "claim_method": "draft_pr",
        "required_capabilities": ["docs"],
        "preferred_capabilities": ["audit"],
        "allowed_files": ["docs/roadmap/TURINGOS_REALITY_MAP_K0.md"],
        "forbidden_files": ["src/**"],
        "task_packet": format!("docs/harness/broadcast/tasks/{atom_id}.json"),
        "acceptance_criteria": ["git diff --check"],
        "duplicate_policy": "first_valid_pr_wins",
        "blockers": []
    })
}

fn append(
    store: &Path,
    event_id: &str,
    event_type: &str,
    previous: Option<String>,
    payload: Value,
) -> String {
    append_event(store, input(event_id, event_type, previous, payload))
        .expect("append should succeed")
        .record_hash
}

#[test]
fn devtape_appends_hash_chain_and_rejects_invalid_records() {
    let dir = temp_path("append");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");

    let first = append_event(
        &store,
        input(
            "e1",
            "DevTaskCreated",
            None,
            task_payload("V5-K0-C0-REALITY-MAP-001"),
        ),
    )
    .expect("first event should append");
    assert!(first.record_hash.starts_with("sha256:"));
    assert_eq!(first.previous_record_hash, None);

    let second = append_event(
        &store,
        input(
            "e2",
            "TaskBroadcasted",
            Some(first.record_hash.clone()),
            json!({"atom_id": "V5-K0-C0-REALITY-MAP-001"}),
        ),
    )
    .expect("second event should append with correct previous hash");
    assert_eq!(
        second.previous_record_hash.as_deref(),
        Some(first.record_hash.as_str())
    );
    assert_ne!(first.record_hash, second.record_hash);

    let bad_previous = append_event(
        &store,
        input(
            "e3",
            "TaskClaimed",
            Some("sha256:not-the-tip".to_string()),
            json!({"atom_id": "V5-K0-C0-REALITY-MAP-001", "claim_pr_url": "https://example.test/pr/1"}),
        ),
    );
    assert!(
        bad_previous.is_err(),
        "append must reject a broken previous hash"
    );

    let runtime_truth = AppendInput {
        previous_record_hash: Some(second.record_hash.clone()),
        envelope: envelope("e4", "TaskClaimed", Some(&second.record_hash), true),
        payload: json!({"atom_id": "V5-K0-C0-REALITY-MAP-001"}),
    };
    assert!(
        append_event(&store, runtime_truth).is_err(),
        "development DevTape must reject runtime_truth=true"
    );

    let unknown = append_event(
        &store,
        input(
            "e5",
            "QueueMessagePublished",
            Some(second.record_hash),
            json!({"atom_id": "V5-K0-C0-REALITY-MAP-001"}),
        ),
    );
    assert!(unknown.is_err(), "append must reject unknown event types");
}

#[test]
fn board_projection_requires_broadcast_and_reports_manual_drift() {
    let dir = temp_path("projection");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");

    let k0_task = append(
        &store,
        "e1",
        "DevTaskCreated",
        None,
        task_payload("V5-K0-C0-REALITY-MAP-001"),
    );
    let board_before_broadcast = derive_board(&store).expect("board should derive");
    assert!(
        board_before_broadcast["tasks"]
            .as_array()
            .is_some_and(|tasks: &Vec<Value>| tasks.is_empty()),
        "task must not appear on board before TaskBroadcasted"
    );

    let k0_broadcast = append(
        &store,
        "e2",
        "TaskBroadcasted",
        Some(k0_task),
        json!({"atom_id": "V5-K0-C0-REALITY-MAP-001"}),
    );
    let mut a1 = task_payload("V5-SYS-A1-BASELINE-SEMANTIC-CLOSE-001");
    a1["blockers"] = json!(["V5-K0-C0-REALITY-MAP-001"]);
    let a1_task = append(&store, "e3", "DevTaskCreated", Some(k0_broadcast), a1);
    append(
        &store,
        "e4",
        "TaskBroadcasted",
        Some(a1_task),
        json!({"atom_id": "V5-SYS-A1-BASELINE-SEMANTIC-CLOSE-001"}),
    );

    let board = derive_board(&store).expect("board should derive from events");
    let tasks = board["tasks"].as_array().expect("tasks must be an array");
    assert_eq!(tasks.len(), 2);
    assert!(
        tasks
            .iter()
            .any(|task| task["atom_id"] == "V5-K0-C0-REALITY-MAP-001"),
        "K0/C0 appears after TaskBroadcasted"
    );
    let a1_row = tasks
        .iter()
        .find(|task| task["atom_id"] == "V5-SYS-A1-BASELINE-SEMANTIC-CLOSE-001")
        .expect("A1 should be projected");
    assert_eq!(a1_row["blockers"], json!(["V5-K0-C0-REALITY-MAP-001"]));

    assert!(audit_board_drift(&store, &board).is_ok());
    let mut mutated = board;
    mutated["tasks"][0]["title"] = json!("manual edit");
    assert!(
        audit_board_drift(&store, &mutated).is_err(),
        "manual board mutation must be reported as drift"
    );
}

#[test]
fn board_projection_removes_superseded_tasks_from_active_board() {
    let dir = temp_path("superseded");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");

    let created = append(
        &store,
        "e1",
        "DevTaskCreated",
        None,
        task_payload("V5-LEGACY-DIRECT-001"),
    );
    let broadcasted = append(
        &store,
        "e2",
        "TaskBroadcasted",
        Some(created),
        json!({"atom_id": "V5-LEGACY-DIRECT-001"}),
    );
    let board_before = derive_board(&store).expect("board should derive");
    assert_eq!(
        board_before["tasks"].as_array().expect("tasks").len(),
        1,
        "broadcasted task should be visible before supersede"
    );

    append(
        &store,
        "e3",
        "TaskSuperseded",
        Some(broadcasted),
        json!({
            "atom_id": "V5-LEGACY-DIRECT-001",
            "reason": "replaced by draft-pr claimable task wave"
        }),
    );
    let board_after = derive_board(&store).expect("board should derive");
    assert!(
        board_after["tasks"].as_array().expect("tasks").is_empty(),
        "superseded tasks must not remain claimable on the active board"
    );
}

#[test]
fn merge_check_requires_claim_report_audit_veto_ci_and_branch_protection() {
    let dir = temp_path("merge");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let atom = "V5-K0-C0-REALITY-MAP-001";

    let e1 = append(&store, "e1", "DevTaskCreated", None, task_payload(atom));
    let e2 = append(
        &store,
        "e2",
        "TaskBroadcasted",
        Some(e1),
        json!({"atom_id": atom}),
    );
    let e3 = append(
        &store,
        "e3",
        "MergeDecisionRecorded",
        Some(e2),
        json!({
            "atom_id": atom,
            "pr_number": 7,
            "decision": "PROCEED",
            "required_ci_passed": true,
            "audit_passed": true,
            "veto_passed": true,
            "conversation_resolution": true,
            "branch_protection_snapshot": "sha256:branch0001",
            "merge_state_status": "CLEAN"
        }),
    );
    let missing_claim = merge_check(&store, 7).expect("merge check should run");
    assert_eq!(missing_claim.decision, MergeGateDecision::HOLD);
    assert!(missing_claim
        .missing_evidence
        .contains(&"TaskClaimed".to_string()));

    let e4 = append(
        &store,
        "e4",
        "TaskClaimed",
        Some(e3),
        json!({
            "atom_id": atom,
            "pr_number": 7,
            "claim_pr_url": "https://github.com/gretjia/turingosv5/pull/7",
            "worker_identity": "worker-test",
            "createdAt": "2026-05-19T00:00:00Z"
        }),
    );
    let missing_report = merge_check(&store, 7).expect("merge check should run");
    assert_eq!(missing_report.decision, MergeGateDecision::HOLD);
    assert!(missing_report
        .missing_evidence
        .contains(&"WorkerReportSubmitted".to_string()));

    let e5 = append(
        &store,
        "e5",
        "WorkerReportSubmitted",
        Some(e4),
        json!({
            "atom_id": atom,
            "pr_number": 7,
            "claim_pr_url": "https://github.com/gretjia/turingosv5/pull/7",
            "ready_pr_url": "https://github.com/gretjia/turingosv5/pull/7",
            "worktree": "/tmp/worktree",
            "files_changed": ["docs/roadmap/TURINGOS_REALITY_MAP_K0.md"],
            "tests_run": [{"cmd": "git diff --check", "status": "pass"}],
            "forbidden_files_touched": false,
            "class4_touched": false,
            "worker_halt_confirmation": "[WORKER_HALT]"
        }),
    );
    let e6 = append(
        &store,
        "e6",
        "AuditVerdictSubmitted",
        Some(e5),
        json!({"atom_id": atom, "pr_number": 7, "verdict": "PASS"}),
    );
    let e7 = append(
        &store,
        "e7",
        "VetoVerdictSubmitted",
        Some(e6),
        json!({"atom_id": atom, "pr_number": 7, "verdict": "PASS"}),
    );
    append(
        &store,
        "e8",
        "MergeDecisionRecorded",
        Some(e7),
        json!({
            "atom_id": atom,
            "pr_number": 7,
            "decision": "PROCEED",
            "required_ci_passed": false,
            "audit_passed": true,
            "veto_passed": true,
            "conversation_resolution": true,
            "branch_protection_snapshot": "sha256:branch0001",
            "merge_state_status": "CLEAN"
        }),
    );
    let failed_ci = merge_check(&store, 7).expect("merge check should run");
    assert_eq!(failed_ci.decision, MergeGateDecision::HOLD);
    assert!(failed_ci
        .reasons
        .iter()
        .any(|reason: &String| reason.contains("CI")));

    let store_ok = dir.join("events-ok.jsonl");
    let e1 = append(&store_ok, "ok1", "DevTaskCreated", None, task_payload(atom));
    let e2 = append(
        &store_ok,
        "ok2",
        "TaskBroadcasted",
        Some(e1),
        json!({"atom_id": atom}),
    );
    let e3 = append(
        &store_ok,
        "ok3",
        "TaskClaimed",
        Some(e2),
        json!({"atom_id": atom, "pr_number": 7, "claim_pr_url": "https://github.com/gretjia/turingosv5/pull/7", "createdAt": "2026-05-19T00:00:00Z"}),
    );
    let e4 = append(
        &store_ok,
        "ok4",
        "WorkerReportSubmitted",
        Some(e3),
        json!({"atom_id": atom, "pr_number": 7, "worker_halt_confirmation": "[WORKER_HALT]"}),
    );
    let e5 = append(
        &store_ok,
        "ok5",
        "AuditVerdictSubmitted",
        Some(e4),
        json!({"atom_id": atom, "pr_number": 7, "verdict": "PASS"}),
    );
    let e6 = append(
        &store_ok,
        "ok6",
        "VetoVerdictSubmitted",
        Some(e5),
        json!({"atom_id": atom, "pr_number": 7, "verdict": "PASS"}),
    );
    append(
        &store_ok,
        "ok7",
        "MergeDecisionRecorded",
        Some(e6),
        json!({
            "atom_id": atom,
            "pr_number": 7,
            "decision": "PROCEED",
            "required_ci_passed": true,
            "audit_passed": true,
            "veto_passed": true,
            "conversation_resolution": true,
            "branch_protection_snapshot": "sha256:branch0001",
            "merge_state_status": "CLEAN"
        }),
    );
    let proceed = merge_check(&store_ok, 7).expect("merge check should run");
    assert_eq!(proceed.decision, MergeGateDecision::PROCEED);
    assert!(proceed.missing_evidence.is_empty());
}

#[test]
fn cli_appends_derives_audits_and_checks_merge() {
    let dir = temp_path("cli");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let event_file = dir.join("event.json");
    let board_file = dir.join("board.json");

    let event = input(
        "cli1",
        "DevTaskCreated",
        None,
        task_payload("V5-K0-C0-REALITY-MAP-001"),
    );
    fs::write(
        &event_file,
        serde_json::to_vec_pretty(&event).expect("input should serialize"),
    )
    .expect("event file should be written");

    let bin = std::env::var("CARGO_BIN_EXE_turingos-dev")
        .expect("cargo should expose turingos-dev binary path to integration tests");
    let append = Command::new(&bin)
        .args([
            "event",
            "append",
            "--file",
            event_file.to_str().expect("path should be utf8"),
            "--store",
            store.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("cli append should run");
    assert!(
        append.status.success(),
        "append stderr: {}",
        String::from_utf8_lossy(&append.stderr)
    );

    let derive = Command::new(&bin)
        .args([
            "board",
            "derive",
            "--store",
            store.to_str().expect("path should be utf8"),
            "--out",
            board_file.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("cli derive should run");
    assert!(
        derive.status.success(),
        "derive stderr: {}",
        String::from_utf8_lossy(&derive.stderr)
    );
    assert!(board_file.exists());

    let audit = Command::new(&bin)
        .args([
            "audit",
            "--store",
            store.to_str().expect("path should be utf8"),
            "--board",
            board_file.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("cli audit should run");
    assert!(
        audit.status.success(),
        "audit stderr: {}",
        String::from_utf8_lossy(&audit.stderr)
    );

    let merge = Command::new(&bin)
        .args([
            "merge",
            "check",
            "--store",
            store.to_str().expect("path should be utf8"),
            "--pr",
            "7",
        ])
        .output()
        .expect("cli merge check should run");
    assert!(
        !merge.status.success(),
        "merge check should fail without MergeDecisionRecorded"
    );
}
