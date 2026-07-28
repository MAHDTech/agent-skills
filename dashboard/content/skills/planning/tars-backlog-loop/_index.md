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

### Invoking the Phase Skills

These skills are marked **user-invoked** — in Claude Code that is `disable-model-invocation: true`; other runtimes spell it differently. Wherever that marking is honoured, the effect is the same: only the user typing the skill's name can invoke it, and **no skill can invoke another**. So a "call `tars-backlog-<phase>`" instruction will simply be refused.

**When it is refused, read that skill's `SKILL.md` and execute its steps inline.** Each call site gives the path. If your runtime does permit skill-to-skill invocation, calling it directly is equivalent and fine.

Keeping the marking costs nothing at rest; removing it would load all seven descriptions into every session's context permanently, for skills that are only ever driven deliberately. Treat a refusal as something to route around, never as a reason to skip the step.

### Topic Branch Workflow (Hub Only)

The Hub must run every audit, triage, implementation, and review step from a topic branch (never the default branch). Implementation spokes branch off — and merge back into — that active topic branch; audit and triage subagents are read-only and take no branch of their own. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) for the full policy and commands.

### Isolation & Concurrency Model

Two rules hold across every phase below. Both are established in full by [tars-backlog-implement](@/skills/engineering/tars-backlog-implement/_index.md):

1. **Writers get private clones; readers get none.** Implementation spokes work in isolated clones outside the repository tree, so no spoke can write to the parent's shared git state. Audit and triage subagents only read, so they use the parent working tree directly and the Hub asserts it is unchanged afterwards.
2. **Heavy commands are serialised.** Test suites, whole-repo hook runs, and nix/container builds produce false failures under CPU contention, so every agent — the Hub included — wraps them in the shared mutex. Cheap deterministic checks stay fully parallel.

### Step 0. Preparation Phase (`tars-backlog-prepare`)

1. Execute [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) inline to verify repository integrity, resolve the isolated spoke workspace root, and clean up leftovers from previous runs.
2. Wait for the preparation phase to run to completion. It records the resolved workspace, clone mode, and lock paths to `.tars/run.env`, which every later phase reads. **Do not skip this step**: without it the implementation phase has no spoke root to clone into, and any corruption left in the shared git state goes undetected.
3. Check the `.tars/issues/todo/` directory for any existing ticket files (`XXX.md` files where `XXX` is a 3-digit ID).
   - **If existing issues are present**: Skip directly to **Step 2. Triage Phase** to triage them, then proceed to **Step 3. Implementation & Review Phase** to resolve them. Once all existing issues are implemented or resolved, proceed to **Step 1. Audit Phase** to scan the updated codebase for any new issues.
   - **If no existing issues are present**: Proceed directly to **Step 1. Audit Phase**.

### Step 1. Audit Phase (`tars-backlog-audit`)

1. Execute [tars-backlog-audit](@/skills/planning/tars-backlog-audit/_index.md) inline to perform a comprehensive codebase audit.
2. The sub-agents will audit logical modules in parallel, and the Hub will synthesize their reports into structured ticket files saved to `.tars/issues/todo/` (following the guidelines in [tars-backlog-create-issue](@/skills/planning/tars-backlog-create-issue/_index.md)).
3. Wait for the audit phase to run to completion.

### Step 2. Triage Phase (`tars-backlog-triage`)

1. Execute [tars-backlog-triage](@/skills/planning/tars-backlog-triage/_index.md) inline to verify the backlog.
2. Sub-agents will check the tickets in parallel batches to ensure accuracy, verify file and line coordinates, eliminate hallucinations, check platform constraints, and append a detailed review section to each ticket.
3. Wait for the triage phase to run to completion.

### Step 3. Implementation & Review Phase (`tars-backlog-implement` & `tars-backlog-review`)

1. Execute [tars-backlog-implement](@/skills/engineering/tars-backlog-implement/_index.md) inline (it lives in the engineering category) to execute the tickets.
2. The Hub will dynamically group tickets into batches that are conflict-free by file **and** free of dependency edges onto unmerged tickets, update their frontmatter `batch` number, and dispatch them to parallel spokes, each in its own isolated clone.
3. As each spoke reports, the Hub fetches its branch into the parent for durability, then runs the full verification gate — the repository's whole-repo hook run plus its test suite, both resolved once by `tars-backlog-prepare` — **inside the spoke's clone, under the mutex, before merging anything**. A red gate goes straight back to the live spoke to fix, for up to 3 rounds.
4. On a green gate, the Hub executes [tars-backlog-review](@/skills/review/tars-backlog-review/_index.md) inline for the double-axis verdict. Approved branches merge sequentially into the topic branch and their tickets move to `.tars/issues/done/`; either way the spoke is told its ticket is resolved so it can stop.
5. For rejected tickets, the Hub will update their status to `rework`, append the review comments, and return them to the todo queue while preserving the implementation branch for the next attempt.
6. After the whole batch has merged, the Hub runs the full gate once more on the topic branch, catching interactions between tickets that were conflict-free by file but not by behaviour.
7. Wait for the implementation and review phase to run to completion.

## Convergence

- If any tickets fail implementation (exceeding 5 attempts), they will reside in `.tars/issues/failed/`.
- The loop continues until all tickets in `.tars/issues/todo/` are resolved (moved to `done/` or `failed/`), and the audit phase reports no further issues.

**Guard against a non-terminating loop.** The convergence condition is an empty `todo/`, so any ticket that can never be scheduled would spin forever. Two cases do this, and both are swept in step 6 of `tars-backlog-implement`'s batching: a ticket whose `dependencies` name a ticket now in `failed/`, and a dependency cycle where no member can ever go first. If a full pass over `todo/` schedules nothing and resolves nothing, stop and report rather than looping — that is the signature of a blocked backlog, not a slow one.

