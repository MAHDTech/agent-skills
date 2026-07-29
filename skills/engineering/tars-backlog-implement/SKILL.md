---
name: tars-backlog-implement
description: Implement pending backlog issues from `.tars/issues/todo/` in parallel, conflict-free batches using isolated clones. Reach for this when asked to implement backlog issues, execute tasks from tickets in parallel, or resolve the issue queue.
disable-model-invocation: true
---

# Backlog Implement

Implement pending issue tickets from `.tars/issues/todo/` in parallel, conflict-free batches using isolated clones, verifying each one before it is merged back into the active topic branch.

This skill operates in a Hub-and-Spoke topology, spawning implementation subagents ("spokes") in parallel, while the Hub manages sequencing, verification, and merges.

## Targets and Paths

- Target Folders: `.tars/issues/{todo,done,failed}/` relative to project root.
- Ticket status updates are written to disk only. Ticket files are never staged, committed, or force-added in git.
- This skill requires `tars-backlog-prepare` to have run first. Read **only** `.tars/run.env` for run facts (paths, opaque commands, land template, CI flags, weaken banner). Do **not** re-open `.tars/config.yaml` for gate/land/CI — prepare already froze policy into `run.env`. Re-read `run.env` rather than remembering values so a compacted Hub context cannot drift mid-run.
- **If `.tars/run.env` does not exist, do not proceed and do not improvise.** Read [tars-backlog-prepare](../../planning/tars-backlog-prepare/SKILL.md) and execute its steps inline, then continue. See **Invoking Sibling Skills** below for why you cannot simply call it.
- Runners ship beside this skill (always invoke as `sh <path>`):

  | Binary                        | Who         | Role                                                                              |
  | ----------------------------- | ----------- | --------------------------------------------------------------------------------- |
  | `resources/manual/tars-gate`  | Hub only    | Full gate: lock → cd → install → hooks → tests from `run.env`                     |
  | `resources/manual/tars-spoke` | Spokes only | Lock → cd → exec caller args (targeted heavy tests)                               |
  | `resources/manual/tars-lock`  | Internal    | Mutex used by the two runners; hubs/spokes should not assemble lock lines by hand |

## Invoking Sibling Skills

These skills are marked **user-invoked** — in Claude Code that is `disable-model-invocation: true`; other runtimes spell it differently. Wherever that marking is honoured, the effect is the same: only the user typing the skill's name can invoke it, and **no skill can invoke another**. So a "call `tars-backlog-<phase>`" instruction will simply be refused.

**When it is refused, read that skill's `SKILL.md` and execute its steps inline.** Each call site gives the path. If your runtime does permit skill-to-skill invocation, calling it directly is equivalent and fine.

Keeping the marking costs nothing at rest; removing it would load all seven descriptions into every session's context permanently, for skills that are only ever driven deliberately. Treat a refusal as something to route around, never as a reason to skip the step.

Skipping `tars-backlog-prepare` in particular leaves the run with no spoke root, no clone mode, no gate recipe, and no integrity check.

## Topic Branch Workflow (Hub Only)

All backlog operations run from a topic branch (never the default branch). Every spoke branches off this active topic branch, and all approved changes are merged back into it. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](../../planning/tars-backlog-prepare/SKILL.md) for the full policy and commands.

## The Two Contention Rules

Everything below follows from two facts about running several agents against one repository. Both are load-bearing; violating either produces failures that look like flaky code rather than orchestration bugs.

1. **A spoke must never be able to write to the parent's git state.** `.git/hooks/` and `.git/config` are shared by every linked worktree of a repository, so a spoke that installs git hooks in a worktree rewrites them for the parent and all its siblings. Giving each spoke its own **clone** makes the whole class of shared-state collisions structurally impossible rather than merely forbidden.

2. **Concurrent heavy commands produce false test failures, not just slow ones.** Test suites that spawn processes, bootstrap temporary directories, or carry timeouts fail under CPU starvation and pass in isolation. A false failure is worse than a slow one: it sends correct work into the rework loop. So heavy commands are serialised by a mutex, while cheap deterministic checks stay fully parallel.

