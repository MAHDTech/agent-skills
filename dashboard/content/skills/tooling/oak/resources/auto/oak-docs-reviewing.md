+++
title = "oak-docs-reviewing"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: "Checkout-free review: triage a fleet of branches, read diffs, and merge from the CLI or web."
title: Reviewing branches · Docs
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

# Reviewing branches

Copy page [.md](https://oak.space/docs/reviewing.md "This page as raw Markdown")

Checkout-free review: triage a fleet of branches, read diffs, and merge from the CLI or web.

When agents do the work, review becomes the bottleneck: ten branches come back, and switching your checkout ten times to read them isn't an option. Everything on this page is **checkout-free** — it reads the server (or your local repo) and prints, without touching your working tree.

## Triage first \#

```bash
oak branch triage --remote                    # every open branch, scored
oak branch triage --remote --only mergeable   # just the ones ready to land
oak branch triage --remote --json             # for an agent doing the triage
```

Each row carries:

| Field | Values |
| --- | --- |
| Mergeability | `clean`, `conflicts` |
| Contribution | `contributes`; or `empty` / `superseded_exact` for a branch that adds nothing `main` doesn't already have |
| CI | the branch head's [CI](https://oak.space/docs/ci) status |
| Recommended action | `validate_then_merge`, `resolve`, `close`, `rebuild`, `review`, `do_not_merge` |

`--only mergeable | closable | ambiguous` sorts a fleet before you read a single diff: land the first bucket, close the second (agent runs that produced nothing new), and spend your attention on the third. `--analysis-depth` trades speed for depth, `--limit N` caps how many branches get the deep pass, and `--against <branch>` compares against something other than `main`.

## Review one branch \#

```bash
oak branch review fix-auth--a1b2c3d4 --remote --merge-preview
```

`oak branch review` gathers the evidence for one branch: what it changed, what it's based on, and — with `--merge-preview` — whether it would merge cleanly, plus a merge-safety verdict that flags a prediction which would destroy work already on `main`. Add `--json` for agents.

## Read a diff without switching \#

```bash
oak diff fix-auth--a1b2c3d4                     # full-screen viewer: what the branch contributes
oak branch diff fix-auth--a1b2c3d4 --remote     # summary of changed files
oak branch diff fix-auth--a1b2c3d4 --remote --print          # the patch, to stdout
oak branch diff fix-auth--a1b2c3d4 --remote --json --hunks   # the patch, as JSON
oak diff 3f9fab02 8fcf5aed                      # any two commits
```

Branch diffs come in three modes, chosen with `--diff-mode` (`oak branch diff`) or `--mode` (`oak diff`):

| Mode | Answers |
| --- | --- |
| `contribution` | What did this branch add since it forked from `main`? The honest "what did the agent do". |
| `tree` | How do the two heads differ right now? (Default for `oak branch diff`.) |
| `net-merge` | What would `main` gain if this branch merged now? |

For large branches, `--changed-files-limit` / `--changed-files-offset` page through the file list and `--max-bytes` caps the patch text in JSON output.

## Preview a merge train \#

```bash
oak branch train fix-auth--a1b2 add-search--e5f6 bump-tokio--c9d0 --remote
```

`oak branch train` previews landing several independent branches in order — would each still merge cleanly after the ones before it? — without publishing anything. Up to 32 branches.

## The full-screen viewers \#

`oak diff` and `oak log` open full-screen on a terminal. Both take the keys you already know:

| Style | Keys |
| --- | --- |
| Arrows | ↑ ↓, PgUp / PgDn, Home / End |
| less / vi | `j` `k`, Space (page down), `b` (page up), `d` / `u` (half page), `g` / `G`, `/` to search |
| emacs | `^N` / `^P`, `^V` / `M-v`, `M-<` / `M->`, `^S` to search, `^G` to cancel |

In `oak diff`, Tab or the arrow keys move between the file tree and the diff pane, and Space folds a directory in the tree. Press `?` in either viewer for the complete list. Pipe the output, or pass `--print` / `--json`, to skip the viewer.

## In the browser \#

The repo's **Branches** page on oak.space lists every open branch with its description and a CI status chip. Each branch opens a detail page with:

- the full diff, file by file;
- per-workflow CI pills that link to live run logs;
- a **Seen by** list — opening a branch page records that you've read it;
- **Merge**, **Update** (merge the latest `main` into the branch), and **Close** / **Reopen** buttons, plus conflict resolution in the browser when a merge would conflict.

Check several branches on the list and **Merge selected** lands them one after another — usually how a fan-out ends when the branches don't overlap.

## Then land it \#

```bash
oak switch fix-auth--a1b2c3d4     # fetches it if needed
oak merge --wait                  # wait for CI, then squash onto main
```

See [Merging and conflicts](https://oak.space/docs/merging).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
