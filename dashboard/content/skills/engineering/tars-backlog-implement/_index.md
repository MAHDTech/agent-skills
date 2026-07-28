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

- Target Folders: `.tars/issues/{todo,done,failed}/` relative to project root.
- Ticket status updates are written to disk only. Ticket files are never staged, committed, or force-added in git.
- This skill requires `tars-backlog-prepare` to have run first. Read `.tars/run.env` for the resolved `TARS_SPOKE_ROOT`, `TARS_CLONE_MODE`, `TARS_LOCK`, `TARS_HEAVY_LOCK`, `TARS_TOPIC_BRANCH`, `TARS_TEST_COMMAND`, `TARS_INSTALL_COMMAND`, and `TARS_HOOK_COMMAND`. Re-read that file rather than remembering its values, so a compacted Hub context cannot drift onto a different spoke root mid-run.
- **If `.tars/run.env` does not exist, do not proceed and do not improvise.** Read [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) and execute its steps inline, then continue. See **Invoking Sibling Skills** below for why you cannot simply call it.

## Invoking Sibling Skills

These skills are marked **user-invoked** — in Claude Code that is `disable-model-invocation: true`; other runtimes spell it differently. Wherever that marking is honoured, the effect is the same: only the user typing the skill's name can invoke it, and **no skill can invoke another**. So a "call `tars-backlog-<phase>`" instruction will simply be refused.

**When it is refused, read that skill's `SKILL.md` and execute its steps inline.** Each call site gives the path. If your runtime does permit skill-to-skill invocation, calling it directly is equivalent and fine.

Keeping the marking costs nothing at rest; removing it would load all seven descriptions into every session's context permanently, for skills that are only ever driven deliberately. Treat a refusal as something to route around, never as a reason to skip the step.

Skipping `tars-backlog-prepare` in particular leaves the run with no spoke root, no clone mode, and no integrity check.

## Topic Branch Workflow (Hub Only)

All backlog operations run from a topic branch (never the default branch). Every spoke branches off this active topic branch, and all approved changes are merged back into it. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) for the full policy and commands.

## The Two Contention Rules

Everything below follows from two facts about running several agents against one repository. Both are load-bearing; violating either produces failures that look like flaky code rather than orchestration bugs.

1. **A spoke must never be able to write to the parent's git state.** `.git/hooks/` and `.git/config` are shared by every linked worktree of a repository, so a spoke that installs git hooks in a worktree (`prek install`, `pre-commit install`, `lefthook install`) rewrites them for the parent and all its siblings. Hook installers generally bake an **absolute** config path into the shim they write, so there is no benign version of this. Giving each spoke its own **clone** makes the whole class of shared-state collisions structurally impossible rather than merely forbidden.

2. **Concurrent heavy commands produce false test failures, not just slow ones.** Test suites that spawn processes, bootstrap temporary directories, or carry timeouts fail under CPU starvation and pass in isolation. A false failure is worse than a slow one: it sends correct work into the rework loop. So heavy commands are serialised by a mutex, while cheap deterministic checks stay fully parallel.

The split that matters is **load-sensitivity, not which agent is running the command**:

| Class                    | Examples                                                      | How it runs                    |
| ------------------------ | ------------------------------------------------------------- | ------------------------------ |
| Deterministic under load | Typecheck, formatters, linters, hooks scoped to changed files | Freely, in parallel, unlocked  |
| Starvation-sensitive     | Test suites, whole-repo hook runs, nix/container builds       | Under the mutex, one at a time |

These get slower under load; those get **wrong** under load.

## The Heavy-Command Mutex

Any starvation-sensitive command — run by a spoke or by the Hub — is wrapped:

```bash
sh "$TARS_LOCK" "$TARS_HEAVY_LOCK" <command>
```

The helper ships at `resources/manual/tars-lock` beside this skill. It prefers `flock(1)` and falls back to an atomic-mkdir lock with stale reclaim where `flock` is absent, which is the normal case on macOS. Always invoke it as `sh <path>`, never by executing it directly, so it works regardless of whether the install method preserved the executable bit.

