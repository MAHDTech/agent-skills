---
description: Install the oak CLI on macOS, Linux, or Windows, sign in, and keep it up to date.
title: Install and sign in · Docs
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

# Install and sign in

Copy page [.md](https://oak.space/docs/install.md "This page as raw Markdown")

Install the oak CLI on macOS, Linux, or Windows, sign in, and keep it up to date.

## Install the CLI \#

On **macOS** (Apple Silicon or Intel) and **Linux x86\_64**:

```bash
curl -fsSL https://oak.space/install | sh
```

On **Windows x86\_64**, from PowerShell:

```powershell
irm https://oak.space/install.ps1 | iex
```

The installer picks the native binary for your machine, verifies its minisign signature, and puts it at `~/.local/bin/oak` (`%USERPROFILE%\.local\bin\oak.exe` on Windows). If that directory isn't on your `PATH`, the installer prints the line to add.

| Installer variable | Effect |
| --- | --- |
| `INSTALL_DIR` | Install somewhere else — e.g. `/usr/local/bin` on a CI image, so the binary is already on `PATH`. |
| `OAK_NO_LOGIN=1` | Skip the "Log in now?" prompt at the end. (It's skipped automatically when there's no terminal.) |

On macOS the installer also fetches the **Oak Mount** app that lazy mounts need; if it can't, the first `oak mount` installs it instead. See [Setting up mounts](https://oak.space/docs/mounts#setting-up-mounts).

> **Linux ARM64:** binaries are published on the [GitHub releases page](https://github.com/oakdotspace/oak/releases), but the installer doesn't select them yet. Download `oak` for `linux-arm64` from there and put it on your `PATH`.

## Sign in \#

```bash
oak login
oak whoami        # prints the username you're signed in as
```

`oak login` opens your browser at oak.space, where you sign in however you normally do (password or GitHub), and hands a token back to the CLI over a one-shot local callback. On a machine with no browser — an SSH session, a container — it prints a URL instead: open it on any other device, approve, and paste the one-time code it shows back into the terminal.

The token is saved in `~/.oak/credentials` and lasts 90 days. Run `oak login` again to refresh it, or `oak logout` to remove it.

To check which credential the CLI is actually using — handy when a push 404s on a repo you know exists — run:

```bash
oak auth status
```

It reports where the credential came from (never the secret itself), which server, and the identity the server resolves it to.

### Non-interactive machines and CI \#

`oak login` needs a human once. For scripts, CI jobs, and long-running agents, use an API key instead and export it:

```bash
export OAK_API_KEY=oak_...
```

`OAK_API_KEY` beats every stored credential. Create keys — including narrowly scoped, read-only, or single-repo keys — as described in [API keys and tokens](https://oak.space/docs/api-keys).

### Self-hosted or staging servers \#

Every command that talks to a server takes `-r, --remote \<URL\>` (default `https://oak.space`). You can also set `OAK_REMOTE` for a whole shell. Once a checkout is linked to a server, it remembers which one.

## Keep it up to date \#

```bash
oak upgrade             # latest stable release
oak upgrade --canary    # track the rolling pre-release channel
```

Upgrades are signature-checked like the installer. The CLI checks for a new version at most once a day and prints a one-line notice; set `OAK_NO_UPDATE_CHECK=1` to turn that off.

## Shell completion \#

```bash
oak completions zsh  > ~/.zfunc/_oak                                     # zsh
oak completions bash > ~/.local/share/bash-completion/completions/oak    # bash
oak completions fish > ~/.config/fish/completions/oak.fish               # fish
```

`elvish` and `powershell` are supported too.

## Teach your agent \#

If you use a coding agent, install the bundled skill in each repo you work in:

```bash
oak skill install            # into ./.claude/skills/ — commit it
oak skill install --global   # or into ~/.claude/skills/ for every project
```

More in [Working with agents](https://oak.space/docs/agents).

## Uninstall \#

Delete the binary (`~/.local/bin/oak`) and, if you want to drop your login and local mount state too, `~/.oak/`. On macOS, also delete **Oak Mount** from `/Applications`. Repositories you've cloned are ordinary directories; each one's Oak metadata lives in its own `.oak/` folder.

Next: [Quickstart](https://oak.space/docs/quickstart).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page