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
- This skill requires `tars-backlog-prepare` to have run first. Read `.tars/run.env` for the resolved `TARS_SPOKE_ROOT`, `TARS_CLONE_MODE`, `TARS_LOCK`, `TARS_HEAVY_LOCK`, and `TARS_TOPIC_BRANCH`. Re-read that file rather than remembering its values, so a compacted Hub context cannot drift onto a different spoke root mid-run.

## Topic Branch Workflow (Hub Only)

All backlog operations run from a topic branch (never the default branch). Every spoke branches off this active topic branch, and all approved changes are merged back into it. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](../../planning/tars-backlog-prepare/SKILL.md) for the full policy and commands.

## The Two Contention Rules

Everything below follows from two facts about running several agents against one repository. Both are load-bearing; violating either produces failures that look like flaky code rather than orchestration bugs.

1. **A spoke must never be able to write to the parent's git state.** `.git/hooks/` and `.git/config` are shared by every linked worktree of a repository, so a spoke running `prek install` in a worktree rewrites the hook for the parent and all its siblings — `prek` bakes an **absolute** config path into every shim it writes, so there is no benign version of this. Giving each spoke its own **clone** makes the whole class of shared-state collisions structurally impossible rather than merely forbidden.

2. **Concurrent heavy commands produce false test failures, not just slow ones.** Test suites that spawn processes, bootstrap temporary directories, or carry timeouts fail under CPU starvation and pass in isolation. A false failure is worse than a slow one: it sends correct work into the rework loop. So heavy commands are serialised by a mutex, while cheap deterministic checks stay fully parallel.

The split that matters is **load-sensitivity, not which agent is running the command**:

| Class                    | Examples                                                        | How it runs                    |
| ------------------------ | --------------------------------------------------------------- | ------------------------------ |
| Deterministic under load | `tsc --noEmit`, `prek run <changed files>`, formatters, linters | Freely, in parallel, unlocked  |
| Starvation-sensitive     | Test suites, `devenv test`, nix builds, `prek run -a`           | Under the mutex, one at a time |

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
    - "devenv test"
    - "prek run -a"
    - "bun test"
    - "npm test"
    - "cargo test"
    - "pytest"