The Hub takes the same lock as the spokes. That is deliberate: the Hub's verification gate is itself a heavy command, and left unlocked it would become an extra concurrent test run on top of a full batch.

The heavy-command set is per-repository. Read it from `.tars/config.yaml` if configured, otherwise use these defaults:

```yaml
concurrency:
  heavy_commands:
    - "test"
    - "coverage"
    - "run -a" # prek run -a
    - "--all-files" # pre-commit / lefthook whole-repo runs
    - "nix build"
```

**Matching rule**: an entry matches if it appears **anywhere in the full command line**, compared case-insensitively. Not prefix, not exact — substring. This is what makes `bun run test:coverage`, `devenv test`, `plugin --action test`, and `cargo test --workspace` all match the single entry `test`, without anyone having to enumerate every invocation a repository happens to use.

**When in doubt, take the lock.** The two errors are not symmetrical: over-matching costs a spoke some waiting, while under-matching costs a false test failure that sends correct work into the rework loop. An entry broad enough to catch an occasional cheap command is the right trade.

The corollary is that a _wrapper_ must not be listed. An entry of `devenv` or `bun` on its own would match `devenv shell -- prek run <file>`, serialising precisely the cheap deterministic checks that are supposed to stay parallel. Match on the action (`test`, `coverage`, `build`), never on the shell that runs it.

### Leaked workers defeat the mutex

Test runners spawn worker processes, and a worker that outlives its parent is **reparented to init**. It then holds no lock, so the mutex cannot see it — while it saturates cores exactly like a second concurrent suite. This is worse than an unlocked command: it starves every spoke and both gates while the system looks idle, so nothing in the pipeline attributes the resulting slowness or false failures to it. One observed leak ran for over three hours across 15 threads and ignored `SIGTERM`.

Three defences, in order of reliability:

1. **`tars-lock` contains and reaps.** It runs each command in its own process group and signals the whole group on exit — `SIGTERM`, escalating to `SIGKILL` after five seconds. This works even after descendants are reparented, because process-group membership survives reparenting. It degrades to no reaping (never to failure) where `ps` is unavailable, and can be disabled with `TARS_LOCK_NO_REAP=1`.
2. **Spokes must not background heavy commands.** Run them in the foreground and let them exit. A spoke that reports completion while its test runner is still alive has leaked by construction.
3. **The Hub sweeps between batches.** Before starting a new batch, look for processes matching the repository's heavy commands that belong to no live spoke, and kill them. Something that ignores `SIGTERM` needs `SIGKILL`.

> Do this sweep even when the batch looked clean. The cost of a leak scales with batch size — five spokes leaking a few hundred megabytes each, every batch, is how a machine reaches an OOM kill — and the symptom, tests failing under load, is the one the mutex exists to prevent, so it will be misread as a flaky suite.

## The Verification Gate

The Hub runs this twice — once per spoke inside its clone, once on the topic branch after the batch. Both use the same shape, and the quoting is the part that goes wrong.

**Substitute the values from `.tars/run.env` directly into the command string. Never leave `$VAR` references to be expanded by the inner shell.** `sh -c` starts a fresh shell that inherits only _exported_ variables, and values read out of a file are not exported. A `$SPOKE_DIR` left unexpanded becomes `cd ""`, which either fails — a red gate on perfectly good code — or, in a shell that treats it as a no-op, silently gates **the parent working tree instead of the clone**, reviewing code that is not the code under test.

So: the `sh -c` argument is **double-quoted**, letting the outer shell interpolate, with paths **single-quoted** inside it so spaces are safe.

```bash
# Plain repository. <angle brackets> are substituted by the Hub before running.
PRE_COMMIT_HOME="<spoke-root>/hook-cache" \
  sh "<tars-lock>" "<heavy-lock>" \
  sh -c "cd '<spoke-dir>' && <install-command> && <hook-command> && <test-command>"
```

