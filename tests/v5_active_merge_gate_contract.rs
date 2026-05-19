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

fn required_fields(schema: &Value) -> Vec<&str> {
    schema["required"]
        .as_array()
        .expect("schema must define required fields")
        .iter()
        .map(|value| value.as_str().expect("required fields must be strings"))
        .collect()
}

#[test]
fn merge_gate_schema_requires_all_external_gate_inputs() {
    let schema = read_json("schemas/v5_dev/merge_gate_check.schema.json");
    let required = required_fields(&schema);
    for field in [
        "merge_decision_cid",
        "pr_head_sha",
        "required_checks",
        "review_decision",
        "conversation_resolution",
        "branch_protection_snapshot_cid",
    ] {
        assert!(required.contains(&field), "missing required field {field}");
    }
}

#[test]
fn active_gate_contract_keeps_branch_protection_mandatory() {
    let doc = read_text("docs/v5_dev/ACTIVE_MERGE_GATE_CONTRACT.md");
    assert!(doc.contains("MergeDecisionAccepted is necessary but not sufficient"));
    assert!(doc.contains("GitHub branch protection remains mandatory"));
    assert!(doc.contains("accepted decision with failed CI is rejected"));
    assert!(doc.contains("accepted decision with unresolved conversations is rejected"));
}

#[test]
fn bootstrap_exception_requires_snapshot_and_restoration_evidence() {
    let doc = read_text("docs/v5_dev/ACTIVE_MERGE_GATE_CONTRACT.md");
    for event in [
        "BootstrapExceptionRequested",
        "BootstrapExceptionAccepted",
        "BranchProtectionSnapshotRecorded",
        "BootstrapExceptionRestored",
    ] {
        assert!(doc.contains(event), "missing bootstrap event {event}");
    }
    assert!(doc.contains("bootstrap exception without restoration evidence is rejected"));
}

#[test]
fn merge_gate_schema_models_conversation_resolution_and_checks() {
    let schema = read_json("schemas/v5_dev/merge_gate_check.schema.json");
    assert_eq!(
        schema["properties"]["conversation_resolution"]["type"],
        "boolean"
    );
    assert_eq!(schema["properties"]["required_checks"]["type"], "array");
    assert_eq!(
        schema["properties"]["bootstrap_exception"]["properties"]["restored_snapshot_cid"]["type"],
        "string"
    );
}

fn contains_const(value: &Value, needle: &str) -> bool {
    match value {
        Value::String(text) => text == needle,
        Value::Array(values) => values.iter().any(|value| contains_const(value, needle)),
        Value::Object(map) => map.values().any(|value| contains_const(value, needle)),
        _ => false,
    }
}

fn contains_property_const(value: &Value, field: &str, expected: bool) -> bool {
    match value {
        Value::Object(map) => {
            if let Some(properties) = map.get("properties").and_then(Value::as_object) {
                if properties
                    .get(field)
                    .and_then(|field_schema| field_schema.get("const"))
                    .and_then(Value::as_bool)
                    == Some(expected)
                {
                    return true;
                }
            }
            map.values()
                .any(|value| contains_property_const(value, field, expected))
        }
        Value::Array(values) => values
            .iter()
            .any(|value| contains_property_const(value, field, expected)),
        _ => false,
    }
}

#[test]
fn merge_decision_schema_constrains_proceed_to_passing_gates() {
    let schema = read_json("schemas/v5_dev/merge_decision.schema.json");
    assert!(
        contains_const(&schema["allOf"], "PROCEED"),
        "schema must have a PROCEED-specific conditional"
    );

    for (field, expected) in [
        ("required_ci_passed", true),
        ("review_passed", true),
        ("veto_passed", true),
        ("branch_protection_satisfied", true),
        ("conversation_resolution", true),
        ("forbidden_files_touched", false),
    ] {
        assert!(
            contains_property_const(&schema["allOf"], field, expected),
            "PROCEED gate missing conditional const for {field}"
        );
    }

    let schema_text = read_text("schemas/v5_dev/merge_decision.schema.json");
    assert!(
        schema_text.contains("\"class4_touched\"")
            && schema_text.contains("\"class4_ratified_when_touched\""),
        "schema must model Class 4 touched and ratification state"
    );
}
