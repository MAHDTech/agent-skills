# Run records

Records support review and recovery. They are not locks, transactions or verified
identity. When records disagree with Git or T3, reconcile before acting.

Records reach Git at three points only: the plan proposal, the approval, and
landing. During execution everything lives in ignored local state, so the source
branch is never touched mid-run.

## Tracked layout

```text
<runs_dir>/<run-id>/
  plan.md             goal, research summary, issues, contract, UAT checklist
  research/           imported Researcher reports
  scopes.json         per issue: initial paths and declared boundaries
  gates.json          gate plan (see verification)
  config.json         effective config snapshot
  run.json            run summary (proposal, approval, landing)
  approval.json       added by the approval commit
  council/            plan review receipts
  # added at landing:
  events.jsonl
  decisions.jsonl
  amendments/         Tier B proposals with council receipts
  issues/<issue-key>/ attempts, reviews, council rounds, responses
  verification/       run verification receipts
  uat/                packets and decisions
```

Use the project's review dossier location instead of `issues/<issue-key>/` when the
project prescribes one. Issue keys are safe path components mapped to stable issue
IDs in `run.json`. Run IDs are unique safe path components, conventionally
`<slug>-<yyyymmdd>-<nn>`.

## Proposal and approval commits

1. **Proposal commit.** Commit the plan files, draft issues in their pending state,
   and `run.json` with status `awaiting_approval`. Present this commit's hash. Each
   revision during planning is a new proposal commit; never rewrite one already
   shown to the human.
2. **Approval.** The human approves a specific proposal commit in an explicit
   message.
3. **Approval commit.** Add `approval.json` with the approved proposal commit and
   tree, the human message reference and timestamp. Apply the issues' active
   lifecycle transitions and set `run.json` status `running`. Change nothing else; any
   other change needs a new proposal. This commit is the execution base.
4. **Run branch.** Create it at the execution base:
   `git branch <run-branch> <execution-base>`.

The approved proposal's tree hash is the approval identity. Receipts carry
`approval_commit`. Git preserves the approved bytes, so no separate snapshot copies
are needed.

## Staging on the run branch

No worktree ever checks out the run branch; children use their own T3 branches and
the UAT checkout resets its own branch. To stage an accepted candidate:

1. Confirm the candidate equals the commit the final council accepted, and that the
   current run-branch head is its ancestor:
   `git merge-base --is-ancestor <run-head> <candidate>`.
2. Move the branch with a compare-and-swap:
   `git update-ref refs/heads/<run-branch> <candidate> <run-head>`.
3. Record the staged commit and tree in live state and the event log.

A verified rebase at landing may move the run branch to rewritten commits with the
same compare-and-swap form. The run branch is Dreamwork-owned and unpublished.

## Local state

```text
<local_state_dir>/<run-id>.json     live state
<local_state_dir>/<run-id>/
  status.md                         human-readable progress
  events.jsonl                      append-only transitions
  decisions.jsonl                   autonomy decisions
  amendments/
  <issue-key>/                      child staging outputs, progress files
  logs/                             raw command logs (never committed)
  uat/
```

Live state holds: source checkout path, Git common directory, captured branch and
execution base, run branch and head, effective config reference, and per issue the
stage, status (`queued`, `implementing`, `in_review`, `council`, `staged`, `parked`,
`blocked_by_parked`, `landed`), counters, candidate, reviewed, council-accepted and
staged identities, T3 task IDs and links, worktree paths and heads, which target
served each task, and expected output paths.

Append an event (timestamp, issue, transition, reason, revisions, evidence paths)
before replacing live state, and replace it via a temporary file in the same
directory plus rename. Missing local state is uncertainty, not proof that nothing is
running: reconstruct from Git branches, worktrees and T3 task state before acting.

## Receipts

Children write receipts using [receipt.example.json](receipt.example.json); the
[child contract](child-contract.md) gives their rules. The Architect checks before
consuming:

