use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_json(path: impl AsRef<Path>) -> Value {
    let path = path.as_ref();
    let text = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", path.display());
    });
    serde_json::from_str(&text).unwrap_or_else(|err| {
        panic!("failed to parse {} as JSON: {err}", path.display());
    })
}

fn read_text(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", path.display());
    })
}

fn required_fields<'a>(schema: &'a Value, name: &str) -> &'a Vec<Value> {
    schema["required"]
        .as_array()
        .unwrap_or_else(|| panic!("{name} schema must declare required fields"))
}

fn collect_control_plane_refs(root: &Path, dir: &Path, matches: &mut Vec<String>) {
    if !dir.exists() {
        return;
    }

    for entry in fs::read_dir(dir).unwrap_or_else(|err| {
        panic!("failed to read directory {}: {err}", dir.display());
    }) {
        let entry = entry.expect("directory entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_control_plane_refs(root, &path, matches);
            continue;
        }

        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        if text.contains("AGENT_ENTRY")
            || text.contains("TASK_BOARD")
            || text.contains("docs/harness/broadcast")
        {
            let rel = path.strip_prefix(root).unwrap_or(&path);
            matches.push(rel.display().to_string());
        }
    }
}

fn assert_no_cli_role_assignment_language(path: &Path, text: &str) {
    let lower = text.to_ascii_lowercase();
    let forbidden = [
        "worker profile suggestion",
        "brand assignment",
        "agent_brand",
        "codex meta",
        "codex-meta",
        "codex may operate in one of two roles",
        "codex worker",
        "claude worker",
        "gemini worker",
        "gemini worker/auditor",
        "gemini auditor",
        "claude is not meta",
        "final-audit",
        "veto-style constitutional inspection",
        "long-context synthesis",
        "autonomous cli workers",
        "workers always start intake",
    ];

    for phrase in forbidden {
        assert!(
            !lower.contains(phrase),
            "{} must not assign role/capability lanes by CLI label; found phrase `{phrase}`",
            path.display()
        );
    }
}

#[test]
fn harness_agent_entry_and_cli_adapters_point_to_shared_agents() {
    let root = repo_root();
    let entry = root.join("AGENT_ENTRY.md");
    let agents = root.join("AGENTS.md");

    assert!(
        entry.exists(),
        "AGENT_ENTRY.md must exist as the single CLI entry point"
    );
    assert!(
        agents.exists(),
        "AGENTS.md must exist as the shared cross-agent contract"
    );

    let entry_text = read_text(&entry);
    assert!(
        entry_text.contains("Read `AGENTS.md`") || entry_text.contains("Read `AGENTS.md`."),
        "AGENT_ENTRY.md must route all workers through AGENTS.md"
    );
    assert!(
        entry_text.contains("not assigned a role"),
        "AGENT_ENTRY.md must start CLI sessions without assigning a role"
    );
    for role_entry in [
        "docs/harness/roles/META_ENTRY.md",
        "docs/harness/roles/WORKER_ENTRY.md",
        "docs/harness/roles/AUDITOR_ENTRY.md",
        "docs/harness/roles/VETO_ENTRY.md",
    ] {
        assert!(
            entry_text.contains(role_entry),
            "AGENT_ENTRY.md must route explicit role assignments to {role_entry}"
        );
    }
    assert!(
        entry_text.contains("docs/harness/broadcast/TASK_BOARD.json"),
        "AGENT_ENTRY.md must mention TASK_BOARD.json as worker role input, not as a universal role assignment"
    );
    for worker_detail in [
        "worker_slot",
        "capabilities =",
        "[CLAIM][<atom_id>][ClassX] <task title>",
        "work/<atom_id>/<worker_slot>",
        "gh pr ready",
        "## Draft PR Claim",
        "## Eligibility",
        "## Class Rules",
    ] {
        assert!(
            !entry_text.contains(worker_detail),
            "AGENT_ENTRY.md must not contain worker-role operational detail `{worker_detail}`"
        );
    }
    assert_no_cli_role_assignment_language(&entry, &entry_text);

    for adapter in ["CLAUDE.md", "GEMINI.md", "CODEX.md"] {
        let path = root.join(adapter);
        assert!(path.exists(), "{adapter} must exist");
        let text = read_text(&path);
        assert!(
            text.contains("AGENTS.md") && text.contains("AGENT_ENTRY.md"),
            "{adapter} must import the shared AGENTS.md and AGENT_ENTRY.md boundary"
        );
        for role_doc in [
            "docs/harness/WORKER_HARNESS.md",
            "docs/harness/META_HARNESS.md",
            "docs/harness/VETO_AI_POLICY.md",
            "docs/harness/broadcast/TASK_BOARD.json",
        ] {
            assert!(
                !text.contains(role_doc),
                "{adapter} must not route directly to role-specific docs"
            );
        }
        assert_no_cli_role_assignment_language(&path, &text);
    }
}

