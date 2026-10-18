+++
title = "oak-docs-branches"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Branches are the unit of work; the branch description is the commit message.
title: Branches and descriptions · Docs
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

# Branches and descriptions

Copy page [.md](https://oak.space/docs/branches.md "This page as raw Markdown")

Branches are the unit of work; the branch description is the commit message.

In Oak the **branch** is the unit of work, not the commit. A branch is one task — usually one agent session or one change a person wants reviewed — and it ends by being squash-merged onto `main` or closed.

## How branches work \#

- **`main` exists only on the server.** There's no local `main` to commit to, and the server refuses pushes to it. It only changes when a branch is [merged](https://oak.space/docs/merging).
- **Every other branch is parented onto `main`.** New branches start from the latest `main` you've fetched, and `oak pull` keeps them current by merging `main` in.
- **You always have a branch.** `oak init` and `oak clone` create a personal one named `\<you\>-\<6 hex chars\>` (for example `zdgeier-3f2a8b`), so two clones of the same repo never collide on push. Your username comes from your login, or `OAK_AUTHOR`, `USER`, or `USERNAME`.
- **Branches are cheap.** Make one per task. The open-branch view on oak.space and `oak branch triage` exist precisely because a repo worked on by agents has a lot of them.

## Create and switch \#

```bash
oak switch -c                       # new branch off the latest main, generated name
oak switch -c fix-login-redirect    # …with a name
oak switch -c --clean               # …and drop uncommitted changes instead of carrying them over
oak switch fix-login-redirect       # switch to an existing branch (fetched from the server if needed)
oak switch                          # pick from a list
oak switch -d 3f9fab02              # detach HEAD at a commit (same as `oak checkout 3f9fab02`)
```

Uncommitted edits come with you when you create a branch, the way `git checkout -b` works; pass `--clean` to start fresh instead. `oak switch main` is an error — there's no local `main` to switch to.

Inside a [mount](https://oak.space/docs/mounts), `oak switch` is refused: a mount *is* one branch. Mount another branch with `oak mount \<org\>/\<repo\> --branch \<name\>` instead.

## Descriptions \#

Commits carry no message. The **branch description** says what the whole branch does, and it becomes the commit message of the squash commit on `main` when the branch merges.

```bash
oak desc "Retry flaky uploads with jittered backoff"
oak desc --file description.md          # from a file
git log -1 --format=%B | oak desc --file -   # …or from stdin
oak desc --append "Also fixes #212."    # add a paragraph to the existing description
```

`oak desc` saves locally and, if the branch has been pushed, updates the server too; otherwise the next `oak push` carries it. It can be changed any number of times until the branch merges.

Write it for someone reading `main`'s history months from now: what changed and why, not a list of touched files. A first line that works as a title, then a blank line and detail, renders best. If a branch merges without a description, the squash commit's message falls back to the branch name.

`--append` refuses, rather than overwrite, when your local copy of the description is out of date with the server's — it tells you the exact command to refresh with. That makes it safe for several agents to add notes to one branch.

## List and inspect \#

```bash
oak branch                      # local branches
oak branch --show-current       # just the current branch's name
oak branch list --remote        # every branch on the server
oak branch list --remote --status open
oak branch show fix-login-redirect --remote
oak info                        # repo, branch, parent, remote, and linked-repo details
```

Add `--json` to any of them for machine-readable output. To compare or review branches without switching to them, see [Reviewing branches](https://oak.space/docs/reviewing).

## Close and reopen \#

```bash
oak close                                   # close the current branch
oak close old-experiment --remote --json    # close a pushed branch without switching to it
oak close old-experiment --reason "superseded by fix-login-redirect"
```

Merging closes a branch automatically. Close the ones you're abandoning so they drop off the open-branch view. Closing is idempotent and keeps everything — a closed branch can be **reopened** from its page on oak.space (or via the [API](https://oak.space/docs/api#branches)).

## Reshape a branch with oak split \#

`oak split` is Oak's `git rebase -i`. It opens a todo list of the branch's commits where you can reorder them, drop some, or send groups of them to new, independent branches — useful when an agent did two unrelated things on one branch.

```bash
oak split                         # interactive, on the current branch
oak split --from big-branch       # on another branch
oak split --plan plan.txt         # apply a plan non-interactively (or --plan - for stdin)
oak split --dry-run               # print the resulting structure without writing
```

`oak histedit` is an alias.

## Branch names \#

Names can't contain whitespace, control characters, `/` or `\`, and `main` is reserved. Mounts and [agent spaces](https://oak.space/docs/spaces) generate names from the task directory plus a short id, like `fix-auth-redirect--a1b2c3d4`.

`oak branch rename` exists but is **temporarily disabled** (the server returns 503) while a storage-integrity issue is fixed. To rename, create a new branch from the old one's work and close the old one.

## Limits \#

A repo can hold up to 10,000 branches, open and closed. There's no cap on open branches during the beta — see [Plans and limits](https://oak.space/docs/limits).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
