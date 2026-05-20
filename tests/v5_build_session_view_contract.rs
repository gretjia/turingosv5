use std::fs;

#[test]
fn build_session_view_is_a_rebuildable_derived_projection() {
    let doc = fs::read_to_string("docs/contracts/build_session_view.md")
        .expect("BuildSessionView contract doc must exist");
    let lower = doc.to_lowercase();

    for required in [
        "derived view",
        "projection",
        "delete cache",
        "rebuild",
        "source inputs",
    ] {
        assert!(
            lower.contains(required),
            "BuildSessionView contract must mention {required:?}"
        );
    }

    for forbidden in [
        "session source of truth",
        "cache is canonical",
        "canonical cache",
    ] {
        assert!(
            !lower.contains(forbidden),
            "BuildSessionView contract must not make session/cache canonical"
        );
    }
}
