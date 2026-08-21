+++
title = "tars-backlog-implement"
description = "Implement pending backlog issues from `.tars/issues/todo/` in parallel, conflict-free batches using isolated clones. Reach for this when asked to implement backlog issues, execute tasks from tickets in parallel, or resolve the issue queue."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Backlog Implement

Implement pending issue tickets from `.tars/issues/todo/` in parallel, conflict-free batches using isolated clones, verifying each one before it is merged back into the active topic branch.

This skill operates in a Hub-and-Spoke topology, spawning implementation subagents ("spokes") in parallel, while the Hub manages sequencing, verification, and merges.

## Targets and Paths

- Target Folders: `.tars/issues/{todo,done,failed,wont-do}/` relative to project root. `wont-do/` holds retired and superseded tickets; nothing is ever dispatched from it, and a dependency pointing into it can never be satisfied.
- Ticket status updates are written to disk only. Ticket files are never staged, committed, or force-added in git.
- This skill requires `tars-backlog-prepare` to have run first. Read **only** `.tars/run.env` for run facts (paths, opaque commands, land template, CI flags, weaken banner). Do **not** re-open `.tars/config.yaml` for gate/land/CI - prepare already froze policy into `run.env`. Re-read `run.env` rather than remembering values so a compacted Hub context cannot drift mid-run.
- **If `.tars/run.env` does not exist, do not proceed and do not improvise.** Read [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) and execute its steps inline, then continue. See **Invoking Sibling Skills** below for why you cannot simply call it.
- **If `.tars/run.env` exists but is stale, treat it as missing.** Existence is the wrong test: a run.env frozen before the last `.tars/config.yaml` change is silently missing policy. At run start, compare `TARS_CONFIG_FINGERPRINT` from `run.env` against `cksum .tars/config.yaml` (both empty when there is no config file is a match). On mismatch, do **not** hand-patch `run.env` - re-run prepare inline, exactly as if the file were absent, and say so in the run notes.
- Runners ship beside this skill (always invoke as `sh <path>`):

  | Binary                        | Who         | Role                                                                              |
  | ----------------------------- | ----------- | --------------------------------------------------------------------------------- |
  | `resources/manual/tars-gate`  | Hub only    | Full gate: lock → cd → install → hooks → tests from `run.env`                     |
  | `resources/manual/tars-spoke` | Spokes only | Lock → cd → exec caller args (targeted heavy tests)                               |
  | `resources/manual/tars-lock`  | Internal    | Mutex used by the two runners; hubs/spokes should not assemble lock lines by hand |

## Invoking Sibling Skills

These skills are marked **user-invoked** - in Claude Code that is `disable-model-invocation: true`; other runtimes spell it differently. Wherever that marking is honoured, the effect is the same: only the user typing the skill's name can invoke it, and **no skill can invoke another**. So a "call `tars-backlog-<phase>`" instruction will simply be refused.

**When it is refused, read that skill's `SKILL.md` and execute its steps inline.** Each call site gives the path. If your runtime does permit skill-to-skill invocation, calling it directly is equivalent and fine.

Keeping the marking costs nothing at rest; removing it would load all seven descriptions into every session's context permanently, for skills that are only ever driven deliberately. Treat a refusal as something to route around, never as a reason to skip the step.

Skipping `tars-backlog-prepare` in particular leaves the run with no spoke root, no clone mode, no gate recipe, and no integrity check.

## Topic Branch Workflow (Hub Only)

All backlog operations run from a topic branch (never the default branch). Every spoke branches off this active topic branch, and all approved changes are merged back into it. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) for the full policy and commands.

## The Two Contention Rules

Everything below follows from two facts about running several agents against one repository. Both are load-bearing; violating either produces failures that look like flaky code rather than orchestration bugs.

1. **A spoke must never be able to write to the parent's git state.** `.git/hooks/` and `.git/config` are shared by every linked worktree of a repository, so a spoke that installs git hooks in a worktree rewrites them for the parent and all its siblings. Giving each spoke its own **clone** makes the whole class of shared-state collisions structurally impossible rather than merely forbidden.

2. **Concurrent heavy commands produce false test failures, not just slow ones.** Test suites that spawn processes, bootstrap temporary directories, or carry timeouts fail under CPU starvation and pass in isolation. A false failure is worse than a slow one: it sends correct work into the rework loop. So heavy commands are serialised by a mutex, while cheap deterministic checks stay fully parallel.