- known `version`, all template keys, a `role` of `researcher`, `implementer`,
  `reviewer`, `verifier` (run verification), `council` (seat under
  `extensions.seat`) or `architect` (evidence the Architect collected during
  recovery), and the role's allowed statuses:
  Implementer `ready_for_review`; Reviewer and council `accepted` or
  `changes_requested`; Researcher and run verification `completed`; any role may
  report `command_pending` or `blocked`;
- `approval_commit`, `base_commit`, `candidate_commit` and `candidate_tree` match
  the dispatch, with full hashes;
- `changed_paths` within the issue's scope, its declared boundaries (log each
  boundary addition from `extensions.scope_additions` as a Tier A decision) and
  approved Tier B additions;
- every required gate present with reliable exit evidence on the matching revision,
  and the artifact hashes matching the files;
- services torn down, and a clean worktree where gates require it.

Consume a receipt once. Preserve stale or duplicate receipts but never advance state
from them. A ready or accepted handoff with any missing piece goes to handoff
recovery.

## Counters and findings

Per issue, starting at zero and never reset: `repair_round` (every product repair,
including council-driven and non-mechanical reconciliation),
`extension_granted`, `review_round`, `council_round`, `uat_repair_round`, and
`infrastructure_recovery_attempts`. Reserve a counter in live state before the
dispatch that consumes it.

Finding IDs are prefixed by source and stable across rounds: `R01` from Reviewers,
`C01` from the council (merged duplicates keep every source seat), `U01` from UAT.
Qualify them as `<issue-id>/<run-id>/C01` in run records. Dispositions: `open`,
`fix_proposed` (Implementer), `disputed` (Implementer), `verified_fixed`,
`withdrawn`, `escalated_to_council`, `escalated_to_uat`. Only the raising role,
the council dispute rule, or the human for `U` findings and escalations can set
`verified_fixed` or `withdrawn`.

## Handoff recovery

A terminal provider event is not a finished handoff. If receipts are missing or a
required command is still pending:

1. Record `handoff_incomplete` locally and reconcile actual task and command state
   once.
2. Reserve an infrastructure recovery attempt, then continue the original child
   with `t3_thread_send` to collect its command result and write the receipt.
3. If that fails again, reserve another attempt and either collect the result
   yourself with a supported foreground wait (recording the takeover) or, after
   confirming the writer stopped, dispatch a replacement.

The Architect may collect evidence but never grants acceptance. Never repeat a check
until you know whether the first run finished.

## Infrastructure recovery

Infrastructure failures are quota and rate limits, provider outages, harness
crashes, and evidenced host OOM or wedged services. Product defects, compile errors
and test failures are not, even if a retry might pass. When unsure, treat it as a
product failure.

1. Reconcile task state. Stop the writer (`task_cancel`) and any product-writing
   subprocesses, and confirm they stopped. Never run two writers on one issue.
2. Reserve an attempt from the issue's counter (the run counter for audits and
   planning tasks).
3. Preserve work. Implementers commit at milestones, so a replacement can usually
   start from the last commit on the old branch. Capture uncommitted changes as a
   binary staged patch, a binary unstaged patch and an archive of untracked files,
   with path and SHA-256 inventory in local state. Keep originals until restoration
   is verified by comparing hashes.
4. Continue the same thread when the same target is healthy. Otherwise dispatch a
   new `delegate_task` with the next fallback target; the new child resets its fresh
   worktree to the preserved commit and restores the snapshot before working.
5. When the counter is exhausted, park the issue (autonomy manual).

Interrupted repair rounds stay consumed. Recovery that only collects evidence
consumes no repair round. If a late continuation delivers the original receipts
after recovery, compare candidate, exit codes and log hashes, keep both, and consume
one.

## Landing import

At landing, copy into the tracked layout: events, decisions, amendments, every
receipt and its Markdown summary, council and UAT records. Copy bytes verbatim so
SHA-256 values still match; children already used repository-relative paths. Leave
raw logs, machine paths, process handles and secrets in local state, and list any
excluded files with their hashes in `run.json`. Write the final `run.json` with
status `completed` (or `partial`), each issue's landed commit and tree, and the
council score tables.