#[test]
fn harness_docs_do_not_reintroduce_cli_role_lanes() {
    let root = repo_root();
    for rel in [
        "AGENTS.md",
        "AGENT_ENTRY.md",
        "README.md",
        "HARNESS.md",
        "docs/harness/META_HARNESS.md",
        "docs/harness/WORKER_HARNESS.md",
        "docs/harness/TASK_BROADCAST_POLICY.md",
        "docs/harness/broadcast/TASK_BOARD.json",
        "docs/harness/schemas/task_board.schema.json",
        "docs/harness/boot_prompts/universal_worker.md",
        "docs/harness/boot_prompts/veto_ai.md",
        "docs/harness/roles/META_ENTRY.md",
        "docs/harness/roles/WORKER_ENTRY.md",
        "docs/harness/roles/AUDITOR_ENTRY.md",
        "docs/harness/roles/VETO_ENTRY.md",
        "docs/harness/broadcast/tasks/V5-R0-HARNESS-001.r1.task.json",
    ] {
        let path = root.join(rel);
        assert_no_cli_role_assignment_language(&path, &read_text(&path));
    }

    for rel in [
        "docs/harness/boot_prompts/codex_meta.md",
        "docs/harness/boot_prompts/claude_worker.md",
        "docs/harness/boot_prompts/gemini_auditor.md",
    ] {
        assert!(
            !root.join(rel).exists(),
            "{rel} must not exist because boot prompts must not encode CLI-specific role lanes"
        );
    }
}

#[test]
fn harness_role_entries_are_explicit_and_separate_from_cli_labels() {
    let root = repo_root();
    let roles = [
        (
            "docs/harness/roles/META_ENTRY.md",
            "docs/harness/META_HARNESS.md",
        ),
        (
            "docs/harness/roles/WORKER_ENTRY.md",
            "docs/harness/WORKER_HARNESS.md",
        ),
        (
            "docs/harness/roles/AUDITOR_ENTRY.md",
            "docs/harness/templates/ReviewPacket.md",
        ),
        (
            "docs/harness/roles/VETO_ENTRY.md",
            "docs/harness/VETO_AI_POLICY.md",
        ),
    ];

    for (role_entry, required_doc) in roles {
        let path = root.join(role_entry);
        assert!(path.exists(), "{role_entry} must exist");
        let text = read_text(&path);
        assert!(
            text.contains("explicitly assigned") && text.contains(required_doc),
            "{role_entry} must activate only after explicit role assignment and point to {required_doc}"
        );
        assert_no_cli_role_assignment_language(&path, &text);
    }
}

