+++
title = "oak-docs-cli"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Every oak command and flag, exit codes, and global options.
title: CLI reference · Docs
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

# CLI reference

Copy page [.md](https://oak.space/docs/cli.md "This page as raw Markdown")

Every oak command and flag, exit codes, and global options.

Every public `oak` command, grouped by what it's for. Run `oak \<command\> --help` for the same information in your terminal; `oak` with no arguments prints a grouped overview.

## Global options \#

| Option | Effect |
| --- | --- |
| `--verbose` | Print per-phase timings. Goes **before** the command: `oak --verbose push`. Same as `OAK_VERBOSE=1`. |
| `-h`, `--help` | Help for any command or subcommand. |
| `--version` | Print the CLI version. |

Commands that talk to a server take `-r, --remote \<URL\>` (default `https://oak.space`). The server used is: the flag, then `OAK_REMOTE`, then the one the checkout is linked to, then the default.

Most commands take `--json` for machine-readable output. Environment variables are listed in [Files and environment variables](https://oak.space/docs/configuration).

## Exit codes \#

| Code | Meaning |
| --- | --- |
| `0` | Success. |
| `1` | General error. |
| `2` | Usage error (bad flags or arguments, or an interactive prompt with no terminal). |
| `3` | Repository locked by another `oak` process — retry. (`oak ci status`: still running.) |
| `4` | The working tree has uncommitted changes that block the operation. |
| `5` | Merge or sync conflicts, or a conflict is already in progress. |
| `6` | Network, server, or authentication failure. |
| `7` | Merge prediction couldn't be certified. |
| `8` | An integrity check ran out of budget — inconclusive. |

## Setup and sign-in \#

### oak init \#

```text
oak init [PATH]
```

Create a repository in `PATH` (default: the current directory) and put you on a personal branch parented onto `main`. On a terminal it offers to import an existing `.git` history, to write an `.oakignore` for a detected Godot / Unity / Unreal project, and to write an `AGENTS.md` for coding agents. Refuses to run in your home directory.

### oak clone \#

```text
oak clone [ORG/REPO | GIT-URL] [DEST]
```

Clone a repository and put you on a personal branch. With no argument, pick from a list. A bare `REPO` means your personal organization. A Git URL is imported instead — see [Git import](https://oak.space/docs/git#import-with-oak-clone).

| Flag | Effect |
| --- | --- |
| `--branch \<NAME\>` | Switch to this remote branch after cloning. |
| `--expected-head \<HASH\>` | With `--branch`: fail unless the branch is open at exactly this commit. |
| `--shallow` | Only the latest commit on `main`. |
| `--path \<PREFIX\>` | [Sparse clone](https://oak.space/docs/sparse-clones): only these paths. Repeatable or comma-separated. |
| `--detached` | Check out `main`'s head with no personal branch. |
| `--from \<CHECKOUT\>` | Reuse content from another local checkout of the same repo. |
| `--json` | Print a machine-readable receipt. |
| `--allow-unverified-integrity` | Continue when the server's integrity proof is inconclusive (otherwise exit 8). |
| `-r, --remote \<URL\>` | Server to clone from. |

### oak login / logout / whoami \#

```text
oak login  [-r URL]
oak logout [-r URL]
oak whoami [-r URL]
```

`oak login` signs in through your browser (or prints a URL and asks for a one-time code when there's no browser) and saves a token in `~/.oak/credentials`. `whoami` prints the signed-in username. See [Install and sign in](https://oak.space/docs/install#sign-in).

### oak auth status \#

```text
oak auth status [--json] [-r URL]
```

Show which credential the CLI would use — `OAK_API_KEY`, the checkout's key, or your login — and who the server says it belongs to. Never prints the secret.

### oak repo list \#

```text
oak repo list [--org ORG] [--limit N] [--json] [-r URL]
```

List repositories you can see (public ones only when signed out). `--limit` defaults to 200.

### oak skill install \#

```text
oak skill install [--global]
```

Install the bundled `oak-vcs` agent skill into `./.claude/skills/` (or `~/.claude/skills/` with `--global`). Re-run after upgrading. See [Working with agents](https://oak.space/docs/agents#install-the-agent-skill).

### oak upgrade \#

```text
oak upgrade [-f] [--canary]
```

Upgrade to the latest signed release. `-f` skips the confirmation; `--canary` follows the pre-release channel.

### oak completions \#

```text
oak completions <bash | zsh | fish | elvish | powershell>
```

Print a shell-completion script.

## Working tree \#

### oak status \#

```text
oak status [--json [--compact]] [--short | --porcelain] [--reconcile]
```

Show modified, added, and deleted files. `--short` / `--porcelain` print Git-compatible short rows; `--json --compact` is bounded output for agents; `--reconcile` applies any pending reconciliation after a remote merge first.

### oak diff \#

```text
oak diff [REV [REV]] [PATHS...] [options]
```

Show changes. With no revision: the working tree against your last checkpoint. One branch name: what that branch contributes, no checkout needed. One commit: that commit against the working tree. Two revisions: between them. On a terminal, opens the full-screen viewer.

| Flag | Effect |
| --- | --- |
| `--branch` | The whole current branch (checkpoints + uncommitted) against where it forked. |
| `--against <BRANCH>` | Base for a branch endpoint (default: its parent). |
| `--mode <contribution \| tree \| net-merge>` | Perspective for branch endpoints. See [Reviewing](https://oak.space/docs/reviewing#read-a-diff-without-switching). |
| `--print` | Print a unified diff instead of opening the viewer. |
| `--stat` | Per-file added/removed counts. |
| `--name-only` | Only the changed paths. |
| `--word-diff` | With `--print`: mark changes within lines. |
| `-U, --unified \<N\>` | Lines of context. |
| `--check` | Report whitespace errors and conflict markers; exit 1 if any. |
| `--exit-code` | Exit 1 if there are differences, 0 if not. |
| `-a, --text` | Treat large text files as text. |
| `--json` | Machine-readable summary. Add `--hunks` for patch text, `--max-bytes \<N\>` to cap it, `--changed-files-limit` / `--changed-files-offset` to page. |
| `--remote \<OLD\> \<NEW\>` | Diff two exact commits on the server, with `--json` or `--print`. |

### oak commit \#

```text
oak commit [PATHS...] [--push] [--no-verify] [--json] [--quiet]
```

Save a local checkpoint of every change (or only those under `PATHS`). Takes no message: `-m` is refused — describe the branch with `oak desc`. `--push` publishes afterwards; `--no-verify` skips [hooks](https://oak.space/docs/making-changes#hooks).

### oak restore \#

```text
oak restore [PATHS...] [-s COMMIT] [-f]
```

Restore files (or everything) to the last checkpoint, or to `COMMIT`. Asks first unless `-f`.

### oak reset \#

```text
oak reset [PATH] [-f]
```

Discard uncommitted changes — everything, or under `PATH`. Asks first unless `-f`.

### oak change \#

```text
oak change capture [PATHS...] [--json]
oak change export <CAPTURE_ID> --output <FILE> [--json]
```

Store uncommitted changes as an immutable capture without committing or touching the working tree; export one as a zip. See [History](https://oak.space/docs/history#save-uncommitted-work-without-committing).

## Branches \#

### oak switch \#

```text
oak switch [NAME] [-c] [--clean] [-d]
```

Switch to a branch (fetching it if needed). With no name, pick from a list.

| Flag | Effect |
| --- | --- |
| `-c, --create` | Create a branch off the latest `main` and switch to it. The name is optional. |
| `--clean` | Start from the latest `main` and discard working-tree changes. |
| `-d, --detach` | Treat `NAME` as a commit and detach HEAD there. |

### oak checkout \#

```text
oak checkout <COMMIT>
```

Detach HEAD at a commit (full hash or a unique prefix of 4+ characters).

### oak desc \#

```text
oak desc [DESCRIPTION | --file FILE] [--append] [--json]
```

Set the current branch's description — the commit message it will merge with. `--file -` reads stdin; `--append` adds a paragraph and refuses if your copy is stale. See [Descriptions](https://oak.space/docs/branches#descriptions).

### oak branch \#

```text
oak branch [--show-current] [--json]
oak branch list [--remote] [--status open|closed] [--json]
oak branch show <NAME> [--remote] [--json]
oak branch rename <OLD> <NEW>
```

List branches, print the current one, or show one branch's details. `--remote` reads the server. `rename` is temporarily disabled.

### oak close \#

```text
oak close [NAME...] [--remote] [--reason TEXT] [--json]
```

Close a branch (default: the current one). `--remote --json` closes pushed branches without switching to them, several at once.

### oak split \#

```text
oak split [--from BRANCH] [--plan FILE] [--dry-run]
```

Reorder or drop a branch's commits, or split them into separate branches (`oak histedit` is an alias). `--plan` applies a todo list non-interactively (`-` for stdin).

### oak finish \#

```text
oak finish (--desc TEXT | --desc-file FILE) [--json]
```

Set the description, checkpoint, and push — after checking it can. Inside a mount, also unmounts once the push succeeds. See [Working with agents](https://oak.space/docs/agents#finishing-a-task-in-one-command).

## Merging and conflicts \#

### oak merge \#

```text
oak merge [BRANCH] [--wait[=MINUTES]] [--force] [--dry-run] [--json]
oak merge --continue | --abort
```

Squash-merge a branch (default: the current one) onto `main` and close it.

| Flag | Effect |
| --- | --- |
| `--wait[=MINUTES]` | Wait for CI to finish (default 30 minutes), then merge if it passed. |
| `--force` | Merge even though CI is red or running. |
| `--dry-run` | With `--json`: predict the merge locally; changes nothing. |
| `--continue` / `--abort` | Finish or cancel a merge after resolving conflicts. |

### oak conflict \#

```text
oak conflict status [--json]
oak conflict show [--json]
oak conflict take <PATH> (--ours | --theirs) [--json]
```

Inspect an in-progress merge, pull, or mount-pull conflict, and resolve a file by taking one side of every conflict block. See [Resolving conflicts](https://oak.space/docs/merging#resolving-conflicts).

### oak agent \#

```text
oak agent state --json [--compact] [--refresh]
```

One JSON document with the repository's state and `recommended_next_commands`. `--refresh` checks the server first.

## Syncing \#

### oak push \#

```text
oak push [--repo ORG/REPO] [-f] [--json] [--plan --json] [-r URL]
```

Publish the current branch. `--repo` (or `OAK_REPO`) links a new repo on its first push without a prompt — the organization must exist; the repo is created. `-f` overwrites a diverged remote branch. `--plan --json` reports what would be sent without sending it.

### oak pull \#

```text
oak pull [--branch-only] [-f] [--json] [-r URL]
oak pull --continue | --abort
```

Fetch new commits on your branch, then merge the latest `main` into it. `--branch-only` skips the merge; `-f` discards local commits the server doesn't have.

### oak fetch \#

```text
oak fetch [-r URL]
```

Refresh your copy of `main` without touching your branch or files.

### oak sparse \#

```text
oak sparse [--json]
oak sparse set <PREFIX...>
oak sparse add <PREFIX...>
oak sparse disable
```

Show or change a [sparse checkout's](https://oak.space/docs/sparse-clones) cone.

## History and inspection \#

### oak log \#

```text
oak log [PATHS...] [-n N] [--oneline] [--verbose] [-S TERM | -G REGEX] [--json]
oak log --remote --json [--branch NAME] [--from HASH] [-n N]
```

Show history — full-screen on a terminal. `-S` finds commits that add or remove a string, `-G` commits with a changed line matching a regex. `--remote` reads a branch's history from the server without fetching it.

### oak hash / rev-parse \#

```text
oak hash
oak rev-parse [--short] HEAD
```

Print the current commit. `rev-parse` exists for Git-compatible scripts and supports only `HEAD`.

### oak info \#

```text
oak info [--json]
```

Repository, branch, parent, remote, and link details.

### oak file inspect / tree inspect / refs inspect \#

```text
oak file inspect <PATH> --at <HEAD|HASH> [--remote] [--output FILE] [--max-bytes N] [--json]
oak tree inspect --at <HEAD|HASH> [--max-files N] [--max-bytes N] [--json]
oak refs inspect [--max-branches N] [--json]
```

Read a file or tree at a commit, or list local branch heads, without changing anything. See [History and inspection](https://oak.space/docs/history#inspect-files-and-trees-at-a-commit).

### oak open \#

```text
oak open [--print | --json]
```

Open the repo on oak.space, or just print its URL.

## Reviewing \#

### oak branch triage \#

```text
oak branch triage [--remote] [--only mergeable|closable|ambiguous] [--against BRANCH]
                  [--status STATUS] [--analysis-depth DEPTH] [--limit N] [--json]
```

Score many branches at once — mergeability, contribution, CI, recommended action — without switching.

### oak branch review \#

```text
oak branch review <NAME> [--remote] [--merge-preview] [--json]
```

Evidence for one branch: what changed, and with `--merge-preview`, whether and how safely it would merge.

### oak branch diff \#

```text
oak branch diff <NAME> [PATHS...] [--remote] [--against BRANCH]
                [--diff-mode tree|contribution|net-merge] [--print] [--json [--hunks]]
```

A branch's diff without checking it out. Default mode is `tree`.

### oak branch train \#

```text
oak branch train <NAME...> [--remote] [--against BRANCH] [--json]
```

Preview merging up to 32 branches in order, without publishing.

## CI \#

### oak ci status \#

```text
oak ci status [--run ID [--commit HASH]] [--json]
```

The merge gate's verdict for the current branch head, or one run. Exit `0` passed, `1` failed or no runs, `3` still running.

### oak ci runs \#

```text
oak ci runs [--limit N] [--json]
```

Recent runs: id, workflow, branch, commit, status, duration.

### oak ci logs \#

```text
oak ci logs <RUN_ID> [--failed] [--summary] [--max-bytes N] [--json]
```

A run's step output. `--failed` shows only failed steps; `--summary` shows step metadata without logs.

### oak ci wait \#

```text
oak ci wait <RUN_ID...> | --current [--timeout SECONDS] [--progress] [--json [--summary]]
```

Block until runs finish. `--current` waits for the current head's runs (waiting up to `--dispatch-timeout` for them to start). Default timeout 30 minutes.

### oak ci trigger \#

```text
oak ci trigger --expected-commit <HASH> --idempotency-key <KEY> [--workflow NAME] [--branch NAME] [--json]
```

Start a run by hand — only if the branch head is still `HASH`. Re-using the key returns the original run. Name the workflow, or every workflow runs.

### oak ci rerun / cancel \#

```text
oak ci rerun <RUN_ID> [--json]
oak ci cancel <RUN_ID> --commit <HASH> [--json]
oak ci cancel --superseded [--yes] [--json]
```

Re-run a run at the same commit (for infrastructure failures), or cancel a push or merge run. `--superseded` lists — or with `--yes`, cancels — the branch's in-flight runs for commits that are no longer its head.

## Mounts and spaces \#

### oak mount \#

```text
oak mount <ORG/REPO> [DEST] [-b BRANCH] [-r URL]
oak mount list [--json]
oak mount finish [DEST] --desc-file FILE [--json]
oak mount end [DEST] [-f]
oak mount forget [DEST] [-f] [--orphaned]
```

Mount a repo without cloning it (default destination `./\<repo\>`), optionally continuing an existing branch. `end` with no `DEST` ends every mount under `~/oaktree`; `-f` discards uncommitted changes. `forget` cleans up stale registrations. See [Lazy mounts](https://oak.space/docs/mounts).

### oak space \#

```text
oak space new <ORG> [DEST]
oak space repos [ORG] [--json]
oak space clean [DEST] [-f]
oak space inventory [ROOT] [--verify-local] [--include-ci] [--max-depth N] [--json]
```

Scaffold and manage an [agent space](https://oak.space/docs/spaces).

## Sites \#

```text
oak site enable [--repo ORG/REPO] [--source PATH]
oak site show [--organization ORG]
oak site list
oak site disable [--organization ORG]
```

Publish a repo as the organization's [static site](https://oak.space/docs/sites). In this group `-r` means `--repo`; the server is `--remote`.

## Tools \#

### oak export \#

```text
oak export <DEST> [-b BRANCH] [--git-branch NAME] [-f]
oak export <DEST> --tree-only --at <HEAD|HASH>
```

Replay a branch's history into a new Git repo, or write one commit's files. See [Export back to Git](https://oak.space/docs/git#export-back-to-git).

### oak archive \#

```text
oak archive [-o PATH]
```

Zip the working tree (default `\<directory\>.zip`).

### oak feedback \#

```text
oak feedback [-m TEXT | --file FILE] [--title TITLE] [--email EMAIL] [--json]
```

Send a bug report or feature request; prints a tracking reference (`fb-N`). Opens your editor on a terminal when no text is given. Alias: `oak feature-request`.

### oak environment \#

```text
oak environment [--json]
```

List every environment variable Oak reads and its effective value (secrets shown as set/unset only). Alias: `oak env`.

### oak maintenance compact \#

Compact the local repository database and reclaim disk space.

### oak doctor \#

```text
oak doctor --repo ORG/REPO [--verify metadata|existence|bytes] [--depth N] [--json]
```

Check a remote repository's content end to end, from commits down to stored bytes. Exit `0` verified, `6` a problem was found, `8` inconclusive.

### oak serve \#

```text
oak serve [-d DIR] [-p PORT] [--host HOST] [--token TOKEN]
```

Run a minimal self-hosted Oak server backed by SQLite (default `./oak-data` on `127.0.0.1:8080`) that speaks the push / pull / clone protocol. No organizations or accounts; binding to anything but loopback requires `--token` (or `OAK_SERVE_TOKEN`). For local development and trusted networks.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
