---
name: tars-backlog-prepare
description: Prepare to run the tars-backlog-loop by verifying repository integrity, resolving isolated spoke workspaces, and cleaning up orphaned clones and branches. Reach for this to reset the environment before starting a full backlog loop.
disable-model-invocation: true
---

# Backlog Prepare

Prepare the repository for a fresh run of the `tars-backlog-loop` by verifying the shared git state is uncorrupted, resolving where isolated spoke workspaces will live, proving the verification gate recipe against the current baseline, and cleaning up orphaned workspaces and subagent branches.

## Targets and Pre-conditions

- This skill modifies the local git repository state and writes `.tars/run.env` (and may create `.tars/config.yaml` only if you are documenting a new override - do not invent policy without need).
- Run every step in order. Steps 1 through 4 are preconditions; abort the whole preparation if any of them fails rather than continuing with a warning.
- **Config vs run.env (strict split):**
  - **`.tars/config.yaml`** - project **policy** (stable across runs). Humans edit this. Prepare reads it; implement does **not**.
  - **`.tars/run.env`** - **run facts** frozen for this prepare. Prepare writes it every run; implement, `tars-gate`, and `tars-spoke` read **only** this file for paths, commands, land template, CI flags, and weaken banners.

## Configuration schema (`.tars/config.yaml`)

Policy keys and skill defaults. Omit any key to keep the default. Prepare merges repo config over these defaults, then freezes concrete values into `run.env`.

```yaml
worktree:
  spoke_dir: null # or absolute path; overrides TARS_SPOKE_DIR env
  sync_markers: [] # extra replicated-folder markers
  shared_append_files:
    - "project-words.txt"
    - "CHANGELOG.md"
  transfer_files:
    - ".pre-commit-config.yaml"
    - ".env*"
    - ".tars/"
    - "devenv.local.nix"
    - "devenv.local.yaml"

concurrency:
  heavy_commands:
    - "test"
    - "coverage"
    - "run -a"
    - "--all-files"
    - "nix build"

# Opaque command overrides (optional). When set, prepare uses them instead of
# auto-detect - still wraps with devenv enter when applicable, still smokes them.
commands:
  install: null
  hooks: null
  test: null
  # Optional pre-dispatch ticket linter. See "Ticket lint contract" in step 4c.
  # Receives the batch number, runs against the working tree, exits non-zero on
  # a batch with errors. Null (the default) means no pre-dispatch check exists.
  ticket_lint: null

land:
  # {{id}} and {{title}} are substituted by the Hub at merge time.
  subject_template: "chore(backlog): land ticket {{id}}"

review:
  always_full:
    - "hooks/**"
    - "**/*secret*"
    - "**/auth/**"
  # Full dual-axis review when risk: high, post-conflict, rework attempts >= 2,
  # or diff touches always_full. Otherwise Hub lightweight checklist.

ci:
  # null = auto-detect in prepare. true/false force on/off.
  check: null
  block_on_red: true
  # Optional opaque check command; when null, prepare may set a gh-based default.
  command: null

gate:
  # If true, prepare may record a weakened test command after classifying
  # baseline-red product/coverage failure. Still requires reason in run.env.
  allow_weaken: true
```

Implement never re-opens this file for gate, land, or CI decisions - only the frozen `run.env` keys.

## Preparation Workflow

### 1. Shared Git Integrity Check

> Run this **first**, before any other git command. A redirected working tree makes `git status`, `git branch`, and `git diff` report on a different directory entirely, so every later check in this skill would be reading someone else's files without saying so.

The backlog pipeline gives each spoke a private clone precisely so that no spoke can write to the parent's git state. That does not make this check redundant. External tools still reach the shared git directory - most notably a file-sync daemon replicating `.git/config` and `.git/hooks/` between machines, which is outside this pipeline's control - and repositories that ran earlier worktree-based versions of these skills may still carry damage.

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

