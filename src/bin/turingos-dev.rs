use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};
use turingosv5::devtool::{
    append_event, audit_board_drift, console_text, create_worker_sandbox, derive_board,
    merge_check, meta_reconcile_report, read_records, validate_worker_sandbox_submission,
    AppendInput, MergeGateDecision,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
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
        [console, rest @ ..] if console == "console" => run_console(rest),
        _ => Err(usage()),
    }
}

fn append_meta_reconcile(
    store: &std::path::Path,
    report: Value,
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
                "trigger": "manual_cli",
                "report": report
            }),
        },
    )
    .map_err(|err| err.to_string())
}

fn event_envelope(event_type: &str, previous: Option<String>) -> Value {
    let observed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| format!("unix:{}", duration.as_secs()))
        .unwrap_or_else(|_| "unix:0".to_string());
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
        "  turingos-dev meta reconcile --dry-run --board <board.json> [--prs-file <prs.json>]",
        "  turingos-dev meta reconcile --append --store <events.jsonl> --board <board.json> [--prs-file <prs.json>]",
        "  turingos-dev worker sandbox create --task <task.json> --repo <repo> --out <sandbox>",
        "  turingos-dev worker sandbox validate --dir <sandbox>",
    ]
    .join("\n")
}