#[test]
fn agent_binding_schema_uses_cli_label_not_brand_identity() {
    let root = repo_root();
    let schema_path = root.join("docs/harness/schemas/agent_binding.schema.json");
    let schema = read_json(&schema_path);
    let required = required_fields(&schema, "agent_binding");

    assert!(
        required.iter().any(|field| field == "cli_label"),
        "AgentBinding must require neutral cli_label"
    );
    assert!(
        !required.iter().any(|field| field == "agent_brand"),
        "AgentBinding must not require agent_brand"
    );
    assert!(
        schema["properties"].get("agent_brand").is_none(),
        "AgentBinding must not define agent_brand"
    );
}

#[test]
fn harness_task_board_declares_meta_only_writer_and_valid_packets() {
    let root = repo_root();
    let board_path = root.join("docs/harness/broadcast/TASK_BOARD.json");
    let board = read_json(&board_path);

    assert_eq!(board["board_version"], "v0.7");
    assert_eq!(board["generated_by_role"], "meta");
    assert!(
        board["default_worker_profile"].is_object(),
        "TASK_BOARD must publish the neutral default_worker_profile for H0 smoke"
    );
    assert!(
        board.get("generated_by").is_none(),
        "TASK_BOARD must not record a CLI-branded generated_by value"
    );
    assert_eq!(board["board_writer"], "meta-only");
    assert_eq!(
        board["runtime_boundary"]["development_control_plane_only"], true,
        "TASK_BOARD must be development control plane only"
    );

    let tasks = board["tasks"]
        .as_array()
        .expect("TASK_BOARD.tasks must be an array");
    assert!(
        !tasks.is_empty(),
        "TASK_BOARD must publish at least one bootstrap task"
    );

    for task in tasks {
        let atom_id = task["atom_id"].as_str().expect("task atom_id missing");
        let class = task["class"].as_u64().expect("task class missing");
        let self_select = task["self_select"]
            .as_bool()
            .expect("task self_select missing");
        if class == 4 {
            assert!(
                !self_select,
                "Class 4 task {atom_id} must never be self-selectable"
            );
        }
        if class == 3 && self_select {
            assert_eq!(
                task["meta_opened"].as_bool(),
                Some(true),
                "Class 3 task {atom_id} may self-select only when meta_opened is true"
            );
        }

        let packet_rel = task["task_packet"]
            .as_str()
            .unwrap_or_else(|| panic!("task {atom_id} missing task_packet"));
        let packet_path = root.join(packet_rel);
        assert!(
            packet_path.exists(),
            "task {atom_id} references missing packet {}",
            packet_path.display()
        );
        let packet = read_json(&packet_path);
        assert_eq!(packet["atom_id"], task["atom_id"]);
        assert_eq!(packet["revision"], task["revision"]);
        assert!(
            packet["acceptance_tests"]
                .as_array()
                .is_some_and(|tests| !tests.is_empty()),
            "task packet {atom_id} must declare visible acceptance tests"
        );
    }
}

#[test]
fn task_board_schema_records_meta_role_not_cli_generator() {
    let root = repo_root();
    let schema = read_json(root.join("docs/harness/schemas/task_board.schema.json"));
    let required = required_fields(&schema, "TaskBoard");

    assert!(
        required.iter().any(|field| field == "generated_by_role"),
        "TaskBoard schema must require generated_by_role"
    );
    assert!(
        !required.iter().any(|field| field == "generated_by"),
        "TaskBoard schema must not require generated_by"
    );
    assert!(
        schema["properties"].get("generated_by").is_none(),
        "TaskBoard schema must not define generated_by"
    );
    assert_eq!(
        schema["properties"]["generated_by_role"]["const"], "meta",
        "TaskBoard generated_by_role must be a role, not a CLI label"
    );
    assert!(
        required
            .iter()
            .any(|field| field == "default_worker_profile"),
        "TaskBoard schema must require default_worker_profile"
    );
    assert!(
        schema["properties"]["default_worker_profile"].is_object(),
        "TaskBoard schema must define default_worker_profile"
    );
}

