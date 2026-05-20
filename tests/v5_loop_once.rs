use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use turingosv5::devtool::{append_event, read_records, AppendInput};

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turingosv5-loop-once-{name}-{}-{nanos}",
        std::process::id()
    ))
}

fn bin() -> String {
    std::env::var("CARGO_BIN_EXE_turingos-dev")
        .expect("cargo should expose turingos-dev binary path")
}

fn envelope(event_id: &str, event_type: &str, previous: Option<&str>) -> Value {
    json!({
        "event_id": event_id,
        "event_type": event_type,
        "project_id": "turingosv5",
        "actor_identity_cid": "sha256:test-actor",
        "payload_cid": "sha256:filled-by-append",
        "previous_event_cid": previous,
        "observed_at": "2026-05-20T00:00:00Z",
        "source": "test",
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
            "risk_class": 1,
            "candidate": true,
            "runtime_truth": false
        },
        "integrity": {
            "payload_hash": "sha256:filled-by-append",
            "envelope_hash": "sha256:filled-by-append"
        }
    })
}

fn append(
    store: &Path,
    event_id: &str,
    event_type: &str,
    previous: Option<String>,
    payload: Value,
) -> String {
    append_event(
        store,
        AppendInput {
            previous_record_hash: previous.clone(),
            envelope: envelope(event_id, event_type, previous.as_deref()),
            payload,
        },
    )
    .expect("append should succeed")
    .record_hash
}

fn task_payload(atom_id: &str, status: &str, pr_number: Option<u64>) -> Value {
    let mut value = json!({
        "atom_id": atom_id,
        "title": format!("Task {atom_id}"),
        "phase": "V5-LOOP",
        "lane": "devtool",
        "risk_class": 1,
        "priority": "P0",
        "status": status,
        "self_select": true,
        "meta_opened": true,
        "claim_mode": "open_pool",
        "claim_required": true,
        "claim_method": "draft_pr",
        "required_capabilities": ["harness"],
        "preferred_capabilities": [],
        "allowed_files": ["docs/allowed.md"],
        "forbidden_files": ["src/runtime/**"],
        "task_packet": "docs/harness/broadcast/tasks/test.json",
        "acceptance_criteria": ["git diff --check"],
        "duplicate_policy": "first_valid_pr_wins",
        "blockers": []
    });
    if let Some(number) = pr_number {
        value["pr_number"] = json!(number);
    }
    value
}

fn seed_task(
    store: &Path,
    atom_id: &str,
    status: &str,
    pr_number: Option<u64>,
    previous: Option<String>,
) -> String {
    let created = append(
        store,
        &format!("{atom_id}-created"),
        "DevTaskCreated",
        previous,
        task_payload(atom_id, status, pr_number),
    );
    append(
        store,
        &format!("{atom_id}-broadcasted"),
        "TaskBroadcasted",
        Some(created),
        json!({"atom_id": atom_id}),
    )
}

fn write_json(path: &Path, value: &Value) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent should be created");
    }
    fs::write(
        path,
        serde_json::to_vec_pretty(value).expect("json should serialize"),
    )
    .expect("json should be written");
}

