+++
title = "tars-backlog-prepare"
description = "Prepare to run the tars-backlog-loop by verifying repository integrity, resolving isolated spoke workspaces, and cleaning up orphaned clones and branches. Reach for this to reset the environment before starting a full backlog loop."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Prepare

Prepare the repository for a fresh run of the `tars-backlog-loop` by verifying the shared git state is uncorrupted, resolving where isolated spoke workspaces will live, and cleaning up orphaned workspaces and subagent branches.

## Targets and Pre-conditions

- This skill modifies the local git repository state.
- Run every step in order. Steps 1 through 4 are preconditions; abort the whole preparation if any of them fails rather than continuing with a warning.

## Preparation Workflow

### 1. Shared Git Integrity Check

> Run this **first**, before any other git command. A redirected working tree makes `git status`, `git branch`, and `git diff` report on a different directory entirely, so every later check in this skill would be reading someone else's files without saying so.

The backlog pipeline gives each spoke a private clone precisely so that no spoke can write to the parent's git state. That does not make this check redundant. External tools still reach the shared git directory — most notably a file-sync daemon replicating `.git/config` and `.git/hooks/` between machines, which is outside this pipeline's control — and repositories that ran earlier worktree-based versions of these skills may still carry damage.

1. **Detect a redirected working tree**:

   ```bash
   git config --local --get-all core.worktree
   ```

   Any value here is damage: it silently redirects every git command in this repository at another directory. Repair it by unsetting the key, then re-run the command to confirm it is gone:

   ```bash
   git config --local --unset-all core.worktree
   ```

2. **Detect a redirected hooks directory**:

   ```bash
   git config --local --get core.hooksPath
   ```

   Unset it if it points anywhere outside this repository. Leave it alone if it resolves inside the repository, since that may be a deliberate project setting.

3. **Detect hijacked hook shims**: `prek install` bakes an **absolute** path to its config into each shim it writes, and `.git/hooks/` is shared by every linked worktree. A shim installed from anywhere other than the repository root therefore points the whole repository at a foreign config.

   ```bash
   REPO_ROOT="$(git rev-parse --show-toplevel)"
   GIT_COMMON="$(git rev-parse --git-common-dir)"
   grep -l -- '--config=' "$GIT_COMMON"/hooks/* 2>/dev/null
   ```

   For each matching shim, read the baked path. If it is not exactly `$REPO_ROOT/.pre-commit-config.yaml`, repair it. Write through a temporary file rather than editing in place, because `sed -i` takes an argument on BSD/macOS and none on GNU, so the in-place form is not portable:

   ```bash
   sed 's|--config="[^"]*"|--config="'"$REPO_ROOT"'/.pre-commit-config.yaml"|' \
     "$hook" > "$hook.tars-tmp" && mv "$hook.tars-tmp" "$hook" && chmod +x "$hook"
   ```

4. **Warn on a replicated git directory**: walk up from `$REPO_ROOT` looking for a `.stfolder` marker. If one is found, read the sibling `.stignore`. Unless it excludes `.git/` (or at minimum `.git/config`, `.git/hooks/`, and `.git/index`), warn the user that their sync daemon is replicating shared git state between machines and can reintroduce exactly the damage repaired above. This is a warning, not an abort — it is the user's environment, not something this pipeline can fix.

Report every repair made. **Abort** on anything unrecognised in the shared git state rather than guessing at a fix.

### 2. Topic Branch Verification

> This is the **canonical** description of the topic-branch policy for the whole backlog pipeline. The other backlog skills (`tars-backlog-loop`, `tars-backlog-audit`, `tars-backlog-triage`, `tars-backlog-implement`) point back to this section instead of restating it.

To comply with branch protection policies, all backlog operations must run from a topic branch (e.g., `fix/<description>`, `feat/<description>`, `chore/<description>`, or a branch descriptive of the run) rather than the default branch:

1. **Determine Default Branch**: Find the default branch name using the GitHub CLI:

   ```bash
   gh repo view --json defaultBranchRef -q .defaultBranchRef.name
   ```

2. **Check Current Branch**: Run `git branch --show-current` to identify the active branch.
3. **Checkout Topic Branch**: If the current branch is the default branch, abort or checkout/create a topic branch before making any modifications or spawning spokes. If the repository is already on a topic branch, proceed on it. Never run backlog operations directly on the default branch.
4. **Spoke Base Branch**: All spawned spokes must be branched off this active topic branch.
5. **Merge Target**: All approved spoke changes must be merged back into this topic branch.

### 3. Working Tree Validation

Verify that the git working tree is completely clean (no unstaged changes, no uncommitted files).

- Run `git status --porcelain`.
- If there are _any_ uncommitted or unstaged changes, you **MUST** abort the preparation immediately and instruct the user to explicitly commit or stash their changes manually.
- Do not automatically commit or stash changes.

### 4. Workspace Preflight

Resolve — and record — everything the implementation phase needs to isolate its spokes. Doing this once here, rather than per spoke, means the whole run shares one answer.