#[test]
fn default_worker_profile_can_claim_at_least_one_open_h0_task() {
    let root = repo_root();
    let board = read_json(root.join("docs/harness/broadcast/TASK_BOARD.json"));
    let profile = &board["default_worker_profile"];
    let allowed_class = profile["allowed_class"]
        .as_u64()
        .expect("default_worker_profile.allowed_class must be an integer");
    let capabilities = profile["capabilities"]
        .as_array()
        .expect("default_worker_profile.capabilities must be an array");
    let tasks = board["tasks"]
        .as_array()
        .expect("TASK_BOARD.tasks must be an array");

    let eligible: Vec<_> = tasks
        .iter()
        .filter(|task| {
            task["status"] == "open"
                && task["self_select"] == true
                && task["claim_required"] == true
                && task["claim_method"] == "draft_pr"
                && task["class"]
                    .as_u64()
                    .is_some_and(|class| class <= allowed_class)
                && task["blockers"]
                    .as_array()
                    .is_some_and(|blockers| blockers.is_empty())
                && task["required_capabilities"]
                    .as_array()
                    .is_some_and(|required| {
                        required
                            .iter()
                            .all(|required_capability| capabilities.contains(required_capability))
                    })
        })
        .collect();

    assert!(
        !eligible.is_empty(),
        "default_worker_profile must be able to claim at least one open H0 smoke task"
    );
}

#[test]
fn harness_task_board_publishes_real_smoke_tasks_and_retires_bootstrap_tasks() {
    let root = repo_root();
    let board = read_json(root.join("docs/harness/broadcast/TASK_BOARD.json"));
    let tasks = board["tasks"]
        .as_array()
        .expect("TASK_BOARD.tasks must be an array");

    for atom_id in ["V5-R0-DOCS-001", "V5-R0-HARNESS-001"] {
        let task = tasks
            .iter()
            .find(|task| task["atom_id"] == atom_id)
            .unwrap_or_else(|| panic!("{atom_id} must remain on the board as retired history"));
        assert_eq!(
            task["status"], "retired",
            "{atom_id} must be retired before the H0 smoke round"
        );
    }

    let expected_smoke_tasks = [
        (
            "V5-H0-HARNESS-JSON-001",
            ["harness", "json", "schema", "qa"].as_slice(),
        ),
        (
            "V5-H0-HARNESS-SINGLESHOT-001",
            ["harness", "docs", "worker-lifecycle"].as_slice(),
        ),
        (
            "V5-H0-HARNESS-REPAIR-001",
            ["harness", "meta-governance", "policy"].as_slice(),
        ),
    ];

    for (atom_id, capabilities) in expected_smoke_tasks {
        let task = tasks
            .iter()
            .find(|task| task["atom_id"] == atom_id)
            .unwrap_or_else(|| panic!("{atom_id} must be published for real CLI smoke"));
        assert_eq!(
            task["status"], "open",
            "{atom_id} must be worker-selectable"
        );
        assert_eq!(
            task["self_select"], true,
            "{atom_id} must be self-selectable"
        );
        assert_eq!(task["class"], 1, "{atom_id} must stay in harness Class 1");
        assert_eq!(
            task["claim_required"], true,
            "{atom_id} must require draft PR claim"
        );
        assert_eq!(
            task["claim_method"], "draft_pr",
            "{atom_id} must claim by draft PR"
        );
        let required = task["required_capabilities"]
            .as_array()
            .unwrap_or_else(|| panic!("{atom_id} must declare required_capabilities"));
        for capability in capabilities {
            assert!(
                required.iter().any(|value| value == capability),
                "{atom_id} must require capability {capability}"
            );
        }
    }
}

