+++
title = "pr-format"
[extra]
skill = false
category = "github"
mermaid = false
skill_name = "gh-create-pr"
+++

# PR title and description format

Reference for the title and body that `gh-create-pr` drafts. Follow every rule here when writing the draft.

## Title format

PR titles and commit messages use Conventional Commits:

```text
type(scope): description
```

- Use a lowercase type: `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `style`, or `revert`.
- The scope is optional and names the affected area, such as `skills` or `ci`; omit its parentheses when there is no scope.
- For breaking changes, put `!` immediately before the colon, for example `feat(api)!: Require authentication`.
- Follow the colon with a space and a concise imperative description of why the change matters or the user/operator outcome, rather than low-level symbol or protocol mechanics (prefer `perf(server): Cut websocket frame size by 70%+ with gzipping` over `perf(server): Negotiate permessage-deflate on the websocket`).
- Keep the title to 70 characters or fewer, with no trailing period.

Examples:

- `feat: Add session-based authentication`
- `fix(queue): Resolve race condition in queue worker`
- `perf(server): Cut websocket frame size by 70%+ with gzipping`
- `docs(api): Update API reference for v2 endpoints`

## Description template

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

> 🔴 Critical fix · 🟡 Improvement · 🟢 New feature · ⚪ Neutral · ⚙️ Infra/tooling · ⚠️ Breaking

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

## GitHub admonitions

Add one GitHub admonition at the very top of the description (before the tl;dr) only when the PR has context reviewers need upfront. GitHub's syntax reference is in `resources/auto/en-get-started-writing-on-github-getting-started-with-writing-and-formatting-on-github-basic-writing-and-formatting-syntax.md`.

```markdown
> [!IMPORTANT]
> Merge after #509; this PR depends on its schema migration.
```

| Type        | When to use                                                                                            |
| ----------- | ------------------------------------------------------------------------------------------------------ |
| `NOTE`      | PR is a follow-up/review of another PR, replaces a previous approach, or has non-obvious scope context |
| `TIP`       | PR unlocks a workflow or has a recommended migration/adoption path reviewers should know               |
| `IMPORTANT` | PR requires a specific merge order, has deployment prerequisites, or needs coordinated rollout         |
| `WARNING`   | PR includes a breaking change, requires a migration, or has a tight deadline                           |
| `CAUTION`   | PR touches sensitive systems (auth, billing, data deletion) or has irreversible side effects           |

- Use at most one admonition per description; pick the most important.
- Keep it to 1-3 sentences that orient the reviewer.
- Reference related PRs and issues by number (for example `#509`) so GitHub auto-links them.

## Section rules

### tl;dr

- Single sentence, 120 characters max (hard ceiling).
- Lead with the user, operator, or developer problem solved and the resulting outcome.
- Keep symbol inventories, deleted helper lists, and file names out of it.

### Summary

- 2-5 bullets (hard floor and ceiling), one per meaningful change, not per file.
- The bold phrase answers "what problem was solved, or what does the user or operator get?"
- After the colon, state the problem before the solution in one plain-language sentence.
- Put implementation inventories (deleted helpers, renamed functions, touched files) inside the collapsible `Technical Details` block.
- Include cleanup, refactor, or docs bullets only when they deliver real value.
- Order by impact, highest first.

### Feature Diff

- Always include this section, with 1-10 rows depending on the size of the PR.
- One row per component, module, config, API, or behavior that changed; group related rows.
- "Component" is the thing that changed: API endpoint, DB table or column, config key, env var, dependency version, CLI flag, permission, or error behavior.
- "Before" is the previous state, or `N/A` if new. "After" is the new state, or `Removed` if deleted.
- Keep cells to short phrases.
- Start every row with one severity emoji in the `S` column, chosen from the table below, and follow the table with the one-line legend blockquote exactly as shown in the template.

| Emoji | Label         | When to use                           |
| ----- | ------------- | ------------------------------------- |
| 🔴    | Critical fix  | Bug fix for broken/incorrect behavior |
| 🟡    | Improvement   | Enhancement to existing behavior      |
| 🟢    | New feature   | Net-new capability                    |
| ⚪    | Neutral       | Config, docs, chore, cleanup          |
| ⚙️    | Infra/tooling | CI, build, dev tooling changes        |
| ⚠️    | Breaking      | Breaking change or deprecation        |

### Details

- Include only for larger PRs (5+ files changed or multiple logical groups).
- Wrap it in a collapsible `<details>` block.
- Group by feature or concern, not by file.
- Put implementation specifics here: file names, function names, migration details.
- Use 1-3 subsections, each with 3-5 bullets.

### All sections

- Wrap every code reference in backticks (tl;dr excluded): file names (`file.py`), paths (`src/auth/`), commands (`npm run build`), config keys (`TOKEN_TTL`), env vars (`NODE_ENV`), endpoints (`/api/v1/users`), function names (`getUser()`), and table or column names (`sessions.token`).
- Italicize or bold keywords when it helps readability.

## Example output

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

> 🔴 Critical fix · 🟡 Improvement · 🟢 New feature · ⚪ Neutral · ⚙️ Infra/tooling · ⚠️ Breaking

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

