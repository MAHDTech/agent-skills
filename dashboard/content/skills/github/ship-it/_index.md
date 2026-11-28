+++
title = "ship-it"
description = "Commit pending changes, push the branch, and open or update a pull request when the user says \"ship it\". Trigger only on the phrase \"ship it\", not general commit or PR requests."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Ship It

Fast-path shortcut triggered when the user says "ship it": verify branch and changes, commit pending work, push to `origin`, and open or update the pull request without pausing for a separate draft-approval round.

Only run this workflow when the user explicitly says "ship it" (or an unambiguous variant such as "ship it now"). For interactive draft review before opening a PR, use `/gh-create-pr`.

## 1. Detect the base branch and ensure a feature branch

Detect the repository's default branch, trying each method until one succeeds:

```bash
BASE_BRANCH=$(gh repo view --json defaultBranchRef -q '.defaultBranchRef.name' 2>/dev/null)
[ -n "$BASE_BRANCH" ] || BASE_BRANCH=$(git remote show origin 2>/dev/null | grep "HEAD branch" | cut -d: -f2 | xargs)
[ -n "$BASE_BRANCH" ] || BASE_BRANCH=$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')
```

If every method fails, stop and ask the user which base branch to target. Fetch the base ref and capture `BASE_REF` (preferring `origin/$BASE_BRANCH`):

```bash
git fetch origin "$BASE_BRANCH" 2>/dev/null || true
BASE_REF=""
if git rev-parse --verify "origin/$BASE_BRANCH" >/dev/null 2>&1; then
  BASE_REF="origin/$BASE_BRANCH"
elif git rev-parse --verify "$BASE_BRANCH" >/dev/null 2>&1; then
  BASE_REF="$BASE_BRANCH"
fi
```

If `BASE_REF` is still empty (neither `origin/$BASE_BRANCH` nor `$BASE_BRANCH` exists locally), stop and ask the user for a valid base branch or ref before continuing.

Check the current branch with `git branch --show-current`:

- If on `BASE_BRANCH` or detached `HEAD`, create a descriptive feature branch before committing (for example `git checkout -b feat/<short-slug>` or `fix/<short-slug>`). Never commit or push directly to the default branch.
- If already on a non-default feature branch, stay on it.

## 2. Inspect changes and run secret preflight

Check working tree state and commits ahead of `BASE_REF`:

```bash
git status --short
git log "$BASE_REF"..HEAD --oneline
```

- If there are no staged/unstaged changes and zero commits ahead of `BASE_REF`, report that there is nothing to ship and stop.
- Stage only the intended files for the current work by explicit path. Exclude sensitive or temporary files (`.env`, private keys, tokens, credentials, swap files). If unrelated or suspicious untracked files are present, stop and ask the user before staging them.
- Before committing, scan staged changes and the planned commit message for secrets, credentials, internal URLs, or private paths (follow `/gh-pr-comments` for the full checklist when installed). If any secret or sensitive detail is found, stop before committing and report it to the user.

## 3. Commit pending changes

If there are staged or pending changes to commit:

1. Write a Conventional Commit subject (`type(scope): description`, lowercase type, 70 characters or fewer, no trailing period, no em-dashes).
2. Commit with repository hooks enabled (never pass `--no-verify`). If a pre-commit hook reformats files or reports an issue, fix the issue, re-stage, and commit cleanly:

   ```bash
   git add <specific intended files>
   git commit -m "<conventional commit subject>"
   ```

If the working tree is already clean and the branch has commits ahead of `BASE_REF`, skip committing and proceed to step 4.

## 4. Push and open or update the PR

1. Scan every outgoing commit's diff (including merge-commit diffs via `-m`), earlier commits on the branch, and commit messages (including any hook-modified commit from step 3) for secrets, credentials, internal URLs, or private paths before pushing:

   ```bash
   git log -p -m "$BASE_REF"..HEAD
   ```

   If any outgoing commit contains a secret or sensitive detail (even if a later commit removes it), stop before pushing and report it to the user.

2. Push the branch and set upstream tracking:

   ```bash
   git push -u origin HEAD
   ```

3. Check for an existing open PR on the current branch so you summarize against its actual base branch when updating an existing PR (for example a release or stacked branch):

   ```bash
   gh pr view --json number,url,state,baseRefName 2>/dev/null
   ```

   If an existing PR is returned with `state == "OPEN"` and its `baseRefName` differs from `BASE_BRANCH`, fetch `origin "$baseRefName"` and set `BASE_BRANCH` and `BASE_REF` to that PR's base branch (stopping to ask the user if neither `origin/$BASE_BRANCH` nor `$BASE_BRANCH` exists locally).

4. Draft the PR title (Conventional Commit format, 70 characters or fewer) and concise PR body from `git diff "$BASE_REF"...HEAD --stat` and `git log "$BASE_REF"..HEAD --oneline`. Follow [pr-format.md](@/skills/github/gh-create-pr/resources/manual/pr-format.md) when installed; otherwise include a one-sentence summary, bulleted key changes, and verification results. Omit comment attribution callouts and `Created with <model>` footers.

5. Write the body to a temporary file outside the repository so it never dirties the working tree, then scan the final PR title and `$PR_BODY` contents for secrets, credentials, internal URLs, or private paths (follow `/gh-pr-comments` for the full checklist when installed):

   ```bash
   PR_BODY="$(mktemp -d)/PR_DESCRIPTION.md"
   ```

   If anything sensitive is present, redact it or stop and ask the user before publishing.

6. Update the existing open PR or create a ready (non-draft) PR targeting `BASE_BRANCH`:

   ```bash
   # If step 3 found a PR with state == "OPEN":
   gh pr edit <number> --title "<title>" --body-file "$PR_BODY"

   # If no PR exists, or the returned PR is CLOSED or MERGED:
   gh pr create --base "$BASE_BRANCH" --title "<title>" --body-file "$PR_BODY"
   ```

   If `gh pr create` or `gh pr edit` fails due to a GraphQL rate limit, fall back to the REST API (`gh api "repos/{owner}/{repo}/pulls" ...`).

7. Delete the temporary directory with `rm -rf "$(dirname "$PR_BODY")"`.

## Completion criteria

- Pending work is committed cleanly on a non-default branch with hooks passing.
- The branch is pushed to `origin` with upstream tracking configured.
- The PR is created or updated against `BASE_BRANCH`, the temporary body file is removed, and the branch name, commit SHA, and PR URL are reported to the user.

