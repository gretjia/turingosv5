use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use turingosv5::devtool::{
    append_event, default_auth_profiles_path, default_provider_profiles_path,
    default_secrets_env_path, meta_ai_welcome_frame, meta_ai_welcome_frame_with_selection,
    read_meta_ai_config, write_deepseek_fallback_config,
    write_deepseek_fallback_config_with_state_dir, write_deepseek_secret_from_env_file,
    AppendInput, CodexAppServerAdapter,
};

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "turingosv5-console-{name}-{}-{nanos}",
        std::process::id()
    ))
}

fn bin() -> String {
    std::env::var("CARGO_BIN_EXE_turingos-dev")
        .expect("cargo should expose turingos-dev binary path to integration tests")
}

fn turingos_bin() -> String {
    std::env::var("CARGO_BIN_EXE_turingos")
        .expect("cargo should expose turingos binary path to integration tests")
}

fn envelope(event_id: &str, event_type: &str, previous: Option<&str>) -> Value {
    json!({
        "event_id": event_id,
        "event_type": event_type,
        "project_id": "turingosv5",
        "actor_identity_cid": "sha256:actor0001",
        "payload_cid": "sha256:filled-by-append",
        "previous_event_cid": previous,
        "observed_at": "2026-05-20T00:00:00Z",
        "source": "local_cli",
        "subject": {
            "repo": "gretjia/turingosv5",
            "branch": null,
            "pr": null,
            "files": []
        },
        "evidence": {
            "commands": [],
            "artifacts": [],
            "source_anchors": []
        },
        "classification": {
            "risk_class": 0,
            "candidate": true,
            "runtime_truth": false
        },
        "integrity": {
            "payload_hash": "sha256:filled-by-append",
            "envelope_hash": "sha256:filled-by-append"
        }
    })
}

fn input(
    event_id: &str,
    event_type: &str,
    previous: Option<String>,
    payload: Value,
) -> AppendInput {
    AppendInput {
        previous_record_hash: previous.clone(),
        envelope: envelope(event_id, event_type, previous.as_deref()),
        payload,
    }
}

fn task_payload(atom_id: &str) -> Value {
    json!({
        "atom_id": atom_id,
        "title": "Build read-only TuringOS console",
        "phase": "V5-CONSOLE",
        "lane": "devtool",
        "risk_class": 1,
        "priority": "P0",
        "status": "open",
        "self_select": true,
        "meta_opened": true,
        "claim_mode": "open_pool",
        "claim_required": true,
        "claim_method": "draft_pr",
        "required_capabilities": ["harness"],
        "preferred_capabilities": ["ui"],
        "allowed_files": ["src/bin/turingos-dev.rs"],
        "forbidden_files": ["src/runtime/**"],
        "task_packet": "docs/harness/broadcast/tasks/V5-CONSOLE-001.json",
        "acceptance_criteria": ["cargo test --test v5_console_tui"],
        "duplicate_policy": "first_valid_pr_wins",
        "blockers": []
    })
}

fn append(
    store: &Path,
    event_id: &str,
    event_type: &str,
    previous: Option<String>,
    payload: Value,
) -> String {
    append_event(store, input(event_id, event_type, previous, payload))
        .expect("append should succeed")
        .record_hash
}

