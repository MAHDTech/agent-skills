---
name: jj
description: Operate Jujutsu (jj), the Git-compatible version control system built around working-copy commits, stable change IDs, revsets, bookmarks, first-class conflicts, and an undoable operation log. Use when working in a Jujutsu repository (.jj/ directory), running jj CLI commands (jj st, jj log, jj diff, jj new, jj describe, jj commit, jj edit, jj squash, jj split, jj absorb, jj rebase, jj bookmark, jj git, jj undo, jj op, jj evolog, jj workspace), writing revset or fileset expressions, resolving conflicts, or syncing with GitHub.
resources:
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/tutorial.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/git-comparison.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/git-command-table.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/git-compatibility.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/bookmarks.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/conflicts.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/revsets.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/filesets.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/operation-log.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/working-copy.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/github.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/cli-reference.md
  - https://raw.githubusercontent.com/jj-vcs/jj/main/docs/config.md
---

<!-- cspell:ignore Jujutsu jujutsu revset revsets fileset filesets evolog obslog colocated colocate bookmark's mergetool diffedit interdiff kxyz kkmpptxz zzzzzzzz reflog -->

# Jujutsu (`jj`) Version Control

Jujutsu (`jj`) is a Git-compatible version control system where the working copy is automatically snapshotted as a real commit (`@`), commits carry persistent **Change IDs** (`kxyz...`) alongside their Git commit hashes, conflicts are recorded inside commits as first-class objects rather than halting rebases, and every repository mutation is logged in an undoable **operation log** (`jj op log` / `jj undo`).

When a repository contains a `.jj/` directory (whether standalone or colocated alongside `.git/`), **always use `jj` commands instead of mutating `git` commands**.

## Progressive Disclosure & Reference Resources

Consult the vendored upstream Jujutsu documentation in `resources/auto/` for full syntax and edge cases:

- **Tutorial & Working Copy:** See [resources/auto/jj-vcs-jj-main-docs-tutorial.md](resources/auto/jj-vcs-jj-main-docs-tutorial.md) and [resources/auto/jj-vcs-jj-main-docs-working-copy.md](resources/auto/jj-vcs-jj-main-docs-working-copy.md).
- **Git Comparison, Command Table & Colocation:** See [resources/auto/jj-vcs-jj-main-docs-git-comparison.md](resources/auto/jj-vcs-jj-main-docs-git-comparison.md), [resources/auto/jj-vcs-jj-main-docs-git-command-table.md](resources/auto/jj-vcs-jj-main-docs-git-command-table.md), and [resources/auto/jj-vcs-jj-main-docs-git-compatibility.md](resources/auto/jj-vcs-jj-main-docs-git-compatibility.md).
- **Revsets & Filesets:** See [resources/auto/jj-vcs-jj-main-docs-revsets.md](resources/auto/jj-vcs-jj-main-docs-revsets.md) and [resources/auto/jj-vcs-jj-main-docs-filesets.md](resources/auto/jj-vcs-jj-main-docs-filesets.md).
- **Bookmarks & GitHub Workflows:** See [resources/auto/jj-vcs-jj-main-docs-bookmarks.md](resources/auto/jj-vcs-jj-main-docs-bookmarks.md) and [resources/auto/jj-vcs-jj-main-docs-github.md](resources/auto/jj-vcs-jj-main-docs-github.md).
- **Conflicts & Operation Log:** See [resources/auto/jj-vcs-jj-main-docs-conflicts.md](resources/auto/jj-vcs-jj-main-docs-conflicts.md) and [resources/auto/jj-vcs-jj-main-docs-operation-log.md](resources/auto/jj-vcs-jj-main-docs-operation-log.md).
- **CLI Reference & Configuration:** See [resources/auto/jj-vcs-jj-main-docs-cli-reference.md](resources/auto/jj-vcs-jj-main-docs-cli-reference.md) and [resources/auto/jj-vcs-jj-main-docs-config.md](resources/auto/jj-vcs-jj-main-docs-config.md).