| Class                    | Examples                                                      | How it runs                                    |
| ------------------------ | ------------------------------------------------------------- | ---------------------------------------------- |
| Deterministic under load | Typecheck, formatters, linters, hooks scoped to changed files | Freely, in parallel, unlocked                  |
| Starvation-sensitive     | Test suites, whole-repo hook runs, nix/container builds       | Via `tars-spoke` (spokes) or `tars-gate` (hub) |

## Heavy commands: mechanical runners, not hand-rolled locks

**Hub full verification** - always:

```bash
# Optional when the hook runner uses a shared cache:
PRE_COMMIT_HOME="<TARS_PRE_COMMIT_HOME>" \
  sh "<TARS_GATE>" "<spoke-dir-or-repo-root>"
```

**Spoke targeted tests** - always:

```bash
PRE_COMMIT_HOME="<TARS_PRE_COMMIT_HOME>" \
  sh "<TARS_SPOKE>" -- <targeted-test-command>
```

Do **not** assemble nested `sh -c` + lock + install chains in prompts. Prepare already froze opaque commands into `run.env`; the runners source that file.

Bare project test runners without `tars-spoke` are **unsupported** for spokes. Whole-repo hook runs and the full suite are **hub-only** via `tars-gate`.

The heavy-command substring list in `run.env` (`TARS_HEAVY_COMMANDS`) is advisory for "when in doubt, use `tars-spoke`". Prefer over-wrapping: waiting is cheap; a false failure is not.

### Leaked workers defeat the mutex

Test runners spawn worker processes, and a worker that outlives its parent is **reparented to init**. It then holds no lock while saturating cores. Defences:

1. **`tars-lock` (inside the runners) contains and reaps** process groups on exit.
2. **Spokes must not background heavy commands.** Run them in the foreground and let them exit.
3. **The Hub sweeps between batches** (prose + examples - no separate helper this version). Before starting a new batch, look for processes matching heavy substrings that belong to no live spoke, and stop them (`SIGTERM`, then `SIGKILL` if needed).

Example sweep shape (adapt patterns to the repo; never kill the hub or unrelated user jobs):

```bash
# List candidates; inspect before killing.
pgrep -af 'bun test|pytest|cargo test|prek run' || true
# After confirming a PID is an orphan from this backlog run:
# kill -TERM <pid>; sleep 2; kill -KILL <pid>   # only if still alive
```

> Sweep even when the batch looked clean. Leaks scale with batch size and surface as "flaky suite under load".

## The Verification Gate

The Hub runs the gate twice - once per spoke inside its clone, once on the topic branch after the batch - **only** via `tars-gate` and the commands prepare recorded. Do not re-derive or shorten those commands.

```bash
PRE_COMMIT_HOME="<TARS_PRE_COMMIT_HOME>" \
  sh "<TARS_GATE>" "<spoke-dir>"
```

**Read the result from the verdict line, never from pipes.** Both runners end with a machine-parseable last line on stdout - `TARS_GATE_RESULT=<exit> step=<install|hooks|tests|ok> target=<dir> log=<path>` from `tars-gate`, `TARS_SPOKE_RESULT=<exit> target=<dir> log=<path>` from `tars-spoke` - and exit with the wrapped command's status. Parse that line. Piping runner output through `tail`, `grep`, or a pager and then reading `$?` reports the pipe's exit status, not the gate's, and has misreported a red gate as green.

**The full output survives in the log.** Both runners tee everything they ran to `$TARS_SPOKE_ROOT/logs/` and name the file in the verdict line, so a diagnosis lost to a pipe or a truncated harness capture is always recoverable - read the log instead of re-running the gate to see the failure again. Cite the log path in ticket Evidence and rework notes; it is the durable record of what the gate actually saw.

**Gate always installs first** (when `TARS_INSTALL_COMMAND` is not `:`) so lockfile moves cannot leave stale dependency trees - see _Installed dependencies go stale_ below.

**Weakened gates:** if `TARS_GATE_WEAKENED=1`, every green result is "gate command green (weakened: …)", never "full suite green". Surface `TARS_GATE_WEAKENED_REASON` in batch and final reports.

**Devenv:** if the project has `devenv.nix` / `devenv.yaml`, prepare already followed the [devenv](@/skills/tooling/devenv/_index.md) skill when freezing commands. Implement does not restate secrets or CI env vars - it only runs `tars-gate` / `tars-spoke`.

