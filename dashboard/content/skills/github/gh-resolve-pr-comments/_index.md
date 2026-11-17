+++
title = "gh-resolve-pr-comments"
description = "Triage and address human and AI bot PR review comments, push back on false positives, and reply before resolving threads. Use when asked to handle PR feedback."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Tend to GitHub PR Comments

**Invocation:** Start only when the user explicitly requests this workflow.

Consult the `gh pr` manual in `resources/auto/manual-gh_pr.md` for more details.

Pull human and AI review-bot comments from the current branch's PR, verify every claim against source, align on a triage plan with the developer, apply minimal fixes or evidence-backed pushback, push, then reply and resolve threads citing the pushed commit.

## 1. Prerequisites

```bash
gh auth status
git branch --show-current
```

If `gh` is not authenticated, stop and tell the user to run `gh auth login`. If the current branch is the default branch, stop and ask the user to check out their feature branch.

## 2. Identify the PR

```bash
gh pr view --json number,url,headRefName,baseRefName
gh repo view --json nameWithOwner -q '.nameWithOwner'
```

If no PR exists for the current branch, stop and tell the user. Parse `nameWithOwner` into `{owner}` and `{repo}` for the API calls below.

## 3. Pull all comments and thread state

Fetch the three REST comment types and the current user:

```bash
gh api repos/{owner}/{repo}/pulls/{pr_number}/comments --paginate   # inline review comments
gh api repos/{owner}/{repo}/pulls/{pr_number}/reviews --paginate    # review-level comments
gh api repos/{owner}/{repo}/issues/{pr_number}/comments --paginate   # PR conversation comments
gh api user -q '.login, .type'
```

For each comment, extract `id`, `node_id`, `user.login`, `user.type`, `body`, `path`, `line`/`original_line`, `diff_hunk`, `in_reply_to_id`, and `created_at`.

REST data has no resolved state, so also fetch every review thread with the `reviewThreads` GraphQL query in [review-threads-graphql.md](@/skills/github/gh-resolve-pr-comments/resources/manual/review-threads-graphql.md). Map each inline comment to its thread by `databaseId`, keeping the thread `id` for step 8.

Filter out only:

- Comments in threads where `isResolved` is `true`
- Non-actionable CI and deploy status-bot top-level comments (`github-actions`, `vercel`, `netlify`, `railway`, `cloudflare`) whose output duplicates check runs

Retain:

- Inline threads and actionable review findings from AI code-review bots
- Human reviewer comments and self-review comments authored by the current user (often left as action items)

When a bot finding cites repo-local rule files, read those rule files before categorizing or replying.

If zero comments remain after filtering, report "No open PR comments to address" and stop.

## 4. Assess and categorize

For each comment or thread, verify the reviewer or bot claim against the actual source code at the referenced `file:line`, surrounding callers, and diff hunk before classifying into one of these categories:

| Category        | Description                                                         | Example                                           |
| --------------- | ------------------------------------------------------------------- | ------------------------------------------------- |
| **Fix**         | Verified bug or actionable in-scope improvement                     | "This dereferences `user` before the null check"  |
| **Decline**     | False positive, wrong claim, speculative guard, or out-of-scope nit | "Add a fallback cache for an unreachable state"   |
| **Investigate** | Needs deeper codebase exploration before deciding                   | "Is this duplicated anywhere else?"               |
| **Discuss**     | Design question, trade-off, or needs user direction                 | "Should we use strategy A or B here?"             |
| **Acknowledge** | FYI or praise, no code change needed                                | "Nice refactor"                                   |
| **Outdated**    | Comment on code that has already changed                            | Diff hunk no longer matches current file contents |

- **Fix:** read the actual file and confirm the defect before proposing a change.
- **Decline:** cite concrete `file:line`, test, or runtime behavior showing why the claim does not hold or falls outside scope; record legitimate out-of-scope ideas as follow-ups rather than expanding the PR.
- **Investigate:** note what to explore, saving deep exploration for step 6.
- **Discuss:** summarize the trade-off and prepare to ask the user.
- **Outdated:** the thread's `isOutdated` is `true`, `diff_hunk` no longer matches the current file, or `position` is `null`.

## 5. Present the plan

Present the plan using the triage plan template in [triage-templates.md](@/skills/github/gh-resolve-pr-comments/resources/manual/triage-templates.md), then ask the user whether to proceed as-is, reclassify items (e.g., move a Fix to Decline or Discuss), or skip specific items. Make no changes until the user confirms.

## 6. Execute the plan

Work through the approved plan in this order:

1. **Fix** items - apply one surgical change at a time, preferring fixes that delete or narrow code over adding new helpers, options, caches, locks, or defensive wrappers. Run the repository's covering checks afterwards.
2. **Investigate** items - explore the codebase, then reclassify to **Fix**, **Decline**, or **Discuss** based on verified evidence.
3. **Decline** items - prepare concise `file:line`, test, or behavior pushback notes; agree only with feedback the evidence supports.
4. **Discuss** items - present verified context and ask the user for direction.

Show the execution summary from [triage-templates.md](@/skills/github/gh-resolve-pr-comments/resources/manual/triage-templates.md).

## 7. Commit and push

Ask the user to confirm the commit message (default `fix: address PR review feedback`, adjusted to repository conventions) and the push. After confirmation:

```bash
git add <specific changed files>
git commit -m "fix: address PR review feedback"
git push
git rev-parse --short HEAD
```

Record the short SHA for the replies. Skip this step when the plan has no code changes.

## 8. Reply and resolve threads

Reply to and resolve addressed threads (`Fix`, `Decline`, resolved `Investigate`, `Acknowledge`, and `Outdated`). Leave **Discuss** threads the user deferred open.

1. **Reply before resolving.** For every `Fix`, `Decline`, and resolved `Investigate` thread (and any `Outdated` thread where pointing to the new location helps), post a concise reply. Cite the pushed SHA in every `Fix` reply (for example "Fixed in `abc1234`"). On a human account, start each reply with the `> [!NOTE]` callout `> 🤖 **<model> on behalf of <user>**`, and scan the full outgoing payload for secrets and private details before posting. Follow `/gh-pr-comments` for the full checklist when it is installed.

   ```bash
   gh api "repos/{owner}/{repo}/pulls/{pr_number}/comments/{comment_id}/replies" \
     -f body="<formatted reply>" --jq '.html_url'
   ```

2. **Resolve the thread** with the `resolveReviewThread` mutation in [review-threads-graphql.md](@/skills/github/gh-resolve-pr-comments/resources/manual/review-threads-graphql.md), using the thread `id` from step 3, only after its reply URL is confirmed.

## 9. Report

Report the final status using the template in [triage-templates.md](@/skills/github/gh-resolve-pr-comments/resources/manual/triage-templates.md). Done when every addressed thread has a confirmed reply URL, every resolved thread returned `isResolved: true`, and the pushed SHA is reported.

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).

