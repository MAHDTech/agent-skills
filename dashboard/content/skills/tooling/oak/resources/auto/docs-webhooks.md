+++
title = "docs-webhooks"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Get an HTTP callback on push, branch creation, and merge — signed, or posted to Discord.
title: Webhooks · Docs
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

# Webhooks

Copy page [.md](https://oak.space/docs/webhooks.md "This page as raw Markdown")

Get an HTTP callback on push, branch creation, and merge — signed, or posted to Discord.

A webhook makes oak.space send an HTTP `POST` to a URL of yours when something happens in a repo — to notify a chat channel, kick off an external build, or update a dashboard.

## Events \#

| Event | Fires when |
| --- | --- |
| `branch.created` | A branch is pushed for the first time. |
| `push` | An existing branch gets new commits (including a revert). |
| `merge` | A branch is squash-merged onto `main`. |

These are the same events [CI workflows](https://oak.space/docs/ci-workflows#on) trigger on.

## Add a webhook \#

Under the repo's **Settings → Webhooks** (you need write access), enter:

- the **URL** to deliver to — it must resolve to a public address;
- a **secret**, to sign deliveries (recommended);
- which **events** to send. Choosing none means all of them.

Or over the API:

```bash
curl -X POST https://oak.space/api/acme/web/webhooks \
  -H "Authorization: Bearer $OAK_API_KEY" -H "Content-Type: application/json" \
  -d '{"url": "https://ci.example.com/oak", "secret": "s3cret", "events": ["push", "branch.created"]}'
```

List them with `GET /api/{owner}/{name}/webhooks` and remove one with `DELETE /api/{owner}/{name}/webhooks/{id}`.

## Payload \#

Each delivery is a JSON `POST`:

```http
POST /oak HTTP/1.1
Content-Type: application/json
User-Agent: oak-webhooks/1
X-Oak-Event: push
X-Oak-Delivery-Id: 7f3c…
X-Oak-Signature-256: sha256=5d61…

{
  "event": "push",
  "owner": "acme",
  "repo": "web",
  "branch": "fix-login-redirect",
  "commit": "8fcf5aed1b2c…",
  "pusher": "zdgeier",
  "timestamp": "2026-10-06T17:04:11.512Z"
}
```

| Field | Value |
| --- | --- |
| `event` | `push`, `branch.created`, or `merge`. |
| `owner`, `repo` | The repository. |
| `branch` | The branch the event is about. |
| `commit` | The resulting commit — for `merge`, the new squash commit on `main`. |
| `pusher` | Who pushed or merged. |
| `timestamp` | When the event happened (RFC 3339, UTC). |

Fields that don't apply are left out rather than sent as `null`. `X-Oak-Delivery-Id`, when present, is unique per delivery — use it to ignore duplicates.

## Verify the signature \#

When a webhook has a secret, `X-Oak-Signature-256` is `sha256=` followed by the hex HMAC-SHA256 of the raw request body, keyed with the secret. Compute it over the bytes exactly as received, before parsing the JSON, and compare in constant time:

```python
import hmac, hashlib

def verify(secret: str, body: bytes, header: str) -> bool:
    expected = "sha256=" + hmac.new(secret.encode(), body, hashlib.sha256).hexdigest()
    return hmac.compare_digest(expected, header or "")
```

```js
import { createHmac, timingSafeEqual } from "node:crypto";

function verify(secret, rawBody, header = "") {
  const expected = "sha256=" + createHmac("sha256", secret).update(rawBody).digest("hex");
  return expected.length === header.length && timingSafeEqual(Buffer.from(expected), Buffer.from(header));
}
```

## Discord \#

Paste a Discord channel webhook URL (`https://discord.com/api/webhooks/…`) and Oak recognizes it: instead of the JSON payload, it posts a readable message to the channel. No secret is needed, and no signature is sent.

## Delivery \#

- Deliveries are best-effort, sent shortly after the event.
- Oak checks that the URL resolves to a public address both when you add it and again before every delivery, and skips it otherwise — so a webhook can't be pointed at internal networks.
- Respond quickly with any `2xx`; do slow work after responding.

## Using webhooks for external CI \#

A webhook is how you'd drive an external CI system: listen for `push` and `branch.created`, then fetch that commit with an [API key](https://oak.space/docs/api-keys) — `oak clone acme/web --branch <branch>`. External systems can't yet report their status into Oak's [merge gate](https://oak.space/docs/merging#the-ci-merge-gate); for that, use [Oak's native CI](https://oak.space/docs/ci).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