4. **Warn on a replicated git directory**: a file-sync tool that replicates `.git/config`, `.git/hooks/`, or `.git/index` between machines can reintroduce exactly the damage repaired above, and no amount of workspace isolation prevents it - the writer is outside this pipeline.

   Walk up from `$REPO_ROOT` looking for any **replicated-directory marker**. Merge `worktree.sync_markers` from `.tars/config.yaml` (if set) over these defaults:

   | Tool         | Marker at the folder root                   |
   | ------------ | ------------------------------------------- |
   | Syncthing    | `.stfolder`                                 |
   | Dropbox      | `.dropbox`, `.dropbox.cache`                |
   | Nextcloud    | `.sync_exclude.lst`, `._sync_*.db`          |
   | Google Drive | `.tmp.driveupload`, `.tmp.drivedownload`    |
   | iCloud Drive | any path under `~/Library/Mobile Documents` |

   If a marker is found, check whether that tool is configured to exclude `.git/` (for Syncthing, the sibling `.stignore`; other tools keep their exclude list elsewhere, and may not support one at all). Warn unless `.git/` is excluded - or at minimum `.git/config`, `.git/hooks/`, and `.git/index`.

   This is a **warning, not an abort**, and it finds nothing on a machine that syncs no directories, which is the common case. It is the user's environment, not something this pipeline can fix.

   > The marker list is a convenience, not a guarantee. The hazard is any external process that mutates the repository behind git's back - a network filesystem, a backup agent that restores files, an editor's remote-sync feature. Treat an unexplained change to shared git state as this class of problem even when no marker matched.

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

   If no method yields a name, **ask the user** rather than guessing - picking the wrong default branch here means the protection this step exists to provide silently does not apply.

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

Resolve - and record - everything the implementation phase needs to isolate its spokes. Doing this once here, rather than per spoke, means the whole run shares one answer.

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

Each candidate must also have room. A batch holds up to 5 clones at once, and spokes that build produce artefacts far larger than the source - a single compiled binary can be 100 MB or more, so budget generously rather than for the checkout alone:

```bash
df -P "$candidate" | awk 'NR==2 {print $4}'   # available 1K-blocks, POSIX on Linux and macOS
```

Abort with a clear message if free space is short. Running out mid-batch surfaces as an unrelated-looking build or test failure in whichever spoke happens to be writing at the time, and costs a diagnosis; a preflight check costs one command.

Reject a candidate that sits inside a replicated folder and move to the next one. `$HOME/.cache` is outside any synced tree on most setups, but `$HOME` itself is replicated on plenty of others - and putting spoke clones there means five live clones being replicated mid-write, which is the hazard step 1d warns about, amplified.

Abort if no candidate passes both.

#### 4b. Choose the clone mode

Local clones hardlink their object store, but only when source and destination share a filesystem. Compare them with `df -P`, which is POSIX and behaves identically on Linux and macOS (`stat` does not - it takes `-c` on GNU and `-f` on BSD):

```bash
df -P "$REPO_ROOT"  | awk 'NR==2 {print $1}'
df -P "$SPOKE_ROOT" | awk 'NR==2 {print $1}'
```

- **Same filesystem** → clone mode `hardlink`: a plain `git clone`. Objects are hardlinked, so the clone is nearly free and stays safe even if the parent runs `git gc`, because the hardlink keeps any pruned object alive.
- **Different filesystem** → clone mode `shared`: `git clone --shared`. Objects are borrowed through alternates rather than copied, which matters most in a sandbox where `/tmp` may be a RAM-backed tmpfs. The cost is that the parent must **not** run `git gc` or `git prune` while any spoke is alive, because pruning objects a spoke borrows will break it.

#### 4c. Locate runners and verify the lock

Resolve paths inside the installed `tars-backlog-implement` skill directory:

