+++
title = "tars-backlog-triage"
description = "Triage pending backlog issues in `.tars/issues/todo/` to verify their accuracy, check for hallucinations, and add review notes. Reach for this when requested to triage tickets, verify backlog accuracy, or prepare issues for implementation."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Triage

Triage pending issue tickets in `.tars/issues/todo/` (conforming to the template in [tars-backlog-create-issue](@/skills/planning/tars-backlog-create-issue/_index.md)) to verify their accuracy, identify implementation gaps, check for hallucinations, and append a detailed review section to each.

This skill runs in a Hub-and-Spoke topology using sub-agents to verify tickets in parallel.

## Targets and Paths

- Target Directory: `.tars/issues/todo/` relative to project root.
- Ticket files are updated on disk only, never staged, committed, or force-added to git (since `.tars/` is gitignored).

## Topic Branch Workflow (Hub Only)

All backlog operations must run from a topic branch, never the default branch. Triage subagents are read-only and take no branch of their own — they read the topic branch's working tree as the Hub has it checked out. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) for the full policy and commands.

## Triage Workflow

### 1. Mode Detection & Backlog Scan

Determine the triage mode based on the user's instructions/invocation:

- **Normal Triage**: Triggered by default. To reduce token spend, identify only the unreviewed tickets (those missing a `## Review` section and having `status: todo` or missing a status). Tickets already having a `## Review` section or with `status: rework` must be skipped.
- **Adversarial Triage (Double-Check)**: Triggered if the user request contains keywords like `adversarial`, `double-check`, `re-triage`, or `force` (e.g., `/tars-backlog-triage perform an adversarial triage of the backlog`). In this mode, identify **all** tickets in `.tars/issues/todo/` (including those with existing `## Review` sections or `status: rework`) to be triaged/double-checked.

Group the identified tickets into parallel batches of at most 5 concurrent subagents. Overlapping files are fine here — triage agents only read, so they cannot conflict with each other. The batch limit bounds concurrent token and CPU spend, nothing more.

### 2. Spawn Triage Spokes

For each ticket to verify in the batch, spawn a read-only research subagent that reads the **parent working tree directly** — no clone, no worktree, no workspace setup or teardown.

Triage agents verify claims against the codebase and report back by message; they never commit. An isolated workspace would cost real setup on every batch and buy nothing, since nobody is modifying the tree they are reading.

The Hub passes the ticket content directly in the subagent's prompt, so no ticket file needs copying anywhere.

The safety this gives up is enforced by checking rather than by construction — see the cleanliness assertion in step 3.

Equip each subagent with:

- **Role**: `Triage-<TICKET_ID>` (substitute the 3-digit ticket ID, e.g. `Triage-044`)
- **Prompt**:
  Adjust the prompt depending on the triage mode. If in **Adversarial Triage** mode, append instruction #5:

  ```text
  You are auditing a pending issue ticket to verify its readiness and accuracy against the codebase.

  Ticket Content:
  <TICKET_CONTENT>

  Instructions:
  1. Source Code Verification: Locate the exact directories, files, functions, and line references mentioned in the ticket. Read those parts of the codebase to verify if the description and tasks match the actual code today.
  2. Implementation Readiness Check:
     - Does the ticket contain sufficient detail for a fresh agent with a small context window to implement the task?
     - Are the YAML frontmatter, tasks, and acceptance criteria formatted according to the guidelines in [tars-backlog-create-issue](../tars-backlog-create-issue/SKILL.md)?
     - Are there any gaps? (e.g., missing package configurations, unmentioned side effects, compile-time type errors, build script modifications).
     - Are there any hallucinations? (e.g., non-existent files, deprecated APIs, incorrect function signatures, wrong line references).
  3. Assess Constraints: Check for platform compatibility concerns (Node vs Bun APIs, Windows path resolution/CRLF issues) and repository-specific guidelines.
  4. STRICT READ-ONLY CONSTRAINT: You are reading the user's live working tree, which is shared with other agents. You must NEVER modify, create, or delete any file, and never run a command that writes to the repository — in particular never run a formatter, a hook runner, or any test that generates artefacts. Never check out, commit to, or merge any branch. Read and report only.
  5. [Adversarial Mode Only] Double-Check/Adversarial Audit: The ticket content includes a `## Review` section from a previous review. Critically assess if those findings are correct and relevant. If any previous findings are incorrect or no longer apply, note that explicitly in your review. If new findings or gaps are discovered, list them.

  Formulate a detailed review of this ticket. If it is accurate and ready, state that. Otherwise, list the critical findings or gaps as bullet points.
  ```

### 3. Save Updates & Cleanup (Hub Only)

When a subagent completes:

1. The Hub receives the subagent's review markdown.
2. The Hub overwrites/replaces (or appends if missing) the `## Review` section at the bottom of the local ticket file in `.tars/issues/todo/` with the new findings, following this exact formatting:

   ```markdown
   ## Review

   - Finding A
   - Finding B
   ```

3. Save the file to disk (do **NOT** stage or commit).
4. **CRITICAL CLEANLINESS ASSERTION**: Triage agents read the parent working tree in place, so the Hub must confirm they left it untouched — run this after every batch, whether the agents succeeded, failed, or timed out:

   ```bash
   git status --porcelain
   ```

   If anything changed, an agent violated its read-only constraint. Restore the tree and warn the user:

   ```bash
   git reset --hard && git clean -fd
   ```

   This is safe here because spoke workspaces live outside the repository tree entirely (see `TARS_SPOKE_ROOT` in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md)), so no in-flight work can be caught by it. There are no worktrees or branches to tear down.

Repeat for subsequent batches until all tickets in `.tars/issues/todo/` have been triaged/double-checked and contain an up-to-date `## Review` section.

