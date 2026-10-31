+++
title = "tars-debug"
description = "Debug and recover the tars-agy Antigravity plugin from outside an active run using tars-agy doctor, inspect, monitor, TARS state logs, and ~/.gemini session transcripts. Use for stuck or failing TARS sessions, hooks, spokes, or factory shifts."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tars"
mermaid = false
+++


# TARS Debug

Diagnose and recover `tars-agy` sessions, projected hooks, spoke subagents, and unattended factory shifts from **outside** an activated TARS conversation.

Inside an active TARS run, self-protection hooks restrict reads to `<workspace_root>` and block `~/.gemini/`, `guards.sqlite3`, and `tars-agy` control commands (leaving only internal `tars-check-health`). Run this workflow from an external shell or agent session started outside `<workspace_root>`.

## Hard rules

- **Stay outside `<workspace_root>`:** Never launch a second `agy` host inside `<workspace_root>` while diagnosing it; single-`agy` ownership scans `/proc` (`cwd` and `--add-dir`) and activated callers in `$XDG_STATE_HOME/tars/hook-callers/` (or `~/.local/state/tars/hook-callers/`) trigger self-protection denials.
- **Never expose raw credentials:** Never print or copy secret values from `~/.config/tars/credentials` (`$TARS_CREDENTIALS_FILE` / `$XDG_CONFIG_HOME/tars/credentials`) or `~/.gemini/*/antigravity-oauth-token`. Check credential health via `tars-agy doctor`.
- **Bound `monitor` calls:** Never run bare `tars-agy monitor <workspace_root> <session_id> --follow` synchronously on a stalled non-terminal session; `--follow` loops every 500ms until a terminal state and blocks the agent turn indefinitely. Default to non-following dumps (`--json` or `--since <secs>`), or wrap live tailing with `timeout 15s` or a background task.
- **Snapshot before recovery:** Capture diagnostic logs and uncommitted spoke worktree diffs before running `approve`, `gc`, `reset`, or terminating stuck processes.
- **Preserve store and receipt integrity:** Never mutate `guards.sqlite3` with raw SQL or hand-edit `~/.gemini/config/hooks.json` or `tars-installation.receipt.json`. Use `tars-agy` subcommands and `tars-agy-installer`.

## 1. Establish external vantage and live processes

Identify `<workspace_root>` (canonical path to the customer repository) and optional `<session_id>` (`42`, `review-pr-13`, `batch-20261029`, `wave-batch-20261029`, or `run-chores-20261027-001`).

Check live `agy` and `tars-agy` processes before reading or mutating state so you can distinguish active execution from orphaned locks:

```bash
tars-agy --version
pgrep -af 'tars-agy|(^|/)agy( |$)' || true
```

When PIDs are present on Linux, inspect `/proc/<pid>/cwd` and `/proc/<pid>/cmdline` to see which `<workspace_root>`, `--spoke`, or `factory` instance owns each process.

## 2. Run read-only CLI triage in order

Run the read-only `tars-agy` diagnostics in this sequence:

1. **Run `doctor` first** (before `inspect`, because `inspect` auto-migrates a behind-version `guards.sqlite3` schema and masks `SchemaFit::Behind`):

   ```bash
   tars-agy doctor <workspace_root> [session_id] --recent-hours 48
   ```

   Pass `--installation-receipt <path>` when diagnosing a factory shift that uses a custom receipt path. Inspect `overall` (`healthy`, `attention`, `broken`), project `checks` (`store`, `sessions`, `readiness`, `logs`, `open_calls`, `caller_identity`, `recent_errors`, `github`, `installation`, `factory_installation`, `sandbox`), and `session` (`status`, `phase`, `last_spoke`, `open_calls`, `errors`, `slow_calls`, `deliverables`, `gaps`, `verdict`).

2. **Dump stored GUARDS state and events:**

   ```bash
   tars-agy inspect <workspace_root> [session_id]
   ```

   If `<session_id>` was unknown, pick the target session key from `inspect` output.

3. **Obtain the merged session timeline via `monitor`:**

   ```bash
   tars-agy monitor <workspace_root> <session_id> --json
   ```

   Use `--since <secs>` to narrow noisy histories, or omit `--json` for the human table (`HH:MM:SS  <audit|store|server>  <start|end|KIND|level>  <summary>`). Exit code `2` indicates nothing is recorded for `<session_id>` in either `guards.sqlite3` or `logs/<session_id>/`. When watching a genuinely active session advance, bound `--follow` explicitly:

   ```bash
   timeout 15s tars-agy monitor <workspace_root> <session_id> --follow
   ```

4. **Check published MCP tool surfaces** when investigating tool-call errors or binary upgrades:

   ```bash
   tars-agy tools
   tars-agy tools --spoke
   ```

## 3. Correlate TARS logs, `~/.gemini` transcripts, and factory state

- Read [log-and-state-map.md](@/skills/tars/tars-debug/resources/manual/log-and-state-map.md) when resolving `<state_base>` (`$XDG_STATE_HOME/tars` or `~/.local/state/tars`), decoding `server.log` / `audit.log` / `error.log` fields, locating `~/.gemini/` brain transcripts, inspecting `FACTORY_LEDGER.ndjson`, or enabling `TARS_LOG=debug`.
- Read [failure-modes.md](@/skills/tars/tars-debug/resources/manual/failure-modes.md) to match `doctor` warnings, hook denials, spoke continuation stalls, 128 KiB transcript tail cutoffs, MCP cache drift, or `/proc` lock collisions to root causes and `crates/agy/` source files.

Join evidence across the three runtime layers:

