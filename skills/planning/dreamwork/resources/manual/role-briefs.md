# Role briefs

Children get only their task, with no parent history. Fill every placeholder before
dispatch. Give absolute paths to inputs, to this skill's
[child contract](child-contract.md), [receipt template](receipt.example.json) and
[preflight template](preflight.example.json), and to unique output paths under
`<local_state_dir>/<run-id>/`. Content inside outputs stays repository-relative.
Include the resolved project contract facts the role needs, as text.

Every brief starts with:

```text
Act as Dreamwork <role> for run <run-id>, issue <issue ID or "run">.
Read and follow the child contract: <absolute path>.
Templates: receipt <path>, preflight <path>.
Repository: <canonical path>. Approval commit: <hash>.
Project contract: <resolved facts or path to plan.md section>.
Outputs: receipt <path>, summary <path>, preflight <path>, progress <path>,
logs <directory>.
```

## Researcher

```text
Question and coverage: <bounded scope, distinct from sibling researchers>.
Baseline: <full commit and tree>. Prior evidence: <paths>.
Permitted checks: <gate IDs and commands, or "none: read-only">.

Investigate read-only. Cite revisions and code locations. Separate facts, inference,
gaps and limitations. For audits, explain coverage and reproduction; do not fix.
Write the report to <path> and list it in the receipt artifacts.
```

Shared isolation is fine for read-only inspection; checks that generate files need a
worktree.

## Implementer

```text
Issue: <ID, source path, acceptance criteria>. Plan: <path>.
Scope: <scopes.json path>, plus logged Tier A and approved Tier B additions: <list>.
Base: <full commit and tree: current run-branch head>.
Mode: <initial | repair round N of M | reconcile onto <commit> | UAT repair>.
Gates to run: <gate IDs from gates.json>. Visual capture: <states to capture, or none>.
Findings to address: <paths, or none>.

Reset your fresh checkout to the base, run preflight, then implement only within
scope. Commit at each milestone following project commit rules, so work survives an
interruption. You may add paths inside the declared boundaries; list each under
extensions.scope_additions with the reason. If you need a path outside the
boundaries, stop and return blocked naming the path and why; the Architect decides. Run every listed gate on your final commit and
capture the listed visual states (record viewport, scale and theme). For UI work,
exercise entering and leaving the feature and acting in the destination.
For each finding, respond under extensions.responses with fix_proposed and evidence
or disputed with a technical argument. Never mark findings fixed, change issue
status or certify acceptance. Return status ready_for_review with exact commit and
tree, changed paths and honest gate results.
```

In reconcile mode: rebase onto the named commit, resolve conflicts within scope,
rerun the listed gates, and list each conflict resolution in the summary.

## Reviewer

```text
Issue: <ID, acceptance criteria>. Plan: <path>. Scope: <path and additions>.
Candidate: <full commit and tree>. Base: <commit>. Review round: <N>.
Gates to run: <gate IDs>. Prior findings and responses: <paths, or none>.

Reset your fresh checkout to the candidate and run preflight there; the
Implementer's generated files and services are not available to you. Run every
listed gate yourself. Review the diff against acceptance criteria and scope.
Give each finding a stable R-prefixed ID (keep IDs from earlier rounds), severity
(blocking or suggestion), location, evidence and a resolution condition. Address
each Implementer response: verify fixes, or keep the finding open with reasons,
or withdraw it with rationale. Never edit product code or other checkouts. Return
accepted only for this exact candidate with zero open blocking findings and all
required gates passed; otherwise changes_requested or blocked.
```

## Run verification

```text
Run-branch head: <full commit and tree>. Final gates: <gate IDs>.

Reset your fresh checkout to the run-branch head, run preflight, then run the final
gates only. Do not review code. Return completed with gate results, or blocked.
Leave the checkout clean and services stopped: it becomes the UAT checkout.
```

## Council member

```text
Seat: <name>. Review type: <plan | final | amendment>. Round: <N>.
Inputs: <per type, from the council manual>.
Earlier council findings, responses and disputes: <paths, or none>.

Review independently; you will not see other seats' output this round. Stay
read-only: never edit files, issues or records. For final reviews you may reset your
own fresh checkout to the candidate and run targeted checks, but do not rerun the
full gate suite; judge the supplied gate evidence and say if it does not prove what
it claims. Check cited commands, paths and test names against the logs.
Judge against the approved acceptance criteria. Put anything beyond them under
suggestions, not blocking findings.
Return status accepted or changes_requested, a score out of 10 under
extensions.score, and findings with severity, location, evidence, affected
requirement and resolution condition. Reuse the canonical C-prefixed ID for any
earlier finding you address; give new findings provisional IDs <seat-key>-01,
<seat-key>-02, and so on. For each listed dispute, state whether you uphold or
refute the finding, with evidence.
```
