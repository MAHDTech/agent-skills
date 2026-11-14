---
name: dreamwork
description: Plan issues with the human, then autonomously research, implement, review and council-check them in isolated T3 Code worktrees, returning to the human only for final UAT before landing. Use only when the user explicitly requests Dreamwork.
disable-model-invocation: true
---

# Dreamwork

Use this skill only when the user explicitly requests its workflow.

Dreamwork is a T3 Code procedure built from skill instructions, Markdown and JSON.
There are no scripts, daemons, schedulers or extra MCP servers. Discussing,
configuring or editing this skill does not start a run.

It is project-agnostic. Every project fact (gates, services, issue tracker, commit
rules, UAT launch) comes from the repository's `AGENTS.md` and the documents it
references, resolved through the [project contract](resources/manual/project-contract.md).
Never carry a default toolchain, tracker or folder layout from another project.

## Shape of a run

```text
Plan ──► council plan review ──► HUMAN GATE 1: approve plan commit
  ──► per issue (parallel along the dependency graph):
        implement ─► reviewer ⇄ repair ─► final council ⇄ repair ─► stage on run branch
  ──► run verification + UAT packet ──► HUMAN GATE 2: UAT
  ──► land on source branch (or repair UAT findings ─► UAT again)
```

The human is the bottleneck. Between the two gates the run never waits for them:
decisions inside approved authority are made and logged, everything else parks the
affected issue and goes into the UAT packet while independent work continues. See
[autonomy](resources/manual/autonomy.md).

## Requests

Accept natural language and `plan <goal or selector>`, `resume <run-id>`,
`status [run-id]`, `uat [run-id]` and `stop <run-id>`. With no actionable goal,
report status.

```text
$dreamwork plan 017
$dreamwork plan all issues labelled mvp
$dreamwork plan epic E001 wave 1
$dreamwork plan reconnect handling
$dreamwork audit the codebase for security problems; report only
$dreamwork uat reconnect-handling-20261109-01
```

Read manuals when their stage starts, not all up front:

| Manual                                                   | Read before                                              |
| -------------------------------------------------------- | -------------------------------------------------------- |
| [configuration](resources/manual/configuration.md)       | resolving role targets                                   |
| [project contract](resources/manual/project-contract.md) | planning; it lists the facts to resolve from `AGENTS.md` |
| [issues](resources/manual/issues.md)                     | selecting, drafting or transitioning issues              |
| [autonomy](resources/manual/autonomy.md)                 | approval, and any mid-run decision                       |
| [council](resources/manual/council.md)                   | plan review, final review or amendment review            |
| [verification](resources/manual/verification.md)         | writing the gate plan                                    |
| [records](resources/manual/records.md)                   | writing or reconciling run state                         |
| [role briefs](resources/manual/role-briefs.md)           | any delegation                                           |
| [UAT and landing](resources/manual/uat.md)               | building the UAT packet or landing                       |

Children receive the [child contract](resources/manual/child-contract.md) and their
brief, not these Architect manuals.

## Roles

- **Architect**: the active session. Owns plan, records, dispatch, Tier A/B
  amendments, run branch, UAT coordination and landing. Never accepts its own work,
  never closes a finding, never edits product code.
- **Researcher**: read-only investigation with distinct scopes; may run in parallel.
- **Implementer**: one writer per issue in its own T3 worktree. Owns code, tests and
  required product docs, and responds to findings.
- **Reviewer**: one fresh delegate per repair round. Runs the gates on the exact
  candidate in its own worktree and owns its findings. Also runs run verification.
- **Council**: two or three seats on different providers. Reviews the plan before
  Gate 1, each issue's reviewer-accepted candidate before staging, and Tier B
  amendments. Read-only.

Children never spawn agents, merge, push, publish or open PRs.

## Invariants

- Exactly two human gates: plan approval and UAT. Nothing reaches the captured
  source branch before UAT approval. Approval grants no push, publication or PR.
- Between gates, never block on the human. Park and continue (autonomy manual).
  Only out-of-authority decisions or human-only inputs (secrets, physical actions,
  taste) need them, and those wait for the UAT packet. When every remaining issue
  is parked, the run goes straight to the UAT packet.
- Fallback targets come only from the approved config snapshot. Never improvise a
  provider, model or option.
- Only the role that raised a finding (or the council, under its dispute rule)
  withdraws or verifies it. The human decides findings escalated to UAT.
- Product edits invalidate review. A changed candidate needs fresh review.
- One Architect per run. Competing Dreamwork runs targeting the same source branch
  block execution until reconciled.

## 1. Plan (interactive)

1. Resolve configuration and call `orchestrator_capabilities` once per thread
   (again only after an error). Validate every configured target, including
   fallbacks; mark unavailable ones in the plan. Do not validate the Architect.
2. Resolve the project contract from `AGENTS.md`. Ask only about missing or
   conflicting facts, one at a time with a recommendation, and offer the exact
   `AGENTS.md` text that would settle them.
