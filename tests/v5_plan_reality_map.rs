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
fn canonical_plan_is_saved_with_required_v08_concepts() {
    let plan = read_text("docs/v5_dev/V5_DEVKERNEL_PLAN_v0.8.md");
    assert!(plan.contains("Canonical local development plan"));
    for term in ["DevEventEnvelope", "AgentIdentity", "BootstrapException"] {
        assert!(plan.contains(term), "plan missing {term}");
    }

    let readme = read_text("docs/v5_dev/README.md");
    assert!(readme.contains("V5_DEVKERNEL_PLAN_v0.8.md` is the canonical architecture plan for"));
}

#[test]
fn reality_map_has_classification_table_and_anchor_sections() {
    let map = read_text("docs/v5_dev/V4_NATIVE_REALITY_MAP.md");
    assert!(map.contains("## Classification Table"));
    assert!(map.contains("| Capability | Classification | Evidence status |"));
    for class in [
        "usable_now",
        "usable_with_adapter",
        "not_usable_yet",
        "do_not_use",
    ] {
        assert!(map.contains(class), "reality map missing class {class}");
    }
    assert!(map.contains("Code anchors:"));
    assert!(map.contains("Test anchors:"));
    assert!(map.contains("Assumption:"));
}
