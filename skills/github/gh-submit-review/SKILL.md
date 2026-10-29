---
name: gh-submit-review
description: Publish a GitHub PR review with verdict, inline findings, secret scanning, and human-account attribution. Use when asked to submit a completed PR review.
disable-model-invocation: true
resources:
  - https://cli.github.com/manual/gh_pr
metadata:
  group: github
  source: t3dotgg/tstack
  license: MIT
---

# Submit a PR Review

**Invocation:** Start only when the user explicitly requests this workflow.

Consult the [gh pr manual](resources/auto/manual-gh_pr.md) for more details.

Close the loop after reviewing someone else's pull request: turn your findings into a formal GitHub review with the right event, a summary body, and inline comments anchored to the lines they concern.

For the other side - resolving comments left on _your own_ PR - use `/gh-resolve-pr-comments`.

## 1. Identify the PR and repo

```bash
gh pr view <pr> --json number,url,headRefName,baseRefName,author
gh repo view --json nameWithOwner -q '.nameWithOwner'
```

Pass an explicit PR number or URL when reviewing a branch you have not checked out. Parse `nameWithOwner` into `{owner}` and `{repo}` for the API calls below.

## 2. Choose the event

Every review carries exactly one event. Pick it by the strongest finding, not the average:

| Event           | Flag                | When                                                                                                                                          |
| --------------- | ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Approve         | `--approve`         | The change is correct and meets the bar. Only nits remain, and you are happy to leave them to the author's discretion.                        |
| Request changes | `--request-changes` | At least one blocking issue - a bug, security hole, spec miss, data-loss risk, or convention violation - must change before merge.            |
| Comment         | `--comment`         | Feedback worth recording but nothing blocking, open questions, or you lack the ownership/context to gate the merge on an area you do not own. |

Etiquette: one blocking finding is enough to request changes - do not soften it to a comment. Conversely, do not request changes over pure style preferences; leave those as comment-level nits and approve.

## 3. Draft the body and inline comments

Apply `/gh-pr-comments` formatting and preflight checks across the review summary and every inline comment:

- **Account-aware attribution:** Check `gh api user -q '.login, .type'`. When posting from a human user's login (`type == "User"` and not a dedicated automation account), prefix the summary body and each inline comment with the `[!NOTE]` `🤖 **<model> responding on behalf of <user>**` callout (skipping `responding on behalf of <user>` when authenticated as a dedicated bot or automation account).
- **Summary body:** Lead with the verdict and its reason. For request-changes, list the blocking items up front with concrete `file:line`, behavior, or check citations so the author sees them without scrolling.
- **Inline comments:** Anchor each specific finding to its file and line rather than describing locations in prose. Use `RIGHT` for the new version of the diff, `LEFT` for the old. The line must appear in the PR diff or the API rejects the comment.
- **Secret and privacy preflight scan:** Before submitting, scan the summary body, every inline comment, and all quoted code or command output for secrets, tokens, internal hostnames or paths, and private details.

## 4. Submit

**Summary only** (no inline comments) - use `gh pr review` directly and capture the printed review URL:

```bash
gh pr review <pr> --request-changes --body-file review-body.md
```

Swap in `--approve` or `--comment` as chosen.

**With inline comments** - build a single JSON payload and post it to the reviews endpoint so the event, body, and all inline comments land as one review, extracting `html_url` to confirm publication:

```json
{
  "event": "REQUEST_CHANGES",
  "body": "Summary of the review verdict and reasoning.",
  "comments": [
    {
      "path": "src/auth.ts",
      "line": 42,
      "side": "RIGHT",
      "body": "This dereferences `user` before the null check on line 39."
    },
    {
      "path": "src/auth.ts",
      "start_line": 60,
      "line": 64,
      "side": "RIGHT",
      "body": "This whole block duplicates `validateSession`."
    }
  ]
}
```

```bash
gh api "repos/{owner}/{repo}/pulls/<pr>/reviews" --input review.json --jq '.html_url'
```

`event` must be `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. Omit `event` to leave the review `PENDING` (a draft only you can see) when you want to eyeball it on GitHub before publishing. Use `start_line` with `line` for a multi-line comment.

## 5. Confirm

Verify that GitHub returned the published review URL (`html_url` from the REST response or the URL printed by `gh pr review`). Report the submitted event, the verified review URL, the summary, the count of inline comments, and the PR URL. Done when the published review URL is confirmed and every intended inline comment is attached.

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).
