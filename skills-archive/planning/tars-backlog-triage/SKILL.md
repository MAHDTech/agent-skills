---
name: tars-backlog-triage
description: Triage pending backlog issues in `.tars/issues/todo/` to verify their accuracy, check for hallucinations, and add review notes. Reach for this when requested to triage tickets, verify backlog accuracy, or prepare issues for implementation.
disable-model-invocation: true
metadata:
  archived: "2026-09-04"
  replaced-by: "tars-run-factory"
---

# Backlog Triage

Triage pending issue tickets in `.tars/issues/todo/` (conforming to the template in [tars-backlog-create-issue](../tars-backlog-create-issue/SKILL.md)) to verify their accuracy, identify implementation gaps, check for hallucinations, **repair the defects found**, and append a detailed review section to each.

This skill runs in a Hub-and-Spoke topology using sub-agents to verify tickets in parallel.

**Division of labour:** the spokes are strictly read-only and report repairs; the **Hub applies them**. Triage that only annotates is triage that has not happened - see step 3.

## Targets and Paths

- Target Directory: `.tars/issues/todo/` relative to project root.
- Ticket files are updated on disk only, never staged, committed, or force-added to git (since `.tars/` is gitignored).

## Topic Branch Workflow (Hub Only)

All backlog operations must run from a topic branch, never the default branch. Triage subagents are read-only and take no branch of their own - they read the topic branch's working tree as the Hub has it checked out. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](../tars-backlog-prepare/SKILL.md) for the full policy and commands.

## Triage Workflow

### 1. Mode Detection & Backlog Scan

Determine the triage mode based on the user's instructions/invocation:

- **Normal Triage**: Triggered by default. To reduce token spend, identify:
  1. Tickets missing a `## Review` section, with `status: todo` or no status at all; **and**
  2. Tickets whose existing `## Review` section is stamped `**Triage verdict:** UNRESOLVED` (see step 3).

  Skip a ticket whose `## Review` is stamped `**Triage verdict:** RESOLVED`, and skip `status: rework` tickets. Anything else with a `## Review` but **no verdict stamp** predates this rule - treat it as UNRESOLVED and re-examine it.

  > The cheap skip is only safe because the Hub now repairs findings rather than merely recording them. Before that, a ticket could carry a correct finding about a wrong value forever, because the skip saw "has a Review" and moved on. Never widen the skip back to "has a `## Review` section".

- **Adversarial Triage (Double-Check)**: Triggered if the user request contains keywords like `adversarial`, `double-check`, `re-triage`, or `force` (e.g., `/tars-backlog-triage perform an adversarial triage of the backlog`). In this mode, identify **all** tickets in `.tars/issues/todo/` (including those with existing `## Review` sections or `status: rework`) to be triaged/double-checked.

Group the identified tickets into parallel batches of at most 5 concurrent subagents. Overlapping files are fine here - triage agents only read, so they cannot conflict with each other. The batch limit bounds concurrent token and CPU spend, nothing more.

### 2. Spawn Triage Spokes

For each ticket to verify in the batch, spawn a read-only research subagent that reads the **parent working tree directly** - no clone, no worktree, no workspace setup or teardown.

Triage agents verify claims against the codebase and report back by message; they never commit. An isolated workspace would cost real setup on every batch and buy nothing, since nobody is modifying the tree they are reading.

The Hub passes the ticket content directly in the subagent's prompt, so no ticket file needs copying anywhere.

The safety this gives up is enforced by checking rather than by construction - see the cleanliness assertion in step 3.

Equip each subagent with:

