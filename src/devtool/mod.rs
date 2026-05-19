use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

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
    "DevTaskCreated",
    "TaskBroadcasted",
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

pub fn derive_board(store: &Path) -> DevToolResult<Value> {
    let records = read_records(store)?;
    let mut tasks: BTreeMap<String, Value> = BTreeMap::new();
    let mut source_hashes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut broadcast_order = Vec::new();
    let mut broadcasted = BTreeSet::new();

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
            "TaskClaimed" => {
                let atom_id = payload_string(record, "atom_id")?;
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
                if let Some(task) = tasks.get_mut(&atom_id) {
                    copy_as(&record.payload, task, "decision", "merge_decision");
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
        "merge_decision": task.get("merge_decision").cloned().unwrap_or(Value::Null),
        "source_event_cids": source_event_cids
    })
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
