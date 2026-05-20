use std::fs;

#[test]
fn edit_regenerate_contract_requires_new_artifact_bundle() {
    let doc = fs::read_to_string("docs/contracts/edit_regenerate_versioning.md")
        .expect("edit_regenerate_versioning contract document must exist");
    let lower = doc.to_lowercase();

    assert!(
        lower.contains("modificationrequestcapsule"),
        "contract must mention ModificationRequestCapsule"
    );
    assert!(
        lower.contains("new artifactbundle") || lower.contains("new artifact bundle"),
        "contract must require a new ArtifactBundle on regenerate"
    );
    assert!(
        lower.contains("old artifact"),
        "contract must mention previewability of old artifacts"
    );
}

#[test]
fn edit_regenerate_contract_requires_non_destructive_prohibitions() {
    let doc = fs::read_to_string("docs/contracts/edit_regenerate_versioning.md")
        .expect("edit_regenerate_versioning contract document must exist");
    let lower = doc.to_lowercase();

    for required in [
        "must not replace prior artifacts",
        "must not mutate prior artifact records",
        "must not discard historical preview references",
    ] {
        assert!(
            lower.contains(required),
            "contract must explicitly prohibit destructive behavior: {required}"
        );
    }
}
