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
fn portable_karpathy_skill_exists_with_v5_constraints() {
    let skill = read_text("docs/agent_skills/KARPATHY_SIMPLE_CODE.md");
    for term in [
        "Think Before Coding",
        "Surgical Changes",
        "Compression Without Obscurity",
        "TuringOS V5 Rules",
        "Do not create a new canonical substrate",
        "Do not add dependencies",
        "Do not copy old kernel code",
    ] {
        assert!(skill.contains(term), "portable skill missing `{term}`");
    }
}

#[test]
fn portable_skill_is_provider_neutral() {
    let skill = read_text("docs/agent_skills/KARPATHY_SIMPLE_CODE.md");
    for forbidden in ["Codex", "Claude", "Gemini", "OpenAI", "Anthropic", "Google"] {
        assert!(
            !skill.contains(forbidden),
            "portable skill must not bind to provider label {forbidden}"
        );
    }
}

#[test]
fn native_codex_skill_is_valid_when_installed_locally() {
    let Some(home) = std::env::var_os("HOME") else {
        return;
    };
    let skill_dir = PathBuf::from(home)
        .join(".codex")
        .join("skills")
        .join("karpathy-simple-code");
    let skill_path = skill_dir.join("SKILL.md");
    if !skill_path.exists() {
        return;
    }

    let skill = read_abs(&skill_path);
    assert!(skill.contains("name: karpathy-simple-code"));
    assert!(skill.contains("description: Use when writing or reviewing code"));
    assert!(skill.contains("No new canonical substrate"));
    assert!(skill.contains("added_dependency:"));
    assert!(
        !skill_dir.join("README.md").exists(),
        "native skill must not include extra README.md"
    );
}

#[test]
fn entry_documents_wire_karpathy_skill_without_provider_role_binding() {
    for path in ["AGENTS.md", "AGENT_ENTRY.md"] {
        let text = read_text(path);
        assert!(
            text.contains("docs/agent_skills/KARPATHY_SIMPLE_CODE.md"),
            "{path} must route implementation/review work through the portable skill"
        );
        for forbidden in ["Codex", "Claude", "Gemini", "OpenAI", "Anthropic", "Google"] {
            assert!(
                !text.contains(forbidden),
                "{path} must not bind skill usage to provider label {forbidden}"
            );
        }
    }
}

#[test]
fn task_packet_template_requires_karpathy_skill_and_style() {
    let template = read_text("docs/harness/templates/TaskPacket.md");
    for term in [
        "Required Skill",
        "docs/agent_skills/KARPATHY_SIMPLE_CODE.md",
        "Follow Karpathy Simple Code",
        "No new dependency",
        "No broad abstraction",
        "No neighboring refactor",
        "No clever compression that hurts readability",
    ] {
        assert!(
            template.contains(term),
            "TaskPacket template missing `{term}`"
        );
    }
}

#[test]
fn worker_report_template_requires_simplicity_self_check() {
    let template = read_text("docs/harness/templates/WorkerReport.md");
    for term in [
        "KARPATHY_SIMPLE_CODE_CHECK",
        "ADDED_DEPENDENCY",
        "ADDED_ABSTRACTION",
        "ABSTRACTION_REASON",
        "LARGEST_NEW_FUNCTION_LINES",
        "CHANGED_ONLY_ALLOWED_FILES",
        "SMALLEST_SOLUTION_REASON",
    ] {
        assert!(
            template.contains(term),
            "WorkerReport template missing `{term}`"
        );
    }
}

#[test]
fn orchestrator_checklist_audits_simplicity() {
    let checklist = read_text("docs/v5_dev/ORCHESTRATOR_EVIDENCE_CHECKLIST.md");
    for term in [
        "Simplicity Checks",
        "dependency was added without explicit task permission",
        "broad abstraction was added without a real boundary",
        "manager/factory/engine naming",
        "compressed code into unclear cleverness",
        "old-system code was copied instead of isolated behind a port/fixture",
    ] {
        assert!(checklist.contains(term), "checklist missing `{term}`");
    }
}
