---
name: tars-backlog-review
description: Review the code implementation of a backlog ticket on a subagent branch, assessing compliance with spec/acceptance criteria and repo standards before merge. Reach for this during the implementation phase of the backlog loop.
---

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

The general Spec-and-Standards review is not backlog-specific, so **delegate it to the [code-review](../code-review/SKILL.md) skill** rather than restating it here. Run that review — either inline, or in a subagent spawned on the implementation branch's isolated workspace/branch — and pass it:

- the **ticket as the originating spec/issue** (its `## Description`, `## Tasks`, and `## Acceptance Criteria`), and
- the **diff** (or the implementation branch and its merge-base) as the range to review.

`/code-review` reports the two axes: **Spec** (does the change do what the ticket asked?) and **Standards** (does it follow the repo's documented conventions and avoid common code smells?).

On top of that, apply the backlog-specific checks the general review does not cover:

- **Task & Criteria Coverage**: Confirm every checkbox under the ticket's `## Tasks` and `## Acceptance Criteria` is genuinely satisfied by the diff, not merely ticked off.
- **Evidence Authenticity (Anti-Fabrication)**: Verify that the command logs, test runs, and outputs recorded under the ticket's `## Evidence` section correspond to the actual implemented code and test output, and are not fabricated, stale, or copied from an unrelated run.

Combine both into a single verdict:

- **APPROVED** — the general review is clean and every task, acceptance criterion, and piece of evidence checks out.
- **REQUEST REWORK** — any missing task, failed criterion, incorrect logic, standards issue, or fabricated/insufficient evidence. Follow it with bulleted, actionable feedback detailing what needs correction.

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

- [code-review](../code-review/SKILL.md) — the general dual-axis (Spec and Standards) review that this skill delegates to.
- [tars-backlog-create-issue](../../planning/tars-backlog-create-issue/SKILL.md) — canonical format and standards for backlog issues.
- [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md) — implementation phase that dispatches tasks to spokes.
- [tars-backlog-loop](../../planning/tars-backlog-loop/SKILL.md) — orchestration of the full backlog pipeline.
