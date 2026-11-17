+++
title = "report-format"
[extra]
skill = false
category = "guards"
mermaid = false
skill_name = "guards-run-factory"
+++

# Factory report format

`FACTORY_REPORT.md` carries these sections, in order:

- **Outcome**: Outcome status and concrete reason.
- **Produced**: Verified deliverable inventory (completed issues, PRs opened, PRs reviewed, PRs landed).
- **Needs Cooper**: Parked items with their exact blockers, for Cooper, the human operator who owns gates, merges, and restarts.
- **Anomalies**: Recorded failure, refusal, and recovery events.
- **Timing**: Observed phase, spoke, tool, wait, and recovery intervals with their measurement limits.
- **Handover**: Outcome-specific next steps and links to durable position and invocation evidence.

The report derives its inventory from the selected shift's recorded evidence, excluding probes and historical deliveries.
Successful, skipped, neutral, pending, failed, and unknown CI outcomes remain separate.
Timing diagnostics are collected on every run; inclusive intervals can overlap and must not be added as elapsed wall time.
Missing or interrupted measurements remain unavailable or incomplete, and a recovered failure remains an anomaly after successful completion.
A drained selection without blockers requires no human action.

