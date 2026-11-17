# Council

The council is two or three read-only reviewers from different model families. It
runs at three points: plan review before Gate 1, final review of each issue's
candidate, and Tier B amendment review. Repair rounds in between use the single
Reviewer, which keeps cost down.

## Dispatch

- One `delegate_task` per seat, all in the same turn, each with that seat's first
  available target. Use `role: "review"`.
- Plan and amendment reviews read committed files and may use `isolation: "shared"`.
  Final reviews use `isolation: "worktree"` so a seat can switch its own checkout to
  the candidate and run targeted checks.
- Each seat gets the [council brief](role-briefs.md#council-member) for its review
  type, the [child contract](child-contract.md), and the evidence listed below. Seats
  do not see each other's output in the same round.
- If a seat's targets are all unavailable, continue with the rest when at least
  `council.min_seats` respond, and record the missing seat. For plan review the human
  is present, so tell them and let them decide. Below the minimum, park the affected
  issue (or, for Tier B, treat the amendment as Tier C).
- Every round is a fresh `delegate_task`. Never continue a seat's earlier thread for
  a new round.

## Evidence per review type

- **Plan:** the proposal commit, plan, scope, gate plan, issues and research.
  Questions: does scope cover every user journey end to end? Is each issue small
  enough to review? Are acceptance criteria testable? Do gates prove them? Which
  human-only checks could be automated? What is missing, risky or scope creep?
- **Final:** the exact candidate commit and tree; the base; the issue and acceptance
  criteria; scope and boundaries; Tier A decisions to ratify; the latest Reviewer
  receipt on this exact candidate, with its gate results as supplied evidence;
  Implementer receipts; captured screenshots or recordings; the finding history,
  including disputes. Seats judge correctness, acceptance, design and UX acceptance
  where the issue has UI, test sufficiency, and whether supplied evidence really
  shows what it claims. They do not rerun the full gate suite.
- **Amendment:** the proposal, its reason and evidence, the approved plan, and the
  current candidate.

## Verdicts and findings

Each seat returns a receipt with `status` `accepted` or `changes_requested`, a
score out of 10, and findings. Each finding is `blocking` or `suggestion`, with a
stable ID, location, evidence, the requirement it affects and a resolution
condition. Suggestions and scope expansions never block.

- **Do not wait idle.** As each seat returns, check its findings against the code
  and evidence, merge duplicates across seats, and assign each new finding a
  canonical `C` ID, recording every source seat and its provisional ID. Later rounds
  reuse canonical IDs. Then prepare the repair brief. Do not dispatch the repair until every seat
  in the round has returned or failed.
- **Validate claims.** Check cited commands, paths and test names against raw logs
  and the candidate. A seat's claim that is wrong about evidence is recorded as a
  reliability observation; its code findings still get checked on their merits.
- **Fight scope creep.** Requests beyond the approved acceptance criteria become
  suggestions or follow-up issue proposals in the UAT packet, not blocking findings.
  Record each one you reclassify and why.

A round passes when every responding seat returns `accepted` with no open blocking
finding. Scores are recorded, never gating.

## After a failed round

Blocking findings go to the Implementer as a repair round. A fresh single Reviewer
verifies the fixes and reruns invalidated gates on the new candidate, then the next
council round reviews that candidate. Stop at `workflow.max_council_rounds`; open
blocking findings then park the issue as not converged.

## Disputes

An Implementer may dispute a council finding with a technical argument and evidence.
The next round shows every seat the finding, the dispute and the evidence. The
finding is withdrawn if the seat that raised it withdraws it, or if every other
responding seat (at least two) refutes it with evidence. Otherwise it stays open.
Reviewer disputes escalated to the council follow the same rule, with the council
seats deciding in place of the Reviewer. The Architect never decides a dispute.

## Score table

Record per issue and per review type. Use the seat's `name`, and add the actual
model when a fallback served the seat.

```markdown
## Council review: <issue ID>

Rounds: 2

| Seat | Round 1 | Round 2 |
| --- | --- | --- |
| Claude Opus | 7/10 | 9/10 |
| GPT Sol | 8/10 | 10/10 |
| Gemini Carbon (served by fallback X) | 7/10 | 9/10 |
```

Write `no score` when a seat gave none and `unavailable` when it did not respond.
Never invent a score. The tables go into the UAT packet, and into a PR description
only if the human later asks for a PR.