```

## Implementation Workflow

### 1. Backlog Scan & Conflict-Free Batching

1. Scan the `.tars/issues/todo/` directory for ticket markdown files.
2. Analyze the `files` or `component` lists of all pending tickets.
3. Dynamically group the tickets into conflict-free batches of at most 5 concurrent tickets. A batch is conflict-free if no two tickets in it modify overlapping files.
4. Update the ticket frontmatter with `batch: X` (starting with `batch: 1`) and write to disk so batches are remembered.
5. Prior to executing a batch, the Hub must verify that all tickets in the current batch are indeed conflict-free.

> Conflict-free here means **non-overlapping files**, which is not the same as behaviourally independent. Two tickets can each pass alone and still break each other once both are merged. Step 5 closes that gap.

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

A fresh clone has no hooks installed — `.git/hooks/` contains only samples. Spoke commits therefore fire nothing, which is why a spoke never needs `--no-verify` and never has a reason to run `prek install`. Verification in a spoke is always an explicit command, never a side effect of committing.

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

#### 2c. Share one hook cache

Set `PRE_COMMIT_HOME` to a **single shared** cache under the spoke root, used by every spoke and by the Hub:

```bash
PRE_COMMIT_HOME="$TARS_SPOKE_ROOT/prek-cache"
```

One cache rather than one per spoke: hook environments are expensive to build, and installing them N times per batch is pure waste. The original per-workspace isolation existed to dodge permission blocks in sandboxed environments — placing the shared cache under the spoke root, which `tars-backlog-prepare` already write-probed, satisfies that requirement too.

#### 2d. Spawn the subagent

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
  3. Verify as you work. Cheap deterministic checks — typecheck, formatters, linters, and `prek run <your changed files>` — you may run freely and as often as you like; they get slower under load but never wrong.
  4. Run test suites ONLY through the mutex, because several suites at once cause false failures. Wrap every test command exactly like this, substituting the paths given above:
     PRE_COMMIT_HOME="<PRE_COMMIT_HOME>" sh "<TARS_LOCK>" "<TARS_HEAVY_LOCK>" <test command>
     Run the tests covering your change, not the whole suite — the Hub runs the full suite at the gate. Detect the runner as usual: `devenv test` if `devenv.nix` or `devenv/default.nix` is present, otherwise the project's standard test config. If a test fails, re-run it alone before treating it as real.
  5. NEVER run `prek install`, and NEVER run `prek run -a`. Installing hooks bakes an absolute config path into a shim; running the whole repository's hooks is a heavy command reserved for the Hub's gate. `prek run <changed files>` is the form you want. See the [prek](../../tooling/prek/SKILL.md) skill.
  6. Commit your changes using Conventional Commits. Your clone has no git hooks installed, so commits are unhooked by design and you must never pass `--no-verify`. STRICT GITIGNORE CONSTRAINT: You must NEVER stage, commit, or force-add any files under the `.tars/` directory (such as the ticket file `.tars/issues/todo/XXX.md`). These files must remain completely unstaged and uncommitted in git.
  7. STRICT ISOLATION CONSTRAINT: Work only on your own branch in your own clone. Never check out the default branch or the topic branch, never commit to them, and never merge your branch into anything. The one merge you may perform is pulling the topic branch INTO your branch (`git merge origin/<topic-branch>`) to sync. The Hub is solely responsible for merging your work back and for cleaning up your workspace.
  8. Update the ticket file `.tars/issues/todo/XXX.md` to complete the checkboxes in the '## Tasks' and '## Acceptance Criteria' sections, and document command runs and outputs proving execution in the '## Evidence' section as outlined in [tars-backlog-create-issue](../../planning/tars-backlog-create-issue/SKILL.md).
  9. Report completion, then STAY AVAILABLE. The Hub will run a full verification gate on your work and may send you failures to fix. Do not consider yourself finished until the Hub tells you the ticket is resolved.
  10. **STRICT TOOL SYNTAX CONSTRAINT**: When calling filesystem or command-execution tools, you must never wrap string argument values in nested, escaped, or literal double quotes (e.g. pass a path argument as `/path/to/file`, not the same value re-wrapped in escaped quotes, which is incorrect and will fail due to invalid characters).
  ```

### 3. Monitor Spokes

Do **NOT** wait passively for the whole batch — a single stuck spoke would block all progress.

- Periodically (e.g. every couple of minutes) check on the running spokes while they work: if your runtime provides a scheduling or wakeup mechanism, use it to trigger the check, otherwise poll. On each check, use your agent's subagent-management capability to list the running spokes and verify their liveness/status.
- **Detect blocked/approval-waiting spokes**: a spoke may issue a command that gets suspended waiting for user approval, and such prompts do not always bubble up. If your runtime surfaces spoke logs or state, inspect the latest entries; otherwise rely on its status signal. When a spoke appears blocked on an approval, warn the user explicitly:
  `"⚠️ Subagent <role> is waiting for your approval to run a command. Please switch to its session or approve the command."`
- **A spoke waiting on the mutex is not stuck.** `tars-lock` prints a notice to stderr after 30 seconds of waiting. Queued is the system working as designed.
- If a spoke has stopped (e.g. due to a crash or restart) before its ticket resolved, revive it with a follow-up query, or restart it on its branch. Prefer `SIGTERM` over `SIGKILL` when stopping a spoke: the mutex's fallback path keys on the wrapper's PID, and `SIGKILL` can briefly let two heavy commands overlap.

### 4. Verify, Then Merge (Hub Only)

