+++
title = "supervisor"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tars-run-factory"
+++

# Factory supervisor

- Run this helper from the installed skill's `resources/manual/` directory after the skill's preflight.
- Requires Bun, Linux `/proc`, `flock` and the authorized `agy` installation.
- The foreman remains the decision-maker; this is its continuously reading transport and durable control loop.
- Keep the helper's process handle live throughout the shift.
- Read every control response; never send another control until the preceding response is handled.

## Start

```bash
bun factory.ts /absolute/customer/workspace --epic 42 --cycles 10 --runtime-minutes 480
```

- Add `--merge` only when authorized for the shift.
- Omit `--epic` only for an explicitly authorized whole-backlog run.
- `--epic` excludes `--audit` and `--triage`; those optional stages can change unrelated backlog.
- `--turn-minutes` defaults to 90; `--recovery-limit` defaults to 10.
- `--conversation ID` supplies an existing authorized conversation when importing a legacy handover or beginning an explicitly authorized new shift.
- Default state directory: `<workspace>/../tars-factory/<workspace-name>/`.
- `--state-dir DIR` selects a different directory outside the customer repository.
- Existing scope, limits, start time and position are immutable across interruption recovery.
- A kernel lock uses the canonical workspace path independently of the state directory.
- Existing agy processes in that workspace prevent acquisition; different workspaces may run concurrently.
- Preserve an existing legacy `FACTORY_LEDGER.md`; inspect its workflow and ownership before importing its conversation.

## Control protocol

- Feed one JSON object per line to the helper's stdin.
- Its stdout emits `ready`, `state`, `control_error` or `stopped` envelopes.
- Raw host output is continuously consumed into `FACTORY_LEDGER.ndjson`, even while the foreman is not displaying output.
- A CLI partial-output timeout or fatal `error:` stderr notice fails the host even when its result says SUCCESS or its exit code is 0.
- `phase: result` requires acknowledgement of that result's `resultSeq`.
- `phase: failed` requires fresh reconciliation or a stop; it never starts a replacement automatically.
- A `control_error` did not authorize progression; correct the control within existing permission or stop.

```json
{"action":"probe"}
```

- Verify the returned MCP tool list includes the required TARS surface.
- A missing server permits one `retry-probe` control naming the current `resultSeq`; it closes the old process and resumes the same conversation in a fresh process.
- A refusal uses refusal recovery instead of the missing-server retry.
- After the probe, use `audit` and `triage` only when those flags were authorized, then `cycle` for each factory leg.
- `status` sends the read-only `/tars-show-status` command.
- Completed probe, audit and triage stages cannot be repeated.

### Acknowledge a result

1. Read the full response and any refusal evidence.
2. Run fresh `tars-agy inspect <workspace>` and the applicable GitHub checks.
3. Reconcile the authorized scope, current owner, workflow phase, parked state, approvals, CI, merge status and independent review evidence.
4. Construct a progress fingerprint from meaningful scoped state, excluding polling timestamps and unrelated issues.
5. Send the current `resultSeq`, the measured directive, the fingerprint and actual evidence.

```json
{"action":"handle","resultSeq":17,"directive":"continue","progress":"scope-state-fingerprint","evidence":{"inspect":"actual inspected state","github":"actual checked state"},"produced":{"completed":[],"opened":[],"reviewed":[],"approved":[],"merged":[]}}
```

- `continue`: the authorized next leg may proceed.
- `pending_ci`: enforces ten minutes before the next turn; keep observing the existing job handles.
- `drained`: ends the shift after the foreman verifies scope completion; a transport SUCCESS alone is insufficient.
- `parked`, `approval_needed`, `pending_human_merge`, `human_door` and `stalled`: stop the shift.
- Blocked approval never becomes an automatic retry or merge, even when DOYLE is available.
- Triage proposals remain unapplied at their human gate; report them for the next authorized shift.
- The evidence and production fields are foreman assertions backed by the checks above, not evidence manufactured or independently verified by the helper.

### Recover a terminal transport failure

- The old host is stopped before recovery can run.
- Inspect the actual stored state again; classify the same authorized leg as `resume`, `completed` or `parked`.
- Supply a meaningful progress fingerprint and the actual inspection evidence.
- `completed` returns a reconciled result for acknowledgement without repeating the leg.
- `parked`, unchanged progress or exhausted recovery budget stops the shift.
- Recovery matches the recorded PID, conversation and leg; it never uses reset, implicit latest-conversation selection or a different owner.

```json
{"action":"recover","reconciliation":{"disposition":"resume","progress":"new-scope-state-fingerprint","evidence":{"inspect":"actual fresh state"}}}
```

- For a refusal, include `correction` only after the foreman establishes an independently permitted alternative under the skill's refusal rules.
- Required correction fields: exact `refusal`, `tool` and `target` from the failure receipt; `permittedBrief`; `permissionEvidence` explaining the existing authorization.
- Keep the refused operation and target off limits and forward the corrected brief to every affected spoke.
- Each tool/target has one durable corrected retry; repeated, uncertain or unresolved refusal stops.
- Authentication failures and parked directives cannot use corrected retries.
- Verify the corrected outcome against fresh response, permitted evidence and deny logs; report a recovered refusal explicitly.

## Stop and handover

```json
{"action":"stop","reason":"exact stopping condition and unfinished work"}
```

- Cycle exhaustion, runtime exhaustion and two consecutive failed or unchanged-progress results stop the shift.
- Runtime is wall time from the durable start, including waits and restarts.
- EOF, malformed control input and termination signals shut down the host and write `FACTORY_REPORT.md`.
- Normal stdin closure allows ten seconds for host cleanup before bounded termination escalation.
- Host failures immediately write a handover and leave the control loop available for permitted recovery.
- After a foreman interruption, restart with the same invocation and state directory; an orphaned host must be gone before acquisition.
- An unfinished durable leg requires reconciliation before new input.
- A deliberately stopped shift remains terminal; the next human decision authorizes a new bounded shift and any use of the recorded conversation.
- Never claim the intermittent 180-second timeout is fixed; terminal recovery does not diagnose its cause.

## Enforcement boundary

| Enforced by code | Requires foreman verification |
| --- | --- |
| Canonical workspace exclusion and old-process death | Applicable workspace and merge authorization |
| Incremental NDJSON consumption and durable positions | Meaning of incomplete or summarized tool output |
| One in-flight turn and receipt acknowledgement | Truth of workflow and GitHub evidence supplied at acknowledgement |
| Refusal detection, immediate host termination and retry budgets | Whether a correction is independently permitted and reaches every affected spoke |
| Fixed epic-scoped command and no batch fallback | Ratified epic scope and meaningful progress fingerprint |
| Limits, CI pacing, parked-state stop and handovers | Native child completion, independent writer stamps and executing-spoke evidence |
| Spawn identity inventory without private URI access | Review approval and merge readiness; spawn DONE is not child completion |

## Permitted transport smoke

- Obtain the applicable live-test approval and coordinate the workspace and installation window first.
- Start with `--smoke --cycles 2` and a dedicated state directory.
- Send two `status` controls, acknowledging each with distinct verified progress and `directive: continue`.
- Smoke mode only asks for a fixed reply and prohibits tools or file changes; it cannot run an epic, batch, audit or triage.
- Verify two SUCCESS results, one conversation, one launch, two handled receipts and a clean exit receipt.
- Check the report and current process state before releasing the window.

