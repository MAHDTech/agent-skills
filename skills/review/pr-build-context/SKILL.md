---
name: pr-build-context
description: "Prepare a briefing before reviewing a pull request: explain the diff, key files, risks, and questions for the author. Use for PR context, or a whole-repository overview when on its default branch."
metadata:
  group: review
---

# Build PR Context

Prepare high-signal context for a pull request before a human pair review.

## What to do

1. Check whether the user supplied an explicit PR number or URL first.
   - If a PR number or URL was given, stay in **PR mode** regardless of the local branch and inspect it directly via `gh pr view <pr> --json number,title,body,baseRefName,headRefName,files,commits` and `gh pr diff <pr>`.
   - Otherwise, identify the repo's default branch (do not guess):
     - Prefer GitHub CLI: `gh repo view --json defaultBranchRef --jq '.defaultBranchRef.name'`
     - Fallback: `git remote show origin | sed -n '/HEAD branch/s/.*: //p'`
     - If still unclear, say so and ask the developer.

2. Check the current branch when no explicit PR target was given.
   - Run: `git branch --show-current`
   - If current branch == default branch and no PR number/URL was supplied, switch to **repo context mode** (see below).
   - Otherwise, continue with PR diff analysis.

3. Diff against the fetched base ref (PR mode on a local branch).
   - Fetch the default branch first (`git fetch origin "$DEFAULT_BRANCH"`) and resolve `BASE_REF` preferring `origin/$DEFAULT_BRANCH` over local `$DEFAULT_BRANCH`.
   - Use triple-dot diff, excluding lockfiles only, so dependency manifest changes stay visible: `git diff "$BASE_REF"...HEAD -- ":(exclude)*.lock" ":(exclude)package-lock.json" ":(exclude)pnpm-lock.yaml"`
   - Also capture: `git diff --stat "$BASE_REF"...HEAD -- ":(exclude)*.lock" ":(exclude)package-lock.json" ":(exclude)pnpm-lock.yaml"`

4. Understand the changes.
   - What behavior changed?
   - Why was it changed?
   - What assumptions or invariants does this rely on?
   - What could break (correctness, security, perf, API, data, ops)?

5. Prepare for pair review.
   - Summarize the change in 3-5 plain-English bullet points.
   - Call out key files and why they matter.
   - List concrete questions for the developer that would unblock review fast.

6. Call out big issues explicitly.
   - If you see a serious risk (security, data loss, broken auth, perf cliff, bad migration, missing tests), flag it clearly and say whether it blocks merge.

## Repo context mode (when on default branch)

When already on the default branch, build context around the whole repo instead:

1. **Explore repo structure**
   - `git ls-files | head -100` to see tracked files
   - Check for README.md, CLAUDE.md, AGENTS.md for project docs
   - Identify key directories and their purpose

2. **Understand the tech stack**
   - Look at package.json, pyproject.toml, Cargo.toml, go.mod, etc.
   - Note languages, frameworks, and dependencies

3. **Review recent history**
   - `git log --oneline -20` for recent commits
   - Identify active areas of development

4. **Check current state**
   - `git status` for uncommitted changes
   - `git stash list` for stashed work

5. **Summarize for the developer**
   - What does this repo do?
   - What's the project structure?
   - What's the current state (clean, WIP, staged changes)?
   - What are the key entry points?

### Repo context output format

- **Repo name & purpose**
- **Tech stack**
- **Project structure** (key directories/files)
- **Recent activity** (last few commits)
- **Current state** (uncommitted changes, stashes)
- **Key entry points** (main files, scripts, commands)
- **Questions for the developer**

## PR context output format

- **Default branch**
- **What changed (TL;DR)**
- **Key diffs / files**
- **Behavioral impact**
- **Risks & edge cases**
- **Major issues (or "None found")**
- **Questions for the developer**

For deep bug, boundary-case, or race-condition hunting beyond this briefing, follow up with `/pr-edge-cases`.

Done when every section of the selected output format is grounded in inspected diff, file, or command output, major risks and open questions are listed, and execution pauses for developer input.
