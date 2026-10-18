---
name: oak
description: Operate Oak (oak.space), the developer platform and version control system built around persistent change IDs, working-copy commits, and Git compatibility. Use when working in an Oak repository (.oak/ directory), running oak CLI commands (oak log, oak st, oak diff, oak desc, oak new, oak sync, oak push, oak pr, oak undo, oak op, oak ws, oak mount, oak space, oak ci), managing stacked changes, isolating parallel AI agents with Oak workspaces, mounting dependencies, or configuring .oak/ci/workflows.json.
resources:
  - https://oak.space/llms.txt
  - https://oak.space/docs/llms-full.txt
---

<!-- cspell:ignore changeid changeids kxyz repoPath subpaths worktrees -->

# Oak Version Control and Developer Platform

Oak (`oak.space`) is a developer platform and Git-compatible version control system built for humans and AI agents. In Oak, every file edit is automatically snapshotted into the working-copy commit (`@`), changes carry persistent change IDs that survive rebases and amends, conflicts are stored inside commits as first-class objects rather than blocking operations, and every repository mutation is recorded in an undoable operation log.

When a repository contains an `.oak/` directory, **always use `oak` commands instead of `git`**. Never run mutating `git` commands (`git commit`, `git checkout`, `git rebase`, `git reset`, `git stash`, `git add`) inside an Oak repository.

## Core Mental Model

| Concept                   | How Oak Works                                                                                                                                                                  |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Working copy `@`**      | Every `oak` command automatically snapshots your working tree into the current change (`@`). There is no staging area (`git add`) and no need to `git stash` before switching. |
| **Change ID vs Hash**     | Every change has a stable short prefix (e.g., `kxyz`) that stays constant across edits and rebases, plus a content hash that changes whenever the commit is rewritten.         |
| **First-class conflicts** | Rebasing or merging a conflicting change succeeds and records the conflict inside the commit (`conflict` badge in `oak log`). Descendants automatically rebase once resolved.  |
| **Operation log**         | Every repo operation is logged in `oak op log`. `oak undo` reverts the last operation; `oak op restore <id>` jumps back to any prior repository state.                         |
| **Bookmarks / Remotes**   | Local work uses anonymous changes and stacks. Named branches are created automatically on `oak push` (`oak/<change-id>`) or explicitly with `--branch`.                        |
| **Workspaces**            | `oak ws create <name>` creates an isolated working copy sharing the same underlying store, allowing parallel agents to work simultaneously without file collisions.            |

## Standard Agent & Developer Workflow

Follow this loop when making changes in an Oak repository:

```bash
# 1. Inspect current state and recent history
oak status
oak log -n 10

# 2. Describe your current change before or while working
oak desc -m "feat(auth): validate session tokens"

# 3. Edit files normally - no 'add' or 'stage' step exists
oak diff

# 4. Start the next stacked change on top of '@'
oak new -m "test(auth): add token expiry tests"

# 5. Rebase local stack onto the latest remote trunk
oak sync

# 6. Push changes and open or update pull requests
oak push
oak pr create
```

### Modifying Earlier Changes in a Stack

Because Oak automatically rebases descendants whenever an ancestor commit changes, you can edit any commit in your stack directly:

```bash
# Option A: Switch the working copy to the earlier change, edit files, then return to tip
oak edit <change-id>
# ... make edits (automatically snapshotted into <change-id>; descendants auto-rebase) ...
oak edit <tip-change-id>

# Option B: Make fixes in '@' and squash them into an earlier change
oak squash --into <target-change-id>
```

## CLI Command Reference

### Inspection and Navigation

- `oak status` (alias `oak st`) - Show modified files, parent change, and conflict status of `@`.
- `oak log [-n <count>]` - Display the change graph with change IDs, commit hashes, descriptions, and bookmarks.
- `oak diff [change-id]` - Show unified diff for `@` or a specified change.
- `oak show [change-id]` - Display change metadata, description, and diff.
- `oak edit <change-id>` - Point `@` at an existing change to inspect or modify it directly.

### Creating and Shaping Changes

- `oak new [-m "<message>"] [parent-id...]` - Create a new empty change on top of `@` (or specified parent IDs; pass multiple parents to create a merge commit).
- `oak desc -m "<message>"` (alias `oak describe`) - Set or update the description of `@` (always pass `-m` in non-interactive agent sessions).
- `oak commit -m "<message>"` (alias `oak ci`) - Describe `@` and immediately create a fresh empty change on top of it (`oak desc -m "..." && oak new`).
- `oak squash [--into <change-id>]` - Squash `@` into its parent (or into `--into <change-id>`).
- `oak split` - Split changes in `@` into multiple commits.
- `oak duplicate <change-id>` - Create a copy of an existing change.
- `oak abandon [change-id]` - Abandon a change and automatically rebase its descendants onto its parent.
- `oak restore <paths...>` - Restore specific file paths in `@` to their parent version.
- `oak absorb` - Automatically move each hunk in `@` into the ancestor commit in the stack where those lines were last modified.

