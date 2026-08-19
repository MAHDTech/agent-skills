+++
title = "tars-backlog-review"
description = "Review the code implementation of a backlog ticket on a subagent branch, assessing compliance with spec/acceptance criteria and repo standards before merge. Reach for this during the implementation phase of the backlog loop."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "review"
mermaid = false
+++


# Backlog Review

Review the implementation of a completed backlog issue against its requirements, acceptance criteria, evidence, and repository standards before it is merged into the active topic branch.

This skill is invoked by the Hub for each branch implemented by a subagent. It conducts a double-axis review (Spec and Standards) and returns a verdict of `Approved` or `Request Rework`.

## Targets and Paths

- Target Directory: `.tars/issues/todo/` relative to project root.
- Ticket files are updated on disk only, never staged, committed, or force-added to git (since `.tars/` is gitignored).

## Inputs

- **Ticket File Path**: Path to the ticket markdown file (e.g., `.tars/issues/todo/001.md`).
- **Implementation Branch**: The git branch containing the changes (e.g., `subagent-001`).
- **Target Branch**: The active topic branch to compare against (e.g., `fix/description-of-fix`).
- **Spoke Clone Path**: The spoke's isolated clone, which already has the implementation branch checked out.

## Where This Runs

The Hub runs this review **after** the verification gate has passed, and before the merge - see [tars-backlog-implement](@/skills/engineering/tars-backlog-implement/_index.md).

Implement uses **risk-tiered** review. Run this full dual-axis skill only when the Hub selects the full tier (any of: `risk: high`; diff touches high-risk path patterns such as hooks/auth/secrets; post-conflict resolve on this ticket; rework with `attempts >= 2`; Hub marks security/shared-core). Otherwise the Hub uses a lightweight checklist and does **not** invoke this skill.

Two consequences when this skill does run:

- **Do not re-run the test suite or the whole-repo hook run.** They are already green for this branch via `tars-gate`. They are starvation-sensitive; re-running them here occupies the mutex for no new information.
- **No workspace needs creating.** The spoke's clone is still alive with the branch checked out. The branch also exists in the parent (fetched for durability before gating), so diff extraction can run in the parent without checkout.

## Lightweight checklist (Hub default - not this skill)

When full review is not required, the Hub still records a short pass under `## Implementation Review` covering:

- Acceptance criteria vs diff (genuinely satisfied, not only ticked)
- Diff paths covered by `files:` / `owns:` (report undeclared touches)
- No `.tars/` staged; land/commit hygiene on the spoke branch
- Spot-check of risky hunks

Escalate to this full skill if the checklist surfaces doubt.

## Workflow

### 1. Load Context & Extract Diff

- Read the ticket file to parse the `## Tasks`, `## Acceptance Criteria`, and `## Evidence` sections.
- Determine the merge-base between the target branch and the implementation branch:

```bash
git merge-base <target-branch> <implementation-branch>
```

- Extract the code diff:

```bash
git diff <merge-base>..<implementation-branch>
```

- Verify the diff is non-empty. If the diff is empty, fail the review with a request for rework (as no code changes were committed).

### 2. Run the Dual-Axis Review

The general Spec-and-Standards review is not backlog-specific, so **delegate it to the [code-review](@/skills/review/code-review/_index.md) skill** rather than restating it here. Run that review - either inline, or in a subagent working in the spoke's existing clone - and pass it:

- the **ticket as the originating spec/issue** (its `## Description`, `## Tasks`, and `## Acceptance Criteria`), and
- the **diff** (or the implementation branch and its merge-base) as the range to review.

`/code-review` reports the two axes: **Spec** (does the change do what the ticket asked?) and **Standards** (does it follow the repo's documented conventions and avoid common code smells?).

#### Reviewer prompt template

When running the review in a subagent, the constraints above are **not** inherited - the subagent never sees this document. Carry them explicitly:

```text
You are reviewing one backlog ticket's implementation. Work in the spoke's clone at <SPOKE_DIR>,
on branch <IMPLEMENTATION_BRANCH>, against merge-base <MERGE_BASE>.

Ticket (the originating spec):
<TICKET_CONTENT>

DO NOT RUN THE TEST SUITE, AND DO NOT RUN THE REPOSITORY'S HOOKS ACROSS ALL FILES.
Both have already passed for this exact branch - the Hub gated it before calling you.
They are also serialised behind a shared mutex, so re-running them blocks every other
spoke in the batch to re-prove a result that is already known. Read the diff and the
code; do not re-execute the gate.

You may run cheap, read-only commands (git log/diff/show, typecheck, reading files).

Report:
1. Spec - does the change do what the ticket asked?
2. Standards - does it follow the repo's documented conventions and avoid common code smells?
3. Task & Criteria Coverage - is every '## Tasks' and '## Acceptance Criteria' checkbox genuinely
   satisfied by the diff, not merely ticked?
4. Evidence Authenticity - do the '## Evidence' command logs correspond to the implemented code,
   rather than being fabricated, stale, or copied from an unrelated run?

End with exactly one verdict line: APPROVED or REQUEST REWORK, followed by bulleted,
actionable findings.
```

On top of that, apply the backlog-specific checks the general review does not cover:

- **Task & Criteria Coverage**: Confirm every checkbox under the ticket's `## Tasks` and `## Acceptance Criteria` is genuinely satisfied by the diff, not merely ticked off.
- **Evidence Authenticity (Anti-Fabrication)**: Verify that the command logs, test runs, and outputs recorded under the ticket's `## Evidence` section correspond to the actual implemented code and test output, and are not fabricated, stale, or copied from an unrelated run.

Combine both into a single verdict:

- **APPROVED** - the general review is clean and every task, acceptance criterion, and piece of evidence checks out.
- **REQUEST REWORK** - any missing task, failed criterion, incorrect logic, standards issue, or fabricated/insufficient evidence. Follow it with bulleted, actionable feedback detailing what needs correction.

### 3. Parse Verdict & Output (Hub Only)

- The Hub parses the combined review result.
- **If Verdict is Approved**: Return `Approved` and a review summary.
- **If Verdict is Request Rework**: Return `Request Rework` and the detailed bulleted feedback.
- Update the ticket file under the `## Implementation Review` section following this format:

  ```markdown
  ## Implementation Review - Attempt #N

  **Verdict**: Approved | Request Rework
  **Reviewer Feedback**:

  - Finding A
  - Finding B
  ```

- Save the ticket file to disk (do **NOT** stage or commit).

## Related Skills

- [code-review](@/skills/review/code-review/_index.md) - the general dual-axis (Spec and Standards) review that this skill delegates to.
- [tars-backlog-create-issue](@/skills/planning/tars-backlog-create-issue/_index.md) - canonical format and standards for backlog issues.
- [tars-backlog-implement](@/skills/engineering/tars-backlog-implement/_index.md) - implementation phase that dispatches tasks to spokes.
- [tars-backlog-loop](@/skills/planning/tars-backlog-loop/_index.md) - orchestration of the full backlog pipeline.