#### 4a. Resolve the spoke root

Spoke workspaces must not live inside the repository tree or inside a synced folder. Try each candidate in order and take the **first one that passes a real write test**. Do not attempt to detect whether you are sandboxed: there are too many confinement mechanisms to enumerate, and a write probe answers the question directly on all of them.

1. `$TARS_SPOKE_DIR`, or `worktree.spoke_dir` from `.tars/config.yaml` if set. An explicit override always wins. This is the injection point for a harness that already knows its writable path, such as an agent runtime that provides a session scratch directory.
2. `${XDG_CACHE_HOME:-$HOME/.cache}/tars/spokes/<repo-name>/`
3. `${TMPDIR:-/tmp}/tars/spokes/<repo-name>/`

```bash
probe="$candidate/.tars-write-probe.$$"
mkdir -p "$candidate" 2>/dev/null && : > "$probe" 2>/dev/null && rm -f "$probe"
```

Abort if no candidate is writable.

#### 4b. Choose the clone mode

Local clones hardlink their object store, but only when source and destination share a filesystem. Compare them with `df -P`, which is POSIX and behaves identically on Linux and macOS (`stat` does not — it takes `-c` on GNU and `-f` on BSD):

```bash
df -P "$REPO_ROOT"  | awk 'NR==2 {print $1}'
df -P "$SPOKE_ROOT" | awk 'NR==2 {print $1}'
```

- **Same filesystem** → clone mode `hardlink`: a plain `git clone`. Objects are hardlinked, so the clone is nearly free and stays safe even if the parent runs `git gc`, because the hardlink keeps any pruned object alive.
- **Different filesystem** → clone mode `shared`: `git clone --shared`. Objects are borrowed through alternates rather than copied, which matters most in a sandbox where `/tmp` may be a RAM-backed tmpfs. The cost is that the parent must **not** run `git gc` or `git prune` while any spoke is alive, because pruning objects a spoke borrows will break it.

#### 4c. Verify the lock mechanism

Heavy commands run under a mutex (see `tars-backlog-implement`). Confirm the helper that provides it is present and runnable:

```bash
sh "$TARS_LOCK" /tmp/tars-preflight-probe true
```

`$TARS_LOCK` is `resources/manual/tars-lock` inside the installed `tars-backlog-implement` skill directory. Always invoke it as `sh <path>` rather than executing it directly, so it works regardless of whether the install method preserved the executable bit. The helper prefers `flock(1)` and falls back to an atomic-mkdir lock where that is missing, which is the normal case on macOS — no action is needed either way, but note which path is active when reporting.

#### 4d. Record the resolved values

Write the results to `.tars/run.env` in the parent workspace:

```sh
TARS_SPOKE_ROOT="…"
TARS_CLONE_MODE="hardlink"   # or "shared"
TARS_LOCK="…/resources/manual/tars-lock"
TARS_HEAVY_LOCK="…/locks/<repo-name>.heavy"
TARS_TOPIC_BRANCH="…"
```

Write these to disk rather than only holding them in context. A backlog run is long enough that the Hub's context may be compacted partway through, and a Hub that has forgotten where its spoke root is will resolve a different one mid-run. `.tars/` is already shared into every spoke workspace, so spokes can read the same file.

### 5. Clean Up Orphaned Workspaces

1. **Legacy worktrees**: earlier versions of these skills gave spokes git worktrees inside or beside the repository. Remove any that remain, which also migrates a repository off the old model:

   ```bash
   git worktree list --porcelain
   git worktree remove --force <path>   # for each spoke worktree
   git worktree prune
   ```

   Removing a worktree does not delete its branch, so no spoke work is lost here — branch cleanup is sub-step 3 below, and it protects rework branches explicitly.

2. **Stale spoke clones**: delete any directory under `$TARS_SPOKE_ROOT` left behind by a previous run. Every clone is disposable scratch; spoke work durably lives on branches in the parent repository, so nothing is lost. Confirm each directory is a clone of this repository before deleting it, and never delete the spoke root itself.

3. **Leftover branches**: force-delete all local branches matching `subagent-*`, EXCEPT those currently referenced by active rework tickets in `.tars/issues/todo/`.
   - Scan `.tars/issues/todo/*.md` files to extract the `branch` field from the frontmatter.
   - List all subagent branches: `git branch --list 'subagent-*'`
   - Force-delete only the branches that are NOT referenced in the active rework list: `git branch -D <branch-name>`

Once these steps are complete and verified, the repository is ready for the `tars-backlog-loop` skill.

## Portable Command Baseline

Every command this pipeline emits runs on both Linux and macOS. Three GNU-only idioms are easy to reach for and silently misbehave on BSD userland:

- `sed -i` — takes a mandatory suffix argument on BSD/macOS. Write through a temporary file and `mv` it into place instead.
- `readlink -f` — GNU-only. Use `realpath`, or resolve the path with `cd` and `pwd -P`.
- `stat -c` — GNU spelling; BSD uses `-f`. For filesystem identity use `df -P` instead.

