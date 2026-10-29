+++
title = "gh-create-pr"
description = "Draft a problem-first Conventional Commit PR title and description with secret scanning and REST fallback. Use when asked to create or update a PR after approval."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


<!-- omit in toc -->

# Quick PR Description

**Invocation:** Start only when the user explicitly requests this workflow.

Consult the [gh pr manual](@/skills/github/gh-create-pr/resources/auto/manual-gh_pr.md) for more details.

Generate a concise PR description by analyzing the diff against a base branch.

Output the result in a markdown file named `PR_DESCRIPTION.md`.

Copy to clipboard if handy - e.g. `pbcopy < PR_DESCRIPTION.md` (macOS) or `xclip -selection clipboard < PR_DESCRIPTION.md` (Linux).

- [Instructions](#instructions)
  - [1. Determine the base branch](#1-determine-the-base-branch)
  - [2. Analyze the changes against the base branch](#2-analyze-the-changes-against-the-base-branch)
  - [3. Generate the title and description using the format below](#3-generate-the-title-and-description-using-the-format-below)
  - [4. Ask user to approve, edit, or reject](#4-ask-user-to-approve-edit-or-reject)
  - [5. On approval: commit, create/update PR](#5-on-approval-commit-create-update-pr)
- [Title Format](#title-format)
- [Output Format](#output-format)
- [Section Rules](#section-rules)
  - [tl;dr](#tldr)
  - [Summary](#summary)
  - [Feature Diff](#feature-diff)
  - [Details](#details)
  - [General Details](#general-details)
- [Example Output](#example-output)

## Instructions

### 1. Determine the base branch

**If the user passed a branch name as an argument** (e.g. `/gh-create-pr feature-branch`), use that as `BASE_BRANCH`. Skip auto-detection entirely.

**Otherwise**, auto-detect the repository's default branch. Try these methods in order until one succeeds:

**Method 1 - GitHub CLI**

```bash
BASE_BRANCH=$(gh repo view --json defaultBranchRef -q '.defaultBranchRef.name' 2>/dev/null)
```

**Method 2 - Git remote**

```bash
BASE_BRANCH=$(git remote show origin 2>/dev/null | grep "HEAD branch" | cut -d: -f2 | xargs)
```

**Method 3 - Git symbolic-ref**

```bash
BASE_BRANCH=$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')
```

**IMPORTANT:** Do NOT assume `master` or `main` as a fallback. If all methods fail, ask the user which branch to use as the base.

**Validation:** Regardless of how `BASE_BRANCH` was determined, verify it exists before proceeding:

```bash
git rev-parse --verify "$BASE_BRANCH" 2>/dev/null || git rev-parse --verify "origin/$BASE_BRANCH" 2>/dev/null
```

If the branch does not exist locally or on the remote, stop and ask the user to confirm the branch name.

### 2. Analyze the changes against the base branch

```bash
git diff $BASE_BRANCH --stat -- ":(exclude)*.lock" ":(exclude)package-lock.json" ":(exclude)pnpm-lock.yaml" ":(exclude)package.json"
git log $BASE_BRANCH..HEAD --oneline
```

### 3. Generate the title and description using the format below

Generate both a **PR title** (see [Title Format](#title-format)) and the full description body (see [Output Format](#output-format)).

Before writing or displaying `PR_DESCRIPTION.md`:

- Scan the title, `PR_DESCRIPTION.md` body, quoted snippets, command outputs, and any attachment links or files for secrets, tokens, credentials, internal hostnames or paths, and private details.
- Never add `[!NOTE]` attribution banners (such as `responding on behalf of`) or `Created with <model> in <harness>` model or harness attribution footers to `PR_DESCRIPTION.md`.

Write the description to `PR_DESCRIPTION.md` and display both the title and description to the user.

### 4. Ask user to approve, edit, or reject

Present the generated title and description to the user and ask them to:

1. **Approve** as-is
2. **Request changes** (provide feedback, re-generate)
3. **Reject** (stop here)

Do NOT proceed to step 5 until the user explicitly approves.

### 5. On approval commit create update PR

Once the user approves, re-run the secret and private-detail scan if any edits were made, then execute the following steps in order:

**Step 5a - Commit unstaged changes (if any):**

Stage only the specific files this PR should include - never blanket-stage with `git add -A`, which sweeps in unrelated changes:

```bash
git add <specific intended files>
git commit -m "<generated title>"
```

If there are no unstaged/staged changes, skip this step.

**Step 5b - Push the branch:**

```bash
git push -u origin HEAD
```

**Step 5c - Create or update the PR:**

Open a ready (non-draft) PR by default so CI workflows and review bots run immediately; pass `--draft` only when the user explicitly asks for a draft PR.

Check if a PR already exists for the current branch:

```bash
gh pr view --json number,url 2>/dev/null
```

If a PR exists, update it and verify the returned PR URL:

```bash
gh pr edit --title "<generated title>" --body-file PR_DESCRIPTION.md
```

If no PR exists, create one and verify the returned PR URL. Always pass `--base` so the PR targets the correct branch (especially important when the base is not the repo default):

```bash
gh pr create --base "$BASE_BRANCH" --title "<generated title>" --body-file PR_DESCRIPTION.md
```

**GraphQL rate-limit fallback:** `gh pr create` and `gh pr edit` use GitHub's GraphQL API. If either fails because the GraphQL rate limit is exhausted, fall back to GitHub's separate REST quota and verify the returned `html_url`:

```bash
# Create a PR via REST when GraphQL quota is exhausted
gh api "repos/$OWNER/$REPO/pulls" \
  -f title="<generated title>" \
  -f head="<head_branch>" \
  -f base="$BASE_BRANCH" \
  -F body=@PR_DESCRIPTION.md --jq '.html_url'

# Update an existing PR via REST when GraphQL quota is exhausted
gh api -X PATCH "repos/$OWNER/$REPO/pulls/$PR" \
  -f title="<generated title>" \
  -F body=@PR_DESCRIPTION.md --jq '.html_url'
```

**Post-filing handoffs:**

- If the user also asked to monitor the PR through CI checks and reviews after filing, continue with `/gh-babysit-pr`.
- Do not post a follow-up PR comment unless the user's request calls for one; format any requested comment with `/gh-pr-comments`.

## Title Format

PR titles and commit messages must use Conventional Commits:

```text
type(scope): description
```

**Rules:**

- Use a lowercase type: `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `style`, or `revert`
- The scope is optional and names the affected area, such as `skills` or `ci`; omit its parentheses when there is no scope
- For breaking changes, put `!` immediately before the colon, for example `feat(api)!: Require authentication`
- Follow the colon with a space and a concise imperative description that explains why the change matters or the user/operator outcome achieved, rather than low-level symbol or protocol mechanics (prefer `perf(server): Cut websocket frame size by 70%+ with gzipping` over `perf(server): Negotiate permessage-deflate on the websocket`)
- Max 70 characters total
- No period at the end

**Examples:**

- `feat: Add session-based authentication`
- `fix(queue): Resolve race condition in queue worker`
- `perf(server): Cut websocket frame size by 70%+ with gzipping`
- `docs(api): Update API reference for v2 endpoints`

## Output Format

```markdown
_tl;dr Single sentence, 120 characters max, summarizing the most important outcome of this PR._

## Summary

- **Subject/topic**: < 100 character explanation
- ...
- ...

## Feature Diff

| S       | Component                          | Before                                     | After                                    |
| ------- | ---------------------------------- | ------------------------------------------ | ---------------------------------------- |
| 🟢/🔴/… | 1-3 words describing the component | 1 sentence describing how it worked before | 1 sentence describing how it works after |
| …       | …                                  | …                                          | …                                        |

> _(legend blockquote - see the Feature Diff section rules for the exact line)_

## Details

<details>
<summary>Technical Details</summary>

### Subsection Title

- **Subject/topic**: < 100 character explanation
- ...

### Another Subsection

- **Subject/topic**: < 100 character explanation
- ...

</details>
```

## GitHub Admonitions

Use [GitHub admonitions](@/skills/github/gh-create-pr/resources/auto/en-get-started-writing-on-github-getting-started-with-writing-and-formatting-on-github-basic-writing-and-formatting-syntax.md#alerts) at the **very top** of the description (before the tl;dr) when the PR has important context that reviewers need upfront. Do NOT use admonitions by default - only when one of the situations below applies.

**Syntax:**

```markdown
> [!NOTE]
> Useful information that users should know, even when skimming content.

> [!TIP]
> Helpful advice for doing things better or more easily.

> [!IMPORTANT]
> Key information users need to know to achieve their goal.

> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.

> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.
```

**When to use each type:**

| Type        | When to use                                                                                            |
| ----------- | ------------------------------------------------------------------------------------------------------ |
| `NOTE`      | PR is a follow-up/review of another PR, replaces a previous approach, or has non-obvious scope context |
| `TIP`       | PR unlocks a workflow or has a recommended migration/adoption path reviewers should know               |
| `IMPORTANT` | PR requires a specific merge order, has deployment prerequisites, or needs coordinated rollout         |
| `WARNING`   | PR includes a breaking change, requires a migration, or has a tight deadline                           |
| `CAUTION`   | PR touches sensitive systems (auth, billing, data deletion) or has irreversible side effects           |

**Rules:**

- Maximum ONE admonition per PR description (pick the most important)
- Keep it to 1-3 sentences - enough context to orient the reviewer, not a full explanation
- Reference related PRs/issues by number (e.g., #509) so GitHub auto-links them
- Place BEFORE the tl;dr line

## Section Rules

### tldr

- Single sentence, **120 characters max** (hard ceiling)
- Lead with the user, operator, or developer problem solved and the resulting outcome
- No symbol inventories, deleted helper lists, or file names

### Summary

- 2-5 bullets, one per meaningful change (not per file)
- Min (2) and max (5) are hard floors and ceilings per section
- Bold phrase answers "what problem was solved or what does the user/operator get?"
- Plain language after the dash: state the problem before the solution in one sentence without jargon
- Never lead with an implementation inventory (lists of deleted helpers, renamed functions, or touched files belong inside collapsible `<details>` `Technical Details`)
- No fluff: skip "minor cleanup", "refactor", "update docs" unless they deliver real value
- **Order by priority/impact, highest first** - the first bullet should be the most important change in the PR
- Use backticks for code references: file names, paths, commands, config keys, env vars, endpoints, function names

### Feature Diff

- **Always include this section**
- Should have anywhere from 1-10 rows depending on the size of the PR
- One row per component, module, config, API, or behavior that changed
- "Component" = the thing that changed (endpoint, table, config key, module, behavior, etc.)
- "Before" = previous state, or `N/A` if new
- "After" = new state, or `Removed` if deleted
- Keep cells concise - short phrases, not sentences
- Group related rows; aim for 3-10 rows
- Good component examples: API endpoint, DB table/column, config key, env var, dependency version, CLI flag, permission, error behavior
- Use backticks for code references in Component, Before, and After cells (e.g., `sessions` table, `/auth/login`, `TOKEN_TTL`)
- **Legend**: Every Feature Diff table is followed by this exact one-line legend as a blockquote. This is the single definition of the severity symbols - the Output Format and Example Output templates above just reference it:

  > 🔴 Critical fix · 🟡 Improvement · 🟢 New feature · ⚪ Neutral · ⚙️ Infra/tooling · ⚠️ Breaking

- **Severity column (S)**: Every row must carry one of these severity emoji as its first column:

| Emoji | Label         | When to use                           |
| ----- | ------------- | ------------------------------------- |
| 🔴    | Critical fix  | Bug fix for broken/incorrect behavior |
| 🟡    | Improvement   | Enhancement to existing behavior      |
| 🟢    | New feature   | Net-new capability                    |
| ⚪    | Neutral       | Config, docs, chore, cleanup          |
| ⚙️    | Infra/tooling | CI, build, dev tooling changes        |
| ⚠️    | Breaking      | Breaking change or deprecation        |

### Details

- **Only include for larger PRs** (5+ files changed or multiple logical groups)
- Use collapsible `<details>` tags
- Group by feature/concern, not by file
- This is where implementation specifics go (file names, function names, migration details)
- Use backticks for code references (`file.py`, `get_user()`, `/api/v1/users`)
- 1-3 subsections, each with 3-5 bullets

### General Details

- **Use backticks everywhere for code references** - this applies to ALL sections (tl;dr excluded): file names (`file.py`), file paths (`src/auth/`), commands (`npm run build`), config keys (`TOKEN_TTL`), env vars (`NODE_ENV`), endpoints (`/api/v1/users`), function names (`getUser()`), table/column names (`sessions.token`)
- Italicize or bold keywords if it helps readability

## Example Output

```markdown
_tl;dr Users can now log in with email/password and stay authenticated across browser sessions._

## Summary

- **Session-based login**: Users authenticate with email/password and maintain sessions across browser restarts
- **Faster auth checks**: Session lookups use an indexed `token` column instead of scanning the full `users` table
- **Remember-me support**: Users can opt into 30-day sessions instead of the default 24-hour expiry

## Feature Diff

| S   | Component        | Before                     | After                                           |
| --- | ---------------- | -------------------------- | ----------------------------------------------- |
| 🟢  | Auth method      | API key only               | Email/password + session cookie                 |
| 🟢  | Session duration | `N/A`                      | 24 hours (default), 30 days (remember-me)       |
| 🟢  | `sessions` table | `N/A`                      | New table with `user_id`, `token`, `expires_at` |
| 🟡  | Token lookup     | Full table scan on `users` | Indexed lookup on `sessions.token`              |
| 🟢  | `/auth/login`    | `N/A`                      | New endpoint                                    |
| 🟢  | `/auth/logout`   | `N/A`                      | New endpoint                                    |

> _(legend blockquote - see the Feature Diff section rules for the exact line)_

## Details

<details>
<summary>Technical Details</summary>

### Authentication Service

- **New login service**: Handles JWT issuance, session creation, and cookie management
- Add `login_service.py` with session create/validate/revoke methods
- Integrate `/auth/login` and `/auth/logout` endpoints in `routes/auth.py`
- Support `remember_me` flag to toggle 24h vs 30d expiry

### Database Schema

- **New sessions table**: Stores active sessions with automatic expiry
- Add `sessions` table with `user_id`, `token`, `expires_at` columns
- Add B-tree index on `token` for O(1) lookups
- Add index on `expires_at` for cleanup job performance

</details>
```

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).

