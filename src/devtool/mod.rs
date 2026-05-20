use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendInput {
    pub previous_record_hash: Option<String>,
    pub envelope: Value,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DevTapeRecord {
    pub record_hash: String,
    pub previous_record_hash: Option<String>,
    pub envelope: Value,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterCommand {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexAppServerAdapter {
    program: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MetaAiConfig {
    pub schema: String,
    pub primary_adapter: String,
    pub openai_oauth_adapter: String,
    pub deepseek_api_key_env: Option<String>,
    pub deepseek_base_url: Option<String>,
    pub deepseek_default_model: Option<String>,
    pub deepseek_reasoning_model: Option<String>,
    pub deepseek_legacy_alias_deprecated_after: Option<String>,
    pub meta_ai_model: Option<String>,
    pub meta_ai_thinking_enabled: Option<bool>,
    pub fallback_adapter: Option<String>,
    pub auth_profiles_path: Option<String>,
    pub secrets_env_path: Option<String>,
    pub provider_profile_source_url: Option<String>,
    pub provider_profile_checked_at: Option<String>,
    pub provider_profile_stale_after_days: u64,
    pub stores_api_key_values: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MergeGateDecision {
    PROCEED,
    HOLD,
    VETO,
    SUPERSEDE,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeGateResult {
    pub decision: MergeGateDecision,
    pub missing_evidence: Vec<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevToolError {
    message: String,
}

impl DevToolError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for DevToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for DevToolError {}

pub type DevToolResult<T> = Result<T, DevToolError>;

const EVENT_TYPES: &[&str] = &[
    "HumanIntentReceived",
    "MetaReconcileRecorded",
    "DevTaskCreated",
    "TaskBroadcasted",
    "TaskSuperseded",
    "TaskClaimed",
    "WorkerReportSubmitted",
    "AuditVerdictSubmitted",
    "PRCreated",
    "CIResultRecorded",
    "ReviewVerdictSubmitted",
    "VetoVerdictSubmitted",
    "MergeDecisionAccepted",
    "MergeDecisionRejected",
    "MergeDecisionRecorded",
    "PRMerged",
    "PRClosed",
    "RepairTaskCreated",
    "BranchProtectionSnapshotRecorded",
    "BootstrapExceptionRequested",
    "BootstrapExceptionAccepted",
    "BootstrapExceptionRestored",
    "ExternalMutationDetected",
];

pub fn append_event(store: &Path, input: AppendInput) -> DevToolResult<DevTapeRecord> {
    let existing = read_records(store)?;
    let tip = existing.last().map(|record| record.record_hash.clone());
    if input.previous_record_hash != tip {
        return Err(DevToolError::new(format!(
            "previous_record_hash mismatch: expected {:?}, got {:?}",
            tip, input.previous_record_hash
        )));
    }

    let mut envelope = input.envelope;
    let event_type = string_at(&envelope, &["event_type"])?;
    if !EVENT_TYPES.contains(&event_type.as_str()) {
        return Err(DevToolError::new(format!(
            "unknown event_type {event_type}"
        )));
    }
    if bool_at(&envelope, &["classification", "runtime_truth"])? {
        return Err(DevToolError::new(
            "DevTape development events must have runtime_truth=false",
        ));
    }

    let payload_hash = hash_json(&input.payload)?;
    fill_hash_field(&mut envelope, &["payload_cid"], &payload_hash)?;
    fill_hash_field(&mut envelope, &["integrity", "payload_hash"], &payload_hash)?;
    let envelope_hash = hash_json(&envelope)?;
    fill_hash_field(
        &mut envelope,
        &["integrity", "envelope_hash"],
        &envelope_hash,
    )?;

    let previous_record_hash = input.previous_record_hash;
    let record_hash = hash_json(&json!({
        "previous_record_hash": previous_record_hash,
        "envelope": envelope,
        "payload": input.payload
    }))?;
    let record = DevTapeRecord {
        record_hash,
        previous_record_hash,
        envelope,
        payload: input.payload,
    };

    if let Some(parent) = store.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(store)
        .map_err(io_error)?;
    let line = serde_json::to_string(&record).map_err(json_error)?;
    writeln!(file, "{line}").map_err(io_error)?;
    Ok(record)
}

impl Default for CodexAppServerAdapter {
    fn default() -> Self {
        Self {
            program: "codex".to_string(),
        }
    }
}

impl CodexAppServerAdapter {
    pub fn stdio_command(&self) -> AdapterCommand {
        AdapterCommand {
            program: self.program.clone(),
            args: vec![
                "app-server".to_string(),
                "--listen".to_string(),
                "stdio://".to_string(),
            ],
        }
    }

    pub fn chatgpt_login_request(&self, id: u64) -> Value {
        json!({
            "id": id,
            "method": "account/login/start",
            "params": {
                "type": "chatgpt"
            }
        })
    }

    pub fn device_code_login_request(&self, id: u64) -> Value {
        json!({
            "id": id,
            "method": "account/login/start",
            "params": {
                "type": "chatgptDeviceCode"
            }
        })
    }
}

pub fn read_records(store: &Path) -> DevToolResult<Vec<DevTapeRecord>> {
    if !store.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(store).map_err(io_error)?;
    let mut records = Vec::new();
    let mut previous = None;
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let record: DevTapeRecord = serde_json::from_str(line).map_err(json_error)?;
        if record.previous_record_hash != previous {
            return Err(DevToolError::new(format!(
                "broken chain at line {}: expected {:?}, got {:?}",
                index + 1,
                previous,
                record.previous_record_hash
            )));
        }
        previous = Some(record.record_hash.clone());
        records.push(record);
    }
    Ok(records)
}

pub fn console_text(store: &Path) -> DevToolResult<String> {
    let mut lines = vec![
        "TuringOS V5 Console".to_string(),
        "mode: read-only DevTape projection".to_string(),
        format!("store: {}", store.display()),
        "note: console does not write TASK_BOARD.json".to_string(),
    ];

    if !store.exists() {
        lines.push("status: DevTape not initialized".to_string());
        lines.push(format!(
            "next: turingos-dev event append --file <event.json> --store {}",
            store.display()
        ));
        return Ok(lines.join("\n"));
    }

    let records = read_records(store)?;
    let tip = records
        .last()
        .map(|record| record.record_hash.as_str())
        .unwrap_or("none");
    lines.push(format!("records: {}", records.len()));
    lines.push(format!("tip: {tip}"));

    let board = derive_board(store)?;
    let empty = Vec::new();
    let tasks = board
        .get("tasks")
        .and_then(Value::as_array)
        .unwrap_or(&empty);
    lines.push(format!("tasks: {}", tasks.len()));
    for task in tasks {
        let atom = task.get("atom_id").and_then(Value::as_str).unwrap_or("-");
        let status = task.get("status").and_then(Value::as_str).unwrap_or("-");
        let title = task.get("title").and_then(Value::as_str).unwrap_or("-");
        let pr = task.get("pr_number").and_then(Value::as_u64);
        match pr {
            Some(number) => lines.push(format!("- {atom} [{status}] PR #{number}: {title}")),
            None => lines.push(format!("- {atom} [{status}]: {title}")),
        }
    }
    Ok(lines.join("\n"))
}

pub fn console_frame(store: &Path, show_help: bool) -> DevToolResult<String> {
    let records = read_records(store)?;
    let board = derive_board(store)?;
    let empty = Vec::new();
    let tasks = board
        .get("tasks")
        .and_then(Value::as_array)
        .unwrap_or(&empty);
    let tip = records
        .last()
        .map(|record| short_hash(&record.record_hash))
        .unwrap_or_else(|| "none".to_string());
    let store_status = if store.exists() {
        "DevTape ready"
    } else {
        "DevTape not initialized"
    };

    let mut lines = vec![
        "\x1b[2J\x1b[H".to_string(),
        "+ TuringOS V5 ---------------------------------------------------------+".to_string(),
        format!(
            "| {store_status:<20} records: {:<5} tip: {:<22} |",
            records.len(),
            tip
        ),
        format!(
            "| store: {:<59} |",
            truncate(&store.display().to_string(), 59)
        ),
        "+ Tasks ---------------------------------------------------------------+".to_string(),
    ];

    if tasks.is_empty() {
        lines.push(
            "| no projected tasks yet                                             |".to_string(),
        );
    } else {
        for task in tasks.iter().take(12) {
            let atom = task.get("atom_id").and_then(Value::as_str).unwrap_or("-");
            let status = task.get("status").and_then(Value::as_str).unwrap_or("-");
            let title = task.get("title").and_then(Value::as_str).unwrap_or("-");
            let pr = task
                .get("pr_number")
                .and_then(Value::as_u64)
                .map(|number| format!(" PR #{number}"))
                .unwrap_or_default();
            lines.push(format!(
                "| {:<25} {:<10} {:<25} |",
                truncate(atom, 25),
                truncate(&format!("{status}{pr}"), 10),
                truncate(title, 25)
            ));
        }
    }

    lines.push(
        "+ Commands ------------------------------------------------------------+".to_string(),
    );
    lines.push(
        "| [m] meta reconcile   [w] welcome   [r] refresh   [h] help   [q] quit |".to_string(),
    );
    if show_help {
        lines.push(
            "+ Help ----------------------------------------------------------------+".to_string(),
        );
        lines.push(
            "| This TUI is a read-only DevTape projection. It never writes board.  |".to_string(),
        );
        lines.push(
            "| Use turingos-dev event append / board derive for state changes.     |".to_string(),
        );
        lines.push(
            "| Meta reconcile is a one-shot dry-run over board + open PR claims.   |".to_string(),
        );
    }
    lines.push(
        "+---------------------------------------------------------------------+".to_string(),
    );
    Ok(lines.join("\n"))
}

pub fn meta_ai_welcome_frame(store: &Path, config_path: &Path) -> DevToolResult<String> {
    meta_ai_welcome_frame_with_selection(store, config_path, 0)
}

pub fn meta_ai_welcome_frame_with_selection(
    store: &Path,
    config_path: &Path,
    selected: usize,
) -> DevToolResult<String> {
    let records = read_records(store)?;
    let config = read_meta_ai_config(config_path).unwrap_or_default();
    let deepseek_env = config
        .deepseek_api_key_env
        .as_deref()
        .unwrap_or("DEEPSEEK_API_KEY");
    let deepseek_model = config
        .deepseek_default_model
        .as_deref()
        .unwrap_or("deepseek-v4-flash");
    let deepseek_status = if std::env::var_os(deepseek_env).is_some() {
        "env present"
    } else {
        "env missing"
    };
    let openai = welcome_action(
        selected == 0,
        "[o]",
        "OpenAI OAuth",
        "Start Codex app-server login; TuringOS stores no OAuth token.",
    );
    let deepseek = welcome_action(
        selected == 1,
        "[d]",
        "DeepSeek API fallback",
        &format!("Set fallback profile: {deepseek_env} · {deepseek_model} · {deepseek_status}."),
    );
    let console = welcome_action(
        selected == 2,
        "[c]",
        "DevTape console",
        &format!("Open current projection: {} record(s).", records.len()),
    );

    Ok([
        "\x1b[2J\x1b[H".to_string(),
        format!(
            "{}╭────────────────────────────────────────────────────────────────────╮{}",
            color("cyan"),
            color("reset")
        ),
        format!(
            "{}│{} {}TuringOS{}  {}MetaAI setup{}                         {}records {:<3}{} │",
            color("cyan"),
            color("reset"),
            color("bold"),
            color("reset"),
            color("muted"),
            color("reset"),
            color("green"),
            records.len(),
            color("reset")
        ),
        format!(
            "{}╰────────────────────────────────────────────────────────────────────╯{}",
            color("cyan"),
            color("reset")
        ),
        format!(
            "{}Welcome to TuringOS{} · choose how MetaAI should connect.",
            color("bold"),
            color("reset")
        ),
        "".to_string(),
        format!(
            "{}Use ↑/↓ to move, Enter to confirm. Letters still work: [o] [d] [c] [q].{}",
            color("muted"),
            color("reset")
        ),
        "".to_string(),
        openai,
        deepseek,
        console,
        "".to_string(),
        format!(
            "{}Trust boundary{}  No secret is written to repo, DevTape, board, or WorkerReport.",
            color("yellow"),
            color("reset")
        ),
        format!(
            "{}Provider cache{}  DeepSeek profile is a refreshable MetaAI hint, not truth.",
            color("yellow"),
            color("reset")
        ),
        "".to_string(),
        format!(
            "{}Footer{}  [r] refresh   [h] help   [q] quit",
            color("muted"),
            color("reset")
        ),
    ]
    .join("\n"))
}

pub fn read_meta_ai_config(path: &Path) -> DevToolResult<MetaAiConfig> {
    if !path.exists() {
        return Ok(MetaAiConfig::default());
    }
    let text = fs::read_to_string(path).map_err(io_error)?;
    serde_json::from_str(&text).map_err(json_error)
}

pub fn write_deepseek_fallback_config(
    path: &Path,
    api_key_env: &str,
) -> DevToolResult<MetaAiConfig> {
    write_deepseek_fallback_config_with_state_dir(path, api_key_env, &default_turingos_home())
}

pub fn write_deepseek_fallback_config_with_state_dir(
    path: &Path,
    api_key_env: &str,
    state_dir: &Path,
) -> DevToolResult<MetaAiConfig> {
    if !is_env_var_name(api_key_env) {
        return Err(DevToolError::new(
            "provide an environment variable name, not an API key value",
        ));
    }
    ensure_private_parent(path)?;
    let auth_profiles_path = state_dir.join("auth-profiles.json");
    let secrets_env_path = state_dir.join("secrets.env");
    ensure_repo_external_path(&auth_profiles_path, "auth profile path")?;
    ensure_repo_external_path(&secrets_env_path, "secret state path")?;
    ensure_auth_profiles_file(&auth_profiles_path)?;

    let config = MetaAiConfig {
        schema: "turingos.v5.meta_ai_config.v1".to_string(),
        primary_adapter: "codex_app_server".to_string(),
        openai_oauth_adapter: "codex_app_server".to_string(),
        deepseek_api_key_env: Some(api_key_env.to_string()),
        deepseek_base_url: Some("https://api.deepseek.com".to_string()),
        deepseek_default_model: Some("deepseek-v4-flash".to_string()),
        deepseek_reasoning_model: Some("deepseek-v4-pro".to_string()),
        deepseek_legacy_alias_deprecated_after: Some("2026-07-24".to_string()),
        meta_ai_model: Some("deepseek-v4-pro".to_string()),
        meta_ai_thinking_enabled: Some(true),
        fallback_adapter: Some("deepseek_openai_compatible_proxy".to_string()),
        auth_profiles_path: Some(auth_profiles_path.display().to_string()),
        secrets_env_path: Some(secrets_env_path.display().to_string()),
        provider_profile_source_url: Some("https://api-docs.deepseek.com/".to_string()),
        provider_profile_checked_at: Some("2026-05-20".to_string()),
        provider_profile_stale_after_days: 14,
        stores_api_key_values: false,
    };
    fs::write(
        path,
        serde_json::to_vec_pretty(&config).map_err(json_error)?,
    )
    .map_err(io_error)?;
    set_private_file(path)?;
    Ok(config)
}

pub fn write_deepseek_secret_from_env_file(
    source_env: &Path,
    secrets_env: &Path,
    key_name: &str,
) -> DevToolResult<()> {
    if !is_env_var_name(key_name) {
        return Err(DevToolError::new("secret key name must be an env var name"));
    }
    ensure_repo_external_path(secrets_env, "secret destination")?;
    let text = fs::read_to_string(source_env).map_err(io_error)?;
    let value = text
        .lines()
        .filter_map(parse_env_line)
        .find(|(key, _)| key == key_name)
        .map(|(_, value)| value)
        .ok_or_else(|| DevToolError::new(format!("{key_name} not found in env file")))?;
    if value.trim().is_empty() {
        return Err(DevToolError::new(format!("{key_name} is empty")));
    }
    ensure_private_parent(secrets_env)?;
    fs::write(secrets_env, format!("{key_name}={value}\n")).map_err(io_error)?;
    set_private_file(secrets_env)
}

pub fn default_turingos_home() -> PathBuf {
    if let Some(path) = std::env::var_os("TURINGOS_HOME") {
        return PathBuf::from(path);
    }
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".turingos")
}

pub fn default_provider_profiles_path() -> PathBuf {
    default_turingos_home().join("provider-profiles.json")
}

pub fn default_auth_profiles_path() -> PathBuf {
    default_turingos_home().join("auth-profiles.json")
}

pub fn default_secrets_env_path() -> PathBuf {
    default_turingos_home().join("secrets.env")
}

fn ensure_private_parent(path: &Path) -> DevToolResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
        set_private_dir(parent)?;
    }
    Ok(())
}

fn ensure_auth_profiles_file(path: &Path) -> DevToolResult<()> {
    if path.exists() {
        set_private_file(path)?;
        return Ok(());
    }
    ensure_private_parent(path)?;
    let value = json!({
        "schema": "turingos.v5.auth_profiles.v1",
        "profiles": {}
    });
    fs::write(path, serde_json::to_vec_pretty(&value).map_err(json_error)?).map_err(io_error)?;
    set_private_file(path)
}

fn is_env_var_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_uppercase()) {
        return false;
    }
    if !chars.all(|ch| ch == '_' || ch.is_ascii_uppercase() || ch.is_ascii_digit()) {
        return false;
    }
    if !name.contains('_') {
        return false;
    }
    name.ends_with("_KEY")
        || name.ends_with("_API_KEY")
        || name.ends_with("_TOKEN")
        || name.ends_with("_SECRET")
        || name.ends_with("_CREDENTIAL")
}

fn ensure_repo_external_path(path: &Path, label: &str) -> DevToolResult<()> {
    let repo = repo_root().or_else(|| std::env::current_dir().ok());
    let repo = repo.ok_or_else(|| DevToolError::new("could not determine repo boundary"))?;
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo.join(path)
    };
    if absolute.starts_with(&repo) {
        return Err(DevToolError::new(format!(
            "{label} must be outside the repo"
        )));
    }
    Ok(())
}

