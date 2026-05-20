use std::fs;

#[test]
fn single_url_mvp_contract_defines_ux_flow_and_refresh_restore() {
    let doc = fs::read_to_string("docs/contracts/single_url_mvp.md")
        .expect("Single URL MVP contract must exist");
    let lower = doc.to_lowercase();

    for required in [
        "/build",
        "spec -> generate -> preview",
        "refresh restore",
        "buildsessionview",
    ] {
        assert!(
            lower.contains(required),
            "Single URL MVP contract must mention {required:?}"
        );
    }
}

#[test]
fn single_url_mvp_contract_avoids_forbidden_stack_terms() {
    let doc = fs::read_to_string("docs/contracts/single_url_mvp.md")
        .expect("Single URL MVP contract must exist");
    let lower = doc.to_lowercase();

    for forbidden in ["next.js", "tauri", "microservice"] {
        assert!(
            !lower.contains(forbidden),
            "Single URL MVP contract must not mention {forbidden}"
        );
    }
}
