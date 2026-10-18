+++
title = "docs-example-session"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: "End to end: several agents on one big repo, each on its own mounted branch, then review and merge."
title: "Example: a fleet of agents · Docs"
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

# Example: a fleet of agents

Copy page [.md](https://oak.space/docs/example-session.md "This page as raw Markdown")

End to end: several agents on one big repo, each on its own mounted branch, then review and merge.

This is the end-to-end shape the rest of the docs assume: one large repo, several agents working on it at once — each on its own mounted branch — and one person reviewing and merging what comes back.

## The shape of it \#

Five lines, before any commands:

1. **A branch is a task** — one unit of work, usually one agent session.
2. **A mount is a working copy of that branch.** Files arrive as they're read, so a 20 GB monorepo is editable in seconds, and N tasks means N mounts, not N clones.
3. **Commits are checkpoints; the branch description is the message.**
4. **Publishing isn't merging.** Each agent pushes its branch and stops. Nothing reaches `main` until a person merges it.
5. **Merging squashes.** `main` gets one commit per task, however many checkpoints the agent made.

If you're coming from Git: mounts do what worktrees do, minus the clone behind each one; the branch description does what a PR title and body do, minus the PR; and the squash merge is what "Squash and merge" does, except it's the only merge there is.

## 1. Set up a space \#

Mounts need a one-time per-machine step first — the Oak Mount app on macOS, `fuse3` on Linux, ProjFS on Windows. See [Setting up mounts](https://oak.space/docs/mounts#setting-up-mounts).

```bash
curl -fsSL https://oak.space/install | sh
oak login

oak space new acme        # scaffolding only — nothing is cloned
cd acme
oak space repos           # which repos can this space see?
# acme/monorepo
# acme/infra
# acme/docs
```

That leaves:

```text
acme/
├── AGENTS.md              # the workflow, written for the agent
├── CLAUDE.md              # one-line @AGENTS.md import
├── .oak-space             # marker recording the org
└── .claude/settings.json  # pre-approves the oak commands agents need
```

The `AGENTS.md` isn't a stub. It's the agent's full operating manual for a space: how to pick a task slug, mount per repo, that `oak commit` takes no message, how to finalize a task, and the mount-specific traps (keep `CARGO_TARGET_DIR` and `node_modules` off the mount, don't run whole-tree formatters across it, how to recover a wedged mount). An agent that follows it does the right thing without further prompting. It's a plain file — add your team's conventions and every agent opened in the space picks them up.

## 2. Fan out \#

One task = one subdirectory = one mount per repo it touches = one branch per mount. Tasks are isolated by construction: separate directories, separate overlays, separate branches, and none of them can touch `main`.

```bash
# From the space root. Each returns in seconds.
oak mount acme/monorepo ./fix-auth-redirect/monorepo
oak mount acme/monorepo ./add-search-index/monorepo
oak mount acme/monorepo ./bump-tokio/monorepo

oak mount list
#   ./fix-auth-redirect/monorepo
#     -> acme/monorepo@8fcf5aed1b2c (virtual branch: fix-auth-redirect--a1b2c3d4)
#   ./add-search-index/monorepo
#     -> acme/monorepo@8fcf5aed1b2c (virtual branch: add-search-index--e5f6a7b8)
#   ./bump-tokio/monorepo
#     -> acme/monorepo@8fcf5aed1b2c (virtual branch: bump-tokio--c9d0e1f2)
```

Branch names come from the *task* directory, so they read as work rather than as repo names. A cross-repo task gets one branch per repo — mount `./unify-errors/monorepo` and `./unify-errors/infra` side by side and they share the slug.

Now launch one agent per task, from the space root so each reads `AGENTS.md`. With Claude Code:

```bash
claude -p "Task slug fix-auth-redirect, mounted at ./fix-auth-redirect/monorepo.
The OAuth callback drops the ?next= param. Fix it, then finalize per AGENTS.md." &

claude -p "Task slug add-search-index, mounted at ./add-search-index/monorepo.
Add a trigram index behind the search-index flag. Finalize per AGENTS.md." &

claude -p "Task slug bump-tokio, mounted at ./bump-tokio/monorepo.
Bump tokio to 1.42 across the workspace and fix the fallout. Finalize per AGENTS.md." &

wait
```

You can also skip pre-mounting and just name a slug — `AGENTS.md` tells the agent to mount what it needs. Pre-mounting is worth it when you're scripting the fan-out and want mount failures to surface up front rather than inside a transcript.

## 3. Each agent finishes its own branch \#

Per `AGENTS.md`, each agent ends with:

```bash
cd ./fix-auth-redirect/monorepo
oak finish --desc-file /tmp/desc.md --json
```

One command that checks it can publish, sets the branch description from the file, checkpoints anything uncommitted, pushes, and tears the mount down — only after the push succeeds. If a step fails, the mount stays intact and the JSON names the next command.

Watch the fleet with `oak mount list --json` (dirty and unpushed counts per mount). Pushed branches appear live on the repo's **Branches** page.

## 4. Review what came back \#

Three branches, no checkouts, and you don't want to switch your working tree three times. Start with triage:

```bash
oak branch triage --remote                     # every open branch, scored
oak branch triage --remote --only mergeable
oak branch triage --remote --only ambiguous
```

Then read the ones that need you:

```bash
oak branch review fix-auth-redirect--a1b2c3d4 --remote --merge-preview
oak diff fix-auth-redirect--a1b2c3d4                       # full-screen, no checkout
oak branch diff add-search-index--e5f6a7b8 --remote --diff-mode net-merge
```

Or do it in the browser: the **Branches** page lists every open branch with its CI status, and each one opens to its diff with a **Merge** button. [Reviewing branches](https://oak.space/docs/reviewing) has the details.

## 5. Land them \#

Merging is a decision, so it happens from somewhere you control — the web UI or a real checkout, not a mount:

```bash
# In a clone of the repo (a sparse one is fine)
oak switch fix-auth-redirect--a1b2c3d4
oak merge --wait                    # wait for CI, then squash onto main
```

Or check off the clean branches on the **Branches** page and click **Merge selected**.

**When a branch conflicts**, don't re-run the agent from scratch — mount the branch itself and fix it in place:

```bash
oak mount acme/monorepo ./fix-auth-redirect/monorepo --branch fix-auth-redirect--a1b2c3d4
cd ./fix-auth-redirect/monorepo
oak pull                     # merge the new main in; conflicts surface here
# resolve, or hand this mount to an agent
oak pull --continue
oak commit && oak push
```

A plain `oak mount` starts a fresh branch; `--branch` continues an existing one. That's also how a follow-up prompt keeps working on a branch an agent already pushed.

## 6. Clean up \#

```bash
oak space clean                  # tear down every mount with nothing in flight
oak space clean ./bump-tokio     # or just one task
```

Mounts with uncommitted or unpushed work are left alone unless you pass `--force`, so a sweep can't eat a session you forgot about.

## All of it at once \#

```bash
# once per machine
curl -fsSL https://oak.space/install | sh && oak login

# once per org
oak space new acme && cd acme

# fan out
oak mount acme/monorepo ./fix-auth-redirect/monorepo
oak mount acme/monorepo ./add-search-index/monorepo
claude -p "task fix-auth-redirect …" &
claude -p "task add-search-index …" &
wait
oak mount list --json

# review, checkout-free
oak branch triage --remote --json
oak branch review <branch> --remote --merge-preview

# land
oak switch <branch> && oak merge --wait      # or Merge in the web UI

# sweep
oak space clean
```

Scaling up is more of the same: more task directories, more mounts, more agents. The two limits worth knowing are the mount's — keep build output and `node_modules` off it — and your own review throughput, which is what `oak branch triage` exists to protect.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
