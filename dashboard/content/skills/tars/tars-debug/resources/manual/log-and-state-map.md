+++
title = "log-and-state-map"
[extra]
skill = false
category = "tars"
mermaid = false
skill_name = "tars-debug"
+++

# TARS and Antigravity Log and State Map

Use this reference to locate runtime state directories, decode JSONL log records, and join `tars-agy` events to Antigravity (`~/.gemini/`) conversation artifacts.

## 1. Base path resolution

Resolve paths dynamically from environment variables and the target `<workspace_root>` rather than assuming fixed user directories:

- **TARS state base (`<state_base>`):**
  - `$XDG_STATE_HOME/tars` when `$XDG_STATE_HOME` is set to an absolute path
  - `~/.local/state/tars` on Linux when `$XDG_STATE_HOME` is unset
  - `~/Library/Application Support/tars` on macOS when `$XDG_STATE_HOME` is unset
- **Project slug (`<project_slug>`):** `<owner>__<repo>` derived from `git -C <workspace_root> remote get-url origin`.
- **TARS credentials file (never print contents):** `$TARS_CREDENTIALS_FILE`, else `$XDG_CONFIG_HOME/tars/credentials`, else `~/.config/tars/credentials` (expected mode `0600`). Note that `tars-agy` reads persona tokens from this file and ignores tokens exported directly in the shell environment.
- **Factory state directory (`<factory_dir>`):** `<workspace_parent>/tars-factory/<repo-name>` by default, or the custom path passed to `tars-agy factory --state-dir <dir>`.
- **Factory workspace locks:** `$XDG_DATA_HOME/tars-factory/locks/<sha256_hex>` (defaults to `~/.local/share/tars-factory/locks/<sha256_hex>` on Linux), where `<sha256_hex>` is the SHA-256 digest of the canonical `<workspace_root>` path.
- **Antigravity host directories (`<agy_app_dir>`):**
  - Headless and CLI sessions: `~/.gemini/antigravity-cli`
  - Desktop and IDE sessions: `~/.gemini/antigravity` (and `~/.gemini/antigravity-ide` when present)
  - ACP provider sessions: `~/.gemini/antigravity-acp`
  - Shared plugin and hook config: `~/.gemini/config`

## 2. TARS state tree (`<state_base>/`)

```text
<state_base>/
├── hook-callers/
│   └── <conversationId>              # Marks an activated TARS caller enforced by pre-tool-use
├── logs/
│   ├── server.log                    # Global events without a resolved project (e.g. reset --factory)
│   ├── error.log                     # Global WARN/ERROR subset
│   └── audit.log                     # Global MCP audit calls without a project
└── <owner>__<repo>/
    ├── guards.sqlite3                # Authoritative GUARDS SQLite store (read via inspect/doctor)
    ├── artifacts/
    │   └── <id>/                     # Deliverables (G.2..D.2), .writers sidecar, .run.json manifests
    ├── investigations/               # Recorded runtime investigation bundles
    ├── publication-locks/            # GitHub publication coordination locks
    ├── evidence-execution-<N>.lock   # Evidence-work gate locks
    └── logs/
        ├── server.log                # Project-wide merged tracing + mirrored audit JSONL
        ├── error.log                 # Project-wide WARN/ERROR JSONL
        ├── audit.log                 # Project-wide MCP tool boundary JSONL
        ├── <session_id>/             # Per-session logs (e.g. 42, review-pr-13, wave-*, run-chores-*)
        │   ├── .seq                  # Lockfile issuing monotonic u64 seq across processes
        │   ├── server.log            # Session tracing + hook decisions + mirrored audit lines
        │   ├── error.log             # Session WARN/ERROR lines and tars_agy::panic records
        │   └── audit.log             # Session hub/spoke MCP call lifecycle lines
        ├── crew/                     # Subagent/crew dispatch diagnostics
        ├── spoke-continuations/      # Hub-to-spoke continuation state and reminders
        ├── workflow-invocations/     # Skill/workflow invocation traces
        ├── attended-resumes/         # Interactive resume records
        ├── feedback-publications/    # PR review/comment publication traces
        ├── publication-journal/      # Durable GitHub write journal
        └── cleanup/                  # Spoke worktree lease and gc reaper notes
```

## 3. TARS JSONL schemas and `TARS_LOG` verbosity

Every line in `server.log`, `error.log`, and `audit.log` is a single JSON object.

### Common fields (`server.log` and `error.log`)

```json
{
  "seq": 148,
  "ts": "2026-10-30T11:42:09.512Z",
  "unix_ms": 1793360529512,
  "pid": 41892,
  "process": "hub",
  "level": "WARN",
  "target": "tars_agy",
  "session": "42",
  "conversation": "8f2c1b40-9e3a-4d1c-8a77-112233445566",
  "message": "hook denied command or control-surface access",
  "reason": "..."
}
```

- `process`: identifies which binary entrypoint wrote the record (`hub`, `spoke`, `hook`, `gc`, `approve`, `reset`). Read-only CLI subcommands (`doctor`, `inspect`, `monitor`, `tools`) leave no log trail.
- Panics log at `ERROR` under `target: "tars_agy::panic"` in both `server.log` and `error.log`.
- **Fail-open `FileLayer` caveat:** If a log write fails once (disk full, permission error, or directory unlinked mid-run), `tars-agy` emits a single warning to `stderr` and disables file logging for the rest of that process lifetime. When `audit.log` stops advancing while the process is still alive, inspect host `cli.log` or custom `--log-file` output for `stderr`.

### MCP audit records (`audit.log`)

`audit.log` captures every `tars_hub` and `tars_spoke` MCP tool invocation across two (or three) phases sharing one `call_id`:

