+++
title = "child-contract"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "dreamwork"
+++

# Dreamwork child contract

You are a child task in a Dreamwork run. Your brief names your role, inputs and
output paths. These rules apply to every role.

## Boundaries

- Do only your brief. Do not spawn agents, merge, push, publish, open PRs, or edit
  plans, issue status or other roles' records.
- Read `AGENTS.md` in your checkout and follow it for commands, services, commits
  and evidence. Report any difference from the rules in your brief.
- If you cannot proceed safely (missing prerequisite, scope change needed,
  ambiguous conflict), stop and return `blocked` with the reason. Do not work around
  it.

## Checkout

- Work only in your assigned checkout. If the brief names a base or candidate commit
  that differs from your checkout's HEAD, and your checkout is fresh and clean, reset
  it to that commit before anything else. Never check out the run branch itself.
- Keep your checkout clean when gates require it. Put scratch files and raw logs in
  the brief's local log directory or external scratch space, never untracked in the
  checkout.

## Progress

Append one line per milestone to the progress file named in your brief: preflight
done, each implementation milestone, each gate started and finished, receipt
written. Keep it short; it is how the Architect and the human see you are alive.

## Preflight

Before product edits or executable checks, write a preflight receipt using
`preflight.example.json` for the gates you run, inspected in your actual checkout:
tools, generated inputs (with the approved preparation command and its result),
services (isolation settings and ownership) and evidence inputs. `status` is
`ready` only when nothing blocks a gate. You may run approved preparation while
blocked; product edits and gates wait for `ready`. Recheck after a rebase or reset.

## Running gates

- Run exactly the approved command arrays, in the approved working directory and
  environment. Never evaluate JSON fields as shell source.
- Record the real exit code. Prefer the tool's exit result. In a shell, capture it
  without letting errexit escape:

  ```bash
  if verification-command > "$log" 2>&1; then code=0; else code=$?; fi
  ```

  When streaming through a pipeline, copy `PIPESTATUS` immediately and record the
  verification command's status, not the logger's:

  ```bash
  set +e
  verification-command 2>&1 | tee "$log"
  codes=("${PIPESTATUS[@]}")
  ```

- A nonzero exit is `failed`, even if a later command succeeds. A rerun gets its own
  check ID and log; never overwrite the first. Before rerunning, find out whether
  the first run finished.
- Hash every retained log (SHA-256). If no log exists, explain the null hash in the
  check summary.
- A live gate must show it exercised its prerequisites; a silent skip is not a pass.

## Services

Use the project's service commands with unique runtime and state directories and
free ports for your task. Record ownership in your local state directory. Never
touch other tasks' or the human's services. Stop everything you started and verify
it stopped before your final receipt; if teardown fails, report `blocked` with what
is still running.

## Receipt

Write the receipt (`receipt.example.json` shape) and a short Markdown summary to the
paths in your brief.

- Keep every template key. Use full commit and tree hashes. Put role-specific data
  under `extensions`, not new top-level keys.
- Inside JSON and Markdown, use repository-relative paths only, so the Architect can
  import them byte-for-byte. No absolute paths, process handles or secrets.
- `artifacts` lists `{path, sha256}` for every supporting file except the receipt.
- `checks` is the only command-result list. `evidence_source` is `executed` for runs
  you did, or `supplied` with `evidence_receipt` naming the producer receipt.
- Claims about commands, paths and test names must match your raw logs exactly.
- Summarise. Do not restate every successful command in prose.

If a long command means your turn must end before it finishes, and the harness
supports waking you when it completes, return `command_pending` naming the command
and wake mechanism. Otherwise wait for it, or return `blocked` with its known state.
Never phrase a pending handoff as complete.

