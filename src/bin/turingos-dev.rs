use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};
use turingosv5::devtool::{
    append_event, apply_worker_sandbox_submission, audit_board_drift, console_text,
    create_worker_sandbox, derive_board, merge_check, meta_reconcile_report, read_records,
    validate_worker_sandbox_submission, AppendInput, MergeGateDecision,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if matches!(args.as_slice(), [arg] if arg == "--help" || arg == "-h") {
        println!("{}", usage());
        return Ok(());
    }
    match args.as_slice() {
        [event, append, rest @ ..] if event == "event" && append == "append" => {
            let file = flag_path(rest, "--file")?;
            let store = flag_path(rest, "--store")?;
            let input: AppendInput =
                serde_json::from_slice(&fs::read(&file).map_err(|err| err.to_string())?)
                    .map_err(|err| err.to_string())?;
            let record = append_event(&store, input).map_err(|err| err.to_string())?;
            println!("{}", record.record_hash);
            Ok(())
        }
        [board, derive, rest @ ..] if board == "board" && derive == "derive" => {
            let store = flag_path(rest, "--store")?;
            let out = flag_path(rest, "--out")?;
            let board = derive_board(&store).map_err(|err| err.to_string())?;
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent).map_err(|err| err.to_string())?;
            }
            fs::write(
                out,
                serde_json::to_vec_pretty(&board).map_err(|err| err.to_string())?,
            )
            .map_err(|err| err.to_string())?;
            Ok(())
        }
        [audit, rest @ ..] if audit == "audit" => {
            let store = flag_path(rest, "--store")?;
            let board_path = flag_path(rest, "--board")?;
            let board: Value =
                serde_json::from_slice(&fs::read(board_path).map_err(|err| err.to_string())?)
                    .map_err(|err| err.to_string())?;
            audit_board_drift(&store, &board).map_err(|err| err.to_string())?;
            println!("AUDIT_PASS");
            Ok(())
        }
        [merge, check, rest @ ..] if merge == "merge" && check == "check" => {
            let store = flag_path(rest, "--store")?;
            let pr = flag_value(rest, "--pr")?
                .parse::<u64>()
                .map_err(|err| format!("--pr must be an integer: {err}"))?;
            let result = merge_check(&store, pr).map_err(|err| err.to_string())?;
            println!(
                "{}",
                serde_json::json!({
                    "decision": format!("{:?}", result.decision),
                    "missing_evidence": result.missing_evidence,
                    "reasons": result.reasons
                })
            );
            if result.decision == MergeGateDecision::PROCEED {
                Ok(())
            } else {
                Err("merge gate did not proceed".to_string())
            }
        }
        [loop_cmd, once, rest @ ..] if loop_cmd == "loop" && once == "once" => {
            let store = flag_path(rest, "--store")?;
            let board_out = flag_path(rest, "--board-out")?;
            let result = loop_once(&store, &board_out, optional_flag_path(rest, "--prs-file"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(&result).map_err(|err| err.to_string())?
            );
            Ok(())
        }
        [meta, reconcile, rest @ ..] if meta == "meta" && reconcile == "reconcile" => {
            let dry_run = rest.iter().any(|arg| arg == "--dry-run");
            let append = rest.iter().any(|arg| arg == "--append");
            if dry_run == append {
                return Err(
                    "meta reconcile requires exactly one of --dry-run or --append".to_string(),
                );
            }
            let board_path = flag_path(rest, "--board")?;
            let board: Value =
                serde_json::from_slice(&fs::read(board_path).map_err(|err| err.to_string())?)
                    .map_err(|err| err.to_string())?;
            let prs = if let Some(path) = optional_flag_path(rest, "--prs-file") {
                serde_json::from_slice(&fs::read(path).map_err(|err| err.to_string())?)
                    .map_err(|err| err.to_string())?
            } else {
                github_open_prs()?
            };
            let report = meta_reconcile_report(&board, &prs).map_err(|err| err.to_string())?;
            if append {
                let store = flag_path(rest, "--store")?;
                let record = append_meta_reconcile(&store, report)?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "record_hash": record.record_hash,
                        "event_type": record.envelope["event_type"],
                        "payload": record.payload
                    }))
                    .map_err(|err| err.to_string())?
                );
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
                );
            }
            Ok(())
        }
        [worker, sandbox, create, rest @ ..]
            if worker == "worker" && sandbox == "sandbox" && create == "create" =>
        {
            let task_path = flag_path(rest, "--task")?;
            let repo = flag_path(rest, "--repo")?;
            let out = flag_path(rest, "--out")?;
            let task: Value =
                serde_json::from_slice(&fs::read(task_path).map_err(|err| err.to_string())?)
                    .map_err(|err| err.to_string())?;
            let manifest =
                create_worker_sandbox(&task, &repo, &out).map_err(|err| err.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&manifest).map_err(|err| err.to_string())?
            );
            Ok(())
        }
        [worker, sandbox, validate, rest @ ..]
            if worker == "worker" && sandbox == "sandbox" && validate == "validate" =>
        {
            let dir = flag_path(rest, "--dir")?;
            let result = validate_worker_sandbox_submission(&dir).map_err(|err| err.to_string())?;
            println!("SANDBOX_SUBMISSION_PASS");
            println!(
                "{}",
                serde_json::to_string_pretty(&result).map_err(|err| err.to_string())?
            );
            Ok(())
        }
        [worker, sandbox, apply, rest @ ..]
            if worker == "worker" && sandbox == "sandbox" && apply == "apply" =>
        {
            let dir = flag_path(rest, "--dir")?;
            let worktree = flag_path(rest, "--worktree")?;
            let result =
                apply_worker_sandbox_submission(&dir, &worktree).map_err(|err| err.to_string())?;
            println!("SANDBOX_APPLY_PASS");
            println!(
                "{}",
                serde_json::to_string_pretty(&result).map_err(|err| err.to_string())?
            );
            Ok(())
        }
        [worker, claim, next, rest @ ..]
            if worker == "worker" && claim == "claim" && next == "next" =>
        {
            let store = flag_path(rest, "--store")?;
            let repo = flag_path(rest, "--repo")?;
            let out_root = flag_path(rest, "--out-root")?;
            let worker_slot = flag_value(rest, "--worker")?;
            let result = worker_claim_next(&store, &repo, &out_root, &worker_slot)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&result).map_err(|err| err.to_string())?
            );
            Ok(())
        }
        [console, rest @ ..] if console == "console" => run_console(rest),
        _ => Err(usage()),
    }
}

