+++
title = "tars-backlog-loop"
description = "Coordinate the full backlog lifecycle by sequentially executing tars-backlog-audit, tars-backlog-triage, and tars-backlog-implement to resolve all issues. Reach for this when asked to run a full backlog loop, converge on a complete project goal, or manage the overall ticket pipeline."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Loop

Coordinate the full lifecycle of codebase issues by orchestrating the audit, triage, and implementation loops in sequence.

This is a meta-skill that chains `tars-backlog-audit`, `tars-backlog-triage`, and `tars-backlog-implement` to systematically find, prepare, and execute development tasks until the backlog converges.

## Workflow

To run a full backlog loop, execute the following steps in sequence. Only run one backlog sub-loop at a time:

### Topic Branch Workflow (Hub Only)

The Hub must run every audit, triage, implementation, and review step from a topic branch (never the default branch), and all spawned subagents branch off — and merge back into — that active topic branch. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) for the full policy and commands.

### Step 0. Preparation Phase (`tars-backlog-prepare`)

1. Call `tars-backlog-prepare` to ensure the repository is clean and ready.
2. Wait for the preparation phase to run to completion.
3. Check the `.tars/issues/todo/` directory for any existing ticket files (`XXX.md` files where `XXX` is a 3-digit ID).
   - **If existing issues are present**: Skip directly to **Step 2. Triage Phase** to triage them, then proceed to **Step 3. Implementation & Review Phase** to resolve them. Once all existing issues are implemented or resolved, proceed to **Step 1. Audit Phase** to scan the updated codebase for any new issues.
   - **If no existing issues are present**: Proceed directly to **Step 1. Audit Phase**.

### Step 1. Audit Phase (`tars-backlog-audit`)

1. Call `tars-backlog-audit` to perform a comprehensive codebase audit.
2. The sub-agents will audit logical modules in parallel, and the Hub will synthesize their reports into structured ticket files saved to `.tars/issues/todo/` (following the guidelines in [tars-backlog-create-issue](@/skills/planning/tars-backlog-create-issue/_index.md)).
3. Wait for the audit phase to run to completion.

### Step 2. Triage Phase (`tars-backlog-triage`)

1. Call `tars-backlog-triage` to verify the backlog.
2. Sub-agents will check the tickets in parallel batches to ensure accuracy, verify file and line coordinates, eliminate hallucinations, check platform constraints, and append a detailed review section to each ticket.
3. Wait for the triage phase to run to completion.

### Step 3. Implementation & Review Phase (`tars-backlog-implement` & `tars-backlog-review`)

1. Call `tars-backlog-implement` (see [tars-backlog-implement](@/skills/engineering/tars-backlog-implement/_index.md), in the engineering category) to execute the tickets.
2. The Hub will dynamically group tickets into conflict-free batches, update their frontmatter `batch` number, and dispatch them to parallel sub-agents for implementation.
3. Once implementation completes, the Hub will call `tars-backlog-review` (see [tars-backlog-review](@/skills/review/tars-backlog-review/_index.md)) on each ticket branch to run a double-axis verification.
4. For approved tickets, the Hub will merge the completed branches back sequentially into the active topic branch, run pre-commit checks (using the [prek](@/skills/tooling/prek/_index.md) tool) and tests, and move the ticket files to `.tars/issues/done/`.
5. For rejected tickets, the Hub will update their status to `rework`, append the review comments, and return them to the todo queue while preserving the implementation branch for the next attempt.
6. Wait for the implementation and review phase to run to completion.

## Convergence

- If any tickets fail implementation (exceeding 5 attempts), they will reside in `.tars/issues/failed/`.
- The loop continues until all tickets in `.tars/issues/todo/` are resolved (moved to `done/` or `failed/`), and the audit phase reports no further issues.

