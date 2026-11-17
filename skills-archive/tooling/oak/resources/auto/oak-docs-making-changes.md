---
description: Status, diff, checkpoints, push and pull, ignore files, hooks, and undoing things.
title: Making changes · Docs
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

# Making changes

Copy page [.md](https://oak.space/docs/making-changes.md "This page as raw Markdown")

Status, diff, checkpoints, push and pull, ignore files, hooks, and undoing things.

The everyday loop: see what changed, checkpoint it, publish it, and stay in sync with `main`.

## See what changed \#

```bash
oak status                 # modified, added, deleted files
oak status --short         # Git-compatible short format (same as --porcelain)
oak status --json --compact  # bounded, machine-readable — what agents should use
```

```bash
oak diff                   # working tree vs. your last checkpoint
oak diff src/ README.md    # only these paths
oak diff --branch          # everything on this branch (commits + uncommitted) vs. where it forked from main
oak diff --stat            # per-file +/- counts
oak diff --name-only       # just the paths
oak diff --print           # plain unified diff to stdout instead of the viewer
oak diff --check           # whitespace errors and leftover conflict markers; exits 1 if any
```

On a terminal, `oak diff` opens a full-screen viewer — a file tree beside the diff — that takes arrow keys, `less`/vi keys, and emacs keys. Press `?` inside it for every binding. Piped, or with `--print` / `--json`, it prints instead. Set `OAK_DIFF_TOOL` to use an external diff tool.

`oak diff` can also compare branches and commits without checking anything out — see [Reviewing branches](https://oak.space/docs/reviewing#read-a-diff-without-switching).

## Checkpoint with oak commit \#

```bash
oak commit                 # snapshot every change in the working tree
oak commit src/parser/     # only changes under these paths
oak commit --push          # checkpoint, then publish the branch
```

There's no staging area and **no commit message** — `-m` is refused with a pointer to `oak desc`. Commits are checkpoints you can return to; what the branch *does* is written once, as its [description](https://oak.space/docs/branches#descriptions). Commit as often as is useful: they're all squashed into one commit on `main` at merge time.

`oak commit` only writes to your local repo. Nothing leaves your machine until you push.

## Publish with oak push \#

```bash
oak push
```

Pushes the current branch's new commits — and its description — to the server. Only the content chunks the server doesn't already have are uploaded, so re-pushing a large binary with a small change sends only the changed chunks.

| Situation | What to do |
| --- | --- |
| First push of a repo made with `oak init` | `oak push --repo \<org\>/\<name\>` (or answer the prompt on a terminal). See [Quickstart](https://oak.space/docs/quickstart#4-publish-the-branch). |
| Push was rejected because the remote branch moved | `oak pull`, then push again. |
| You really mean to overwrite the remote branch | `oak push --force`. |
| You want to see what would be sent first | `oak push --plan --json` (read-only). |

Pushing to `main` is always refused — `main` only changes by [merging](https://oak.space/docs/merging).

## Stay in sync with oak pull \#

```bash
oak pull
```

`oak pull` does two things: it fetches any new commits on **your** branch from the server (pushed from another machine, or by an agent), then merges the latest **`main`** into your branch. Conflicts surface here, on your branch, where you can take your time — see [Merging and conflicts](https://oak.space/docs/merging#resolving-conflicts).

| Command | What it does |
| --- | --- |
| `oak pull` | Fetch your branch, then merge `main` into it. |
| `oak pull --branch-only` | Fetch your branch only; skip merging `main`. |
| `oak fetch` | Refresh your local copy of `main` without touching your branch or working tree. |
| `oak pull --force` | Throw away local commits the server doesn't have and match the remote branch. |

## Undo things \#

```bash
oak restore path/to/file      # put a file back to your last checkpoint
oak restore -s 3f9fab02 file  # …or to how it was at another commit
oak reset                     # discard every uncommitted change (asks first; -f to skip)
oak reset src/                # …or just under a path
```

To go back to an earlier checkpoint as a starting point, `oak checkout \<hash\>` detaches HEAD there; `oak switch -c` from that state starts a new branch. To reshape a branch's commits — drop one, reorder them, or split them into separate branches — use `oak split` (Oak's answer to `git rebase -i`).

## Ignore files \#

Put gitignore-syntax patterns in **`.oakignore`** at the repo root. If there's no `.oakignore`, Oak reads `.gitignore` instead — so an imported Git repo keeps working unchanged. (When both exist, only `.oakignore` counts.)

Always ignored, whatever the files say: `.oak/` (except the tracked config files below), `.git/`, and OS clutter such as `.DS_Store`, `._*` AppleDouble files, `Thumbs.db`, and `desktop.ini`.

`oak init` offers a ready-made `.oakignore` when it detects a Godot, Unity, or Unreal project.

## Tracked files under .oak/ \#

The `.oak/` directory holds your local repository database, which is never committed. A few files inside it are ordinary, versioned files that the server reads:

| Path | Purpose |
| --- | --- |
| `.oak/workflows/*.yml` | [CI workflows](https://oak.space/docs/ci-workflows) |
| `.oak/PERMISSIONS` | [Path permissions](https://oak.space/docs/path-permissions) |
| `.oak/attributes` | Reserved for per-path attributes |

## Hooks \#

Executable files at `.oak/hooks/\<event\>` run at fixed points, like Git hooks:

| Hook | When | On failure |
| --- | --- | --- |
| `pre-commit` | Before `oak commit` writes anything | Aborts the commit |
| `post-commit` | After a successful commit | Prints a warning |

Hooks run from the repo root with `OAK_HOOK` set to the event name. They're **local only** — never pushed — so each person chooses what runs on their machine; files ending in `.sample` are ignored. Skip them for one commit with `oak commit --no-verify`. Hooks don't run inside a [mount](https://oak.space/docs/mounts).

A typical Rust `pre-commit`:

```bash
#!/bin/sh
set -e
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

## Large files \#

There's nothing to configure. Every file is split into content-defined chunks and stored once per organization, so large binaries, game assets, and datasets are just files: pushes and pulls move only the chunks that changed, and the same asset in two repos of one org is stored once. A single push can carry up to 25 GiB; see [Plans and limits](https://oak.space/docs/limits).

## Exit codes \#

Every command uses the same exit codes, so scripts can branch without parsing output — for example `5` means "conflicts" and `3` means "repository locked, retry". The full table is in the [CLI reference](https://oak.space/docs/cli#exit-codes).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page