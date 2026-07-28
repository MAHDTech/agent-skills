---
name: tars-backlog-prepare
description: Prepare to run the tars-backlog-loop by verifying repository integrity, resolving isolated spoke workspaces, and cleaning up orphaned clones and branches. Reach for this to reset the environment before starting a full backlog loop.
disable-model-invocation: true
---

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

3. **Detect hijacked hook shims**: hook installers (`prek`, `pre-commit`, `lefthook`) bake an **absolute** path into each shim they write, and `.git/hooks/` is shared by every linked worktree. A shim installed from anywhere other than the repository root therefore points the whole repository at a foreign config.

   ```bash
   REPO_ROOT="$(git rev-parse --show-toplevel)"
   GIT_COMMON="$(git rev-parse --git-common-dir)"
   # Any absolute path in a shim that does not point inside $REPO_ROOT is suspect.
   grep -o -- '/[^"'\'' ]*' "$GIT_COMMON"/hooks/* 2>/dev/null | grep -v "$REPO_ROOT"
   ```

   For each matching shim, read the baked path. If it is not exactly `$REPO_ROOT/.pre-commit-config.yaml`, repair it. Write through a temporary file rather than editing in place, because `sed -i` takes an argument on BSD/macOS and none on GNU, so the in-place form is not portable:

   ```bash
   sed 's|--config="[^"]*"|--config="'"$REPO_ROOT"'/.pre-commit-config.yaml"|' \
     "$hook" > "$hook.tars-tmp" && mv "$hook.tars-tmp" "$hook" && chmod +x "$hook"
   ```

4. **Warn on a replicated git directory**: a file-sync tool that replicates `.git/config`, `.git/hooks/`, or `.git/index` between machines can reintroduce exactly the damage repaired above, and no amount of workspace isolation prevents it — the writer is outside this pipeline.

   Walk up from `$REPO_ROOT` looking for any **replicated-directory marker**. Merge `worktree.sync_markers` from `.tars/config.yaml` (if set) over these defaults:

   | Tool         | Marker at the folder root                   |
   | ------------ | ------------------------------------------- |
   | Syncthing    | `.stfolder`                                 |
   | Dropbox      | `.dropbox`, `.dropbox.cache`                |
   | Nextcloud    | `.sync_exclude.lst`, `._sync_*.db`          |
   | Google Drive | `.tmp.driveupload`, `.tmp.drivedownload`    |
   | iCloud Drive | any path under `~/Library/Mobile Documents` |

   If a marker is found, check whether that tool is configured to exclude `.git/` (for Syncthing, the sibling `.stignore`; other tools keep their exclude list elsewhere, and may not support one at all). Warn unless `.git/` is excluded — or at minimum `.git/config`, `.git/hooks/`, and `.git/index`.

   This is a **warning, not an abort**, and it finds nothing on a machine that syncs no directories, which is the common case. It is the user's environment, not something this pipeline can fix.

   > The marker list is a convenience, not a guarantee. The hazard is any external process that mutates the repository behind git's back — a network filesystem, a backup agent that restores files, an editor's remote-sync feature. Treat an unexplained change to shared git state as this class of problem even when no marker matched.

Report every repair made. **Abort** on anything unrecognised in the shared git state rather than guessing at a fix.

### 2. Topic Branch Verification

> This is the **canonical** description of the topic-branch policy for the whole backlog pipeline. The other backlog skills (`tars-backlog-loop`, `tars-backlog-audit`, `tars-backlog-triage`, `tars-backlog-implement`) point back to this section instead of restating it.

To comply with branch protection policies, all backlog operations must run from a topic branch (e.g., `fix/<description>`, `feat/<description>`, `chore/<description>`, or a branch descriptive of the run) rather than the default branch:

1. **Determine Default Branch**: use the first of these that yields a name. They are ordered so the pipeline works on any git host, with no network and no remote at all:

   ```bash
   # a. What the remote itself says the default is (any host, works offline once set).
   git symbolic-ref --quiet --short refs/remotes/origin/HEAD    # -> origin/main

   # b. Ask the remote directly, if there is one and it is reachable.
   git remote show origin | sed -n 's/.*HEAD branch: //p'

   # c. This repository's configured default for new branches.
   git config --get init.defaultBranch

   # d. Last resort: whichever of these exists locally.
   git branch --list main master trunk
   ```

   Only if all of the above fail, and only when the remote is a GitHub one, fall back to `gh repo view --json defaultBranchRef -q .defaultBranchRef.name`. Never make the GitHub CLI a hard requirement: it is absent on many machines, useless for GitLab, Gitea, Codeberg, or a bare remote over SSH, and it needs the network.

   If no method yields a name, **ask the user** rather than guessing — picking the wrong default branch here means the protection this step exists to provide silently does not apply.

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

