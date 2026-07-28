---
name: tars-backlog-implement
description: Implement pending backlog issues from `.tars/issues/todo/` in parallel, conflict-free batches using isolated workspaces. Reach for this when asked to implement backlog issues, execute tasks from tickets in parallel, or resolve the issue queue.
disable-model-invocation: true
---

# Backlog Implement

Implement pending issue tickets from `.tars/issues/todo/` in parallel, conflict-free batches using isolated workspaces, and verify and merge them back into the active topic branch sequentially.

This skill operates in a Hub-and-Spoke topology, spawning implementation subagents in parallel, while the Hub manages sequencing, merges, and testing.

## Targets and Paths

- Target Folders: `.tars/issues/{todo,done,failed}/` relative to project root.
- Ticket status updates are written to disk only. Ticket files are never staged, committed, or force-added in git.

## Topic Branch Workflow (Hub Only)

All backlog operations run from a topic branch (never the default branch). Every implementation subagent branches off this active topic branch, and all approved changes are merged back into it. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](../../planning/tars-backlog-prepare/SKILL.md) for the full policy and commands.

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
4. **Isolate Pre-commit Cache**: To prevent pre-commit cache folder permission blocks or hangs inside the subagent's sandboxed environment, the Hub **MUST** set the `PRE_COMMIT_HOME` environment variable to point to a local directory inside the worktree (e.g. `<worktree-path>/.cache/pre-commit`) before spawning or symlinking.
5. **Pass Context**: Pass the ticket content directly in the subagent's prompt as context.

Spawn each subagent for its ticket on its own isolated workspace/branch, and equip it with:

- **Role**: `Implement-<TICKET_ID>` (substitute the 3-digit ticket ID, e.g. `Implement-044`)
- **Prompt**:

  ```text
  You are tasked with implementing the changes described in this ticket.

  Ticket Details:
  <TICKET_CONTENT>

  Instructions:
  1. Read the ticket details completely, including the Tasks, Acceptance Criteria (conforming to the guidelines in [tars-backlog-create-issue](../../planning/tars-backlog-create-issue/SKILL.md)), the '## Review' section, and the '## Implementation Review' section (if it exists).
  2. Branch Resumption: Check the ticket frontmatter. If a `branch` is specified (e.g., `branch: subagent-XXX`), ensure you checkout and resume work on that branch, then run `git merge <topic-branch>` (substituting the active topic branch name) to sync with the latest changes. Otherwise, create a new branch from the active topic branch.
  3. Implement the changes described, addressing any feedback listed in the '## Implementation Review' section.
  4. Verify your implementation by running tests:
     - Detect if 'devenv.nix' or 'devenv/default.nix' is present in the workspace root. If so, run 'devenv test'.
     - Otherwise, check for standard project test configs (e.g., package.json -> 'npm test', cargo.toml -> 'cargo test', pytest, etc.) and execute them.
     - Ensure the test suite passes prior to returning.
  5. Ensure all pre-commit hooks run and pass using `prek` (see the [prek](../../tooling/prek/SKILL.md) skill). To avoid permission errors or hangs, prefix the execution command with the isolated cache environment: `PRE_COMMIT_HOME="<worktree-path>/.cache/pre-commit" prek run -a`. Fix any failing checks before committing.
  6. Commit your changes using Conventional Commits. STRICT GITIGNORE CONSTRAINT: You must NEVER stage, commit, or force-add any files under the `.tars/` directory (such as the ticket file `.tars/issues/todo/XXX.md`). These files must remain completely unstaged and uncommitted in git.
  7. STRICT ISOLATION CONSTRAINT: You must NEVER check out the default branch or the active topic branch, commit directly to them, or attempt to merge branches. You must only commit changes on your local isolated workspace branch and report completion. The orchestrator Hub is solely responsible for merging branches and cleaning up workspaces.
  8. Update the ticket file `.tars/issues/todo/XXX.md` (which has been copied to your worktree) to complete the checkboxes in the '## Tasks' and '## Acceptance Criteria' sections, and document command runs and outputs proving execution in the '## Evidence' section as outlined in [tars-backlog-create-issue](../../planning/tars-backlog-create-issue/SKILL.md).
  9. **STRICT TOOL SYNTAX CONSTRAINT**: When calling filesystem or command-execution tools, you must never wrap string argument values in nested, escaped, or literal double quotes (e.g. pass a path argument as `/path/to/file`, not the same value re-wrapped in escaped quotes, which is incorrect and will fail due to invalid characters).
  ```