fn loop_once(
    store: &std::path::Path,
    board_out: &std::path::Path,
    prs_file: Option<PathBuf>,
) -> Result<Value, String> {
    let board = derive_board(store).map_err(|err| err.to_string())?;
    if let Some(parent) = board_out.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    fs::write(
        board_out,
        serde_json::to_vec_pretty(&board).map_err(|err| err.to_string())?,
    )
    .map_err(|err| err.to_string())?;
    audit_board_drift(store, &board).map_err(|err| err.to_string())?;

    let prs = if let Some(path) = prs_file {
        serde_json::from_slice(&fs::read(path).map_err(|err| err.to_string())?)
            .map_err(|err| err.to_string())?
    } else {
        github_open_prs()?
    };
    let report = meta_reconcile_report(&board, &prs).map_err(|err| err.to_string())?;
    let reconcile = append_meta_reconcile_with_trigger(store, report.clone(), "loop_once")?;
    let followups = append_reconcile_followups(store, &report)?;
    Ok(serde_json::json!({
        "mode": "loop_once",
        "board_out": board_out.display().to_string(),
        "meta_reconcile_record": reconcile.record_hash,
        "followup_records": followups,
        "merge_executed": false,
        "report": report
    }))
}

fn append_meta_reconcile(
    store: &std::path::Path,
    report: Value,
) -> Result<turingosv5::devtool::DevTapeRecord, String> {
    append_meta_reconcile_with_trigger(store, report, "manual_cli")
}

fn append_meta_reconcile_with_trigger(
    store: &std::path::Path,
    report: Value,
    trigger: &str,
) -> Result<turingosv5::devtool::DevTapeRecord, String> {
    let previous = read_records(store)
        .map_err(|err| err.to_string())?
        .last()
        .map(|record| record.record_hash.clone());
    append_event(
        store,
        AppendInput {
            previous_record_hash: previous.clone(),
            envelope: event_envelope("MetaReconcileRecorded", previous),
            payload: serde_json::json!({
                "mode": "append",
                "trigger": trigger,
                "report": report
            }),
        },
    )
    .map_err(|err| err.to_string())
}

