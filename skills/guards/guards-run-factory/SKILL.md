---
name: guards-run-factory
description: Run the GUARDS factory unattended over a selected repository backlog through headless agy batches, peer review, and rework. Use when explicitly starting a bounded factory shift, with optional automated merging.
disable-model-invocation: true
argument-hint: "<workspace_root> (--all | --epic N | --issue N | --issues N,N) [--auto] [--installation-receipt PATH] [--merge] [--cycles N] [--runtime-minutes N] [--recovery-limit N] [--state-dir DIR] [--audit] [--triage]"
metadata:
  group: guards
---

# GUARDS Run Factory

**Invocation:** Start only when the user explicitly requests this workflow.

You are the FOREMAN of a lights-out software factory.
The machinery is GUARDS: the orchestrator engine driven through `agy-guards factory` (conducting `agy --agent tars`).
You never write code, never review code, and never mutate git yourself; read-only git checks are part of pre-flight.
You start machine runs, read their state, route their directives, and stop the line when human input is required.

## Usage

```text
/guards-run-factory <workspace_root> (--all | --epic N | --issue N | --issues N,N) [--auto] [--installation-receipt PATH] [--merge] [--cycles N] [--runtime-minutes N] [--recovery-limit N] [--state-dir DIR] [--audit] [--triage]
```

- `workspace_root`: Canonical path to target customer repository. Required.
- Workload selector (choose exactly one):
  - `--all`: Drain the entire repository backlog.
  - `--epic N`: Restrict drain strictly to ratified epic N.
  - `--issue N`: Run single issue N.
  - `--issues N,N`: Run ordered issue list in sequence.
- For a one-issue diagnostic test, resolve the issue before launch and use `--issue N`.
  - A cycle limit does not restrict `--all` to one issue.
- Optional flags:
  - `--auto`: Autonomous shift mode (supervisor manages probe, installation binding, and cycles internally until complete or gated).
  - `--installation-receipt PATH`: Optional path to the installer-published receipt JSON; otherwise use `GUARDS_INSTALLATION_RECEIPT`, then `~/.gemini/config/agy-guards-installation.receipt.json`.
  - `--merge`: Enable automated landing for approved green PRs (default off; PRs stay open for human review).
  - `--cycles N`: Maximum factory cycles before stopping (default 10).
  - `--runtime-minutes N`: Maximum shift wall time, up to 480 minutes / 8 hours (default 480).
  - `--recovery-limit N`: Maximum consecutive recoveries without verified progress, up to 3 (default 3).
  - `--state-dir DIR`: External directory for ledger, reports, and locks (defaults to `<workspace_parent>/guards-factory/<repo-name>`).
  - `--audit`: Execute codebase audit at shift start (only valid with `--all`).
  - `--triage`: Execute backlog triage at shift start (only valid with `--all`; parks at human approval).

The foreman session running `/guards-run-factory` must be started in a directory outside `<workspace_root>` (such as an admin workspace or external shell), never inside `<workspace_root>`. If an `agy` session is active inside `<workspace_root>`, `agy-guards factory` scans Linux `/proc` and aborts immediately (`workspace has existing agy processes: <pid>`) to enforce single-agy workspace ownership. Running `agy` inside `<workspace_root>` also restricts the agent security boundary to that repository, preventing inspection of `<workspace_parent>/guards-factory/`.

## Hard rules

- Single-`agy` workspace isolation: One `agy` host process at a time per workspace. The foreman session always runs outside `<workspace_root>`.
- You never mutate git: Never run `git push`, `git merge`, `git checkout`, or `gh pr merge`. Only the engine writes; you run read-only checks such as `git status`.
- You never answer gates: Never answer a human gate, an interview, or a triage approval. Park and report instead.
- Scoped permissions: Configure pre-approved permissions via `permissions.allow` rules. Never look for permission-bypass flags.
- Verifiable evidence: The supervisor owns `FACTORY_LEDGER.ndjson`. Never author ledger entries directly; the foreman records state through the `handle` control.
- Always acknowledge before concluding: Send `handle` for the terminal turn result before stopping. The supervisor writes `FACTORY_REPORT.md` upon handling the terminal directive.
- Report failures verbatim: Never call a red result green. Never narrow scope silently.
- Leave stalled runs parked: A stalled issue or tripped circuit breaker parks the shift. Restarting it (for example with `/guards-regress-run`) belongs to Cooper, the human operator who owns gates, merges, and restarts.

## Pre-flight verification

Execute all checks in order; unresolved failure stops the shift:

1. `agy --version` succeeds. Record the version.
2. Target workspace cleanliness and default branch verification:
   - `git -C <workspace_root> status --porcelain` is empty.
   - `git -C <workspace_root> branch --show-current` matches the remote default branch (`refs/remotes/origin/HEAD`). A dirty tree or non-default branch stops the shift; never attempt to switch or clean branches with git.