fn repo_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join(".git").exists() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn parse_env_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let (key, value) = trimmed.split_once('=')?;
    let value = value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string();
    Some((key.trim().to_string(), value))
}

#[cfg(unix)]
fn set_private_dir(path: &Path) -> DevToolResult<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(io_error)
}

#[cfg(not(unix))]
fn set_private_dir(_path: &Path) -> DevToolResult<()> {
    Ok(())
}

#[cfg(unix)]
fn set_private_file(path: &Path) -> DevToolResult<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(io_error)
}

#[cfg(not(unix))]
fn set_private_file(_path: &Path) -> DevToolResult<()> {
    Ok(())
}

pub fn derive_board(store: &Path) -> DevToolResult<Value> {
    let records = read_records(store)?;
    let mut tasks: BTreeMap<String, Value> = BTreeMap::new();
    let mut source_hashes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut broadcast_order = Vec::new();
    let mut broadcasted = BTreeSet::new();
    let mut pr_atoms: BTreeMap<u64, String> = BTreeMap::new();

    for record in &records {
        match event_type(record)?.as_str() {
            "DevTaskCreated" => {
                let atom_id = payload_string(record, "atom_id")?;
                tasks.insert(atom_id.clone(), record.payload.clone());
                source_hashes
                    .entry(atom_id)
                    .or_default()
                    .push(record.record_hash.clone());
            }
            "TaskBroadcasted" => {
                let atom_id = payload_string(record, "atom_id")?;
                if broadcasted.insert(atom_id.clone()) {
                    broadcast_order.push(atom_id.clone());
                }
                source_hashes
                    .entry(atom_id)
                    .or_default()
                    .push(record.record_hash.clone());
            }
            "TaskSuperseded" => {
                let atom_id = payload_string(record, "atom_id")?;
                tasks.remove(&atom_id);
                broadcasted.remove(&atom_id);
                source_hashes
                    .entry(atom_id)
                    .or_default()
                    .push(record.record_hash.clone());
            }
            "TaskClaimed" => {
                let atom_id = payload_string(record, "atom_id")?;
                remember_pr_atom(&record.payload, &atom_id, &mut pr_atoms);
                if let Some(task) = tasks.get_mut(&atom_id) {
                    task["status"] = json!("claimed");
                    copy_optional(&record.payload, task, "pr_number");
                }
                source_hashes
                    .entry(atom_id)
                    .or_default()
                    .push(record.record_hash.clone());
            }
            "WorkerReportSubmitted" => {
                let atom_id = payload_string(record, "atom_id")?;
                remember_pr_atom(&record.payload, &atom_id, &mut pr_atoms);
                if let Some(task) = tasks.get_mut(&atom_id) {
                    task["status"] = json!("pr_open");
                    copy_optional(&record.payload, task, "pr_number");
                }
                source_hashes
                    .entry(atom_id)
                    .or_default()
                    .push(record.record_hash.clone());
            }
            "MergeDecisionRecorded" => {
                let atom_id = payload_string(record, "atom_id")?;
                remember_pr_atom(&record.payload, &atom_id, &mut pr_atoms);
                if let Some(task) = tasks.get_mut(&atom_id) {
                    copy_as(&record.payload, task, "decision", "merge_decision");
                }
                source_hashes
                    .entry(atom_id)
                    .or_default()
                    .push(record.record_hash.clone());
            }
            "PRMerged" => {
                let atom_id = record
                    .payload
                    .get("atom_id")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .or_else(|| {
                        record
                            .payload
                            .get("pr_number")
                            .and_then(Value::as_u64)
                            .and_then(|pr_number| pr_atoms.get(&pr_number).cloned())
                    });
                let Some(atom_id) = atom_id else {
                    continue;
                };
                remember_pr_atom(&record.payload, &atom_id, &mut pr_atoms);
                if let Some(task) = tasks.get_mut(&atom_id) {
                    task["status"] = json!("merged");
                    copy_optional(&record.payload, task, "pr_number");
                    copy_optional(&record.payload, task, "pr_url");
                    copy_optional(&record.payload, task, "merge_method");
                    copy_optional(&record.payload, task, "main_after");
                    copy_optional(&record.payload, task, "merge_commit_sha");
                    copy_optional(&record.payload, task, "squash_commit_sha");
                    copy_optional(&record.payload, task, "merge_decision_cid");
                }
                source_hashes
                    .entry(atom_id)
                    .or_default()
                    .push(record.record_hash.clone());
            }
            _ => {}
        }
    }

    let rows: Vec<Value> = broadcast_order
        .into_iter()
        .filter_map(|atom_id| {
            let task = tasks.get(&atom_id)?;
            Some(board_row(
                &atom_id,
                task,
                source_hashes.get(&atom_id).cloned().unwrap_or_default(),
            ))
        })
        .collect();

    let all_source_hashes: Vec<Value> = records
        .iter()
        .map(|record| Value::String(record.record_hash.clone()))
        .collect();

    Ok(json!({
        "board_version": "v0.7",
        "generated_at": "1970-01-01T00:00:00Z",
        "generated_by_role": "meta",
        "source": "devtape_derived",
        "board_writer": "meta-only",
        "default_worker_profile": {
            "allowed_class": 1,
            "capabilities": ["docs", "harness"]
        },
        "runtime_boundary": {
            "development_control_plane_only": true,
            "runtime_truth": false
        },
        "default_duplicate_policy": "first_valid_pr_wins",
        "max_repair_attempts": 3,
        "worker_halt_required": true,
        "conflict_policy": "supersede_on_dirty",
        "source_event_cids": all_source_hashes,
        "tasks": rows
    }))
}

