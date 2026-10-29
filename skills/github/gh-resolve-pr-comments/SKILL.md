---
name: gh-resolve-pr-comments
description: "Triage and address human and AI bot PR review comments, push back on false positives, and reply before resolving threads. Use when asked to handle PR feedback."
disable-model-invocation: true
resources:
  - https://cli.github.com/manual/gh_pr
metadata:
  group: github
  source: t3dotgg/tstack
  license: MIT
---

<!-- omit in toc -->

# Tend to GitHub PR Comments

**Invocation:** Start only when the user explicitly requests this workflow.

Consult the [gh pr manual](resources/auto/manual-gh_pr.md) for more details.

Pull human and AI review-bot comments from the current branch's PR, verify every claim against source, align on a triage plan with the developer, apply minimal fixes or evidence-backed pushback, and reply before resolving threads.

- [1. Prerequisites](#1-prerequisites)
- [2. Identify the PR](#2-identify-the-pr)
- [3. Pull All Comments](#3-pull-all-comments)
- [4. Assess and Categorize](#4-assess-and-categorize)
- [5. Present the Plan](#5-present-the-plan)
- [6. Execute the Plan](#6-execute-the-plan)
- [7. Resolve Comment Threads](#7-resolve-comment-threads)
- [8. Wrap Up](#8-wrap-up)

## 1. Prerequisites

Verify the environment is ready.

**Check authentication:**

```bash
gh auth status
```

If not authenticated, stop and tell the user to run `gh auth login`.

**Check current branch:**

```bash
git branch --show-current
```

Confirm you are not on the default branch. If you are, stop and ask the user to check out their feature branch.

## 2. Identify the PR

**Get PR details:**

```bash
gh pr view --json number,url,headRefName,baseRefName
```

If no PR exists for the current branch, stop and tell the user. Record `baseRefName` as `BASE_BRANCH` for diff-size checks in step 6.

**Get repo owner/name:**

```bash
gh repo view --json nameWithOwner -q '.nameWithOwner'
```

Parse this into `{owner}` and `{repo}` for API calls below.

## 3. Pull All Comments

Fetch three types of comments and combine them.

**Inline review comments:**

```bash
gh api repos/{owner}/{repo}/pulls/{pr_number}/comments --paginate
```

**Review-level comments:**

```bash
gh api repos/{owner}/{repo}/pulls/{pr_number}/reviews --paginate
```

**General PR conversation comments:**

```bash
gh api repos/{owner}/{repo}/issues/{pr_number}/comments --paginate
```

**Get current user for reference:**

```bash
gh api user -q '.login'
```

For each comment, extract: `id`, `node_id`, `user.login`, `user.type`, `body`, `path`, `line`/`original_line`, `diff_hunk`, `in_reply_to_id`, `created_at`.

**Filter out only:**

- Non-actionable CI and deploy status-bot top-level comments (`github-actions`, `vercel`, `netlify`, `railway`, `cloudflare`) whose output duplicates check runs
- Already-resolved review threads

**Retain:**

- Inline threads and actionable review findings from AI code-review bots (such as `greptile`, `macroscope`, `coderabbit`, `copilot`, or `cursor` bugbot)
- Human reviewer comments and self-review comments authored by the current user (often left as action items)

When a bot finding cites repo-local rule files (such as `.macroscope/check-run-agents/*.md`), read those rule files before categorizing or replying.

**Group** inline review comments into threads by `in_reply_to_id`. The latest comment in a thread determines the thread's status.

If zero comments remain after filtering, report "No open PR comments to address" and stop.

## 4. Assess and Categorize

For each comment or thread, verify the reviewer or bot claim against the actual source code at the referenced `file:line`, surrounding callers, and diff hunk before classifying into one of these categories:

| Category        | Description                                                         | Example                                           |
| --------------- | ------------------------------------------------------------------- | ------------------------------------------------- |
| **Fix**         | Verified bug or actionable in-scope improvement                     | "This dereferences `user` before the null check"  |
| **Decline**     | False positive, wrong claim, speculative guard, or out-of-scope nit | "Add a fallback cache for an unreachable state"   |
| **Investigate** | Needs deeper codebase exploration before deciding                   | "Is this duplicated anywhere else?"               |
| **Discuss**     | Design question, trade-off, or needs user direction                 | "Should we use strategy A or B here?"             |
| **Acknowledge** | FYI or praise, no code change needed                                | "Nice refactor"                                   |
| **Outdated**    | Comment on code that has already changed                            | Diff hunk no longer matches current file contents |

**Rules:**

- For **Fix** items: read the actual file and verify the defect before proposing a change. Do not guess.
- For **Decline** items: cite concrete `file:line`, test, or runtime behavior showing why the claim does not hold or falls outside scope; record legitimate out-of-scope ideas as follow-ups rather than expanding the PR.
- For **Investigate** items: note what to explore, saving deep exploration for step 6.
- For **Discuss** items: summarize the trade-off and prepare to ask the user.
- For **Outdated** detection: check whether `diff_hunk` still matches the current file content or `position` is `null`.

## 5. Present the Plan

**Stop here and present the plan to the user.** Use this format:

```markdown
## PR Comment Triage: {pr_url}

### Fix ({count})

- [ ] **{file}:{line}** - {summary of minimal change}
  > {abbreviated reviewer or bot comment}

### Decline ({count})

- [ ] **{file}:{line}** - {file:line or behavior reason to push back; note as follow-up if valid out of scope}
  > {abbreviated reviewer or bot comment}

### Investigate ({count})

- [ ] **{file}:{line}** - {what to explore and why}
  > {abbreviated reviewer or bot comment}

### Discuss ({count})

- [ ] **{file}:{line}** - {question or design decision}
  > {abbreviated reviewer or bot comment}

### Acknowledge ({count})

- {comment summary} - will resolve, no code change needed

### Outdated ({count})

- {comment summary} - code has changed, will resolve
```

Ask the user:

> Here is the plan for addressing the PR comments. Do you want to:
>
> 1. Proceed as-is
> 2. Reclassify any items (e.g., move a Fix to Decline or Discuss)
> 3. Skip specific items
>
> Let me know before I start making changes.

**Do not proceed until the user confirms.**

## 6. Execute the Plan

Work through the approved plan in this order:

1. **Fix** items first - apply one surgical change at a time, preferring fixes that delete or narrow code over adding new helpers, options, caches, locks, or defensive wrappers.
2. **Investigate** items - explore the codebase, then reclassify to **Fix**, **Decline**, or **Discuss** based on verified evidence.
3. **Decline** items - prepare concise `file:line`, test, or behavior pushback notes (never agree with wrong feedback merely to close a thread).
4. **Discuss** items - present verified context and ask the user for direction.

After applying code changes, check for review-induced diff bloat against the PR base branch:

```bash
git diff --shortstat "$BASE_BRANCH"
```

If review fixes inflated the diff with speculative guards or helpers the user's goal does not need, trim them before continuing and run the repository's covering checks.

Show an execution summary:

```markdown
## Changes Made

| File | Line | Category    | Action Taken                               |
| ---- | ---- | ----------- | ------------------------------------------ |
| …    | …    | Fix         | Deleted redundant branch and updated check |
| …    | …    | Decline     | Replied with file:line rationale           |
| …    | …    | Investigate | Verified caller invariant; replied         |
| …    | …    | Discuss     | User decided to defer                      |
```

## 7. Resolve Comment Threads

**Immediately after execution completes**, post replies and resolve addressed comment threads on GitHub (`Fix`, `Decline`, resolved `Investigate`, `Acknowledge`, and `Outdated` items). Skip resolution for **Discuss** items the user deferred.

1. **Reply before resolving:** For every `Fix`, `Decline`, and resolved `Investigate` thread (and any `Outdated` thread where pointing to the new location helps), post a concise reply formatted per `/gh-pr-comments` (including its secret preflight scan and human-account `[!NOTE]` attribution) and confirm GitHub returned the reply URL:

   ```bash
   gh api "repos/{owner}/{repo}/pulls/{pr_number}/comments/{comment_id}/replies" \
     -f body="<formatted reply>" --jq '.html_url'
   ```

   Do not resolve a thread until its reply URL is confirmed.

2. **Match thread IDs via GraphQL:** A REST review comment has no field back to its parent thread, so query `reviewThreads` and match each thread to the addressed comment by `databaseId` (the REST `id` from step 3) or node `id`:

   ```bash
   gh api graphql --paginate -f query='
   query($owner: String!, $repo: String!, $pr: Int!, $endCursor: String) {
     repository(owner: $owner, name: $repo) {
       pullRequest(number: $pr) {
         reviewThreads(first: 100, after: $endCursor) {
           nodes {
             id
             isResolved
             comments(first: 100) {
               nodes {
                 id
                 databaseId
               }
             }
           }
           pageInfo {
             hasNextPage
             endCursor
           }
         }
       }
     }
   }' -f owner="{owner}" -f repo="{repo}" -F pr={pr_number}
   ```

3. **Resolve the matched thread:**

   ```bash
   gh api graphql -f query='
   mutation($threadId: ID!) {
     resolveReviewThread(input: {threadId: $threadId}) {
       thread {
         isResolved
       }
     }
   }' -f threadId="{thread_id}"
   ```

## 8. Wrap Up

Ask the user:

> All comments have been addressed, replies posted, and {M} threads resolved on GitHub. Ready to:
>
> 1. Commit with message `fix: address PR review feedback` (edit to match repository conventions)
> 2. Push to remote
>
> Proceed? Or would you like to review the changes first?

**Do not proceed until the user confirms.**

**Commit and push:**

```bash
git add <specific changed files>
```

```bash
git commit -m "fix: address PR review feedback"
```

```bash
git push
```

**Report final status:**

```markdown
## Done

- {N} comments addressed ({declined_count} declined with rationale)
- {M} threads resolved on GitHub after confirmed replies
- Commit: {short_sha}
- PR: {pr_url}
```

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).