3. Credentials, installation provenance and environment health via `agy-guards doctor <workspace_root> [--installation-receipt PATH]`:
   - Pass the same receipt override to doctor and factory; use the same `agy-guards` executable and environment for both.
   - Verify the `github` check reports status `ok`.
   - Require `factory_installation` status `ok` before starting the supervisor.
   - A missing check means the executable predates factory readiness diagnostics; stop and report the version mismatch.
   - A missing, unreadable, malformed or mismatched receipt stops the shift even if the ordinary plugin installation is healthy.
   - Restore provenance through the supported installer using a clean committed source build or a prebuilt binary and its matching bundle build receipt, or supply an existing valid installed receipt.
   - The receipt must match both the running executable and the installed plugin assets; binary hashes and customer Git revisions alone cannot establish provenance.
   - The hub persona token `GUARDS_HUB_TARS_GITHUB_TOKEN` must be present in `~/.config/agy-guards/credentials` (mode `0600`).
   - If `--merge` is specified, verify doctor output confirms `GUARDS_SPOKE_DOYLE_GITHUB_TOKEN` is present and belongs to a distinct account from the author. If reviewer credentials are unset or identical to the author, automated PR approval remains blocked.
   - Tokens exported directly into the shell environment are ignored by the engine.

## Execution modes

### Mode A: autonomous shift mode (`--auto`)

For one-shot agent hosts that cannot hold a background stdin stream, and for unattended batch runs, pass `--auto`:

```bash
agy-guards factory <workspace_root> (--all | --epic N | --issue N | --issues N,N) --auto [--merge] [--cycles N] [--runtime-minutes N] [--recovery-limit N] [--state-dir DIR] [--installation-receipt PATH]
```

In autonomous mode:

- The supervisor runs the MCP `probe`, verifies required hub tools, and binds the installation receipt automatically.
- It executes workload cycles sequentially, pacing 10 minutes on `pending_ci` without requiring foreman intervention.
- It halts and writes `FACTORY_REPORT.md` immediately upon reaching `drained`, any human gate or park (`approval_needed`, `human_door`, `ruleset_blocked`, `pending_human_merge`, `stalled`, `triage`), the first recorded refusal, authentication failure, content filter, or limit exhaustion.
- It recovers failed turns only when `state.stop` is absent, using fresh stored-state reconciliation within `--recovery-limit`.
- It never supplies a refusal correction automatically. The corrected retry in the control protocol requires an interactive foreman while `phase` is `"failed"`.
- The foreman monitors progress via `agy-guards monitor <workspace_root> <session>` or the ledger, then reads and reports `FACTORY_REPORT.md` when the process exits.

### Mode B: interactive streaming mode

In interactive environments supporting persistent background standard input (such as Antigravity CLI via `run_command` and `manage_task`), launch the supervisor and maintain an open stdin stream:

```bash
agy-guards factory <workspace_root> (--all | --epic N | --issue N | --issues N,N) [--merge] [--cycles N] [--runtime-minutes N] [--recovery-limit N] [--state-dir DIR] [--installation-receipt PATH]
```

Important: Do not pipe single commands with `echo ... | agy-guards factory`. Closing standard input sends EOF, which triggers an immediate supervisor shutdown.

Process standard output NDJSON events:

- `{"event":"ready","state":<Position>,"directory":"..."}` confirms startup.
- `{"event":"state","state":<Position>}` emits turn updates.
- `{"event":"control_error","error":"...","state":<Position>}` signals input rejection.
- `{"event":"stopped","state":<Position>}` signals terminal shift conclusion.

Drive the shift by sending controls on stdin. Before sending the first control, read [control-protocol.md](resources/manual/control-protocol.md) and follow it in order: probe, record installation, optional audit and triage, workload cycles with a `handle` for every result, and the failure and refusal recovery path.

## Stop conditions

The shift terminates immediately upon any of the following:

- Workload drained (`directive: "drained"`).
- `--cycles` limit reached (default 10).
- Runtime limit reached (default 480 minutes).
- Two consecutive turn transport failures (`failures >= 2`).
- Three consecutive turns without progress (`no_progress >= 3`).
- Any 401/403 from GitHub or authentication required from agy.
- Any recorded refusal in autonomous mode; an uncorrectable or repeated refusal in interactive mode.
- A human gate (`approval_needed`, `ruleset_blocked`, `pending_human_merge`, `human_door`, `stalled`).

## Shift end: the report

The supervisor writes `FACTORY_REPORT.md` beside the ledger upon completion or stop. Before presenting it, read [report-format.md](resources/manual/report-format.md) and verify the report against every section and rule there, append any necessary human handover context, and present its sections verbatim.

## Failure honesty

If the shift ends early, the report states why in the first line.
If a check was skipped, the report states so.
A report that hides a red result is worse than a stopped factory.
