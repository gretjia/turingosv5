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
        lower.contains("new artifactbundle"),
        "contract must require a new ArtifactBundle on regenerate"
    );
    assert!(
        lower.contains("old artifact"),
        "contract must mention previewability of old artifacts"
    );
}

#[test]
fn edit_regenerate_contract_must_not_demand_overwrite() {
    let doc = fs::read_to_string("docs/contracts/edit_regenerate_versioning.md")
        .expect("edit_regenerate_versioning contract document must exist");
    let lower = doc.to_lowercase();

    for forbidden in ["overwrite existing", "patch file", "in place"] {
        assert!(
            !lower.contains(forbidden),
            "contract must not mandate forbidden behavior: {forbidden}"
        );
    }
}