- **Role**: `Triage-<TICKET_ID>` (substitute the 3-digit ticket ID, e.g. `Triage-044`)
- **Prompt**:
  Adjust the prompt depending on the triage mode. If in **Adversarial Triage** mode, append the final instruction (the Double-Check/Adversarial Audit item); omit it otherwise:

  ```text
  You are auditing a pending issue ticket to verify its readiness and accuracy against the codebase.

  Ticket Content:
  <TICKET_CONTENT>

  Instructions:
  1. Source Code Verification: Locate the directories, files, and **symbols** (functions, methods, constants, types, exported names) the ticket names. Read those parts of the codebase and verify that the cited symbol exists and that the claim made about it still holds today.
     - Verify by symbol, not by coordinate. If the ticket carries a line number and it has drifted, that alone is NOT a defect worth reporting: find the symbol and check the claim. Report a stale line number only as a cleanup suggestion, and report the *claim* as wrong only when the code itself contradicts it.
     - If the ticket cites a line number but names no symbol, report that as a real gap: the reference will be worthless by the time the ticket is implemented.
  2. Implementation Readiness Check:
     - Does the ticket contain sufficient detail for a fresh agent with a small context window to implement the task?
     - Are the YAML frontmatter, tasks, and acceptance criteria formatted according to the guidelines in [tars-backlog-create-issue](../tars-backlog-create-issue/SKILL.md)? In particular check the shapes that fail silently rather than loudly: `dependencies:` must be an inline array on ONE line (`[12, 19]` or `[]`) - a multi-line list parses as empty and the edges vanish; `files:`/`owns:` must be block lists indented exactly two spaces; `batch:` must be a bare integer or `null`, never quoted; `id:` must be a bare integer.
     - Is `status:` one of `todo`, `rework`, `done`, `failed`, `wont-do`, `in-review`?
     - Do any `dependencies:` point at a ticket now in `wont-do/` or `failed/`? That is an error, not a satisfied edge - the ticket can never be scheduled and must be re-pointed or retired.
     - Are there any gaps? (e.g., missing package configurations, unmentioned side effects, compile-time type errors, build script modifications).
     - Are there any hallucinations? (e.g., non-existent files, non-existent symbols, deprecated APIs, incorrect function signatures).
     - Does the ticket prose name a batch number ("this is batch 4", "#660 is batch 3")? Report it. `batch:` is reallocated every run and reset to `null` on rework, so a batch number in prose is stale almost immediately; the relationship belongs in `dependencies:` instead. The one permitted exception is the `` `batch: N` - rationale `` bullet inside `## Review`.
     - Is the `files:` frontmatter present, non-empty, and complete? It is a primary input to the batching rules stated canonically in [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md) (see its **Conflict-Free Batching** section - do not restate those rules, cite them), so an omission is not cosmetic: the ticket either gets serialised or gets batched into a collision. Check the ticket body for files the work will clearly touch that the list omits - tests, fixtures, barrel re-exports, lockfiles, shared implementation modules - and report them.
     - Should `owns:` list a shared module or `path#symbol` (e.g. a constant two tickets might both introduce)? If the body clearly claims ownership of an export or shared helper, require or suggest `owns:` entries so implement can serialise collisions the file rules would miss. Note that a bare path and a `path#Symbol` on that same path DO collide.
     - Soft dependencies: if this ticket needs an export or behaviour another open ticket will add, require a `dependencies:` edge (or report the gap) even when `files:` do not overlap. Give the edge as an inline array.
     - Are `risk: high` / `complexity: high` set when appropriate (hooks, auth, secrets, huge multi-file work)?
  3. Assess Constraints: Check for platform compatibility concerns and repository-specific guidelines.
  4. Binding sections: if the ticket carries a section marked BINDING (e.g. `## Rework directives`), read it closely. That section - NOT the `## Review` commentary above it - is what the implementing agent will execute. For every finding you report, say whether the defect is inside a binding section, and quote the exact offending line. When a value inside a binding section is stale, also state whether the CONCLUSION drawn from it still holds: an ordering claim, a rebase direction, or a "must not co-batch" constraint can reverse entirely once the number is corrected. Say so explicitly either way.
  5. STRICT READ-ONLY CONSTRAINT: You are reading the user's live working tree, which is shared with other agents. You must NEVER modify, create, or delete any file, and never run a command that writes to the repository - in particular never run a formatter, a hook runner, or any test that generates artefacts. Never check out, commit to, or merge any branch. Read and report only. You report the repair; the Hub applies it.
  6. [Adversarial Mode Only] Double-Check/Adversarial Audit: The ticket content includes a `## Review` section from a previous review. Critically assess if those findings are correct and relevant. If any previous findings are incorrect or no longer apply, note that explicitly in your review. If new findings or gaps are discovered, list them.

  Formulate a detailed review of this ticket. If it is accurate and ready, state that.

  Otherwise, for each finding give the Hub enough to APPLY a fix without re-reading the codebase:
  - the exact text of the offending line, verbatim;
  - which section it sits in, and whether that section is BINDING;
  - the exact replacement text, or - if the fix needs a human decision - the word UNRESOLVED
    and what decision is needed.

  A finding phrased only as an observation ("the line number here is stale") is not actionable.
  Phrase it as a repair ("line `see bash-policy.ts:3071` in `## Rework directives` should read
  `see verifyBashPermission in bash-policy.ts`").
  ```

### 3. Save Updates & Cleanup (Hub Only)

When a subagent completes:

1. The Hub receives the subagent's review markdown.

2. **The Hub repairs the ticket, it does not merely annotate it.**

   > **Annotating a defect is not repairing it.** An implementing agent reads the ticket's binding instructions, not the commentary appended below them. A correct note in `## Review` sitting above a wrong value in `## Rework directives` changes nothing: the wrong value is still what gets built. This is not a hypothetical - four tickets in one run each carried an accurate note identifying a stale value, left in place above the stale value it described, and all four were implemented from the stale value. One note literally read "fix the direction or delete the note", and neither happened.

   For each finding the spoke reports:
   - **Edit the offending line in place**, in the section the defect actually lives in. If the defect is in `## Description`, fix `## Description`. If it is in a binding section, fix the binding section.
   - **Precedence:** where `## Review` and a section marked BINDING (e.g. `## Rework directives`) disagree, **the binding section is what gets executed**, so that is what must be corrected. Never resolve the disagreement by writing a clarification into `## Review` and leaving the binding text alone.
   - **When you correct a value, re-check the conclusion drawn from it.** A stale number frequently conceals a _reversed_ conclusion, not merely an imprecise one: "rebase on #663 if it lands first" is wrong in a new way once you discover this ticket lands first. Ordering claims, rebase directions, and "must not co-batch" constraints all invert when the underlying fact moves. Correcting the number and leaving the sentence is the same failure one level up.
   - **Record the repair as provenance** in `## Review`, as a one-line note saying what was changed and why - not as a request for someone else to change it.
   - **Where the Hub cannot repair the finding itself** - it needs a human decision, a product call, or information not in the repository - leave the text alone and mark that finding **UNRESOLVED**, stating what decision is needed.

   Frontmatter defects are repaired the same way: convert a multi-line `dependencies:` list to the inline form, unquote a quoted `batch:`, fix a two-space indent. These are silent-failure shapes, so leaving them "reported" leaves them broken.

3. The Hub overwrites/replaces (or appends if missing) the `## Review` section at the bottom of the local ticket file in `.tars/issues/todo/` with the new findings, following this exact formatting:

   ```markdown
   ## Review

   **Triage verdict:** RESOLVED

   - Finding A - repaired: `## Rework directives` line "see bash-policy.ts:3071" now reads "see `verifyBashPermission` in bash-policy.ts".
   - Finding B - no change needed; the cited symbol exists and the claim holds.
   - `batch: 4` - shares no path with the rest of the batch; the shared `PolicyDenial` type it adds is owned by #41, which lands earlier.
   ```

   The verdict stamp is mandatory and must be exactly one of:

   | Stamp                            | Meaning                                                                               |
   | -------------------------------- | ------------------------------------------------------------------------------------- |
   | `**Triage verdict:** RESOLVED`   | Every finding is either repaired in place or confirmed to need no change.             |
   | `**Triage verdict:** UNRESOLVED` | At least one finding needs a decision the Hub could not make. Mark each such finding. |

   A later triage run skips RESOLVED tickets and re-opens UNRESOLVED ones, so an omitted or vague stamp either buries an open question forever or pays to re-review a clean ticket. Once repairs happen in place, most tickets resolve on the first pass and the re-open cost stays small.

   Keep the `` `batch: N` - rationale `` bullet when the ticket has a batch allocated: it is a bare restatement of frontmatter, so it can be corrected mechanically, and its rationale is where conceptual overlaps the mechanical batching rules cannot see get recorded. Do **not** write batch numbers anywhere else in the ticket.

4. Save the file to disk (do **NOT** stage or commit).
5. **CRITICAL CLEANLINESS ASSERTION**: Triage agents read the parent working tree in place, so the Hub must confirm they left it untouched - run this after every batch, whether the agents succeeded, failed, or timed out:

   ```bash
   git status --porcelain
   ```

   If anything changed, an agent violated its read-only constraint. Restore the tree and warn the user:

   ```bash
   git reset --hard && git clean -fd
   ```

   This is safe here because spoke workspaces live outside the repository tree entirely (see `TARS_SPOKE_ROOT` in [tars-backlog-prepare](../tars-backlog-prepare/SKILL.md)), so no in-flight work can be caught by it. There are no worktrees or branches to tear down.

Repeat for subsequent batches until all tickets in `.tars/issues/todo/` have been triaged/double-checked and contain an up-to-date `## Review` section carrying a verdict stamp.

Report at the end how many tickets were repaired in place and which are left **UNRESOLVED**, listing the decision each is waiting on. An UNRESOLVED ticket is not ready to implement; it is waiting on a human.
