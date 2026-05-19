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
fn derived_board_schema_requires_devkernel_source_and_event_cids() {
    let schema = read_json("schemas/v5_dev/derived_task_board.schema.json");
    let required = required_fields(&schema);
    for field in [
        "schema",
        "board_version",
        "generated_at",
        "generated_by_role",
        "source",
        "source_event_cids",
        "tasks",
    ] {
        assert!(required.contains(&field), "missing required field {field}");
    }

    assert_eq!(
        schema["properties"]["source"]["const"],
        "v4_devkernel_derived"
    );
    assert_eq!(schema["properties"]["generated_by_role"]["const"], "MetaAI");
}

#[test]
fn task_projection_requires_source_event_cids() {
    let schema = read_json("schemas/v5_dev/derived_task_board.schema.json");
    let task_required = schema["properties"]["tasks"]["items"]["required"]
        .as_array()
        .expect("task projection must define required fields");
    let task_required: Vec<&str> = task_required
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("task required fields must be strings")
        })
        .collect();
    assert!(task_required.contains(&"atom_id"));
    assert!(task_required.contains(&"status"));
    assert!(task_required.contains(&"source_event_cids"));
}

#[test]
fn derived_board_contract_marks_manual_source_as_drift() {
    let doc = read_text("docs/v5_dev/DERIVED_TASK_BOARD_CONTRACT.md");
    assert!(doc.contains("board is rebuildable"));
    assert!(doc.contains("manual mutation is drift"));
    assert!(doc.contains("board does not mutate DevKernel state"));
    assert!(doc.contains("source = v4_devkernel_derived"));
    assert!(doc.contains("manual source is invalid for V4D-Q1"));
}

#[test]
fn current_h0_task_board_is_not_rewritten_by_v4d_q1_contract() {
    let board = read_json("docs/harness/broadcast/TASK_BOARD.json");
    assert!(
        board["board_version"].as_str().is_some(),
        "existing control board must still parse"
    );
}
