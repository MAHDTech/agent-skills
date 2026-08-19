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

### Invoking the Phase Skills

These skills are marked **user-invoked** - in Claude Code that is `disable-model-invocation: true`; other runtimes spell it differently. Wherever that marking is honoured, the effect is the same: only the user typing the skill's name can invoke it, and **no skill can invoke another**. So a "call `tars-backlog-<phase>`" instruction will simply be refused.

**When it is refused, read that skill's `SKILL.md` and execute its steps inline.** Each call site gives the path. If your runtime does permit skill-to-skill invocation, calling it directly is equivalent and fine.

Keeping the marking costs nothing at rest; removing it would load all seven descriptions into every session's context permanently, for skills that are only ever driven deliberately. Treat a refusal as something to route around, never as a reason to skip the step.

### Topic Branch Workflow (Hub Only)

The Hub must run every audit, triage, implementation, and review step from a topic branch (never the default branch). Implementation spokes branch off - and merge back into - that active topic branch; audit and triage subagents are read-only and take no branch of their own. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](../tars-backlog-prepare/SKILL.md) for the full policy and commands.

### Isolation & Concurrency Model

Two rules hold across every phase below. Both are established in full by [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md):

1. **Writers get private clones; readers get none.** Implementation spokes work in isolated clones outside the repository tree, so no spoke can write to the parent's shared git state. Audit and triage subagents only read, so they use the parent working tree directly and the Hub asserts it is unchanged afterwards.
2. **Heavy commands are serialised.** Test suites, whole-repo hook runs, and nix/container builds produce false failures under CPU contention. The Hub runs the full gate only via `tars-gate`; spokes run targeted heavy tests only via `tars-spoke`. Cheap deterministic checks stay fully parallel.

### Step 0. Preparation Phase (`tars-backlog-prepare`)

1. Execute [tars-backlog-prepare](../tars-backlog-prepare/SKILL.md) inline to verify repository integrity, resolve the isolated spoke workspace root, freeze opaque install/hooks/test commands (following the [devenv](../../tooling/devenv/SKILL.md) skill when the project uses devenv), smoke the baseline gate, and clean up leftovers from previous runs.
2. Wait for the preparation phase to run to completion. It records workspace, clone mode, lock/gate/spoke paths, land template, CI flags, and weaken banners to `.tars/run.env`, which every later phase reads. **Do not skip this step**: without it the implementation phase has no spoke root, no proven gate recipe, and any corruption left in the shared git state goes undetected. If prepare reports `TARS_GATE_WEAKENED=1`, carry that banner through the whole loop.
3. Check the `.tars/issues/todo/` directory for any existing ticket files (`XXX.md` files where `XXX` is a 3-digit ID).
   - **If existing issues are present**: Skip directly to **Step 2. Triage Phase** to triage them, then proceed to **Step 3. Implementation & Review Phase** to resolve them. Once all existing issues are implemented or resolved, proceed to **Step 1. Audit Phase** to scan the updated codebase for any new issues.
   - **If no existing issues are present**: Proceed directly to **Step 1. Audit Phase**.

### Step 1. Audit Phase (`tars-backlog-audit`)

1. Execute [tars-backlog-audit](../tars-backlog-audit/SKILL.md) inline to perform a comprehensive codebase audit.
2. The sub-agents will audit logical modules in parallel, and the Hub will synthesize their reports into structured ticket files saved to `.tars/issues/todo/` (following the guidelines in [tars-backlog-create-issue](../tars-backlog-create-issue/SKILL.md)).
3. Wait for the audit phase to run to completion.

### Step 2. Triage Phase (`tars-backlog-triage`)

1. Execute [tars-backlog-triage](../tars-backlog-triage/SKILL.md) inline to verify the backlog.
2. Sub-agents will check the tickets in parallel batches to ensure accuracy, verify file and line coordinates, eliminate hallucinations, check platform constraints, and append a detailed review section to each ticket.
3. Wait for the triage phase to run to completion.

### Step 3. Implementation & Review Phase (`tars-backlog-implement` & `tars-backlog-review`)

1. Execute [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md) inline (it lives in the engineering category) to execute the tickets.
2. The Hub groups tickets into batches that are conflict-free by `files:` **and** `owns:`, free of hard dependency edges onto unmerged tickets, and free of soft-ownership collisions, then dispatches parallel spokes in isolated clones (minimal spoke contract; checkpoint protocol when `complexity: high` or rework).
3. As each spoke reports, the Hub force-updates topic refs, fetches the spoke branch into the parent for durability, runs **`tars-gate`** inside the clone (flake classify → isolate → one re-gate when appropriate), commits hook autofixes if the gate dirtied the tree, then reviews.
4. On a green clean tree, the Hub uses a **risk-tiered** review: lightweight checklist by default; full [tars-backlog-review](../../review/tars-backlog-review/SKILL.md) when high-risk / post-conflict / rework. Approved branches merge sequentially with the prepare-frozen land commit subject; tickets move to `.tars/issues/done/`. Every terminal path dismisses the spoke.
5. For rejected tickets, the Hub updates `rework`, appends review comments, and preserves the implementation branch for the next attempt.
6. After the batch merges, the Hub runs **`tars-gate`** again on the topic branch for cross-ticket interactions.
7. If `TARS_CI_CHECK=1` in `run.env`, the Hub confirms CI on the batch head before the next batch (blocks on red when `TARS_CI_BLOCK_ON_RED=1`). Local green is not CI green.
8. Wait for the implementation and review phase to run to completion. Final reports must banner any weakened gate reason.

## Convergence

- If any tickets fail implementation (exceeding 5 attempts), they will reside in `.tars/issues/failed/`.
- The loop continues until all tickets in `.tars/issues/todo/` are resolved (moved to `done/` or `failed/`), and the audit phase reports no further issues.

**Guard against a non-terminating loop.** The convergence condition is an empty `todo/`, so any ticket that can never be scheduled would spin forever. Two cases do this, and both are swept in step 6 of `tars-backlog-implement`'s batching: a ticket whose `dependencies` name a ticket now in `failed/`, and a dependency cycle where no member can ever go first. If a full pass over `todo/` schedules nothing and resolves nothing, stop and report rather than looping - that is the signature of a blocked backlog, not a slow one.
