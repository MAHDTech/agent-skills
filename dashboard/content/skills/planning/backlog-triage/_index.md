+++
title = "backlog-triage"
description = "Triage pending backlog issues in `.tars/issues/todo/` to verify their accuracy, check for hallucinations, and add review notes. Reach for this when requested to triage tickets, verify backlog accuracy, or prepare issues for implementation."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Triage

Triage pending issue tickets in `.tars/issues/todo/` (conforming to the template in [backlog-create-issue](@/skills/planning/backlog-create-issue/_index.md)) to verify their accuracy, identify implementation gaps, check for hallucinations, and append a detailed review section to each.

This skill runs in a Hub-and-Spoke topology using sub-agents to verify tickets in parallel.

## Targets and Paths

- Target Directory: `.tars/issues/todo/` relative to project root.
- Ticket files are updated on disk only, never staged, committed, or force-added to git (since `.tars/` is gitignored).

## Triage Workflow

### 1. Backlog Scan & Conflict-Free Grouping

List all markdown files in `.tars/issues/todo/`. Identify the unreviewed tickets (those missing a `## Review` section and having `status: todo` or missing a status). Rework tickets (with `status: rework`) already contain review notes and should not be re-triaged.

Group them into parallel batches of at most 5 concurrent subagents. Ensure that tickets within the same batch do not audit overlapping files to avoid git or environment conflicts.

### 2. Spawn Triage Spokes

For each ticket to verify in the batch, spawn a `research` subagent in a dedicated, isolated workspace.

Since `.tars/` is gitignored, the subagent's workspace will not have access to the ticket files. Because triage agents are read-only and return their findings directly to the Hub via message, the Hub only needs to pass the ticket content directly in the subagent's prompt (copying the file to the subagent workspace is not required).

Equip each subagent with:

- **TypeName**: `research`
- **Role**: `Triage-<TICKET_ID>` (substitute the 3-digit ticket ID, e.g. `Triage-044`)
- **Workspace**: `branch`
- **Prompt**:

  ```text
  You are auditing a pending issue ticket to verify its readiness and accuracy against the codebase.

  Ticket Content:
  <TICKET_CONTENT>

  Instructions:
  1. Source Code Verification: Locate the exact directories, files, functions, and line references mentioned in the ticket. Read those parts of the codebase to verify if the description and tasks match the actual code today.
  2. Implementation Readiness Check:
     - Does the ticket contain sufficient detail for a fresh agent with a small context window to implement the task?
     - Are the YAML frontmatter, tasks, and acceptance criteria formatted according to the guidelines in [backlog-create-issue](../../planning/backlog-create-issue/SKILL.md)?
     - Are there any gaps? (e.g., missing package configurations, unmentioned side effects, compile-time type errors, build script modifications).
     - Are there any hallucinations? (e.g., non-existent files, deprecated APIs, incorrect function signatures, wrong line references).
  3. Assess Constraints: Check for platform compatibility concerns (Node vs Bun APIs, Windows path resolution/CRLF issues) and repository-specific guidelines.
  4. STRICT ISOLATION CONSTRAINT: You must NEVER check out the source/main branch, commit directly to the source/main branch, or attempt to merge branches. You must operate strictly within your local isolated workspace.

  Formulate a detailed review of this ticket. If it is accurate and ready, state that. Otherwise, list the critical findings or gaps as bullet points.
  ```

### 3. Save Updates & Cleanup (Hub Only)

When a subagent completes:

1. The Hub receives the subagent's review markdown.
2. The Hub appends (or replaces) a `## Review` section at the bottom of the local ticket file in `.tars/issues/todo/` following this exact formatting:

   ```markdown
   ## Review

   - Finding A
   - Finding B
   ```

3. Save the file to disk (do **NOT** stage or commit).
4. **CRITICAL CLEANUP CONSTRAINT**: As the Hub, you MUST clean up each subagent's worktree and branch immediately, regardless of whether the subagent succeeded, failed, or timed out. Failure to do so will break future iterations.
   - Run `git worktree remove --force <path>`
   - Run `git branch -D <branch-name>`

Repeat for subsequent batches until all tickets in `.tars/issues/todo/` have been triaged and contain a `## Review` section.

