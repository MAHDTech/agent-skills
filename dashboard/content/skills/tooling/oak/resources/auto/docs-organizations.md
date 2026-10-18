+++
title = "docs-organizations"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Organizations, members and roles, repo visibility, collaborators, and groups.
title: Organizations and access · Docs
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

# Organizations and access

Copy page [.md](https://oak.space/docs/organizations.md "This page as raw Markdown")

Organizations, members and roles, repo visibility, collaborators, and groups.

## Organizations \#

Every repository belongs to an **organization**, and its URL is `oak.space/\<organization\>/\<repo\>`. Every account comes with a **personal organization** named after its username; create shared ones for teams and companies from your dashboard (or `POST /api/orgs`).

The organization is also where storage is accounted and **deduplicated**: content is stored once per organization, so the same large asset committed to five of its repos costs one copy. Storage quotas are per organization — see [Plans and limits](https://oak.space/docs/limits) — and so are [CI runners](https://oak.space/docs/ci-runners): an organization's runners take only its repos' jobs, managed under **Settings → Runners**.

A personal organization always has exactly one member. To work with others, create a shared organization and invite them.

## Members and roles \#

| Role | Can |
| --- | --- |
| **Owner** | Everything, including deleting the organization. |
| **Admin** | Manage members, groups, repository access, service accounts, and settings. Full access to every repo. |
| **Member** | Create repos in the organization, and read or write the repos they've been given access to. |

Manage members under **Settings → Members** on the organization's page (`oak.space/\<org\>/settings`). Admins and owners can always read and write every repo in the organization, and are the only people who can change a repo's [path permissions](https://oak.space/docs/path-permissions) once they're set up.

## Who can see a repository \#

A **public** repo can be read by anyone, signed in or not — on the web, with `oak clone`, and with stock `git clone`. Only people with write access can push branches or merge.

A **private** repo can be read by:

- the organization's **owners and admins**, and
- people added to that repo as **collaborators**, with either the `reader` or `writer` role.

Note that being a plain **member** of the organization doesn't by itself grant access to its private repos — add members as collaborators on the repos they need. Manage collaborators under the organization's **Settings → Repository access**; invited people get an email.

Change a repo's visibility under the repo's **Settings**, or with `PATCH /api/{owner}/{name}/visibility`.

Inside a repo, [path permissions](https://oak.space/docs/path-permissions) can narrow access to particular directories, or publish parts of a private repo to everyone.

## Groups \#

A **group** is a named list of an organization's members, such as `sec-team` or `contractors`. Groups exist so that [`.oak/PERMISSIONS`](https://oak.space/docs/path-permissions) can grant access to `@group/sec-team` instead of listing people one by one — membership is managed in one place, here, rather than in every repo's file.

Owners and admins manage groups under **Settings → Groups**, or with the [API](https://oak.space/docs/api#organizations).

## Service accounts \#

A **service account** (listed as **Agents** in organization settings) is a non-human identity for automation — a deploy bot, a CI system, a long-running coding agent. It gets its own API keys with explicit scopes, and its actions show under its own name in the audit log. See [API keys and tokens](https://oak.space/docs/api-keys#service-accounts).

## Moving and renaming \#

- **Transfer a repo** to another organization from the repo's **Settings** (or `POST /api/{owner}/{name}/transfer`). Its history, branches, and settings come along.
- **Rename a repo** from its **Settings**.
- **Rename an organization's slug** from the organization's **Settings → General**. URLs that use the old slug stop working, so update remotes and links.

## Audit log \#

Owners and admins can see who did what — membership changes, merges, forced merges past CI, settings changes — at `oak.space/orgs/\<org\>/audit`, or `GET /api/orgs/{slug}/audit`.

## The organization page \#

`oak.space/\<org\>` is the organization's landing page. Under **Settings → Page** you can give it an About section in Markdown, an image, custom CSS, and choose whether it's public. To serve a whole static site from a repo, see [Sites](https://oak.space/docs/sites).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