| Class                    | Examples                                                      | How it runs                                    |
| ------------------------ | ------------------------------------------------------------- | ---------------------------------------------- |
| Deterministic under load | Typecheck, formatters, linters, hooks scoped to changed files | Freely, in parallel, unlocked                  |
| Starvation-sensitive     | Test suites, whole-repo hook runs, nix/container builds       | Via `tars-spoke` (spokes) or `tars-gate` (hub) |

## Heavy commands: mechanical runners, not hand-rolled locks

**Hub full verification** — always:

```bash
# Optional when the hook runner uses a shared cache:
PRE_COMMIT_HOME="<TARS_PRE_COMMIT_HOME>" \
  sh "<TARS_GATE>" "<spoke-dir-or-repo-root>"
```

**Spoke targeted tests** — always:

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
3. **The Hub sweeps between batches** (prose + examples — no separate helper this version). Before starting a new batch, look for processes matching heavy substrings that belong to no live spoke, and stop them (`SIGTERM`, then `SIGKILL` if needed).

Example sweep shape (adapt patterns to the repo; never kill the hub or unrelated user jobs):

```bash
# List candidates; inspect before killing.
pgrep -af 'bun test|pytest|cargo test|prek run' || true
# After confirming a PID is an orphan from this backlog run:
# kill -TERM <pid>; sleep 2; kill -KILL <pid>   # only if still alive
```

> Sweep even when the batch looked clean. Leaks scale with batch size and surface as "flaky suite under load".

## The Verification Gate

The Hub runs the gate twice — once per spoke inside its clone, once on the topic branch after the batch — **only** via `tars-gate` and the commands prepare recorded. Do not re-derive or shorten those commands.

```bash
PRE_COMMIT_HOME="<TARS_PRE_COMMIT_HOME>" \
  sh "<TARS_GATE>" "<spoke-dir>"
```

**Gate always installs first** (when `TARS_INSTALL_COMMAND` is not `:`) so lockfile moves cannot leave stale dependency trees — see _Installed dependencies go stale_ below.

**Weakened gates:** if `TARS_GATE_WEAKENED=1`, every green result is "gate command green (weakened: …)", never "full suite green". Surface `TARS_GATE_WEAKENED_REASON` in batch and final reports.

**Devenv:** if the project has `devenv.nix` / `devenv.yaml`, prepare already followed the [devenv](../../tooling/devenv/SKILL.md) skill when freezing commands. Implement does not restate secrets or CI env vars — it only runs `tars-gate` / `tars-spoke`.

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
4. **Isolation green** → one full `tars-gate` re-run under the mutex. Green → accept. Red → real failure path.
5. **Cap:** at most **one** full re-gate per gate attempt. No flake loops.

### Post-conflict smoke

After any merge conflict resolve that is **not** a shared-append union, before full gate:

1. Run targeted typecheck / unit tests for touched modules via `tars-spoke` (under the mutex).
2. Only if that smoke is green, run full `tars-gate`.

Semantic merges of the same function are easy to get wrong; the full gate is a late place to discover a missing symbol.

## Implementation Workflow

### 1. Backlog Scan & Conflict-Free Batching

1. Scan the `.tars/issues/todo/` directory for ticket markdown files.
2. Analyze `files:`, optional `owns:`, `dependencies:`, and soft-ownership signals in the ticket body.
3. Dynamically group tickets into batches of at most 5. A batch is admissible only if **all** rules hold:
   - **File rule**: no two tickets modify overlapping paths in `files:`.
   - **Owns rule**: no two tickets share an overlapping `owns:` entry (string equality on `path` or `path#symbol`). Treat overlap like a file collision — serialise.
   - **Dependency rule**: no ticket names, in `dependencies`, another ticket that is in the same batch or still unmerged (`todo/` or `failed/`).

   **A ticket with no `files:` list fails the File rule — it does not pass it.** When `files:` is missing or empty, either derive it first or schedule that ticket **alone**.

   **Soft dependencies:** if two tickets clearly add or own the same export/shared constant (from body, `owns:`, or triage notes) but lack a hard `dependencies:` edge, **do not batch them together** — land the natural owner first (or the lower id if unclear), then the other. Prefer adding an explicit `dependencies:` entry when editing frontmatter.

   > `component:` is not a substitute for `files:` or `owns:`. Use it to suspect collisions, never to clear them.

