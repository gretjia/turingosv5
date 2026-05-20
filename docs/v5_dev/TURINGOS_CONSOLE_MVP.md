# TuringOS Console MVP

Status: development console contract.

## Core Shape

The TuringOS V5 console is a read-only materialized view over DevTape:

```text
DevTape JSONL -> board projection -> console text view
```

It is not a truth source. It does not write `TASK_BOARD.json`, does not mutate
DevTape, and does not decide merge eligibility. Merge eligibility still comes
from DevTape evidence plus the merge gate.

## Current Command

```bash
turingos
```

In a real terminal this opens the interactive terminal UI. For scripts and
tests, use:

```bash
turingos --plain
turingos --tui-frame
turingos --welcome-frame
```

The explicit development command remains available:

```bash
turingos-dev console --store .turingos_system/devtape/turingosv5/events.jsonl
```

The command prints:

- the DevTape store path;
- record count and latest record hash;
- projected task rows;
- claim PR numbers when present.

## Welcome Page

The default TUI starts at the welcome page so a Human Architect can configure
MetaAI before dispatching work:

```text
1. OpenAI OAuth
   Use Codex app-server login. TuringOS does not store OAuth tokens.

2. DeepSeek API fallback
   Store only provider profile details and the environment variable name,
   normally DEEPSEEK_API_KEY.
```

TUI commands:

```text
↑/↓  Move selection on the welcome page
Enter  Confirm selected action
o  OpenAI OAuth guidance
d  Configure DeepSeek fallback as DEEPSEEK_API_KEY
c  Show DevTape console
r  Refresh current screen
h  Toggle console help
q  Quit
```

If raw terminal mode is unavailable, TuringOS falls back to line mode instead
of exiting. In line mode, type `up`/`down` or `k`/`j`, then press Enter.

For non-interactive setup:

```bash
turingos meta set-deepseek --api-key-env DEEPSEEK_API_KEY
turingos meta set-deepseek --api-key-env DEEPSEEK_API_KEY --from-env-file /home/zephryj/projects/turingosv4/.env
```

This writes `~/.turingos/provider-profiles.json`, not a file inside the V5
repo. The file records adapter names, provider profile details, and the API-key
environment variable name; the API key value remains outside repo state.
When `--from-env-file` is used, the matching secret value is copied into
`~/.turingos/secrets.env` with private file permissions. The provider profile
still does not contain the secret value.

TuringOS local state follows the OpenClaw-style split between provider profile
and auth material:

```text
~/.turingos/provider-profiles.json  provider/model profile cache
~/.turingos/auth-profiles.json      future OAuth refresh material / auth profiles
~/.turingos/secrets.env             optional local env file for API keys
```

On Unix, TuringOS creates `~/.turingos` as `0700` and local profile files as
`0600`. These paths are outside the repository and must never be committed,
copied into DevTape, or printed in WorkerReports with token values.

The local DeepSeek profile is a cache, not authority. As of 2026-05-20, the
profile records:

```text
base_url: https://api.deepseek.com
default_model: deepseek-v4-flash
reasoning_model: deepseek-v4-pro
meta_ai_model: deepseek-v4-pro
meta_ai_thinking_enabled: true
legacy_alias_deprecated_after: 2026-07-24
source: https://api-docs.deepseek.com/
stale_after_days: 14
```

MetaAI may use this cache to avoid user configuration mistakes, but before a
real provider change or long-running work it should refresh the profile from
the provider docs and update the local cache. This keeps the workflow practical
without turning a stale config file into a Software 2.0-style hard-coded truth.

If the store does not exist, it exits successfully and reports that DevTape is
not initialized. This lets a Human Architect enter the system without pretending
that bootstrap state already exists.

## LLM Adapter Boundary

The console may later call model tools, but model access is an adapter, not a
truth source.

Primary option:

```text
Codex app-server adapter
```

This adapter should spawn or connect to `codex app-server --listen stdio://`
and speak JSON-RPC. The console can request login by sending
`account/login/start` with either `type: "chatgpt"` or
`type: "chatgptDeviceCode"`.

This is not `codex exec`. The point is to let the console own the TuringOS loop
while Codex app-server owns ChatGPT OAuth, token refresh, model streaming, and
approval mechanics. TuringOS must not parse, copy, or persist Codex OAuth
tokens.

Fallback option:

```text
DeepSeek fallback through a local OpenAI-compatible proxy
```

The old system already used an OpenAI-compatible proxy shape. The V5 console
should keep the same boundary if this path is revived:

```text
TuringOS console -> localhost proxy -> DeepSeek OpenAI-compatible endpoint
```

Keys stay in environment variables such as `DEEPSEEK_API_KEY` and
`DEEPSEEK_API_KEY_SECONDARY`, or in a private local `~/.turingos/secrets.env`
file if a later adapter explicitly loads it. Secret values are never persisted
in repo files, DevTape records, board projections, WorkerReports, or console
logs.

## Non-Goals

- No curses dependency yet.
- No service, daemon, queue, watcher, or database.
- No direct cloud HTTPS call from the console MVP.
- No API key storage inside the V5 repo.
- No OpenAI OAuth token storage inside the V5 repo or DevTape.
- No board mutation from the console.
- No V5 runtime dependency on the console or DevTape.