### Dirty gate (hook autofix)

Whole-repo hooks may **rewrite** files (`eslint --fix`, formatters). A green gate can leave the clone dirty. That is expected:

1. After a green `tars-gate`, run `git status --porcelain` in the gated tree.
2. If dirty: commit autofixes on the **spoke branch** (conventional commit; do not stage `.tars/`), re-fetch the branch into the parent, then continue to review/merge.
3. **Never** review or merge a dirty tree. **Never** leave autofixes only in a disposable clone.

### Installed dependencies go stale

Installed dependencies are derived from the lockfile, and **git moves the lockfile without touching** dependency directories. Any operation that changes the manifest invalidates what is installed.

| Moment                                           | Why it goes stale                                                      |
| ------------------------------------------------ | ---------------------------------------------------------------------- |
| Spoke starts in a fresh clone                    | Nothing is installed at all                                            |
| Spoke merges the topic branch to sync            | A sibling ticket may have added a dependency since this clone was made |
| Parent, after merging a dependency-adding ticket | Parent's installed tree predates the merge; batch-final gate fails     |

Putting install inside `tars-gate` covers all three. Re-run install (or full gate) after step 4.1's sync merge too.

### Flake policy (classify, then act)

On a **red** gate, do **not** immediately burn a spoke fix round:

1. **Classify.** Only **transient-shaped** failures qualify for the flake path: timeouts, killed/runner infrastructure, single-hook flake with no assertion/compile/lint failure. Hard `expect` failures, type errors, and lint errors → real failure path (spoke fix).
2. **Isolate.** Re-run the failing test or hook alone under the mutex (`tars-spoke` for a single test; or a narrowed command).
3. **Isolation red** → real failure → spoke fix rounds.
4. **Isolation green, sole failure** → accept **without** a second full gate when all three hold: (a) the flaky test was the **only** failure in the gate run, (b) it is green in isolation this run, and (c) the rest of that same gate run was green. Name the acceptance in the batch report ("accepted `<test>` as flake: sole failure, isolation green"). If the same test earns this acceptance more than once in a run, file a ticket for it - a recurring flake is product debt, not gate noise. On each acceptance, append the test's name to `TARS_KNOWN_FLAKES` in `.tars/run.env` (comma-separated). This is the **one** key the Hub may update after prepare - run.env is otherwise read-only - and it exists so every later gate, spoke, and review in this run recognises the flake instead of re-diagnosing it independently. It resets naturally: prepare rewrites run.env, so a flake never outlives the run that observed it.
5. **Isolation green, other failures present** → one full `tars-gate` re-run under the mutex. Green → accept. Red → real failure path.
6. **Cap:** at most **one** full re-gate per gate attempt. No flake loops, and no silent acceptances - every flake path taken appears in the batch report.

### Post-conflict smoke

After any merge conflict resolve that is **not** a shared-append union, before full gate:

1. Run targeted typecheck / unit tests for touched modules via `tars-spoke` (under the mutex).
2. Only if that smoke is green, run full `tars-gate`.

Semantic merges of the same function are easy to get wrong; the full gate is a late place to discover a missing symbol.

## Implementation Workflow

### 1. Backlog Scan & Conflict-Free Batching

> This section is the **canonical** statement of the batching rules for the whole pipeline. `tars-backlog-audit`, `tars-backlog-triage`, and `tars-backlog-create-issue` cite it rather than restating it, and tickets must not restate it either - a duplicated rule set disagrees with the original the first time either changes.