#[test]
fn harness_worker_lifecycle_is_single_shot_and_requires_halt() {
    let root = repo_root();
    let entry = fs::read_to_string(root.join("docs/harness/roles/WORKER_ENTRY.md"))
        .expect("WORKER_ENTRY.md should be readable");
    let worker_harness = fs::read_to_string(root.join("docs/harness/WORKER_HARNESS.md"))
        .expect("WORKER_HARNESS.md should be readable");
    let worker_report = fs::read_to_string(root.join("docs/harness/templates/WorkerReport.md"))
        .expect("WorkerReport template should be readable");

    let entry_lower = entry.to_ascii_lowercase();
    assert!(
        entry_lower.contains("single-shot"),
        "AGENT_ENTRY.md must describe the H0 worker lifecycle as single-shot case-insensitively"
    );
    assert!(
        entry.contains("[WORKER_HALT]"),
        "AGENT_ENTRY.md must require [WORKER_HALT] after PR and WorkerReport"
    );
    assert!(
        !entry.contains("Return to idle polling") && !entry.contains("## Polling Loop"),
        "AGENT_ENTRY.md must not encourage initial idle polling during H0 smoke"
    );
    assert!(
        worker_harness.contains("[WORKER_HALT]"),
        "WORKER_HARNESS.md must require [WORKER_HALT]"
    );
    assert!(
        worker_report.contains("[WORKER_HALT]"),
        "WorkerReport template must include an explicit [WORKER_HALT] confirmation"
    );
}

#[test]
fn harness_worker_claim_protocol_is_draft_pr_from_isolated_worktree() {
    let root = repo_root();
    let entry = fs::read_to_string(root.join("docs/harness/roles/WORKER_ENTRY.md"))
        .expect("WORKER_ENTRY.md should be readable");
    let worker_harness = fs::read_to_string(root.join("docs/harness/WORKER_HARNESS.md"))
        .expect("WORKER_HARNESS.md should be readable");
    let task_policy = fs::read_to_string(root.join("docs/harness/TASK_BROADCAST_POLICY.md"))
        .expect("TASK_BROADCAST_POLICY.md should be readable");

    for text in [&entry, &worker_harness] {
        assert!(
            text.contains("cd /home/zephryj/projects/turingosv5"),
            "workers must start intake from the V5 main directory"
        );
        assert!(
            text.contains("/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>"),
            "workers must create task code in the isolated worktree path"
        );
        assert!(
            text.contains("work/<atom_id>/<worker_slot>"),
            "workers must use the standard work branch pattern"
        );
        assert!(
            text.contains("origin/main"),
            "workers must create worktrees from latest origin/main"
        );
        assert!(
            text.contains("[CLAIM][<atom_id>][ClassX] <task title>"),
            "workers must open a draft PR claim with the standard title"
        );
        assert!(
            text.contains("gh pr ready"),
            "workers must convert the same draft PR to ready"
        );
    }

    assert!(
        task_policy.contains("createdAt") && task_policy.contains("earliest valid claim"),
        "duplicate draft PR claims must use earliest createdAt valid claim"
    );
    assert!(
        task_policy.contains("SUPERSEDE") && task_policy.contains("duplicate evidence"),
        "duplicate claims must become supersede or duplicate evidence"
    );
}

#[test]
fn harness_claim_and_worker_report_schemas_cover_pr_intake() {
    let root = repo_root();
    let claim_schema = read_json(root.join("docs/harness/schemas/claim_record.schema.json"));
    let worker_report_schema =
        read_json(root.join("docs/harness/schemas/worker_report.schema.json"));
    let task_board_schema = read_json(root.join("docs/harness/schemas/task_board.schema.json"));
    let task_packet_schema = read_json(root.join("docs/harness/schemas/task_packet.schema.json"));

    for field in [
        "board_version",
        "board_sha256",
        "task_packet_path",
        "task_packet_sha256",
        "allowed_files",
        "forbidden_files",
        "worker_profile",
        "claim_timestamp",
    ] {
        assert!(
            required_fields(&claim_schema, "ClaimRecord")
                .iter()
                .any(|required| required == field),
            "ClaimRecord schema must require {field}"
        );
    }

    for field in ["claim_pr_url", "ready_pr_url", "worktree"] {
        assert!(
            required_fields(&worker_report_schema, "WorkerReport")
                .iter()
                .any(|required| required == field),
            "WorkerReport schema must require {field}"
        );
    }

    let board_task_required = task_board_schema["$defs"]["task"]["required"]
        .as_array()
        .expect("TaskBoard task schema must declare required fields");
    for field in ["claim_required", "claim_method"] {
        assert!(
            board_task_required.iter().any(|required| required == field),
            "TaskBoard task schema must require {field}"
        );
    }

    for field in ["claim_required", "claim_method"] {
        assert!(
            required_fields(&task_packet_schema, "TaskPacket")
                .iter()
                .any(|required| required == field),
            "TaskPacket schema must require {field}"
        );
    }
}

