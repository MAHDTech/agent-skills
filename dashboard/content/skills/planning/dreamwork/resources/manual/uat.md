+++
title = "uat"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "dreamwork"
+++

# UAT and landing

## Run verification

When every issue is staged or parked:

1. Dispatch the Reviewer target in a fresh worktree (role `test`) to reset its
   checkout to the run-branch head, run preflight, and run the plan's final gates.
   It does not review code. Its receipt is the run-level gate evidence.
2. If a final gate fails, find the responsible issue from the evidence and send it
   back through repair, Reviewer verification and restaging. If the cause is unclear
   or spans issues, park the run's remaining work as not converged and report it.
3. Keep that worktree. It is the UAT checkout. Record its path and head locally.

For audits there is no run branch; the accepted report is the UAT subject.

## The packet

Write `uat/packet-<n>.md` in local state. The human should be able to finish it in
about ten minutes. Lead with what needs them.

1. **Decisions needed:** every Tier C question, parked issue and unresolved dispute.
   Give the evidence, options and the Architect's recommendation for each.
2. **How to run it:** the project's UAT launch command and checkout, how to reach
   the app, and the run-branch head it shows.
3. **Checklist per issue:** what changed in user terms, the steps to try, the
   expected result, and screenshots or recordings captured by agents. Mark the
   steps that only a human can do (physical actions, real devices, taste).
4. **Evidence summary:** final gate results, council score tables, repair and
   recovery counts, fallbacks used, Tier A and B decisions, and reclassified scope
   creep.
5. **Known limitations** and proposed follow-up issues (not yet created).

Notify the human (autonomy manual) and end the turn with owned services stopped.

## Running UAT

On `uat`:

1. Recheck that the UAT checkout is at the recorded head and clean. Start the UAT
   environment there with project isolation rules, recording ownership locally.
2. Walk the human through the packet. Record every verdict, answer and finding
   verbatim with its message reference in `uat/decision-<n>.json`. An ambiguous
   answer gets one clarifying question, not an assumption.
3. Tear down the environment and verify it stopped.

Outcomes:

- **Approved:** land the approved issues.
- **Findings:** each human finding gets an ID (`U01`, ...). Send them through the
  Implementer, then a fresh Reviewer that also checks each finding's fix. UAT
  repairs have their own counter and do not consume `max_repair_rounds`. Rerun the
  council only if the human asks or the fix needs a Tier B decision. Build a new
  packet; only the human closes `U` findings.
- **Partial approval:** land approved issues whose dependencies are also approved,
  following the same landing rules, and keep the rest on the run branch for repair.
  If landing a subset needs history surgery the plan did not anticipate, ask the
  human rather than rewrite history.
- **Rejected or abandoned:** keep branches and worktrees, record the decision, and
  commit only the records if the human wants them kept.

## Landing

Only after explicit UAT approval, in the captured source checkout:

1. Verify it is the captured repository, on the captured branch, and clean. If not,
   stop and ask. The human is present at this point.
2. If the source branch has moved past the run's execution base, dispatch the
   Implementer to rebase the run branch onto it in the UAT checkout, then a fresh
   Reviewer to verify the rebase and rerun final gates. Tell the human what changed
   between the tree they tested and the tree that will land, and get a yes if the
   rebase touched anything they tested.
3. Fast-forward: `git merge --ff-only <run-branch>`. Never land a tree that no
   Reviewer has verified, and never create a merge commit the plan did not approve.
4. Write one bookkeeping commit following project commit rules. It imports the run
   records (see [records](@/skills/planning/dreamwork/resources/manual/records.md#landing-import)), moves issues to their
   completed state and review dossiers to their project location, and updates
   indexes. Never describe it as reviewed.
5. Verify all owned services are stopped. List the remaining worktrees and the run
   branch, and offer to clean them up. Do not push, publish or open a PR unless the
   human asks separately.

For audits, landing is steps 1 and 4 only: the bookkeeping commit adds the accepted
report and run records.

