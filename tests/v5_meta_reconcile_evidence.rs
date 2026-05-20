use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use turingosv5::devtool::read_records;

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turingosv5-meta-reconcile-{name}-{}-{nanos}",
        std::process::id()
    ))
}

fn bin() -> String {
    std::env::var("CARGO_BIN_EXE_turingos-dev")
        .expect("cargo should expose turingos-dev binary path")
}

fn write_json(path: &PathBuf, value: &Value) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent should be created");
    }
    fs::write(
        path,
        serde_json::to_vec_pretty(value).expect("json should serialize"),
    )
    .expect("json file should be written");
}

#[test]
fn meta_reconcile_append_records_actions_as_devtape_evidence() {
    let dir = temp_path("append");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let board = dir.join("board.json");
    let prs = dir.join("prs.json");

    write_json(
        &board,
        &json!({
            "tasks": [
                {
                    "atom_id": "V5-K1-C2-NO-NEW-SUBSTRATE-REGRESSION-001",
                    "status": "pr_open",
                    "pr_number": 10
                }
            ]
        }),
    );
    write_json(
        &prs,
        &json!([
            {
                "number": 10,
                "title": "[CLAIM][V5-K1-C2-NO-NEW-SUBSTRATE-REGRESSION-001][Class1] No substrate",
                "isDraft": false,
                "createdAt": "2026-05-20T01:49:27Z",
                "url": "https://github.com/gretjia/turingosv5/pull/10",
                "body": "ClaimRecord\nWorkerReport\n[WORKER_HALT]",
                "mergeStateStatus": "BEHIND",
                "statusCheckRollup": [
                    {"name": "ci-basic", "conclusion": "FAILURE"}
                ]
            }
        ]),
    );

    let output = Command::new(bin())
        .args([
            "meta",
            "reconcile",
            "--append",
            "--store",
            store.to_str().expect("utf8 store"),
            "--board",
            board.to_str().expect("utf8 board"),
            "--prs-file",
            prs.to_str().expect("utf8 prs"),
        ])
        .output()
        .expect("meta reconcile append should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let records = read_records(&store).expect("records should read");
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.envelope["event_type"], "MetaReconcileRecorded");
    assert_eq!(record.envelope["classification"]["runtime_truth"], false);
    assert_eq!(record.payload["mode"], "append");
    assert_eq!(
        record.payload["report"]["actions"][0]["action"],
        "hold_failed_ci"
    );
}

#[test]
fn meta_reconcile_requires_explicit_dry_run_or_append_mode() {
    let dir = temp_path("mode-required");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let board = dir.join("board.json");
    let prs = dir.join("prs.json");
    write_json(&board, &json!({"tasks": []}));
    write_json(&prs, &json!([]));

    let output = Command::new(bin())
        .args([
            "meta",
            "reconcile",
            "--board",
            board.to_str().expect("utf8 board"),
            "--prs-file",
            prs.to_str().expect("utf8 prs"),
        ])
        .output()
        .expect("meta reconcile should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--dry-run or --append"));
}