**The gate always installs first.** It is idempotent and costs milliseconds when the lockfile has not moved, and it removes an entire class of false red gates — see _Installed dependencies go stale_ below.

**When a substituted command is empty** — `TARS_HOOK_COMMAND` for a repository with no hook runner, or `TARS_INSTALL_COMMAND` for one with no install step — substitute `:`, the POSIX no-op. Splicing an empty string leaves `&& &&`, which is a syntax error, not an empty step.

**In a `devenv` repository** every command must run inside `devenv shell --`, which nests a third shell. Interpolate at the outer level and keep the innermost layer in single quotes:

```bash
PRE_COMMIT_HOME="<spoke-root>/hook-cache" \
  sh "<tars-lock>" "<heavy-lock>" \
  sh -c "cd '<spoke-dir>' && devenv shell -- sh -c '<install-command> && <hook-command> && <test-command>'"
```

The hook and test commands sit inside single quotes here, so neither may itself contain a single quote. If one does, write the pair into a small script in the clone and run that instead of fighting the quoting.

> Verify the gate command before trusting a red result. The first time it runs in a repository, confirm it actually executed in the clone — a gate that fails instantly, or that reports problems in files the ticket never touched, is a quoting bug rather than a finding.

### Installed dependencies go stale

Installed dependencies are derived from the lockfile, and **git moves the lockfile without touching `node_modules/`** (or `vendor/`, `target/`, `.venv/`). Any operation that changes the manifest therefore invalidates what is installed, and the next command fails with a missing-module error that looks like broken code.

This bites in three distinct places, and fixing only one leaves the other two:

| Moment                                           | Why it goes stale                                                                   |
| ------------------------------------------------ | ----------------------------------------------------------------------------------- |
| Spoke starts in a fresh clone                    | Nothing is installed at all                                                         |
| Spoke merges the topic branch to sync            | A **sibling** ticket may have added a dependency since this clone was made          |
| Parent, after merging a dependency-adding ticket | The parent's installed tree predates the merge, so the batch-final gate fails on it |

The third is the one that surprises people: the ticket's own gate was green — it installed its new dependency in its own clone — and the parent then fails with `Cannot find module` for that same dependency, immediately after a successful merge. Every dependency-adding ticket reproduces it.

Putting the install inside the gate chain covers all three, because every one of those moments is followed by a gate. Re-run it after step 4.1's sync merge too, since the spoke's own cheap checks run outside the gate.

> Use the **lockfile-respecting** install form — `bun install --frozen-lockfile`, `npm ci`, `cargo fetch --locked`, `uv sync --frozen`. At gate time the committed lockfile is the authority; an install that is free to resolve new versions could turn a green gate into a different dependency tree than the one being reviewed.

## Implementation Workflow

### 1. Backlog Scan & Conflict-Free Batching

1. Scan the `.tars/issues/todo/` directory for ticket markdown files.
2. Analyze the `files` or `component` lists of all pending tickets, and their `dependencies` frontmatter.
3. Dynamically group the tickets into batches of at most 5 concurrent tickets. A batch is admissible only if **both** rules hold:
   - **File rule**: no two tickets in the batch modify overlapping files, compared using each ticket's `files:` frontmatter.
   - **Dependency rule**: no ticket in the batch names, in its `dependencies`, another ticket that is in the same batch or is still unmerged (anywhere in `.tars/issues/todo/` or `.tars/issues/failed/`).

   **A ticket with no `files:` list fails the File rule — it does not pass it.** An absent list means "unknown footprint", and comparing two unknowns finds no overlap, so a naive reading batches everything together precisely when it has the least idea what will collide. When `files:` is missing or empty, either derive it first (read the ticket body and the code it names, then write the list back to the frontmatter) or schedule that ticket **alone**. Never treat an empty list as a wide berth.

   > `component:` is not a substitute. It is coarse by design — a backlog where most tickets share one component would batch them all, and several tickets editing different files under one directory is exactly the case the File rule exists to catch. Use it to _suspect_ a collision, never to clear one.