## Core Concepts vs Git

| Concept                         | How Jujutsu (`jj`) Works                                                                                                                                                             |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Working copy (`@`)**          | Every `jj` invocation snapshots the working copy into `@`. There is no index/staging area (`git add`), and `git stash` is unnecessary because your edits are already a commit.       |
| **Change ID vs Commit ID**      | Each revision has a stable **Change ID** (letters `k-z`, e.g., `kkmpptxz`) that persists across amends and rebases, plus a hex **Commit ID** (`0-9a-f`) that changes when rewritten. |
| **Anonymous branches**          | Commits do not need branch names to stay visible. **Bookmarks** (`jj bookmark`) exist primarily for interoperating with Git remotes and pull requests, and do **not** auto-advance.  |
| **First-class conflicts**       | Rebasing or merging a conflict succeeds immediately and stores the conflict algebraically inside the commit. Descendant commits automatically rebase once you resolve the parent.    |
| **Automatic descendant rebase** | Amending, squashing, editing, or resolving a conflict in an ancestor commit automatically rebases all of its descendants.                                                            |
| **Operation & Evolution logs**  | `jj op log` tracks every state of the entire repository (`jj undo` reverts the last operation). `jj evolog` tracks how a single change evolved across amends and rebases.            |

## Standard Workflows

### 1. The Squash Workflow (Edit in `@`, Squash into Parent `@-`)

Ideal when iterating on a feature commit in `@-` while using `@` as a scratchpad staging area:

```bash
# 1. Create the feature commit with a description, then open a scratch working copy on top
jj describe -m "feat(parser): support nested revset aliases"
jj new

# 2. Edit files in '@', inspect what is in the scratch commit vs parent
jj st
jj diff              # diff in scratch commit '@'
jj diff -r @-        # diff in feature commit '@-'

# 3. Move finished changes from '@' into '@-'
jj squash            # moves all changes from '@' into '@-'
jj squash path/to/rs # moves only selected file paths into '@-'
```

### 2. The Edit Workflow (Work Directly in the Commit)

Ideal for stacked changes or quick single-commit tasks:

```bash
# 1. Work directly in '@' and set its description
jj describe -m "fix(cli): handle empty fileset union"

# 2. Or complete '@' and immediately advance to a new empty commit on top
jj commit -m "fix(cli): handle empty fileset union"

# 3. Jump directly into an earlier commit in your stack to modify it
jj edit <change-id>
# ... edit files (descendants rebase automatically) ...
jj new <tip-change-id>
```

## Git Commands to `jj` Equivalents

| Task                                 | Git Command                         | Jujutsu (`jj`) Command                                         |
| ------------------------------------ | ----------------------------------- | -------------------------------------------------------------- |
| Clone a Git repo                     | `git clone <url>`                   | `jj git clone <url>` (add `--colocate` to keep `.git` in root) |
| Initialize in existing Git repo      | `git init`                          | `jj git init --colocate`                                       |
| Working copy status & diff summary   | `git status`                        | `jj st`                                                        |
| Inspect commit graph                 | `git log --oneline --graph`         | `jj log`                                                       |
| Show diff of working copy / commit   | `git diff` / `git show <rev>`       | `jj diff [-r <rev>]` / `jj show [<rev>]`                       |
| Amend commit message                 | `git commit --amend -m "..."`       | `jj describe -m "..." [-r <rev>]`                              |
| Amend working copy into parent       | `git commit --amend -a --no-edit`   | `jj squash`                                                    |
| Automatically distribute fixups      | `git absorb`                        | `jj absorb`                                                    |
| Discard changes in file              | `git checkout -- <file>`            | `jj restore <file>`                                            |
| Create a new commit on top of `<r>`  | `git switch -c ... <r>`             | `jj new <r> [-m "..."]`                                        |
| Create a merge commit                | `git merge <revA> <revB>`           | `jj new <revA> <revB> -m "Merge ..."`                          |
| Rebase a single commit               | `git cherry-pick` / `rebase --onto` | `jj rebase -r <rev> -d <dest>`                                 |
| Rebase a commit and descendants      | `git rebase --onto <dest> <parent>` | `jj rebase -s <source> -d <dest>`                              |
| Rebase whole branch onto `<dest>`    | `git rebase <dest> <branch>`        | `jj rebase -b <branch> -d <dest>`                              |
| Delete/drop a commit (keep children) | `git rebase -i` (drop line)         | `jj abandon <rev>`                                             |
| Copy/cherry-pick a commit            | `git cherry-pick <rev>`             | `jj duplicate <rev> [-d <dest>]`                               |
| Undo last repo operation             | `git reflog` + `git reset --hard`   | `jj undo`                                                      |
| Blame / file annotation              | `git annotate <file>`               | `jj file annotate <file>`                                      |

