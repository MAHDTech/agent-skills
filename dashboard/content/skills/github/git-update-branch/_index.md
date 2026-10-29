+++
title = "git-update-branch"
description = "Assess stale or diverged PRs for conceptual conflicts and update a branch against its base via rebase or merge. Use when asked to modernize or update a branch or PR."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Update a Branch Against Its Base

**Invocation:** Start only when the user explicitly requests this workflow.

Sync a feature branch or pull request with the base branch it will merge into: verify that its design still fits the current base, choose rebase or merge deliberately, and update without clobbering anyone's work.

When the update stops on conflicts, hand off to `/git-resolve-conflicts`.

## 1. Fetch and detect the base

Always fetch first so you integrate against the true remote tip, not a stale local copy:

```bash
git fetch origin --prune
```

Detect the base branch - do **not** assume `main` or `master`. If the user named one, use it. Otherwise try in order until one succeeds:

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

Treat existing uncommitted changes in the active checkout as user-owned:

- When the active worktree is dirty or occupied by other work, create a disposable worktree at the recorded branch or PR head (`git worktree add ../<disposable-dir> <branch-or-sha>`) rather than discarding or entangling unrelated edits.
- On a solo checkout where the user wants in-place work, commit intended changes or run `git stash push` first and restore with `git stash pop` after the update.
- Never start a rebase or merge with a dirty tree.

## 3. Assess stale or diverged PR intent

A clean Git rebase does not prove an older PR still belongs in the codebase. Before integrating a diverged branch or aging PR, record its URL, current head SHA, and merge base, then inspect what landed on `origin/$BASE` since divergence:

```bash
MERGE_BASE=$(git merge-base HEAD "origin/$BASE")
git log --oneline "$MERGE_BASE..origin/$BASE"
```

Read the PR title, body, linked issues, review threads, and diff to pin down the smallest coherent problem it solves, then check `origin/$BASE` for conceptual conflicts beyond overlapping lines:

- Was the same user-facing problem already solved on `origin/$BASE`?
- Did APIs, types, schemas, dependencies, or architectural boundaries shift?
- Do both sides now claim ownership of the same behavior or invariant?
- Was the target code deleted, moved, or intentionally simplified?
- Would landing the branch restore retired behavior or create two ways to do the same thing?
- Does the remaining benefit still justify the integration and maintenance cost?

Select a disposition before integrating:

- **`MODERNIZE`:** Goal and implementation shape still fit. Proceed with rebase or merge, verification, and push.
- **`REWORK`:** Goal is still valid, right idea, wrong shape for the current base. Explain the smallest credible rewrite and stop before pushing unless the user also requested that rework.
- **`CLOSE`:** PR is superseded, obsolete, or directionally wrong. Cite concrete base-branch commits or PRs and stop without rebasing. Never close or merge a PR without explicit user permission.

## 4. Choose rebase vs merge

| Factor               | Rebase                                           | Merge                                         |
| -------------------- | ------------------------------------------------ | --------------------------------------------- |
| History              | Linear - replays your commits on top of the base | Preserves both histories, adds a merge commit |
| Shared-branch safety | Unsafe - rewrites your commit SHAs               | Safe - never rewrites existing commits        |
| Conflicts            | May resurface per replayed commit                | Resolved once, in the merge commit            |
| Push after           | Needs `--force-with-lease`                       | Plain `git push`                              |

**The golden rule**: never rebase a branch other people have based work on or pulled. Rewriting shared history forces every collaborator to recover their copy by hand. If anyone else is building on this branch, merge.

Default: rebase a solo, unshared feature branch to keep history linear; merge when the branch is shared, or when preserving the exact integration history matters more than a clean line.

## 5. Integrate and verify

Record the pre-update commit and remote branch SHA before rewriting history:

```bash
BRANCH=$(git branch --show-current)
PRE_REBASE_HEAD=$(git rev-parse HEAD)
REMOTE_SHA=$(git rev-parse "origin/$BRANCH" 2>/dev/null || true)
```

**Rebase** onto the freshly fetched base tip:

```bash
git rebase "origin/$BASE"
```

**Merge** the base in:

```bash
git merge "origin/$BASE"
```

To bail out at any point and return to the pre-update state, use `git rebase --abort` or `git merge --abort`.

After rebasing, compare the commit series and final diff, then run focused checks for the touched area:

```bash
git range-diff "origin/$BASE" "$PRE_REBASE_HEAD" HEAD
git diff "origin/$BASE"...HEAD
```

Confirm that no intentional behavior disappeared and no already-landed base changes were reverted or duplicated. Run the repository's focused tests or lint checks through its documented environment wrapper, separating branch regressions from pre-existing or infrastructure failures.

## 6. Handle conflicts

If the rebase or merge stops with conflicts, stop and invoke `/git-resolve-conflicts` to work through them according to the branch's intent and the base branch's newer invariants. If a conflict reveals a product or architecture disagreement, stop and reconsider `REWORK` or `CLOSE` instead of guessing. After resolutions are staged, continue with `git rebase --continue` (repeat until the replay finishes) or `git commit` to seal the merge.

## 7. Push and report

After a **merge**, push normally:

```bash
git push
```

After a **rebase**, the remote branch and your rewritten local branch have diverged, so push with `--force-with-lease` bound to the remote head observed before rebasing - never a bare `--force`:

```bash
git push --force-with-lease="${BRANCH}:${REMOTE_SHA}" origin "$BRANCH"
```

When the branch has no prior remote ref or `REMOTE_SHA` matches the freshly fetched remote-tracking ref, `git push --force-with-lease` provides the same lease check. `--force-with-lease` refuses the push if the remote moved since your fetch, protecting commits you have not seen; if the lease is rejected, re-fetch, reconcile, and reassess before pushing again.

Do not merge the PR or post a PR comment unless the user asked for one; format any requested PR comment with `/gh-pr-comments`.

Lead your report with `MODERNIZE`, `REWORK`, or `CLOSE` (when assessing a PR), followed by base-branch changes, conceptual-conflict findings, `git range-diff` and check results, and push status. Done when a `MODERNIZE` branch contains the base's latest commits, focused checks pass, the tree is clean, and the push succeeds (or when `REWORK` / `CLOSE` is reported cleanly without mutating the remote).

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).