fn short_hash(hash: &str) -> String {
    hash.strip_prefix("sha256:")
        .and_then(|rest| rest.get(..12))
        .unwrap_or(hash)
        .to_string()
}

fn welcome_action(selected: bool, key: &str, title: &str, detail: &str) -> String {
    let pointer = if selected { "▸" } else { " " };
    let accent = if selected {
        color("blue")
    } else {
        color("muted")
    };
    let title_color = if selected {
        color("bold")
    } else {
        color("reset")
    };
    format!(
        "{}{} {} {:<22}{} {}{}{}",
        accent,
        pointer,
        key,
        title,
        color("reset"),
        title_color,
        truncate(detail, 72),
        color("reset")
    )
}

fn color(name: &str) -> &'static str {
    match name {
        "blue" => "\x1b[38;5;75m",
        "cyan" => "\x1b[38;5;81m",
        "green" => "\x1b[38;5;114m",
        "yellow" => "\x1b[38;5;179m",
        "muted" => "\x1b[38;5;245m",
        "bold" => "\x1b[1m",
        "reset" => "\x1b[0m",
        _ => "\x1b[0m",
    }
}

fn truncate(value: &str, width: usize) -> String {
    if value.len() <= width {
        return value.to_string();
    }
    if width <= 3 {
        return value[..width].to_string();
    }
    format!("{}...", &value[..width - 3])
}

