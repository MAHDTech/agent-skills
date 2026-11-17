---
name: gh-create-pr
description: Draft a problem-first Conventional Commit PR title and description, then create or update the PR once the user approves the draft. Use when asked to open or update a pull request.
disable-model-invocation: true
resources:
  - https://cli.github.com/manual/gh_pr
  - https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax
metadata:
  group: github
  source: t3dotgg/tstack
  license: MIT
---

# Create a Pull Request

**Invocation:** Start only when the user explicitly requests this workflow.

Consult the `gh pr` manual in `resources/auto/manual-gh_pr.md` for more details.

Draft a concise PR title and description from the diff against a base branch, get the user's approval, then create or update the PR.

## 1. Determine the base branch

If the user passed a branch name as an argument (e.g. `/gh-create-pr feature-branch`), use it as `BASE_BRANCH` and skip auto-detection. Otherwise detect the repository's default branch, trying each method until one succeeds:

```bash
BASE_BRANCH=$(gh repo view --json defaultBranchRef -q '.defaultBranchRef.name' 2>/dev/null)
[ -n "$BASE_BRANCH" ] || BASE_BRANCH=$(git remote show origin 2>/dev/null | grep "HEAD branch" | cut -d: -f2 | xargs)
[ -n "$BASE_BRANCH" ] || BASE_BRANCH=$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')
```

If every method fails, ask the user which branch to use; never fall back to a guessed `main` or `master`. Verify the branch exists before proceeding, and stop to ask the user if it does not:

```bash
git rev-parse --verify "$BASE_BRANCH" 2>/dev/null || git rev-parse --verify "origin/$BASE_BRANCH" 2>/dev/null
```

## 2. Analyze the changes

Exclude lockfiles from the stat so generated churn does not drown the real changes; dependency manifests such as `package.json` stay in:

```bash
git diff "$BASE_BRANCH" --stat -- ":(exclude)*.lock" ":(exclude)package-lock.json" ":(exclude)pnpm-lock.yaml"
git log "$BASE_BRANCH"..HEAD --oneline
```

## 3. Draft the title and description

Read [pr-format.md](resources/manual/pr-format.md) before drafting, and follow its title format, description template, admonition rules, section rules, and example.

Write the description to a temp file outside the working tree so it never lands in a commit:

```bash
PR_BODY="$(mktemp -d)/PR_DESCRIPTION.md"
```

Before displaying the draft:

- Scan the title, body, quoted snippets, command output, and any attachment links for secrets and private details. Follow `/gh-pr-comments` for the full checklist when it is installed.
- Keep the description free of comment attribution callouts and `Created with <model> in <harness>` footers; those belong to comments, not PR descriptions.

Display the title and description to the user.

## 4. Get approval

Ask the user to approve as-is, request changes (then re-draft), or reject (stop here). Proceed to step 5 only after explicit approval.

## 5. Commit, push, and create or update the PR

Re-run the secret and private-detail scan if the user edited the draft, then:

1. Commit pending changes, if any. Stage only the specific files this PR should include:

   ```bash
   git add <specific intended files>
   git commit -m "<generated title>"
   ```

2. Push the branch:

   ```bash
   git push -u origin HEAD
   ```

3. Create or update the PR. Open a ready (non-draft) PR by default so CI and review bots run immediately; pass `--draft` only when the user asks for one. Check for an existing PR first:

   ```bash
   gh pr view --json number,url 2>/dev/null
   ```

   If a PR exists, update it; otherwise create one, always passing `--base` so it targets the right branch:

   ```bash
   gh pr edit --title "<generated title>" --body-file "$PR_BODY"
   gh pr create --base "$BASE_BRANCH" --title "<generated title>" --body-file "$PR_BODY"
   ```

   If `gh pr create` or `gh pr edit` fails because the GraphQL rate limit is exhausted, use the REST API instead (`/gh-babysit-pr` covers checking the GraphQL budget):

   ```bash
   gh api "repos/{owner}/{repo}/pulls" -f title="<generated title>" -f head="$(git branch --show-current)" \
     -f base="$BASE_BRANCH" -F body=@"$PR_BODY" --jq '.html_url'
   gh api -X PATCH "repos/{owner}/{repo}/pulls/<pr>" -f title="<generated title>" -F body=@"$PR_BODY" --jq '.html_url'
   ```

4. Confirm GitHub returned the PR URL, then delete the temp file with `rm -rf "$(dirname "$PR_BODY")"`.

Done when the PR URL is confirmed and reported to the user.

If the user also asked to monitor the PR through CI and reviews, continue with `/gh-babysit-pr`. Post a follow-up PR comment only when the user's request calls for one, following `/gh-pr-comments`.

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).
