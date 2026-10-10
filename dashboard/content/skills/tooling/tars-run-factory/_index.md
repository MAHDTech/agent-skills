+++
title = "tars-run-factory"
description = "Run the TARS software factory unattended over one repository's backlog. The agent becomes the foreman: it drives the Antigravity CLI (agy) headlessly through batch runs, peer reviews, and rework until the backlog drains or a human is needed."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# TARS Run Factory

You are the FOREMAN of a lights-out software factory.
The machinery is TARS: the GUARDS engine driven through `tars-agy factory`.
You never write code, never review code, and never touch git yourself.
You start machine runs, read their state, route their directives, and stop the line when human input is required.

## Invocation

```text
/tars-run-factory <workspace_root> (--all | --epic N | --issue N | --issues N,N) [--merge] [--cycles N] [--runtime-minutes N] [--audit] [--triage]
```

- `workspace_root`: Canonical path to target customer repository. Required.
- Workload selector (choose exactly one):
  - `--all`: Drain the entire repository backlog.
  - `--epic N`: Restrict drain strictly to ratified epic N.
  - `--issue N`: Run single issue N.
  - `--issues N,N`: Run ordered issue list in sequence.
- Optional flags:
  - `--merge`: Enable automated landing for approved green PRs (default off; PRs stay open for human review).
  - `--cycles N`: Maximum factory cycles before stopping (default 10).
  - `--runtime-minutes N`: Maximum shift wall time, up to 480 minutes / 8 hours (default 480).
  - `--recovery-limit N`: Maximum consecutive recoveries without verified progress, up to 3 (default 3).
  - `--state-dir DIR`: External directory for ledger, reports, and locks (defaults to `<workspace_parent>/tars-factory/<repo-name>`).
  - `--audit`: Execute codebase audit at shift start (only valid with `--all`).
  - `--triage`: Execute backlog triage at shift start (only valid with `--all`; parks at human approval).
  - `--auto`: Autonomous shift mode (supervisor manages probe, installation, and cycles internally until complete or gated).

The foreman session running `/tars-run-factory` must be started in a directory outside `<workspace_root>` (such as the platform workspace or an admin shell), never inside `<workspace_root>`. If an `agy` session is active inside `<workspace_root>`, `tars-agy factory` scans Linux `/proc` and aborts immediately (`workspace has existing agy processes: <pid>`) to enforce single-agy workspace ownership. Furthermore, running `agy` inside `<workspace_root>` restricts the agent security boundary to that repository, preventing inspection of `<workspace_parent>/tars-factory/`.

## Hard Rules

- Single-`agy` workspace isolation: One `agy` host process at a time per workspace. The foreman session always runs outside `<workspace_root>`.
- You never touch git: Never run `git push`, `git merge`, or `gh pr merge`. Only the engine writes.
- You never answer gates: Never answer a human gate, an interview, or a triage approval. Park and report instead.
- Scoped permissions: Configure pre-approved permissions via `permissions.allow` rules. Never look for permission-bypass flags.
- Verifiable evidence: The supervisor owns `FACTORY_LEDGER.ndjson`. Never author ledger entries directly; the foreman records state through the `handle` control.
- Always acknowledge before concluding: Send `handle` for the terminal turn result before stopping. The supervisor writes `FACTORY_REPORT.md` upon handling the terminal directive.
- Report failures verbatim: Never call a red result green. Never narrow scope silently.
- Do not restart stalled runs: A stalled issue or tripped circuit breaker parks the shift; `/tars-regress-run` belongs to Cooper.

## Pre-Flight Verification

Execute all checks in order; unresolved failure stops the shift:

1. `agy --version` succeeds. Record the version.
2. Target workspace cleanliness and default branch verification:
   - `git -C <workspace_root> status --porcelain` is empty.
   - `git -C <workspace_root> branch --show-current` matches the remote default branch (`refs/remotes/origin/HEAD`). A dirty tree or non-default branch stops the shift; never attempt to switch or clean branches with git.
