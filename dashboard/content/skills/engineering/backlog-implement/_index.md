+++
title = "backlog-implement"
description = "Implement pending backlog issues from `.tars/issues/todo/` in parallel, conflict-free batches using isolated workspaces. Reach for this when asked to implement backlog issues, execute tasks from tickets in parallel, or resolve the issue queue."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Backlog Implement

Implement pending issue tickets from `.tars/issues/todo/` in parallel, conflict-free batches using isolated workspaces, and verify and merge them back sequentially.

This skill operates in a Hub-and-Spoke topology, spawning implementation subagents in parallel, while the Hub manages sequencing, merges, and testing.

## Targets and Paths

- Target Folders: `.tars/issues/{todo,done,failed}/` relative to project root.
- Ticket status updates are written to disk only. Ticket files are never staged, committed, or force-added in git.

## Implementation Workflow

### 1. Backlog Scan & Conflict-Free Batching

1. Scan the `.tars/issues/todo/` directory for ticket markdown files.
2. Analyze the `files` or `component` lists of all pending tickets.
3. Dynamically group the tickets into conflict-free batches of at most 5 concurrent tickets. A batch is conflict-free if no two tickets in it modify overlapping files.
4. Update the ticket frontmatter with `batch: X` (starting with `batch: 1`) and write to disk so batches are remembered.
5. Prior to executing a batch, the Hub must verify that all tickets in the current batch are indeed conflict-free.

### 2. Spawn Implementation Spokes & Initialize Worktrees

For each ticket in the selected batch, spawn an implementation subagent in an isolated workspace (worktree).

Since gitignored files/directories (like `.tars/` or `.pre-commit-config.yaml`) do not exist in the subagent's new worktree workspace, the Hub must initialize the worktree workspace and transfer the necessary files/directories before the subagent starts execution.

#### Hub-to-Spoke Gitignore Transfer Logic

1. **Locate Worktree Path**: Run `git worktree list` to retrieve the absolute path of the new subagent's worktree directory.
2. **Read Configured Transfer Files**: Check if `.tars/config.yaml` exists and contains a `worktree.transfer_files` key configured.
   - If configured, read the list of transfer patterns.
   - If not configured, use these default patterns:

     ```yaml
     worktree:
       transfer_files:
         - ".pre-commit-config.yaml"
         - ".env*"
         - ".tars/"
         - "devenv.local.nix"
         - "devenv.local.yaml"
     ```

3. **Synchronize/Symlink Files**: For each matching file or directory in the parent workspace root:
   - **Symlink Attempt (Preferred)**: Try to create a symlink in the subagent's worktree pointing to the corresponding item in the parent workspace root. (e.g., symlink `<subagent-worktree>/.tars/` to `<parent-workspace>/.tars/`).
     > [!IMPORTANT]
     > Symlinking `.tars/` allows the subagent to read and write directly to the shared issue queue, meaning you do not need to manually copy ticket updates back when the subagent finishes.
   - **Copy Fallback**: If symlinking fails (e.g., on Windows without Developer Mode or due to a permissions error), fallback to copying the file or directory physically into the subagent's worktree root.
     - Note: If `.tars/` is copied as a fallback, the Hub **MUST** copy the modified ticket file (`.tars/issues/todo/XXX.md`) back from the subagent's worktree to the parent workspace before cleaning up the worktree.
4. **Pass Context**: Pass the ticket content directly in the subagent's prompt as context.

Equip each subagent with:

