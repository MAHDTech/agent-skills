+++
title = "failure-modes"
[extra]
skill = false
category = "guards"
mermaid = false
skill_name = "guards-debug"
+++

# GUARDS Plugin Failure Modes and Source Map

Use this playbook to map `agy-guards doctor` checks, session gaps, hook denials, spoke hangs, and factory stalls to concrete root causes and (when available) `agy-guards` source modules.

## 1. Interpreting `agy-guards doctor` checks and session gaps

### Project-level `checks`

- **`store` (`SchemaFit::Behind` or `SchemaFit::Ahead`):**
  - `Behind`: the database schema predates the running `agy-guards` binary. Running `agy-guards inspect <workspace_root>` opens the read-write migrator and upgrades `guards.sqlite3`.
  - `Ahead`: `~/.local/bin/agy-guards` is older than `guards.sqlite3`. Reinstall or rebuild `agy-guards` from the newer revision; do not delete the store unless discarding state is intended.
- **`installation` and `factory_installation`:**
  - Verify `~/.gemini/config/import_manifest.json` has a single `guards` import, `~/.gemini/config/hooks.json` matches `~/.gemini/config/agy-guards-hooks.receipt.json`, and `~/.gemini/config/plugins/guards/mcp_config.json` points to an executable `~/.local/bin/agy-guards`.
  - `factory_installation` additionally verifies `~/.gemini/config/agy-guards-installation.receipt.json` (or `--installation-receipt <path>`) against both the running binary hash and installed plugin assets (`plugin.json`, `mcp_config.json`, `hooks.json`, `README.md`, `agents/`, `rules/`, `skills/`). Rebuilding `~/.local/bin/agy-guards` with `cargo build` without running the installer invalidates factory provenance even when ordinary `installation` reports `ok`.
- **`github`:**
  - Verifies persona tokens from `~/.config/agy-guards/credentials` (`0600`) via `GET /user`.
  - Warns when `GUARDS_*` or `GITHUB_TOKEN` variables are exported in the shell environment (the engine ignores shell env tokens).
  - Warns when `GUARDS_HUB_TARS_GITHUB_TOKEN` and `GUARDS_SPOKE_DOYLE_GITHUB_TOKEN` authenticate as the same GitHub account, which blocks independent PR approval and `--merge`.
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
- `store_ahead`: store schema is newer than the `agy-guards` binary.
- `skipped_phase` / `deliverables`: a passed phase (`G.2` through `D.2`) is missing expected artifact files under `artifacts/<id>/` or lacks provenance entries in `.writers`.
- `skill_health_unchecked`: `skill_run: true` runs (backed by `.run.json`, such as `run-chores-*`) only verify artifact presence (`health_coverage: "artifacts_only"`).

## 2. Common runtime bug signatures

### A. Spoke subagent hang, `wait_for_spoke` timeout, or `hook denied a prod`

1. Inspect `logs/<session_id>/server.log` and `logs/spoke-continuations/` for `SPOKE_WAIT_PROGRESS`, `PROD_SENT`, and `"hook denied a prod"`.
2. Locate the spoke `<conversationId>` from `audit.log` (`process: "spoke"`) or `PROD_SENT`, then inspect `<agy_app_dir>/brain/<spokeConversationId>/.system_generated/`:
   - **Permission stall (`Ask` prompt in headless/background spoke):** check the last entry in `steps/` or `logs/transcript.jsonl`. Spokes require pre-approved `mcp(guards_spoke/*)` and workspace permissions; an interactive prompt stalls unattended execution.
   - **Wrong MCP server target:** check `server.log` for `"spoke subagents must call ServerName: \"guards_spoke\", never \"guards_hub\""`.
   - **Premature spoke stop without reporting:** spoke hit `model_stop`, `NO_TOOL_CALL`, rate limit, or crash before calling its `guards_spoke` completion tool. Check `messages/` to see whether hub prods arrived, and check `gate_send_message` in `server.log` for cleanup-lease or `prod_effect` refusals (`"hook denied a prod"`).
   - **Worktree drift or premature reap:** check `logs/cleanup/` and `~/.gemini/antigravity-cli/worktrees/<hubConversationId>/subagent-*`. Verify `Write` spokes were spawned with `Workspace: "branch"` and MCP inheritance disabled.

### B. Bundle gate or G.1 interview loop (`AwaitingApproval`, `AlreadyConsumed`, `Ambiguous`)

