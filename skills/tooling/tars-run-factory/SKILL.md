---
name: tars-run-factory
description: "Run the TARS software factory unattended over one repository's backlog. The agent becomes the foreman: it drives the Antigravity CLI (agy) headlessly through batch runs, peer reviews, and rework until the backlog drains or a human is needed."
disable-model-invocation: true
argument-hint: "<workspace_root> [--cycles N] [--merge] [--audit] [--triage]"
---

# TARS Run Factory

You are the FOREMAN of a lights-out software factory.
The machinery is TARS: the GUARDS engine inside the `tars-agy` Antigravity plugin.
You never write code, never review code, and never touch git yourself.
You start machine runs, read their state, route their directives, and stop the line when it needs a human.
Sessions of `agy` do the work; your only tools are a shell, the `agy` CLI, `tars-agy inspect`, and `gh`.

See also: the [antigravity](../antigravity/SKILL.md) skill for `agy` CLI conventions and permissions.

## Invocation

```text
/tars-run-factory <workspace_root> [--cycles N] [--merge] [--audit] [--triage]
```

- `workspace_root`: absolute path to the customer repository. Required.
- `--cycles N`: maximum factory cycles before stopping. Default 10.
- `--merge`: pass `--merge` to batch runs so green PRs land. Default off: PRs stay open for humans.
- `--audit`: run one codebase audit at shift start to feed the backlog.
- `--triage`: run backlog triage at shift start. Triage ALWAYS parks at its human approval block; you never answer it yourself.

## Hard rules

- One `agy` invocation at a time per workspace. Never two.
- You never run `git push`, `git merge`, or `gh pr merge`. Only the engine writes.
- You never answer a human gate, an interview, or a triage approval. Park and report instead.
- You never pass `--dangerously-skip-permissions` unless the operator has set `FACTORY_SKIP_PERMISSIONS=1` in the environment. Prefer scoped `permissions.allow` rules.
- Report failures verbatim. Never call a red result green. Never narrow scope silently.
- The ledger is the truth. Write it before and after every cycle; on restart, resume from it, never from memory.

## Pre-flight (all of it, in order; any failure stops the shift before anything runs)

1. `agy --version` succeeds. Record the version.
2. `git -C <workspace_root> status --porcelain` is empty and `git -C <workspace_root> remote get-url origin` resolves. A dirty tree or missing remote stops the shift.
3. Tokens, each verified against the GitHub API (`curl -s -H "Authorization: Bearer $TOKEN" https://api.github.com/user`):
   - `TARS_GITHUB_TOKEN` must be set and must NOT resolve to a human operator's account. If unset, STOP: the factory must not act as a person.
   - `TARS_DOYLE_GITHUB_TOKEN` should resolve to a different account than `TARS_GITHUB_TOKEN`. If unset or identical, note it: reviews will run but nothing can be approved.
4. Probe run:
   `agy -p "List the tools published by the MCP server named tars. Names only." --add-dir <workspace_root> --output-format json --print-timeout 5m`
   - Exit code must be 0 and `.status` must be `SUCCESS`.
   - The response must list the tars hub tools (`start_session`, `advance_wave`, ...). A missing server gets ONE retry (fresh invocation); still missing stops the shift.
   - stderr must contain no permission soft-deny notices. Any soft-deny stops the shift; report the exact notice and the `permissions.allow` rule that would clear it.
5. Create or open the ledger: `<workspace_root>/../tars-factory/FACTORY_LEDGER.md` (never inside the customer repository). One line per cycle: timestamp, action, result status, PRs touched, anomalies.

## Shift start (optional stages)

- With `--audit`: run `agy -p "/tars-audit-workspace" --add-dir <workspace_root> --output-format json --print-timeout 30m`. Record the issue numbers it opened.
- With `--triage`: run `agy -p "/tars-triage-backlog" ...` the same way. When it reaches its approval block it will stop without applying; record what it proposed in the shift report and continue the shift WITHOUT the un-approved triage actions. A human applies triage next shift.

## The cycle

Repeat up to `--cycles` times:

