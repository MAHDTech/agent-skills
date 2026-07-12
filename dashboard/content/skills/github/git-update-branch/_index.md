+++
title = "git-update-branch"
description = "Bring a feature branch up to date with its base branch safely — fetch first, detect the base, choose rebase vs merge deliberately, use --force-with-lease, and hand conflicts off cleanly. Use when a branch has fallen behind its base, when the user wants to rebase or merge in the latest from the base/default branch, when a PR reports merge conflicts or an out-of-date branch, or when deciding between rebase and merge for an update."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Update a Branch Against Its Base

Sync a feature branch with the base branch it will merge into, choosing the integration strategy deliberately and doing it without clobbering anyone's work.

When the update stops on conflicts, hand off to `/git-resolve-conflicts`.

## 1. Fetch and detect the base

Always fetch first so you integrate against the true remote tip, not a stale local copy:

```bash
git fetch origin --prune
```

Detect the base branch — do **not** assume `main` or `master`. If the user named one, use it. Otherwise try in order until one succeeds:

```bash
BASE=$(gh repo view --json defaultBranchRef -q '.defaultBranchRef.name' 2>/dev/null)
BASE=${BASE:-$(git remote show origin 2>/dev/null | sed -n 's/.*HEAD branch: //p')}
BASE=${BASE:-$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')}
```

If a PR exists, its `baseRefName` is authoritative: `gh pr view --json baseRefName -q '.baseRefName'`. If detection fails, ask the user rather than guessing.

## 2. Secure the working tree

An update rewrites or moves your checkout, so start clean:

```bash
git status --short
```

If there are uncommitted changes, commit them or `git stash push` first, and restore with `git stash pop` after the update. Never start a rebase or merge with a dirty tree.

## 3. Choose rebase vs merge

| Factor               | Rebase                                           | Merge                                         |
| -------------------- | ------------------------------------------------ | --------------------------------------------- |
| History              | Linear — replays your commits on top of the base | Preserves both histories, adds a merge commit |
| Shared-branch safety | Unsafe — rewrites your commit SHAs               | Safe — never rewrites existing commits        |
| Conflicts            | May resurface per replayed commit                | Resolved once, in the merge commit            |
| Push after           | Needs `--force-with-lease`                       | Plain `git push`                              |

**The golden rule**: never rebase a branch other people have based work on or pulled. Rewriting shared history forces every collaborator to recover their copy by hand. If anyone else is building on this branch, merge.

Default: rebase a solo, unshared feature branch to keep history linear; merge when the branch is shared, or when preserving the exact integration history matters more than a clean line.

## 4. Integrate

**Rebase** onto the freshly fetched base tip:

```bash
git rebase "origin/$BASE"
```

**Merge** the base in:

```bash
git merge "origin/$BASE"
```

To bail out at any point and return to the pre-update state, use `git rebase --abort` or `git merge --abort`.

## 5. Handle conflicts

If the rebase or merge stops with conflicts, stop and invoke `/git-resolve-conflicts` to work through them. After it stages the resolutions, continue with `git rebase --continue` (repeat until the replay finishes) or `git commit` to seal the merge.

## 6. Push

After a **merge**, push normally:

```bash
git push
```

After a **rebase**, the remote branch and your rewritten local branch have diverged, so push with a lease — never a bare `--force`:

```bash
git push --force-with-lease
```

`--force-with-lease` refuses the push if the remote moved since your last fetch, protecting a collaborator's commits you have not seen; a plain `--force` overwrites them unconditionally. If the lease is rejected, re-fetch, reconcile, and only then push again.

Done when the branch contains the base's latest commits, the tree is clean, and the push succeeds.

