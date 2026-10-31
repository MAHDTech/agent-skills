+++
title = "failure-modes"
[extra]
skill = false
category = "tars"
mermaid = false
skill_name = "tars-debug"
+++

# TARS Plugin Failure Modes and Source Map

Use this playbook to map `tars-agy doctor` checks, session gaps, hook denials, spoke hangs, and factory stalls to concrete root causes and (when available) `tars-agy` source modules.

## 1. Interpreting `tars-agy doctor` checks and session gaps

### Project-level `checks`

- **`store` (`SchemaFit::Behind` or `SchemaFit::Ahead`):**
  - `Behind`: the database schema predates the running `tars-agy` binary. Running `tars-agy inspect <workspace_root>` opens the read-write migrator and upgrades `guards.sqlite3`.
  - `Ahead`: `~/.local/bin/tars-agy` is older than `guards.sqlite3`. Reinstall or rebuild `tars-agy` from the newer revision; do not delete the store unless discarding state is intended.
- **`installation` and `factory_installation`:**
  - Verify `~/.gemini/config/import_manifest.json` has a single `tars` import, `~/.gemini/config/hooks.json` matches `~/.gemini/config/tars-hooks.receipt.json`, and `~/.gemini/config/plugins/tars/mcp_config.json` points to an executable `~/.local/bin/tars-agy`.
  - `factory_installation` additionally verifies `~/.gemini/config/tars-installation.receipt.json` (or `--installation-receipt <path>`) against both the running binary hash and installed plugin assets (`plugin.json`, `mcp_config.json`, `hooks.json`, `README.md`, `agents/`, `rules/`, `skills/`). Rebuilding `~/.local/bin/tars-agy` with `cargo build` without running the installer invalidates factory provenance even when ordinary `installation` reports `ok`.
- **`github`:**
  - Verifies persona tokens from `~/.config/tars/credentials` (`0600`) via `GET /user`.
  - Warns when `TARS_*` or `GITHUB_TOKEN` variables are exported in the shell environment (the engine ignores shell env tokens).
  - Warns when `TARS_HUB_TARS_GITHUB_TOKEN` and `TARS_SPOKE_DOYLE_GITHUB_TOKEN` authenticate as the same GitHub account, which blocks independent PR approval and `--merge`.
- **`open_calls`:**
  - `historical: false`: an MCP call (commonly `wait_for_spoke` or a slow GitHub operation) is still open in the active conversation or crashed mid-call.
  - `historical: true` with `liveness: "unknown"`: an unmatched `start` record left behind by an earlier conversation that exited abruptly.
- **`caller_identity`:**
  - Recent `hub` or `spoke` MCP calls arrived with an empty `conversation` field, indicating `pre-tool-use` did not run `note_caller` before `call_mcp_tool` (broken hook projection or non-projected host).
- **`recent_errors` (hook-denial blind spot):**
  - `doctor` intentionally treats `"hook denied..."` messages as policy enforcement rather than engine errors. When an agent loops or stalls on refused tool calls, `recent_errors` can still report `ok`; always search `logs/<session_id>/server.log` for `"hook denied"`.
- **`sandbox`:**
  - Checks `bwrap` (bubblewrap) namespace availability. Failures indicate missing `bwrap` or restricted unprivileged user namespaces on the Linux host.

### Per-session `gaps` and `deliverables`

- `missing_caller`: session calls lack caller conversation identity.
- `no_logs`: session exists in `guards.sqlite3` without a matching `logs/<session_id>/` directory.
- `stale`: session is non-terminal and inactive past `--stale-secs` (default `600`).
- `store_ahead`: store schema is newer than the `tars-agy` binary.
- `skipped_phase` / `deliverables`: a passed phase (`G.2` through `D.2`) is missing expected artifact files under `artifacts/<id>/` or lacks provenance entries in `.writers`.
- `skill_health_unchecked`: `skill_run: true` runs (backed by `.run.json`, such as `run-chores-*`) only verify artifact presence (`health_coverage: "artifacts_only"`).

## 2. Common runtime bug signatures

### A. Spoke subagent hang, `wait_for_spoke` timeout, or `hook denied a prod`

1. Inspect `logs/<session_id>/server.log` and `logs/spoke-continuations/` for `SPOKE_WAIT_PROGRESS`, `PROD_SENT`, and `"hook denied a prod"`.
2. Locate the spoke `<conversationId>` from `audit.log` (`process: "spoke"`) or `PROD_SENT`, then inspect `<agy_app_dir>/brain/<spokeConversationId>/.system_generated/`:
   - **Permission stall (`Ask` prompt in headless/background spoke):** check the last entry in `steps/` or `logs/transcript.jsonl`. Spokes require pre-approved `mcp(tars_spoke/*)` and workspace permissions; an interactive prompt stalls unattended execution.
   - **Wrong MCP server target:** check `server.log` for `"spoke subagents must call ServerName: \"tars_spoke\", never \"tars_hub\""`.
   - **Premature spoke stop without reporting:** spoke hit `model_stop`, `NO_TOOL_CALL`, rate limit, or crash before calling its `tars_spoke` completion tool. Check `messages/` to see whether hub prods arrived, and check `gate_send_message` in `server.log` for cleanup-lease or `prod_effect` refusals (`"hook denied a prod"`).
   - **Worktree drift or premature reap:** check `logs/cleanup/` and `~/.gemini/antigravity-cli/worktrees/<hubConversationId>/subagent-*`. Verify `Write` spokes were spawned with `Workspace: "branch"` and MCP inheritance disabled.

