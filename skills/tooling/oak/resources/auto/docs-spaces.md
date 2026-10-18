---
description: One directory per org, one subdirectory per task, one mount per repo the task touches.
title: Agent spaces · Docs
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

# Agent spaces

Copy page [.md](https://oak.space/docs/spaces.md "This page as raw Markdown")

One directory per org, one subdirectory per task, one mount per repo the task touches.

An **agent space** is a directory where coding agents work across one organization's repos. Each task gets its own subdirectory, and inside it you mount whichever repos the task needs — each on its own branch. It's like a set of Git worktrees, except there's no clone behind any of them, so starting a task takes seconds.

```text
acme/                          ← the space (one per org)
├── AGENTS.md
├── fix-auth-redirect/         ← one task
│   └── web/                   ← a mount of acme/web, on branch fix-auth-redirect--a1b2c3d4
└── unify-errors/              ← a cross-repo task
    ├── web/                   ← mount of acme/web,   branch unify-errors--…
    └── api/                   ← mount of acme/api,   branch unify-errors--…
```

Spaces are built on [lazy mounts](https://oak.space/docs/mounts), so the [one-time mount setup](https://oak.space/docs/mounts#setting-up-mounts) applies.

## Create a space \#

```bash
oak space new acme              # creates ./acme
oak space new acme ~/work/acme  # …or somewhere else
```

This writes:

| File | Purpose |
| --- | --- |
| `AGENTS.md` | The workflow, written for an agent: picking a task slug, mounting, finishing, and the mount traps to avoid. Edit it to add your team's conventions. |
| `CLAUDE.md` | A one-line `@AGENTS.md` import, for Claude Code. |
| `.claude/settings.json` | Pre-approves the `oak` commands agents need, and lists mounts when a Claude Code session stops. |
| `.oak-space` | Marks the directory as a space for the org. |

Nothing is cloned or mounted yet.

## The per-task loop \#

1. **Pick a slug** — short and kebab-case, like `fix-auth-redirect`. It becomes the task directory and the start of each branch name.
2. **See what's available:** `oak space repos` lists the org's repos (add `--json` for agents).
3. **Mount each repo the task needs** under the task directory:

   ```bash
   oak mount acme/web ./fix-auth-redirect/web
   ```

4. **Work inside each mount.** `oak status`, `diff`, `commit`, and `push` all act on that mount's branch.
5. **Finish each repo you touched:**

   ```bash
   oak mount finish ./fix-auth-redirect/web --desc-file desc.md --json   # from the space root
   # or, from inside the mount: oak finish --desc-file desc.md --json
   ```

    This sets the description, checkpoints, pushes, and unmounts — unmounting only after the push succeeds.
6. **Sweep up:** `oak space clean` tears down every mount in the space that has nothing uncommitted or unpushed. Add a path to clean one task; `--force` also removes dirty mounts, discarding their changes.

Each repo's mount is independent: describe, commit, and push them separately, even within one cross-repo task.

## Take stock \#

```bash
oak space inventory --verify-local --include-ci --json
```

`oak space inventory` walks a directory tree and reports every Oak checkout and mount it finds. `--verify-local` adds each one's dirty-file and unpushed-commit counts; `--include-ci` adds the CI state of each one's head. It's an observation, not a guarantee — it never certifies that a directory is safe to delete.

## Running agents in a space \#

Launch agents from the space root so they read `AGENTS.md`, and tell each which task directory is its own. [Example: a fleet of agents](https://oak.space/docs/example-session) walks through fanning out, reviewing, and merging end to end.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page