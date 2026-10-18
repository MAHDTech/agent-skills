+++
title = "docs-quickstart"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Create or clone a repo, make a change, publish a branch, and merge it — in five minutes.
title: Quickstart · Docs
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

# Quickstart

Copy page [.md](https://oak.space/docs/quickstart.md "This page as raw Markdown")

Create or clone a repo, make a change, publish a branch, and merge it — in five minutes.

This page takes you from nothing to a change merged on `main`. It assumes you've [installed the CLI and signed in](https://oak.space/docs/install).

## 1. Get a repository \#

Pick whichever matches where your code is now.

**Start a new repo:**

```bash
oak init my-app
cd my-app
```

**Clone one that's already on Oak:**

```bash
oak clone acme/web      # or just `oak clone` to pick from a list
cd web
```

**Bring a Git repo over:**

```bash
oak clone https://github.com/acme/web.git     # fetches with git, replays history into Oak
cd web
```

…or run `oak init` inside an existing Git checkout and accept the import prompt. [Git import, export, and interop](https://oak.space/docs/git) covers the details, including importing from GitHub in the browser with nothing installed.

**Working against a huge repo?** Skip the clone and [mount](https://oak.space/docs/mounts) it instead — you're editing in seconds.

Whichever way you came in, you're now on a **personal branch** — something like `zdgeier-3f2a8b` — parented onto `main`. You never work on `main` directly; see [Branches and descriptions](https://oak.space/docs/branches).

## 2. Make a change and checkpoint it \#

Edit some files, then:

```bash
oak status          # what changed
oak diff            # the changes themselves (full-screen viewer on a terminal)
oak commit          # save a local checkpoint
```

`oak commit` takes no message, on purpose. Commit as often as you like — every checkpoint gets folded into one commit when the branch merges.

## 3. Describe the change \#

```bash
oak desc "Add a dark-mode toggle to the settings page"
```

The **branch description** is the commit message for the whole branch. It becomes the message of the single commit that lands on `main` when you merge, so write it for someone reading `main`'s history later.

## 4. Publish the branch \#

If you **cloned** the repo, it's already linked to the server:

```bash
oak push
```

If you **created** it with `oak init` (or imported from Git), the first push also creates the repo on oak.space, so Oak needs to know which organization owns it. On an interactive terminal, plain `oak push` asks you to pick one. In a script or an agent run, there's no terminal to ask, so name it:

```bash
oak push --repo acme/my-app
```

`acme` must be an existing organization you can publish to — every account has a personal one named after its username. The repo itself is created if it doesn't exist. After this first link, plain `oak push` works.

> **Agents:** never guess the owner. If you don't know which organization to publish into, ask. `OAK_REPO=acme/my-app` works in place of the flag.

Your branch now shows up on the repo's **Branches** page at `https://oak.space/acme/my-app/branches`. Open it in a browser with `oak open`.

## 5. Merge it \#

When the change is ready, bring in anything that's landed on `main` since you started, then merge:

```bash
oak pull            # merge the latest main into your branch
oak merge           # squash the branch onto main
```

Or click **Merge** on the branch's page in the web UI. Either way, `main` gains exactly one commit whose message is your branch description, and the branch is closed.

If the repo has [CI](https://oak.space/docs/ci), the merge waits for it to pass — `oak merge --wait` rides it out for you. If `oak pull` hits a conflict, [Merging and conflicts](https://oak.space/docs/merging) walks through resolving it.

## 6. Start the next task \#

```bash
oak switch -c       # new branch off the latest main, with a generated name
oak switch -c fix-login-redirect   # …or name it yourself
```

That's the whole loop: **branch → edit → commit → describe → push → merge**.

## Where next \#

- [Coming from Git](https://oak.space/docs/coming-from-git) — command-by-command translation.
- [Making changes](https://oak.space/docs/making-changes) — ignore files, hooks, undoing things.
- [Working with agents](https://oak.space/docs/agents) — set up Claude Code, Codex, or Cursor to follow this loop on its own.
- [Continuous integration](https://oak.space/docs/ci) — add a workflow file and gate merges on it.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