#### Shared append-only files

Some files every ticket may need to touch without knowing it in advance (spellcheck dictionary, changelog, barrel re-export, i18n catalogue, lockfile). Read `worktree.shared_append_files` from prepare-time knowledge / ticket practice; the list is configured in `.tars/config.yaml` and applied at merge time.

For a file on that list, a merge conflict is **expected and not a rework trigger**. The Hub resolves it by taking the union of both sides, applying the file's ordering convention, and continuing. **Do not** extend this exception to ordinary source files.

Then continue batch bookkeeping:

1. Update ticket frontmatter with `batch: X` and write to disk.
2. Before executing a batch, re-verify all rules.
3. **Resolve tickets the dependency rule can never admit:**
   - Dependency in `failed/` → move dependent to `failed/` with a note.
   - Dependency cycle → fail every member of the cycle.
   - Dependency on a missing ID → treat edge as satisfied and warn.

> Conflict-free by file/`owns` is not the same as behaviourally independent. The batch-final gate catches cross-ticket behavioural breaks.

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

A fresh clone has no hooks installed. Spoke commits are unhooked by design — never `--no-verify`, never install hooks.

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
- **Prompt** (default — do **not** paste the full ticket body):

  ```text
  You implement one backlog ticket in an isolated clone.

  Workspace: <SPOKE_DIR> (branch already checked out; origin = parent repo)
  Ticket file (read it fully): <REPO_OR_SPOKE>/.tars/issues/todo/<TICKET_ID>.md
  Run facts: source/read .tars/run.env (do not invent paths or gate commands)

  Rules:
  1. Read the ticket (Tasks, AC, Review, Implementation Review). Implement it; address rework feedback if any.
  2. Bootstrap with TARS_INSTALL_COMMAND from run.env before verifying.
  3. Cheap checks only on YOUR changed files (typecheck/format/lint/hooks scoped to those paths). Never whole-repo hooks.
  4. Heavy tests ONLY via: sh "<TARS_SPOKE>" -- <targeted test command>
     Never bare test runners. Never full-suite / whole-repo prek — Hub runs tars-gate.
  5. Never install git hooks. Never merge into topic/default. You may `git merge` topic INTO your branch to sync.
  6. Conventional commits; never --no-verify; never stage anything under .tars/.
  7. Update Tasks/AC checkboxes and ## Evidence on the ticket file.
  8. Report completion and STAY AVAILABLE until Hub says the ticket is resolved.
  9. Tool args: never wrap paths in nested escaped quotes.
  ```

- **Add checkpoint protocol** when `complexity: high` or `status: rework`:

  ```text
  Checkpoint: commit incremental work on your branch. If stuck after repeated failed approaches,
  stop, leave commits + Evidence notes, and report BLOCKED with what you tried — do not burn a
  long unproductive loop. Hub may resume or rework.
  ```

### 3. Monitor Spokes

Do **NOT** wait passively for the whole batch.

- Periodically check spoke liveness via the runtime's subagent status — never infer completion from log keywords (`error`/`fail`/`done` appear in passing suites).
- Approval-blocked spokes: warn the user explicitly.
- A spoke waiting on the mutex is not stuck.
- Prefer `SIGTERM` over `SIGKILL` when stopping a spoke.

### 4. Verify, Then Merge (Hub Only)

Process each spoke as it reports. **Gate in the clone before merge.** Parent tree stays pristine.

For each completed spoke:

