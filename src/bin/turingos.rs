use serde_json::Value;
use std::env;
use std::io::{self, IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::{self, Command};
use turingosv5::devtool::{
    console_frame, console_text, default_provider_profiles_path, derive_board,
    meta_ai_welcome_frame_with_selection, meta_reconcile_report, write_deepseek_fallback_config,
    write_deepseek_secret_from_env_file,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{}", usage());
        return Ok(());
    }

    let store = optional_flag_path(&args, "--store").unwrap_or_else(default_store);
    let meta_config =
        optional_flag_path(&args, "--meta-config").unwrap_or_else(default_meta_config);
    if args.len() >= 2 && args[0] == "meta" && args[1] == "set-deepseek" {
        let rest = &args[2..];
        let api_key_env = flag_value(rest, "--api-key-env")?;
        let config_path =
            optional_flag_path(rest, "--meta-config").unwrap_or_else(default_meta_config);
        let config = write_deepseek_fallback_config(&config_path, &api_key_env)
            .map_err(|err| err.to_string())?;
        if let Some(source_env) = optional_flag_path(rest, "--from-env-file") {
            let secrets_env = config
                .secrets_env_path
                .as_deref()
                .map(PathBuf::from)
                .unwrap_or_else(|| config_path.with_file_name("secrets.env"));
            write_deepseek_secret_from_env_file(&source_env, &secrets_env, &api_key_env)
                .map_err(|err| err.to_string())?;
        }
        println!(
            "DeepSeek fallback configured via env {} (secret value not stored)",
            config.deepseek_api_key_env.unwrap_or(api_key_env)
        );
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--plain") {
        println!("{}", console_text(&store).map_err(|err| err.to_string())?);
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--tui-frame") {
        println!(
            "{}",
            console_frame(&store, false).map_err(|err| err.to_string())?
        );
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--welcome-frame") {
        let selected = optional_flag_value(&args, "--selected")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(0);
        println!(
            "{}",
            meta_ai_welcome_frame_with_selection(&store, &meta_config, selected)
                .map_err(|err| err.to_string())?
        );
        return Ok(());
    }
    if io::stdin().is_terminal() && io::stdout().is_terminal() {
        return run_tui(&store, &meta_config);
    }
    println!("{}", console_text(&store).map_err(|err| err.to_string())?);
    Ok(())
}

fn run_tui(store: &std::path::Path, meta_config: &std::path::Path) -> Result<(), String> {
    let raw = RawTerminal::enter().ok();
    let mut show_help = false;
    let mut screen = Screen::Welcome;
    let mut welcome_selected = 0usize;
    let mut status = if raw.is_some() {
        String::new()
    } else {
        "Raw keyboard mode unavailable; type letters and press Enter.".to_string()
    };
    loop {
        let frame = match screen {
            Screen::Welcome => {
                meta_ai_welcome_frame_with_selection(store, meta_config, welcome_selected)
                    .map_err(|err| err.to_string())?
            }
            Screen::Console => console_frame(store, show_help).map_err(|err| err.to_string())?,
        };
        println!("{frame}");
        if !status.is_empty() {
            println!("\x1b[38;5;245m{status}\x1b[0m");
        }
        print!("\x1b[38;5;245mkey>\x1b[0m ");
        io::stdout().flush().map_err(|err| err.to_string())?;

        let key = if raw.is_some() {
            read_key()?
        } else {
            read_line_key()?
        };
        match key {
            Key::Quit | Key::Char('q') => {
                println!("\r\nleaving TuringOS");
                return Ok(());
            }
            Key::Up if screen == Screen::Welcome => {
                welcome_selected = welcome_selected.saturating_sub(1);
                status.clear();
            }
            Key::Down if screen == Screen::Welcome => {
                welcome_selected = (welcome_selected + 1).min(2);
                status.clear();
            }
            Key::Enter if screen == Screen::Welcome => match welcome_selected {
                0 => status = openai_oauth_status(),
                1 => {
                    write_deepseek_fallback_config(meta_config, "DEEPSEEK_API_KEY")
                        .map_err(|err| err.to_string())?;
                    status =
                        "DeepSeek fallback configured via DEEPSEEK_API_KEY; key value not stored."
                            .to_string();
                }
                _ => {
                    screen = Screen::Console;
                    status.clear();
                }
            },
            Key::Char('w') => {
                screen = Screen::Welcome;
                status.clear();
            }
            Key::Char('c') => {
                screen = Screen::Console;
                status.clear();
            }
            Key::Char('o') => {
                screen = Screen::Welcome;
                welcome_selected = 0;
                status = openai_oauth_status();
            }
            Key::Char('d') => {
                write_deepseek_fallback_config(meta_config, "DEEPSEEK_API_KEY")
                    .map_err(|err| err.to_string())?;
                screen = Screen::Welcome;
                welcome_selected = 1;
                status = "DeepSeek fallback configured via DEEPSEEK_API_KEY; key value not stored."
                    .to_string();
            }
            Key::Char('m') => {
                screen = Screen::Console;
                status = meta_reconcile_status(store);
            }
            Key::Char('h') => show_help = !show_help,
            Key::Char('r') | Key::Enter => status.clear(),
            Key::Char(other) => {
                show_help = true;
                status = format!("unknown command: {other}");
            }
            _ => {}
        }
    }
}

