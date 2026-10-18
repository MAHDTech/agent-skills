+++
title = "oak-docs"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: What Oak is, the five ideas behind it, and where to go next.
title: Documentation
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

# Introduction

Copy page [.md](https://oak.space/docs/index.md "This page as raw Markdown")

What Oak is, the five ideas behind it, and where to go next.

Oak is the **agentic substrate** for software development: the version-control and storage layer that coding agents — and the people directing them — build on. It's where your agent (Claude Code, Codex, Cursor, or anything else) reads, writes, branches, and publishes code and assets.

Oak doesn't run agents. Bring your own; Oak is the version control system they push to. It makes no AI calls on your behalf and never trains on your code.

## The five ideas \#

Everything in these docs follows from five decisions. If you only read one section, read this one.

1. **A branch is a task.** Not a long-lived line of development — one unit of work, usually one agent session. `oak init` and `oak clone` put you on a fresh personal branch, and branches are cheap enough to make one per task.
2. **Commits are checkpoints; the branch description is the message.** `oak commit` takes no `-m`. What the change *is* gets written once, on the branch, with `oak desc`.
3. **`main` lives on the server, and only merges reach it.** You never commit to `main` locally and direct pushes to it are refused. Publishing a branch and merging it are separate steps, and merging is a human's call.
4. **Merging squashes.** A merged branch becomes exactly one commit on `main`, whose message is the branch description. `main` reads as one line per task.
5. **You don't have to clone.** `oak mount` puts a working tree on top of a remote repo and fetches files as they're read, so a 20 GB monorepo is editable in seconds — and ten parallel tasks means ten mounts, not ten clones.

Around that core, oak.space adds what a team needs to land changes safely: [native CI](https://oak.space/docs/ci) that gates merges, [path permissions](https://oak.space/docs/path-permissions) declared in the repo, organizations, webhooks, and a JSON API.

## Where to start \#

| If you want to… | Read |
| --- | --- |
| Install the CLI and sign in | [Install and sign in](https://oak.space/docs/install) |
| Get a repo onto Oak and merge your first change | [Quickstart](https://oak.space/docs/quickstart) |
| Translate what you know from Git | [Coming from Git](https://oak.space/docs/coming-from-git) |
| Point a coding agent at Oak | [Working with agents](https://oak.space/docs/agents) |
| Run several agents on one big repo | [Example: a fleet of agents](https://oak.space/docs/example-session) |
| Set up CI | [Continuous integration](https://oak.space/docs/ci) |
| Look up a command or flag | [CLI reference](https://oak.space/docs/cli) |
| Script against oak.space | [HTTP API reference](https://oak.space/docs/api) |

## Platforms \#

The `oak` CLI ships for **macOS** (Apple Silicon and Intel), **Linux x86\_64**, and **Windows x86\_64**. Lazy mounts use each OS's own virtual-filesystem layer — FSKit on macOS 26+, FUSE on Linux, the Projected File System on Windows. Linux ARM64 binaries are published on the [GitHub releases page](https://github.com/oakdotspace/oak/releases) but the installer doesn't select them yet.

## For agents \#

Every page has a **Copy page** button and a **.md** link that return its Markdown source. [`/docs/llms-full.txt`](https://oak.space/docs/llms-full.txt) is every page in one file. And `oak skill install` drops an `oak-vcs` skill into your repo so agents pick up the workflow without being told — see [Working with agents](https://oak.space/docs/agents#install-the-agent-skill).

## Your data stays yours \#

`oak export` replays any Oak repo's full history into an ordinary Git repository — authors, emails, and timestamps intact. It's the documented escape hatch, and it always works. See [Git import, export, and interop](https://oak.space/docs/git).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
