---
description: Mount a repo without cloning it — files arrive as they&#39;re read — plus per-platform setup.
title: Lazy mounts · Docs
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

# Lazy mounts

Copy page [.md](https://oak.space/docs/mounts.md "This page as raw Markdown")

Mount a repo without cloning it — files arrive as they're read — plus per-platform setup.

`oak mount` puts a working tree on top of a remote repository **without cloning it**. Files are fetched the first time something reads them, so you can be editing a multi-gigabyte monorepo within seconds, and running ten tasks in parallel costs ten mounts, not ten clones.

```bash
oak mount acme/monorepo                 # mounts at ./monorepo
oak mount acme/monorepo ./fix-auth      # …or wherever you say
cd fix-auth
$EDITOR src/auth/callback.rs            # only the files you touch are downloaded
oak status && oak commit && oak push
```

Each mount runs as a background daemon; the command returns as soon as the mount is live. Before your first mount, do the [one-time setup](#setting-up-mounts) for your OS.

## How a mount works \#

- **A mount is one branch.** By default `oak mount` starts a fresh *virtual branch* off the latest `main`, named after the mount directory plus a short id — mount at `./fix-auth` and you're on `fix-auth--a1b2c3d4`.
- **Edits stay local until you push.** Changes go into an overlay on your machine — the **active commit**, the one you're amending as you edit. `oak commit` checkpoints it onto the virtual branch and starts a new one; `oak push` publishes the branch like any other.
- **The normal commands just work.** Inside a mount, `oak status`, `diff`, `commit`, `desc`, `log`, `hash`, `info`, `push`, `pull`, `finish`, `conflict`, and `agent state` all operate on the mount's virtual branch, so a mount feels like a clone.

A few things are different inside a mount:

| Command | Inside a mount |
| --- | --- |
| `oak switch` | Refused — a mount is one branch. Mount another branch elsewhere. |
| `oak merge` | Doesn't run. Merge from a clone or the web. |
| `oak reset`, `oak restore` | Don't run. Use `oak diff` / `oak status` to see the overlay, or end the mount with `--force` to discard it. |
| Hooks | Don't run. |
| `oak commit --json`, `--push` | Not supported; run `oak commit` then `oak push`. |

## Continue an existing branch \#

```bash
oak mount acme/monorepo ./fix-auth --branch fix-auth--a1b2c3d4
```

`--branch` mounts an existing remote branch instead of starting a fresh one: its history and files become the mount's. Use it to pick up a branch an agent already pushed, give it a follow-up, or [resolve its conflicts](https://oak.space/docs/merging#when-a-pushed-branch-conflicts) without a full clone.

## Finish, end, and list mounts \#

```bash
oak mount list                     # every mount, labelled live, stale, or orphaned
oak mount list --json              # with dirty and unpushed counts — good for watching a fleet

oak finish --desc-file desc.md --json          # inside a mount: describe, commit, push, then unmount
oak mount finish ./fix-auth --desc-file desc.md --json   # same thing, from outside

oak mount end ./fix-auth           # unmount and delete the directory; refuses if there's unpushed work
oak mount end ./fix-auth --force   # …discarding uncommitted changes
oak mount end                      # end every mount under ~/oaktree
```

`oak finish` / `oak mount finish` check everything they can **before** changing anything, and only unmount once the push has succeeded. If a step fails the mount stays intact and the output names the command to run next — so finishing never silently loses work.

## Setting up mounts \#

Mounts use your operating system's own virtual-filesystem layer. Each needs a one-time setup per machine; no third-party kernel extension or driver is involved. Everything other than mounting — clone, commit, push, pull — works without it.

### macOS: the Oak Mount app (macOS 26+) \#

On macOS the filesystem runs inside a signed FSKit extension, **OakFS**, shipped in the **Oak Mount** app. The app only carries the extension: it must be installed and enabled once, but it doesn't need to be running, and one install serves every mount.

1. Run your first `oak mount`. If the installer didn't already, the CLI downloads Oak Mount, verifies its checksum, installs it in `/Applications` (or `~/Applications`), and opens it once so macOS registers the extension.
2. Enable it: **System Settings → General → Login Items & Extensions → File System Extensions**, then turn on **OakFS**. The Oak Mount window has a button that takes you there. No restart needed.
3. Run `oak mount` again. From now on it attaches immediately.

If the extension is ever switched off, `oak mount` says so rather than failing mysteriously. Keep the app in `/Applications`.

**Pick a destination outside `~/Documents`, `~/Desktop`, `~/Downloads`, and iCloud Drive.** macOS requires Full Disk Access for your terminal to mount inside those; somewhere like `~/oaktree` avoids it.

### Linux: FUSE \#

1. Install FUSE 3: `sudo apt install fuse3` on Debian and Ubuntu, or your distro's equivalent. Oak mounts through the `fusermount3` helper it provides.
2. If your agent or editor runs as a different user, or in a sandbox, and needs to see into the mount, uncomment `user_allow_other` in `/etc/fuse.conf`.
3. On distros that gate FUSE behind a group (often `fuse`), add yourself to it and log in again.

### Windows: Projected File System \#

Windows mounts use ProjFS — the same primitive GVFS and Scalar use for huge Git monorepos. Enable it once, from an elevated PowerShell:

```powershell
Enable-WindowsOptionalFeature -Online -FeatureName Client-ProjFS -NoRestart
```

Or **Settings → Apps → Optional features → Windows Projected File System**.

### Check it works \#

```bash
oak mount acme/web /tmp/web-test && oak mount list
oak mount end /tmp/web-test
```

If the mount doesn't attach, the error almost always points back at one of the steps above.

## Keep build output off the mount \#

A mount is ideal for reading and editing source and poor for build output and tool caches. Keep those on your real disk:

- **Redirect build directories.** `CARGO_TARGET_DIR=/tmp/fix-auth-target cargo test`; likewise for `dist/`, `build/`, and other generated directories. A build cache on a mount is slow and can produce baffling, wrong errors.
- **Don't install `node_modules` on a mount.** On Linux, FUSE refuses to create symlinks and hard links ( `EPERM`), which package managers rely on. Install outside the mount and point the tool at it.
- **Don't rewrite the whole tree at once.** A repo-wide formatter run, codemod, or huge `rm -rf` can wedge a mount. Scope formatters to what you changed ( `cargo fmt -p \<crate\>`).

## Troubleshooting \#

| Symptom | Fix |
| --- | --- |
| Commands fail with `os error 22` | The mount is wedged. Commit and push what you can, then `oak mount end --force \<dir\>` and mount again. **`--force` discards uncommitted changes.** |
| `oak mount list` shows a mount as `stale` after a crash or reboot | Running `oak mount` again for it recovers it from its saved state. |
| A mount shows as `orphaned` (its directory is gone) | `oak mount forget --orphaned` drops those registrations; their saved state is kept and reported. |
| A registration won't go away | `oak mount forget \<dir\>` removes it without touching on-disk state (`--force` if it looks live). |

Mount state lives under `~/.oak/mounts/` (override with `OAK_MOUNTS_ROOT`).

## Mounts, clones, or sparse clones? \#

|  | Mount | Clone | Sparse clone |
| --- | --- | --- | --- |
| Time to first edit on a huge repo | Seconds | Full download | Download of the subtree |
| Files on disk | Only what's been read | Everything | Only the cone |
| Needs OS setup | Yes | No | No |
| Good for builds | Keep output off the mount | Yes | Yes |
| Can merge from it | No | Yes | Yes |

For a plain on-disk checkout of part of a monorepo, see [Sparse clones](https://oak.space/docs/sparse-clones). To run many agents across an org's repos, see [Agent spaces](https://oak.space/docs/spaces).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page