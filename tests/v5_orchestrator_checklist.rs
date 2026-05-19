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
    let text = read_text(path);
    serde_json::from_str(&text).expect("schema must be valid JSON")
}

#[test]
fn orchestrator_checklist_names_all_required_evidence() {
    let doc = read_text("docs/v5_dev/ORCHESTRATOR_EVIDENCE_CHECKLIST.md");
    for term in [
        "Task exists",
        "Role assignment evidence exists",
        "WorkerReport exists",
        "PRCreated exists",
        "CIResult exists",
        "ReviewVerdict exists when required",
        "VetoVerdict exists when required",
        "MergeDecisionAccepted exists before merge",
        "GitHub gates pass",
        "No author final-audit",
        "No forbidden files touched",
        "Class 4 ratified when touched",
    ] {
        assert!(doc.contains(term), "missing checklist term {term}");
    }
}

#[test]
fn merge_decision_template_references_dev_event_cids() {
    let template = read_text("docs/harness/templates/MergeDecision.md");
    for term in [
        "task_event_cid",
        "worker_report_event_cid",
        "pr_created_event_cid",
        "ci_result_event_cid",
        "merge_decision_event_cid",
    ] {
        assert!(
            template.contains(term),
            "missing MergeDecision CID field {term}"
        );
    }
}

#[test]
fn merge_decision_template_exposes_schema_required_gate_fields() {
    let template = read_text("docs/harness/templates/MergeDecision.md");
    for term in [
        "author_final_audit",
        "forbidden_files_touched",
        "class4_touched",
        "class4_ratified_when_touched",
    ] {
        assert!(
            template.contains(term),
            "MergeDecision template missing schema gate field {term}"
        );
    }
    assert!(!template.contains("FORBIDDEN_FILES_TOUCHED:"));
    assert!(!template.contains("CLASS4_TOUCHED:"));
}

#[test]
fn merge_decision_schema_accepts_checklist_cid_fields() {
    let schema_text = read_text("schemas/v5_dev/merge_decision.schema.json");
    for term in [
        "task_event_cid",
        "role_assignment_event_cid",
        "worker_report_event_cid",
        "pr_created_event_cid",
        "ci_result_event_cid",
        "review_verdict_event_cid",
        "veto_verdict_event_cid",
        "branch_protection_snapshot_cid",
        "merge_decision_event_cid",
        "bootstrap_exception_event_cid",
    ] {
        assert!(
            schema_text.contains(term),
            "MergeDecision schema missing CID field {term}"
        );
    }
}

#[test]
fn merge_decision_cid_fields_are_nonempty_and_shape_constrained() {
    let schema = read_json("schemas/v5_dev/merge_decision.schema.json");
    for field in [
        "task_event_cid",
        "role_assignment_event_cid",
        "worker_report_event_cid",
        "pr_created_event_cid",
        "ci_result_event_cid",
        "review_verdict_event_cid",
        "veto_verdict_event_cid",
        "branch_protection_snapshot_cid",
        "merge_decision_event_cid",
    ] {
        let field_schema = &schema["properties"][field];
        assert_eq!(field_schema["type"], "string", "{field} must be a string");
        assert_eq!(
            field_schema["minLength"], 8,
            "{field} must reject empty CID values"
        );
        assert!(
            field_schema["pattern"].as_str().is_some(),
            "{field} must constrain CID/hash shape"
        );
    }

    let bootstrap = &schema["properties"]["bootstrap_exception_event_cid"];
    assert_eq!(bootstrap["anyOf"][0]["type"], "string");
    assert_eq!(bootstrap["anyOf"][0]["minLength"], 8);
    assert!(bootstrap["anyOf"][0]["pattern"].as_str().is_some());
    assert_eq!(bootstrap["anyOf"][1]["type"], "null");
}

#[test]
fn checklist_blocks_provider_authority_and_class4_shortcuts() {
    let doc = read_text("docs/v5_dev/ORCHESTRATOR_EVIDENCE_CHECKLIST.md");
    assert!(doc.contains("provider label is not authority"));
    assert!(doc.contains("Class 4 ratified when touched"));
    assert!(doc.contains("branch protection satisfied"));
}
