+++
title = "docs-configuration"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: The files Oak reads in a repo and in your home directory, and every environment variable.
title: Files and environment variables · Docs
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

# Files and environment variables

Copy page [.md](https://oak.space/docs/configuration.md "This page as raw Markdown")

The files Oak reads in a repo and in your home directory, and every environment variable.

## In a repository \#

| Path | Versioned? | Purpose |
| --- | --- | --- |
| `.oak/` | — | The local repository: its database (`.oak/oak.db`), in-progress merge state, and hooks. Never committed, except the files below. |
| `.oak/workflows/*.yml` | Yes | [CI workflows](https://oak.space/docs/ci-workflows). |
| `.oak/PERMISSIONS` | Yes | [Path permissions](https://oak.space/docs/path-permissions), read from `main`. |
| `.oak/attributes` | Yes | Reserved for per-path attributes. |
| `.oak/hooks/pre-commit`, `post-commit` | No | Local [hooks](https://oak.space/docs/making-changes#hooks). |
| `.oakignore` | Yes | Files Oak should ignore, in `.gitignore` syntax. |
| `.gitignore` | Yes | Used **only** when there's no `.oakignore`. |
| `AGENTS.md`, `CLAUDE.md` | Yes | Instructions for coding agents; `oak init` offers to write them. |
| `.claude/skills/oak-vcs/` | Yes | The agent skill, from `oak skill install`. |
| `.oak-space` | — | Marks the root of an [agent space](https://oak.space/docs/spaces). |

## In your home directory \#

| Path | Purpose |
| --- | --- |
| `~/.local/bin/oak` | The CLI, where the installer puts it. |
| `~/.oak/credentials` | Saved logins, one per server (from `oak login`). |
| `~/.oak/mounts/` | Mount state and caches. Override with `OAK_MOUNTS_ROOT`. |
| `~/.oak/version_check` | When the CLI last checked for an update. |
| `~/.oak/feedback.json` | The contact email `oak feedback` remembers. |
| `~/oaktree/` | The default parent directory for mounts. |
| `~/.claude/skills/oak-vcs/` | The agent skill, from `oak skill install --global`. |

## Environment variables \#

`oak environment` (or `oak env`) prints every variable below with its effective value — secrets as set/unset only.

### Server and credentials \#

| Variable | Effect |
| --- | --- |
| `OAK_REMOTE` | Server URL for this invocation, overriding the checkout's linked server. |
| `OAK_API_KEY` | Token to authenticate with. Beats every stored credential. See [API keys](https://oak.space/docs/api-keys). |
| `OAK_REPO` | `org/repo` to link on a first push — the same as `oak push --repo`. |
| `OAK_SERVE_TOKEN` | Bearer token for `oak serve`. |

### Identity \#

| Variable | Effect |
| --- | --- |
| `OAK_AUTHOR` | Author name on commits, and the prefix of generated branch names. Defaults to your login, then `USER` / `USERNAME`. |
| `OAK_EMAIL` | Contact email for `oak feedback`. |

### Network \#

| Variable | Effect |
| --- | --- |
| `OAK_NO_UPDATE_CHECK` | Turn off the daily check for a new CLI version. |
| `OAK_UPLOAD_CONCURRENCY` | Parallel uploads during push. |
| `OAK_DOWNLOAD_CONCURRENCY` | Parallel downloads during clone and pull (default 32). |
| `OAK_PROBE_TIMEOUT_SECS` | Timeout for server probes, 1–300 (default 20). |
| `OAK_ALLOW_PARTIAL_CLONE` | Recovery only: skip files a broken server can't send instead of failing. |
| `HTTPS_PROXY`, `HTTP_PROXY`, `ALL_PROXY`, `NO_PROXY` | Standard proxy settings (lowercase forms too). |

### Mounts \#

| Variable | Effect |
| --- | --- |
| `OAK_MOUNTS_ROOT` | Where mount state lives (default `~/.oak/mounts`). |

### Output \#

| Variable | Effect |
| --- | --- |
| `OAK_VERBOSE` | Print per-phase timings, like `--verbose`. |
| `OAK_LOG` | Log filter for debugging (for example `OAK_LOG=debug`). |
| `OAK_PROGRESS` | `always`, `never`, or `auto` progress display. |
| `OAK_SPINNER` | Spinner style: `line`, `pulse`, `arc`, `minimal`, `oak`. |
| `NO_COLOR`, `CLICOLOR_FORCE` | Disable or force color. |
| `CI` | When set, progress display is off by default. |

### Tools \#

| Variable | Effect |
| --- | --- |
| `OAK_DIFF_TOOL` | External program to use instead of `oak diff`'s viewer. |
| `PAGER` | Pager for long output. |
| `VISUAL`, `EDITOR` | Editor for `oak feedback` and `oak split` (falls back to `vi`). |

### Set by Oak \#

| Variable | Where |
| --- | --- |
| `OAK_HOOK` | In [hooks](https://oak.space/docs/making-changes#hooks): the event name. |
| `OAK_CI_RUN_ID`, `OAK_CI_COMMIT`, `OAK_CI_BRANCH` | In [CI steps](https://oak.space/docs/ci-workflows#what-a-job-sees). |

### Installer \#

| Variable | Effect |
| --- | --- |
| `INSTALL_DIR` | Where `install.sh` / `install.ps1` put the binary. |
| `OAK_NO_LOGIN` | Skip the installer's login prompt. |

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
