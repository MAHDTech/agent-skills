---
name: backlog-implement
description: Implement pending backlog issues from `.tars/issues/todo/` in parallel, conflict-free batches using isolated workspaces. Reach for this when asked to implement backlog issues, execute tasks from tickets in parallel, or resolve the issue queue.
---

# Backlog Implement

Implement pending issue tickets from `.tars/issues/todo/` in parallel, conflict-free batches using isolated workspaces, and verify and merge them back sequentially.

This skill operates in a Hub-and-Spoke topology, spawning implementation subagents in parallel, while the Hub manages sequencing, merges, and testing.

## Targets and Paths

- Target Folders: `.tars/issues/{todo,done,failed}/` relative to project root.
- Ticket status updates are written to disk only. Ticket files are never staged or committed in git.

## Implementation Workflow

### 1. Backlog Scan & Conflict-Free Batching

1. Scan the `.tars/issues/todo/` directory for ticket markdown files.
2. Analyze the `files` or `component` lists of all pending tickets.
3. Dynamically group the tickets into conflict-free batches of at most 5 concurrent tickets. A batch is conflict-free if no two tickets in it modify overlapping files.
4. Update the ticket frontmatter with `batch: X` (starting with `batch: 1`) and write to disk so batches are remembered.
5. Prior to executing a batch, the Hub must verify that all tickets in the current batch are indeed conflict-free.

### 2. Spawn Implementation Spokes

For each ticket in the selected batch, spawn an implementation subagent.

Since `.tars/` is gitignored, the Hub must read the ticket content and pass it directly in the subagent's prompt.

Equip each subagent with:

- **TypeName**: `self` (or `case` if custom configured)
- **Workspace**: `branch`
- **Prompt**:

  ```text
  You are tasked with implementing the changes described in this ticket.

  Ticket Details:
  <TICKET_CONTENT>

  Instructions:
  1. Read the ticket details completely, including the Tasks, Acceptance Criteria, and the '## Review' section.
  2. Implement the changes described.
  3. Verify your implementation by running tests:
     - Detect if 'devenv.nix' or 'devenv/default.nix' is present in the workspace root. If so, run 'devenv test'.
     - Otherwise, check for standard project test configs (e.g., package.json -> 'npm test', cargo.toml -> 'cargo test', pytest, etc.) and execute them.
     - Ensure the test suite passes prior to returning.
  4. Ensure all pre-commit hooks run and pass using `prek` (see the [prek](../../tooling/prek/SKILL.md) skill). Fix any failing checks before committing.
  5. Commit your changes using Conventional Commits.
  6. STRICT ISOLATION CONSTRAINT: You must NEVER check out the source/main branch, commit directly to the source/main branch, or attempt to merge branches. You must only commit changes on your local isolated workspace branch and report completion. The orchestrator Hub is solely responsible for merging branches and cleaning up workspaces.
  7. Document command runs and outputs proving execution in the 'Evidence' section of the ticket file/response.
  ```

### 3. Sequential Merge-Back & Verification (Hub Only)

When all subagents in the batch complete:

1. **Merge Sequentially**: Sequentially merge each subagent's branch back into the main/source branch one at a time. Never perform parallel merges.
2. **Pre-commit Integrity**: For each merge, ensure that all pre-commit hooks run and pass using `prek` (see the [prek](../../tooling/prek/SKILL.md) skill). **NEVER** use `--no-verify` or bypass hooks.
3. **Parent Test Verification**: After each individual merge, run the test suite in the parent workspace to verify stability.
4. **Move Ticket**:
   - If the merge and subsequent tests pass: move the ticket file to `.tars/issues/done/`.
   - If the merge, tests, or `prek` checks fail: abort the merge, restore the main branch state, move the ticket to `.tars/issues/failed/` (or back to `todo/` if it can be retried), and log the failure.
5. **CRITICAL CLEANUP CONSTRAINT**: Immediately after a branch is successfully merged or rejected, the Hub (and ONLY the Hub) MUST clean up its worktree and branch. You must force-remove them regardless of whether the subagent succeeded, failed, or timed out. Failure to do so will break future iterations.
   - Run `git worktree remove --force <path>`
   - Run `git branch -D <branch-name>`

Repeat for subsequent batches until all batches are processed.
