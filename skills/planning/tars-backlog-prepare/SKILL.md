---
name: tars-backlog-prepare
description: Prepare to run the tars-backlog-loop by cleaning up orphaned git worktrees and branches, and ensuring the working tree is clean. Reach for this to reset the environment before starting a full backlog loop.
disable-model-invocation: true
---

# Backlog Prepare

Prepare the repository for a fresh run of the `tars-backlog-loop` by ensuring the working tree is clean and cleaning up orphaned git worktrees and subagent branches.

## Targets and Pre-conditions

- This skill modifies the local git repository state.

## Preparation Workflow

### 1. Topic Branch Verification

> This is the **canonical** description of the topic-branch policy for the whole backlog pipeline. The other backlog skills (`tars-backlog-loop`, `tars-backlog-audit`, `tars-backlog-triage`, `tars-backlog-implement`) point back to this section instead of restating it.

To comply with branch protection policies, all backlog operations must run from a topic branch (e.g., `fix/<description>`, `feat/<description>`, `chore/<description>`, or a branch descriptive of the run) rather than the default branch:

1. **Determine Default Branch**: Find the default branch name using the GitHub CLI:

   ```bash
   gh repo view --json defaultBranchRef -q .defaultBranchRef.name
   ```

2. **Check Current Branch**: Run `git branch --show-current` to identify the active branch.
3. **Checkout Topic Branch**: If the current branch is the default branch, abort or checkout/create a topic branch before making any modifications or spawning worktrees. If the repository is already on a topic branch, proceed on it. Never run backlog operations directly on the default branch.
4. **Subagent Base Branch**: All spawned subagents must be branched off this active topic branch.
5. **Merge Target**: All approved subagent changes must be merged back into this topic branch.

### 2. Working Tree Validation

Verify that the git working tree is completely clean (no unstaged changes, no uncommitted files).

- Run `git status --porcelain`.
- If there are _any_ uncommitted or unstaged changes, you **MUST** abort the preparation immediately and instruct the user to explicitly commit or stash their changes manually.
- Do not automatically commit or stash changes.

### 3. Prune Git Worktrees

Clean up any leftover or orphaned git worktrees.

- Run `git worktree prune`.
- Run `git worktree list` to retrieve all registered worktree paths.
- Check each worktree path on disk: if the directory name contains `subagent-` and is no longer tracked by git (or marked as `prunable`), delete the directory.

### 4. Clean Leftover Branches

Force-delete all local branches that match the `subagent-*` pattern, EXCEPT those currently referenced by active rework tickets in `.tars/issues/todo/`.

- Scan `.tars/issues/todo/*.md` files to extract the `branch` field from the frontmatter.
- List all subagent branches: `git branch --list 'subagent-*'`
- Force-delete only the branches that are NOT referenced in the active rework list: `git branch -D <branch-name>`

Once these steps are complete and verified, the repository is ready for the `tars-backlog-loop` skill.