Process each spoke the moment it reports, rather than waiting for the batch. **Verification happens before the merge, inside the spoke's own clone.** That ordering is what keeps the parent working tree pristine — there is no `git reset --hard` anywhere in this path — and it means a failure is found while the agent that wrote the code is still alive to fix it.

For each spoke that reports completion:

1. **Sync the spoke onto the latest topic branch.** Instruct the spoke to run `git fetch origin && git merge origin/<topic-branch>`. Let the spoke resolve any conflicts; it has the context for its own code.

2. **Capture the work durably.** Fetch the spoke's branch into the parent repository:

   ```bash
   git fetch "$SPOKE_DIR" "+subagent-<TICKET_ID>:subagent-<TICKET_ID>"
   ```

   Do this whether the gate later passes or fails. The parent repository is the durable store of all spoke work; clone directories are disposable scratch. Skipping this step means deleting a clone destroys its commits.

3. **Run the verification gate, in the clone, under the mutex:**

   ```bash
   PRE_COMMIT_HOME="$TARS_SPOKE_ROOT/prek-cache" \
     sh "$TARS_LOCK" "$TARS_HEAVY_LOCK" \
     sh -c 'cd "$SPOKE_DIR" && prek run -a && <full test command>'
   ```

   **NEVER** use `--no-verify` or bypass hooks.

4. **Handle a red gate**: send the failure output back to the live spoke and let it fix its own work. No rework ticket, no respawn — the agent still holds the context. Allow up to **3** fix rounds. If it is still red after that, treat it as `Request Rework` below; a spoke that has failed three times usually has a context anchored on a wrong approach, and a fresh agent reading the feedback beats a tired one re-reading its own reasoning.

5. **Run the implementation review**: on a green gate, call the `tars-backlog-review` skill (see [tars-backlog-review](../../review/tars-backlog-review/SKILL.md)) against the spoke's clone and ticket file.

   > Gate first, review second. A gate failure is self-service — the spoke fixes it with no Hub tokens spent — whereas a review rejection costs synthesis and interpretation. Reviewing first would also mean reviewing code that is about to change under it, and spending reviewer attention on lint that `prek` already catches.

6. **Handle the verdict**:
   - **If Approved**:
     - **Merge sequentially** into the active topic branch, one spoke at a time. Never perform parallel merges.
     - **Move Ticket**: move the ticket file to `.tars/issues/done/`.
     - **Clean up**: delete the clone directory `$SPOKE_DIR` and the branch (`git branch -D subagent-<TICKET_ID>`), and release the spoke.
   - **If Request Rework**:
     - Do **NOT** merge the branch.
     - Increment the ticket's `attempts` count in the frontmatter.
     - If `attempts >= 5`, move the ticket file to `.tars/issues/failed/`, delete the clone directory, and `git branch -D subagent-<TICKET_ID>`.
     - Otherwise, set `status: rework`, set `batch: null`, update `branch: subagent-<TICKET_ID>` in the frontmatter, and append the review feedback under `## Implementation Review` following the format in [tars-backlog-review](../../review/tars-backlog-review/SKILL.md). The ticket remains in `.tars/issues/todo/`. Delete the clone directory but **keep the branch** — it was fetched into the parent in step 2, and the next attempt clones from there.

Repeat until every spoke in the batch has been merged, sent to rework, or failed.

### 5. Batch-Final Gate (Hub Only)

Once the whole batch is merged, run the full gate once more — on the **topic branch**, in the parent workspace, under the mutex:

```bash
sh "$TARS_LOCK" "$TARS_HEAVY_LOCK" sh -c 'prek run -a && <full test command>'
```

Each spoke was verified against the topic branch as it stood at its own gate, but the topic branch moves as its siblings merge. This pass catches the semantic interaction that file-level conflict-free batching cannot see: two tickets that touch no common file, each green alone, that break each other once both are in.

If this gate is red, the offending merge is one of the batch just landed. Identify it, revert that merge, and return its ticket to rework with the failure recorded under `## Implementation Review`. Then proceed to the next batch.