#### Shared append-only files

Some files every ticket may need to touch without knowing it in advance: a spellcheck dictionary, a changelog, a barrel/index re-export, an i18n catalogue, a lockfile. A spoke adding a new identifier discovers mid-implementation that a blocking hook requires a dictionary entry, and appends one line to the tail of a file every other spoke is also appending to.

The File rule cannot prevent this — the need is unforeseeable at ticket-writing time — and it would be wrong to try: serialising every ticket that _might_ need a dictionary entry would serialise the whole backlog.

Declare them instead, and resolve them mechanically. Read `worktree.shared_append_files` from `.tars/config.yaml`:

```yaml
worktree:
  shared_append_files:
    - "project-words.txt"
    - "CHANGELOG.md"
```

For a file on this list, a merge conflict is **expected and not a rework trigger**. The Hub resolves it by taking the union of both sides, applying the file's own ordering convention (sorted for a dictionary, newest-first for a changelog), and continuing the merge. Sending a spoke back to rework over two independent additions to a word list would burn an attempt on a conflict that carries no disagreement about the code.

> Everything else stays a real conflict. This exception is for files whose semantics are "an unordered set of lines", where both sides are simply right. Do not extend it to source files. 4. Update the ticket frontmatter with `batch: X` (starting with `batch: 1`) and write to disk so batches are remembered. 5. Prior to executing a batch, the Hub must verify that all tickets in the current batch satisfy **both** rules. 6. **Resolve tickets the dependency rule can never admit.** Before declaring a pass complete, check every ticket still unbatched:

- **Dependency in `.tars/issues/failed/`**: the dependent can never become eligible. Move it to `.tars/issues/failed/` too, recording under `## Implementation Review` which failed dependency blocked it.
- **Dependency cycle** (A depends on B, B depends on A, directly or transitively): no member can ever be batched. Move every ticket in the cycle to `.tars/issues/failed/`, naming the cycle, and report it — a cycle is a ticket-authoring bug, not a scheduling one.
- **Dependency on an ID that does not exist** in any of the three folders: treat the edge as satisfied and warn, rather than blocking forever on a ticket that was never written.

> Step 6 is what stops the dependency rule from stalling the loop. "Cannot be batched yet" and "can never be batched" look identical to a scheduler, and without an explicit sweep a ticket blocked behind a failed or cyclic dependency stays in `todo/` forever — which never terminates, because the convergence condition is an empty `todo/`.
>
> The two rules catch different failures, and the file rule alone is not enough. Ticket 527 with `dependencies: ["526"]` may touch entirely different files from 526 — the file rule says "batch them together", and 527's spoke then builds against a base where 526's work does not exist yet. Nothing errors; the spoke simply produces work founded on nothing and the gate passes it. A dependency edge means "must already be merged", not "must not collide".
>
> Conflict-free by file is also not the same as behaviourally independent. Two tickets with no dependency edge and no shared file can each pass alone and still break each other once both are merged. That residue is caught by the batch-final gate in step 5 of this workflow.

### 2. Spawn Spokes in Isolated Clones

For each ticket in the selected batch, create an isolated clone and spawn an implementation subagent in it.

#### 2a. Create the clone

```bash
SPOKE_DIR="$TARS_SPOKE_ROOT/<TICKET_ID>"
```

Clone from the parent repository using the mode `tars-backlog-prepare` resolved:

```bash
# TARS_CLONE_MODE=hardlink  (spoke root shares a filesystem with the repo)
git clone --branch "$TARS_TOPIC_BRANCH" "$REPO_ROOT" "$SPOKE_DIR"

# TARS_CLONE_MODE=shared    (different filesystem; borrow objects, copy nothing)
git clone --shared --branch "$TARS_TOPIC_BRANCH" "$REPO_ROOT" "$SPOKE_DIR"
```

In `shared` mode the Hub must **not** run `git gc` or `git prune` in the parent while any spoke is alive, because spokes borrow the parent's objects through alternates.