| Key          | Path under skill              |
| ------------ | ----------------------------- |
| `TARS_LOCK`  | `resources/manual/tars-lock`  |
| `TARS_GATE`  | `resources/manual/tars-gate`  |
| `TARS_SPOKE` | `resources/manual/tars-spoke` |

Always invoke helpers as `sh <path> …` so missing executable bits do not matter.

Confirm the lock works:

```bash
sh "$TARS_LOCK" /tmp/tars-preflight-probe true
```

Confirm gate and spoke scripts exist and are readable. Do **not** document platform-specific lock backends to the user - one consistent `sh tars-gate` / `sh tars-spoke` surface everywhere.

##### Ticket lint contract (optional)

The implementation phase can run a **pre-dispatch check** over a batch before it spawns any spoke. This pipeline does not ship such a checker and does not require one; it defines the contract and resolves the command from configuration, so any project may supply its own.

Resolve `commands.ticket_lint` from `.tars/config.yaml`, or `$TARS_TICKET_LINT_COMMAND` from the environment if set. **Never hardcode a particular project's tool here.** The command a repository supplies is its own business; the pipeline only knows the contract:

| Aspect      | Contract                                                                                                     |
| ----------- | ------------------------------------------------------------------------------------------------------------ |
| Invocation  | Called with the batch number as its argument                                                                 |
| Working dir | Repository root                                                                                              |
| Reads       | The **working tree** - ticket files under `.tars/issues/`, as they are on disk right now                     |
| Not a hook  | It must **not** be wired as a pre-commit hook: `.tars/` is gitignored, so a hook would never see the tickets |
| Exit status | `0` when the batch is clean; non-zero when the batch has errors                                              |

Freeze the resolved string into `run.env` as `TARS_TICKET_LINT_COMMAND`. When nothing is configured, freeze it as the **empty string** - implement then skips the check and logs that it skipped, which is the expected case for most repositories.

If a value **is** configured, smoke it once here (invoke it with a batch number, or with whatever no-op argument it accepts) so a broken or missing command is found now rather than at dispatch time. A linter that is configured but does not run is an error: fix it or clear the config key. Do not silently freeze a command that does not run.

#### 4d. Resolve the repository's own commands (opaque strings)

Determine, once, the three commands every spoke and gate will need. Detect them from what the repository contains - **never assume a specific toolchain** in implement prose later. Prefer `commands.*` overrides from `.tars/config.yaml` when set.

- **Test command** - what the Hub's verification gate runs. Prefer the script the repository itself treats as its full suite (`test:coverage`, `check`, `devenv test` entry, `cargo test`, `pytest`, `go test ./...`, `dotnet test`, …).
- **Install command** - what a fresh clone needs before it can build or typecheck. Use the **lockfile-respecting** form when the ecosystem has one (`bun install --frozen-lockfile`, `npm ci`, `cargo fetch --locked`, `uv sync --frozen`, `go mod download`). Empty is valid when there is no install step.
- **Hook command** - whole-repo hooks the gate runs before tests:

  | Repository contains                                   | Hook command                          |
  | ----------------------------------------------------- | ------------------------------------- |
  | `.pre-commit-config.yaml`, `prek` on PATH             | `prek run -a`                         |
  | `.pre-commit-config.yaml`, `pre-commit` only (legacy) | `pre-commit run --all-files`          |
  | `lefthook.yml` / `lefthook.yaml`                      | `lefthook run pre-commit --all-files` |
  | `.husky/`                                             | the script the hook itself runs       |
  | none of the above                                     | empty - the gate runs tests only      |

  An empty hook command is a legitimate answer. The gate substitutes `:` for empty steps.

**Devenv projects:** if `devenv.nix`, `devenv.yaml`, or `devenv/default.nix` is present, **do not invent enter flags here.** Read and follow the [devenv](../../tooling/devenv/SKILL.md) skill, then bake its non-interactive enter recipe into the **opaque** install/hooks/test strings (or a single outer wrapper that those strings already include). Other skills only point at devenv; prepare is the only backlog phase that expands it.