pub fn audit_board_drift(store: &Path, board: &Value) -> DevToolResult<()> {
    let derived = derive_board(store)?;
    if &derived == board {
        Ok(())
    } else {
        Err(DevToolError::new(
            "TASK_BOARD drift: board does not match DevTape projection",
        ))
    }
}

pub fn merge_check(store: &Path, pr_number: u64) -> DevToolResult<MergeGateResult> {
    let records = read_records(store)?;
    let mut atom_id = None;
    let mut has_claim = false;
    let mut has_report = false;
    let mut has_audit = false;
    let mut has_veto = false;
    let mut latest_decision = None;

    for record in &records {
        let record_pr = record.payload.get("pr_number").and_then(Value::as_u64);
        if record_pr != Some(pr_number) {
            continue;
        }
        match event_type(record)?.as_str() {
            "TaskClaimed" => {
                has_claim = true;
                atom_id = record
                    .payload
                    .get("atom_id")
                    .and_then(Value::as_str)
                    .map(str::to_string);
            }
            "WorkerReportSubmitted" => has_report = true,
            "AuditVerdictSubmitted" => {
                has_audit = record.payload.get("verdict").and_then(Value::as_str) == Some("PASS");
            }
            "VetoVerdictSubmitted" => {
                has_veto = record.payload.get("verdict").and_then(Value::as_str) == Some("PASS");
            }
            "MergeDecisionRecorded" => {
                atom_id = record
                    .payload
                    .get("atom_id")
                    .and_then(Value::as_str)
                    .map(str::to_string);
                latest_decision = Some(record.payload.clone());
            }
            _ => {}
        }
    }

    let mut missing = Vec::new();
    if atom_id.is_none() {
        missing.push("TaskClaimed".to_string());
    }
    if !has_claim {
        push_missing(&mut missing, "TaskClaimed");
    }
    if !has_report {
        push_missing(&mut missing, "WorkerReportSubmitted");
    }
    if !has_audit {
        push_missing(&mut missing, "AuditVerdictSubmitted");
    }
    if !has_veto {
        push_missing(&mut missing, "VetoVerdictSubmitted");
    }
    let Some(decision) = latest_decision else {
        push_missing(&mut missing, "MergeDecisionRecorded");
        return Ok(MergeGateResult {
            decision: MergeGateDecision::HOLD,
            missing_evidence: missing,
            reasons: vec!["missing merge decision".to_string()],
        });
    };
    if !missing.is_empty() {
        return Ok(MergeGateResult {
            decision: MergeGateDecision::HOLD,
            missing_evidence: missing,
            reasons: vec!["missing required DevTape evidence".to_string()],
        });
    }

    let mut reasons = Vec::new();
    if decision.get("decision").and_then(Value::as_str) != Some("PROCEED") {
        reasons.push("merge decision is not PROCEED".to_string());
    }
    if !payload_bool(&decision, "required_ci_passed") {
        reasons.push("required CI did not pass".to_string());
    }
    if !payload_bool(&decision, "audit_passed") {
        reasons.push("audit did not pass".to_string());
    }
    if !payload_bool(&decision, "veto_passed") {
        reasons.push("veto did not pass".to_string());
    }
    if !payload_bool(&decision, "conversation_resolution") {
        reasons.push("conversations are unresolved".to_string());
    }
    if decision
        .get("branch_protection_snapshot")
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        reasons.push("missing branch protection snapshot".to_string());
    }
    if decision.get("merge_state_status").and_then(Value::as_str) != Some("CLEAN") {
        reasons.push("merge state is not CLEAN".to_string());
    }

    if reasons.is_empty() {
        Ok(MergeGateResult {
            decision: MergeGateDecision::PROCEED,
            missing_evidence: Vec::new(),
            reasons: Vec::new(),
        })
    } else {
        Ok(MergeGateResult {
            decision: MergeGateDecision::HOLD,
            missing_evidence: Vec::new(),
            reasons,
        })
    }
}