1. Scan the `.tars/issues/todo/` directory for ticket markdown files.
2. Analyze `files:`, optional `owns:`, `dependencies:`, and soft-ownership signals in the ticket body.
3. Dynamically group tickets into batches. A batch is admissible only if **all four** rules hold:
   1. **Path rule.** Within one batch, no two `status: todo` tickets share **any** path, across `files:` **or** `owns:`. The comparison is over the union of both lists, on the **normalised path**, so all of these collide:

      | Ticket A             | Ticket B             | Collide?                                  |
      | -------------------- | -------------------- | ----------------------------------------- |
      | `files: src/a.ts`    | `files: ./src/a.ts`  | yes - normalise the path before comparing |
      | `owns: src/a.ts`     | `owns: src/a.ts#Foo` | yes - a symbol lives inside its file      |
      | `files: src/a.ts`    | `owns: src/a.ts#Foo` | yes - `files:` × `owns:` cross-check      |
      | `owns: src/a.ts#Foo` | `owns: src/a.ts#Bar` | yes - same file, two writers              |

      Strip the `#Symbol` suffix before comparing, and compare paths after normalising them (resolve `./`, collapse duplicate separators, use one consistent separator). Two tickets editing the same file in the same batch is the collision this whole design exists to prevent; a `#Symbol` suffix narrows _intent_, not the file the writes land in.

   2. **Dependency rule.** Every dependency of a ticket must sit in a **strictly earlier** batch. It may not be in the same batch, and it may not be unmerged (`todo/`, `failed/`, or `wont-do/`).

   3. **Size rule.** At most **5** `status: todo` tickets per batch. Count only tickets that will actually dispatch a spoke: a parked ticket that spawns nothing occupies no slot. (`status: rework` **does** dispatch a spoke here by design - see the rework spawn path in step 2 - so it does occupy a slot.)

   4. **Empty-footprint rule.** A ticket with a missing or empty `files:` list **fails** the path rule; it does not pass it. Either derive its `files:` list first, or schedule it **alone**.

   **Soft dependencies:** if two tickets clearly add or own the same export/shared constant (from body, `owns:`, or triage notes) but lack a hard `dependencies:` edge, **do not batch them together** - land the natural owner first (or the lower id if unclear), then the other. Prefer adding an explicit `dependencies:` entry when editing frontmatter. Write it as an inline array on one line (`dependencies: [12, 19]`); a multi-line YAML list parses as **empty** and the edge disappears without any error.

   > `component:` is not a substitute for `files:` or `owns:`. Use it to suspect collisions, never to clear them.

#### `batch:` is the plan; the dependency check is the re-verification

These two statements are the **same invariant observed at two different times**, not a contradiction to be reconciled by deleting one of them:

- **`batch:` is the static plan.** A dependency must land in a strictly earlier batch number.
- **The dependency rule is the runtime check.** At dispatch time, a dependency must already be **merged**.

Batches execute in ascending order with merges in between, so when batch N dispatches, batches 1…N-1 are already merged and both statements hold at once. If they ever disagree, the plan is stale - re-batch; do not relax either rule.

#### Shared append-only files

Some files every ticket may need to touch without knowing it in advance (spellcheck dictionary, changelog, barrel re-export, i18n catalogue, lockfile). Read `worktree.shared_append_files` from prepare-time knowledge / ticket practice; the list is configured in `.tars/config.yaml` and applied at merge time.

For a file on that list, a merge conflict is **expected and not a rework trigger**. The Hub resolves it by taking the union of both sides, applying the file's ordering convention, and continuing. **Do not** extend this exception to ordinary source files.

Then continue batch bookkeeping:

1. Update ticket frontmatter with `batch: X` (bare integer, never quoted) and write to disk.

   When a ticket's `batch:` changes, **re-verify anything that referenced the old value**: the `` `batch: N` - rationale `` bullet in that ticket's `## Review`, and any `dependencies:` edge pointing at it. Never write a batch number into ticket prose outside that one Review bullet - `batch:` is reallocated every run and reset to `null` on rework, so prose batch numbers ("this lands in batch 4", "#660 is batch 3") are stale almost immediately. Express the relationship as a `dependencies:` edge, which is machine-checkable and cannot drift.

2. **Resolve tickets the dependency rule can never admit** before dispatching anything:
   - Dependency in `failed/` or `wont-do/` → move the dependent to `failed/` with a note naming the blocking ticket. `wont-do/` holds retired and superseded tickets; an edge into it can never be satisfied, so a dependent left in `todo/` would spin the loop forever.
   - Dependency cycle → fail every member of the cycle, and say so in the report. **Never proceed silently** by breaking the cycle arbitrarily or dropping one of its edges: a cycle is a metadata defect that needs a human, and quietly picking a winner ships the tickets in an order nobody chose.
   - **Dependency on an ID that appears to be missing → normalise before concluding it is missing.** IDs are written unpadded in frontmatter (`9`) and padded in filenames (`009.md`), so a naive string comparison reports a real dependency as absent and the edge silently disappears - a fail-open in exactly the place that must fail closed. Compare numerically: parse both sides as integers and match on the number. Only after that normalisation fails is the ID genuinely missing; treat that as a defect to report, not an edge to wave through.

