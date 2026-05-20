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
        .map(|value| value.as_str().expect("required field must be string"))
        .collect()
}

#[test]
fn artifact_bundle_schema_is_valid_object_contract() {
    let schema = read_json("schemas/v5_dev/artifact_bundle.schema.json");

    assert_eq!(schema["title"], "ArtifactBundle");
    assert_eq!(schema["type"], "object");
    assert_eq!(schema["additionalProperties"], false);
}

#[test]
fn artifact_bundle_schema_requires_core_lineage_and_files() {
    let schema = read_json("schemas/v5_dev/artifact_bundle.schema.json");
    let required = required_fields(&schema);

    for field in [
        "artifact_bundle_cid",
        "spec_capsule_cid",
        "generation_attempt_cid",
        "files",
        "entrypoint",
    ] {
        assert!(required.contains(&field), "schema missing {field}");
        assert!(
            schema["properties"].get(field).is_some(),
            "schema must define property {field}"
        );
    }

    assert_eq!(schema["properties"]["files"]["type"], "array");
    assert_eq!(schema["properties"]["files"]["minItems"], 1);
}

#[test]
fn artifact_bundle_file_entries_are_cid_addressed() {
    let schema = read_json("schemas/v5_dev/artifact_bundle.schema.json");
    let file_item = &schema["properties"]["files"]["items"];
    let required = required_fields(file_item);

    for field in ["path", "content_cid", "media_type", "role"] {
        assert!(required.contains(&field), "file entry missing {field}");
        assert!(
            file_item["properties"].get(field).is_some(),
            "file entry must define property {field}"
        );
    }
}

#[test]
fn artifact_bundle_contract_names_boundaries_and_required_terms() {
    let contract = read_text("docs/contracts/artifact_bundle.md");

    for term in [
        "artifact_bundle_cid",
        "spec_capsule_cid",
        "generation_attempt_cid",
        "files",
        "entrypoint",
        "naked HTML string",
        "temporary directory truth",
    ] {
        assert!(contract.contains(term), "contract missing {term}");
    }
}