1. Inspect `~/.gemini/{antigravity-cli,antigravity}/brain/<hubConversationId>/.system_generated/logs/transcript.jsonl` (and `transcript_full.jsonl`).
2. Check four failure causes:
   - **128 KiB transcript tail cutoff:** `agy-guards` only inspects the last 128 KiB of the transcript file. Large tool outputs emitted after the user's answer push `"approved"` or `ask_question` completions out of window.
   - **Ambiguous multi-issue approval:** more than one session in the workspace is waiting at a gate, so bare `approved` returns `ApprovalAction::Ambiguous` and requires `approved <issue_number>`.
   - **Malformed `ask_question` reconciliation:** `accept_bundle_question` / `note_interview_outcome` did not see a completed answer matching the recorded question.
   - **Unblocking externally:** once logs are saved and human intent is clear, run `agy-guards approve <workspace_root> <issue_number>` from outside the run.

### C. Self-protection hook lockouts or `Scope::Unknown` denials

- **Activated caller lockout:** once `<state_base>/hook-callers/<conversationId>` exists, `pre-tool-use` blocks reads outside `<workspace_root>`, writes to `~/.gemini/`, and `run_command` strings mentioning `guards.sqlite3`, `~/.gemini`, or `agy-guards` subcommands other than `--version` and `inspect`. Always debug from a separate conversation outside `<workspace_root>`.
- **`Cannot verify the host conversation identity or GUARDS activity` (`Scope::Unknown`):** `pre-tool-use` could not verify caller metadata and failed closed. Inspect `~/.gemini/config/hooks.json`, `~/.gemini/config/agy-guards-hooks.receipt.json`, and `cli.log` for `agy-guards projected hook:` stderr errors.

### D. Stale Antigravity MCP schema cache

- Compare cached tool JSON files in `~/.gemini/antigravity-cli/mcp/guards_hub/` and `guards_spoke/` against `agy-guards tools` and `agy-guards tools --spoke`.
- If a binary upgrade added, renamed, or changed arguments on an MCP tool while the host kept cached schemas (or an old `agy-guards` hub MCP server PID stayed alive), restart the host `agy` process so MCP `tools/list` refreshes.

### E. Single-`agy` `/proc` workspace collision or wedged factory lease

- `agy-guards factory` scans `/proc/<pid>/cmdline`, `/proc/<pid>/cwd`, and `--add-dir` arguments, aborting with `workspace has existing agy processes: <pid>` if any `agy` process resolves to canonical `<workspace_root>`.
- It also holds a non-blocking `flock` at `~/.local/share/guards-factory/locks/<sha256_hex>`.
- Check `FACTORY_START_FAILURE.md`, `FACTORY_LEDGER.ndjson`, and live `/proc` PIDs before terminating orphaned `agy` or `agy-guards` processes.

## 3. Source localization map (`<plugin_root>/`)

When the `plugin-agy-guards` repository (`crates/agy-guards/Cargo.toml` and `antigravity/plugin.json`) is available locally, map runtime failures directly to these modules:

| Subsystem                                                                       | Source path under `<plugin_root>/`                                                                                                                                     |
| ------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI parsing, hook dispatch, `reset`, `gc`, `approve`, `inspect`                 | `crates/agy-guards/src/main.rs`, `crates/agy-guards/src/cli.rs`                                                                                                        |
| Hook scope activation, workspace/control-surface guards, projected hooks        | `crates/agy-guards/src/hook_scope.rs`, `crates/agy-guards/src/hook_projection.rs`, `crates/agy-guards/src/hooks/`                                                      |
| `doctor` checks, receipt provenance, `bwrap` check                              | `crates/agy-guards/src/doctor.rs`, `crates/agy-guards/src/doctor/installation.rs`                                                                                      |
| `monitor` timeline merge, deduplication, `--follow` exit rules                  | `crates/agy-guards/src/monitor.rs`                                                                                                                                     |
| JSONL `FileLayer`, `.seq` counter, secret redaction                             | `crates/agy-guards/src/logging.rs`                                                                                                                                     |
| Gemini `transcript.jsonl` 128 KiB tail parser and gate/interview reconciliation | `crates/agy-guards/src/transcript.rs`                                                                                                                                  |
| Spoke liveness hashing, continuations, `send_message` prods, artifact status    | `crates/agy-guards/src/spoke_monitor.rs`, `crates/agy-guards/src/spoke_continuation.rs`, `crates/agy-guards/src/prod_effect.rs`, `crates/agy-guards/src/spoke_artifact.rs` |
| Spoke worktree lease and `gc` reaper                                            | `crates/agy-guards/src/cleanup/`                                                                                                                                       |
| SQLite GUARDS store, `Door::Reset` ownership checks, MCP tools                  | `crates/agy-guards/src/store/`, `crates/agy-guards/src/ownership.rs`, `crates/agy-guards/src/tools/`                                                                   |
| Unattended factory supervisor, NDJSON ledger, `/proc` scanner, workspace lock   | `crates/agy-guards/src/factory/`                                                                                                                                       |
| Plugin manifests, persona subagents, rules, and internal skills                 | `antigravity/plugin.json`, `antigravity/mcp_config.json`, `antigravity/agents/`, `antigravity/rules/`, `antigravity/skills/`                                           |