Then position the spoke's branch inside the clone:

- **New ticket**: `git checkout -b subagent-<TICKET_ID>`
- **Rework ticket** (frontmatter carries `branch:`): the clone already fetched it, so `git checkout subagent-<TICKET_ID>` then `git merge origin/<topic-branch>` to pick up everything merged since the last attempt.

A fresh clone has no hooks installed — `.git/hooks/` contains only samples. Spoke commits therefore fire nothing, which is why a spoke never needs `--no-verify` and never has a reason to install hooks. Verification in a spoke is always an explicit command, never a side effect of committing.

#### 2b. Transfer gitignored files

Gitignored files do not exist in a fresh clone, and several are required to work — `.pre-commit-config.yaml` in particular is generated by tooling and gitignored in many repositories. Read `worktree.transfer_files` from `.tars/config.yaml` if configured, otherwise use these defaults:

```yaml
worktree:
  transfer_files:
    - ".pre-commit-config.yaml"
    - ".env*"
    - ".tars/"
    - "devenv.local.nix"
    - "devenv.local.yaml"
```

For each matching file or directory in the parent workspace root:

- **Symlink (preferred)**: create a symlink in the clone pointing at the parent's copy.
  > [!IMPORTANT]
  > Symlinking `.tars/` gives the spoke direct read/write access to the shared issue queue and to `.tars/run.env`, so ticket updates need no copying back when the spoke finishes.
- **Copy (fallback)**: if symlinking fails, copy the file or directory in. If `.tars/` was copied rather than symlinked, the Hub **MUST** copy the ticket file back from `<spoke-dir>/.tars/issues/todo/XXX.md` to the parent before deleting the clone.

Then **exclude every transferred name in the clone**:

```bash
printf '%s\n' .pre-commit-config.yaml .env .tars devenv.local.nix devenv.local.yaml \
  >> "$SPOKE_DIR/.git/info/exclude"
```

Without this the spoke sees `?? .tars` in every `git status`, and one `git add -A` commits the transfer into the branch. A repository's own `.gitignore` does not cover this: an entry written as `.tars/` — with a trailing slash — matches **directories only**, and a symlink pointing at a directory is not a directory as far as git is concerned. So the ignore rule that works fine in the parent silently fails on the transferred symlink.

The skill tells spokes never to stage `.tars/`, and they generally comply, but that is an instruction where a mechanism is available for two lines. Use the mechanism.

> A clone has a real `.git` directory, so `.git/info/exclude` is read normally. This is one of the things that quietly breaks under worktrees, where the per-worktree git dir is not where `info/exclude` is read from.

#### 2c. Bootstrap dependencies

A fresh clone has no installed dependencies and no build outputs — no `node_modules/`, `vendor/`, `target/`, `dist/`. These are correctly absent from `transfer_files`: symlinking a dependency directory across workspaces produces partial-install failures that surface later as baffling type or resolution errors.

The Hub must therefore tell the spoke to bootstrap before verifying anything, using the repository's own install command (`bun install`, `npm ci`, `cargo fetch`, `uv sync`, …).

> Watch for environments that suppress auto-install. A `devenv.nix` that disables Bun's automatic install when `CI=true` will leave a spoke with no `node_modules` and no error explaining why — the first symptom is something like `TS2688: Cannot find type definition file for 'node'`, which reads as a broken tsconfig rather than a missing install.

#### 2d. Share one hook cache

Set `PRE_COMMIT_HOME` to a **single shared** cache under the spoke root, used by every spoke and by the Hub:

```bash
PRE_COMMIT_HOME="$TARS_SPOKE_ROOT/hook-cache"
```

`PRE_COMMIT_HOME` is honoured by both `prek` and `pre-commit`; skip this step for a repository whose hook runner does not use it.

One cache rather than one per spoke: hook environments are expensive to build, and installing them N times per batch is pure waste. The original per-workspace isolation existed to dodge permission blocks in sandboxed environments — placing the shared cache under the spoke root, which `tars-backlog-prepare` already write-probed, satisfies that requirement too.