#[test]
fn loop_once_derives_board_and_appends_reconcile_followup_events() {
    let dir = temp_path("followups");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let board = dir.join("TASK_BOARD.json");
    let prs = dir.join("prs.json");

    let tip = seed_task(&store, "V5-LOOP-FAILED-CI", "pr_open", Some(10), None);
    let tip = seed_task(
        &store,
        "V5-LOOP-MISSING-REPORT",
        "claimed",
        Some(11),
        Some(tip),
    );
    let tip = seed_task(&store, "V5-LOOP-BEHIND", "pr_open", Some(12), Some(tip));
    let _tip = seed_task(&store, "V5-LOOP-DUPLICATE", "open", None, Some(tip));

    write_json(
        &prs,
        &json!([
            {
                "number": 10,
                "title": "[CLAIM][V5-LOOP-FAILED-CI][Class1] Failed CI",
                "isDraft": false,
                "createdAt": "2026-05-20T01:00:00Z",
                "url": "https://github.com/gretjia/turingosv5/pull/10",
                "body": "ClaimRecord\nWorkerReport\n[WORKER_HALT]",
                "mergeStateStatus": "BEHIND",
                "statusCheckRollup": [{"name": "ci-basic", "conclusion": "FAILURE"}]
            },
            {
                "number": 11,
                "title": "[CLAIM][V5-LOOP-MISSING-REPORT][Class1] Missing report",
                "isDraft": false,
                "createdAt": "2026-05-20T01:01:00Z",
                "url": "https://github.com/gretjia/turingosv5/pull/11",
                "body": "ClaimRecord",
                "mergeStateStatus": "BEHIND",
                "statusCheckRollup": [{"name": "ci-basic", "conclusion": "SUCCESS"}]
            },
            {
                "number": 12,
                "title": "[CLAIM][V5-LOOP-BEHIND][Class1] Behind",
                "isDraft": false,
                "createdAt": "2026-05-20T01:02:00Z",
                "url": "https://github.com/gretjia/turingosv5/pull/12",
                "body": "ClaimRecord\nWorkerReport\n[WORKER_HALT]",
                "mergeStateStatus": "BEHIND",
                "statusCheckRollup": [{"name": "ci-basic", "conclusion": "SUCCESS"}]
            },
            {
                "number": 13,
                "title": "[CLAIM][V5-LOOP-DUPLICATE][Class1] First duplicate",
                "isDraft": false,
                "createdAt": "2026-05-20T01:03:00Z",
                "url": "https://github.com/gretjia/turingosv5/pull/13",
                "body": "ClaimRecord",
                "mergeStateStatus": "CLEAN",
                "statusCheckRollup": []
            },
            {
                "number": 14,
                "title": "[CLAIM][V5-LOOP-DUPLICATE][Class1] Second duplicate",
                "isDraft": true,
                "createdAt": "2026-05-20T01:04:00Z",
                "url": "https://github.com/gretjia/turingosv5/pull/14",
                "body": "ClaimRecord",
                "mergeStateStatus": "CLEAN",
                "statusCheckRollup": []
            }
        ]),
    );

    let output = Command::new(bin())
        .args([
            "loop",
            "once",
            "--store",
            store.to_str().expect("utf8 store"),
            "--board-out",
            board.to_str().expect("utf8 board"),
            "--prs-file",
            prs.to_str().expect("utf8 prs"),
        ])
        .output()
        .expect("loop once should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(board.exists());
    let board_projection: Value =
        serde_json::from_slice(&fs::read(&board).expect("board should read")).expect("board json");
    assert_eq!(board_projection["source"], "devtape_derived");

    let records = read_records(&store).expect("records should read");
    let event_types: Vec<String> = records
        .iter()
        .map(|record| {
            record.envelope["event_type"]
                .as_str()
                .unwrap_or("")
                .to_string()
        })
        .collect();
    assert!(event_types
        .iter()
        .any(|event| event == "MetaReconcileRecorded"));
    assert!(event_types.iter().any(|event| event == "RepairTaskCreated"));
    assert!(event_types
        .iter()
        .any(|event| event == "WorkerFollowupRequested"));
    assert!(event_types
        .iter()
        .any(|event| event == "BranchUpdateRequested"));
    assert!(event_types
        .iter()
        .any(|event| event == "DuplicateClaimRecorded"));
    assert!(!event_types.iter().any(|event| event == "PRMerged"));
}

#[test]
fn loop_once_records_pr_merged_and_rederives_board_projection() {
    let dir = temp_path("pr-merged");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let board = dir.join("TASK_BOARD.json");
    let prs = dir.join("prs.json");
    let atom = "V5-LOOP-MERGED";
    seed_task(&store, atom, "pr_open", Some(36), None);
    write_json(
        &prs,
        &json!([
            {
                "number": 36,
                "title": "[CLAIM][V5-LOOP-MERGED][Class1] Worker claim",
                "state": "MERGED",
                "isDraft": false,
                "createdAt": "2026-05-20T07:55:44Z",
                "mergedAt": "2026-05-20T08:01:33Z",
                "url": "https://github.com/gretjia/turingosv5/pull/36",
                "body": "ClaimRecord\n- atom_id: V5-LOOP-MERGED\n\nWorkerReport\n[WORKER_HALT]",
                "mergeStateStatus": "UNKNOWN",
                "mergeCommit": {"oid": "59f284a9b1905851f104f9bf2ec512ce11ee33ca"},
                "statusCheckRollup": [{"name": "ci-basic", "conclusion": "SUCCESS"}]
            }
        ]),
    );

    let output = Command::new(bin())
        .args([
            "loop",
            "once",
            "--store",
            store.to_str().expect("utf8 store"),
            "--board-out",
            board.to_str().expect("utf8 board"),
            "--prs-file",
            prs.to_str().expect("utf8 prs"),
        ])
        .output()
        .expect("loop once should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let records = read_records(&store).expect("records should read");
    let pr_merged = records
        .iter()
        .find(|record| record.envelope["event_type"] == "PRMerged")
        .expect("loop once must append PRMerged evidence");
    assert_eq!(pr_merged.payload["atom_id"], atom);
    assert_eq!(pr_merged.payload["pr_number"], 36);
    assert_eq!(
        pr_merged.payload["merge_commit_sha"],
        "59f284a9b1905851f104f9bf2ec512ce11ee33ca"
    );

    let board_projection: Value =
        serde_json::from_slice(&fs::read(&board).expect("board should read")).expect("board json");
    let row = board_projection["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .find(|task| task["atom_id"] == atom)
        .expect("merged task row");
    assert_eq!(row["status"], "merged");
    assert_eq!(row["pr_number"], 36);
    assert_eq!(
        row["main_after"],
        "59f284a9b1905851f104f9bf2ec512ce11ee33ca"
    );
    assert!(
        row["source_event_cids"]
            .as_array()
            .expect("source cids")
            .iter()
            .any(|cid| cid == &json!(pr_merged.record_hash)),
        "derived board must cite PRMerged record hash"
    );
}

#[test]
fn loop_once_overwrites_manual_board_projection_from_devtape() {
    let dir = temp_path("board-rederive");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let board = dir.join("TASK_BOARD.json");
    let prs = dir.join("prs.json");
    seed_task(&store, "V5-LOOP-OPEN", "open", None, None);
    write_json(
        &board,
        &json!({"source": "manual", "tasks": [{"atom_id": "BOGUS"}]}),
    );
    write_json(&prs, &json!([]));

    let output = Command::new(bin())
        .args([
            "loop",
            "once",
            "--store",
            store.to_str().expect("utf8 store"),
            "--board-out",
            board.to_str().expect("utf8 board"),
            "--prs-file",
            prs.to_str().expect("utf8 prs"),
        ])
        .output()
        .expect("loop once should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let board_projection: Value =
        serde_json::from_slice(&fs::read(&board).expect("board should read")).expect("board json");
    assert_eq!(board_projection["source"], "devtape_derived");
    assert_eq!(board_projection["tasks"][0]["atom_id"], "V5-LOOP-OPEN");
}
