+++
title = "oak-docs-merging"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Squash-merging onto main, resolving conflicts, and the CI merge gate.
title: Merging and conflicts · Docs
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

# Merging and conflicts

Copy page [.md](https://oak.space/docs/merging.md "This page as raw Markdown")

Squash-merging onto main, resolving conflicts, and the CI merge gate.

Merging is how work reaches `main`, and it's the only way. A merge takes a whole branch — every checkpoint on it — and lands it on `main` as **one squash commit** whose message is the [branch description](https://oak.space/docs/branches#descriptions).

Merging is a human decision. Agents should push their branch and stop; a person reviews and merges. (Nothing enforces that — it's the workflow Oak is built around, and what the agent skill teaches.)

## The merge, step by step \#

```bash
oak desc "Cache tile lookups in the renderer"   # this becomes the commit message on main
oak push                                          # publish the branch

oak pull                       # bring main's newer commits into your branch
oak diff --branch              # last look at everything the branch changes
oak merge --dry-run --json     # would it merge cleanly? (local prediction, changes nothing)

oak merge                      # squash onto main and close the branch
```

What each step is for:

- **`oak pull` first.** It merges the current `main` *into your branch*, so conflicts surface on your branch, where you can resolve them calmly — not halfway through landing on `main`.
- **`oak merge --dry-run`** predicts the merge locally without fetching, pushing, or touching files.
- **`oak merge`** asks the server to squash the branch onto its parent ( `main`) and close it. If the repo has CI, the [merge gate](#the-ci-merge-gate) applies.

You can merge a branch you aren't on: `oak merge fix-login-redirect`. And you can merge from the browser — every branch's page on oak.space has a **Merge** button, and the branches list has **Merge selected** for landing several non-overlapping branches at once. The web merge checks for conflicts before you click.

Merging doesn't run inside a [mount](https://oak.space/docs/mounts). Merge from a regular clone (a [sparse](https://oak.space/docs/sparse-clones) one is fine) or from the web.

## What lands on main \#

One commit:

- **Message:** the branch description (or the branch name, if it has none).
- **Contents:** the branch's final tree, merged with `main`.
- **Ancestry:** the squash commit keeps a pointer ( `merge_parent_hash`) to the branch's last checkpoint, so the full pre-squash history stays reachable for tooling, even though `main` shows one line per task.

A merge to `main` fires the `merge` event for [CI](https://oak.space/docs/ci) and [webhooks](https://oak.space/docs/webhooks).

## Resolving conflicts \#

A conflict can come up during `oak pull` (merging `main` into your branch), during `oak merge`, or during `oak pull` inside a mount. They all work the same way.

```bash
oak pull
# CONFLICT in src/render/tiles.rs

oak conflict status              # what's in progress and how much is left
oak conflict show                # per-file details for every conflicted path

$EDITOR src/render/tiles.rs      # resolve by hand: remove the <<<<<<< ======= >>>>>>> markers
oak conflict take src/render/tiles.rs --theirs   # …or take one side of every block in a file

oak pull --continue              # finish
oak pull --abort                 # …or back out; the branch is left as it was
```

During a merge, the same commands finish or cancel it: `oak merge --continue`, `oak merge --abort`.

- **Markers are line-level.** Oak wraps only the diverging lines, not whole files.
- **`--ours`** keeps your branch's side; **`--theirs`** keeps `main`'s (or the remote's).
- **Every `oak conflict` command takes `--json`**, so handing resolution to an agent is a supported path. `oak diff --check` catches markers you forgot to remove.
- While a conflict is in progress, commands that would change the branch exit with code `5`.

### When a pushed branch conflicts \#

If an agent's branch has gone stale against `main`, you don't need to redo the work. [Mount the branch itself](https://oak.space/docs/mounts#continue-an-existing-branch), pull, resolve, and push:

```bash
oak mount acme/web ./fix-auth --branch fix-auth--a1b2c3d4
cd fix-auth
oak pull            # conflicts appear here
# resolve, or hand the mount to an agent to resolve
oak pull --continue
oak commit && oak push
```

## The CI merge gate \#

If the repo has [CI workflows](https://oak.space/docs/ci), a merge onto `main` is **refused while the branch head's CI is failing or still running**. Specifically: for the commit at the tip of the branch, the latest run of each workflow must have concluded `success` (or been skipped). A tip with no runs at all isn't gated.

```bash
oak ci status          # the gate's verdict for your branch head — exit 0 pass, 1 fail, 3 still running
oak merge --wait       # wait for CI to finish (default 30 min; --wait=60 for longer), then merge if it passed
oak ci logs <run-id>   # read the failure
oak ci rerun <run-id>  # re-run it, if the failure was infrastructure rather than code
oak merge --force      # deliberately override the gate — after you've read why it's red
```

A forced merge is recorded in the organization's audit log. Over the API, the gate is HTTP **412**, and `?force=1` overrides it — see [the API reference](https://oak.space/docs/api#branches).

## Undoing a merge \#

Open the commit on oak.space and click **Revert**: Oak lands a new commit on `main` that applies the inverse of that commit's changes, like `git revert`. If any file the commit touched has changed since, the revert is refused rather than guessed at. The API equivalent is `POST /api/{owner}/{name}/commits/{hash}/revert`.

## When a merge is refused \#

| What you see | Meaning | What to do |
| --- | --- | --- |
| CI gate (412) | CI on the branch head is red or still running. | `oak ci status`, `oak merge --wait`, or fix and push. |
| Conflicts (409 with `conflict_paths`) | The branch and `main` changed the same lines. | `oak pull`, resolve, push, merge again. |
| `main_moved` / `branch_moved` (409, retryable) | Someone landed on `main`, or pushed to the branch, during the merge. Nothing was merged. | Retry. |
| "already closed" (400) | The branch was already merged or closed. | Nothing — check `oak branch show \<name\> --remote`. |
| "Repository isn't linked to an organization yet" | The repo has never been pushed. | `oak push --repo \<org\>/\<name\>` first. |

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