#### 2e. Spawn the subagent

Spawn each spoke on its clone, and equip it with:

- **Role**: `Implement-<TICKET_ID>` (substitute the 3-digit ticket ID, e.g. `Implement-044`)
- **Prompt**:

  ```text
  You are tasked with implementing the changes described in this ticket.

  Ticket Details:
  <TICKET_CONTENT>

  Your workspace is an isolated clone at <SPOKE_DIR>, already checked out on
  your branch. Its `origin` is the parent repository.

  Instructions:
  1. Read the ticket details completely, including the Tasks, Acceptance Criteria (conforming to the guidelines in [tars-backlog-create-issue](../../planning/tars-backlog-create-issue/SKILL.md)), the '## Review' section, and the '## Implementation Review' section (if it exists).
  2. Implement the changes described, addressing any feedback listed in the '## Implementation Review' section.
  3. Your clone is fresh and has NO installed dependencies or build outputs. Bootstrap them before verifying anything, using: <TARS_INSTALL_COMMAND>
  4. Verify as you work. Cheap deterministic checks — typecheck, formatters, linters, and hooks scoped to YOUR CHANGED FILES ONLY (with prek that is `prek run <changed files>`) — you may run freely and as often as you like; they get slower under load but never wrong.
  5. Run test suites ONLY through the mutex, because several suites at once cause false failures. Wrap every test command exactly like this, substituting the paths given above:
     PRE_COMMIT_HOME="<PRE_COMMIT_HOME>" sh "<TARS_LOCK>" "<TARS_HEAVY_LOCK>" <test command>
     A command counts as a test command if its command line contains any of <HEAVY_COMMANDS> (case-insensitive substring). When unsure, take the lock — waiting is cheap, a false failure is not.
     Run the tests covering your change, not the whole suite — the Hub runs the full suite at the gate. If a test fails, re-run it alone before treating it as real.
  6. NEVER install git hooks (`prek install`, `pre-commit install`, `lefthook install`), and NEVER run the hooks across the whole repository. Installing bakes an absolute config path into a shim; a whole-repo hook run is a heavy command reserved for the Hub's gate. Run hooks scoped to your changed files only — with prek that is `prek run <changed files>`, see the [prek](../../tooling/prek/SKILL.md) skill.
  7. Commit your changes using Conventional Commits. Your clone has no git hooks installed, so commits are unhooked by design and you must never pass `--no-verify`. STRICT GITIGNORE CONSTRAINT: You must NEVER stage, commit, or force-add any files under the `.tars/` directory (such as the ticket file `.tars/issues/todo/XXX.md`). These files must remain completely unstaged and uncommitted in git.
  8. STRICT ISOLATION CONSTRAINT: Work only on your own branch in your own clone. Never check out the default branch or the topic branch, never commit to them, and never merge your branch into anything. The one merge you may perform is pulling the topic branch INTO your branch (`git merge origin/<topic-branch>`) to sync. The Hub is solely responsible for merging your work back and for cleaning up your workspace.
  9. Update the ticket file `.tars/issues/todo/XXX.md` to complete the checkboxes in the '## Tasks' and '## Acceptance Criteria' sections, and document command runs and outputs proving execution in the '## Evidence' section as outlined in [tars-backlog-create-issue](../../planning/tars-backlog-create-issue/SKILL.md).
  10. Report completion, then STAY AVAILABLE. The Hub will run a full verification gate on your work and may send you failures to fix. Do not consider yourself finished until the Hub tells you the ticket is resolved.
  11. **STRICT TOOL SYNTAX CONSTRAINT**: When calling filesystem or command-execution tools, you must never wrap string argument values in nested, escaped, or literal double quotes (e.g. pass a path argument as `/path/to/file`, not the same value re-wrapped in escaped quotes, which is incorrect and will fail due to invalid characters).
  ```

### 3. Monitor Spokes

Do **NOT** wait passively for the whole batch — a single stuck spoke would block all progress.