fn append_reconcile_followups(store: &std::path::Path, report: &Value) -> Result<Value, String> {
    let mut records = Vec::new();
    for action in report
        .get("actions")
        .and_then(Value::as_array)
        .ok_or_else(|| "report.actions must be an array".to_string())?
    {
        let Some(event_type) = followup_event_type(action.get("action").and_then(Value::as_str))
        else {
            continue;
        };
        let previous = read_records(store)
            .map_err(|err| err.to_string())?
            .last()
            .map(|record| record.record_hash.clone());
        let record = append_event(
            store,
            AppendInput {
                previous_record_hash: previous.clone(),
                envelope: event_envelope(event_type, previous),
                payload: serde_json::json!({
                    "source_action": action,
                    "merge_executed": false
                }),
            },
        )
        .map_err(|err| err.to_string())?;
        records.push(serde_json::json!({
            "event_type": event_type,
            "record_hash": record.record_hash
        }));
    }
    Ok(Value::Array(records))
}

fn followup_event_type(action: Option<&str>) -> Option<&'static str> {
    match action? {
        "await_worker_report" => Some("WorkerFollowupRequested"),
        "hold_failed_ci" | "hold_dirty_claim" => Some("RepairTaskCreated"),
        "hold_until_branch_updated" => Some("BranchUpdateRequested"),
        "supersede_duplicate_claim" => Some("DuplicateClaimRecorded"),
        "run_merge_check" => Some("MergeCheckRequested"),
        _ => None,
    }
}

fn worker_claim_next(
    store: &std::path::Path,
    repo: &std::path::Path,
    out_root: &std::path::Path,
    worker_slot: &str,
) -> Result<Value, String> {
    let board = derive_board(store).map_err(|err| err.to_string())?;
    let Some(task) = next_claimable_task(&board) else {
        return Ok(serde_json::json!({
            "decision": "NO_ELIGIBLE_TASK",
            "runtime_truth": false
        }));
    };
    let atom_id = task
        .get("atom_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "task.atom_id must be a string".to_string())?;
    let task_packet = task
        .get("task_packet")
        .and_then(Value::as_str)
        .ok_or_else(|| "task.task_packet must be a string".to_string())?;
    if task_packet.is_empty() {
        return Err(format!("task {atom_id} has empty task_packet"));
    }

    let task_path = path_from_repo(repo, task_packet);
    let task_json: Value = serde_json::from_slice(
        &fs::read(&task_path).map_err(|err| format!("read {}: {err}", task_path.display()))?,
    )
    .map_err(|err| err.to_string())?;
    let sandbox_dir = out_root
        .join(safe_segment(worker_slot))
        .join(safe_segment(atom_id));
    create_worker_sandbox(&task_json, repo, &sandbox_dir).map_err(|err| err.to_string())?;

    let previous = read_records(store)
        .map_err(|err| err.to_string())?
        .last()
        .map(|record| record.record_hash.clone());
    let observed_at = unix_timestamp();
    let record = append_event(
        store,
        AppendInput {
            previous_record_hash: previous.clone(),
            envelope: worker_event_envelope("TaskClaimed", previous),
            payload: serde_json::json!({
                "atom_id": atom_id,
                "worker_slot": worker_slot,
                "claim_method": "sandbox",
                "sandbox_path": sandbox_dir.display().to_string(),
                "task_packet": task_packet,
                "createdAt": observed_at,
                "runtime_truth": false
            }),
        },
    )
    .map_err(|err| err.to_string())?;

    Ok(serde_json::json!({
        "decision": "CLAIMED",
        "atom_id": atom_id,
        "sandbox_dir": sandbox_dir.display().to_string(),
        "task_packet": task_packet,
        "claim_record_hash": record.record_hash,
        "runtime_truth": false
    }))
}

fn next_claimable_task(board: &Value) -> Option<Value> {
    board.get("tasks")?.as_array()?.iter().find_map(|task| {
        let status = task.get("status").and_then(Value::as_str).unwrap_or("open");
        let self_select = task
            .get("self_select")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let blockers_empty = task
            .get("blockers")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty);
        if status == "open" && self_select && blockers_empty {
            Some(task.clone())
        } else {
            None
        }
    })
}

fn path_from_repo(repo: &std::path::Path, path: &str) -> PathBuf {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        path
    } else {
        repo.join(path)
    }
}

fn safe_segment(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn unix_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| format!("unix:{}", duration.as_secs()))
        .unwrap_or_else(|_| "unix:0".to_string())
}

