+++
title = "jj"
description = "Use Jujutsu (jj) in repositories with .jj, or for jj-specific tasks. Inspect history, shape changes, write revsets/filesets, resolve conflicts, and manage bookmarks or remotes."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


<!-- cspell:ignore Jujutsu jujutsu revset revsets fileset filesets evolog colocated colocate colocation mergetool diffedit kxyz kkmpptxz zzzzzzzz reflog -->

# Jujutsu (`jj`) version control

Jujutsu is a Git-compatible VCS. The working copy is snapshotted as a real commit (`@`), commits carry a stable **change ID** alongside the Git commit hash, conflicts are stored inside commits instead of halting a rebase, and every repository mutation is recorded in an undoable **operation log**.

When a repository contains a `.jj/` directory (standalone or colocated with `.git/`), use `jj` commands for every mutation instead of mutating `git` commands.

## Reference files

The vendored upstream docs in `resources/auto/` are the source of truth for syntax and edge cases:

- Git command equivalents: `resources/auto/jj-vcs-jj-main-docs-git-command-table.yml.md` (one entry per use case, with the Git and jj commands).
- Every command and flag: `resources/auto/jj-vcs-jj-main-cli-tests-cli-reference.md.snap.md`. The installed `jj help <command>` is authoritative when the two differ.
- Revset functions and string patterns: `resources/auto/jj-vcs-jj-main-docs-revsets.md`; path patterns: `resources/auto/jj-vcs-jj-main-docs-filesets.md`.
- Bookmarks, pushing, and GitHub: `resources/auto/jj-vcs-jj-main-docs-bookmarks.md`, `resources/auto/jj-vcs-jj-main-docs-github.md`.
- Conflicts and the operation log: `resources/auto/jj-vcs-jj-main-docs-conflicts.md`, `resources/auto/jj-vcs-jj-main-docs-operation-log.md`.
- Colocation and Git interop: `resources/auto/jj-vcs-jj-main-docs-git-compatibility.md`; settings: `resources/auto/jj-vcs-jj-main-docs-config.md`.

## Core concepts

| Concept                      | Behaviour                                                                                                                                       |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Working copy (`@`)           | Every `jj` command snapshots file edits into `@`. There is no staging area, and stashing is unnecessary because edits already live in a commit. |
| Change ID vs commit ID       | The change ID (letters `k`-`z`, e.g. `kkmpptxz`) survives amends and rebases; the hex commit ID changes on every rewrite.                       |
| Bookmarks                    | Named pointers used for Git remotes and pull requests. They follow a rewritten commit but do not advance when you run `jj new`.                 |
| First-class conflicts        | A conflicting rebase or merge succeeds and records the conflict in the commit; resolve it later.                                                |
| Automatic descendant rebase  | Rewriting a commit (amend, squash, edit, resolve) rebases all of its descendants.                                                               |
| Operation and evolution logs | `jj op log` records every repository state (`jj undo` reverts the last one); `jj evolog` shows how one change evolved.                          |

New repositories are colocated by default (`jj git init`, `jj git clone`); pass `--no-colocate` or set `git.colocate = false` to opt out.

## Workflows

### Squash workflow

Describe the feature commit, then work in a scratch commit on top and move finished edits down:

```bash
jj describe -m "feat(parser): support nested revset aliases"
jj new                 # scratch commit '@' on top of the feature commit '@-'
jj st
jj diff                # changes in '@'
jj diff -r @-          # changes in the feature commit
jj squash              # move everything from '@' into '@-'
jj squash path/to/file # move only matching paths
```

### Edit workflow

Work directly in a commit, or jump back into the stack:

```bash
jj commit -m "fix(cli): handle empty fileset union"   # describe '@' and start a new empty commit
jj edit <change-id>                                    # edit an earlier commit; descendants rebase
jj new <tip-change-id>                                 # return to the top of the stack
```

### Rebasing