#[test]
fn harness_repair_and_dirty_conflict_guards_are_declared() {
    let root = repo_root();
    let board = read_json(root.join("docs/harness/broadcast/TASK_BOARD.json"));
    assert_eq!(
        board["max_repair_attempts"], 3,
        "TASK_BOARD must cap repair attempts at 3"
    );
    assert_eq!(
        board["worker_halt_required"], true,
        "TASK_BOARD must declare worker_halt_required"
    );
    assert_eq!(
        board["conflict_policy"], "supersede_on_dirty",
        "TASK_BOARD must quarantine dirty merge states by superseding"
    );

    let dirty_policy = fs::read_to_string(root.join("docs/harness/DIRTY_TREE_POLICY.md"))
        .expect("DIRTY_TREE_POLICY.md should be readable");
    assert!(
        dirty_policy.contains("mergeStateStatus == \"dirty\"")
            && dirty_policy.contains("SUPERSEDE"),
        "dirty mergeStateStatus must be documented as SUPERSEDE-only"
    );

    let worker_report_schema =
        read_json(root.join("docs/harness/schemas/worker_report.schema.json"));
    let required = worker_report_schema["required"]
        .as_array()
        .expect("WorkerReport schema must declare required fields");
    assert!(
        required
            .iter()
            .any(|field| field == "worker_halt_confirmation"),
        "WorkerReport schema must require worker_halt_confirmation"
    );
    assert_eq!(
        worker_report_schema["properties"]["worker_halt_confirmation"]["const"], "[WORKER_HALT]",
        "WorkerReport schema must require the exact [WORKER_HALT] marker"
    );

    let task_board_schema = read_json(root.join("docs/harness/schemas/task_board.schema.json"));
    let status_enum = task_board_schema["$defs"]["task"]["properties"]["status"]["enum"]
        .as_array()
        .expect("TaskBoard status schema must declare enum");
    assert!(
        status_enum
            .iter()
            .any(|status| status == "BLOCKED_NEEDS_HUMAN"),
        "repair breaker must support BLOCKED_NEEDS_HUMAN state"
    );

    let failure_playbook = fs::read_to_string(root.join("docs/harness/FAILURE_PLAYBOOK.md"))
        .expect("FAILURE_PLAYBOOK.md should be readable");
    assert!(
        failure_playbook.contains("BLOCKED_NEEDS_HUMAN"),
        "repair breaker must stop at BLOCKED_NEEDS_HUMAN after 3 repairs"
    );
}

#[test]
fn harness_runtime_does_not_read_agent_broadcast() {
    let root = repo_root();
    let mut matches = Vec::new();
    collect_control_plane_refs(&root, &root.join("src"), &mut matches);
    assert!(
        matches.is_empty(),
        "V5 runtime must not read harness broadcast/control-plane files:\n{}",
        matches.join("\n")
    );
}

#[test]
fn harness_v5_does_not_carry_minif2f_development_dataset() {
    let root = repo_root();
    assert!(
        !root.join("experiments/minif2f_v4").exists(),
        "MiniF2F is a V4 development/eval corpus, not a V5 product asset"
    );
}
