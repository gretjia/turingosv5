use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use turingosv5::devtool::{append_event, derive_board, read_records, AppendInput};

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turingosv5-worker-claim-next-{name}-{}-{nanos}",
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

fn write_json(path: &Path, value: &Value) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent should be created");
    }
    fs::write(
        path,
        serde_json::to_vec_pretty(value).expect("json should serialize"),
    )
    .expect("json file should be written");
}

fn task_payload(atom_id: &str, task_packet: &str) -> Value {
    json!({
        "atom_id": atom_id,
        "title": "Worker claim next fixture",
        "phase": "V5-CLAIM",
        "lane": "devtool",
        "risk_class": 1,
        "priority": "P0",
        "status": "open",
        "self_select": true,
        "meta_opened": true,
        "claim_mode": "open_pool",
        "claim_required": true,
        "claim_method": "sandbox",
        "required_capabilities": ["docs"],
        "preferred_capabilities": [],
        "allowed_files": ["docs/allowed.md"],
        "forbidden_files": ["src/runtime/**"],
        "task_packet": task_packet,
        "acceptance_criteria": ["git diff --check"],
        "duplicate_policy": "first_valid_claim_wins",
        "blockers": []
    })
}

fn seed_open_task(store: &Path, repo: &Path, atom_id: &str) {
    fs::create_dir_all(repo.join("docs")).expect("repo docs should be created");
    fs::write(repo.join("docs/allowed.md"), "before\n").expect("allowed file should exist");
    let task_packet = format!("docs/harness/broadcast/tasks/{atom_id}.r1.task.json");
    write_json(
        &repo.join(&task_packet),
        &task_payload(atom_id, &task_packet),
    );
    let created = append(
        store,
        "claim-next-created",
        "DevTaskCreated",
        None,
        task_payload(atom_id, &task_packet),
    );
    append(
        store,
        "claim-next-broadcasted",
        "TaskBroadcasted",
        Some(created),
        json!({"atom_id": atom_id}),
    );
}

#[test]
fn worker_claim_next_creates_sandbox_from_devtape_board_and_records_claim() {
    let dir = temp_path("claim");
    let store = dir.join("events.jsonl");
    let repo = dir.join("repo");
    let out_root = dir.join("sandboxes");
    let atom = "V5-CLAIM-NEXT-001";
    seed_open_task(&store, &repo, atom);

    let output = Command::new(bin())
        .args([
            "worker",
            "claim",
            "next",
            "--store",
            store.to_str().expect("utf8 store"),
            "--repo",
            repo.to_str().expect("utf8 repo"),
            "--out-root",
            out_root.to_str().expect("utf8 out root"),
            "--worker",
            "worker-a",
        ])
        .output()
        .expect("claim next should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"decision\": \"CLAIMED\""));
    assert!(stdout.contains(atom));
    let sandbox = out_root.join("worker-a").join(atom);
    assert!(sandbox.join("TASK.md").exists());
    assert!(sandbox.join("CONTEXT.md").exists());
    assert!(sandbox.join("allowed_files/docs/allowed.md").exists());

    let records = read_records(&store).expect("records should read");
    let claim = records
        .iter()
        .find(|record| record.envelope["event_type"] == "TaskClaimed")
        .expect("claim event should be appended");
    assert_eq!(claim.payload["atom_id"], atom);
    assert_eq!(claim.payload["claim_method"], "sandbox");
    assert_eq!(claim.payload["worker_slot"], "worker-a");

    let board = derive_board(&store).expect("board should derive after claim");
    let row = board["tasks"]
        .as_array()
        .expect("tasks array")
        .iter()
        .find(|task| task["atom_id"] == atom)
        .expect("task row should exist");
    assert_eq!(row["status"], "claimed");
}

#[test]
fn worker_claim_next_returns_no_eligible_task_without_creating_sandbox() {
    let dir = temp_path("none");
    fs::create_dir_all(&dir).expect("temp dir should exist");
    let store = dir.join("events.jsonl");
    let repo = dir.join("repo");
    let out_root = dir.join("sandboxes");
    fs::create_dir_all(&repo).expect("repo dir should exist");

    let output = Command::new(bin())
        .args([
            "worker",
            "claim",
            "next",
            "--store",
            store.to_str().expect("utf8 store"),
            "--repo",
            repo.to_str().expect("utf8 repo"),
            "--out-root",
            out_root.to_str().expect("utf8 out root"),
            "--worker",
            "worker-a",
        ])
        .output()
        .expect("claim next should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"decision\": \"NO_ELIGIBLE_TASK\""));
    assert!(!out_root.exists());
}