- `phase: "start"` - records `tool`, `process` (`hub` or `spoke`), `session`, and `conversation` (attributed via `note_caller`).
- `phase: "args"` - emitted only when `TARS_LOG=debug` (or a matching `EnvFilter` directive) is active; contains redacted tool arguments.
- `phase: "end"` - records `ok` (`true` or `false`), `duration_ms`, and error summary when `ok` is `false`.

`tars-agy` automatically redacts keys matching `token`, `secret`, `password`, `authorization`, and `api_key`, plus GitHub token prefixes (`ghp_`, `gho_`, `ghu_`, `ghs_`, `ghr_`, `github_pat_`).

### Enabling debug argument logging for a reproduction run

```bash
TARS_LOG=debug agy
```

`TARS_LOG` accepts standard `tracing_subscriber::EnvFilter` directives (default `info`).

## 4. Antigravity (`~/.gemini/`) host tree and transcript schemas

```text
~/.gemini/
├── config/
│   ├── import_manifest.json              # Must list exactly one tars plugin import
│   ├── hooks.json                        # Host hook file with projected tars-agy hook entries
│   ├── tars-hooks.receipt.json           # Projection receipt verified by --projected hooks
│   ├── tars-installation.receipt.json    # Binary + asset provenance receipt for factory runs
│   └── plugins/tars/
│       ├── plugin.json                   # Plugin manifest
│       ├── hooks.json                    # Intentionally empty (hooks run from ~/.gemini/config/hooks.json)
│       ├── mcp_config.json               # Defines hub MCP server (timeoutSeconds: 3900)
│       └── {agents,rules,skills}/        # Persona definitions (case, doyle, romilly, kipp, brand) and skills
└── antigravity-cli/                      # Check ~/.gemini/antigravity/ for Desktop/IDE runs
    ├── cli.log                           # Symlink to log/cli-YYYYMMDD_HHMMSS.log (host + MCP stderr)
    ├── crashes/crash_*.log               # Host crash dumps
    ├── cache/
    │   ├── last_conversations.json       # Maps workspace path -> most recent conversationId
    │   └── conversation_metadata.json    # Conversation titles, workspaces, timestamps
    ├── mcp/
    │   ├── tars_hub/*.json               # Cached hub MCP tool schemas
    │   └── tars_spoke/*.json             # Cached spoke MCP tool schemas
    ├── worktrees/
    │   └── <hubConversationId>/
    │       └── subagent-*                # Branched Git worktrees for write-scoped spokes
    └── brain/
        └── <conversationId>/
            └── .system_generated/
                ├── logs/
                │   ├── transcript.jsonl       # Compact step-indexed conversation transcript
                │   └── transcript_full.jsonl  # Full step-indexed transcript fallback
                ├── steps/                     # Per-step tool inputs and outputs
                ├── tasks/                     # Background command logs (task-*.log)
                └── messages/                  # Inter-agent send_message queue records
```

### Transcript tail behavior (`transcript.jsonl` and `transcript_full.jsonl`)

- `tars-agy` searches both `~/.gemini/antigravity-cli/brain/<conversationId>` and `~/.gemini/antigravity/brain/<conversationId>`.
- Step entries include `USER_INPUT`, `PLANNER_RESPONSE`, `TOOL_CALL`, and `GENERIC` events (such as `ask_question` completions).
- When reconciling human approvals (`approved` / `approved <issue_number>`) or G.1 / bundle `ask_question` answers (`decision <id>: ...`), `tars-agy` reads only the **last 128 KiB** of `transcript.jsonl` (falling back to `transcript_full.jsonl`). Oversized steps after the user's answer can push the answer past the 128 KiB tail window.

## 5. Factory supervisor artifacts (`<factory_dir>/`)

Unattended `/tars-run-factory` shifts (`tars-agy factory`) maintain supervisor state outside `<workspace_root>`:

- `FACTORY_LEDGER.ndjson` - append-only redacted NDJSON stream of `Receipt` objects (`seq`, `received`, `channel` such as `configuration`, `checkpoint`, `turn_result`, optional `position`, optional `conversation`, and `payload`).
- `FACTORY_REPORT.md` - terminal shift summary written when the supervisor handles the final directive or stops in `--auto` mode.
- `FACTORY_START_FAILURE.md` - written when preflight, provenance binding, or workspace lease acquisition fails at shift start.

## 6. Cross-system join keys

| Key                | Where to find it                                                                                                                                     | What it unlocks                                                                                                                           |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `<session_id>`     | Issue number (`42`), `review-pr-<N>`, `wave-*`, `run-chores-*`, `tars-agy inspect <root>`                                                            | `tars-agy doctor <root> <id>`, `tars-agy monitor <root> <id>`, `<state_base>/<project_slug>/logs/<id>/`, `artifacts/<id>/`                |
| `<conversationId>` | `audit.log` (`conversation`), store `PROD_SENT` (`identity.conversation`), `FACTORY_LEDGER.ndjson` (`conversation`), `cache/last_conversations.json` | `<agy_app_dir>/brain/<conversationId>/.system_generated/`, `<state_base>/hook-callers/<conversationId>`, `worktrees/<hubConversationId>/` |
| `call_id` + `seq`  | `audit.log`, `server.log`, `doctor` `open_calls`                                                                                                     | Pairs `start` / `args` / `end` records, measures `duration_ms`, and isolates hung in-flight MCP calls                                     |
| Spoke `worktree`   | `PostToolUse` `invoke_subagent` notes, `PROD_SENT` (`identity.worktree`), `worktrees/<hubConversationId>/subagent-*`                                 | Uncommitted spoke edits (`git status`, `git diff`, `git log -n 5`) before running `tars-agy gc`                                           |