> [!IMPORTANT]
> **A devenv command with no secrets reason dies before it starts anything.** SecretSpec's `require_reason` policy defaults to `"agents"` **even when the key is absent from `secretspec.toml`**, so any command entering a devenv shell without `SECRETSPEC_REASON` fails with _"Accessing secrets requires a reason"_. `tars-gate` `eval`s these frozen strings blind and cannot add the missing variable, so the omission surfaces as a red gate that never ran a test - and it is discovered at gate time, several minutes into a batch, not here.
>
> Every frozen string that enters a devenv shell (`TARS_INSTALL_COMMAND`, `TARS_HOOK_COMMAND`, `TARS_TEST_COMMAND`, and `TARS_CI_COMMAND` if it enters one) **must** carry, per the [devenv](../../tooling/devenv/SKILL.md) skill's non-interactive shape:
>
> ```bash
> CI=true SECRETSPEC_PROVIDER=env SECRETSPEC_ENV=<ci-if-defined-else-default> \
>   SECRETSPEC_REASON="<why you are running this>" \
>   devenv --no-tui shell --quiet -- <command>
> ```
>
> - `SECRETSPEC_REASON` - **mandatory**; a short human-readable purpose, e.g. `"backlog gate: full test suite"`. There is no default and no way to omit it.
> - `SECRETSPEC_PROVIDER=env` - no interactive authorization prompt.
> - `SECRETSPEC_ENV` - `ci` when `[profiles.ci]` exists in `secretspec.toml`, else `default`, else the first defined profile. Omit only when there is no `secretspec.toml`.
> - `CI=true` - match CI's install/auto-dependency behaviour.
>
> Freeze the same values into `run.env` (`TARS_SECRETSPEC_REASON`, `TARS_SECRETSPEC_PROVIDER`, `TARS_SECRETSPEC_ENV`) so a later reader can see what the opaque strings carry without parsing them. The variables inside the frozen command strings are what actually take effect; the `run.env` keys are the record.
>
> Before writing any command string, **read it back and confirm the reason is present**. This is the single most common way a devenv repository produces a gate that is red for a reason unrelated to the code.

**Then check for overlap, and subtract it.** Hook runners frequently include a hook that already runs the test suite - so a naive gate of `<hooks> && <tests>` runs the whole suite twice. Inspect the hook config: if a hook already covers typecheck, lint, build, or test, set the test command to only what the hooks do **not** cover. A common residue is a coverage-threshold run, which a plain test invocation does not enforce.

> This matters more than ordinary waste, because the gate runs **while holding the mutex**. Every second spent re-running an already-green suite is a second no other spoke can test.
>
> Check whether the repository suppresses automatic dependency installation under CI-like env - several devenv setups do. Where it does, the install command is mandatory rather than a convenience.

#### 4e. Land commit template

Resolve `land.subject_template` from config (default `chore(backlog): land ticket {{id}}`).

If the repo enforces conventional commits (commitlint config, convco, a `commit-msg` hook that rejects `merge(` subjects), keep a conventional template. If there is no commit policy, a plain `Land ticket {{id}}` is fine.

Detect by reading common config files and hook samples - do not require a network. Freeze the template string into `run.env` as `TARS_LAND_SUBJECT_TEMPLATE`. Implement only substitutes `{{id}}` / `{{title}}`.

#### 4f. CI check resolution

Resolve whether the Hub must confirm CI after each batch:

1. If `ci.check` is `true` or `false` in config, honour it.
2. If `null` (default): turn **on** when a remote exists, the topic branch appears pushable/pushed, and a usable check is available (for example `gh` authenticated against a GitHub remote with Actions, or `ci.command` set). Otherwise **off**.
3. Freeze:
   - `TARS_CI_CHECK=0|1`
   - `TARS_CI_CHECK_REASON="…"` (why on or off)
   - `TARS_CI_COMMAND="…"` when on (opaque; Hub runs it for the batch head)
   - `TARS_CI_BLOCK_ON_RED=0|1` from `ci.block_on_red` (default on)