## Revsets (`-r`) and Filesets

Jujutsu uses a functional query language called **revsets** to select commits across all commands (`jj log -r`, `jj rebase -s`, `jj diff -r`, `jj abandon`):

### Key Revset Symbols and Operators

- `@` - Working-copy commit (`<workspace>@` for another workspace's working copy).
- `@-` - Parent of `@` (`x-` is parents of `x`; `x+` is children of `x`).
- `root()` - The virtual root commit of the repository (`zzzzzzzz`).
- `trunk()` - Tip of the primary remote branch (`main@origin` or `master@origin`).
- `::x` / `x::` / `x::y` - DAG ancestors of `x`, descendants of `x`, and DAG range from `x` to `y`.
- `x..y` / `x..` - Ancestors of `y` that are **not** ancestors of `x` (`git log x..y`). `x..` means `x..@`.
- `x | y`, `x & y`, `~x`, `x ~ y` - Union (`OR`), intersection (`AND`), negation (`NOT`), difference.

### High-Signal Revset Functions

- `bookmarks([pattern])` / `remote_bookmarks([bookmark], [remote])` - Commits pointed to by local or remote bookmarks.
- `conflicts()` - All commits containing unresolved conflicts.
- `empty()` - Commits that introduce no file modifications compared to their parents (`~empty()` selects non-empty commits).
- `description(pattern)` / `subject(pattern)` - Match commit description text (supports `"exact:..."`, `"glob:..."`, `"regex:..."`, `"substring-i:..."`).
- `author(pattern)` / `committer(pattern)` / `mine()` - Match author/committer name or email (`mine()` matches configured user email).
- `files(fileset)` / `diff_lines(text, [files])` - Commits modifying matching file paths or diff content.
- `heads(x)` / `roots(x)` - Maximal or minimal commits within set `x`.
- `mutable()` / `immutable()` - Mutable local commits vs protected immutable history.
- `present(x)` - Evaluates to empty set instead of erroring if symbol `x` does not exist.

### Filesets

Path arguments use **fileset** expressions:

- `glob:"crates/**/*.rs"` (default when unquoted), `root-glob:"*.toml"`, `cwd-file:"SKILL.md"`.
- Combine with `|` (union), `&` (intersection), and `~` (difference), e.g., `'crates/** & ~glob:"**/tests/**"'`.

## Bookmarks and GitHub Workflows (`jj bookmark` & `jj git`)

Unlike Git branches, **bookmarks do not automatically move forward when you create a new commit with `jj new`**.

```bash
# 1. Fetch latest remote state and rebase your stack onto trunk()
jj git fetch
jj rebase -d 'trunk()'

# 2. Option A: Auto-generate a bookmark name ('push-<change-id>') and push in one step
jj git push -c @-

# 3. Option B: Create or move a named bookmark to your finished commit ('@-' or '@') and push
jj bookmark set feat/my-feature -r @-
jj git push --bookmark feat/my-feature
```

- **Updating a PR after amending/rebasing:** When you edit or rebase the commit that `feat/my-feature` points to, Jujutsu automatically moves the bookmark to the rewritten commit! Simply run `jj git push` (it force-with-lease updates the remote branch safely).
- **If you added a new commit on top with `jj new`:** Advance the bookmark explicitly with `jj bookmark set feat/my-feature -r @-` (or `jj tug` if aliased to `jj bookmark move --from 'heads(::@- & bookmarks())' --to @-`), then `jj git push`.
- **Tracking an existing remote branch:** `jj bookmark track <name>@origin` links a remote bookmark so `jj git push` can update it.

## Resolving First-Class Conflicts

When `jj rebase` or `jj new revA revB` encounters a conflict, **the command succeeds** and marks the resulting commit with `conflict`:

```bash
# 1. List conflicted commits and conflicted files
jj log -r 'conflicts()'
jj resolve --list -r <change-id>

# 2. Check out the conflicted commit (or create a child on top of it)
jj edit <change-id>

# 3. Edit the conflicted file markers directly in your editor, or restore one side
jj restore --from <side-rev> path/to/file

# 4. Inspect remaining conflict markers; once none remain, '@`'s conflict status clears
#    and all descendant commits automatically rebase onto the resolved tree!
jj st
```

### Understanding `jj`'s Default `diff` Conflict Markers

By default (`ui.conflict-marker-style = "diff"`), Jujutsu renders conflicts as a base snapshot (`%%%%%%%` or `+++++++`) plus a minimal diff:

```text
<<<<<<< conflict 1 of 1
%%%%%%% diff from: <base-commit>
\\\\\\\        to: <side-1-commit>
-old_line();
+side_1_line();
+++++++ <side-2-commit>
side_2_line();
>>>>>>> conflict 1 of 1 ends
```

- Sections starting with `%%%%%%%` show the **diff** from the merge base to that side (`-` removed, `+` added, space unchanged).
- Sections starting with `+++++++` show the **literal snapshot** of that side.
- If you prefer standard Git 3-way markers (`<<<<<<<`, `|||||||`, `=======`, `>>>>>>>`), pass `--config ui.conflict-marker-style=git` or set it in `jj config`.

## Operation Log, Undo, and Evolution Log

Every repository mutation is atomic and reversible:

```bash
# Undo the most recent jj command
jj undo

# Inspect the full operation history of the repository
jj op log -n 10

# Restore the entire repository to a prior operation ID
jj op restore <operation-id>

# Inspect how a single change ID evolved across amends, squashes, and rebases
jj evolog -r <change-id> -p
```

## Workspaces (`jj workspace`)

Share one `.jj/repo` store across multiple working directories for parallel tasks or AI agents:

```bash
# Create an isolated workspace directory at ../repo-agent-1
jj workspace add ../repo-agent-1 --name agent-1

# List active workspaces and their '@' commits
jj workspace list

# If another workspace rewrote the commit checked out in the current workspace:
jj workspace update-stale

# Stop tracking a workspace when finished
jj workspace forget agent-1
```

## Guardrails for AI Agents

- **Always pass `-m` to `jj describe`, `jj commit`, `jj new`, and `jj squash`:** Omitting `-m` on `jj describe` or `jj commit` (or running `jj split` / `jj diffedit` without a non-interactive tool) opens `$EDITOR` or a diff editor and blocks headless agent sessions.
- **To split files non-interactively without opening a diff editor:** Instead of interactive `jj split`, create a new commit before `@` (`jj new -B @ -m "first part"`) and move specific paths with `jj squash --from @ --into @- <paths...>`, or use `jj restore`.
- **Use Change IDs (`k-z`) when scripting multi-step rebases:** Commit hashes (`0-9a-f`) change whenever a commit or its ancestors are rewritten; Change IDs stay stable across rebases.
- **Check `~empty()` and `description("")` before pushing:** Commits with empty descriptions or no changes (`empty()`) cannot be pushed to Git remotes by default.
