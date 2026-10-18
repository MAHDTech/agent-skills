+++
title = "oak-docs-ci"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: "Oak runs CI on your own runners: GitHub Actions workflows, triggers, secrets, and the merge gate."
title: Continuous integration · Docs
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

# Continuous integration

Copy page [.md](https://oak.space/docs/ci.md "This page as raw Markdown")

Oak runs CI on your own runners: GitHub Actions workflows, triggers, secrets, and the merge gate.

Oak runs CI on **your own machines**. Register a computer as a runner for your organization, put a GitHub Actions–style workflow file in the repo, and Oak hands its jobs to your runners when branches are pushed and merged, shows the result on every branch, and refuses to merge a branch whose CI is red.

Runners are the stock open-source [Gitea runner](https://gitea.com/gitea/act_runner) (`gitea-runner`), which speaks Oak's runner API. Oak doesn't host CI compute: a job runs only when one of your runners picks it up.

## Your first workflow \#

1. **Register a runner.** In your organization's **Settings → Runners**, generate a registration token, then on the machine:

   ```bash
   gitea-runner register --no-interactive --instance https://oak.space \
     --token <registration-token> --name my-laptop --labels linux:docker://node:20-bookworm
   gitea-runner daemon
   ```

   The full setup — labels, Docker vs. host mode, running it as a service — is in [Self-hosted runners](https://oak.space/docs/ci-runners).
2. **Add a workflow.** Create `.oak/workflows/ci.yml`:

   ```yaml
   on: [push]
   jobs:
     test:
       runs-on: linux
       steps:
         - uses: actions/checkout@v4
         - run: cargo test --locked
   ```

   `runs-on: linux` sends the job to a runner carrying the `linux` label.
3. **Push it:**

   ```bash
   oak commit && oak push
   ```

That push starts a run. Watch it from the CLI, or on the branch's page on oak.space, where a status chip links to live logs:

```bash
oak ci status          # the verdict for your branch head
oak ci runs            # recent runs in this repo
oak ci logs <run-id>   # a run's step-by-step output
```

## When workflows run \#

Oak maps its own events onto the GitHub events a workflow lists in `on:`:

| Oak event | GitHub event a workflow sees | Runs against |
| --- | --- | --- |
| A branch is pushed (its first push too) | `push`, or `pull_request` for workflows that only list that | The branch head |
| A branch is squash-merged onto `main` | `push` to `main` | The new commit on `main` |
| You trigger it — see [Running a workflow by hand](#running-a-workflow-by-hand) | `workflow_dispatch` | The head of the branch you name |

`branches` / `branches-ignore` and `paths` / `paths-ignore` filters work as on GitHub. Events with no Oak equivalent (`schedule`, `release`, `issues`, …) are accepted and never fire.

Workflow files are read **from the commit being tested**, so a change to a workflow takes effect on the push that makes it. Every file in `.oak/workflows/` whose `on:` matches starts its own run. The full format — and what Oak does differently from GitHub — is in the [Workflow file reference](https://oak.space/docs/ci-workflows).

## What a run looks like \#

A run is one job per job in the workflow (one per matrix combination). A job waits for the jobs it `needs:` and for a runner whose labels include **every** label in its `runs-on`. On the runner, `actions/checkout` fetches the exact commit under test; steps, `uses:` actions, expressions, `services:` and `container:` all run the way they do on GitHub, because the runner executes them.

A job whose dependency failed is skipped, unless it has its own `if:` (such as `if: always()`). The run fails if any job fails.

## Secrets \#

Values that mustn't live in the repo — deploy keys, API tokens — go in **CI secrets**, set per repo under **Settings → CI** on oak.space (you need write access).

- Reference them as `${{ secrets.NAME }}`, exactly as on GitHub. The runner masks their values in logs.
- `${{ secrets.GITHUB_TOKEN }}` is a per-job token that lets that job fetch its repo's source, and stops working when the job ends.
- Names are letters, digits, and `_`, can't start with a digit, and can't start with `OAK_CI_`. Values can be up to 64 KiB, and multi-line values are passed through exactly.
- Once saved, a secret's value can't be read back — only replaced or deleted.

There's no per-workflow scoping: a secret is available to every workflow in the repo, on every branch. Anyone who can push a branch can write a workflow that reads it — and runs code on your runners — so treat repo write access as access to its secrets and its runners. For credentials that must not be reachable that way — deploy keys, signing certificates — use an [environment](https://oak.space/docs/ci-environments): its secrets reach only an approved deploy, on runners dedicated to it.

Secrets can also be managed over the API (`GET`/`PUT`/`DELETE /api/{owner}/{name}/ci/secrets`) — see [the API reference](https://oak.space/docs/api#ci).

## The merge gate \#

CI decides whether a branch can land. A merge onto `main` is **refused while the branch head's CI is failing or still running** — the latest run of every workflow for the head commit must have succeeded. A head with no runs isn't gated.

```bash
oak ci status          # exit 0 = passed, 1 = failed (or no runs), 3 = still running
oak merge --wait       # wait for CI to finish, then merge if it passed
oak merge --force      # override, deliberately, after reading the failure
```

A job no runner can take blocks the gate until it's picked up — register a runner with its labels, or cancel the run. Oak cancels jobs nobody picked up within 24 hours. The gate applies to every way of merging — CLI, web, batch merge, and API (where it's HTTP 412, overridden with `?force=1`). Forced merges are written to the organization's audit log. More in [Merging and conflicts](https://oak.space/docs/merging#the-ci-merge-gate).

## Watching and managing runs \#

**In the browser:** there's no separate CI page. Each branch row on the **Branches** page wears a status chip for its head, each branch and commit page shows a pill per workflow with its duration, and every chip opens the run's page — each job with the runner it ran on (or the labels it's waiting for), and logs per step that stream live while it runs.

**From the CLI:**

```bash
oak ci runs --limit 50                    # recent runs: id, workflow, branch, commit, status, duration
oak ci status --json                      # gate verdict for the current head
oak ci status --run 1204                  # one specific run
oak ci wait --current --timeout 1800      # block until the current head's runs finish
oak ci wait 1204 1205 --progress          # …or specific runs, printing step transitions
oak ci logs 1204 --failed                 # only the failing steps' output
oak ci logs 1204 --summary                # step metadata only, no log output
oak ci rerun 1204                         # re-run at the same commit (for infrastructure flakes)
oak ci cancel 1204 --commit <full-hash>   # cancel a push or merge run
oak ci cancel --superseded --yes          # cancel this branch's in-flight runs for commits that are no longer its head
```

`oak ci rerun` is for failures that weren't the code's fault — a network blip, a runner that went away. A code fix needs a new commit. Cancelling stops a running job within a few seconds: the runner learns from its next report.

## Running a workflow by hand \#

```bash
oak ci trigger --workflow ci --expected-commit <full-hash> --idempotency-key retry-1
```

`oak ci trigger` starts a run at your branch's head — but only if the head is still exactly `--expected-commit`, so you never test something you didn't mean to. Re-sending the same `--idempotency-key` returns the original run instead of starting another.

Over the API:

```bash
curl -X POST https://oak.space/api/acme/web/ci/runs \
  -H "Authorization: Bearer $OAK_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"workflow": "ci", "branch": "main"}'
```

> **Always name the workflow.** A manual trigger with no `workflow` runs **every** workflow at that head, whether or not it lists `workflow_dispatch` in `on:`. Workflows that deploy to an [environment](https://oak.space/docs/ci-environments) are the exception: they never start this way, only with `oak ci deploy`.

## Limits \#

CI runs on your machines, so there are no CI minutes and no concurrency caps — a runner takes one job at a time per unit of its `capacity`, and more runners run more jobs at once. A whole run is timed out after **6 hours**; set `timeout-minutes` on a job for a tighter bound (the runner enforces it). A job whose runner stops reporting for 10 minutes fails, and a job no runner picks up within 24 hours is cancelled.

## Bring your own CI \#

To use an external CI system instead, have it listen to Oak's [webhooks](https://oak.space/docs/webhooks) (`push`, `branch.created`, `merge`), then fetch the code with `oak clone --branch \<name\>` using an [API key](https://oak.space/docs/api-keys). External systems can't yet report status back into Oak's merge gate.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