1. **TARS JSONL logs (`<state_base>/<owner>__<repo>/logs/`):**
   - Inspect `logs/<session_id>/{error.log,audit.log,server.log}` (plus project-level `logs/` and subsystem directories `spoke-continuations/`, `crew/`, `cleanup/`, `workflow-invocations/`, `feedback-publications/`, `publication-journal/`).
   - Search `server.log` explicitly for `"hook denied"` and `tars_agy::panic`; `doctor` excludes `"hook denied..."` policy enforcements from `recent_errors` even when a mis-scoped denial wedges an agent loop.
   - Extract `conversation` UUIDs (`hub` and `spoke` entries in `audit.log` or `PROD_SENT` events) and unmatched `call_id` values.

2. **Antigravity host state (`~/.gemini/antigravity-cli/` or `~/.gemini/antigravity/`):**
   - If `audit.log` lacks `conversation` (`missing_caller`), look up `<workspace_root>` in `~/.gemini/antigravity-cli/cache/last_conversations.json` and `conversation_metadata.json`.
   - Inspect `<agy_app_dir>/brain/<conversationId>/.system_generated/`:
     - `logs/transcript.jsonl` and `logs/transcript_full.jsonl` for `USER_INPUT`, `PLANNER_RESPONSE`, `TOOL_CALL`, and `ask_question` completions (remembering `tars-agy` reads only the last 128 KiB tail for approvals and interview answers).
     - `steps/` and `tasks/task-*.log` for spoke progress (`SPOKE_WAIT_PROGRESS`) or stuck interactive prompts.
     - `messages/` for queued `send_message` prods between hub and spoke.
   - Check `cli.log` (MCP stdio `stderr`, including `FileLayer` write failures and projected-hook errors), `crashes/crash_*.log`, cached schemas in `mcp/{tars_hub,tars_spoke}/`, and spoke worktrees in `worktrees/<hubConversationId>/subagent-*`.

3. **Unattended factory shifts** (see [../tars-run-factory/SKILL.md](@/skills/tars/tars-run-factory/_index.md)):
   - Inspect `<workspace_parent>/tars-factory/<repo-name>/` (or `--state-dir`) for `FACTORY_START_FAILURE.md`, `FACTORY_LEDGER.ndjson` (`seq`, `channel`, `conversation`, `payload`), and `FACTORY_REPORT.md`, plus workspace lock files under `~/.local/share/tars-factory/locks/`.

## 4. Snapshot evidence and execute autonomous recovery

Before running any recovery action, save a diagnostic bundle to a temporary scratch directory so bug evidence survives cleanup or resets:

```bash
snap_dir="$(mktemp -d "${TMPDIR:-/tmp}/tars-debug-XXXXXX")"
tars-agy doctor <workspace_root> [session_id] --recent-hours 48 >"$snap_dir/doctor.json" 2>&1 || true
tars-agy inspect <workspace_root> [session_id] >"$snap_dir/inspect.json" 2>&1 || true
tars-agy monitor <workspace_root> <session_id> --json >"$snap_dir/monitor.jsonl" 2>&1 || true
```

If spoke worktrees exist under `~/.gemini/antigravity-cli/worktrees/<hubConversationId>/`, also capture `git -C <spoke_worktree> status --porcelain` and `git -C <spoke_worktree> diff` into `"$snap_dir"` before reaping them.

Once evidence is preserved, recover stuck state autonomously using the narrowest sufficient action first:

1. **Open a verified parked bundle gate:**

   ```bash
   tars-agy approve <workspace_root> <issue_number>
   ```

2. **Garbage-collect leaked spoke worktrees:**

   ```bash
   tars-agy gc <workspace_root>
   ```

3. **Terminate wedged `agy` or `tars-agy` processes** when a hung host or orphaned MCP process blocks workspace ownership or `Door::Reset` live-owner checks (`SIGTERM` first, verify `/proc/<pid>` exit, then `SIGKILL` only if still alive).

4. **Reset run state** (all three scopes preserve `artifacts/`):
   - Single issue first (clears one issue's session, gates, approvals, and events):

     ```bash
     tars-agy reset <workspace_root> <issue_number>
     ```

   - Project-wide store reset when multiple sessions or store state in `<workspace_root>` must be cleared:

     ```bash
     tars-agy reset <workspace_root>
     ```

   - Global factory reset across every project in `<state_base>` when cross-project factory test state must be wiped:

     ```bash
     tars-agy reset --factory
     ```

Re-run `tars-agy doctor <workspace_root>` after recovery to confirm clean state.

## 5. Produce the root-cause report

Conclude every investigation with a concise, evidence-backed report:

1. **Symptom and scope:** workspace, `<session_id>` (or factory shift), GUARDS phase, and failure mode.
2. **Correlated timeline:** ordered events joining `tars-agy` (`audit.log` / `server.log` / `guards.sqlite3`) with `~/.gemini/` (`transcript.jsonl`, `tasks/`, `messages/`, `cli.log`) or `FACTORY_LEDGER.ndjson` via `<conversationId>` and `call_id`.
3. **Root cause and source boundary:** exact invariant, hook policy, MCP schema/transport mismatch, transcript window limit, or host condition responsible. When the `tars-agy` platform checkout (`crates/agy/` and `antigravity/`) is present locally, cite the responsible files and functions using [failure-modes.md](@/skills/tars/tars-debug/resources/manual/failure-modes.md); modify plugin source code only if the user asked for a fix.
4. **Recovery and verification:** path to saved diagnostic snapshot (clean up temporary scratch files if no longer needed), autonomous recovery commands executed, and final `tars-agy doctor` status.

