+++
title = "report-format"
[extra]
skill = false
category = "guards"
mermaid = false
skill_name = "guards-run-factory"
+++

# Factory report format

## Single-repository report (`FACTORY_REPORT.md`)

`FACTORY_REPORT.md` carries these sections, in order:

- **Outcome**: Outcome status and concrete reason.
- **Produced**: Verified deliverable inventory (completed issues, PRs opened, PRs reviewed, PRs landed, and in `--chores` shifts, verified chore PRs merged, queued, or bootstrap PRs opened).
- **Needs Cooper**: Parked items with their exact blockers, for Cooper, the human operator who owns gates, merges, and restarts.
- **Anomalies**: Recorded failure, refusal, and recovery events.
- **Timing**: Observed phase, spoke, tool, wait, and recovery intervals with their measurement limits.
- **Handover**: Outcome-specific next steps and links to durable position and invocation evidence.

The report derives its inventory from the selected shift's recorded evidence, excluding probes and historical deliveries.
Successful, skipped, neutral, pending, failed, and unknown CI outcomes remain separate.
Timing diagnostics are collected on every run; inclusive intervals can overlap and must not be added as elapsed wall time.
Missing or interrupted measurements remain unavailable or incomplete, and a recovered failure remains an anomaly after successful completion.
A drained selection without blockers requires no human action.

## Multi-repository fleet report (`FLEET_REPORT.md`)

In `--repos-dir` fleet mode, `<state_dir>/FLEET_REPORT.md` summarizes the entire fleet alongside `FLEET_LEDGER.ndjson`:

- **Outcome**: Fleet completion status, discovered repository count, concurrency setting, and elapsed duration.
- **Summary Table**: One row per discovered repository with pre-check classification, terminal status (`drained`, `bootstrap`, `skipped`, `parked`, `blocked`, `failed`), merged/queued/bootstrap/blocked chore tallies, and relative link to `<repo-name>/FACTORY_REPORT.md` when a shift ran.
- **Needs Cooper**: Every repository requiring human action (bootstrap PRs awaiting review, ruleset blocks, dirty/non-default checkouts, or failed shifts) with its concrete reason.
- **Skipped**: Repositories skipped at zero LLM cost (`no_chores`, `bootstrap_pending`, `archived`, `unsupported_host`, `active_agy`) with their recorded reason.

