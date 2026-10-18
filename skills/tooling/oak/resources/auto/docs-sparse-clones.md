---
description: Check out only part of a monorepo, Perforce-style, with `oak clone --path`.
title: Sparse clones · Docs
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

# Sparse clones

Copy page [.md](https://oak.space/docs/sparse-clones.md "This page as raw Markdown")

Check out only part of a monorepo, Perforce-style, with \`oak clone --path\`.

A **sparse clone** checks out only part of a repository — Perforce-style. Pass one or more path prefixes, and only files under them are downloaded and written to disk:

```bash
oak clone acme/monorepo --path services/api --path libs/shared
# or: --path services/api,libs/shared
```

The set of prefixes is the **cone**.

## How it behaves \#

- **The directory structure still comes down.** Oak's trees are content-addressed and verified on the client, so the full listing arrives; files outside the cone are listed but their content is never fetched.
- **Commits carry out-of-cone files forward untouched.** Narrowing a checkout never deletes what it leaves out, and `oak status` never reports those files as missing.
- **Everything else is normal.** Commit, push, pull, and merge work exactly as in a full clone — including `oak merge`, which a [mount](https://oak.space/docs/mounts) can't do.

## Change the cone \#

```bash
oak sparse                       # show the current cone (--json for machines)
oak sparse set services/web      # replace the cone and re-sync the working tree
oak sparse add docs/             # widen it
oak sparse disable               # back to a full checkout
```

After widening, files that were never downloaded are listed; run `oak pull` to fetch them.

## Sparse clone or mount? \#

For a very large repo, a [lazy mount](https://oak.space/docs/mounts) is usually the better default: it fetches any path on demand, with no cone to manage. Reach for a sparse clone when you want a plain on-disk checkout of a subtree — for builds, for tools that dislike virtual filesystems, for merging from — without the per-machine setup a mount needs.

Sparse clones and [path permissions](https://oak.space/docs/path-permissions) share a mechanism: both withhold file content while keeping the tree intact. The difference is who decides — a sparse clone is a bandwidth choice you make; path permissions are access control set by the repo's admins.

## Other ways to clone less \#

| Flag | Effect |
| --- | --- |
| `oak clone --shallow` | Only the latest commit on `main`, no history. |
| `oak clone --from ../other-checkout` | Reuse content already in another local checkout of the same repo instead of downloading it again. |
| `oak clone --detached` | Check out `main`'s head without creating a personal branch — for read-only tooling. |

## Recovery switch \#

If a server is ever in a broken state and a normal clone or pull fails on a missing file, `OAK_ALLOW_PARTIAL_CLONE=1` skips each missing file (and reports it) instead of failing. It's for recovery only — use `--path` when you want part of a repo on purpose.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page