fn openai_oauth_status() -> String {
    "OpenAI OAuth: use Codex app-server account/login/start; token remains outside TuringOS."
        .to_string()
}

fn meta_reconcile_status(store: &std::path::Path) -> String {
    match meta_reconcile_once(store) {
        Ok(report) => {
            let scanned = report
                .get("scanned_prs")
                .and_then(Value::as_u64)
                .unwrap_or(0);
            let action_count = report
                .get("actions")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            let first_action = report
                .get("actions")
                .and_then(Value::as_array)
                .and_then(|actions| actions.first())
                .and_then(|action| action.get("action"))
                .and_then(Value::as_str)
                .unwrap_or("none");
            format!(
                "Meta reconcile dry-run: scanned {scanned} PR(s), {action_count} action(s), first action: {first_action}."
            )
        }
        Err(error) => format!("Meta reconcile failed: {error}"),
    }
}

fn meta_reconcile_once(store: &std::path::Path) -> Result<Value, String> {
    let board = derive_board(store).map_err(|err| err.to_string())?;
    let prs = github_open_prs()?;
    meta_reconcile_report(&board, &prs).map_err(|err| err.to_string())
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Up,
    Down,
    Enter,
    Quit,
    Char(char),
    Unknown,
}

fn read_key() -> Result<Key, String> {
    let mut stdin = io::stdin();
    let mut first = [0u8; 1];
    stdin
        .read_exact(&mut first)
        .map_err(|err| err.to_string())?;
    match first[0] {
        b'\r' | b'\n' => Ok(Key::Enter),
        3 => Ok(Key::Quit),
        27 => {
            let mut rest = [0u8; 2];
            stdin.read_exact(&mut rest).map_err(|err| err.to_string())?;
            match rest {
                [b'[', b'A'] => Ok(Key::Up),
                [b'[', b'B'] => Ok(Key::Down),
                _ => Ok(Key::Unknown),
            }
        }
        byte => Ok(Key::Char(byte as char)),
    }
}

fn read_line_key() -> Result<Key, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|err| err.to_string())?;
    match input.trim() {
        "" => Ok(Key::Enter),
        "up" | "k" => Ok(Key::Up),
        "down" | "j" => Ok(Key::Down),
        "quit" | "exit" => Ok(Key::Quit),
        value if value.len() == 1 => Ok(Key::Char(value.chars().next().unwrap_or_default())),
        _ => Ok(Key::Unknown),
    }
}

struct RawTerminal {
    state: String,
}

impl RawTerminal {
    fn enter() -> Result<Self, String> {
        let output = Command::new("stty")
            .arg("-g")
            .output()
            .map_err(|err| err.to_string())?;
        if !output.status.success() {
            return Err("failed to read terminal state with stty -g".to_string());
        }
        let state = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let status = Command::new("stty")
            .args(["raw", "-echo"])
            .status()
            .map_err(|err| err.to_string())?;
        if !status.success() {
            return Err("failed to enter raw terminal mode".to_string());
        }
        print!("\x1b[?25l");
        io::stdout().flush().map_err(|err| err.to_string())?;
        Ok(Self { state })
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        let _ = Command::new("stty").arg(&self.state).status();
        print!("\x1b[?25h\x1b[0m");
        let _ = io::stdout().flush();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Welcome,
    Console,
}

fn optional_flag_path(args: &[String], name: &str) -> Option<PathBuf> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| PathBuf::from(window[1].clone()))
}

fn optional_flag_value(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| window[1].clone())
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

fn default_meta_config() -> PathBuf {
    default_provider_profiles_path()
}

fn usage() -> String {
    [
        "usage:",
        "  turingos [--store <events.jsonl>]",
        "  turingos --plain [--store <events.jsonl>]",
        "  turingos --tui-frame [--store <events.jsonl>]",
        "  turingos --welcome-frame [--store <events.jsonl>] [--meta-config <config.json>]",
        "  turingos meta set-deepseek --api-key-env <ENV_NAME> [--from-env-file <PATH>]",
        "",
        "Enter the TuringOS V5 terminal UI.",
        "This entry does not write TASK_BOARD.json or mutate DevTape.",
        "MetaAI setup stores env-var references only, never API key values.",
    ]
    .join("\n")
}