pub fn meta_reconcile_report(board: &Value, prs: &Value) -> DevToolResult<Value> {
    let tasks = board
        .get("tasks")
        .and_then(Value::as_array)
        .ok_or_else(|| DevToolError::new("board.tasks must be an array"))?;
    let prs = prs
        .as_array()
        .ok_or_else(|| DevToolError::new("prs must be an array"))?;

    let mut task_status: BTreeMap<String, String> = BTreeMap::new();
    let mut open_task_atoms = Vec::new();
    for task in tasks {
        let atom = task.get("atom_id").and_then(Value::as_str).unwrap_or("");
        if atom.is_empty() {
            continue;
        }
        let status = task.get("status").and_then(Value::as_str).unwrap_or("open");
        task_status.insert(atom.to_string(), status.to_string());
        if status == "open" {
            open_task_atoms.push(Value::String(atom.to_string()));
        }
    }

    let mut claims: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    let mut actions = Vec::new();
    for pr in prs {
        if let Some(atom) = claim_atom(pr) {
            claims.entry(atom).or_default().push(pr);
        } else {
            actions.push(pr_action(pr, Value::Null, "orphan_pr", false));
        }
    }

    for (atom, mut atom_prs) in claims {
        atom_prs.sort_by_key(|pr| pr_string(pr, "createdAt"));
        for (index, pr) in atom_prs.iter().enumerate() {
            let status = task_status.get(&atom).map(String::as_str);
            let needs_report = !has_worker_report(pr);
            let action = if index > 0 {
                "supersede_duplicate_claim"
            } else if matches!(
                status,
                Some("merged") | Some("superseded") | Some("retired")
            ) {
                "supersede_closed_task_claim"
            } else if pr_string(pr, "mergeStateStatus").eq_ignore_ascii_case("DIRTY") {
                "hold_dirty_claim"
            } else if status == Some("pr_open") && !needs_report && has_failed_check(pr) {
                "hold_failed_ci"
            } else if status == Some("pr_open")
                && !needs_report
                && pr_string(pr, "mergeStateStatus") != "CLEAN"
            {
                "hold_until_branch_updated"
            } else if status == Some("pr_open") && !needs_report {
                "run_merge_check"
            } else if status == Some("claimed") && needs_report {
                "await_worker_report"
            } else if needs_report {
                "record_task_claim"
            } else {
                "record_worker_report"
            };
            actions.push(pr_action(
                pr,
                Value::String(atom.clone()),
                action,
                needs_report,
            ));
        }
    }

    Ok(json!({
        "mode": "dry-run",
        "scanned_prs": prs.len(),
        "open_task_atoms": open_task_atoms,
        "actions": actions
    }))
}

