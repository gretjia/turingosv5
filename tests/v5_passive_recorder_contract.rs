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

#[test]
fn passive_recorder_mvp_does_not_claim_active_governance() {
    let doc = read_text("docs/v5_dev/PASSIVE_RECORDER_MVP.md");
    assert!(doc.contains("records events"));
    assert!(doc.contains("does not block merge"));
    assert!(doc.contains("does not claim V4-native governance"));
    assert!(doc.contains("V4D-1 Passive Recorder"));
}

#[test]
fn passive_recorder_declares_minimal_command_targets() {
    let doc = read_text("docs/v5_dev/PASSIVE_RECORDER_MVP.md");
    for command in [
        "turingos dev event append --envelope event.json",
        "turingos dev audit --project turingosv5",
        "turingos dev board derive",
    ] {
        assert!(doc.contains(command), "missing command target {command}");
    }
}

#[test]
fn active_gate_wording_requires_merge_decision_accepted() {
    let doc = read_text("docs/v5_dev/PASSIVE_RECORDER_MVP.md");
    assert!(doc.contains("Active Merge Gate requires MergeDecisionAccepted"));
    assert!(doc.contains("V4D-1 must not state that V4 controls merge"));
}

#[test]
fn passive_recorder_templates_exist() {
    let envelope = read_text("docs/harness/templates/DevEventEnvelope.md");
    let identity = read_text("docs/harness/templates/AgentIdentity.md");
    assert!(envelope.contains("actor_identity_cid"));
    assert!(envelope.contains("payload_cid"));
    assert!(identity.contains("role_assignment_cid"));
    assert!(identity.contains("provider_label"));
}