- Periodically (e.g. every couple of minutes) check on the running spokes while they work: if your runtime provides a scheduling or wakeup mechanism, use it to trigger the check, otherwise poll. On each check, use your agent's subagent-management capability to list the running spokes and verify their liveness/status.
- **Detect blocked/approval-waiting spokes**: a spoke may issue a command that gets suspended waiting for user approval, and such prompts do not always bubble up. If your runtime surfaces spoke logs or state, inspect the latest entries; otherwise rely on its status signal. When a spoke appears blocked on an approval, warn the user explicitly:
  `"⚠️ Subagent <role> is waiting for your approval to run a command. Please switch to its session or approve the command."`
- **A spoke waiting on the mutex is not stuck.** `tars-lock` prints a notice to stderr after 30 seconds of waiting. Queued is the system working as designed.
- **Never infer completion from log content.** Use the runtime's own status signal for the subagent, or an explicit completion report from the spoke. Pattern-matching its output for words like `error`, `fail`, or `done` gives false readings, because a passing suite prints those words while exercising its error paths — so a still-running gate reads as finished. A Hub that trusts such a signal merges on an unfinished gate, which is the one failure this whole ordering exists to prevent.
- If a spoke has stopped (e.g. due to a crash or restart) before its ticket resolved, revive it with a follow-up query, or restart it on its branch. Prefer `SIGTERM` over `SIGKILL` when stopping a spoke: the mutex's fallback path keys on the wrapper's PID, and `SIGKILL` can briefly let two heavy commands overlap.

### 4. Verify, Then Merge (Hub Only)

Process each spoke the moment it reports, rather than waiting for the batch. **Verification happens before the merge, inside the spoke's own clone.** That ordering is what keeps the parent working tree pristine — there is no `git reset --hard` anywhere in this path — and it means a failure is found while the agent that wrote the code is still alive to fix it.

For each spoke that reports completion:

1. **Sync the spoke onto the latest topic branch.** Instruct the spoke to run `git fetch origin && git merge origin/<topic-branch>`, then re-run the install command. Let the spoke resolve any conflicts; it has the context for its own code. The re-install matters because a sibling ticket may have added a dependency since this clone was made — see _Installed dependencies go stale_ above.

2. **Capture the work durably.** Fetch the spoke's branch into the parent repository:

   ```bash
   git fetch "$SPOKE_DIR" "+subagent-<TICKET_ID>:subagent-<TICKET_ID>"
   ```

   Do this whether the gate later passes or fails. The parent repository is the durable store of all spoke work; clone directories are disposable scratch. Skipping this step means deleting a clone destroys its commits.

3. **Run the verification gate, in the clone, under the mutex** — using the exact shape and quoting rules from **The Verification Gate** above, with `<spoke-dir>` set to this spoke's clone. Substitute every value; leave no `$VAR` for the inner shell to expand.

   **NEVER** use `--no-verify` or bypass hooks.

4. **Handle a red gate**: send the failure output back to the live spoke and let it fix its own work. No rework ticket, no respawn — the agent still holds the context. Allow up to **3** fix rounds. If it is still red after that, treat it as `Request Rework` below; a spoke that has failed three times usually has a context anchored on a wrong approach, and a fresh agent reading the feedback beats a tired one re-reading its own reasoning.

   > **The two counters are nested, not additive.** Fix rounds live _inside_ one attempt: exhausting all 3 increments `attempts` by exactly **one**, not by three. So a ticket gets up to 5 attempts, each with a fresh spoke and up to 3 in-spoke fix rounds. Incrementing per fix round would burn a ticket's whole budget on a single spoke having a bad day, which is the opposite of what the cap is for — `attempts` counts _how many agents have tried_, not how many commands failed.

