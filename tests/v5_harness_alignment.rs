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

fn index_of(text: &str, needle: &str) -> usize {
    text.find(needle)
        .unwrap_or_else(|| panic!("missing expected text: {needle}"))
}

#[test]
fn worker_read_order_starts_from_global_entry_then_generic_entry_then_role_entry() {
    let worker = read_text("docs/harness/WORKER_HARNESS.md");
    let agents = index_of(&worker, "`AGENTS.md`");
    let entry = index_of(&worker, "`AGENT_ENTRY.md`");
    let role = index_of(&worker, "`docs/harness/roles/WORKER_ENTRY.md`");
    let harness = index_of(&worker, "`docs/harness/WORKER_HARNESS.md`");

    assert!(
        agents < entry,
        "AGENTS.md must be read before AGENT_ENTRY.md"
    );
    assert!(entry < role, "generic entry must be read before role entry");
    assert!(
        role < harness,
        "role entry must be read before worker harness"
    );
}

#[test]
fn unassigned_entry_blocks_merge_but_explicit_meta_can_merge_after_gates() {
    let entry = read_text("AGENT_ENTRY.md");
    assert!(entry.contains("Unassigned intake sessions must not merge PRs."));
    assert!(entry
        .contains("Explicit MetaAI role sessions may merge only after all required gates pass."));
    assert!(!entry.contains("Never merge PR."));
}

#[test]
fn harness_is_intake_layer_not_worker_execution_manager() {
    let readme = read_text("README.md");
    let harness_readme = read_text("docs/harness/README.md");
    let entry = read_text("AGENT_ENTRY.md");
    let meta_entry = read_text("docs/harness/roles/META_ENTRY.md");
    let meta_harness = read_text("docs/harness/META_HARNESS.md");
    let worker_prompt = read_text("docs/harness/boot_prompts/universal_worker.md");

    for text in [&readme, &harness_readme, &entry] {
        assert!(
            text.contains("CLI intake layer"),
            "top-level harness docs must define harness as intake layer"
        );
        assert!(
            text.contains("not the V5 kernel")
                || text.contains("not an execution manager, scheduler, kernel"),
            "harness docs must not present harness as kernel/runtime authority"
        );
    }

    for text in [&readme, &meta_entry, &meta_harness] {
        let normalized = text.replace('\n', " ");
        assert!(
            text.contains("board-first"),
            "Meta task distribution must be board-first"
        );
        assert!(
            normalized.contains("private worker-specific")
                || normalized.contains("private MetaAI execution instructions"),
            "Meta must not replace board flow with private worker execution instructions"
        );
    }

    assert!(
        worker_prompt.contains("Do not wait for private MetaAI execution instructions"),
        "worker boot prompt must direct workers to the board, not private Meta instructions"
    );
    assert!(
        !harness_readme.contains("turns multi-CLI development into candidate state transitions"),
        "harness README must not overstate the harness as a state-transition system"
    );
}

#[test]
fn meta_harness_distinguishes_passive_recorder_from_active_gate() {
    let meta = read_text("docs/harness/META_HARNESS.md");
    assert!(meta.contains("V4D-1 Passive Recorder"));
    assert!(meta.contains("V4D-2 Active Merge Gate"));
    assert!(meta.contains("does not claim V4 controls merge"));
    assert!(meta.contains("MergeDecisionAccepted"));
}

#[test]
fn harness_alignment_keeps_provider_neutral_roles() {
    for path in [
        "AGENT_ENTRY.md",
        "docs/harness/WORKER_HARNESS.md",
        "docs/harness/META_HARNESS.md",
        "docs/harness/roles/META_ENTRY.md",
        "docs/harness/roles/WORKER_ENTRY.md",
    ] {
        let text = read_text(path);
        for forbidden in ["Codex", "Claude", "Gemini", "OpenAI", "Anthropic", "Google"] {
            assert!(
                !text.contains(forbidden),
                "{path} must not bind roles to provider label {forbidden}"
            );
        }
    }
}