3. Capture the initiating checkout, repository identity, branch ref and full HEAD.
   That branch is the landing destination. Never infer a default or upstream branch.
   A detached checkout needs an explicit destination.
4. Establish outcome, kind (`delivery` or `audit`), constraints, non-goals and
   acceptance. Fresh prompts get draft issues following project conventions.
5. Investigate directly or with Researchers in parallel. Pin evidence to revisions;
   separate facts, inference and gaps.
6. Write the plan: goal, research, ordered issues with dependencies, acceptance,
   per-issue scope (initial paths plus declared boundaries for autonomous growth),
   gate plan and preflight inventory, automation of human-only checks, UAT checklist
   per issue, and the run branch name. Walk each user journey end to end (entry,
   exit, adjacent destinations, every adapter or transport) when deriving scope.
   Flag issues too broad for one review cycle and propose a split.

Planning may read a dirty checkout. Execution needs a clean one. Never stash or
commit unrelated edits.

## 2. Council plan review

Dispatch the council on the committed plan proposal (see records for the proposal
commit). Fix blocking findings and repeat while the human is present. Show the
human the score table, open disagreements and anything you declined as scope creep.

## 3. Gate 1: approval

Present the proposal commit, the issue list and order, scope boundaries, gates,
standing authority (autonomy manual), effective config with fallbacks, and the
expected UAT checklist. Approval must be an explicit human message naming or clearly
accepting that proposal; invocation alone is not approval. Then create the approval
commit and run branch, per the records manual. After this point, run without the
human.

## 4. Execute each issue

Dispatch issues whose dependencies are staged, up to `workflow.max_parallel_issues`.

1. **Implement.** `delegate_task`, role `implementation`, `isolation: "worktree"`.
   The child resets its fresh checkout to the named base (current run-branch head),
   runs preflight, implements within scope, commits at milestones, runs all
   automated gates and captures UAT evidence such as screenshots.
2. **Validate the handoff.** Check the receipt against the child contract: exact
   commit/tree, changed paths within scope, every required gate with reliable exit
   evidence, clean worktree, services stopped. A terminal provider event is not a
   finished handoff; incomplete handoffs go to recovery (records manual).
3. **Review loop.** Fresh Reviewer per round (round 0 first) in its own worktree on
   the exact candidate. On `changes_requested`, increment the repair round, send
   findings to the Implementer (continue its thread with `t3_thread_send` once its
   previous turn stopped), validate, and dispatch a new Reviewer. A finding disputed
   in two consecutive rounds goes to the council.
4. **Final council.** When the Reviewer accepts, dispatch the council on that exact
   candidate with the Reviewer's gate evidence, screenshots, disputes and Tier A
   amendments to ratify. Blocking findings go back through repair and a Reviewer fix
   check, then a new council round, up to `workflow.max_council_rounds`.
5. **Stage.** If the accepted candidate descends from the current run-branch head,
   fast-forward the run branch to it (records manual). Otherwise the Implementer
   rebases onto the run-branch head and a fresh Reviewer verifies the rebased
   candidate and its conflict resolutions; council re-review is needed only if the
   reconciliation changed product behaviour.

Non-convergence (repair budget exhausted after any extension, council rounds
exhausted, infrastructure budget exhausted) parks the issue and its dependents with
full evidence. Independent issues continue.

For audits, Researchers write the report, the Reviewer checks it against the plan's
coverage and acceptance, and the final council reviews it. No implementation is
dispatched and audit findings are reported, not fixed.

## 5. Run verification and the UAT packet

When every issue is staged or parked, dispatch the Reviewer target in a fresh
worktree to run the plan's final gates at the run-branch head. That worktree becomes
the UAT checkout. Build the UAT packet and notify the human (UAT manual). Stop owned
services before ending the turn.

## 6. Gate 2: UAT

When the human returns (`uat`), start the UAT environment from the UAT checkout per
the project contract, walk them through the packet, record each verdict and
decision verbatim with its message reference, then tear the environment down.
UAT findings go back through the Implementer and a Reviewer, then a new packet.

## 7. Land

Only after explicit UAT approval: verify the source checkout is still on the
captured branch and clean, reconcile any source advance through a reviewed rebase,
fast-forward, then write one bookkeeping commit with imported records and issue
transitions (UAT manual).

## Status, resume and stop

- Follow T3's returned `next`. When a completion will wake the thread, end the
  turn. Never poll, sleep, schedule loops or add watchers.
- `status` is read-only: reconcile local state with Git and T3, then report each
  issue's stage, candidate, repair and recovery budgets, open findings, parked
  reasons, child links, recent child progress lines and the next action. Silence
  never proves completion; an empty task list never proves nothing is running.
- `resume` reconciles records, local handles, Git and provider state before any
  dispatch. Consume each receipt once. Missing local data means reconstruction, not
  replaying ambiguous side effects.
- `stop` cancels active children with `task_cancel`, stops owned services,
  preserves worktrees, branches and evidence, and marks the run paused. Resuming
  needs an explicit request.
- Request secrets with the host's secret-request tool, never chat or files. Store
  no secrets in records.
