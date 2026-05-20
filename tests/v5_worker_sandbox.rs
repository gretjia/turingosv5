use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turingosv5-worker-sandbox-{name}-{}-{nanos}",
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

fn task_packet() -> Value {
    json!({
        "atom_id": "V5-SANDBOX-001",
        "title": "Sandboxed worker slice",
        "goal": "Change only the allowed docs file.",
        "allowed_files": ["docs/allowed.md"],
        "forbidden_files": ["src/runtime/**", "secrets.env"],
        "acceptance_tests": ["git diff --check"],
        "step_by_step_instructions": ["Edit docs/allowed.md only."],
        "worker_halt_required": true
    })
}

fn create_sandbox(root_name: &str) -> (PathBuf, PathBuf) {
    let dir = temp_path(root_name);
    let repo = dir.join("repo");
    let sandbox = dir.join("sandbox");
    fs::create_dir_all(repo.join("docs")).expect("repo docs should be created");
    fs::create_dir_all(repo.join("src/runtime")).expect("repo runtime should be created");
    fs::write(repo.join("docs/allowed.md"), "before\n").expect("allowed file should be written");
    fs::write(repo.join("src/runtime/secret.rs"), "secret\n")
        .expect("secret file should be written");
    let task = dir.join("task.json");
    write_json(&task, &task_packet());

    let output = Command::new(bin())
        .args([
            "worker",
            "sandbox",
            "create",
            "--task",
            task.to_str().expect("utf8 task"),
            "--repo",
            repo.to_str().expect("utf8 repo"),
            "--out",
            sandbox.to_str().expect("utf8 sandbox"),
        ])
        .output()
        .expect("sandbox create should run");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    (dir, sandbox)
}

#[test]
fn worker_sandbox_create_exports_only_allowed_context_and_submit_contract() {
    let (_dir, sandbox) = create_sandbox("create");

    assert!(sandbox.join("TASK.md").exists());
    assert!(sandbox.join("CONTEXT.md").exists());
    assert!(sandbox.join("allowed_files/docs/allowed.md").exists());
    assert!(!sandbox.join("allowed_files/src/runtime/secret.rs").exists());
    assert!(sandbox.join("submit").is_dir());

    let task_text = fs::read_to_string(sandbox.join("TASK.md")).expect("task should read");
    let context_text = fs::read_to_string(sandbox.join("CONTEXT.md")).expect("context should read");
    assert!(task_text.contains("candidate.patch"));
    assert!(task_text.contains("WorkerReport.json"));
    assert!(task_text.contains("[WORKER_HALT]"));
    assert!(task_text.contains("soft sandbox"));
    assert!(context_text.contains("do not read the full repo"));
    assert!(context_text.contains("ask MetaAI for a richer context bundle"));

    let manifest: Value = serde_json::from_slice(
        &fs::read(sandbox.join("sandbox_manifest.json")).expect("manifest should exist"),
    )
    .expect("manifest should parse");
    assert_eq!(manifest["runtime_truth"], false);
    assert_eq!(manifest["allowed_files"], json!(["docs/allowed.md"]));
}

#[test]
fn worker_sandbox_validate_rejects_patch_outside_allowed_files() {
    let (_dir, sandbox) = create_sandbox("reject");
    fs::write(
        sandbox.join("submit/candidate.patch"),
        "diff --git a/docs/not_allowed.md b/docs/not_allowed.md\n--- a/docs/not_allowed.md\n+++ b/docs/not_allowed.md\n@@\n-before\n+changed\n",
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
            "validate",
            "--dir",
            sandbox.to_str().expect("utf8 sandbox"),
        ])
        .output()
        .expect("sandbox validate should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not in allowed_files"));
}

#[test]
fn worker_sandbox_validate_accepts_allowed_patch_and_halt_report() {
    let (_dir, sandbox) = create_sandbox("accept");
    fs::write(
        sandbox.join("submit/candidate.patch"),
        "diff --git a/docs/allowed.md b/docs/allowed.md\n--- a/docs/allowed.md\n+++ b/docs/allowed.md\n@@\n-before\n+after\n",
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
            "validate",
            "--dir",
            sandbox.to_str().expect("utf8 sandbox"),
        ])
        .output()
        .expect("sandbox validate should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("SANDBOX_SUBMISSION_PASS"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("not_run_by_sandbox_v0"));
}

#[test]
fn worker_sandbox_apply_writes_allowed_patch_into_isolated_worktree() {
    let (_dir, sandbox) = create_sandbox("apply");
    let worktree = temp_path("apply-worktree");
    fs::create_dir_all(worktree.join("docs")).expect("worktree docs should be created");
    fs::write(worktree.join("docs/allowed.md"), "before\n").expect("worktree file");
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
            "apply",
            "--dir",
            sandbox.to_str().expect("utf8 sandbox"),
            "--worktree",
            worktree.to_str().expect("utf8 worktree"),
        ])
        .output()
        .expect("sandbox apply should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(worktree.join("docs/allowed.md")).expect("patched file should read"),
        "after\n"
    );
    assert!(sandbox.join("submit/application_report.json").exists());
    assert!(String::from_utf8_lossy(&output.stdout).contains("SANDBOX_APPLY_PASS"));
}

#[test]
fn worker_sandbox_apply_refuses_forbidden_patch() {
    let (_dir, sandbox) = create_sandbox("apply-forbidden");
    let worktree = temp_path("apply-forbidden-worktree");
    fs::create_dir_all(worktree.join("docs")).expect("worktree docs should be created");
    fs::write(worktree.join("docs/allowed.md"), "before\n").expect("worktree file");
    fs::write(
        sandbox.join("submit/candidate.patch"),
        "diff --git a/constitution.md b/constitution.md\n--- a/constitution.md\n+++ b/constitution.md\n@@ -1 +1 @@\n-before\n+after\n",
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
            "apply",
            "--dir",
            sandbox.to_str().expect("utf8 sandbox"),
            "--worktree",
            worktree.to_str().expect("utf8 worktree"),
        ])
        .output()
        .expect("sandbox apply should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not in allowed_files"));
    assert!(!sandbox.join("submit/application_report.json").exists());
}

#[test]
fn turingos_dev_help_exits_successfully_for_worker_clients() {
    let output = Command::new(bin())
        .arg("--help")
        .output()
        .expect("help should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("worker sandbox create"));
    assert!(stdout.contains("worker sandbox validate"));
}
