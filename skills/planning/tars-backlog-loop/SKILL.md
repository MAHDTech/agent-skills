---
name: tars-backlog-loop
description: Coordinate the full backlog lifecycle by sequentially executing tars-backlog-audit, tars-backlog-triage, and tars-backlog-implement to resolve all issues. Reach for this when asked to run a full backlog loop, converge on a complete project goal, or manage the overall ticket pipeline.
disable-model-invocation: true
---

# Backlog Loop

Coordinate the full lifecycle of codebase issues by orchestrating the audit, triage, and implementation loops in sequence.

This is a meta-skill that chains `tars-backlog-audit`, `tars-backlog-triage`, and `tars-backlog-implement` to systematically find, prepare, and execute development tasks until the backlog converges.

## Workflow

To run a full backlog loop, execute the following steps in sequence. Only run one backlog sub-loop at a time:

### Topic Branch Workflow (Hub Only)

The Hub must run every audit, triage, implementation, and review step from a topic branch (never the default branch). Implementation spokes branch off — and merge back into — that active topic branch; audit and triage subagents are read-only and take no branch of their own. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](../tars-backlog-prepare/SKILL.md) for the full policy and commands.

### Isolation & Concurrency Model

Two rules hold across every phase below. Both are established in full by [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md):

1. **Writers get private clones; readers get none.** Implementation spokes work in isolated clones outside the repository tree, so no spoke can write to the parent's shared git state. Audit and triage subagents only read, so they use the parent working tree directly and the Hub asserts it is unchanged afterwards.
2. **Heavy commands are serialised.** Test suites, `devenv test`, and `prek run -a` produce false failures under CPU contention, so every agent — the Hub included — wraps them in the shared mutex. Cheap deterministic checks stay fully parallel.

### Step 0. Preparation Phase (`tars-backlog-prepare`)

1. Call `tars-backlog-prepare` to verify repository integrity, resolve the isolated spoke workspace root, and clean up leftovers from previous runs.
2. Wait for the preparation phase to run to completion. It records the resolved workspace, clone mode, and lock paths to `.tars/run.env`, which every later phase reads. **Do not skip this step**: without it the implementation phase has no spoke root to clone into, and any corruption left in the shared git state goes undetected.
3. Check the `.tars/issues/todo/` directory for any existing ticket files (`XXX.md` files where `XXX` is a 3-digit ID).
   - **If existing issues are present**: Skip directly to **Step 2. Triage Phase** to triage them, then proceed to **Step 3. Implementation & Review Phase** to resolve them. Once all existing issues are implemented or resolved, proceed to **Step 1. Audit Phase** to scan the updated codebase for any new issues.
   - **If no existing issues are present**: Proceed directly to **Step 1. Audit Phase**.

### Step 1. Audit Phase (`tars-backlog-audit`)

1. Call `tars-backlog-audit` to perform a comprehensive codebase audit.
2. The sub-agents will audit logical modules in parallel, and the Hub will synthesize their reports into structured ticket files saved to `.tars/issues/todo/` (following the guidelines in [tars-backlog-create-issue](../tars-backlog-create-issue/SKILL.md)).
3. Wait for the audit phase to run to completion.

### Step 2. Triage Phase (`tars-backlog-triage`)

1. Call `tars-backlog-triage` to verify the backlog.
2. Sub-agents will check the tickets in parallel batches to ensure accuracy, verify file and line coordinates, eliminate hallucinations, check platform constraints, and append a detailed review section to each ticket.
3. Wait for the triage phase to run to completion.

### Step 3. Implementation & Review Phase (`tars-backlog-implement` & `tars-backlog-review`)

1. Call `tars-backlog-implement` (see [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md), in the engineering category) to execute the tickets.
2. The Hub will dynamically group tickets into conflict-free batches, update their frontmatter `batch` number, and dispatch them to parallel spokes, each in its own isolated clone.
3. As each spoke reports, the Hub fetches its branch into the parent for durability, then runs the full verification gate — `prek run -a` plus the test suite (using the [prek](../../tooling/prek/SKILL.md) tool) — **inside the spoke's clone, under the mutex, before merging anything**. A red gate goes straight back to the live spoke to fix, for up to 3 rounds.
4. On a green gate, the Hub calls `tars-backlog-review` (see [tars-backlog-review](../../review/tars-backlog-review/SKILL.md)) for the double-axis verdict. Approved branches merge sequentially into the topic branch and their tickets move to `.tars/issues/done/`.
5. For rejected tickets, the Hub will update their status to `rework`, append the review comments, and return them to the todo queue while preserving the implementation branch for the next attempt.
6. After the whole batch has merged, the Hub runs the full gate once more on the topic branch, catching interactions between tickets that were conflict-free by file but not by behaviour.
7. Wait for the implementation and review phase to run to completion.

## Convergence

- If any tickets fail implementation (exceeding 5 attempts), they will reside in `.tars/issues/failed/`.
- The loop continues until all tickets in `.tars/issues/todo/` are resolved (moved to `done/` or `failed/`), and the audit phase reports no further issues.
