use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(path: impl AsRef<Path>) -> String {
    let path = repo_root().join(path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_str(&read_text(path)).expect("file must be valid JSON")
}

#[test]
fn devtape_v1_execution_plan_is_materialized() {
    let plan = read_text("docs/v5_dev/DEVTAPE_V1_EXECUTION_PLAN.md");
    for term in [
        "TuringOS V5 DevTape v1.0",
        "DevEvent[] -> projections -> decisions",
        "A0: Architecture Pin",
        "A4: Micro DevTape Proof",
        "A11: Orchestrator Audit + Veto",
        "Spark WorkerAI",
        "KARPATHY_SIMPLE_CODE.md",
        "KARPATHY_ARCHITECT.md",
    ] {
        assert!(plan.contains(term), "v1.0 plan missing `{term}`");
    }
}

#[test]
fn core_dev_flow_names_architectural_data_shapes() {
    let flow = read_text("docs/v5_dev/CORE_DEV_FLOW.md");
    for term in [
        "Core Illusion",
        "Data Flow Layout",
        "Micro-Implementation",
        "DevEvent[]",
        "BoardProjection",
        "MergeDecisionCandidate",
        "runtime_truth: false",
        "V5 runtime",
    ] {
        assert!(flow.contains(term), "core dev flow missing `{term}`");
    }
}

#[test]
fn task_board_contains_devtape_v1_wave() {
    let board = read_json("docs/harness/broadcast/TASK_BOARD.json");
    assert_eq!(board["source"], "TuringOS V5 DevTape v1.0 final preflight");
    let tasks = board["tasks"].as_array().expect("tasks must be an array");
    for atom_id in [
        "V5-SYS-A0-ARCH-PIN-001",
        "V5-SYS-A1-BASELINE-SEMANTIC-CLOSE-001",
        "V5-SYS-A2-WORKBENCH-BOUNDARY-001",
        "V5-SYS-A3-CHARACTERIZATION-SEEDS-001",
        "V5-SYS-A4-MICRO-DEVTAPE-001",
        "V5-SYS-A5-DERIVED-BOARD-PROJECTOR-001",
        "V5-SYS-A6-CLI-INTAKE-WRAPPER-001",
        "V5-SYS-A7-REUSE-PORT-CONTRACT-001",
        "V5-SYS-A8-GITHUB-EVIDENCE-SNAPSHOT-001",
        "V5-SYS-A9-ACTIVE-MERGE-GATE-001",
        "V5-SYS-A10-HARNESS-THINNING-001",
        "V5-SYS-A11-AUDIT-VETO-001",
    ] {
        let task = tasks
            .iter()
            .find(|task| task["atom_id"] == atom_id)
            .unwrap_or_else(|| panic!("board missing {atom_id}"));
        assert_eq!(task["phase"], "V5-SYS");
        assert_eq!(task["revision"], 1);
        assert!(task["task_packet"]
            .as_str()
            .is_some_and(|path| path.contains(atom_id)));
    }
}

#[test]
fn task_board_keeps_architect_skill_meta_only() {
    let board = read_json("docs/harness/broadcast/TASK_BOARD.json");
    let tasks = board["tasks"].as_array().expect("tasks must be an array");
    let a0 = tasks
        .iter()
        .find(|task| task["atom_id"] == "V5-SYS-A0-ARCH-PIN-001")
        .expect("A0 must exist");
    assert_eq!(a0["self_select"], false);
    assert_eq!(a0["claim_mode"], "direct_assignment");
    assert_eq!(a0["required_capabilities"][0], "architecture");

    let task_packet_template = read_text("docs/harness/templates/TaskPacket.md");
    assert!(!task_packet_template.contains("KARPATHY_ARCHITECT.md"));
    assert!(task_packet_template.contains("KARPATHY_SIMPLE_CODE.md"));
}
