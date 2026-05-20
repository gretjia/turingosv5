use std::fs;

#[test]
fn friendly_error_contract_declares_reject_class_and_l4e() {
    let doc = fs::read_to_string("docs/contracts/friendly_error_l4e.md")
        .expect("friendly_error_l4e contract document must exist");
    assert!(
        doc.contains("RejectClass"),
        "contract must define RejectClass"
    );
    assert!(
        doc.contains("L4.E"),
        "contract must define L4.E rejection lane"
    );
    assert!(
        doc.contains("world_head"),
        "contract must state world_head behavior"
    );
}

#[test]
fn friendly_error_contract_forbids_user_exposure_of_internal_details() {
    let doc = fs::read_to_string("docs/contracts/friendly_error_l4e.md")
        .expect("friendly_error_l4e contract document must exist");
    for forbidden in ["panic stack", "prompt user", "secret user"] {
        assert!(
            !doc.to_lowercase().contains(&forbidden.to_lowercase()),
            "contract must not include forbidden pattern {forbidden}"
        );
    }
}
