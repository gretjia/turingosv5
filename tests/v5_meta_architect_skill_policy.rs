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

fn read_abs(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

#[test]
fn portable_architect_skill_exists_with_required_architecture_protocol() {
    let skill = read_text("docs/agent_skills/KARPATHY_ARCHITECT.md");
    for term in [
        "Core Illusion",
        "Data Flow Layout",
        "Micro-Implementation",
        "Data Shapes > Logic",
        "Monolithic & Flat by Default",
        "TuringOS V5 Architecture Rules",
        "board is a derived view",
        "DevEvents/Tape are development facts",
    ] {
        assert!(skill.contains(term), "architect skill missing `{term}`");
    }
}

#[test]
fn portable_architect_skill_is_provider_neutral() {
    let skill = read_text("docs/agent_skills/KARPATHY_ARCHITECT.md");
    for forbidden in ["Codex", "Claude", "Gemini", "OpenAI", "Anthropic", "Google"] {
        assert!(
            !skill.contains(forbidden),
            "portable architect skill must not bind to provider label {forbidden}"
        );
    }
}

#[test]
fn native_architect_skill_is_valid_when_installed_locally() {
    let Some(home) = std::env::var_os("HOME") else {
        return;
    };
    let skill_dir = PathBuf::from(home)
        .join(".codex")
        .join("skills")
        .join("karpathy-architect");
    let skill_path = skill_dir.join("SKILL.md");
    if !skill_path.exists() {
        return;
    }

    let skill = read_abs(&skill_path);
    assert!(skill.contains("name: karpathy-architect"));
    assert!(skill.contains("description: Use when acting as MetaAI"));
    assert!(skill.contains("Core Illusion"));
    assert!(
        !skill_dir.join("README.md").exists(),
        "native architect skill must not include extra README.md"
    );
}

#[test]
fn architect_skill_is_meta_only_and_not_worker_routed() {
    let meta_entry = read_text("docs/harness/roles/META_ENTRY.md");
    let meta_harness = read_text("docs/harness/META_HARNESS.md");
    let worker_entry = read_text("docs/harness/roles/WORKER_ENTRY.md");
    let task_packet = read_text("docs/harness/templates/TaskPacket.md");
    let worker_report = read_text("docs/harness/templates/WorkerReport.md");

    assert!(meta_entry.contains("docs/agent_skills/KARPATHY_ARCHITECT.md"));
    assert!(meta_harness.contains("Architecture Discipline"));
    assert!(!worker_entry.contains("KARPATHY_ARCHITECT.md"));
    assert!(!task_packet.contains("KARPATHY_ARCHITECT.md"));
    assert!(!worker_report.contains("KARPATHY_ARCHITECT"));
}

#[test]
fn orchestrator_checklist_requires_architecture_review() {
    let checklist = read_text("docs/v5_dev/ORCHESTRATOR_EVIDENCE_CHECKLIST.md");
    for term in [
        "Architecture Checks",
        "core data shape is named",
        "micro end-to-end model exists",
        "new infrastructure is justified by a physical bottleneck",
        "board/runtime truth boundary is preserved",
    ] {
        assert!(checklist.contains(term), "checklist missing `{term}`");
    }
}
