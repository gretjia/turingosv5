const INVENTORY: &str = include_str!("../docs/roadmap/C3_LLM_CALL_EVIDENCE_INVENTORY.md");

#[test]
fn inventory_names_required_llm_evidence_terms() {
    for term in [
        "PromptCapsule",
        "AttemptTelemetry",
        "EvidenceTuple",
        "no naked LLM call",
    ] {
        assert!(
            INVENTORY.contains(term),
            "inventory must name required LLM evidence term: {term}"
        );
    }
}