5. **Run the implementation review**: on a green gate, execute [tars-backlog-review](@/skills/review/tars-backlog-review/_index.md) against the spoke's clone and ticket file — reading its `SKILL.md` and following it inline, per **Invoking Sibling Skills** above.

   > Gate first, review second. A gate failure is self-service — the spoke fixes it with no Hub tokens spent — whereas a review rejection costs synthesis and interpretation. Reviewing first would also mean reviewing code that is about to change under it, and spending reviewer attention on lint that `prek` already catches.

6. **Handle the verdict**:
   - **If Approved**:
     - **Merge sequentially** into the active topic branch, one spoke at a time. Never perform parallel merges.
     - **Move Ticket**: move the ticket file to `.tars/issues/done/`.
     - **Dismiss the spoke**: message it that its ticket is resolved and it may stop, then release it.
     - **Clean up**: delete the clone directory `$SPOKE_DIR` and the branch (`git branch -D subagent-<TICKET_ID>`).
   - **If Request Rework**:
     - Do **NOT** merge the branch.
     - Increment the ticket's `attempts` count in the frontmatter.
     - **Dismiss the spoke**: message it that its ticket is going to rework and it may stop, then release it.
     - If `attempts >= 5`, move the ticket file to `.tars/issues/failed/`, delete the clone directory, and `git branch -D subagent-<TICKET_ID>`.
     - Otherwise, set `status: rework`, set `batch: null`, update `branch: subagent-<TICKET_ID>` in the frontmatter, and append the review feedback under `## Implementation Review` following the format in [tars-backlog-review](@/skills/review/tars-backlog-review/_index.md). The ticket remains in `.tars/issues/todo/`. Delete the clone directory but **keep the branch** — it was fetched into the parent in step 2, and the next attempt clones from there.

> Dismissing the spoke is not optional bookkeeping. The spoke prompt instructs it to stay available until the Hub says its ticket is resolved, so a spoke that is never told will sit idle for the rest of the run. Every terminal path — approved, rework, and attempts-exhausted — must end with that message.

Repeat until every spoke in the batch has been merged, sent to rework, or failed.

### 5. Batch-Final Gate (Hub Only)

Once the whole batch is merged, run the full gate once more — on the **topic branch**, in the parent workspace, under the mutex. Same shape and quoting rules as **The Verification Gate** above, with `<spoke-dir>` set to the parent repository root.

Use the **same resolved commands** as the per-spoke gate — the values `tars-backlog-prepare` recorded, unchanged. Do not re-derive or substitute a shortened command here. A batch-final gate that runs less than the per-spoke gate cannot catch the cross-ticket interaction it exists for, and the divergence is invisible: both report green.

Each spoke was verified against the topic branch as it stood at its own gate, but the topic branch moves as its siblings merge. This pass catches the semantic interaction that file-level conflict-free batching cannot see: two tickets that touch no common file, each green alone, that break each other once both are in.

If this gate is red, the offending merge is one of the batch just landed. Identify it, revert that merge, and return its ticket to rework with the failure recorded under `## Implementation Review`. Then proceed to the next batch.

### 6. Confirm CI Agrees (Hub Only)

A green local gate is **not** a green CI. If the topic branch is pushed and the repository runs CI, check the result for the batch's head commit before starting the next batch.

This is not belt-and-braces. The loop's value is that merged work is verified, and a CI-only failure discovered five batches later has to be bisected across everything merged since — whereas checking per batch names the culprit immediately. Where CI disagrees with a green local gate, record which workflow failed against the responsible ticket, even if the cause is not yet understood. "Merged, local gate green, CI workflow X red, undiagnosed" is a useful state; silently continuing is not.

> **The mutex does not reach CI.** It bounds concurrent heavy commands during a backlog run _on this host_. A CI runner is a different machine under its own load, so a test that is timing-sensitive can still fail there while passing every local run — the same starvation class the mutex exists for, outside its reach. Two things make this materially worse and are worth checking in the CI config: a suite that runs more than once per workflow (hooks that invoke tests, plus a separate test step, plus a test task — the same duplication the gate avoids), and jobs running in parallel on one runner. Deduplicating those removes real pressure from exactly the tests that fail this way.

