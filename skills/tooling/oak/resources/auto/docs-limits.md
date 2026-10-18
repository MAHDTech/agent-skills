---
description: Storage, CI time limits, and the other limits that apply during the beta.
title: Plans and limits · Docs
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

# Plans and limits

Copy page [.md](https://oak.space/docs/limits.md "This page as raw Markdown")

Storage, CI time limits, and the other limits that apply during the beta.

Oak is **free while it's in beta**. Every organization gets the same generous allowances, and nothing is behind a paywall. Paid plans will come back after the beta; if you need more than what's here in the meantime, email [zach@oak.space](mailto:zach@oak.space).

## Beta allowances \#

|  | Per organization |
| --- | --- |
| Storage | 100 GB |
| Repositories | Unlimited (up to 1,000) |
| Members | Unlimited |
| Open branches | Unlimited |
| CI runners | Unlimited — your own machines |

## Storage \#

Storage is counted per organization, after deduplication: content shared between repos — or between versions of a file — is stored and counted once. See an organization's usage on its **Settings** page, or `GET /api/orgs/{slug}/storage`; a single repo's with `GET /api/{owner}/{name}/size`.

The storage quota is enforced: a push that would exceed it is refused. Oak emails the organization's owners as it gets close.

## CI \#

CI runs on [your own runners](https://oak.space/docs/ci-runners), so there are no CI minutes or concurrency limits: more runners run more jobs at once. A whole run is stopped and marked `timed_out` after **6 hours**; a job whose runner stops reporting for 10 minutes fails; and a job no runner picks up within 24 hours is cancelled.

More in [Continuous integration](https://oak.space/docs/ci#limits).

## Hard limits \#

These apply on every plan.

| Limit | Value |
| --- | --- |
| Repositories per organization | 1,000 |
| Branches per repository (open and closed) | 10,000 |
| Data in a single push | 25 GiB |
| File versions (blobs) in a single push | 50,000 |
| Organizations you can own | 10 |
| Workflow file size | 64 KiB |
| CI secret value | 64 KiB |
| CI cache per repository | 10 GiB |
| `.oak/PERMISSIONS` file | 64 KiB |

If one of these is in your way, [say so](mailto:zach@oak.space).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page