### B. Bundle gate or G.1 interview loop (`AwaitingApproval`, `AlreadyConsumed`, `Ambiguous`)

1. Inspect `~/.gemini/{antigravity-cli,antigravity}/brain/<hubConversationId>/.system_generated/logs/transcript.jsonl` (and `transcript_full.jsonl`).
2. Check four failure causes:
   - **128 KiB transcript tail cutoff:** `tars-agy` only inspects the last 128 KiB of the transcript file. Large tool outputs emitted after the user's answer push `"approved"` or `ask_question` completions out of window.
   - **Ambiguous multi-issue approval:** more than one session in the workspace is waiting at a gate, so bare `approved` returns `ApprovalAction::Ambiguous` and requires `approved <issue_number>`.
   - **Malformed `ask_question` reconciliation:** `accept_bundle_question` / `note_interview_outcome` did not see a completed answer matching the recorded question.
   - **Unblocking externally:** once logs are saved and human intent is clear, run `tars-agy approve <workspace_root> <issue_number>` from outside the run.

### C. Self-protection hook lockouts or `Scope::Unknown` denials

- **Activated caller lockout:** once `<state_base>/hook-callers/<conversationId>` exists, `pre-tool-use` blocks reads outside `<workspace_root>`, writes to `~/.gemini/`, and `run_command` strings mentioning `guards.sqlite3`, `~/.gemini`, or `tars-agy` subcommands other than `--version` and `inspect`. Always debug from a separate conversation outside `<workspace_root>`.
- **`Cannot verify the host conversation identity or TARS activity` (`Scope::Unknown`):** `pre-tool-use` could not verify caller metadata and failed closed. Inspect `~/.gemini/config/hooks.json`, `~/.gemini/config/tars-hooks.receipt.json`, and `cli.log` for `tars-agy projected hook:` stderr errors.

### D. Stale Antigravity MCP schema cache

- Compare cached tool JSON files in `~/.gemini/antigravity-cli/mcp/tars_hub/` and `tars_spoke/` against `tars-agy tools` and `tars-agy tools --spoke`.
- If a binary upgrade added, renamed, or changed arguments on an MCP tool while the host kept cached schemas (or an old `tars-agy` hub MCP server PID stayed alive), restart the host `agy` process so MCP `tools/list` refreshes.

### E. Single-`agy` `/proc` workspace collision or wedged factory lease

- `tars-agy factory` scans `/proc/<pid>/cmdline`, `/proc/<pid>/cwd`, and `--add-dir` arguments, aborting with `workspace has existing agy processes: <pid>` if any `agy` process resolves to canonical `<workspace_root>`.
- It also holds a non-blocking `flock` at `~/.local/share/tars-factory/locks/<sha256_hex>`.
- Check `FACTORY_START_FAILURE.md`, `FACTORY_LEDGER.ndjson`, and live `/proc` PIDs before terminating orphaned `agy` or `tars-agy` processes.

## 3. Source localization map (`<platform_root>/`)

When the `tars-agy` platform repository (`crates/agy/Cargo.toml` and `antigravity/plugin.json`) is available locally, map runtime failures directly to these modules:

| Subsystem                                                                       | Source path under `<platform_root>/`                                                                                                           |
| ------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI parsing, hook dispatch, `reset`, `gc`, `approve`, `inspect`                 | `crates/agy/src/main.rs`, `crates/agy/src/cli.rs`                                                                                              |
| Hook scope activation, workspace/control-surface guards, projected hooks        | `crates/agy/src/hook_scope.rs`, `crates/agy/src/hook_projection.rs`, `crates/agy/src/hooks/`                                                   |
| `doctor` checks, receipt provenance, `bwrap` check                              | `crates/agy/src/doctor.rs`, `crates/agy/src/doctor/installation.rs`                                                                            |
| `monitor` timeline merge, deduplication, `--follow` exit rules                  | `crates/agy/src/monitor.rs`                                                                                                                    |
| JSONL `FileLayer`, `.seq` counter, secret redaction                             | `crates/agy/src/logging.rs`                                                                                                                    |
| Gemini `transcript.jsonl` 128 KiB tail parser and gate/interview reconciliation | `crates/agy/src/transcript.rs`                                                                                                                 |
| Spoke liveness hashing, continuations, `send_message` prods, artifact status    | `crates/agy/src/spoke_monitor.rs`, `crates/agy/src/spoke_continuation.rs`, `crates/agy/src/prod_effect.rs`, `crates/agy/src/spoke_artifact.rs` |
| Spoke worktree lease and `gc` reaper                                            | `crates/agy/src/cleanup/`                                                                                                                      |
| SQLite GUARDS store, `Door::Reset` ownership checks, MCP tools                  | `crates/agy/src/store/`, `crates/agy/src/ownership.rs`, `crates/agy/src/tools/`                                                                |
| Unattended factory supervisor, NDJSON ledger, `/proc` scanner, workspace lock   | `crates/agy/src/factory/`                                                                                                                      |
| Plugin manifests, persona subagents, rules, and internal skills                 | `antigravity/plugin.json`, `antigravity/mcp_config.json`, `antigravity/agents/`, `antigravity/rules/`, `antigravity/skills/`                   |

