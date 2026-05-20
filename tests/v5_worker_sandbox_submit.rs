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
        "turingosv5-worker-sandbox-submit-{name}-{}-{nanos}",
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

fn run_git(repo: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git {:?} stderr: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
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
        "title": "Submit sandbox patch",
        "phase": "V5-SUBMIT",
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

fn setup_claimed_sandbox(name: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf, String) {
    let dir = temp_path(name);
    let store = dir.join("events.jsonl");
    let repo = dir.join("repo");
    let sandbox_root = dir.join("sandboxes");
    let atom = format!("V5-SUBMIT-{name}-001");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    fs::create_dir_all(repo.join("docs")).expect("repo docs should be created");
    fs::write(repo.join("docs/allowed.md"), "before\n").expect("allowed file should exist");
    run_git(&dir, &["init", "repo"]);
    run_git(&repo, &["add", "docs/allowed.md"]);
    run_git(
        &repo,
        &[
            "-c",
            "user.name=TuringOS Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-m",
            "Initial fixture",
        ],
    );

    let task_packet = format!("docs/harness/broadcast/tasks/{atom}.r1.task.json");
    write_json(&repo.join(&task_packet), &task_payload(&atom, &task_packet));
    let created = append(
        &store,
        "submit-created",
        "DevTaskCreated",
        None,
        task_payload(&atom, &task_packet),
    );
    append(
        &store,
        "submit-broadcasted",
        "TaskBroadcasted",
        Some(created),
        json!({"atom_id": atom}),
    );

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
            sandbox_root.to_str().expect("utf8 sandbox root"),
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
    let sandbox = sandbox_root.join("worker-a").join(&atom);
    (dir, store, repo, sandbox, atom)
}

#[test]
fn worker_sandbox_submit_commits_patch_in_isolated_worktree_and_records_report() {
    let (dir, store, repo, sandbox, atom) = setup_claimed_sandbox("commit");
    fs::write(
        sandbox.join("submit/candidate.patch"),
        "diff --git a/docs/allowed.md b/docs/allowed.md\n--- a/docs/allowed.md\n+++ b/docs/allowed.md\n@@ -1 +1 @@\n-before\n+after\n",
    )
    .expect("patch should be written");
    fs::write(
        sandbox.join("submit/WorkerReport.json"),
        r#"{"worker_halt_confirmation":"[WORKER_HALT]","tests_run":["git diff --check"]}"#,
    )
    .expect("report should be written");

    let worktree_root = dir.join("worktrees");
    let output = Command::new(bin())
        .args([
            "worker",
            "sandbox",
            "submit",
            "--dir",
            sandbox.to_str().expect("utf8 sandbox"),
            "--store",
            store.to_str().expect("utf8 store"),
            "--repo",
            repo.to_str().expect("utf8 repo"),
            "--worktree-root",
            worktree_root.to_str().expect("utf8 worktree root"),
            "--worker",
            "worker-a",
        ])
        .output()
        .expect("sandbox submit should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"decision\": \"SUBMITTED\""));
    assert!(stdout.contains(&atom));

    let worktree = worktree_root.join("worker-a").join(&atom);
    assert_eq!(
        fs::read_to_string(worktree.join("docs/allowed.md")).expect("patched file should read"),
        "after\n"
    );
    let commit = Command::new("git")
        .arg("-C")
        .arg(&worktree)
        .args(["log", "-1", "--pretty=%s"])
        .output()
        .expect("git log should run");
    assert!(String::from_utf8_lossy(&commit.stdout).contains(&atom));

    let records = read_records(&store).expect("records should read");
    let report = records
        .iter()
        .find(|record| record.envelope["event_type"] == "WorkerReportSubmitted")
        .expect("WorkerReportSubmitted should be recorded");
    assert_eq!(report.payload["atom_id"], atom);
    assert_eq!(report.payload["worker_slot"], "worker-a");
    assert_eq!(report.payload["submission_commit_created"], true);
    assert_eq!(report.payload["pr_creation"], "not_requested");

    let board = derive_board(&store).expect("board should derive");
    let row = board["tasks"]
        .as_array()
        .expect("tasks array")
        .iter()
        .find(|task| task["atom_id"] == atom)
        .expect("task row should exist");
    assert_eq!(row["status"], "submitted");
    assert_eq!(row["pr_number"], Value::Null);
}

#[test]
fn worker_sandbox_submit_requires_existing_task_claim() {
    let dir = temp_path("no-claim");
    let store = dir.join("events.jsonl");
    let repo = dir.join("repo");
    let sandbox = dir.join("sandbox");
    fs::create_dir_all(repo.join("docs")).expect("repo docs should be created");
    fs::write(repo.join("docs/allowed.md"), "before\n").expect("allowed file should exist");
    write_json(
        &dir.join("task.json"),
        &task_payload("V5-SUBMIT-NO-CLAIM-001", "task.json"),
    );
    let output = Command::new(bin())
        .args([
            "worker",
            "sandbox",
            "create",
            "--task",
            dir.join("task.json").to_str().expect("utf8 task"),
            "--repo",
            repo.to_str().expect("utf8 repo"),
            "--out",
            sandbox.to_str().expect("utf8 sandbox"),
        ])
        .output()
        .expect("sandbox create should run");
    assert!(output.status.success());
    fs::write(
        sandbox.join("submit/candidate.patch"),
        "diff --git a/docs/allowed.md b/docs/allowed.md\n--- a/docs/allowed.md\n+++ b/docs/allowed.md\n@@ -1 +1 @@\n-before\n+after\n",
    )
    .expect("patch should be written");
    fs::write(
        sandbox.join("submit/WorkerReport.json"),
        r#"{"worker_halt_confirmation":"[WORKER_HALT]"}"#,
    )
    .expect("report should be written");

    let output = Command::new(bin())
        .args([
            "worker",
            "sandbox",
            "submit",
            "--dir",
            sandbox.to_str().expect("utf8 sandbox"),
            "--store",
            store.to_str().expect("utf8 store"),
            "--repo",
            repo.to_str().expect("utf8 repo"),
            "--worktree-root",
            dir.join("worktrees").to_str().expect("utf8 worktree root"),
            "--worker",
            "worker-a",
        ])
        .output()
        .expect("sandbox submit should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing TaskClaimed"));
    assert!(!dir.join("worktrees").exists());
}