pub fn create_worker_sandbox(task: &Value, repo: &Path, out: &Path) -> DevToolResult<Value> {
    if out.exists() {
        return Err(DevToolError::new("worker sandbox output already exists"));
    }
    let atom_id = task
        .get("atom_id")
        .and_then(Value::as_str)
        .ok_or_else(|| DevToolError::new("task.atom_id must be a string"))?;
    let allowed_files = string_array(task, "allowed_files")?;
    let forbidden_files = string_array(task, "forbidden_files")?;
    for path in &allowed_files {
        ensure_safe_relative_file(path, "allowed_files")?;
    }

    fs::create_dir_all(out.join("allowed_files")).map_err(io_error)?;
    fs::create_dir_all(out.join("submit")).map_err(io_error)?;
    for path in &allowed_files {
        let source = repo.join(path);
        if source.is_file() {
            let destination = out.join("allowed_files").join(path);
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent).map_err(io_error)?;
            }
            fs::copy(source, destination).map_err(io_error)?;
        }
    }

    let manifest = json!({
        "schema": "turingos.v5.worker_sandbox.v0",
        "atom_id": atom_id,
        "runtime_truth": false,
        "soft_sandbox": true,
        "allowed_files": allowed_files,
        "forbidden_files": forbidden_files,
        "submit_contract": {
            "candidate_patch": "submit/candidate.patch",
            "worker_report": "submit/WorkerReport.json",
            "halt_required": "[WORKER_HALT]"
        }
    });
    fs::write(
        out.join("sandbox_manifest.json"),
        serde_json::to_vec_pretty(&manifest).map_err(json_error)?,
    )
    .map_err(io_error)?;
    fs::write(out.join("TASK.md"), worker_task_text(task, &manifest)).map_err(io_error)?;
    fs::write(out.join("CONTEXT.md"), worker_context_text(task, &manifest)).map_err(io_error)?;
    Ok(manifest)
}

pub fn validate_worker_sandbox_submission(dir: &Path) -> DevToolResult<Value> {
    let manifest_path = dir.join("sandbox_manifest.json");
    let manifest: Value =
        serde_json::from_slice(&fs::read(manifest_path).map_err(io_error)?).map_err(json_error)?;
    let allowed_files = string_array(&manifest, "allowed_files")?;
    let forbidden_files = string_array(&manifest, "forbidden_files")?;
    let patch_path = dir.join("submit/candidate.patch");
    let report_path = dir.join("submit/WorkerReport.json");
    let patch = fs::read_to_string(&patch_path).map_err(io_error)?;
    let report = fs::read_to_string(&report_path).map_err(io_error)?;
    if !report.contains("[WORKER_HALT]") {
        return Err(DevToolError::new(
            "WorkerReport.json must contain [WORKER_HALT]",
        ));
    }

    let paths = patch_paths(&patch);
    if paths.is_empty() {
        return Err(DevToolError::new("candidate.patch touches no files"));
    }
    for path in &paths {
        ensure_safe_relative_file(path, "candidate.patch")?;
        if forbidden_files
            .iter()
            .any(|pattern| forbidden_match(pattern, path))
        {
            return Err(DevToolError::new(format!(
                "patch path {path} matches forbidden_files"
            )));
        }
        if !allowed_files.iter().any(|allowed| allowed == path) {
            return Err(DevToolError::new(format!(
                "patch path {path} is not in allowed_files"
            )));
        }
    }

    Ok(json!({
        "decision": "PASS",
        "submission_contract": "PASS",
        "acceptance_status": "not_run_by_sandbox_v0",
        "paths": paths,
        "runtime_truth": false
    }))
}