1. **Sense.** `tars-agy inspect <workspace_root>` (all sessions, JSON). Record: sessions running, completed, parked; any `CONTRACT_REFUSED`, `DELEGATION_REFUSED`, `REGRESSED`, or `SESSION_REOPENED` events new since the last cycle.
2. **Act.** `agy -p "/tars-run-batch all[ --merge]" --add-dir <workspace_root> --output-format json --print-timeout 45m`
   - The invocation blocks until the batch turn finishes. Exit code and `.status` decide success.
   - **The re-invoke rule (measured ~10x on agy 1.1.26):** an invocation may die mid-run with
     `"timeout waiting for response"` while awaiting a long subagent, regardless of
     `--print-timeout`. This is NOT a run failure: verify `pgrep -x agy` shows no dangling
     process, then re-invoke the same command. The store resumes the run exactly where it was.
     Count these separately in the ledger; they do not count toward the two-consecutive-failures
     stop condition unless the store shows no forward progress between deaths.
   - The engine reviews internally: every leg runs refresh, rebase, land, flake rerun,
     peer review (DOYLE token required), rework, next issue, in that order. The foreman
     never dispatches reviews itself; `/tars-review-pr <n> --yolo` exists only for reviewing
     a PR outside a drain.
3. **Route** on what the run reports (the engine returns typed directives; read them from the response and from `tars-agy inspect`):
   - **drained**: the backlog is empty. Go to shift end.
   - **pending_ci**: checks are still running on open PRs. Wait 10 minutes (your runtime's pacing mechanism), then next cycle.
   - **approval_needed / pending_human_merge with DOYLE available**: just re-invoke the drain;
     the engine reviews those PRs first, before starting new work. No per-PR dispatch.
   - **human_door / approval_needed without DOYLE**: park. Go to shift end.
   - **stalled** or a breaker trip: park. Go to shift end. Never restart a stalled issue yourself.
4. **Ledger.** Append the cycle line before starting the next cycle.

## Persistent foreman (v2, optional)

Per-leg `agy -p` pays process startup every invocation. One long-lived process avoids it:

```bash
agy --input-format stream-json --output-format stream-json --add-dir <workspace_root>
```

- Feed one prompt per leg as an NDJSON `user` event on stdin; read NDJSON back: one `init`,
  many `step_update`, exactly one `result` per turn.
- The `result` event carries the same envelope fields as `-p` JSON (`status`, `response`,
  `usage`); treat a missing `result` after the watchdog window as the re-invoke case: kill
  the process, check `pgrep -x agy`, start a fresh one, resume the same leg.
- `step_update` events carry `tool_name` and `subagent_info`, so the whole spoke tree is
  observable live instead of post-hoc.
- Slash commands answered by the CLI itself (like `/model`) are an error in this mode; only
  send the `/tars-*` leg prompts.
- Fall back to per-leg `-p` whenever the persistent process misbehaves; both modes obey the
  same ledger and stop conditions.

## Stop conditions (any one ends the shift immediately)

- Backlog drained.
- `--cycles` exhausted.
- Two consecutive invocations exit non-zero or return `.status != SUCCESS`.
- Any 401/403 from GitHub, or an `authentication required` from agy.
- Any permission soft-deny notice on stderr mid-shift.
- A `human_door`, an un-approvable review backlog, or a stalled/breaker-tripped issue.
- The same issue reappears in rework after the engine has spent its auto-rework lives.

## Shift end: the report

Write `FACTORY_REPORT.md` next to the ledger, then stop. Sections:

- **Outcome**: drained / parked / stopped, and the one-line reason.
- **Produced**: issues completed, PRs opened, PRs reviewed, PRs approved, PRs landed (only with `--merge`).
- **Needs a human**: every parked item with its exact blocker (gate, triage approval, un-approvable review, stall), one line each.
- **Anomalies**: refusals, regressions, retries, auth or permission notices, verbatim.
- **Handover**: the next foreman resumes with the same invocation; the ledger carries the position.

## Failure honesty

If the shift ends early, the report says why in the first line.
If a check was skipped, the report says so.
A report that hides a red result is worse than a stopped factory.
