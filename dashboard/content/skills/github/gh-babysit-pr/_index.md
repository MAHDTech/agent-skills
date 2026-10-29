+++
title = "gh-babysit-pr"
description = "Monitor an open pull request through CI checks, review bots, and base-branch updates with pr-wait. Use only when the user explicitly asks to watch, monitor, or babysit a PR."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# GitHub Babysit PR

**Invocation:** Start only when the user explicitly requests this workflow.

Monitor an open pull request through CI checks, AI review bots, human reviews, and base-branch updates until the PR is ready or requires a user decision.

## 1. Run one watcher per PR

Address existing failed checks and open review comments first. Then run the bundled `resources/manual/pr-wait` script from this skill's directory rather than writing custom `sleep` loops or ad-hoc polling scripts:

```bash
resources/manual/pr-wait <pr> [--repo owner/name] [--until ready|green|any-change] [--require-bot <login>]... [--interval 20]
```

- **Single watcher rule:** Run at most one `resources/manual/pr-wait` process per PR. Every watcher on a GitHub account shares one rate limit, and `resources/manual/pr-wait` holds a per-PR `flock` lock in `$XDG_STATE_HOME/pr-wait` (defaulting to `~/.local/state/pr-wait/`). Only the parent workflow that owns the PR watches it; delegated review or implementation subagents must never start a second watcher.
- **Execution mode:** Run `resources/manual/pr-wait` through your agent runtime's background command monitor so stdout event lines wake you on changes, or run it as a single foreground command that exits on its own.
- **Stop conditions (`--until`):**
  - `ready` (default): required checks green (or all non-advisory checks when none are marked required), every `--require-bot <login>` review bot finished on the current head without failing checks, zero unresolved review threads, and clean merge state.
  - `green`: required checks green on the current head, ignoring review threads and `BEHIND` state.
  - `any-change`: exit `0` after the first poll that prints a new event.
- **Optional bot gates (`--require-bot`):** Pass `--require-bot greptile`, `--require-bot macroscope`, or any custom bot login when the repository or user requires specific review bots to finish on the head commit before declaring readiness.
- **Exit codes:**
  - `0`: `--until` is satisfied or the PR merged.
  - `1`: action is required (failed required or `--require-bot` check, `DIRTY` merge conflicts, `BEHIND` merge state on `ready`, unresolved review threads on `ready`, PR closed without merge, or persistent GitHub unreachability).
  - `2`: invalid arguments, unreadable PR, or another PID already holds the PR lock (kill the stale PID only when its events no longer reach you).
- **Restarting and silence:** Start `resources/manual/pr-wait` again after pushing fixes, after handling an exit `1`, or after resuming a session; its 6-hour state cache prevents replaying already-reported events while surfacing events that arrived between runs. When an event requires no action, end your turn quietly without posting or printing `"still waiting"` messages.

## 2. Handle GitHub GraphQL rate limits

`gh pr create`, `gh pr view`, `gh pr checks`, and `resources/manual/pr-wait` consume GitHub's GraphQL quota. When a watch stops because a PR could not be read or GraphQL fails on quota, inspect the remaining GraphQL budget before restarting:

```bash
gh api rate_limit --jq '.resources.graphql'
```

- If `remaining` is near `0`, wait once until the `reset` epoch timestamp before starting `resources/manual/pr-wait` again; never retry in a tight loop.
- While GraphQL is exhausted, inspect PR metadata and checks via GitHub's separate REST quota:

```bash
gh api "repos/$OWNER/$REPO/pulls/$PR"
gh api "repos/$OWNER/$REPO/commits/$SHA/check-runs"
```

## 3. Triage CI and review-bot findings without diff bloat

- **Latest head only:** Act only on check runs, reviews, and threads that apply to the latest pushed commit.
- **Verify against source:** Read the referenced code and tests before changing anything. Separate repository defects from transient CI or infrastructure flakes.
- **Gating vs advisory bots:** Wait on review bots passed via `--require-bot`. Treat CodeRabbit as advisory unless explicitly required via `--require-bot coderabbit`: evaluate useful catches, never hold readiness or a merge waiting for it. When replying to Macroscope findings in a repo that carries `.macroscope/check-run-agents/*.md`, read those local rule files first because Macroscope findings cite them.
- **Diff-bloat guard:** Review loops often inflate a PR one defensive tweak at a time. Check `git diff --shortstat` against the base branch after each fix round:
  - Prefer fixes that delete or narrow code over adding new helpers, configuration flags, caches, locks, or speculative wrappers.
  - Fix genuine bugs within the user's intended scope.
  - Decline false positives, speculative edge-case guards, unneeded extra compatibility layers, or out-of-scope nits with a concise `file:line`, test, or behavior explanation, and record valid out-of-scope ideas as follow-ups rather than expanding the PR.
- **Attributed replies before resolution:** Format every comment posted on the user's behalf with `/gh-pr-comments` (including its secret preflight scan and human-account `[!NOTE]` attribution). When multiple open threads need structured triage, use `/gh-resolve-pr-comments`. Resolve a review thread only after GitHub returns the confirmed URL for your reply.

## 4. Reconcile base-branch movement and readiness

`resources/manual/pr-wait` emits events when new base-branch commits touch files modified by this PR and when `mergeStateStatus` becomes `BEHIND` or `DIRTY`:

1. When the branch is `BEHIND`, `DIRTY`, or overlaps newly landed base commits, run `/git-update-branch` to evaluate conceptual conflicts against the updated base and rebase or merge cleanly.
2. If an overlapping commit or PR on the base branch makes this PR obsolete, stop monitoring, report the concrete base-branch evidence to the user, and ask before closing the PR unless closure was explicitly authorized.
3. Stop watching once `resources/manual/pr-wait` exits `0` on the latest head. Report readiness to the user unless merging was explicitly requested.
4. Immediately before any user-authorized merge, run `git fetch origin` and re-verify PR status and checks. Never merge while `mergeStateStatus` is `BEHIND` or `DIRTY`, or when base commits landed after the last green run on files this PR touches; update the branch, push, and wait for green checks again first.

Done when `resources/manual/pr-wait` exits `0` on the current head with required checks green, zero unresolved threads, and clean base-branch alignment (or when a concrete blocker or superseding base change is reported to the user).

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).

