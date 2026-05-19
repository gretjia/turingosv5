use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_json(path: impl AsRef<Path>) -> Value {
    let path = path.as_ref();
    let text = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", path.display());
    });
    serde_json::from_str(&text).unwrap_or_else(|err| {
        panic!("failed to parse {} as JSON: {err}", path.display());
    })
}

#[test]
fn harness_agent_entry_and_brand_adapters_point_to_shared_agents() {
    let root = repo_root();
    let entry = root.join("AGENT_ENTRY.md");
    let agents = root.join("AGENTS.md");

    assert!(entry.exists(), "AGENT_ENTRY.md must exist as the single CLI entry point");
    assert!(agents.exists(), "AGENTS.md must exist as the shared cross-agent contract");

    let entry_text = fs::read_to_string(&entry).expect("AGENT_ENTRY.md should be readable");
    assert!(
        entry_text.contains("Read `AGENTS.md`") || entry_text.contains("Read `AGENTS.md`."),
        "AGENT_ENTRY.md must route all workers through AGENTS.md"
    );
    assert!(
        entry_text.contains("docs/harness/broadcast/TASK_BOARD.json"),
        "AGENT_ENTRY.md must point workers at TASK_BOARD.json"
    );

    for adapter in ["CLAUDE.md", "GEMINI.md", "CODEX.md"] {
        let path = root.join(adapter);
        assert!(path.exists(), "{adapter} must exist");
        let text = fs::read_to_string(&path).unwrap_or_else(|err| {
            panic!("failed to read {adapter}: {err}");
        });
        assert!(
            text.contains("AGENTS.md") && text.contains("AGENT_ENTRY.md"),
            "{adapter} must import the shared AGENTS.md and AGENT_ENTRY.md boundary"
        );
    }
}

#[test]
fn harness_task_board_declares_meta_only_writer_and_valid_packets() {
    let root = repo_root();
    let board_path = root.join("docs/harness/broadcast/TASK_BOARD.json");
    let board = read_json(&board_path);

    assert_eq!(board["board_version"], "v0.7");
    assert_eq!(board["board_writer"], "meta-only");
    assert_eq!(
        board["runtime_boundary"]["development_control_plane_only"],
        true,
        "TASK_BOARD must be development control plane only"
    );

    let tasks = board["tasks"]
        .as_array()
        .expect("TASK_BOARD.tasks must be an array");
    assert!(!tasks.is_empty(), "TASK_BOARD must publish at least one bootstrap task");

    for task in tasks {
        let atom_id = task["atom_id"].as_str().expect("task atom_id missing");
        let class = task["class"].as_u64().expect("task class missing");
        let self_select = task["self_select"]
            .as_bool()
            .expect("task self_select missing");
        if class == 4 {
            assert!(!self_select, "Class 4 task {atom_id} must never be self-selectable");
        }
        if class == 3 && self_select {
            assert_eq!(
                task["meta_opened"].as_bool(),
                Some(true),
                "Class 3 task {atom_id} may self-select only when meta_opened is true"
            );
        }

        let packet_rel = task["task_packet"]
            .as_str()
            .unwrap_or_else(|| panic!("task {atom_id} missing task_packet"));
        let packet_path = root.join(packet_rel);
        assert!(
            packet_path.exists(),
            "task {atom_id} references missing packet {}",
            packet_path.display()
        );
        let packet = read_json(&packet_path);
        assert_eq!(packet["atom_id"], task["atom_id"]);
        assert_eq!(packet["revision"], task["revision"]);
        assert!(
            packet["acceptance_tests"].as_array().is_some_and(|tests| !tests.is_empty()),
            "task packet {atom_id} must declare visible acceptance tests"
        );
    }
}

#[test]
fn harness_runtime_does_not_read_agent_broadcast() {
    let root = repo_root();
    let output = Command::new("rg")
        .args([
            "-n",
            "AGENT_ENTRY|TASK_BOARD|docs/harness/broadcast",
            "src",
        ])
        .current_dir(&root)
        .output()
        .expect("rg should be available");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.trim().is_empty(),
        "V5 runtime must not read harness broadcast/control-plane files:\n{stdout}"
    );
}

#[test]
fn harness_v5_does_not_carry_minif2f_development_dataset() {
    let root = repo_root();
    assert!(
        !root.join("experiments/minif2f_v4").exists(),
        "MiniF2F is a V4 development/eval corpus, not a V5 product asset"
    );
}