3. **Pre-dispatch verification (before spawning any spoke).**

   Self-verification by the same agent that did the allocation catches little - it re-runs the reasoning that produced the error. When the run provides a ticket-lint command, use it.

   Read `TARS_TICKET_LINT_COMMAND` from `.tars/run.env` (frozen there by `tars-backlog-prepare`; implement never reads `.tars/config.yaml` directly).
   - **Set** → run it with the batch number, from the repository root, against the working tree. A non-zero exit means the batch has errors: **do not proceed to §2 "Spawn Spokes"**. Repair the reported tickets and re-run until it is green, or park the offending tickets and re-batch.
   - **Empty or absent** → **skip the check and log that you skipped it**, naming the reason ("no `TARS_TICKET_LINT_COMMAND` in run.env"). Most repositories will not provide one; the pipeline must stay usable there. A silent skip is forbidden - a quiet run must never read as a passing check.

   The command must run **after** `batch: X` is written to disk (step 1), because that is the field it reads.

   Whether or not a linter ran, re-verify the four batching rules by hand before dispatch.

> Conflict-free by path is not the same as behaviourally independent. The batch-final gate catches cross-ticket behavioural breaks.

### 2. Spawn Spokes in Isolated Clones

For each ticket in the selected batch, create an isolated clone and spawn an implementation subagent in it.

#### 2a. Create the clone

```bash
SPOKE_DIR="$TARS_SPOKE_ROOT/<TICKET_ID>"
```

```bash
# TARS_CLONE_MODE=hardlink
git clone --branch "$TARS_TOPIC_BRANCH" "$REPO_ROOT" "$SPOKE_DIR"

# TARS_CLONE_MODE=shared
git clone --shared --branch "$TARS_TOPIC_BRANCH" "$REPO_ROOT" "$SPOKE_DIR"
```

In `shared` mode the Hub must **not** run `git gc` or `git prune` in the parent while any spoke is alive.

Branch inside the clone:

- **New ticket**: `git checkout -b subagent-<TICKET_ID>`
- **Rework ticket** (`branch:` set): checkout that branch, then sync topic (see ref hygiene below).

A fresh clone has no hooks installed. Spoke commits are unhooked by design - never `--no-verify`, never install hooks.

**Verify the invariant rather than assuming it.** User-level git configuration can hook a clone at birth: a global `core.hooksPath` in `~/.gitconfig`, or an `init.templateDir` that seeds `.git/hooks/`, gives every fresh clone live hooks the pipeline never installed. After creating the clone, check:

```bash
git -C "$SPOKE_DIR" config --get core.hooksPath || true
ls "$SPOKE_DIR/.git/hooks/" | grep -v '\.sample$' || true
```

