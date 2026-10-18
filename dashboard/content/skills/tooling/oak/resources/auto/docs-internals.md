+++
title = "docs-internals"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Manifests, BLAKE3, FastCDC chunking, and how Oak compares to Git, Mercurial, and Fossil.
title: How Oak works · Docs
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

# How Oak works

Copy page [.md](https://oak.space/docs/internals.md "This page as raw Markdown")

Manifests, BLAKE3, FastCDC chunking, and how Oak compares to Git, Mercurial, and Fossil.

For people who want to know what's under the hood: the object model, the chunking, and how Oak compares to the version control systems that influenced it. Oak is written in Rust; the CLI is a single binary.

## Manifests and commits, like Mercurial \#

Oak's object model is closer to [Mercurial's](https://www.mercurial-scm.org/wiki/Design) than Git's. Each commit points at a **manifest** — the full snapshot of the tree as a list of `(path, blob hash, mode)` entries — rather than at a hierarchy of tree objects.

Everything is **content-addressed with BLAKE3**: a file's identity is its hash, so storing the same content twice costs nothing. Diffing two commits means walking two manifests and comparing hashes.

Commits on a branch carry no message. The narrative lives on the branch, as its description, and only the squash commit that lands on `main` gets a message — derived from that description. The squash keeps a pointer to the branch's last commit, so the detailed history stays reachable without cluttering `main`.

## Content-defined chunking with FastCDC \#

Files are split into variable-length chunks with [FastCDC](https://www.usenix.org/system/files/conference/atc16/atc16-paper-xia.pdf). Chunk boundaries are chosen by the content itself rather than at fixed offsets, so inserting a line near the top of a 100 MB file changes only the chunks around the edit, not everything after it.

Git stores each version of a large file as a separate blob and relies on packfile delta compression after the fact. Oak chunks at write time, so deduplication is immediate and works across files: two assets that share a header share those chunks. Each chunk is hashed with BLAKE3, each blob records its ordered chunk list, and push and pull transfer only the chunks the other side is missing. On the server, chunks are deduplicated across every repo in an organization.

## Trust: the client verifies \#

The client re-derives every commit's manifest hash from the entries the server sends, and rejects a mismatch. That's what makes clones, pulls, and mounts trustworthy — and it's also why [path permissions](https://oak.space/docs/path-permissions) withhold a restricted file's *contents* but can't hide its name: removing an entry would change the hash.

## Lazy mounts \#

A mount is a virtual filesystem backed by the server: [FSKit](https://developer.apple.com/documentation/fskit) on macOS, FUSE on Linux, and the Projected File System on Windows. Directory listings come from the manifest; a file's chunks are fetched the first time something reads it and cached locally. Writes go to a local overlay — the "active commit" — that `oak commit` turns into a real commit on the mount's branch. Nothing is uploaded until you push.

## Local storage, like Fossil \#

A local repository is a single SQLite database (`.oak/oak.db`) holding commits, manifests, and branch state, with large content stored as chunks — not a directory of loose objects.

## How Oak differs from Fossil \#

[Fossil](https://fossil-scm.org) is the closest prior art in philosophy: a single local database file instead of a loose-object store, and a repository you can back up with `cp`. Where they diverge:

- **Large files.** Fossil stores content inline in SQLite, which caps out on large binaries. Oak chunks content and stores chunks by hash, so large assets are first-class rather than a workaround.
- **Object model.** Fossil uses its own text artifact format. Oak uses flat manifests — path to blob hash — which are simpler to reason about.
- **Branching.** In Fossil a branch is a tag on a commit in a timeline. Oak has named branches with a parent, and branches are the unit of work.
- **Scope.** Fossil bundles a wiki, bug tracker, and forum. Oak stays close to version control, plus what gates a change landing — [CI](https://oak.space/docs/ci) from `.oak/workflows/` and access rules from `.oak/PERMISSIONS`, both files in the repo. No wiki, issue tracker, or forum; keep the tools you already use for those.
- **Agents.** Oak is built for branch-per-agent work: clone or mount, commit on a personal branch, push. There's no separate pull-request object — the branch *is* the review unit, and its description is the change's story.

## The server \#

oak.space is a single Rust binary (Axum) with PostgreSQL for metadata and object storage for chunks, running in Oregon. The web UI is server-rendered HTML with [htmx](https://htmx.org) for partial updates — no client-side framework or JavaScript bundle. CI runs in isolated Linux containers dispatched by the server.

Questions about the internals, or interested in working on Oak? Email [zach@oak.space](mailto:zach@oak.space) or join the [Discord](https://oak.space/discord).

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