```bash
jj rebase -r <rev> -o <dest>      # one commit (its children stay behind)
jj rebase -s <source> -o <dest>   # a commit and its descendants
jj rebase -b <branch> -o <dest>   # the whole branch containing <branch>
jj duplicate <rev> -o <dest>      # copy, like cherry-pick
```

`-o` is short for `--onto`. Use `-A`/`--insert-after` or `-B`/`--insert-before` to place commits inside a stack.

## Revsets and filesets

Revsets select commits for any `-r`, `-s`, `-b`, or `-o` argument:

- `@` is the working-copy commit; `x-` is the parents of `x`, `x+` its children.
- `::x` ancestors, `x::` descendants, `x::y` the DAG range; `x..y` is ancestors of `y` that are not ancestors of `x` (like `git log x..y`).
- `|` union, `&` intersection, `~x` negation, `x ~ y` difference.
- `trunk()` is the main remote branch tip, `conflicts()` the conflicted commits, `mutable()` the rewritable ones, `mine()` your commits. Look up any other function in the revsets reference.

Filesets select paths, relative to the current directory. A bare `"path"` is a `prefix-glob:` pattern: it matches that path and everything below it. Use `glob:"**/*.rs"` for a plain glob and the `root:`, `root-file:`, `root-glob:` kinds to anchor at the workspace root, and combine with `|`, `&`, `~`, for example `'crates/** & ~glob:"**/tests/**"'`.

## Bookmarks and pushing

```bash
jj git fetch
jj rebase -o 'trunk()'            # rebase the current branch onto trunk

jj git push -c @-                 # create a 'push-<change-id>' bookmark and push it

jj bookmark set feat/my-feature -r @-
jj git push --bookmark feat/my-feature
```

- After amending or rebasing the bookmarked commit, the bookmark follows it; run `jj git push` again (it behaves like `--force-with-lease`).
- After `jj new` adds a commit on top, move the bookmark explicitly: `jj bookmark set feat/my-feature -r @-`, then push.
- Track an existing remote branch with `jj bookmark track <name> --remote=origin` (or `jj bookmark track <name>@origin`).
- `jj git push` refuses commits with empty descriptions and private commits; describe them, or pass `--allow-empty-description` / `--allow-private` when the user wants that.

## Resolving conflicts

A conflicting `jj rebase` or `jj new A B` succeeds and marks the result as conflicted:

```bash
jj log -r 'conflicts()'
jj resolve --list -r <change-id>
jj new <change-id>                        # or 'jj edit <change-id>' to fix in place
jj restore --from <side-rev> path/to/file # take one side wholesale, or edit the markers
jj st                                     # the conflict clears once no markers remain
```

Once the fix is squashed into (or made in) the conflicted commit, its descendants rebase onto the resolved tree.

The default marker style (`ui.conflict-marker-style = "diff"`) shows one side as a diff from the base (a `%%%%%%%` section with `-`/`+` lines) and the other as a literal snapshot (a `+++++++` section). For Git-style markers, pass `--config ui.conflict-marker-style=git`. Details are in the conflicts reference.

## Undo and history

```bash
jj undo                      # revert the last operation
jj op log -n 10
jj op restore <operation-id> # return the whole repo to an earlier operation
jj evolog -r <change-id> -p  # how one change evolved
```

## Workspaces

Several working directories can share one repository, for example one per parallel agent:

```bash
jj workspace add ../repo-agent-1 --name agent-1
jj workspace list
jj workspace update-stale   # after another workspace rewrote this one's '@'
jj workspace forget agent-1
```

## Headless agent guardrails

- Pass `-m` to `jj describe`, `jj commit`, and `jj split`; without it they open `$EDITOR` and block a headless session. When both sides of a `jj squash` have descriptions it also opens the editor: pass `-m` or `-u` (keep the destination's description).
- Split by path without a diff editor: `jj split -m "first part" <filesets>` puts the matching files in the first commit and leaves the rest in a child. Without filesets, `jj split` opens the interactive diff editor, as do `-i` and `--tool` on any command.
- Script multi-step rewrites with change IDs; commit IDs change on every rewrite.