Any live hook found here is environment damage, not spoke misbehaviour: remove the shims, warn the user that their global git config hooks fresh clones (the same class of hazard prepare's integrity check covers), and continue. A hooked spoke clone silently breaks the contract everywhere - commits run whole-repo suites that the design says only the Hub's gate runs, and half the "cheap scoped checks only" rule stops being true.

#### 2a-i. Spokes run git through `tars-git`

Give every spoke `resources/manual/tars-git` and require it for **all** git operations:

```bash
sh "$TARS_GIT" checkout -b "subagent-$TICKET_ID"
sh "$TARS_GIT" commit -m "..."
```

`tars-git` resolves the repository git would actually act on and **refuses (exit 65)** if it
is not a clone under `$TARS_SPOKE_ROOT`. It also passes `-c commit.gpgsign=false`, because a
signing prompt (e.g. 1Password `op-ssh-sign`) needs a human click and will hang a headless
spoke while it holds the heavy-command mutex.

This is structural, not advisory, and it exists because advice failed. On 2026-08-11 three
spokes created branches - and one committed - in the developer's live repository. Each time a
`cd "$SPOKE_DIR"` had failed while `set -e` was suppressed by a pipeline or a redirect, so the
next bare `git` ran against whatever tree the shell was already in. The spoke brief was
tightened twice, including an explicit "never run git against the main repo" rule, and it
happened again - because the fault is a script falling through, not an agent disobeying.

The Hub does **not** use `tars-git`; it legitimately fetches, merges and commits in the repo
root. `tars-gate` and `tars-spoke` carry the weaker form of the same check: they refuse any
target that is neither the run's repo root nor a spoke clone.

**The Hub must therefore disable commit signing itself, on every commit it makes.** `tars-git`
does this for spokes; nothing does it for the Hub. With `commit.gpgsign=true` and a signer that
prompts for approval (for example `gpg.ssh.program=op-ssh-sign`), a Hub merge opens a window a
human has to click, and a headless run hangs there - while holding the heavy-command mutex, so
every spoke waiting to test hangs behind it too. Pass `--no-gpg-sign` (or
`git -c commit.gpgsign=false …` where the subcommand has no such flag) on the land merge, on any
revert, and on any other Hub-side commit.

Never confuse this with `--no-verify`. Signing is skipped deliberately; `--no-verify` is
prohibited everywhere in this pipeline. But do not lean on hooks to police Hub commits
either: **merge commits run no `pre-commit` hook.** Git fires only `commit-msg` (and
`pre-merge-commit`, which hook managers do not install) on the landing path, so the Hub's
`git merge --no-ff` lands content no hook ever saw - not because anything was bypassed, but
because git never runs `pre-commit` there. Do not "fix" this by installing hooks (this
pipeline forbids installing hooks everywhere); the **batch-final gate is the backstop** that
re-checks the whole merged tree, which is why it is never skippable (see step 5).

#### 2b. Transfer gitignored files

Defaults (overridable in config, applied by hub from prepare knowledge):

```yaml
worktree:
  transfer_files:
    - ".pre-commit-config.yaml"
    - ".env*"
    - ".tars/"
    - "devenv.local.nix"
    - "devenv.local.yaml"
```

- **Symlink (preferred)** parent → clone. Symlinking `.tars/` shares the issue queue and `run.env`.
- **Copy (fallback)**; if `.tars/` was copied, copy the ticket file back before deleting the clone.

Exclude transferred names in the clone:

```bash
printf '%s\n' .pre-commit-config.yaml .env .tars devenv.local.nix devenv.local.yaml \
  >> "$SPOKE_DIR/.git/info/exclude"
```

#### 2c. Bootstrap dependencies

Tell the spoke to bootstrap with the opaque `TARS_INSTALL_COMMAND` from `run.env` (may already include devenv enter). Never symlink dependency directories across clones.

#### 2d. Share one hook cache

When `TARS_PRE_COMMIT_HOME` is set, export it for every spoke and hub gate:

```bash
PRE_COMMIT_HOME="$TARS_PRE_COMMIT_HOME"
```

#### 2e. Spawn the subagent (minimal contract)

Spawn each spoke with:

- **Role**: `Implement-<TICKET_ID>`
- **Prompt** (default - do **not** paste the full ticket body):

  ```text
  You implement one backlog ticket in an isolated clone.

  Workspace: <SPOKE_DIR> (branch already checked out; origin = parent repo)
  Ticket file (read it fully): <REPO_OR_SPOKE>/.tars/issues/todo/<TICKET_ID>.md
  Run facts: source/read .tars/run.env (do not invent paths or gate commands)

  Rules:
  1. Read the ticket (Tasks, AC, Review, Implementation Review). Implement it; address rework feedback if any.
  2. Bootstrap with TARS_INSTALL_COMMAND from run.env before verifying.
  3. Cheap checks only on YOUR changed AND newly created files (typecheck/format/lint/hooks
     scoped to those paths). Run the repo's own scoped hook command over them before
     reporting - anything only the whole-repo gate checks costs a full gate cycle when it
     catches you, so catch it here in seconds. Never whole-repo hooks.
  4. Heavy tests ONLY via: sh "<TARS_SPOKE>" -- <targeted test command>
     Never bare test runners. Never full-suite / whole-repo prek - Hub runs tars-gate.
     tars-spoke execs your argv, so VAR=x prefixes are not commands - use env VAR=x <cmd>.
     Its last stdout line (TARS_SPOKE_RESULT=<exit> ... log=<path>) is the result: parse
     that, never a piped $?, and cite the log path in Evidence.
     Check TARS_KNOWN_FLAKES in run.env first: a failure ONLY in a listed test, green when
     re-run in isolation, is a known flake - note it and move on, do not diagnose it as
     your ticket's failure.
  5. Never install git hooks. Never merge into topic/default. You may `git merge` topic INTO
     your branch to sync - always with an explicit conventional -m subject, never git's default.
  6. Conventional commits; never --no-verify; never stage anything under .tars/.
  7. Update Tasks/AC checkboxes and ## Evidence on the ticket file.
  8. NEVER copy a 4-or-more-digit line number out of the ticket into a source or test file -
     not in a name, a comment, or fixture data. Repositories commonly scan test sources for
     bare long digit runs (mock IDs, fixture keys) and cannot tell a line reference from a
     real identifier, so a pasted coordinate reds the gate from an unrelated file. Refer to
     the symbol by name instead.
  9. Report completion and STAY AVAILABLE until Hub says the ticket is resolved.
  10. Tool args: never wrap paths in nested escaped quotes.
  ```

- **Add checkpoint protocol** when `complexity: high` or `status: rework`:

  ```text
  Checkpoint: commit incremental work on your branch. If stuck after repeated failed approaches,
  stop, leave commits + Evidence notes, and report BLOCKED with what you tried - do not burn a
  long unproductive loop. Hub may resume or rework.
  ```

### 3. Monitor Spokes

Do **NOT** wait passively for the whole batch.

- Periodically check spoke liveness via the runtime's subagent status - never infer completion from log keywords (`error`/`fail`/`done` appear in passing suites).
- Approval-blocked spokes: warn the user explicitly.
- A spoke waiting on the mutex is not stuck.
- Prefer `SIGTERM` over `SIGKILL` when stopping a spoke.

### 4. Verify, Then Merge (Hub Only)

Process each spoke as it reports. **Gate in the clone before merge.** Parent tree stays pristine.

For each completed spoke:

1. **Sync topic into the spoke (ref hygiene).** Force-update the remote-tracking topic ref, then merge:

   ```bash
   git -C "$SPOKE_DIR" fetch origin "+refs/heads/<topic>:refs/remotes/origin/<topic>"
   git -C "$SPOKE_DIR" merge --no-gpg-sign \
     -m "chore(backlog): sync <topic> into subagent-<TICKET_ID>" "origin/<topic>"
   ```

   Prefer this force-refspec over a plain `git fetch` that can leave a stale/broken `origin/<topic>` after many hardlink-clone cycles.

   Always pass `-m` with a conventional subject on sync merges (and any other merge this pipeline makes): git's default `Merge branch '…'` subject is exactly what a conventional-commit `commit-msg` policy rejects, and a rejected merge subject leaves the merge half-done at the worst possible moment. `--no-gpg-sign` for the same headless-signing reason as every other pipeline commit.

   **Repair (once) if fetch fails** with unable-to-lock-ref / dangling / corrupt remote-tracking ref:

   ```bash
   # Remove only the bad remote-tracking ref, then retry the force fetch once.
   git -C "$SPOKE_DIR" update-ref -d "refs/remotes/origin/<topic>" 2>/dev/null || true
   # If packed-refs holds a stale line for that ref, edit carefully or:
   git -C "$SPOKE_DIR" pack-refs --all 2>/dev/null || true
   git -C "$SPOKE_DIR" fetch origin "+refs/heads/<topic>:refs/remotes/origin/<topic>"
   ```

   If still failing, surface to the user - do not mass-edit packed-refs.

   After sync, re-run install (or rely on the upcoming gate's install step). Spoke resolves its own conflicts when possible. After any **non-append** conflict resolve → **post-conflict smoke**, then gate.

2. **Capture durably** in the parent:

   ```bash
   git fetch "$SPOKE_DIR" "+subagent-<TICKET_ID>:subagent-<TICKET_ID>"
   ```

   Always, even if the gate will fail.

   While here, re-check the unhooked-clone invariant: `ls "$SPOKE_DIR/.git/hooks/" | grep -v '\.sample$'`. Live hooks at capture time that were absent at clone creation mean the **spoke installed them** despite the contract - flag it in the ticket's review notes and treat the spoke's own verification claims with suspicion (its commits ran checks the design says they must not, or were shaped around them). Do not merge silently over the finding.

3. **Run `tars-gate` on the clone** (with `PRE_COMMIT_HOME` if set). Never `--no-verify`.

4. **Dirty tree after green gate** → commit autofixes on the spoke branch, re-fetch into parent (step 2 again).

5. **Red gate** → flake policy first; then up to **3** in-spoke fix rounds. Exhausting 3 increments `attempts` by **one** (nested counters). Still red → Request Rework path.

6. **Implementation review** (risk-tiered), only on green clean tree:

   | Tier                                                                      | When                                                                                                                                                                                               | What                                                                                                        |
   | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
   | **Lightweight Hub checklist**                                             | Default                                                                                                                                                                                            | AC vs diff; `files:`/`owns:` match touched paths; no `.tars` staged; commit hygiene; spot-check risky hunks |
   | **Full** [tars-backlog-review](@/skills/review/tars-backlog-review/_index.md) | Any of: `risk: high`; diff touches a path matching `TARS_REVIEW_ALWAYS_FULL` globs from `run.env`; **post-conflict** resolve this ticket; `attempts >= 2` / rework; Hub marks security/shared-core | Dual-axis Spec + Standards                                                                                  |

   Gate first, review second.

7. **Verdict**:
   - **Approved**:
     - Merge **sequentially** into the topic branch (never parallel merges).
     - Land commit subject from `TARS_LAND_SUBJECT_TEMPLATE` with `{{id}}` / `{{title}}` filled - e.g. `chore(backlog): land ticket 551`. **Never** use a subject that starts with `merge(` or default `Merge branch 'subagent-…'` if the repo rejects it; set `merge.ff` / message explicitly:

       ```bash
       git merge --no-ff --no-gpg-sign -m "<filled land template>" "subagent-<TICKET_ID>"
       ```

     - Move ticket to `.tars/issues/done/`.
     - Dismiss spoke; delete clone; `git branch -D subagent-<TICKET_ID>`.

   - **Request Rework**:
     - Do not merge. Increment `attempts`. Dismiss spoke.
     - If `attempts >= 5` → `failed/`, delete branch and clone.
     - Else `status: rework`, `batch: null`, keep `branch:`, append `## Implementation Review`, delete clone, **keep branch**.
     - Because `batch:` just changed, re-verify what referenced it: drop or correct this ticket's `` `batch: N` - rationale `` bullet in `## Review`, and check any `dependencies:` edge pointing at this ticket - the dependent can no longer be in a batch that was "later" than a batch this ticket no longer has.

> Every terminal path must dismiss the spoke.

Repeat until the batch is done.

### 5. Batch-Final Gate (Hub Only)

```bash
PRE_COMMIT_HOME="<TARS_PRE_COMMIT_HOME>" \
  sh "<TARS_GATE>" "$REPO_ROOT"
```

**This gate is never skippable**, even when the last per-spoke gate ran minutes ago on a fully synced clone and this run looks redundant. The Hub's land merges run no `pre-commit` hook (see the merge-hook note in step 2a-i), so the batch-final gate is the **only** check that sees the merged tree as it will actually ship. Skipping it reopens exactly the hole the unhooked landing path creates.

Same frozen commands as per-spoke gates. Apply flake policy on red. If still red, identify the offending merge, revert it (`git revert --no-gpg-sign -m 1 <merge-sha>` - Hub commits are unsigned, never `--no-verify`), and return that ticket to rework.

If `TARS_GATE_WEAKENED=1`, banner the reason again in the batch report.

### 6. Confirm CI Agrees (Hub Only)

Read `TARS_CI_CHECK` from `run.env`:

- **`0`**: skip blocking check; record `TARS_CI_CHECK_REASON` in the batch notes.
- **`1`**: run `TARS_CI_COMMAND` (or the prepare-documented check) against the batch head commit **before** the next batch. On red: record workflow/failure on responsible ticket(s). If `TARS_CI_BLOCK_ON_RED=1` (default), **do not** start the next batch until the user unblocks or CI is green.

Local green ≠ CI green. The mutex does not reach CI runners.

### 7. Final report honesty

When the backlog run ends (or each batch, if weakened), the Hub report must include:

- Gate weakened? reason?
- CI checked? result?
- Any flake re-gates accepted?
- Residual known-red items

Never claim "full suite green" when `TARS_GATE_WEAKENED=1`.

## Related Skills

- [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) - integrity, spoke root, command freeze, baseline smoke, `run.env`.
- [tars-backlog-review](@/skills/review/tars-backlog-review/_index.md) - full dual-axis review when risk-tiered rules require it.
- [tars-backlog-create-issue](@/skills/planning/tars-backlog-create-issue/_index.md) - ticket format including `files:` / `owns:`.
- [devenv](@/skills/tooling/devenv/_index.md) - only if you must understand how prepare built commands; implement does not re-enter devenv by hand when `run.env` is present.