fn board_row(atom_id: &str, task: &Value, source_event_cids: Vec<String>) -> Value {
    json!({
        "atom_id": atom_id,
        "revision": number_or(task, "revision", 1),
        "title": task.get("title").cloned().unwrap_or_else(|| json!(atom_id)),
        "status": task.get("status").cloned().unwrap_or_else(|| json!("open")),
        "phase": task.get("phase").cloned().unwrap_or_else(|| json!("V5-K0")),
        "lane": task.get("lane").cloned().unwrap_or_else(|| json!("devtape")),
        "class": task.get("risk_class").cloned().unwrap_or_else(|| json!(0)),
        "priority": task.get("priority").cloned().unwrap_or_else(|| json!("P0")),
        "self_select": task.get("self_select").cloned().unwrap_or_else(|| json!(true)),
        "meta_opened": task.get("meta_opened").cloned().unwrap_or_else(|| json!(true)),
        "claim_mode": task.get("claim_mode").cloned().unwrap_or_else(|| json!("open_pool")),
        "claim_required": task.get("claim_required").cloned().unwrap_or_else(|| json!(true)),
        "claim_method": task.get("claim_method").cloned().unwrap_or_else(|| json!("draft_pr")),
        "required_capabilities": task.get("required_capabilities").cloned().unwrap_or_else(|| json!([])),
        "preferred_capabilities": task.get("preferred_capabilities").cloned().unwrap_or_else(|| json!([])),
        "allowed_files": task.get("allowed_files").cloned().unwrap_or_else(|| json!([])),
        "forbidden_files": task.get("forbidden_files").cloned().unwrap_or_else(|| json!([])),
        "task_packet": task.get("task_packet").cloned().unwrap_or_else(|| json!("")),
        "acceptance_tests": task.get("acceptance_criteria").cloned().unwrap_or_else(|| json!(["git diff --check"])),
        "duplicate_policy": task.get("duplicate_policy").cloned().unwrap_or_else(|| json!("first_valid_pr_wins")),
        "blockers": task.get("blockers").cloned().unwrap_or_else(|| json!([])),
        "pr_number": task.get("pr_number").cloned().unwrap_or(Value::Null),
        "pr_url": task.get("pr_url").cloned().unwrap_or(Value::Null),
        "merge_decision": task.get("merge_decision").cloned().unwrap_or(Value::Null),
        "merge_method": task.get("merge_method").cloned().unwrap_or(Value::Null),
        "main_after": task.get("main_after").cloned().unwrap_or(Value::Null),
        "merge_commit_sha": task.get("merge_commit_sha").cloned().unwrap_or(Value::Null),
        "squash_commit_sha": task.get("squash_commit_sha").cloned().unwrap_or(Value::Null),
        "merge_decision_cid": task.get("merge_decision_cid").cloned().unwrap_or(Value::Null),
        "source_event_cids": source_event_cids
    })
}

fn claim_atom(pr: &Value) -> Option<String> {
    let title = pr.get("title")?.as_str()?;
    let rest = title.strip_prefix("[CLAIM][")?;
    let (atom, _) = rest.split_once("]")?;
    if atom.is_empty() {
        None
    } else {
        Some(atom.to_string())
    }
}

fn has_worker_report(pr: &Value) -> bool {
    let body = pr.get("body").and_then(Value::as_str).unwrap_or("");
    body.contains("WorkerReport") && body.contains("[WORKER_HALT]")
}

fn has_failed_check(pr: &Value) -> bool {
    pr.get("statusCheckRollup")
        .and_then(Value::as_array)
        .is_some_and(|checks| {
            checks.iter().any(|check| {
                matches!(
                    check.get("conclusion").and_then(Value::as_str),
                    Some("FAILURE") | Some("CANCELLED") | Some("TIMED_OUT")
                )
            })
        })
}

fn pr_action(pr: &Value, atom_id: Value, action: &str, needs_worker_report: bool) -> Value {
    json!({
        "pr_number": pr.get("number").cloned().unwrap_or(Value::Null),
        "url": pr.get("url").cloned().unwrap_or(Value::Null),
        "atom_id": atom_id,
        "action": action,
        "needs_worker_report": needs_worker_report,
        "is_draft": pr.get("isDraft").cloned().unwrap_or(Value::Null),
        "created_at": pr.get("createdAt").cloned().unwrap_or(Value::Null),
        "merge_state_status": pr.get("mergeStateStatus").cloned().unwrap_or(Value::Null)
    })
}

fn pr_string(pr: &Value, key: &str) -> String {
    pr.get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn worker_task_text(task: &Value, manifest: &Value) -> String {
    let atom_id = task.get("atom_id").and_then(Value::as_str).unwrap_or("-");
    let title = task.get("title").and_then(Value::as_str).unwrap_or("-");
    let goal = task.get("goal").and_then(Value::as_str).unwrap_or("-");
    format!(
        "# Worker Sandbox Task\n\n\
         atom_id: {atom_id}\n\
         title: {title}\n\n\
         Goal:\n{goal}\n\n\
         This is a soft sandbox: it limits the context package and submission \
         contract, but it is not a hard OS security boundary.\n\n\
         Allowed files:\n{}\n\n\
         Submit exactly:\n\
         - submit/candidate.patch\n\
         - submit/WorkerReport.json\n\n\
         WorkerReport.json must contain [WORKER_HALT].\n",
        markdown_list(manifest.get("allowed_files").unwrap_or(&Value::Null))
    )
}

fn worker_context_text(task: &Value, manifest: &Value) -> String {
    format!(
        "# Worker Context\n\n\
         Read only the files exported under allowed_files/ and this task context.\n\
         Do not assume access to the full repository.\n\
         If an instruction asks for a repo document that is not exported here, \
         do not read the full repo; ask MetaAI for a richer context bundle.\n\n\
         Instructions:\n{}\n\n\
         Acceptance tests:\n{}\n\n\
         Forbidden files:\n{}\n",
        markdown_list(
            task.get("step_by_step_instructions")
                .unwrap_or(&Value::Null)
        ),
        markdown_list(task.get("acceptance_tests").unwrap_or(&Value::Null)),
        markdown_list(manifest.get("forbidden_files").unwrap_or(&Value::Null))
    )
}

fn markdown_list(value: &Value) -> String {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(|item| format!("- {item}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| "- none".to_string())
}

fn string_array(value: &Value, key: &str) -> DevToolResult<Vec<String>> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| DevToolError::new(format!("{key} must be an array")))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| DevToolError::new(format!("{key} entries must be strings")))
        })
        .collect()
}

