+++
title = "gh-pr-comments"
description = "Format and preflight-scan GitHub comments posted on a user's behalf with account-aware attribution. Use when posting PR or issue comments, inline review notes, or thread replies."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# GitHub PR Comments

Format, scan, and verify every agent-authored GitHub comment before posting on a user's behalf.

## 1. Check active GitHub account

Inspect the authenticated GitHub identity before drafting the comment:

```bash
gh api user -q '.login, .type'
```

- **Human user account (`type == "User"` and not a dedicated automation login):** Prefix the comment with this GitHub `[!NOTE]` callout and place the actual comment outside the blockquote:

  ```markdown
  > [!NOTE]
  > 🤖 **<model> on behalf of <user>**

  Actual comment starts here.
  ```

  Use the verified public model name exposed by your runtime, and the user's name or GitHub login. When the runtime does not expose a model name, use the harness name instead; never guess a model identifier. Start the attribution line with the robot emoji inside the note, and keep the callout to that single line: no horizontal rules, Setext underlines, extra headings, or second attribution footers.

  To name the workflow that produced the comment, append an optional context suffix in parentheses inside the bold text, for example `> 🤖 **<model> on behalf of <user> (triage)**`. Keep the suffix to one or two words.

- **Dedicated bot or automation account (`type == "Bot"` or a dedicated agent GitHub login):** Skip the callout because the account identity already identifies automation, and post the direct comment body.

## 2. Respect scope boundaries

Apply this format to:

- Top-level PR comments and issue-thread PR replies
- Inline pull request review comments
- Replies to AI review bots or human reviewers
- Written explanations posted before resolving or declining review feedback
- Triage comments on issues and PRs (agent briefs, triage notes, close explanations)

Leave the callout off PR descriptions, issue bodies, release notes, commit messages, and text the user explicitly asked to post verbatim without attribution.

## 3. Scan for secrets and private details

Before posting, inspect the full outgoing payload - including the comment body, quoted code or logs, command output, and attachment URLs - for:

- API keys, tokens, credentials, cookies, or session headers
- Internal hostnames, private URLs, or non-public endpoints
- Machine-local filesystem paths or private personal details

Stop and redact or ask the user before posting if any sensitive value is present.

## 4. Write directly, preview, and verify the URL

Write the comment body as an operational note:

- Lead with the conclusion or action taken.
- Cite concrete `file:line` references, observed behavior, check runs, or reproduction steps.
- Cut filler, canned thanks, and status narration.
- Separate verified facts from judgment calls.

Preview the final Markdown before sending so blockquote markers and blank lines separate the `[!NOTE]` callout from the comment body. After posting with `gh`, confirm that GitHub returned the published comment or review URL. When closing out a review thread (see `/gh-resolve-pr-comments`), resolve the thread only after its reply URL is confirmed.

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).

