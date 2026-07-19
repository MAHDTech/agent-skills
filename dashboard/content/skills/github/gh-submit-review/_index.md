+++
title = "gh-submit-review"
description = "Post a completed code review to a GitHub PR via the gh CLI — pick the event (approve, request-changes, or comment), attach inline line comments, and a summary body. Use when you have finished reviewing someone else's PR and need to submit the verdict, when a review pass (e.g. /code-review or /pr-edge-cases) produced findings to publish, or when the user asks to approve, request changes on, or leave review comments on a pull request."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Submit a PR Review

Consult the [gh pr manual](@/skills/github/gh-submit-review/resources/manual-gh_pr.md) for more details.

Close the loop after reviewing someone else's pull request: turn your findings into a formal GitHub review with the right event, a summary body, and inline comments anchored to the lines they concern.

For the other side — resolving comments left on _your own_ PR — use `/gh-resolve-pr-comments`.

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
| Request changes | `--request-changes` | At least one blocking issue — a bug, security hole, spec miss, data-loss risk, or convention violation — must change before merge.            |
| Comment         | `--comment`         | Feedback worth recording but nothing blocking, open questions, or you lack the ownership/context to gate the merge on an area you do not own. |

Etiquette: one blocking finding is enough to request changes — do not soften it to a comment. Conversely, do not request changes over pure style preferences; leave those as comment-level nits and approve.

## 3. Draft the body and inline comments

- **Summary body**: lead with the verdict and its reason. For request-changes, list the blocking items up front so the author sees them without scrolling.
- **Inline comments**: anchor each specific finding to its file and line rather than describing locations in prose. Use `RIGHT` for the new version of the diff, `LEFT` for the old. The line must appear in the PR diff or the API rejects the comment.

## 4. Submit

**Summary only** (no inline comments) — use `gh pr review` directly:

```bash
gh pr review <pr> --request-changes --body "Two blocking issues inline; see comments."
```

Swap in `--approve` or `--comment` as chosen. Use `--body-file` for a long body.

**With inline comments** — build a single JSON payload and post it to the reviews endpoint so the event, body, and all inline comments land as one review:

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
gh api repos/{owner}/{repo}/pulls/<pr>/reviews --input review.json
```

`event` must be `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. Omit `event` to leave the review `PENDING` (a draft only you can see) when you want to eyeball it on GitHub before publishing. Use `start_line` with `line` for a multi-line comment.

## 5. Confirm

Report the submitted event, the summary, and the count of inline comments, plus the PR URL. Done when the review appears on the PR with its event and every intended inline comment attached.