### Rebasing, Syncing, and Remotes

- `oak rebase -d <destination>` - Rebase `@` onto `<destination>`.
- `oak rebase -s <source> -d <destination>` - Rebase `<source>` and all of its descendants onto `<destination>`.
- `oak rebase -b <branch> -d <destination>` - Rebase an entire branch stack relative to destination.
- `oak sync` - Fetch from remote and rebase local active stacks onto the updated trunk branch.
- `oak push [--branch <name>] [--change <id>]` - Push changes to the remote (defaults to auto-named `oak/<change-id>` branches).
- `oak pull` / `oak fetch` - Pull or fetch remote updates.

### Pull Requests (`oak pr`)

- `oak pr create` - Create pull requests for the current change or entire stack (stacked PRs automatically target their parent change's branch).
- `oak pr list` (alias `oak pr ls`) - List open pull requests.
- `oak pr view <number>` - View PR details, checks, and reviews.
- `oak pr merge <number>` - Merge a pull request.
- `oak pr update` - Push updated changes in a stack and synchronize stacked PR base branches.

### Undo and Operation Log

- `oak undo` - Undo the most recent repository operation.
- `oak op log` - List the chronological operation log with operation IDs and timestamps.
- `oak op restore <op-id>` - Restore the entire repository state to a specific operation ID.

## Parallel Agent Workspaces (`oak ws`)

When running multiple AI agents or parallel tasks against the same repository, give each agent its own Oak workspace so their working copies (`@`) never collide:

```bash
# Create an isolated workspace for an agent task
oak ws create agent-refactor-auth

# List active workspaces
oak ws list

# Remove a workspace when the agent finishes
oak ws remove agent-refactor-auth
```

- Every workspace shares the same commit graph and object store, so commits created in `agent-refactor-auth` are immediately visible in `oak log` from the default workspace.
- If another workspace rewrites a commit that your current workspace had checked out, Oak flags the working copy as stale; run `oak ws update-stale` to refresh `@`.

## Resolving Conflicts

Conflicts in Oak never leave the repository in a half-finished `REBASE_HEAD` state:

1. Check `oak status` or `oak log` to identify commits marked with `conflict`.
2. Switch to the conflicted commit with `oak edit <change-id>`.
3. Open the conflicted files and resolve the conflict markers (`<<<<<<<`, `%%%%%%%` or `|||||||`, `=======`, `>>>>>>>`), or restore one side with `oak restore`.
4. Verify with `oak status` that the conflict is cleared. All descendant commits in your stack automatically rebase onto the resolved state.
5. Return to the top of your stack with `oak edit <tip-change-id>`.

## Dependency Mounts (`oak mount`)

Oak Mounts graft external repositories (or pinned commits/subpaths of external repos) into a workspace directory with read-only agent policies:

```bash
# Mount an external repo at a local path for cross-repo context
oak mount add https://oak.space/org/shared-lib --path vendor/shared-lib --pin main

# List or sync mounts
oak mount list
oak mount sync
oak mount remove vendor/shared-lib
```

## Multi-Repo Spaces (`oak space`)

Spaces group multiple related repositories into a single workspace manifest (`space.yaml`) for coordinated cross-repo changes:

```bash
oak space init my-platform
oak space add https://oak.space/org/backend
oak space add https://oak.space/org/frontend
oak space status
oak space sync
```

## Sparse Clones (`oak sparse`)

In large monorepos, restrict the working tree to relevant directories while keeping full commit history access:

```bash
oak clone --sparse https://oak.space/org/monorepo
oak sparse set services/api packages/schema
oak sparse add tools/codegen
oak sparse list
```

## Oak CI (`.oak/ci/workflows.json`)

Oak CI workflows are defined in `.oak/ci/workflows.json`:

```json
{
  "workflows": [
    {
      "name": "ci",
      "on": ["push", "pull_request"],
      "steps": [
        {
          "name": "test",
          "run": "cargo test --workspace",
          "env": {
            "RUST_BACKTRACE": "1"
          }
        }
      ]
    }
  ]
}
```

Manage CI runs from the CLI:

- `oak ci status` - Show CI check status for the current change.
- `oak ci run [workflow]` - Trigger a workflow run.
- `oak ci logs <run-id>` - Stream or inspect logs from a CI run.

## Guardrails for AI Agents

- **Always pass `-m` to `oak desc` and `oak commit`:** Calling `oak desc` without `-m` opens `$EDITOR` and hangs headless agent sessions.
- **Never use `git` commands in an `.oak` repo:** Use `oak status`, `oak diff`, `oak log`, `oak new`, `oak desc`, and `oak push`.
- **Use `oak undo` instead of destructive resets:** If a rebase, squash, or abandon goes wrong, immediately run `oak undo` (or inspect `oak op log`).
- **Keep changes atomic with stacks:** Prefer `oak new -m "..."` to start a separate logical step in a stack rather than piling unrelated edits into a single commit.