When off, implement still documents that local green ≠ CI green, but does not block the loop.

#### 4g. Baseline gate smoke (prove the recipe)

Before freezing commands, **run the real gate once** on the clean topic branch in the parent workspace - the same path hub will use:

1. Write a **draft** `.tars/run.env` with the paths and candidate commands (weaken flags off).
2. **On a devenv project, check the candidate strings before running anything.** Every command that enters a devenv shell must already carry `SECRETSPEC_REASON` (plus `SECRETSPEC_PROVIDER=env`, `SECRETSPEC_ENV` where a `secretspec.toml` exists, and `CI=true`) - see step 4d. A missing reason is **red: fix it before proceeding**, not something to discover when the gate runs. It costs one read now and a full failed gate cycle later.
3. Export `PRE_COMMIT_HOME` to `$TARS_SPOKE_ROOT/hook-cache` when the hook runner uses it.
4. Run:

   ```bash
   sh "$TARS_GATE" "$REPO_ROOT"
   ```

5. Classify the result:

   | Outcome                                                                                                                             | Action                                                                                                                                                                                                                                                                                                                                       |
   | ----------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
   | **Green**                                                                                                                           | Freeze commands. `TARS_GATE_WEAKENED=0`.                                                                                                                                                                                                                                                                                                     |
   | **Red - environment / tooling** (missing secrets reason, secrets prompt, missing toolchain, install failure, quoting, devenv enter) | **Fix or abort.** Do not weaken the suite to hide this. A failure naming a missing reason (_"Accessing secrets requires a reason"_) means a frozen string lost its `SECRETSPEC_REASON` - repair the string, do not work around it. Re-read the [devenv](../../tooling/devenv/SKILL.md) skill if the project uses devenv.                     |
   | **Red - pre-existing product / coverage / known baseline**                                                                          | Prefer narrowing `TARS_TEST_COMMAND` to the green subset the repo can honestly run. Only if `gate.allow_weaken` is true and no useful subset exists, set `TARS_TEST_COMMAND` to `:` (or the narrowed command), `TARS_GATE_WEAKENED=1`, and a precise `TARS_GATE_WEAKENED_REASON`. Never write a no-op test command without both weaken keys. |
   | **Red - ambiguous / flaky**                                                                                                         | Re-run the full gate **once**. Still red → report output and **ask the user** rather than auto-weaken.                                                                                                                                                                                                                                       |

6. Re-write the final `.tars/run.env` with the frozen values. If the smoke left the working tree dirty (hook autofix), restore cleanliness before finishing prepare: the tree must be clean at the end of prepare (`git status --porcelain` empty). Prefer restoring autofixes with `git checkout -- .` / `git clean` only for smoke dirt **you** caused; if unsure, abort and ask the user.

Report the smoke outcome and any weaken banner prominently in the prepare summary.

#### 4h. Record the resolved values

Write the results to `.tars/run.env` in the parent workspace:

