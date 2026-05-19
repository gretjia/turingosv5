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
        .expect("schema must have required array")
        .iter()
        .map(|value| value.as_str().expect("required fields must be strings"))
        .collect()
}

fn assert_required(schema: &Value, fields: &[&str]) {
    let required = required_fields(schema);
    for field in fields {
        assert!(required.contains(field), "missing required field {field}");
    }
}

fn enum_values<'a>(schema: &'a Value, field: &str) -> Vec<&'a str> {
    schema["properties"][field]["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("{field} must define enum values"))
        .iter()
        .map(|value| value.as_str().expect("enum values must be strings"))
        .collect()
}

#[test]
fn v5_dev_event_schemas_are_valid_json() {
    let schema_paths = [
        "schemas/v5_dev/dev_event_envelope.schema.json",
        "schemas/v5_dev/agent_identity.schema.json",
        "schemas/v5_dev/dev_task_created.schema.json",
        "schemas/v5_dev/worker_report_submitted.schema.json",
        "schemas/v5_dev/pr_created.schema.json",
        "schemas/v5_dev/ci_result_recorded.schema.json",
        "schemas/v5_dev/review_verdict_submitted.schema.json",
        "schemas/v5_dev/veto_verdict_submitted.schema.json",
        "schemas/v5_dev/merge_decision.schema.json",
        "schemas/v5_dev/pr_merged.schema.json",
        "schemas/v5_dev/bootstrap_exception.schema.json",
        "schemas/v5_dev/branch_protection_snapshot.schema.json",
    ];

    for path in schema_paths {
        let schema = read_json(path);
        assert_eq!(
            schema["type"], "object",
            "{path} must define an object schema"
        );
    }
}

#[test]
fn dev_event_envelope_is_required_for_payload_authority() {
    let schema = read_json("schemas/v5_dev/dev_event_envelope.schema.json");
    assert_required(
        &schema,
        &[
            "event_id",
            "event_type",
            "project_id",
            "actor_identity_cid",
            "payload_cid",
            "previous_event_cid",
            "observed_at",
            "source",
        ],
    );

    let event_types = enum_values(&schema, "event_type");
    for event_type in [
        "DevTaskCreated",
        "WorkerReportSubmitted",
        "PRCreated",
        "CIResultRecorded",
        "ReviewVerdictSubmitted",
        "VetoVerdictSubmitted",
        "MergeDecisionAccepted",
        "MergeDecisionRejected",
        "PRMerged",
    ] {
        assert!(
            event_types.contains(&event_type),
            "envelope must include {event_type}"
        );
    }
}

#[test]
fn review_verdict_is_a_first_class_dev_event_payload() {
    let envelope = read_json("schemas/v5_dev/dev_event_envelope.schema.json");
    let event_types = enum_values(&envelope, "event_type");
    assert!(
        event_types.contains(&"ReviewVerdictSubmitted"),
        "review evidence must be representable in DevEventEnvelope"
    );

    let review = read_json("schemas/v5_dev/review_verdict_submitted.schema.json");
    assert_required(
        &review,
        &[
            "pr_number",
            "verdict",
            "reviewer_identity_cid",
            "author_identity_cid",
            "author_is_reviewer",
            "review_body_cid",
        ],
    );
    assert_eq!(
        review["properties"]["author_is_reviewer"]["const"], false,
        "review schema must reject author self-audit"
    );
}

#[test]
fn review_verdict_vocabulary_matches_harness_review_packet() {
    let review = read_json("schemas/v5_dev/review_verdict_submitted.schema.json");
    let verdict_values = enum_values(&review, "verdict");
    assert_eq!(verdict_values, vec!["PASS", "HOLD", "VETO"]);

    let review_packet_template = read_text("docs/harness/templates/ReviewPacket.md");
    assert!(review_packet_template.contains("PASS | HOLD | VETO"));
    assert!(!review_packet_template.contains("PROCEED | CHALLENGE | VETO"));

    let review_packet_schema = read_json("docs/harness/schemas/review_packet.schema.json");
    let required_format = review_packet_schema["properties"]["required_verdict_format"]["enum"]
        .as_array()
        .expect("ReviewPacket schema must constrain required_verdict_format");
    let required_format: Vec<&str> = required_format
        .iter()
        .map(|value| value.as_str().expect("verdict format must be string"))
        .collect();
    assert_eq!(required_format, vec!["PASS | HOLD | VETO"]);
}

