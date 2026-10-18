---
description: How Git&#39;s concepts and commands map onto Oak, and what&#39;s deliberately different.
title: Coming from Git · Docs
image: https://oak.space/social-card-217ab603.png
---

Get started

- [Introduction](https://oak.space/docs)
- [Install](https://oak.space/docs/install)
- [Quickstart](https://oak.space/docs/quickstart)
- [Coming from Git](https://oak.space/docs/coming-from-git)

Everyday work

- [Making changes](https://oak.space/docs/making-changes)
- [Branches](https://oak.space/docs/branches)
- [Merging & conflicts](https://oak.space/docs/merging)
- [Reviewing](https://oak.space/docs/reviewing)
- [History](https://oak.space/docs/history)

Agents

- [Working with agents](https://oak.space/docs/agents)
- [Example session](https://oak.space/docs/example-session)

Large repositories

- [Lazy mounts](https://oak.space/docs/mounts)
- [Agent spaces](https://oak.space/docs/spaces)
- [Sparse clones](https://oak.space/docs/sparse-clones)

CI

- [Overview](https://oak.space/docs/ci)
- [Workflow syntax](https://oak.space/docs/ci-workflows)
- [Runners](https://oak.space/docs/ci-runners)
- [Environments](https://oak.space/docs/ci-environments)

Organizations and access

- [Organizations](https://oak.space/docs/organizations)
- [Path permissions](https://oak.space/docs/path-permissions)
- [API keys](https://oak.space/docs/api-keys)
- [Plans & limits](https://oak.space/docs/limits)

Integrations

- [Git import & export](https://oak.space/docs/git)
- [Webhooks](https://oak.space/docs/webhooks)
- [Sites](https://oak.space/docs/sites)

Reference

- [CLI reference](https://oak.space/docs/cli)
- [Files & env vars](https://oak.space/docs/configuration)
- [HTTP API](https://oak.space/docs/api)
- [How Oak works](https://oak.space/docs/internals)

For agents

- [All docs as one file](https://oak.space/docs/llms-full.txt)

\<details\>

\<summary\>Docs menu ▾</summary>



Get started

- <a href="https://oak.space/docs">Introduction</a>
- <a href="https://oak.space/docs/install">Install</a>
- <a href="https://oak.space/docs/quickstart">Quickstart</a>
- <a href="https://oak.space/docs/coming-from-git">Coming from Git</a>

Everyday work

- <a href="https://oak.space/docs/making-changes">Making changes</a>
- <a href="https://oak.space/docs/branches">Branches</a>
- <a href="https://oak.space/docs/merging">Merging & conflicts</a>
- <a href="https://oak.space/docs/reviewing">Reviewing</a>
- <a href="https://oak.space/docs/history">History</a>

Agents

- <a href="https://oak.space/docs/agents">Working with agents</a>
- <a href="https://oak.space/docs/example-session">Example session</a>

Large repositories

- <a href="https://oak.space/docs/mounts">Lazy mounts</a>
- <a href="https://oak.space/docs/spaces">Agent spaces</a>
- <a href="https://oak.space/docs/sparse-clones">Sparse clones</a>

CI

- <a href="https://oak.space/docs/ci">Overview</a>
- <a href="https://oak.space/docs/ci-workflows">Workflow syntax</a>
- <a href="https://oak.space/docs/ci-runners">Runners</a>
- <a href="https://oak.space/docs/ci-environments">Environments</a>

Organizations and access

- <a href="https://oak.space/docs/organizations">Organizations</a>
- <a href="https://oak.space/docs/path-permissions">Path permissions</a>
- <a href="https://oak.space/docs/api-keys">API keys</a>
- <a href="https://oak.space/docs/limits">Plans & limits</a>

Integrations

- <a href="https://oak.space/docs/git">Git import & export</a>
- <a href="https://oak.space/docs/webhooks">Webhooks</a>
- <a href="https://oak.space/docs/sites">Sites</a>

Reference

- <a href="https://oak.space/docs/cli">CLI reference</a>
- <a href="https://oak.space/docs/configuration">Files & env vars</a>
- <a href="https://oak.space/docs/api">HTTP API</a>
- <a href="https://oak.space/docs/internals">How Oak works</a>

</details>

# Coming from Git

Copy page [.md](https://oak.space/docs/coming-from-git.md "This page as raw Markdown")

How Git's concepts and commands map onto Oak, and what's deliberately different.

Oak will feel familiar — repos, branches, push, pull, diff — but a few of Git's defaults are deliberately turned around. This page is the translation table.

## What's different, in short \#

- **No staging area.** `oak commit` snapshots the whole working tree (or just the paths you name). There's no `add`.
- **No commit messages.** Commits are checkpoints. The *branch* carries the message, set with `oak desc`, and it becomes the message of the squash commit on `main`.
- **No local `main`.** `main` lives on the server. Locally you're always on a feature branch; `oak switch main` is an error, not a checkout.
- **One kind of merge.** Merging a branch onto `main` always squashes it to a single commit. There's no merge-commit or rebase option to choose between, and no pull-request object — the branch *is* the review unit.
- **Pull merges, never rebases.** `oak pull` merges the latest `main` into your branch. History on your branch is never rewritten under you.
- **Clones are optional.** `oak mount` gives you a working tree backed by the server, with files fetched on first read.
- **Large files are normal files.** Content is chunked and deduplicated (FastCDC + BLAKE3), so there's no LFS to set up.

## Command mapping \#

| Git | Oak | Notes |
| --- | --- | --- |
| `git init` | `oak init` | Also offers to import an existing `.git` history. |
| `git clone \<url\>` | `oak clone \<org\>/\<repo\>` | A Git URL works too — it's imported. |
| `git clone --depth 1` | `oak clone --shallow` | |
| `git sparse-checkout` | `oak clone --path \<dir\>`, `oak sparse` | See [Sparse clones](https://oak.space/docs/sparse-clones). |
| `git worktree add` | `oak mount \<org\>/\<repo\> \<dir\>` | No clone behind each one. See [Lazy mounts](https://oak.space/docs/mounts). |
| `git status` | `oak status` | `--short`/`--porcelain` are Git-compatible. |
| `git diff` | `oak diff` | Opens a full-screen viewer on a terminal; `--print` for plain output. |
| `git diff main...feature` | `oak diff feature` | Shows what the branch contributes. |
| `git add -A && git commit -m …` | `oak commit` | `-m` is refused. Describe the branch with `oak desc`. |
| `git commit --amend -m …` | `oak desc "…"` | Change what the branch says it does. |
| `git push` | `oak push` | Publishes the current branch. Pushes to `main` are refused. |
| `git pull` / `git pull --rebase` | `oak pull` | Always merges `main` into your branch. |
| `git fetch` | `oak fetch` | Refreshes your copy of `main`. |
| `git switch <b>` / `git checkout <b>` | `oak switch <b>` | Fetches the branch if you don't have it. |
| `git switch -c <b>` | `oak switch -c [<b>]` | Name is optional; one is generated. |
| `git checkout \<sha\>` | `oak checkout \<sha\>` / `oak switch -d \<sha\>` | Detached HEAD. |
| `git branch` | `oak branch` / `oak branch list` | `--remote` reads the server. |
| `git branch --show-current` | `oak branch --show-current` | |
| `git branch -d <b>` | `oak close <b>` | Closed branches can be reopened from the web. |
| `git merge` / "Squash and merge" | `oak merge` | Server-side squash onto `main`. |
| `git rebase -i` | `oak split` | Reorder, drop, or split a branch's commits into separate branches. |
| `git restore \<path\>` | `oak restore \<path\>` | `-s \<commit\>` to restore from elsewhere. |
| `git reset --hard` | `oak reset` | Discards uncommitted changes. |
| `git log` | `oak log` | `-S` / `-G` pickaxe search work as in Git. |
| `git rev-parse HEAD` | `oak hash` / `oak rev-parse HEAD` | |
| `git archive` | `oak archive` | Zip of the working tree. |
| `.gitignore` | `.oakignore` | Same syntax. Oak falls back to `.gitignore` when there's no `.oakignore`. |
| `.git/hooks/pre-commit` | `.oak/hooks/pre-commit` | Local only, like Git. |
| GitHub Actions | `.oak/workflows/*.yml` | [Native CI](https://oak.space/docs/ci), GitHub-Actions-shaped but much smaller. |
| `CODEOWNERS` | `.oak/PERMISSIONS` | Controls *read access*, not review. See [Path permissions](https://oak.space/docs/path-permissions). |

## Things Git has that Oak doesn't (yet) \#

Being honest about the gaps:

- **Tags.** There's no tag object; refer to commits by hash.
- **Issues, pull-request review threads, line comments.** Review happens on the branch page and in the CLI ( `oak branch review`), not in comment threads.
- **Submodules.** Imports skip them.
- **Push to arbitrary remotes.** You push to an Oak server. To leave, `oak export` writes a complete Git repo you can push anywhere.
- **Branch rename.** `oak branch rename` is temporarily disabled.

## Using Git tools against an Oak repo \#

Stock `git` can clone any public Oak repo as a **read-only snapshot of `main`**:

```bash
git clone https://oak.space/acme/web.git
```

That's one commit with no history, meant for tooling (package managers, `/plugin marketplace add`, deploy scripts). For full history in Git form, use `oak export`. Both are covered in [Git import, export, and interop](https://oak.space/docs/git).

> **Inside an Oak checkout, don't run `git`.** An Oak repo has a `.oak/` directory and no `.git`. If a tool insists on Git, export or snapshot-clone into a separate directory instead.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page