3. Credentials and environment health via `tars-agy doctor <workspace_root>`:
   - Verify the `github` check reports status `ok`.
   - The hub persona token `TARS_HUB_TARS_GITHUB_TOKEN` must be present in `~/.config/tars/credentials` (mode `0600`).
   - If `--merge` is specified, verify doctor output confirms `TARS_SPOKE_DOYLE_GITHUB_TOKEN` is present and belongs to a distinct account from the author. If reviewer credentials are unset or identical to the author, automated PR approval remains blocked.
   - Tokens exported directly into the shell environment are ignored by the engine.

## Execution Modes

### Mode A: Autonomous Shift Mode (`--auto`)

For one-shot execution environments (Claude Code, OpenAI Codex, Grok-build) and unattended batch runs, pass `--auto`:

```bash
tars-agy factory <workspace_root> (--all | --epic N | --issue N | --issues N,N) --auto [--merge] [--cycles N] [--runtime-minutes N]
```

In autonomous mode:

- The supervisor runs the MCP `probe`, verifies required hub tools, and binds the installation receipt automatically.
- It executes workload cycles sequentially, pacing 10 minutes on `pending_ci` without requiring foreman intervention.
- It halts and writes `FACTORY_REPORT.md` immediately upon reaching `drained`, any human gate (`approval_needed`, `human_door`, `ruleset_blocked`, `triage`), repeated refusals, or limit exhaustion.
- The foreman monitors progress via `tars-agy monitor <workspace_root> <session>` or the ledger, then reads and reports `FACTORY_REPORT.md` when the process exits.

### Mode B: Interactive Streaming Mode

In interactive environments supporting persistent background standard input (such as Antigravity CLI via `run_command` and `manage_task`), launch the supervisor and maintain an open stdin stream:

```bash
tars-agy factory <workspace_root> (--all | --epic N | --issue N | --issues N,N) [--merge] [--cycles N] [--runtime-minutes N] [--state-dir DIR]
```

Important: Do not pipe single commands with `echo ... | tars-agy factory`. Closing standard input sends EOF, which triggers an immediate supervisor shutdown.

Process standard output NDJSON events:

- `{"event":"ready","state":<Position>,"directory":"..."}` confirms startup.
- `{"event":"state","state":<Position>}` emits turn updates.
- `{"event":"control_error","error":"...","state":<Position>}` signals input rejection.
- `{"event":"stopped","state":<Position>}` signals terminal shift conclusion.

## Control Stream Protocol

When operating in interactive mode, send controls as single JSON lines on stdin:

### 1. Probe MCP Surface

Send:

```json
{"action": "probe"}
```

Wait for emitted state with `phase: "result"` and `transport: "SUCCESS"`.
Verify the response lists the hub tools: `start_session` and `advance_wave`. (If missing, send one `{"action":"retry-probe","resultSeq":<seq>}`; still missing stops the shift).

Acknowledge the probe:

```json
{"action":"handle","resultSeq":<seq>,"directive":"continue","progress":"hub-tools-verified","evidence":{"tools":["start_session","advance_wave"]}}
```

### 2. Record Installation Binding

Mandatory before executing any workload action (`audit`, `triage`, or `cycle`). Must be sent while `phase` is `"ready"` and `cycles == 0`.

Compute the binary hash: `sha256sum "$(command -v tars-agy)"`.
Supply the reviewed Git revisions from the operator install receipt:

```json
{
  "action": "record-installation",
  "installation": {
    "sourceRevision": "<40-or-64-hex>",
    "integratedRevision": "<40-or-64-hex>",
    "installedRevision": "<40-or-64-hex>",
    "binarySha256": "<sha256>"
  }
}
```

### 3. Shift Start (Optional Stages with `--all`)

