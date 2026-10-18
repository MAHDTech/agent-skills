---
description: Personal API keys for scripts, CI, and agents — and how the CLI picks its credential.
title: API keys and tokens · Docs
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

# API keys and tokens

Copy page [.md](https://oak.space/docs/api-keys.md "This page as raw Markdown")

Personal API keys for scripts, CI, and agents — and how the CLI picks its credential.

Everything that talks to oak.space — the CLI, the [HTTP API](https://oak.space/docs/api), CI jobs, agents — authenticates with a bearer token:

```text
Authorization: Bearer oak_...
```

There are three kinds of token.

| Token | Comes from | Lifetime | Use it for |
| --- | --- | --- | --- |
| **Login session** | `oak login` | 90 days | You, at your own terminal. |
| **Personal API key** | You create it | Until revoked | Scripts and tools acting as you. |
| **Service-account key** | An org admin creates it | Until revoked | Bots, CI systems, and agents with their own identity. |

## Personal API keys \#

Create one under **Settings → API keys** on oak.space, or over the API:

```bash
curl -X POST https://oak.space/api/api-keys \
  -H "Authorization: Bearer $OAK_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "release script", "scope": "read", "repo": "acme/web"}'
```

| Field | Value |
| --- | --- |
| `name` | 1–100 characters, for your own reference. |
| `scope` | `full` (default) or `read` — a read key can only make read requests. |
| `repo` | Optional `owner/name`. The key then works only for that one repo. |

**The full key is shown once**, when it's created. After that only its first characters are displayed, so store it somewhere safe. List your keys with `GET /api/api-keys` and revoke one with `DELETE /api/api-keys/{id}`.

A personal key acts as you, with your access — a repo-scoped or read-only key can only do less, never more.

## Service accounts \#

A **service account** is a non-human identity owned by an organization — a deploy bot, an external CI system, a coding agent that runs unattended. Its actions show up under its own name in the audit log, and its keys carry explicit **scopes**, so you can give it exactly what its job needs.

Organization owners and admins manage them under the organization's **Settings → Agents**, or over the API:

```bash
# Create the account
curl -X POST https://oak.space/api/orgs/acme/service-accounts \
  -H "Authorization: Bearer $OAK_TOKEN" -H "Content-Type: application/json" \
  -d '{"name": "nightly-agent", "display_name": "Nightly refactor agent"}'

# Give it a key that can push branches but never merge
curl -X POST https://oak.space/api/orgs/acme/service-accounts/nightly-agent/keys \
  -H "Authorization: Bearer $OAK_TOKEN" -H "Content-Type: application/json" \
  -d '{"name": "prod", "scopes": "read,write"}'
```

| Scope | Allows |
| --- | --- |
| `read` | Every read request: clone, pull, list, view. |
| `write` | Push branches, create repos and branches, and other changes — except the ones below. |
| `merge` | Merge branches onto `main`. |
| `admin` | Organization management (`/api/orgs/...`), and changing a repo's [environments](https://oak.space/docs/ci-environments). |
| `release` | Deploy to an [environment](https://oak.space/docs/ci-environments) (`oak ci deploy`). Never implied by any other scope, including `admin`. |

Scopes are required on service-account keys. A key can never do more than the person who created it can. A request outside a key's scopes gets a **403** naming the missing scope.

Disable a service account (`PATCH …/service-accounts/{name}` with `{"disabled": true}`) to stop all its keys at once, or delete it. Revoke one key with `DELETE …/service-accounts/{name}/keys/{key_id}`.

> **For coding agents**, a `read,write` service-account key is a good default: the agent can clone, push its branches, and file what it did, but can't merge to `main` — merging stays a human decision.

## Using a key with the CLI \#

```bash
export OAK_API_KEY=oak_...
oak clone acme/web
oak auth status       # confirms which credential is in use and who it resolves to
```

The CLI picks its credential in this order:

1. `OAK_API_KEY`, if set.
2. A key stored in the checkout itself ( `oak clone` saves one, bound to the server it came from).
3. Your `oak login` session for that server, from `~/.oak/credentials`.

If a push or pull unexpectedly says a repo you know exists isn't found, a stale key from step 2 is the usual suspect — `oak auth status` shows which source is winning.

## Using a key with the API \#

```bash
curl https://oak.space/api/repos -H "Authorization: Bearer $OAK_API_KEY"
```

See the [HTTP API reference](https://oak.space/docs/api).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page