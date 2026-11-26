+++
title = "control-protocol"
[extra]
skill = false
category = "guards"
mermaid = false
skill_name = "guards-run-factory"
+++

# Control stream protocol (interactive mode)

When operating in interactive mode, send controls as single JSON lines on stdin:

## 1. Probe MCP surface

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

## 2. Record installation binding

Mandatory before executing any workload action (`audit`, `triage`, or `cycle`). Must be sent while `phase` is `"ready"` and `cycles == 0`.

Resolve the same installed receipt verified by doctor, using `--installation-receipt`, `GUARDS_INSTALLATION_RECEIPT`, then `~/.gemini/config/agy-guards-installation.receipt.json`.
Send the complete receipt unchanged as the value of `installation`, including the bundle provenance fields.
Keep `integratedRevision` as `null` when integration is unknown.

```json
{
  "action": "record-installation",
  "installation": {
    "sourceRevision": "<40-or-64-hex>",
    "integratedRevision": null,
    "installedRevision": "<40-or-64-hex>",
    "binarySha256": "<sha256>",
    "bundleSha256": "<reviewed-bundle-sha256>",
    "installedBundle": {
      "root": "<persistent-installed-plugin-directory>",
      "sha256": "<localized-installed-bundle-sha256>"
    }
  }
}
```

Never construct a replacement receipt from the target repository or the currently observed hashes.

## 3. Shift start (optional stages with `--all`)

- With `--audit`: send `{"action":"audit"}`. Acknowledge result with `handle`.
- With `--triage`: send `{"action":"triage"}`. Triage always parks at its human approval gate. Acknowledge with `handle` using `directive: "parked"`, and record the proposal without applying it.

Note: Exact selections (`--epic`, `--issue`, `--issues`) and chores-only shifts (`--chores`) exclude repository-wide audit and triage.

## 4. Workload cycle

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
   - `bootstrap`: A `--chores` repository without chores configuration opened a researched `guards/bootstrap-chores` PR for Cooper's review; park shift cleanly.
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

## 5. Failure and refusal recovery

This control path is for interactive mode while `phase` is `"failed"`.
Autonomous mode stops on the first recorded refusal and preserves the terminal report.
A `"stopped"` shift cannot accept `recover`; preserve its evidence and obtain authorization for a separate bounded shift.

If `phase` is `"failed"`:

1. Reconcile stored state using `agy-guards inspect <workspace_root>`.
2. Verify the failed host process has exited.
3. If `state.stop` is absent, send `recover` with fresh reconciliation; no correction is needed. If `state.stop.kind` is `"refusal"` and an independently permitted alternative exists, the foreman may supply exactly one corrected retry for that tool and target within the remaining recovery budget. Authentication, content-filter and parked stops cannot use this path. Send `recover` with reconciliation and structured correction:

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

4. Keep the refused operation and target off limits. Never retry a private path through another tool. If the refusal repeats or cannot be corrected within existing policy, stop the shift:

   ```json
   {"action": "stop", "reason": "<concrete blocker description>"}
   ```

