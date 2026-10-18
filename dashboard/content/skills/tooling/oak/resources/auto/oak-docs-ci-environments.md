+++
title = "oak-docs-ci-environments"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: "Protect deploy secrets: environments with reviewers, branch rules, workflow pins, and dedicated runners, deployed with `oak ci deploy`."
title: Environments · Docs
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

# Environments

Copy page [.md](https://oak.space/docs/ci-environments.md "This page as raw Markdown")

Protect deploy secrets: environments with reviewers, branch rules, workflow pins, and dedicated runners, deployed with \`oak ci deploy\`.

An **environment** is a named place your CI deploys to — `production`, `staging`, `app-store` — with its own secrets and its own rules about who can deploy, from which branches, and on which machines. It's how you keep a signing key or a deploy credential away from the rest of your CI.

[CI secrets](https://oak.space/docs/ci#secrets) are available to every workflow on every branch, so anyone who can push can read them. An environment's secrets are much harder to reach. A job receives them only when all of these hold:

- the job declares the environment ( `environment: production`),
- someone allowed to deploy approved that exact commit and workflow,
- the commit is the current head of a branch the environment allows,
- and the job runs on a runner dedicated to the environment, which runs nothing else.

Environment workflows never run on a push or a merge. They run when someone deploys them.

## Set one up \#

Environments are configured by the repository's **managers** (organization owners and admins) in the repo's **Settings → Environments**, or over the [API](#api).

1. **Create it.** A new environment is enabled, requires a reviewer, and allows deploying from `main`.
2. **Add its secrets and variables.** Secret values are encrypted and never shown again; variables are plain values anyone with write access can read.
3. **Add reviewers** — the people who may approve a deploy. They need read access to the repo. (Or turn off **Required reviewers** to let anyone with write access deploy.)
4. **Dedicate a runner.** Register a [runner](https://oak.space/docs/ci-runners) for it, ideally on a machine that holds nothing else, and bind it to the environment. A bound runner takes only that environment's jobs — never ordinary CI — so code from an arbitrary branch never runs where the secrets land.
5. **Point a workflow at it.** Give the job `environment:` and a manual trigger:

   ```yaml
   # .oak/workflows/release.yml
   on: [workflow_dispatch]
   jobs:
     release:
       runs-on: release-mac
       environment: production
       steps:
         - uses: actions/checkout@v4
         - run: ./scripts/release.sh
           env:
             SIGNING_KEY: ${{ secrets.SIGNING_KEY }}
   ```

   An environment workflow has exactly one job. `environment:` also takes GitHub's mapping form, `{name: production, url: https://example.com}`. The name must be written out: it can't come from an expression.
6. **Deploy it** (below).

## Deploy \#

```bash
oak ci deploy --environment production            # preview, confirm, deploy main's head
oak ci deploy -e production --workflow release --branch main --yes
oak ci deploy -e production --dry-run             # just show what would deploy
```

A deploy has two steps. First Oak shows what would run: the branch head, the workflow and its exact content hash, the environment's current policy version, its reviewers, and anything in the way. Then you approve exactly those values. If anything moves in between, the approval is refused: the branch gets a new commit, the workflow changes, or someone edits the environment. You re-review the new state; nothing runs that you didn't see.

An approval starts one run and is good for 30 minutes. The first runner bound to the environment that polls takes the job. Just before it hands the job over, Oak checks every rule again: environment enabled, same policy, head unchanged, approver still allowed, runner still bound. If anything failed, the run fails and nothing is sent.

The same deploy is available from **Settings → Environments → Deploy**, and over the API as a preview (`GET …/release-preview`) followed by an authorization (`POST …/ci/release-authorizations`) carrying the preview's values. An API key used to deploy must carry the explicit [`release` scope](https://oak.space/docs/api-keys).

## Protection rules \#

| Rule | Default | What it does |
| --- | --- | --- |
| **Enabled** | on | A disabled environment deploys nothing. |
| **Branches** | `main` | Only the current head of one of these branches can be deployed. |
| **Required reviewers** | on | Only listed reviewers can approve a deploy. Off: anyone with write access can. Removing the last reviewer doesn't open it up; with this on and nobody listed, nobody can deploy. |
| **Workflow pins** | none | With no pins, any workflow whose job declares the environment can deploy. Add pins to allow only those workflow files. A pin can also fix the file's exact content hash, so even a merged change to the workflow can't deploy until someone re-pins it. |
| **Runners** | none | The runners that take this environment's jobs. At least one must be online to approve a deploy. |

Every change to an environment moves its **policy version**, which cancels any approval made against the old settings.

Platform administrators of the Oak instance can also approve deploys.

## What the job sees \#

- **Secrets:** the environment's secrets, through `${{ secrets.NAME }}` as usual, plus the job's own `GITHUB_TOKEN`. Your repo's ordinary CI secrets are *not* included, so an environment job only ever has what you put in the environment.
- **Variables:** as environment variables, and as `${{ vars.NAME }}`.
- **Who and what:** besides the usual `OAK_CI_RUN_ID`, `OAK_CI_COMMIT`, and `OAK_CI_BRANCH`, the job gets `OAK_CI_ENVIRONMENT_NAME`, `OAK_CI_POLICY_VERSION`, `OAK_CI_AUTHORIZATION_ID`, `OAK_CI_WORKFLOW_PATH`, and `OAK_CI_WORKFLOW_HASH`, so a deploy script can record exactly what was approved.

Two rules keep secrets from leaking sideways. A secret and a variable in one environment can't share a name. And an ordinary repo secret with the same name as any environment secret is withheld from ordinary jobs, so deleting a key from your CI secrets and adding it to an environment really moves it. Delete the ordinary copy.

Each line of a secret must be at least 4 characters, so the runner can mask it in logs.

## Who can do what \#

|  | Reader | Writer | Owner / admin |
| --- | --- | --- | --- |
| See an environment's settings, variables, and secret names |  | ✓ | ✓ |
| Create, change, or delete an environment, its secrets, reviewers, and runners |  |  | ✓ |
| Deploy | if a reviewer | if a reviewer, or reviewers aren't required | if a reviewer, or reviewers aren't required |

An API key that changes an environment needs the `admin` scope. One that deploys needs `release`.

## Differences from GitHub \#

| GitHub Actions | On Oak |
| --- | --- |
| A job with `environment:` runs on push and waits for approval | It runs only when someone deploys it, at a commit they approved |
| Any runner with matching labels | Only runners bound to the environment, which take no other jobs |
| Environment secrets plus repo secrets | Environment secrets only |
| Deployment branch rules, wait timers, custom rules | Branch list, required reviewers, workflow pins; no wait timers yet |
| A workflow can mix environment and ordinary jobs | An environment workflow has exactly one job |
| `environment:` from an expression | A fixed name |

## API \#

All under `/api/{owner}/{name}/ci/`, authenticated like the rest of the [API](https://oak.space/docs/api).

| Method and path | Who | Does |
| --- | --- | --- |
| `GET environments` | write access | List environments with their settings, variables, secret names, reviewers, and runners. |
| `GET environments/{env}` | write access | One environment. |
| `PUT environments/{env}` | owner/admin | Create it, or change some settings: `{"enabled", "required_reviewers", "branches": [...], "workflows": [{"path", "hash"}]}`. Fields you leave out stay as they are. `201` when created. |
| `DELETE environments/{env}` | owner/admin | Delete it. Its runners go back to taking ordinary CI. |
| `PUT` / `DELETE environments/{env}/secrets/{NAME}` | owner/admin | Set (`{"value": "..."}`) or delete a secret. |
| `PUT` / `DELETE environments/{env}/variables/{NAME}` | owner/admin | Set (`{"value": "..."}`) or delete a variable. |
| `PUT` / `DELETE environments/{env}/reviewers/{username}` | owner/admin | Add or remove a reviewer. |
| `PUT` / `DELETE environments/{env}/runners/{runner-uuid}` | owner/admin | Bind or unbind one of the organization's runners. |
| `GET environments/{env}/release-preview?workflow=&branch=` | read access | What a deploy would carry right now, whether you may approve it, and what's in the way. |
| `POST release-authorizations` | per the environment | Approve and start one deploy: `{"environment", "branch", "commit", "workflow_path", "workflow_hash", "event": "manual", "policy_version"}`, copied from the preview. Returns `{"authorization_id", "run_id"}`. |

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
