+++
title = "backlog-review"
description = "Review the code implementation of a backlog ticket on a subagent branch, assessing compliance with spec/acceptance criteria and repo standards before merge. Reach for this during the implementation phase of the backlog loop."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "review"
mermaid = false
+++


# Backlog Review

Review the implementation of a completed backlog issue against its requirements, acceptance criteria, evidence, and repository standards before it is merged into the main/source branch.

This skill is invoked by the Hub for each branch implemented by a subagent. It conducts a double-axis review (Spec and Standards) and returns a verdict of `Approved` or `Request Rework`.

## Targets and Paths

- Target Directory: `.tars/issues/todo/` relative to project root.
- Ticket files are updated on disk only, never staged or committed in git (since `.tars/` is gitignored).

## Inputs

- **Ticket File Path**: Path to the ticket markdown file (e.g. `.tars/issues/todo/001.md`).
- **Implementation Branch**: The git branch containing the changes (e.g. `subagent-001`).
- **Target Branch**: The main branch to compare against (e.g. `main` or `master`).

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

### 2. Spawn Review Spoke

Spawn a `research` subagent to perform the double-axis review of the diff against the ticket requirements and codebase standards.

Equip the subagent with:

- **TypeName**: `research`
- **Workspace**: `branch`
- **Prompt**:

  ```text
  You are an implementation reviewer auditing changes made for a backlog ticket.

  Ticket Content:
  <TICKET_CONTENT>

  Code Diff:
  <CODE_DIFF>

  Instructions:
  1. Spec Alignment Review:
     - Check if all tasks under '## Tasks' are fully implemented in the diff.
     - Check if all '## Acceptance Criteria' are satisfied by the code.
     - Verify that the '## Evidence' provided corresponds to the implemented code and test output, and is not fabricated.
  2. Standards & Quality Review:
     - Verify code complies with project conventions (e.g. styles, typing, architectures).
     - Check for common code smells (e.g. duplicated logic, hardcoded values, lack of error handling, missing unit tests).
  3. Formulate Verdict:
     - If the implementation is complete, correct, and standards-compliant, output: VERDICT: APPROVED.
     - If there are missing tasks, failed criteria, incorrect logic, or standards issues, output: VERDICT: REQUEST REWORK, followed by bulleted, actionable feedback detailing what needs correction.
  ```

### 3. Parse Verdict & Output (Hub Only)

- The Hub parses the subagent's response.
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

- [backlog-create-issue](@/skills/planning/backlog-create-issue/_index.md) — canonical format and standards for backlog issues.
- [backlog-implement](@/skills/engineering/backlog-implement/_index.md) — implementation phase that dispatches tasks to spokes.
- [backlog-loop](@/skills/planning/backlog-loop/_index.md) — orchestration of the full backlog pipeline.

