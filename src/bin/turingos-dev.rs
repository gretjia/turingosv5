use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};
use turingosv5::devtool::{
    append_event, audit_board_drift, console_text, derive_board, merge_check,
    meta_reconcile_report, AppendInput, MergeGateDecision,
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
            if !rest.iter().any(|arg| arg == "--dry-run") {
                return Err("meta reconcile currently requires --dry-run".to_string());
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
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
            );
            Ok(())
        }
        [console, rest @ ..] if console == "console" => run_console(rest),
        _ => Err(usage()),
    }
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
    ]
    .join("\n")
}