```sh
TARS_SPOKE_ROOT="…"
TARS_CLONE_MODE="hardlink"   # or "shared"
TARS_LOCK="…/resources/manual/tars-lock"
TARS_GATE="…/resources/manual/tars-gate"
TARS_SPOKE="…/resources/manual/tars-spoke"
TARS_HEAVY_LOCK="…/locks/<repo-name>.heavy"
TARS_TOPIC_BRANCH="…"
TARS_PRE_COMMIT_HOME="…/hook-cache"   # may be empty if unused
TARS_INSTALL_COMMAND="…"   # opaque; may be :
TARS_HOOK_COMMAND="…"      # opaque; may be :
TARS_TEST_COMMAND="…"      # opaque; may be : only if weakened
# Secrets context baked INTO the three opaque strings above on a devenv project.
# Recorded here so a reader can see what they carry without parsing them; the
# copies inside the command strings are what actually take effect.
# A devenv command with no reason fails before it runs anything - see step 4d.
TARS_SECRETSPEC_REASON="backlog gate: …"   # mandatory on devenv; never empty
TARS_SECRETSPEC_PROVIDER="env"             # non-interactive; no authorization prompt
TARS_SECRETSPEC_ENV="ci"                   # or "default"/first profile; empty if no secretspec.toml
# Optional pre-dispatch ticket check. Empty = none configured; implement then
# skips the check and LOGS that it skipped. Invoked with the batch number,
# run against the working tree, non-zero exit = batch has errors.
TARS_TICKET_LINT_COMMAND=""
TARS_GATE_WEAKENED=0       # or 1
TARS_GATE_WEAKENED_REASON=""  # required when weakened
TARS_LAND_SUBJECT_TEMPLATE="chore(backlog): land ticket {{id}}"
TARS_CI_CHECK=0            # or 1
TARS_CI_CHECK_REASON="…"
TARS_CI_COMMAND=""         # opaque when CI check on
TARS_CI_BLOCK_ON_RED=1
TARS_HEAVY_COMMANDS="test,coverage,run -a,--all-files,nix build"
# Comma-separated globs from review.always_full (frozen so implement never opens config)
TARS_REVIEW_ALWAYS_FULL="hooks/**,**/*secret*,**/auth/**"
```

Write these to disk rather than only holding them in context. A backlog run is long enough that the Hub's context may be compacted partway through, and a Hub that has forgotten where its spoke root is will resolve a different one mid-run. `.tars/` is already shared into every spoke workspace, so spokes can read the same file.

### 5. Clean Up Orphaned Workspaces

1. **Legacy worktrees**: earlier versions of these skills gave spokes git worktrees inside or beside the repository. Remove any that remain, which also migrates a repository off the old model:

   ```bash
   git worktree list --porcelain
   git worktree remove --force <path>   # for each spoke worktree
   git worktree prune
   ```

   Removing a worktree does not delete its branch, so no spoke work is lost here - branch cleanup is sub-step 3 below, and it protects rework branches explicitly.

2. **Stale spoke clones**: delete any directory under `$TARS_SPOKE_ROOT` left behind by a previous run. Every clone is disposable scratch; spoke work durably lives on branches in the parent repository, so nothing is lost. Confirm each directory is a clone of this repository before deleting it, and never delete the spoke root itself.

3. **Leftover branches**: force-delete all local branches matching `subagent-*`, EXCEPT those currently referenced by active rework tickets in `.tars/issues/todo/`.
   - Scan `.tars/issues/todo/*.md` files to extract the `branch` field from the frontmatter.
   - List all subagent branches: `git branch --list 'subagent-*'`
   - Force-delete only the branches that are NOT referenced in the active rework list: `git branch -D <branch-name>`

Once these steps are complete and verified, the repository is ready for the `tars-backlog-loop` skill. If `TARS_GATE_WEAKENED=1`, the prepare report **must** end with a known-residual banner quoting `TARS_GATE_WEAKENED_REASON`.

## Portable Command Baseline

Every command this pipeline emits runs on both Linux and macOS. Three GNU-only idioms are easy to reach for and silently misbehave on BSD userland:

- `sed -i` - takes a mandatory suffix argument on BSD/macOS. Write through a temporary file and `mv` it into place instead.
- `readlink -f` - GNU-only. Use `realpath`, or resolve the path with `cd` and `pwd -P`.
- `stat -c` - GNU spelling; BSD uses `-f`. For filesystem identity use `df -P` instead.

## Related Skills

- [devenv](../../tooling/devenv/SKILL.md) - when the project has `devenv.nix` / `devenv.yaml`, follow it to build non-interactive enter commands during step 4d.
- [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md) - consumes `run.env` and runs `tars-gate` / `tars-spoke`.