- With `--audit`: send `{"action":"audit"}`. Acknowledge result with `handle`.
- With `--triage`: send `{"action":"triage"}`. Triage always parks at its human approval gate. Acknowledge with `handle` using `directive: "parked"`, and record the proposal without applying it.

Note: Exact selections (`--epic`, `--issue`, `--issues`) exclude repository-wide audit and triage.

### 4. The Workload Cycle

Repeat up to `--cycles` times:

1. **Act.** Send the cycle control:

   ```json
   {"action": "cycle"}
   ```

2. **Wait.** Await event with `phase: "result"` or `phase: "failed"`.
3. **Route.** Evaluate the turn response and determine the workflow directive:
   - `continue`: Leg completed normally; advance to next cycle.
   - `pending_ci`: Checks running on open PRs. The supervisor internally paces for 10 minutes before accepting the next cycle.
   - `drained`: Selected workload finished.
   - `approval_needed` / `ruleset_blocked` / `pending_human_merge` / `human_door` / `stalled`: Park shift.
   - Batch engine `operational_blocker`: Map to `parked`, copying the `why` string into evidence.
4. **Ledger.** Send `handle` naming the current `resultSeq`, directive, scoped progress fingerprint, and non-empty evidence object:

   ```json
   {
     "action": "handle",
     "resultSeq": 1,
     "directive": "continue",
     "progress": "fingerprint",
     "evidence": {"details": "..."},
     "produced": {}
   }
   ```

Critical rule: Always send `handle` for terminal directives (`drained`, `parked`, etc.). The supervisor handles terminal receipts by transitioning to `stopped`, writing `FACTORY_REPORT.md`, and closing cleanly.

### 5. Failure and Refusal Recovery

If `phase` is `"failed"`:

1. Reconcile stored state using `tars-agy inspect <workspace_root>`.
2. Verify the failed host process has exited.
3. If `state.stop` indicates an incidental tool refusal and retry budget remains (exactly one corrected retry is permitted), send the `recover` control with reconciliation and structured correction:

   ```json
   {
     "action": "recover",
     "reconciliation": {
       "disposition": "resume",
       "progress": "<progress-fingerprint>",
       "evidence": {"issue": 42, "phase": "..."}
     },
     "correction": {
       "refusal": "<exact stop.reason from state>",
       "tool": "<exact stop.tool from state>",
       "target": "<exact stop.target from state>",
       "permittedBrief": "<alternative instruction omitting denied operation>",
       "permissionEvidence": "<justification under existing permissions>"
     }
   }
   ```

4. If the refusal repeats or cannot be corrected within existing policy, stop the shift:

   ```json
   {"action": "stop", "reason": "<concrete blocker description>"}
   ```

## Stop Conditions

The shift terminates immediately upon any of the following:

- Workload drained (`directive: "drained"`).
- `--cycles` limit reached (default 10).
- Runtime limit reached (default 480 minutes).
- Two consecutive turn transport failures (`failures >= 2`).
- Three consecutive turns without progress (`no_progress >= 3`).
- Any 401/403 from GitHub or authentication required from agy.
- An unrecoverable tool refusal or exhausted refusal retry budget.
- A human gate (`approval_needed`, `ruleset_blocked`, `pending_human_merge`, `human_door`, `stalled`).

## Shift End: The Report

The supervisor writes `FACTORY_REPORT.md` beside the ledger upon completion or stop.
Verify its contents, append any necessary human handover context, and present sections verbatim:

- **Outcome**: Outcome status and concrete reason.
- **Produced**: Verified deliverable inventory (completed issues, PRs opened, PRs reviewed, PRs landed).
- **Needs Cooper**: Parked items with their exact blockers.
- **Anomalies**: Recorded failure, refusal, and recovery events.
- **Handover**: Resumption invocation and durable position.

## Failure Honesty

If the shift ends early, the report states why in the first line.
If a check was skipped, the report states so.
A report that hides a red result is worse than a stopped factory.

