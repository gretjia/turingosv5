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
fn preview_run_schema_requires_truth_path_links() {
    let schema = read_json("schemas/v5_dev/preview_run.schema.json");
    let required = required_fields(&schema);

    for field in [
        "preview_run_cid",
        "artifact_bundle_cid",
        "redaction_result_cid",
    ] {
        assert!(required.contains(&field), "missing required field {field}");
    }

    assert_eq!(schema["title"], "PreviewRunCapsule");
    assert_eq!(schema["properties"]["kind"]["const"], "PreviewRunCapsule");
    assert_eq!(schema["additionalProperties"], false);
}

#[test]
fn preview_run_contract_defines_read_only_artifact_bundle_render() {
    let doc = read_text("docs/contracts/preview_run.md");

    assert!(doc.contains("PreviewRunCapsule"));
    assert!(doc.contains("read-only"));
    assert!(doc.contains("ArtifactBundle CID"));
    assert!(doc.contains("artifact_bundle_cid"));
    assert!(doc.contains("redaction_result_cid"));
}

#[test]
fn preview_run_contract_rejects_file_path_truth_and_raw_prompt_logs() {
    let doc = read_text("docs/contracts/preview_run.md");

    assert!(!doc.contains("arbitrary file path"));
    assert!(!doc.contains("raw logs with prompt"));
    assert!(!doc.contains("raw logs and prompt"));
}
