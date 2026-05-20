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
    assert_eq!(board["source"], "devtape_derived");
    assert!(board["source_event_cids"]
        .as_array()
        .is_some_and(|events| !events.is_empty()));
    let tasks = board["tasks"].as_array().expect("tasks must be an array");
    for atom_id in [
        "V5-K0-C0-REALITY-MAP-HARD-GATE-001",
        "V5-K0-C1-PATH-DECISION-CHRONOLOGY-001",
        "V5-K1-C2-NO-NEW-SUBSTRATE-REGRESSION-001",
        "V5-K1-C3-LLM-CALL-EVIDENCE-INVENTORY-001",
        "V5-K2-C4-ARTIFACT-BUNDLE-CONTRACT-001",
        "V5-K3-C5-PREVIEW-TRUTH-PATH-CONTRACT-001",
        "V5-K4-C6-BUILD-SESSION-DERIVED-VIEW-001",
        "V5-K4-C7-FRIENDLY-ERROR-L4E-CONTRACT-001",
        "V5-K4-C8-SINGLE-URL-MVP-CONTRACT-001",
        "V5-K5-C9-EDIT-REGENERATE-VERSIONING-001",
        "V5-K6-C10-SPEC-DERIVED-TESTRUN-001",
        "V5-K7-C11-AUDIT-PACKET-CONTRACT-001",
    ] {
        let task = tasks
            .iter()
            .find(|task| task["atom_id"] == atom_id)
            .unwrap_or_else(|| panic!("board missing {atom_id}"));
        assert_eq!(task["revision"], 1);
        assert_eq!(task["claim_required"], true);
        assert_eq!(task["claim_method"], "draft_pr");
        assert!(task["source_event_cids"]
            .as_array()
            .is_some_and(|events| events.len() >= 2));
        assert!(task["task_packet"]
            .as_str()
            .is_some_and(|path| path.contains(atom_id)));
    }
}

#[test]
fn task_board_keeps_architect_skill_meta_only() {
    let board = read_json("docs/harness/broadcast/TASK_BOARD.json");
    let tasks = board["tasks"].as_array().expect("tasks must be an array");
    let c0 = tasks
        .iter()
        .find(|task| task["atom_id"] == "V5-K0-C0-REALITY-MAP-HARD-GATE-001")
        .expect("C0 must exist");
    assert_eq!(c0["self_select"], true);
    assert_eq!(c0["claim_mode"], "open_pool");
    assert_eq!(c0["required_capabilities"][0], "docs");

    let task_packet_template = read_text("docs/harness/templates/TaskPacket.md");
    assert!(!task_packet_template.contains("KARPATHY_ARCHITECT.md"));
    assert!(task_packet_template.contains("KARPATHY_SIMPLE_CODE.md"));

    let task_packet =
        read_text("docs/harness/broadcast/tasks/V5-K0-C0-REALITY-MAP-HARD-GATE-001.r1.task.json");
    assert!(!task_packet.contains("KARPATHY_ARCHITECT.md"));
    assert!(task_packet.contains("KARPATHY_SIMPLE_CODE.md"));
}