Each candidate must pass **both** checks:

```bash
# 1. Writable?
probe="$candidate/.tars-write-probe.$$"
mkdir -p "$candidate" 2>/dev/null && : > "$probe" 2>/dev/null && rm -f "$probe"

# 2. Outside any replicated folder? Reuse the marker walk from step 1d,
#    this time starting at $candidate rather than at $REPO_ROOT.
```

Each candidate must also have room. A batch holds up to 5 clones at once, and spokes that build produce artefacts far larger than the source — a single compiled binary can be 100 MB or more, so budget generously rather than for the checkout alone:

```bash
df -P "$candidate" | awk 'NR==2 {print $4}'   # available 1K-blocks, POSIX on Linux and macOS
```

Abort with a clear message if free space is short. Running out mid-batch surfaces as an unrelated-looking build or test failure in whichever spoke happens to be writing at the time, and costs a diagnosis; a preflight check costs one command.

Reject a candidate that sits inside a replicated folder and move to the next one. `$HOME/.cache` is outside any synced tree on most setups, but `$HOME` itself is replicated on plenty of others — and putting spoke clones there means five live clones being replicated mid-write, which is the hazard step 1d warns about, amplified.

Abort if no candidate passes both.

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

#### 4d. Resolve the repository's own commands

Determine, once, the three commands every spoke and gate will need, and verify each actually runs in this repository before recording it. Detect them from what the repository contains — never assume a specific toolchain:

- **Test command** — what the Hub's verification gate runs. `devenv test` if `devenv.nix` or `devenv/default.nix` is present; otherwise the project's standard entry point (`bun test`, `npm test`, `cargo test`, `pytest`, `go test ./...`, `mix test`, …). Prefer the script the repository itself treats as its full suite, which is often not the bare runner: a `test:coverage` or `check` script may be the real gate.
- **Install command** — what a fresh clone needs before it can build or typecheck. Use the **lockfile-respecting** form, because this command also runs inside the verification gate where the committed lockfile is the authority: `bun install --frozen-lockfile`, `npm ci`, `cargo fetch --locked`, `uv sync --frozen`, `go mod download`. Record an empty value for repositories that need no install step.
- **Hook command** — how this repository runs its whole-repo hooks, which the gate invokes before the tests:

  | Repository contains                          | Hook command                          |
  | -------------------------------------------- | ------------------------------------- |
  | `.pre-commit-config.yaml`, `prek` on PATH    | `prek run -a`                         |
  | `.pre-commit-config.yaml`, `pre-commit` only | `pre-commit run --all-files`          |
  | `lefthook.yml` / `lefthook.yaml`             | `lefthook run pre-commit --all-files` |
  | `.husky/`                                    | the script the hook itself runs       |
  | none of the above                            | empty — the gate runs tests only      |

  An empty hook command is a legitimate answer, not a failure. Record it; the gate substitutes `:` for it.

**Then check for overlap, and subtract it.** Hook runners frequently include a hook that already runs the test suite — so a naive gate of `<hooks> && <tests>` runs the whole suite twice. Inspect the hook config: if a hook already covers typecheck, lint, build, or test, set the test command to only what the hooks do **not** cover. A common residue is a coverage-threshold run, which a plain test invocation does not enforce.

> This matters more than ordinary waste, because the gate runs **while holding the mutex**. Every second spent re-running an already-green suite is a second no other spoke can test, so duplicated work in the gate does not merely cost time — it shrinks the whole batch's throughput by extending the one serialised section in the pipeline.

Resolving these here rather than per spoke means every spoke bootstraps the same way and the gate is a concrete command instead of a placeholder each Hub re-derives on a repository it may be seeing for the first time.

> Check whether the repository suppresses automatic dependency installation under `CI=true` — several `devenv.nix` setups do. Where it does, the install command is mandatory rather than a convenience, and a spoke that skips it fails with an error that names a missing type definition rather than a missing install.

#### 4e. Record the resolved values

Write the results to `.tars/run.env` in the parent workspace:

```sh
TARS_SPOKE_ROOT="…"
TARS_CLONE_MODE="hardlink"   # or "shared"
TARS_LOCK="…/resources/manual/tars-lock"
TARS_HEAVY_LOCK="…/locks/<repo-name>.heavy"
TARS_TOPIC_BRANCH="…"
TARS_TEST_COMMAND="…"
TARS_INSTALL_COMMAND="…"   # may be empty
TARS_HOOK_COMMAND="…"      # may be empty
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