- **TypeName**: `self` (or `case` if custom configured)
- **Role**: `Implement-<TICKET_ID>` (substitute the 3-digit ticket ID, e.g. `Implement-044`)
- **Workspace**: `branch`
- **Prompt**:

  ```text
  You are tasked with implementing the changes described in this ticket.

  Ticket Details:
  <TICKET_CONTENT>

  Instructions:
  1. Read the ticket details completely, including the Tasks, Acceptance Criteria (conforming to the guidelines in [backlog-create-issue](../../planning/backlog-create-issue/SKILL.md)), the '## Review' section, and the '## Implementation Review' section (if it exists).
  2. Branch Resumption: Check the ticket frontmatter. If a `branch` is specified (e.g., `branch: subagent-XXX`), ensure you checkout and resume work on that branch, then run `git merge main` to sync with the latest main branch. Otherwise, create a new branch from main.
  3. Implement the changes described, addressing any feedback listed in the '## Implementation Review' section.
  4. Verify your implementation by running tests:
     - Detect if 'devenv.nix' or 'devenv/default.nix' is present in the workspace root. If so, run 'devenv test'.
     - Otherwise, check for standard project test configs (e.g., package.json -> 'npm test', cargo.toml -> 'cargo test', pytest, etc.) and execute them.
     - Ensure the test suite passes prior to returning.
  5. Ensure all pre-commit hooks run and pass using `prek` (see the [prek](../../tooling/prek/SKILL.md) skill). Fix any failing checks before committing.
  6. Commit your changes using Conventional Commits. STRICT GITIGNORE CONSTRAINT: You must NEVER stage, commit, or force-add any files under the `.tars/` directory (such as the ticket file `.tars/issues/todo/XXX.md`). These files must remain completely unstaged and uncommitted in git.
  7. STRICT ISOLATION CONSTRAINT: You must NEVER check out the source/main branch, commit directly to the source/main branch, or attempt to merge branches. You must only commit changes on your local isolated workspace branch and report completion. The orchestrator Hub is solely responsible for merging branches and cleaning up workspaces.
  8. Update the ticket file `.tars/issues/todo/XXX.md` (which has been copied to your worktree) to complete the checkboxes in the '## Tasks' and '## Acceptance Criteria' sections, and document command runs and outputs proving execution in the '## Evidence' section as outlined in [backlog-create-issue](../../planning/backlog-create-issue/SKILL.md).
  ```

### 3. Sequential Merge-Back & Verification (Hub Only)

When all subagents in the batch complete:

1. **Sync Ticket Updates**: If the `.tars/` directory in the subagent's worktree was copied as a fallback instead of symlinked, copy the updated ticket markdown file from `<subagent-worktree>/.tars/issues/todo/XXX.md` back to the parent workspace's `.tars/issues/todo/XXX.md` to preserve the completed checklists and evidence. (If symlinking was successful, they share the same physical file, and copying is a harmless no-op).
2. **Run Implementation Review**: Call the `backlog-review` skill (see [backlog-review](@/skills/review/backlog-review/_index.md)) on each subagent's branch and the synced ticket file to verify the implementation.
3. **Handle Verdicts**:
   - **If Approved**:
     - **Merge Sequentially**: Sequentially merge the branch back into the main/source branch one at a time. Never perform parallel merges.
     - **Pre-commit Integrity**: For each merge, ensure that all pre-commit hooks run and pass using `prek` (see the [prek](@/skills/tooling/prek/_index.md) skill). **NEVER** use `--no-verify` or bypass hooks.
     - **Parent Test Verification**: After each individual merge, run the test suite in the parent workspace to verify stability.
     - **Move Ticket**: Move the ticket file to `.tars/issues/done/`.
     - **CRITICAL CLEANUP CONSTRAINT**: Immediately clean up the worktree and branch.
       - Run `git worktree remove --force <path>`
       - Run `git branch -D <branch-name>`
   - **If Request Rework**:
     - Do **NOT** merge the branch.
     - Increment the ticket's `attempts` count in the frontmatter.
     - If `attempts >= 5`, move the ticket file to `.tars/issues/failed/` and clean up the worktree and branch.
       - Run `git worktree remove --force <path>`
       - Run `git branch -D <branch-name>`
     - Otherwise, set `status: rework`, set `batch: null`, update `branch: <branch-name>` in the frontmatter, and append the review feedback under `## Implementation Review` following the format in [backlog-review](@/skills/review/backlog-review/_index.md). The ticket remains in `.tars/issues/todo/` and the branch is **NOT** cleaned up.

Repeat for subsequent batches until all batches are processed.