### 3. Incremental Merge-Back, Liveness Checking & Verification (Hub Only)

Rather than waiting passively for the entire batch to complete (which blocks progress if a single spoke gets stuck or dies), the Hub must monitor spokes dynamically and process them incrementally:

1. **Monitor Spoke Liveness, Approvals & Revive**:
   - Do **NOT** wait passively. Periodically (e.g. every couple of minutes) check on the running subagents while they work: if your runtime provides a scheduling or wakeup mechanism, use it to trigger the check, otherwise poll. On each check, use your agent's subagent-management capability to list the running subagents and verify their liveness/status.
   - **Detect blocked/approval-waiting subagents**: Subagents running commands inside sandboxed worktrees may issue a command that gets suspended waiting for user approval, and such approval prompts do not always bubble up automatically. If your runtime surfaces subagent logs or state, inspect the latest entries to detect a subagent blocked awaiting approval; otherwise rely on its status/liveness signal. When a subagent appears blocked on an approval, output an explicit warning to the user:
     `"⚠️ Subagent <role> is waiting for your approval to run a command. Please switch to its session or approve the command."`
   - If a subagent has stopped (e.g., due to a server restart or crash) before completing its task, check its branch status. Revive it by sending it a follow-up query to resume its task, or restart the subagent on that branch if needed.
2. **Process Completed Spokes Incrementally**: As soon as any individual subagent in the batch reports completion, immediately run the merge-back and verification workflow for that spoke:
   - **Reset Staged Side-Effects**: Running tests or git hooks in the parent repository can generate untracked or staged test files (e.g., `target-skill`) that block git merges. Before executing git merge, verify `git status --porcelain`. If any unstaged or staged changes exist in the parent repository, run `git reset --hard` and `git clean -fd` to completely clear the git index and working tree.
   - **Sync Ticket Updates**: If the `.tars/` directory in the subagent's worktree was copied as a fallback instead of symlinked, copy the updated ticket markdown file from `<subagent-worktree>/.tars/issues/todo/XXX.md` back to the parent workspace's `.tars/issues/todo/XXX.md` to preserve the completed checklists and evidence. (If symlinking was successful, they share the same physical file, and copying is a harmless no-op).
   - **Run Implementation Review**: Call the `tars-backlog-review` skill (see [tars-backlog-review](../../review/tars-backlog-review/SKILL.md)) on the subagent's branch and the synced ticket file to verify the implementation.
   - **Handle Verdicts**:
     - **If Approved**:
       - **Merge Sequentially**: Sequentially merge the branch back into the active topic branch one at a time. Never perform parallel merges.
       - **Pre-commit Integrity**: For each merge, ensure that all pre-commit hooks run and pass using `prek` (see the [prek](../../tooling/prek/SKILL.md) skill). **NEVER** use `--no-verify` or bypass hooks.
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
       - Otherwise, set `status: rework`, set `batch: null`, update `branch: <branch-name>` in the frontmatter, and append the review feedback under `## Implementation Review` following the format in [tars-backlog-review](../../review/tars-backlog-review/SKILL.md). The ticket remains in `.tars/issues/todo/` and the branch is **NOT** cleaned up.

Repeat monitoring and incremental merges until all spokes in the batch have been successfully merged or moved to rework/failed, then proceed to the next batch.