#[test]
fn agent_identity_requires_role_assignment_evidence() {
    let schema = read_json("schemas/v5_dev/agent_identity.schema.json");
    assert_required(
        &schema,
        &[
            "actor_id",
            "declared_role",
            "role_assignment_source",
            "role_assignment_ref",
            "role_assignment_cid",
            "assigned_by_actor_id",
            "provider_label",
            "runtime_label",
            "capabilities",
            "started_at",
        ],
    );

    let declared_roles = enum_values(&schema, "declared_role");
    for role in [
        "MetaAI",
        "WorkerAI",
        "AuditorAI",
        "VetoAI",
        "HumanArchitect",
    ] {
        assert!(declared_roles.contains(&role), "missing role {role}");
    }
}

#[test]
fn provider_label_is_not_authority() {
    let schema = read_json("schemas/v5_dev/agent_identity.schema.json");
    let description = schema["properties"]["provider_label"]["description"]
        .as_str()
        .expect("provider_label must explain its authority boundary");
    assert!(
        description.contains("provenance only") && description.contains("not authority"),
        "provider_label must be provenance only, not authority"
    );

    let contract = read_text("docs/contracts/v5_dev_events.md");
    assert!(contract.contains("provider_label is provenance only"));
    assert!(contract.contains("A provider label never grants role authority"));
}

#[test]
fn veto_and_merge_payloads_reject_invalid_authority_shapes() {
    let veto = read_json("schemas/v5_dev/veto_verdict_submitted.schema.json");
    let verdict_values = enum_values(&veto, "verdict");
    assert_eq!(verdict_values, vec!["PASS", "VETO"]);

    let merge = read_json("schemas/v5_dev/merge_decision.schema.json");
    assert_required(&merge, &["pr_number", "decision", "required_ci_passed"]);

    let contract = read_text("docs/contracts/v5_dev_events.md");
    assert!(contract.contains("Payload without DevEventEnvelope is invalid"));
    assert!(contract.contains("declared_role without role_assignment_cid is invalid"));
}

#[test]
fn dev_event_envelope_schema_matches_canonical_plan_surfaces() {
    let schema = read_json("schemas/v5_dev/dev_event_envelope.schema.json");
    assert_required(
        &schema,
        &["subject", "evidence", "classification", "integrity"],
    );

    let classification_required = schema["properties"]["classification"]["required"]
        .as_array()
        .expect("classification must define required fields");
    let classification_required: Vec<&str> = classification_required
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("classification fields must be strings")
        })
        .collect();
    for field in ["risk_class", "candidate", "runtime_truth"] {
        assert!(
            classification_required.contains(&field),
            "classification missing {field}"
        );
    }
    assert_eq!(
        schema["properties"]["classification"]["properties"]["runtime_truth"]["const"],
        false
    );

    let integrity_required = schema["properties"]["integrity"]["required"]
        .as_array()
        .expect("integrity must define required fields");
    let integrity_required: Vec<&str> = integrity_required
        .iter()
        .map(|value| value.as_str().expect("integrity fields must be strings"))
        .collect();
    for field in ["payload_hash", "envelope_hash"] {
        assert!(
            integrity_required.contains(&field),
            "integrity missing {field}"
        );
    }
}

#[test]
fn bootstrap_exception_schema_requires_v4_v5_boundary_evidence() {
    let schema = read_json("schemas/v5_dev/bootstrap_exception.schema.json");
    assert_required(
        &schema,
        &[
            "exact_v4_anchors",
            "v5_files_affected",
            "risk_class",
            "replacement_path",
            "meta_decision",
        ],
    );
    assert_eq!(schema["properties"]["risk_class"]["maximum"], 4);
    assert_eq!(schema["properties"]["restoration_required"]["const"], true);
}