1. **Sync topic into the spoke (ref hygiene).** Force-update the remote-tracking topic ref, then merge:

   ```bash
   git -C "$SPOKE_DIR" fetch origin "+refs/heads/<topic>:refs/remotes/origin/<topic>"
   git -C "$SPOKE_DIR" merge "origin/<topic>"
   ```

   Prefer this force-refspec over a plain `git fetch` that can leave a stale/broken `origin/<topic>` after many hardlink-clone cycles.

   **Repair (once) if fetch fails** with unable-to-lock-ref / dangling / corrupt remote-tracking ref:

   ```bash
   # Remove only the bad remote-tracking ref, then retry the force fetch once.
   git -C "$SPOKE_DIR" update-ref -d "refs/remotes/origin/<topic>" 2>/dev/null || true
   # If packed-refs holds a stale line for that ref, edit carefully or:
   git -C "$SPOKE_DIR" pack-refs --all 2>/dev/null || true
   git -C "$SPOKE_DIR" fetch origin "+refs/heads/<topic>:refs/remotes/origin/<topic>"
   ```

   If still failing, surface to the user — do not mass-edit packed-refs.

   After sync, re-run install (or rely on the upcoming gate's install step). Spoke resolves its own conflicts when possible. After any **non-append** conflict resolve → **post-conflict smoke**, then gate.

2. **Capture durably** in the parent:

   ```bash
   git fetch "$SPOKE_DIR" "+subagent-<TICKET_ID>:subagent-<TICKET_ID>"
   ```

   Always, even if the gate will fail.

3. **Run `tars-gate` on the clone** (with `PRE_COMMIT_HOME` if set). Never `--no-verify`.

4. **Dirty tree after green gate** → commit autofixes on the spoke branch, re-fetch into parent (step 2 again).

5. **Red gate** → flake policy first; then up to **3** in-spoke fix rounds. Exhausting 3 increments `attempts` by **one** (nested counters). Still red → Request Rework path.

6. **Implementation review** (risk-tiered), only on green clean tree:

   | Tier                                                                      | When                                                                                                                                                                                               | What                                                                                                        |
   | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
   | **Lightweight Hub checklist**                                             | Default                                                                                                                                                                                            | AC vs diff; `files:`/`owns:` match touched paths; no `.tars` staged; commit hygiene; spot-check risky hunks |
   | **Full** [tars-backlog-review](../../review/tars-backlog-review/SKILL.md) | Any of: `risk: high`; diff touches a path matching `TARS_REVIEW_ALWAYS_FULL` globs from `run.env`; **post-conflict** resolve this ticket; `attempts >= 2` / rework; Hub marks security/shared-core | Dual-axis Spec + Standards                                                                                  |

   Gate first, review second.

7. **Verdict**:
   - **Approved**:
     - Merge **sequentially** into the topic branch (never parallel merges).
     - Land commit subject from `TARS_LAND_SUBJECT_TEMPLATE` with `{{id}}` / `{{title}}` filled — e.g. `chore(backlog): land ticket 551`. **Never** use a subject that starts with `merge(` or default `Merge branch 'subagent-…'` if the repo rejects it; set `merge.ff` / message explicitly:

       ```bash
       git merge --no-ff -m "<filled land template>" "subagent-<TICKET_ID>"
       ```

     - Move ticket to `.tars/issues/done/`.
     - Dismiss spoke; delete clone; `git branch -D subagent-<TICKET_ID>`.

   - **Request Rework**:
     - Do not merge. Increment `attempts`. Dismiss spoke.
     - If `attempts >= 5` → `failed/`, delete branch and clone.
     - Else `status: rework`, `batch: null`, keep `branch:`, append `## Implementation Review`, delete clone, **keep branch**.

> Every terminal path must dismiss the spoke.

Repeat until the batch is done.

### 5. Batch-Final Gate (Hub Only)

```bash
PRE_COMMIT_HOME="<TARS_PRE_COMMIT_HOME>" \
  sh "<TARS_GATE>" "$REPO_ROOT"
```

Same frozen commands as per-spoke gates. Apply flake policy on red. If still red, identify the offending merge, revert it, return that ticket to rework.

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

- [tars-backlog-prepare](../../planning/tars-backlog-prepare/SKILL.md) — integrity, spoke root, command freeze, baseline smoke, `run.env`.
- [tars-backlog-review](../../review/tars-backlog-review/SKILL.md) — full dual-axis review when risk-tiered rules require it.
- [tars-backlog-create-issue](../../planning/tars-backlog-create-issue/SKILL.md) — ticket format including `files:` / `owns:`.
- [devenv](../../tooling/devenv/SKILL.md) — only if you must understand how prepare built commands; implement does not re-enter devenv by hand when `run.env` is present.