fn event_envelope(event_type: &str, previous: Option<String>) -> Value {
    let observed_at = unix_timestamp();
    serde_json::json!({
        "event_id": format!("{event_type}-{observed_at}"),
        "event_type": event_type,
        "project_id": "turingosv5",
        "actor_identity_cid": "sha256:meta-reconcile-cli",
        "payload_cid": "sha256:filled-by-append",
        "previous_event_cid": previous,
        "observed_at": observed_at,
        "source": "turingos-dev meta reconcile",
        "subject": {
            "repo": "gretjia/turingosv5",
            "branch": null,
            "pr": null,
            "files": []
        },
        "evidence": {
            "commands": ["turingos-dev meta reconcile --append"],
            "artifacts": [],
            "source_anchors": []
        },
        "classification": {
            "risk_class": 1,
            "candidate": true,
            "runtime_truth": false
        },
        "integrity": {
            "payload_hash": "sha256:filled-by-append",
            "envelope_hash": "sha256:filled-by-append"
        }
    })
}

fn worker_event_envelope(event_type: &str, previous: Option<String>) -> Value {
    let observed_at = unix_timestamp();
    serde_json::json!({
        "event_id": format!("{event_type}-{observed_at}"),
        "event_type": event_type,
        "project_id": "turingosv5",
        "actor_identity_cid": "sha256:worker-claim-cli",
        "payload_cid": "sha256:filled-by-append",
        "previous_event_cid": previous,
        "observed_at": observed_at,
        "source": "turingos-dev worker claim next",
        "subject": {
            "repo": "gretjia/turingosv5",
            "branch": null,
            "pr": null,
            "files": []
        },
        "evidence": {
            "commands": ["turingos-dev worker claim next"],
            "artifacts": [],
            "source_anchors": []
        },
        "classification": {
            "risk_class": 1,
            "candidate": true,
            "runtime_truth": false
        },
        "integrity": {
            "payload_hash": "sha256:filled-by-append",
            "envelope_hash": "sha256:filled-by-append"
        }
    })
}

fn github_open_prs() -> Result<Value, String> {
    let output = Command::new("gh")
        .args([
            "pr",
            "list",
            "--state",
            "open",
            "--json",
            "number,title,headRefName,isDraft,createdAt,url,body,mergeStateStatus,statusCheckRollup",
        ])
        .output()
        .map_err(|err| err.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    serde_json::from_slice(&output.stdout).map_err(|err| err.to_string())
}

fn run_console(args: &[String]) -> Result<(), String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{}", console_usage());
        return Ok(());
    }
    let store = optional_flag_path(args, "--store").unwrap_or_else(default_store);
    println!("{}", console_text(&store).map_err(|err| err.to_string())?);
    Ok(())
}

fn flag_path(args: &[String], name: &str) -> Result<PathBuf, String> {
    flag_value(args, name).map(PathBuf::from)
}

fn optional_flag_path(args: &[String], name: &str) -> Option<PathBuf> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| PathBuf::from(window[1].clone()))
}

fn flag_value(args: &[String], name: &str) -> Result<String, String> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| window[1].clone())
        .ok_or_else(|| format!("missing {name}\n{}", usage()))
}

fn default_store() -> PathBuf {
    PathBuf::from(".turingos_system/devtape/turingosv5/events.jsonl")
}

fn console_usage() -> String {
    [
        "usage:",
        "  turingos-dev console [--store <events.jsonl>]",
        "",
        "read-only TuringOS V5 DevTape console.",
        "Renders the DevTape-derived board projection and does not write TASK_BOARD.json.",
    ]
    .join("\n")
}

fn usage() -> String {
    [
        "usage:",
        "  turingos-dev console [--store <events.jsonl>]",
        "  turingos-dev event append --file <event.json> --store <events.jsonl>",
        "  turingos-dev board derive --store <events.jsonl> --out <board.json>",
        "  turingos-dev audit --store <events.jsonl> --board <board.json>",
        "  turingos-dev merge check --store <events.jsonl> --pr <number>",
        "  turingos-dev loop once --store <events.jsonl> --board-out <board.json> [--prs-file <prs.json>]",
        "  turingos-dev meta reconcile --dry-run --board <board.json> [--prs-file <prs.json>]",
        "  turingos-dev meta reconcile --append --store <events.jsonl> --board <board.json> [--prs-file <prs.json>]",
        "  turingos-dev worker sandbox create --task <task.json> --repo <repo> --out <sandbox>",
        "  turingos-dev worker sandbox validate --dir <sandbox>",
        "  turingos-dev worker sandbox apply --dir <sandbox> --worktree <worktree>",
        "  turingos-dev worker claim next --store <events.jsonl> --repo <repo> --out-root <dir> --worker <worker>",
    ]
    .join("\n")
}
