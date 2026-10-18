---
description: How to set up Claude Code, Codex, Cursor and friends to work on Oak — and the rules they should follow.
title: Working with agents · Docs
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

# Working with agents

Copy page [.md](https://oak.space/docs/agents.md "This page as raw Markdown")

How to set up Claude Code, Codex, Cursor and friends to work on Oak — and the rules they should follow.

Oak doesn't run agents — you bring your own: Claude Code, Codex, Cursor, Aider, a script of your own. What Oak gives them is a version-control model that fits how they work, and a CLI built to be driven by a program as well as a person.

## The model \#

- **One branch per session.** Start each agent run on its own branch ( `oak switch -c`, or a fresh [mount](https://oak.space/docs/mounts)). A session you don't like gets closed, and `main` never notices.
- **The agent writes the description.** Commits carry no message, so the agent's summary of what it did goes in `oak desc` — and becomes the commit message on `main`.
- **The agent publishes; a person merges.** The agent ends by pushing its branch. You [review](https://oak.space/docs/reviewing) and [merge](https://oak.space/docs/merging).
- **Mount instead of clone.** For a big repo, or many agents at once, give each agent its own mount. See [Example: a fleet of agents](https://oak.space/docs/example-session).

## Install the agent skill \#

```bash
oak skill install            # writes .claude/skills/oak-vcs/ in this repo — commit it
oak skill install --global   # or ~/.claude/skills/oak-vcs/ for every project on this machine
```

The `oak-vcs` skill is a `SKILL.md` plus reference files in the open Agent Skills format. It teaches an agent to recognize an Oak repo (a `.oak/` directory and no `.git`), to use `oak` instead of `git`, and the whole workflow on this site — including the traps, like interactive commands that hang without a terminal. Committed to the repo, every collaborator's agent picks it up.

The files are baked into the CLI binary, so the skill always matches the CLI version that wrote it. Re-run `oak skill install` after `oak upgrade`.

For agents that read `AGENTS.md` rather than skills (Codex, Cursor, and others), `oak init` offers to write an `AGENTS.md` explaining Oak, plus a one-line `CLAUDE.md` that points at it.

## Rules to give an agent \#

If you'd rather put it in your own prompt or `AGENTS.md`, this is the core:

```text
This repo uses Oak (the `oak` CLI), not git. Never run git commands here.
- Work on the current feature branch. main lives on the server; never try to push to it.
- `oak commit` takes no message. Describe the branch once with `oak desc "..."`;
  it becomes the commit message on main, so write it for a reviewer.
- Finish every task with: oak desc "...", oak commit, oak push
  (or `oak finish --desc-file <file> --json`).
- Do not run `oak merge`. A human merges.
- Prefer --json output. Commands that open a UI or prompt (bare `oak diff`,
  `oak switch` with no name, `oak clone` with no repo, `oak split` without --plan)
  hang without a terminal — use `oak diff --print`/`--stat`/`--json` and pass names.
- `oak reset` / `oak restore` ask for confirmation; pass -f only after
  `oak status` shows what you'd discard.
- First push of a brand-new repo: `oak push --repo <org>/<name>`. Never guess the
  org — ask if you don't know it.
- Lost? Run `oak agent state --json --compact`.
```

## Machine-readable everything \#

Every command an agent needs takes `--json`, and the results are designed to be acted on:

| Command | Use |
| --- | --- |
| `oak agent state --json --compact` | One document: repo, branch, what's dirty, what's unpushed, and `recommended_next_commands`. The "where am I and what now?" call. |
| `oak status --json --compact` | Bounded working-tree status. |
| `oak diff --json --hunks --max-bytes N` | The diff, with a byte budget. |
| `oak commit --json`, `oak push --json`, `oak pull --json` | Receipts with what happened and what to do next. |
| `oak finish --desc-file \<file\> --json` | Describe, checkpoint, and publish in one step — see below. |
| `oak conflict status`, `oak conflict show --json` | Conflict state, file by file. |
| `oak ci status --json` | The merge gate's verdict (also via exit code). |
| `oak branch triage --remote --json` | Score every open branch, for an agent doing review. |

Many JSON payloads include `recommended_next_commands`: exact invocations for the natural next step. An agent should run one of those rather than improvise flags.

Exit codes are stable and meaningful — `3` repository locked (retry), `4` dirty working tree, `5` conflicts, `6` network/server/auth — so a wrapper can branch without parsing text. The table is in the [CLI reference](https://oak.space/docs/cli#exit-codes).

## Finishing a task in one command \#

```bash
oak finish --desc-file /tmp/desc.md --json
```

`oak finish` checks it can publish (is a remote linked? is anything blocking?) **before** changing anything, then sets the branch description, checkpoints uncommitted changes, and pushes. Inside a mount it also unmounts — but only once the push has succeeded. If any step fails, nothing is lost: the JSON names the step and the command to run next.

## Credentials for unattended agents \#

`oak login` needs a browser once. An agent on a server, in CI, or in a container should use an API key instead:

```bash
export OAK_API_KEY=oak_...
```

For an agent, prefer a key that can do no more than its job — a **service account** with only `read` and `write` scopes (so it can push but never merge), or a key scoped to a single repo. See [API keys and tokens](https://oak.space/docs/api-keys).

## Running many agents at once \#

Give each agent its own branch *and* its own working tree, so they can't step on each other. With mounts that costs seconds, not a clone per agent:

```bash
oak space new acme && cd acme
oak mount acme/web ./fix-login/web
oak mount acme/web ./add-search/web
claude -p "Task fix-login, mounted at ./fix-login/web ..." &
claude -p "Task add-search, mounted at ./add-search/web ..." &
wait
oak branch triage --remote
```

The [example session](https://oak.space/docs/example-session) walks through this end to end, and [Agent spaces](https://oak.space/docs/spaces) covers the scaffolding.

## Feedback from agents \#

Agents hit rough edges first. `oak feedback` files a bug or feature request from the terminal and returns a tracking reference (`fb-N`):

```bash
oak feedback -m "oak pull --json omits parent_head_after on fast-forward" --json
```

Without `-m` or `--file` it opens your editor on a terminal, and exits with an error (rather than hanging) when there's no terminal.

## Privacy \#

Oak makes no AI calls on your behalf and doesn't train on your code. Whatever agent you use is its own integration, with its own data policy.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page