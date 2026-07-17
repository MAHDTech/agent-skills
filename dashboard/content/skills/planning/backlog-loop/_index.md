+++
title = "backlog-loop"
description = "Coordinate the full backlog lifecycle by sequentially executing backlog-audit, backlog-triage, and backlog-implement to resolve all issues. Reach for this when asked to run a full backlog loop, converge on a complete project goal, or manage the overall ticket pipeline."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Loop

Coordinate the full lifecycle of codebase issues by orchestrating the audit, triage, and implementation loops in sequence.

This is a meta-skill that chains `backlog-audit`, `backlog-triage`, and `backlog-implement` to systematically find, prepare, and execute development tasks until the backlog converges.

## Workflow

To run a full backlog loop, execute the following steps in sequence. Only run one backlog sub-loop at a time:

> [!IMPORTANT]
> **Fresh Skill Reloading**: To prevent context staleness (where the agent relies on cached, outdated versions of skills from the start of the session), the Hub **MUST** explicitly re-read and reload the relevant `SKILL.md` files (using the `view_file` tool) at the beginning of each step transition (e.g., before starting Step 0, Step 1, Step 2, and Step 3). Do not rely on cached memory of the skills.

### Step 0. Preparation Phase (`backlog-prepare`)

1. Call `backlog-prepare` to ensure the repository is clean and ready.
2. Wait for the preparation phase to run to completion.
3. Check the `.tars/issues/todo/` directory for any existing ticket files (`XXX.md` files where `XXX` is a 3-digit ID).
   - **If existing issues are present**: Skip directly to **Step 2. Triage Phase** to triage them, then proceed to **Step 3. Implementation & Review Phase** to resolve them. Once all existing issues are implemented or resolved, proceed to **Step 1. Audit Phase** to scan the updated codebase for any new issues.
   - **If no existing issues are present**: Proceed directly to **Step 1. Audit Phase**.

### Step 1. Audit Phase (`backlog-audit`)

1. Call `backlog-audit` to perform a comprehensive codebase audit.
2. The sub-agents will audit logical modules in parallel, and the Hub will synthesize their reports into structured ticket files saved to `.tars/issues/todo/` (following the guidelines in [backlog-create-issue](@/skills/planning/backlog-create-issue/_index.md)).
3. Wait for the audit phase to run to completion.

### Step 2. Triage Phase (`backlog-triage`)

1. Call `backlog-triage` to verify the backlog.
2. Sub-agents will check the tickets in parallel batches to ensure accuracy, verify file and line coordinates, eliminate hallucinations, check platform constraints, and append a detailed review section to each ticket.
3. Wait for the triage phase to run to completion.

### Step 3. Implementation & Review Phase (`backlog-implement` & `backlog-review`)

1. Call `backlog-implement` to execute the tickets.
2. The Hub will dynamically group tickets into conflict-free batches, update their frontmatter `batch` number, and dispatch them to parallel sub-agents for implementation.
3. Once implementation completes, the Hub will call `backlog-review` (see [backlog-review](@/skills/review/backlog-review/_index.md)) on each ticket branch to run a double-axis verification.
4. For approved tickets, the Hub will merge the completed branches back sequentially, run pre-commit checks (using the [prek](@/skills/tooling/prek/_index.md) tool) and tests, and move the ticket files to `.tars/issues/done/`.
5. For rejected tickets, the Hub will update their status to `rework`, append the review comments, and return them to the todo queue while preserving the implementation branch for the next attempt.
6. Wait for the implementation and review phase to run to completion.

## Convergence

- If any tickets fail implementation (exceeding 5 attempts), they will reside in `.tars/issues/failed/`.
- The loop continues until all tickets in `.tars/issues/todo/` are resolved (moved to `done/` or `failed/`), and the audit phase reports no further issues.

