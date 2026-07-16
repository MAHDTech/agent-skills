---
name: backlog-loop
description: Coordinate the full backlog lifecycle by sequentially executing backlog-audit, backlog-triage, and backlog-implement to resolve all issues. Reach for this when asked to run a full backlog loop, converge on a complete project goal, or manage the overall ticket pipeline.
---

# Backlog Loop

Coordinate the full lifecycle of codebase issues by orchestrating the audit, triage, and implementation loops in sequence.

This is a meta-skill that chains `backlog-audit`, `backlog-triage`, and `backlog-implement` to systematically find, prepare, and execute development tasks until the backlog converges.

## Workflow

To run a full backlog loop, execute the following steps in sequence. Only run one backlog sub-loop at a time:

### Step 0. Preparation Phase (`backlog-prepare`)

1. Call `backlog-prepare` to ensure the repository is clean and ready.
2. Wait for the preparation phase to run to completion.

### Step 1. Audit Phase (`backlog-audit`)

1. Call `backlog-audit` to perform a comprehensive codebase audit.
2. The sub-agents will audit logical modules in parallel, and the Hub will synthesize their reports into structured ticket files saved to `.tars/issues/todo/` (following the guidelines in [backlog-create-issue](../backlog-create-issue/SKILL.md)).
3. Wait for the audit phase to run to completion.

### Step 2. Triage Phase (`backlog-triage`)

1. Once the audit phase completes, call `backlog-triage` to verify the backlog.
2. Sub-agents will check the tickets in parallel batches to ensure accuracy, verify file and line coordinates, eliminate hallucinations, check platform constraints, and append a detailed review section to each ticket.
3. Wait for the triage phase to run to completion.

### Step 3. Implementation & Review Phase (`backlog-implement` & `backlog-review`)

1. Once the triage phase completes, call `backlog-implement` to execute the tickets.
2. The Hub will dynamically group tickets into conflict-free batches, update their frontmatter `batch` number, and dispatch them to parallel sub-agents for implementation.
3. Once implementation completes, the Hub will call `backlog-review` (see [backlog-review](../../review/backlog-review/SKILL.md)) on each ticket branch to run a double-axis verification.
4. For approved tickets, the Hub will merge the completed branches back sequentially, run pre-commit checks (using the [prek](../../tooling/prek/SKILL.md) tool) and tests, and move the ticket files to `.tars/issues/done/`.
5. For rejected tickets, the Hub will update their status to `rework`, append the review comments, and return them to the todo queue while preserving the implementation branch for the next attempt.
6. Wait for the implementation and review phase to run to completion.

## Convergence

- If any tickets fail implementation (exceeding 5 attempts), they will reside in `.tars/issues/failed/`.
- The loop continues until all tickets in `.tars/issues/todo/` are resolved (moved to `done/` or `failed/`), and the audit phase reports no further issues.
