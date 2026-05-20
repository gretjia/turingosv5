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
        "turingosv5-meta-run-{name}-{}-{nanos}",
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

fn seed_task(store: &Path, atom_id: &str) {
    let created = append(
        store,
        "task-created",
        "DevTaskCreated",
        None,
        json!({
            "atom_id": atom_id,
            "title": "Meta run smoke task",
            "phase": "V5-META",
            "lane": "devtool",
            "risk_class": 1,
            "priority": "P0",
            "status": "open",
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
        }),
    );
    append(
        store,
        "task-broadcasted",
        "TaskBroadcasted",
        Some(created),
        json!({"atom_id": atom_id}),
    );
}

#[test]
fn meta_run_once_records_deepseek_candidate_observation() {
    let dir = temp_path("deepseek-candidate");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let board = dir.join("TASK_BOARD.json");
    let prs = dir.join("prs.json");
    let model_response = dir.join("deepseek-response.txt");
    seed_task(&store, "V5-META-RUN-SMOKE");
    fs::write(&prs, "[]").expect("prs fixture should write");
    fs::write(
        &model_response,
        r#"{"verdict":"OBSERVE","summary":"board and PR scan complete"}"#,
    )
    .expect("model response should write");

    let output = Command::new(bin())
        .args([
            "meta",
            "run",
            "--store",
            store.to_str().expect("utf8 store"),
            "--board-out",
            board.to_str().expect("utf8 board"),
            "--iterations",
            "1",
            "--interval-ms",
            "0",
            "--prs-file",
            prs.to_str().expect("utf8 prs"),
            "--meta-adapter",
            "deepseek",
            "--model-response-file",
            model_response.to_str().expect("utf8 model response"),
        ])
        .output()
        .expect("meta run should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout: Value =
        serde_json::from_slice(&output.stdout).expect("meta run stdout should be JSON");
    assert_eq!(stdout["mode"], "meta_run");
    assert_eq!(stdout["iterations_completed"], 1);
    assert_eq!(stdout["model_adapter"], "deepseek");
    assert!(board.exists(), "meta run must materialize board projection");

    let records = read_records(&store).expect("records should read");
    let reconcile = records
        .iter()
        .find(|record| record.envelope["event_type"] == "MetaReconcileRecorded")
        .expect("meta run must append MetaReconcileRecorded");
    assert_eq!(reconcile.payload["trigger"], "meta_run");
    let observation = &reconcile.payload["model_observation"];
    assert_eq!(observation["adapter"], "deepseek");
    assert_eq!(observation["model"], "deepseek-v4-pro");
    assert_eq!(observation["candidate"], true);
    assert_eq!(observation["runtime_truth"], false);
    assert!(observation["content"]
        .as_str()
        .expect("content")
        .contains("OBSERVE"));
    assert!(
        !serde_json::to_string(observation)
            .expect("json")
            .contains("test-secret-value"),
        "model observation must not store API key values"
    );
}

#[test]
fn deepseek_meta_adapter_uses_timeout_and_does_not_put_key_in_process_args() {
    let source = fs::read_to_string("src/bin/turingos-dev.rs").expect("dev cli source should read");
    assert!(
        source.contains("--max-time"),
        "DeepSeek adapter must bound provider calls with curl --max-time"
    );
    assert!(
        source.contains("--config") && source.contains("deepseek-curl-config"),
        "DeepSeek adapter must pass Authorization through a private curl config file"
    );
    assert!(
        !source.contains("Authorization: Bearer $DEEPSEEK_API_KEY"),
        "DeepSeek API key must not appear in curl process arguments"
    );
}

#[test]
fn meta_run_records_provider_error_without_killing_reconcile_loop() {
    let dir = temp_path("provider-error");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let board = dir.join("TASK_BOARD.json");
    let prs = dir.join("prs.json");
    let meta_config = dir.join("missing-provider-profiles.json");
    seed_task(&store, "V5-META-RUN-PROVIDER-ERROR");
    fs::write(&prs, "[]").expect("prs fixture should write");

    let output = Command::new(bin())
        .env_remove("DEEPSEEK_API_KEY")
        .env("TURINGOS_HOME", dir.join("empty-home"))
        .args([
            "meta",
            "run",
            "--store",
            store.to_str().expect("utf8 store"),
            "--board-out",
            board.to_str().expect("utf8 board"),
            "--iterations",
            "1",
            "--interval-ms",
            "0",
            "--prs-file",
            prs.to_str().expect("utf8 prs"),
            "--meta-config",
            meta_config.to_str().expect("utf8 config"),
            "--meta-adapter",
            "deepseek",
        ])
        .output()
        .expect("meta run should execute");

    assert!(
        output.status.success(),
        "provider failure should become evidence, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let records = read_records(&store).expect("records should read");
    let reconcile = records
        .iter()
        .find(|record| record.envelope["event_type"] == "MetaReconcileRecorded")
        .expect("meta run must still append MetaReconcileRecorded");
    let observation = &reconcile.payload["model_observation"];
    assert_eq!(observation["adapter"], "deepseek");
    assert_eq!(observation["status"], "error");
    assert_eq!(observation["candidate"], true);
    assert_eq!(observation["runtime_truth"], false);
}
