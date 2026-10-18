---
description: Bring a Git repo into Oak, get your history back out, and clone Oak repos with stock git.
title: Git import, export, and interop · Docs
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

# Git import, export, and interop

Copy page [.md](https://oak.space/docs/git.md "This page as raw Markdown")

Bring a Git repo into Oak, get your history back out, and clone Oak repos with stock git.

Oak is designed to be easy to arrive at and easy to leave. This page covers every way code moves between Git and Oak.

| You want to… | Use |
| --- | --- |
| Bring a GitHub repo over, with nothing installed | [Import from GitHub on the web](#import-from-github-on-the-web) |
| Bring any Git repo over from the terminal | [`oak clone <git-url>`](#import-with-oak-clone) |
| Convert a Git checkout you already have | [`oak init` in place](#convert-a-checkout-in-place) |
| Start a repo from a folder in the browser | [Upload a folder](#upload-a-folder) |
| Get full history back out as Git | [`oak export`](#export-back-to-git) |
| Let Git tooling read an Oak repo | [`git clone …/.git`](#clone-an-oak-repo-with-git) |
| Keep a copy on GitHub, or run GitHub Actions | [GitHub mirror](#mirror-to-github) |

## Import from GitHub on the web \#

No CLI, no Git — oak.space clones and converts the repo on its servers.

1. **Connect GitHub, once:** under **Settings → GitHub**, choose **Connect GitHub** and install Oak's GitHub App on the repositories you want (all, or a selection). For importing, Oak asks only for **read** access and mints a short-lived, single-repo token for each import.
2. **Pick a repo:** the **Import from GitHub** panel on your dashboard lists the repositories your connection can see.
3. **Choose what to bring:** the **branch** to import and the **destination organization**; optionally a different name, public visibility, and collaborators to invite. Oak checks the name is free as you type.
4. **Import:** a progress page streams status and opens the new Oak repo when it's done.

The web importer brings the **one branch you choose** straight onto the server as `main` — there's nothing to push afterwards. Run `oak clone \<org\>/\<repo\>` (or [mount it](https://oak.space/docs/mounts)) to get a working copy.

## Import with oak clone \#

```bash
oak clone https://github.com/acme/web.git
oak clone git@github.com:acme/web.git
oak clone https://git.example.com/acme/web.git my-dir    # choose the directory
```

Oak runs your system `git clone` (so `git` must be on your `PATH`, and your Git credentials apply), creates an Oak repo, replays every commit onto `main`, and leaves you on a personal branch at the tip. It then offers to delete the now-redundant `.git` directory — keeping it if you decline or there's no terminal to ask.

**What counts as a Git URL:** anything starting with `git@`, `ssh://`, or `git://`; anything ending in `.git`; and github.com, gitlab.com, and bitbucket.org URLs. For a self-hosted server over HTTPS, make sure the URL ends in `.git` so Oak doesn't read it as an Oak `org/repo`.

Oak-only flags (`--branch`, `--path`, `--detached`, `--from`, `--json`) are rejected with a Git URL — the importer always brings the checked-out history.

## Convert a checkout in place \#

```bash
cd my-existing-git-repo
oak init
# Detected an existing git repository here. Import its history into oak?
```

Accept, and Oak replays the Git history onto `main`, puts you on a personal branch at the tip, and offers to remove `.git`. Decline, and your `.git` is untouched — handy if you want to keep using Git alongside for a while. (The prompt needs a terminal.)

## Publish what you imported \#

A CLI import lives on your machine until you push. The first push names the owner, which creates the repo on oak.space:

```bash
oak push --repo acme/web
```

Then merge your branch, or start working — see the [Quickstart](https://oak.space/docs/quickstart).

## What's preserved \#

**Kept:**

- The full commit graph in order, including both parents of merge commits.
- Every commit's original author, email, message, and timestamp.
- File modes: regular, executable, and symlinks.

**Dropped or simplified:**

- **Submodules** are skipped — Oak has no submodule concept.
- **Octopus merges** (three or more parents) keep their first two parents.
- **Only one line of history** comes across: the checked-out `HEAD` for CLI imports, or the branch you pick on the web. Other branches and tags aren't recreated.

Your first `oak status` after importing is clean: Oak rewrites the working tree from its own records, so `.gitattributes` smudge filters and leftover submodule directories don't show up as changes. Your `.gitignore` keeps working — Oak reads it when there's no `.oakignore`.

## Upload a folder \#

When creating a repo on oak.space, you can upload a folder from your computer instead of starting empty. The import dialog has presets for Godot, Unity, and Unreal projects that skip their generated directories. Files can also be uploaded into an existing repo from its page.

## Export back to Git \#

```bash
oak export ../web-as-git
oak export ../web-as-git --branch main --git-branch main
```

`oak export` replays an Oak branch's history (by default the current one) into a brand-new Git repository, preserving each commit's author, email, and timestamp. Push it anywhere. This is Oak's documented escape hatch: your code and its history are never trapped.

| Flag | Effect |
| --- | --- |
| `-b, --branch` | The Oak branch to export (default: current). |
| `--git-branch` | Name of the branch in the Git repo (default: same as the Oak branch). |
| `-f, --force` | Write into a destination that isn't empty. |
| `--tree-only --at \<commit\>` | Write just one commit's files, without history. |

## Clone an Oak repo with Git \#

```bash
git clone https://oak.space/acme/web.git
```

Every Oak repo can be cloned by stock Git — and anything built on it, like package managers, deploy scripts, or Claude Code's `/plugin marketplace add` — as a **read-only snapshot of `main`**:

- It's a **single commit** containing `main`'s current files, with no history. Use `oak export` for history.
- It's **fetch-only**; `git push` to it fails.
- It works for **public repos**, and for the `@public` paths of a private repo. The endpoint doesn't accept API keys or passwords yet, so a private repo answers "not found" to `git` — use `oak clone` or `oak export` for those.
- [Path permissions](https://oak.space/docs/path-permissions) apply: restricted files are left out, and for outsiders only `@public` paths are included.

## Mirror to GitHub \#

A repo can keep a copy of `main` on GitHub — as a backup, or to run GitHub Actions or other GitHub-side tooling. Set it up under the repo's **Settings → Export**:

1. Create an **empty** repo on GitHub, and make sure Oak's GitHub App (or your connected GitHub account) can push to it.
2. Choose it, and the branch to mirror to.

On every merge to `main` (and whenever you click **Sync now**), Oak writes `main`'s current files to that GitHub branch as **a single snapshot commit and force-pushes it**. Like the Git clone endpoint, it's the current state, not a history replay. Because it force-pushes, use a repo or branch dedicated to the mirror; Oak refuses to overwrite a branch whose latest commit it didn't create. Files over 100 MB can't be mirrored (GitHub's limit), and restricted paths are left out.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page