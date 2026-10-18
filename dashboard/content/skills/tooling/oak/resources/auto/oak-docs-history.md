+++
title = "oak-docs-history"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Read the log, search it, and inspect files and trees at any commit.
title: History and inspection · Docs
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

# History and inspection

Copy page [.md](https://oak.space/docs/history.md "This page as raw Markdown")

Read the log, search it, and inspect files and trees at any commit.

## The log \#

```bash
oak log                        # full-screen viewer on a terminal: commit list + detail pane
oak log -n 20 --oneline        # the last 20, one per line
oak log --verbose              # include each commit's changed files
oak log src/parser/            # only commits touching these paths
oak log --json                 # machine-readable
```

On your branch, the log shows your checkpoints. On `main` — `oak log` after `oak switch -d` to a main commit, or the repo's history on oak.space — it's one squash commit per merged branch, each carrying that branch's description as its message.

The viewer takes arrows, `less`/vi, and emacs keys; press `?` for the list. See [the full-screen viewers](https://oak.space/docs/reviewing#the-full-screen-viewers).

### Search history \#

```bash
oak log -S parse_header        # commits that add or remove the literal string (like git log -S)
oak log -G 'fn parse_\w+'      # commits with an added/removed line matching a regex (like git log -G)
```

### Read a remote branch's history without fetching it \#

```bash
oak log --remote --json --branch fix-auth--a1b2c3d4 -n 50
```

A bounded, read-only walk of the branch's history on the server. Each row is hash-verified; when the list is truncated, the output says so and gives a `--from \<hash\>` to continue from. Requires `--json`.

## Where am I? \#

```bash
oak hash                       # current HEAD commit
oak rev-parse --short HEAD     # Git-compatible form, for scripts
oak branch --show-current      # current branch name
oak info                       # repo, branch, parent, remote, and linked-repo details
oak open                       # open this repo on oak.space (--print to just print the URL)
```

## Inspect files and trees at a commit \#

These read committed content without changing your working tree — useful for agents that need one file from another commit without a checkout.

```bash
oak file inspect src/main.rs --at 3f9fab02            # one file at a commit
oak file inspect src/main.rs --at HEAD --output /tmp/main.rs
oak file inspect src/main.rs --at <full-hash> --remote   # from the server, no clone needed

oak tree inspect --at HEAD --max-files 500 --json      # a commit's whole tree, bounded
oak refs inspect --json                                # every local branch and its head
```

`file inspect` caps output at 16 MiB by default (`--max-bytes` to change it). All three take `--json`.

## Browse files at any commit \#

`oak checkout \<hash\>` (or `oak switch -d \<hash\>`) puts your working tree at an old commit with a detached HEAD. On oak.space, every commit page has **Browse files at this commit**, and the API serves trees and raw files by commit — `GET /api/{owner}/{name}/tree/{commit}` and `/raw/{commit}/{path}`; see [the API reference](https://oak.space/docs/api#commits-and-content).

## Save uncommitted work without committing \#

```bash
oak change capture --json                    # snapshot uncommitted changes as immutable objects
oak change capture src/ --json               # …or only part of them
oak change export <capture-id> --output work.zip
```

`oak change capture` stores your current uncommitted changes durably — like a stash that doesn't touch the working tree or move any branch — and prints a capture id. `oak change export` writes one capture out as a self-contained zip (a manifest plus the changed files) you can hand to someone else.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
