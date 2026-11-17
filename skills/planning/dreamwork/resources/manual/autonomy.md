# Autonomy between the gates

Plan approval grants standing authority for the run. The Architect uses it to keep
work moving while the human is away, records every use, and brings anything outside
it to the UAT packet. The human can still message mid-run, and an explicit message
can settle a Tier C item early, but the run never waits for one.

## Front-load human inputs

Most mid-run stops in practice come from things planning could have caught. Before
Gate 1, settle:

- secrets, sign-ins and external access the gates need (request them through the
  host secret tool while the human is present);
- device, browser and GPU availability for every planned check, including the
  human's own environment for UAT;
- which human-only checks can use an approved automated substitute;
- open product decisions. Write the recommended default into the plan so approval
  covers it.

## Decision tiers

| Tier | Covers | Who decides | Record |
| --- | --- | --- | --- |
| A | Adding paths inside an issue's declared boundaries; choices the plan left open that stay inside its acceptance criteria and non-goals | Architect | Decision log; final council ratifies |
| B | Paths outside declared boundaries but serving the approved goal; new dependencies; schema or migration changes; touching a security, auth or tenancy boundary; changing a gate's command without weakening it | Architect proposes, council approves before dispatch | Amendment record with council receipts |
| C | Changing goal, acceptance criteria or non-goals; adding or dropping issues; weakening or removing a gate; raising a budget; a config change beyond listed fallbacks; destructive or irreversible operations; anything needing remote, publication or credential authority | Human only | Parked; question in UAT packet |

Classify conservatively: when unsure between tiers, use the higher one.

Tier A ratification: the final council sees each Tier A decision for the issue. A
seat that rejects one raises a blocking finding; the fix goes through normal repair.

Tier B approval: dispatch an amendment council (see [council](council.md)). It
passes when every responding seat (at least `council.min_seats`) returns no
blocking objection. Otherwise rework the proposal once; a second rejection makes it
Tier C.

## Fallbacks and budgets

- **Model fallback.** On an infrastructure failure of a target (quota, outage,
  startup failure), reserve an infrastructure recovery attempt and use the next
  target in that role's or seat's list, following
  [records](records.md#infrastructure-recovery). A list with no remaining target
  parks the affected work.
- **Repair extension.** When an issue exhausts `max_repair_rounds`, grant
  `repair_extension_rounds` once if all hold: open blocking findings strictly
  decreased across the last two review rounds, no verified finding reopened, and the
  remaining work needs no Tier B or C decision. Record the evidence. Otherwise park
  as not converged.
- **Disputes.** A Reviewer finding disputed in two consecutive rounds goes to the
  council, whose dispute rule decides it. A dispute still open after
  `max_council_rounds` goes to the UAT packet with each side's evidence and the
  Architect's recommendation.

## Parking

Parking stops work on one issue without stopping the run:

1. Let the current writer finish its turn or cancel it with `task_cancel`; stop its
   owned services.
2. Preserve the worktree, branch, receipts and evidence.
3. Record the issue as `parked` with a reason (`not_converged`,
   `needs_human_decision`, `infrastructure_exhausted`, `missing_prerequisite`,
   `missing_access`), the evidence, the exact question for the human and the
   Architect's recommendation.
4. Mark issues that depend on it `blocked_by_parked`.
5. Continue independent issues. When nothing runnable remains, build the UAT packet.

Resuming a parked issue after the human answers starts a fresh review cycle on the
preserved candidate. Counters are never reset.

## Decision log and progress

Append every Tier A and B decision, fallback, extension, park and dispute
escalation to the run's local decision log (see [records](records.md)) with the
evidence and reason. The UAT packet summarises it.

Keep a short local `status.md` current at each milestone (dispatch, handoff, review
verdict, staging, park) so `status` and a returning human see progress without
asking. When the UAT packet is ready, or every remaining issue is parked, notify the
human through the host's notification mechanism if one exists; otherwise the turn's
final message is the notification.
