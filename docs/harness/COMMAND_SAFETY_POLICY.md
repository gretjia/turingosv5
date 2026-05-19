# Command Safety Policy

## Allowed By Default

- `git status`
- `git diff`
- `git diff --check`
- `git log`
- `git show`
- `git branch`
- `git switch assigned-branch`
- `rg`
- read-only `cat`, `sed`, `awk`
- `cargo check`
- `cargo test`
- `cargo fmt --check`
- `gh pr view`
- `gh pr checks`
- `gh pr list`

## Conditional

- `git push`: assigned branch only.
- `gh pr create`: assigned branch only.
- `cargo update`: Dependency PR only.
- shared contract edits: Contract PR only.
- CI edits: CI/harness PR only.

## Forbidden By Default

- `git push origin main`
- `git reset --hard`
- `git clean -fdx`
- `gh pr merge`
- `sudo`
- `curl | sh`
- `wget | sh`
- reading or printing secrets
- writing `constitution.md`
- writing `genesis_payload.toml` unless ratified
- editing `docs/harness/broadcast/TASK_BOARD.json` unless Meta broadcast PR