fn ensure_safe_relative_file(path: &str, label: &str) -> DevToolResult<()> {
    let relative = Path::new(path);
    if path.is_empty() || relative.is_absolute() || path.contains('*') {
        return Err(DevToolError::new(format!(
            "{label} path must be a plain relative file path"
        )));
    }
    if relative
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::RootDir))
    {
        return Err(DevToolError::new(format!(
            "{label} path must not escape the sandbox"
        )));
    }
    Ok(())
}

fn patch_paths(patch: &str) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            for part in rest.split_whitespace().take(2) {
                if let Some(path) = strip_patch_prefix(part) {
                    paths.insert(path);
                }
            }
        } else if let Some(rest) = line.strip_prefix("--- ") {
            if let Some(part) = rest.split_whitespace().next() {
                if let Some(path) = strip_patch_prefix(part) {
                    paths.insert(path);
                }
            }
        } else if let Some(rest) = line.strip_prefix("+++ ") {
            if let Some(part) = rest.split_whitespace().next() {
                if let Some(path) = strip_patch_prefix(part) {
                    paths.insert(path);
                }
            }
        }
    }
    paths.into_iter().collect()
}

fn strip_patch_prefix(path: &str) -> Option<String> {
    if path == "/dev/null" {
        return None;
    }
    path.strip_prefix("a/")
        .or_else(|| path.strip_prefix("b/"))
        .or(Some(path))
        .map(str::to_string)
}

fn forbidden_match(pattern: &str, path: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix("/**") {
        path == prefix || path.starts_with(&format!("{prefix}/"))
    } else {
        pattern == path
    }
}

fn hash_json(value: &Value) -> DevToolResult<String> {
    let bytes = serde_json::to_vec(value).map_err(json_error)?;
    let digest = Sha256::digest(&bytes);
    Ok(format!("sha256:{}", hex(&digest)))
}

fn hex(bytes: &[u8]) -> String {
    const CHARS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(CHARS[(byte >> 4) as usize] as char);
        out.push(CHARS[(byte & 0x0f) as usize] as char);
    }
    out
}

fn fill_hash_field(envelope: &mut Value, path: &[&str], computed: &str) -> DevToolResult<()> {
    let value = value_at_mut(envelope, path)?;
    let current = value
        .as_str()
        .ok_or_else(|| DevToolError::new(format!("{} must be a string", path.join("."))))?;
    if current.is_empty() {
        return Err(DevToolError::new(format!("{} is missing", path.join("."))));
    }
    if current != "sha256:filled-by-append" && current != computed {
        return Err(DevToolError::new(format!(
            "{} does not match computed hash",
            path.join(".")
        )));
    }
    *value = Value::String(computed.to_string());
    Ok(())
}

fn event_type(record: &DevTapeRecord) -> DevToolResult<String> {
    string_at(&record.envelope, &["event_type"])
}

fn payload_string(record: &DevTapeRecord, key: &str) -> DevToolResult<String> {
    record
        .payload
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| DevToolError::new(format!("payload.{key} must be a string")))
}

fn string_at(value: &Value, path: &[&str]) -> DevToolResult<String> {
    value_at(value, path)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| DevToolError::new(format!("{} must be a string", path.join("."))))
}

fn bool_at(value: &Value, path: &[&str]) -> DevToolResult<bool> {
    value_at(value, path)?
        .as_bool()
        .ok_or_else(|| DevToolError::new(format!("{} must be a bool", path.join("."))))
}

fn value_at<'a>(value: &'a Value, path: &[&str]) -> DevToolResult<&'a Value> {
    let mut current = value;
    for key in path {
        current = current
            .get(*key)
            .ok_or_else(|| DevToolError::new(format!("missing {}", path.join("."))))?;
    }
    Ok(current)
}

fn value_at_mut<'a>(value: &'a mut Value, path: &[&str]) -> DevToolResult<&'a mut Value> {
    let mut current = value;
    for key in path {
        current = current
            .get_mut(*key)
            .ok_or_else(|| DevToolError::new(format!("missing {}", path.join("."))))?;
    }
    Ok(current)
}

fn payload_bool(payload: &Value, key: &str) -> bool {
    payload.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn copy_optional(from: &Value, to: &mut Value, key: &str) {
    if let Some(value) = from.get(key) {
        to[key] = value.clone();
    }
}

fn copy_as(from: &Value, to: &mut Value, source: &str, target: &str) {
    if let Some(value) = from.get(source) {
        to[target] = value.clone();
    }
}

fn remember_pr_atom(payload: &Value, atom_id: &str, pr_atoms: &mut BTreeMap<u64, String>) {
    if let Some(pr_number) = payload.get("pr_number").and_then(Value::as_u64) {
        pr_atoms.insert(pr_number, atom_id.to_string());
    }
}

fn number_or(value: &Value, key: &str, fallback: u64) -> Value {
    value
        .get(key)
        .and_then(Value::as_u64)
        .map(Value::from)
        .unwrap_or_else(|| Value::from(fallback))
}

fn push_missing(missing: &mut Vec<String>, item: &str) {
    if !missing.iter().any(|existing| existing == item) {
        missing.push(item.to_string());
    }
}

fn io_error(error: std::io::Error) -> DevToolError {
    DevToolError::new(error.to_string())
}

fn json_error(error: serde_json::Error) -> DevToolError {
    DevToolError::new(error.to_string())
}
