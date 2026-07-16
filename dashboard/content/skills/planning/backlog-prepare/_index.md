+++
title = "backlog-prepare"
description = "Prepare to run the backlog-loop by cleaning up orphaned git worktrees and branches, and ensuring the working tree is clean. Reach for this to reset the environment before starting a full backlog loop."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Prepare

Prepare the repository for a fresh run of the `backlog-loop` by ensuring the working tree is clean and cleaning up orphaned git worktrees and subagent branches.

## Targets and Pre-conditions

- This skill modifies the local git repository state.

## Preparation Workflow

### 1. Working Tree Validation

Verify that the git working tree is completely clean (no unstaged changes, no uncommitted files).

- Run `git status --porcelain`.
- If there are _any_ uncommitted or unstaged changes, you **MUST** abort the preparation immediately and instruct the user to explicitly commit or stash their changes manually.
- Do not automatically commit or stash changes.

### 2. Prune Git Worktrees

Clean up any leftover or orphaned git worktrees.

- Run `git worktree prune`.
- Check if `.system_generated/worktrees/` contains any `subagent-*` directories. If they are no longer tracked by git as worktrees, delete those directories.

### 3. Clean Leftover Branches

Force-delete all local branches that match the `subagent-*` pattern, EXCEPT those currently referenced by active rework tickets in `.tars/issues/todo/`.

- Scan `.tars/issues/todo/*.md` files to extract the `branch` field from the frontmatter.
- List all subagent branches: `git branch --list 'subagent-*'`
- Force-delete only the branches that are NOT referenced in the active rework list: `git branch -D <branch-name>`

Once these steps are complete and verified, the repository is ready for the `backlog-loop` skill.