#[test]
fn console_help_describes_read_only_devtape_view() {
    let output = Command::new(bin())
        .args(["console", "--help"])
        .output()
        .expect("console help should run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("read-only"));
    assert!(stdout.contains("DevTape"));
    assert!(stdout.contains("does not write TASK_BOARD.json"));
}

#[test]
fn console_reports_missing_store_without_creating_a_board() {
    let dir = temp_path("missing");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("missing-events.jsonl");
    let board = dir.join("TASK_BOARD.json");

    let output = Command::new(bin())
        .args([
            "console",
            "--store",
            store.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("console should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TuringOS V5 Console"));
    assert!(stdout.contains("DevTape not initialized"));
    assert!(
        !board.exists(),
        "console must not create a TASK_BOARD projection as a side effect"
    );
}

#[test]
fn console_renders_tasks_from_devtape_projection() {
    let dir = temp_path("tasks");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let atom = "V5-CONSOLE-TUI-001";

    let task = append(&store, "e1", "DevTaskCreated", None, task_payload(atom));
    let broadcast = append(
        &store,
        "e2",
        "TaskBroadcasted",
        Some(task),
        json!({"atom_id": atom}),
    );
    append(
        &store,
        "e3",
        "TaskClaimed",
        Some(broadcast),
        json!({
            "atom_id": atom,
            "pr_number": 22,
            "claim_pr_url": "https://github.com/gretjia/turingosv5/pull/22",
            "createdAt": "2026-05-20T00:10:00Z"
        }),
    );

    let output = Command::new(bin())
        .args([
            "console",
            "--store",
            store.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("console should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("records: 3"));
    assert!(stdout.contains(atom));
    assert!(stdout.contains("claimed"));
    assert!(stdout.contains("PR #22"));
}

#[test]
fn turingos_short_entry_opens_console_directly() {
    let dir = temp_path("short-entry");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let atom = "V5-CONSOLE-SHORT-ENTRY-001";

    let task = append(&store, "e1", "DevTaskCreated", None, task_payload(atom));
    append(
        &store,
        "e2",
        "TaskBroadcasted",
        Some(task),
        json!({"atom_id": atom}),
    );

    let output = Command::new(turingos_bin())
        .args([
            "--plain",
            "--store",
            store.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("turingos short entry should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TuringOS V5 Console"));
    assert!(stdout.contains(atom));
}

#[test]
fn turingos_tui_frame_has_screen_layout_and_commands() {
    let dir = temp_path("tui-frame");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let atom = "V5-CONSOLE-TUI-FRAME-001";

    let task = append(&store, "e1", "DevTaskCreated", None, task_payload(atom));
    append(
        &store,
        "e2",
        "TaskBroadcasted",
        Some(task),
        json!({"atom_id": atom}),
    );

    let output = Command::new(turingos_bin())
        .args([
            "--tui-frame",
            "--store",
            store.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("turingos tui frame should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\x1b[2J\x1b[H"));
    assert!(stdout.contains("+ TuringOS V5"));
    assert!(stdout.contains("DevTape"));
    assert!(stdout.contains(atom));
    assert!(stdout.contains("[w] welcome"));
    assert!(stdout.contains("[h] help"));
    assert!(stdout.contains("[q] quit"));
}

#[test]
fn welcome_page_guides_openai_oauth_then_deepseek_fallback() {
    let dir = temp_path("welcome");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let config = dir.join("meta_ai.json");

    let frame = meta_ai_welcome_frame(&store, &config).expect("welcome frame should render");
    assert!(frame.contains("Welcome to TuringOS"));
    assert!(frame.contains("MetaAI setup"));
    assert!(frame.contains("OpenAI OAuth"));
    assert!(frame.contains("Codex app-server"));
    assert!(frame.contains("DeepSeek API fallback"));
    assert!(frame.contains("DEEPSEEK_API_KEY"));
    assert!(frame.contains("No secret is written"));
    assert!(frame.contains("↑/↓"));
    assert!(frame.contains("Enter"));
    assert!(frame.contains("[o]"));
    assert!(frame.contains("[d]"));
}

#[test]
fn welcome_page_has_color_and_visible_selected_action() {
    let dir = temp_path("welcome-selected");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let config = dir.join("meta_ai.json");

    let frame = meta_ai_welcome_frame_with_selection(&store, &config, 1)
        .expect("welcome frame should render");
    assert!(frame.contains("\x1b[38;5;"));
    assert!(frame.contains("▸"));
    assert!(frame.contains("DeepSeek API fallback"));
    assert!(frame.contains("Set fallback profile"));
}

#[test]
fn deepseek_fallback_config_stores_env_name_not_secret_value() {
    let dir = temp_path("deepseek-config");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let config = dir.join("meta_ai.json");
    let private_state = dir.join("private-state");

    write_deepseek_fallback_config_with_state_dir(&config, "DEEPSEEK_API_KEY", &private_state)
        .expect("config should be written");
    let text = fs::read_to_string(&config).expect("config should exist");
    assert!(text.contains("DEEPSEEK_API_KEY"));
    assert!(!text.contains("sk-"));
    assert!(!text.contains("test-secret-value"));

    let parsed = read_meta_ai_config(&config).expect("config should parse");
    assert_eq!(
        parsed.deepseek_api_key_env.as_deref(),
        Some("DEEPSEEK_API_KEY")
    );
    assert_eq!(
        parsed.deepseek_base_url.as_deref(),
        Some("https://api.deepseek.com")
    );
    assert_eq!(
        parsed.deepseek_default_model.as_deref(),
        Some("deepseek-v4-flash")
    );
    assert_eq!(
        parsed.deepseek_reasoning_model.as_deref(),
        Some("deepseek-v4-pro")
    );
    assert_eq!(parsed.meta_ai_model.as_deref(), Some("deepseek-v4-pro"));
    assert_eq!(parsed.meta_ai_thinking_enabled, Some(true));
    assert_eq!(
        parsed.deepseek_legacy_alias_deprecated_after.as_deref(),
        Some("2026-07-24")
    );
    assert_eq!(
        parsed.provider_profile_source_url.as_deref(),
        Some("https://api-docs.deepseek.com/")
    );
    assert_eq!(parsed.provider_profile_stale_after_days, 14);
    assert!(!parsed.stores_api_key_values);
    assert_eq!(
        parsed.auth_profiles_path.as_deref(),
        Some(
            private_state
                .join("auth-profiles.json")
                .to_str()
                .expect("utf8 path")
        )
    );
    assert_eq!(
        parsed.secrets_env_path.as_deref(),
        Some(
            private_state
                .join("secrets.env")
                .to_str()
                .expect("utf8 path")
        )
    );
    assert!(private_state.join("auth-profiles.json").exists());
}

#[test]
fn deepseek_fallback_rejects_api_key_values_as_env_names() {
    let dir = temp_path("deepseek-invalid-env-name");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let config = dir.join("meta_ai.json");

    for invalid in [
        "",
        "sk-pasted-secret",
        "dsk_pasted_secret",
        "DeepSeekApiKey",
        "DEEPSEEK_API_KEY=value",
        "DEEPSEEK API KEY",
        "DEEPSEEKAPIKEY",
    ] {
        let err = write_deepseek_fallback_config(&config, invalid)
            .expect_err("invalid env names should be rejected");
        assert!(
            err.to_string().contains("environment variable name"),
            "unexpected error for {invalid:?}: {err}"
        );
    }
}

#[test]
fn deepseek_secret_import_reads_env_file_without_putting_key_in_provider_profile() {
    let dir = temp_path("secret-import");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source_env = dir.join("source.env");
    let secrets_env = dir.join("secrets.env");
    let provider_profile = dir.join("provider-profiles.json");
    fs::write(
        &source_env,
        "OTHER_KEY=not-this\nDEEPSEEK_API_KEY=test-secret-value\n",
    )
    .expect("source env should be written");

    write_deepseek_fallback_config(&provider_profile, "DEEPSEEK_API_KEY")
        .expect("profile should be written");
    write_deepseek_secret_from_env_file(&source_env, &secrets_env, "DEEPSEEK_API_KEY")
        .expect("secret should import");

    let profile_text = fs::read_to_string(provider_profile).expect("profile should exist");
    assert!(!profile_text.contains("test-secret-value"));
    let secret_text = fs::read_to_string(&secrets_env).expect("secret file should exist");
    assert!(secret_text.contains("DEEPSEEK_API_KEY=test-secret-value"));
}

#[test]
fn deepseek_secret_import_rejects_repo_local_secret_destination() {
    let dir = temp_path("repo-secret-source");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source_env = dir.join("source.env");
    fs::write(&source_env, "DEEPSEEK_API_KEY=test-secret-value\n")
        .expect("source env should be written");
    let repo_secret = std::env::current_dir()
        .expect("current dir should exist")
        .join("target/turingosv5-test-secret.env");

    let err = write_deepseek_secret_from_env_file(&source_env, &repo_secret, "DEEPSEEK_API_KEY")
        .expect_err("repo-local secret destination should be rejected");
    assert!(err.to_string().contains("outside the repo"));
}

#[test]
fn deepseek_setup_rejects_repo_local_state_even_when_run_from_subdir() {
    let dir = temp_path("repo-state-subdir");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let source_env = dir.join("missing.env");
    let repo = std::env::current_dir().expect("current dir should exist");
    let subdir = repo.join("src/devtool");

    let output = Command::new(turingos_bin())
        .current_dir(&subdir)
        .args([
            "meta",
            "set-deepseek",
            "--api-key-env",
            "DEEPSEEK_API_KEY",
            "--from-env-file",
            source_env.to_str().expect("path should be utf8"),
        ])
        .env("TURINGOS_HOME", repo.join("target/repo-local-state"))
        .output()
        .expect("deepseek setup should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("outside the repo"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn local_credential_paths_are_outside_repo_by_default() {
    let provider_profiles = default_provider_profiles_path();
    let auth_profiles = default_auth_profiles_path();
    let secrets_env = default_secrets_env_path();
    let repo = std::env::current_dir().expect("current dir should exist");

    for path in [&provider_profiles, &auth_profiles, &secrets_env] {
        assert!(
            !path.starts_with(&repo),
            "credential path must not live inside repo: {}",
            path.display()
        );
    }
    assert!(provider_profiles.ends_with("provider-profiles.json"));
    assert!(auth_profiles.ends_with("auth-profiles.json"));
    assert!(secrets_env.ends_with("secrets.env"));
}

#[cfg(unix)]
#[test]
fn local_provider_and_auth_files_are_private_on_unix() {
    use std::os::unix::fs::PermissionsExt;

    let dir = temp_path("private-perms");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let config = dir.join("provider-profiles.json");
    write_deepseek_fallback_config_with_state_dir(&config, "DEEPSEEK_API_KEY", &dir)
        .expect("config should be written");

    let dir_mode = fs::metadata(&dir)
        .expect("state dir should exist")
        .permissions()
        .mode()
        & 0o777;
    let config_mode = fs::metadata(&config)
        .expect("provider config should exist")
        .permissions()
        .mode()
        & 0o777;
    let auth_mode = fs::metadata(dir.join("auth-profiles.json"))
        .expect("auth profiles should exist")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(dir_mode, 0o700);
    assert_eq!(config_mode, 0o600);
    assert_eq!(auth_mode, 0o600);
}

#[test]
fn turingos_welcome_frame_flag_outputs_setup_page() {
    let dir = temp_path("welcome-flag");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let store = dir.join("events.jsonl");
    let config = dir.join("meta_ai.json");

    let output = Command::new(turingos_bin())
        .args([
            "--welcome-frame",
            "--selected",
            "1",
            "--store",
            store.to_str().expect("path should be utf8"),
            "--meta-config",
            config.to_str().expect("path should be utf8"),
        ])
        .output()
        .expect("welcome frame should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Welcome to TuringOS"));
    assert!(stdout.contains("OpenAI OAuth"));
    assert!(stdout.contains("DeepSeek API fallback"));
    assert!(stdout.contains("▸"));
}

#[test]
fn turingos_deepseek_setup_command_writes_untracked_local_config() {
    let dir = temp_path("deepseek-cli");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let config = dir.join("meta_ai.json");
    let state_dir = dir.join("state");
    let source_env = dir.join("source.env");
    fs::write(&source_env, "DEEPSEEK_API_KEY=test-secret-value\n")
        .expect("source env should be written");

    let output = Command::new(turingos_bin())
        .args([
            "meta",
            "set-deepseek",
            "--api-key-env",
            "DEEPSEEK_API_KEY",
            "--from-env-file",
            source_env.to_str().expect("path should be utf8"),
            "--meta-config",
            config.to_str().expect("path should be utf8"),
        ])
        .env("TURINGOS_HOME", &state_dir)
        .output()
        .expect("deepseek setup should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DEEPSEEK_API_KEY"));
    assert!(stdout.contains("secret value not stored"));
    let text = fs::read_to_string(config).expect("config should exist");
    assert!(text.contains("DEEPSEEK_API_KEY"));
    assert!(!text.contains("sk-"));
    assert!(!text.contains("test-secret-value"));
    assert!(!dir.join("secrets.env").exists());
    let secret_text =
        fs::read_to_string(state_dir.join("secrets.env")).expect("secret should exist");
    assert!(secret_text.contains("DEEPSEEK_API_KEY=test-secret-value"));
}

#[test]
fn codex_app_server_adapter_uses_json_rpc_login_not_codex_exec() {
    let adapter = CodexAppServerAdapter::default();
    let command = adapter.stdio_command();
    assert_eq!(command.program, "codex");
    assert_eq!(
        command.args,
        vec![
            "app-server".to_string(),
            "--listen".to_string(),
            "stdio://".to_string()
        ]
    );
    assert!(
        !command.args.iter().any(|arg| arg == "exec"),
        "adapter must not use codex exec for the app-server path"
    );

    let browser_login = adapter.chatgpt_login_request(3);
    assert_eq!(browser_login["method"], "account/login/start");
    assert_eq!(browser_login["params"]["type"], "chatgpt");

    let device_login = adapter.device_code_login_request(4);
    assert_eq!(device_login["method"], "account/login/start");
    assert_eq!(device_login["params"]["type"], "chatgptDeviceCode");
}

#[test]
fn console_doc_keeps_llm_injection_as_adapter_boundary() {
    let doc = fs::read_to_string("docs/v5_dev/TURINGOS_CONSOLE_MVP.md")
        .expect("console MVP doc should exist");
    for required in [
        "Codex app-server adapter",
        "DeepSeek fallback",
        "DEEPSEEK_API_KEY",
        "local OpenAI-compatible proxy",
        "never persisted",
        "not a truth source",
    ] {
        assert!(
            doc.contains(required),
            "missing console doc term: {required}"
        );
    }
}
