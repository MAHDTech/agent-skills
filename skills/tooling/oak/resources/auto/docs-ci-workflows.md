---
description: What Oak reads from a GitHub Actions workflow in `.oak/workflows/`, and where it differs from GitHub.
title: Workflow file reference · Docs
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

# Workflow file reference

Copy page [.md](https://oak.space/docs/ci-workflows.md "This page as raw Markdown")

What Oak reads from a GitHub Actions workflow in \`.oak/workflows/\`, and where it differs from GitHub.

Workflows live in `.oak/workflows/` and use **GitHub Actions syntax**. Oak reads each file to decide *which jobs run and where*; your [runner](https://oak.space/docs/ci-runners) executes them, so steps, `uses:` actions, expressions, `services:`, `container:` and the rest behave as they do on GitHub. This page covers what Oak itself reads, and where it differs from GitHub.

A workflow that fails to parse still produces a run — one that fails immediately and shows the error — so a broken workflow can't silently stop testing anything.

## Files \#

- **Location:** directly in `.oak/workflows/`, ending in `.yml` or `.yaml`. Subdirectories are ignored. ( `.github/workflows/` is not read; copy or move files you want Oak to run.)
- **Name:** the file name without its extension. `.oak/workflows/ci.yml` is the workflow `ci` — that's the name `oak ci` and the API use.
- **Size:** at most 256 KiB.
- **Independence:** each file that matches an event starts its own run, and the [merge gate](https://oak.space/docs/ci#the-merge-gate) checks each one.

## A complete example \#

```yaml
# .oak/workflows/ci.yml
name: CI
on:
  push:
    paths-ignore: ['docs/**']
  workflow_dispatch:

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [linux, macos]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: cargo-${{ hashFiles('Cargo.lock') }}
      - run: cargo test --locked

  deploy-docs:
    needs: test
    runs-on: linux
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/publish-docs.sh
        env:
          DOCS_TOKEN: ${{ secrets.DOCS_TOKEN }}
```

## on \#

Which events start the workflow. As on GitHub, `on:` is an event name, a list, or a map of events to filters:

| You write | Runs when |
| --- | --- |
| `push` | Any branch is pushed — its first push too — and when a branch is merged onto `main` (a push to `main`). |
| `pull_request` | A branch other than `main` is pushed. An Oak branch is its own pull request against `main`, so `branches:` here filters the *base* branch. A workflow listing both `push` and `pull_request` runs once, as `push`. |
| `workflow_dispatch` | Someone triggers it with `oak ci trigger` or the API. |

`branches`, `branches-ignore`, `paths`, and `paths-ignore` work as on GitHub: `*` matches within one path segment, `**` crosses directories, a `!` pattern excludes, and the last matching pattern wins. Two Oak-specific rules keep the merge gate honest:

- On a feature branch, "changed" means **the whole branch** — everything that differs from where it left `main` — not just the latest push. Otherwise a docs-only push after a broken code push would leave the head untested, and ungated.
- A change to the workflow file itself always runs it.

Other events (`schedule`, `release`, `issues`, `workflow_call`, …) are accepted, so a file shared with GitHub keeps working, but never fire on Oak. Oak's original event names still work too: `branch.created` (pushes to branches other than `main`), `merge` (merges onto `main`), and `manual` (`workflow_dispatch`).

## jobs \#

Oak reads these job keys; everything else (`env`, `container`, `services`, `timeout-minutes`, `outputs`, `continue-on-error`, `defaults`, …) passes through to the runner unchanged.

| Key | Required | What Oak does with it |
| --- | --- | --- |
| `runs-on` | yes | The labels a runner must **all** carry to take the job. A string, a list, or `{labels: [...]}`. May use `${{ matrix.* }}` values, and nothing else. |
| `steps` | yes | Listed on the run page before the runner reports on them. Each step needs `run` or `uses`. |
| `needs` | no | The job waits until these jobs finish. Must name jobs in this file. |
| `if` | no | Evaluated by the runner. Without one, a job whose dependency didn't succeed is skipped; with one (`always()`, `failure()`, …) the runner decides. |
| `environment` | no | The [environment](https://oak.space/docs/ci-environments) the job deploys to: a fixed name, or `{name, url}`. A workflow with such a job never runs on a push or an ordinary trigger, only when someone deploys it with `oak ci deploy`, and must have exactly one job. |
| `strategy.matrix` | no | Expanded into one job per combination, `include` / `exclude` as on GitHub. The matrix must be written out — one computed by an expression (`fromJSON(...)`) isn't supported. |
| `name` | no | Display name; `${{ matrix.* }}` values are filled in. |

Not supported: reusable workflows (`jobs.<id>.uses`), and runner groups in `runs-on`. Either is an error.

## What a job sees \#

The runner gives every job a GitHub-shaped environment:

- **`github` context** — `github.repository` is `owner/name`, `github.ref` is `refs/heads/<branch>`, `github.event_name` is the event above, and `github.server_url` is oak.space.
- **`github.sha` is a git commit id.** Oak's commit ids are 64 hex characters, which git tooling doesn't accept, so each run's commit is presented as a single-commit git snapshot and `github.sha` names that. Oak's own id is in `$OAK_CI_COMMIT`.
- **Checkout history is one commit deep.** `actions/checkout` works, but `fetch-depth: 0`, tags, and `git describe` find a single commit with no history.
- **`OAK_CI_*` variables** in every step: `OAK_CI_RUN_ID`, `OAK_CI_COMMIT` (the Oak commit), and `OAK_CI_BRANCH`. Names starting with `OAK_CI_` are reserved in every `env:`.
- **`secrets.GITHUB_TOKEN`** authenticates fetching this repo's source, for this job only. It can't call any other API.
- **`uses:` actions** are fetched from GitHub by the runner. JavaScript actions need Node (or Bun) on host-mode runners — see [Self-hosted runners](https://oak.space/docs/ci-runners#javascript-actions).
- **Caching:** `actions/cache` works against the runner's own built-in cache server, so a cache is local to the machine that saved it.
- **Artifacts:** `actions/upload-artifact` and `download-artifact` aren't supported yet. Pass small values between jobs with job `outputs` and `needs.\<job\>.outputs`.

## Differences from GitHub, in one place \#

| GitHub Actions | On Oak |
| --- | --- |
| `.github/workflows/` | `.oak/workflows/` |
| GitHub-hosted runners (`ubuntu-latest`) | Your runners; `runs-on` names their labels |
| `github.sha` is the pushed commit | A single-commit snapshot of the pushed commit; Oak's id is `$OAK_CI_COMMIT` |
| Full history on checkout | One commit |
| `pull_request` from forks | Oak branches are pull requests; there are no forks |
| `schedule`, `release`, … | Accepted, never fire |
| Reusable workflows, computed matrices | Not supported |
| Artifacts | Not supported yet |
| `environment:` jobs run on push and wait for approval | They run only when deployed, on runners dedicated to the environment — see [Environments](https://oak.space/docs/ci-environments) |
| `GITHUB_TOKEN` calls the GitHub API | It fetches this repo's source only |

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page