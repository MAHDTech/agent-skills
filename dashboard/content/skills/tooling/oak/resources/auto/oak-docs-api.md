+++
title = "oak-docs-api"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: The JSON API under https://oak.space/api that the CLI is built on.
title: HTTP API reference · Docs
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

# HTTP API reference

Copy page [.md](https://oak.space/docs/api.md "This page as raw Markdown")

The JSON API under https://oak.space/api that the CLI is built on.

oak.space has a JSON API — the same one the `oak` CLI is built on. Use it to script repositories, branches, merges, CI, and organizations, or to wire Oak into your own tools and agents.

## Basics \#

**Base URL:** `https://oak.space/api`. Paths below are relative to it.

**Authentication:** send a token as a bearer header. Personal API keys, service-account keys, and `oak login` sessions all work — see [API keys and tokens](https://oak.space/docs/api-keys).

```bash
curl https://oak.space/api/repos -H "Authorization: Bearer $OAK_API_KEY"
```

Requests without a token can read public resources only.

**Format:** request and response bodies are JSON (`Content-Type: application/json`). `{owner}` is an organization slug (including personal organizations, which are named after their user).

**Errors** use the usual status codes — `400` bad request, `401` not signed in, `403` signed in but not allowed (including a key missing a scope), `404` not found (also returned for things you can't see), `409` conflict, `412` precondition failed (the CI gate), `503` temporarily unavailable — with a body like:

```json
{ "error": "human-readable message" }
```

Some errors add a machine-readable `code` and `retryable: true`.

## Account and keys \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/whoami` | The user (or service account) behind the token. |
| `GET` | `/version` | Server version. |
| `GET` | `/api-keys` | Your personal API keys (prefixes only). |
| `POST` | `/api-keys` | Create a key: `{"name", "scope": "full"\|"read", "repo"?: "owner/name"}`. Returns the full key **once**. |
| `DELETE` | `/api-keys/{id}` | Revoke a key. |

## Repositories \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/repos?sort=name\|updated\|created` | Repositories you can access. |
| `POST` | `/repos` | Create one: `{"name", "description"?, "is_public"?, "organization_slug"?}`. |
| `GET` | `/{owner}/{name}` | A repository: name, visibility, head, and more. |
| `DELETE` | `/{owner}/{name}` | Delete a repository. |
| `PATCH` | `/{owner}/{name}/visibility` | `{"is_public": true\|false}`. |
| `POST` | `/{owner}/{name}/transfer` | Move it to another organization: `{"to_organization": "slug"}`. |
| `GET` | `/{owner}/{name}/size` | Storage the repository uses. |

## Branches \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/{owner}/{name}/branches` | List branches, with descriptions, heads, and status. |
| `POST` | `/{owner}/{name}/branches` | Create one: `{"name", "description"?, "parent_branch"?}`. |
| `GET` | `/{owner}/{name}/branches/{branch}` | One branch. Add `?record_read=1` to record that you've read it; the response includes who else has. |
| `POST` | `/{owner}/{name}/branches/{branch}/merge` | Squash-merge onto the parent. See below. |
| `POST` | `/{owner}/{name}/branches/{branch}/sync` | Merge the parent into the branch. `409` with `code: "branch_moved"` (retryable) if it received new commits meanwhile. |
| `POST` | `/{owner}/{name}/branches/{branch}/close` | Close a branch. Idempotent. |
| `POST` | `/{owner}/{name}/branches/{branch}/reopen` | Reopen a closed branch. |
| `POST` | `/{owner}/{name}/branches/{branch}/rename` | Temporarily disabled — returns `503`. |

Branch descriptions are set by pushing (`oak desc` then `oak push`); there's no separate endpoint.

### Merge responses \#

`POST …/merge` lands one squash commit on `main` whose message is the branch description.

| Status | Meaning |
| --- | --- |
| `200` | Merged. The body has `commit_hash` (the new commit on `main`) and `merge_parent_hash` (the branch tip). |
| `412` | The [CI gate](https://oak.space/docs/merging#the-ci-merge-gate): CI on the branch head is failing or running. Add `?force=1` to override (recorded in the audit log). |
| `409` with `conflict_paths` | The branch conflicts with `main`. Sync, resolve, and push. |
| `409` with `code: "main_moved"` or `"branch_moved"` | `main` or the branch changed during the merge. Nothing was merged; retry. |
| `400` "already closed" | The branch was already merged or closed. |

## Commits and content \#

| Method | Path | Description |
| --- | --- | --- |
| `POST` | `/{owner}/{name}/commits/info` | Metadata for a list of commit hashes. |
| `POST` | `/{owner}/{name}/commits/{hash}/revert` | Land a commit that undoes `{hash}` on its branch. `409` if the affected files have changed since. |
| `GET` | `/{owner}/{name}/tree/{commit}` | The root directory listing at a commit. |
| `GET` | `/{owner}/{name}/tree/{commit}/{path}` | A subdirectory listing. |
| `GET` | `/{owner}/{name}/raw/{commit}/{path}` | A file's bytes at a commit. |

Outside the JSON API, `GET https://oak.space/{owner}/{name}/download` returns a zip of `main`, and `git clone https://oak.space/{owner}/{name}.git` a [Git snapshot](https://oak.space/docs/git#clone-an-oak-repo-with-git).

## CI \#

Reading needs read access to the repo; triggering, cancelling, and secrets need write access.

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/{owner}/{name}/ci/runs?limit=N` | Recent runs (default 50, max 200). |
| `GET` | `/{owner}/{name}/ci/runs/{id}` | One run, with its jobs, steps, exit codes, and logs. |
| `POST` | `/{owner}/{name}/ci/runs` | Run workflows at a branch head: `{"workflow"?: "ci", "branch"?: "main"}`. Returns `{"run_ids": [...]}`. **Without `workflow`, every workflow runs.** |
| `POST` | `/{owner}/{name}/ci/trigger` | Exact-head, idempotent trigger (what `oak ci trigger` uses) — see below. |
| `POST` | `/{owner}/{name}/ci/runs/{id}/cancel` | Cancel a queued or running run. |
| `GET` | `/{owner}/{name}/ci/secrets` | Secret names and update times (never values). |
| `PUT` | `/{owner}/{name}/ci/secrets` | Create or replace one: `{"name", "value"}`. |
| `DELETE` | `/{owner}/{name}/ci/secrets/{secret}` | Delete one. |
| `GET` | `/{owner}/{name}/ci/environments[/{env}]` | [Environments](https://oak.space/docs/ci-environments) with their settings (write access). |
| `PUT` `DELETE` | `/{owner}/{name}/ci/environments/{env}/…` | Configure an environment and its `secrets`, `variables`, `reviewers`, and `runners` (org owners and admins) — see [Environments](https://oak.space/docs/ci-environments#api). |
| `GET` | `/{owner}/{name}/ci/environments/{env}/release-preview` | What a deploy would carry now, and what's in its way. |
| `POST` | `/{owner}/{name}/ci/release-authorizations` | Approve and start one deploy with the preview's exact values. Needs the `release` scope on an API key. |

An organization's [runners](https://oak.space/docs/ci-runners) are managed by its owners and admins:

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/orgs/{slug}/ci/runners` | Registered runners: name, labels, version, `status` (`online`, `active`, or `offline`), last seen. |
| `POST` | `/orgs/{slug}/ci/runners/registration-token` | Generate a new registration token (replacing the old one): `{"token", "instance"}`. The token is only ever returned here. |
| `DELETE` | `/orgs/{slug}/ci/runners/{id}` | Remove a runner; its credentials stop working at once. |

Runners themselves talk to `/api/actions/…` — Gitea's runner protocol — with their own credentials, not API keys.

The exact-head trigger only runs if the branch head is still the commit you name, and replays instead of duplicating when you retry with the same key:

```json
{
  "protocol": "ordinary_trigger_v1",
  "expected_commit": "<full 64-character hash>",
  "idempotency_key": "deploy-check-1",
  "branch": "main",
  "workflow": "ci"
}
```

It returns `{"run_ids", "replayed"}`; `412` if the head moved, `409` if the key was used with a different request, and `503` with `retryable: true` when busy.

Run statuses go `queued` → `running` → `completed`, and a completed run's conclusion is `success`, `failure`, `cancelled`, `timed_out`, or `skipped`.

## Path permissions \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/{owner}/{name}/path-permissions` | The server's parse of `.oak/PERMISSIONS` on `main`: entries, principals, warnings, and whether enforcement is on. |
| `POST` | `/{owner}/{name}/path-permissions/propose` | Propose a new file as a branch: `{"content": "...", "description"?: "..."}`, or `{"delete": true}`. `403` if you may not change it. |

## Organizations \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/orgs` | Your organizations. |
| `POST` | `/orgs` | Create one: `{"slug", "name", "description"?}`. |
| `GET` | `/orgs/{slug}` | One organization. |
| `PATCH` | `/orgs/{slug}` | Update its name or description. |
| `DELETE` | `/orgs/{slug}` | Delete it. |
| `GET` | `/orgs/{slug}/members` | Members and roles. |
| `POST` | `/orgs/{slug}/members` | Add someone: `{"username", "role"?: "member"\|"admin"\|"owner"}`. |
| `PATCH` | `/orgs/{slug}/members/{username}` | Change a role: `{"role"}`. |
| `DELETE` | `/orgs/{slug}/members/{username}` | Remove a member. |
| `GET` | `/orgs/{slug}/groups` | Groups. |
| `POST` | `/orgs/{slug}/groups` | Create a group: `{"slug", "description"?}`. |
| `DELETE` | `/orgs/{slug}/groups/{group}` | Delete a group. |
| `POST` | `/orgs/{slug}/groups/{group}/members` | Add a member: `{"username"}`. |
| `DELETE` | `/orgs/{slug}/groups/{group}/members/{username}` | Remove a member. |
| `GET` | `/orgs/{slug}/storage` | Storage used against the quota. |
| `GET` | `/orgs/{slug}/audit` | The audit log. |

Managing members, groups, and service accounts needs owner or admin.

### Service accounts \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/orgs/{slug}/service-accounts` | List them. |
| `POST` | `/orgs/{slug}/service-accounts` | Create one: `{"name", "display_name"?}`. |
| `PATCH` | `/orgs/{slug}/service-accounts/{name}` | `{"disabled": true\|false}`. |
| `DELETE` | `/orgs/{slug}/service-accounts/{name}` | Delete it. |
| `GET` | `/orgs/{slug}/service-accounts/{name}/keys` | Its keys. |
| `POST` | `/orgs/{slug}/service-accounts/{name}/keys` | Create a key: `{"name", "scopes": "read,write"}`. Returns the key **once**. |
| `DELETE` | `/orgs/{slug}/service-accounts/{name}/keys/{key_id}` | Revoke a key. |

Scopes are explained in [API keys and tokens](https://oak.space/docs/api-keys#service-accounts).

## Webhooks \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/{owner}/{name}/webhooks` | A repo's webhooks. |
| `POST` | `/{owner}/{name}/webhooks` | Create one: `{"url", "secret"?, "events"?: ["push", "branch.created", "merge"]}`. |
| `DELETE` | `/{owner}/{name}/webhooks/{id}` | Delete one. |

Payloads and signatures: [Webhooks](https://oak.space/docs/webhooks).

## Sites \#

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/orgs/{slug}/site` | The organization's site settings. |
| `PUT` | `/orgs/{slug}/site` | Enable or update: `{"repo", "source_dir"?}`. |
| `DELETE` | `/orgs/{slug}/site` | Disable it. |
| `GET` | `/sites` | Sites you can see. |

## CLI releases \#

Public endpoints the installer and `oak upgrade` use to fetch and verify binaries.

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/releases` | Published CLI versions. |
| `GET` | `/releases/latest` | The latest version. |
| `GET` | `/releases/{version}/{platform}` | Download a binary. |
| `GET` | `/releases/{version}/{platform}/sha256` | Its SHA-256. |
| `GET` | `/releases/{version}/{platform}/minisig` | Its minisign signature. |

## Sync protocol \#

Push, pull, clone, and mount use content-addressed endpoints under `/{owner}/{name}/` — `push`, `pull`, `blobs/*`, and `chunks/*`. They're built for the CLI, which verifies every hash, and may change between versions; use the CLI rather than calling them directly.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
