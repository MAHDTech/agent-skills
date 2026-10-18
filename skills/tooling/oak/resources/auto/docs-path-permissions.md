---
description: Restrict or publish subtrees of a repo with `.oak/PERMISSIONS`.
title: Path permissions · Docs
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

# Path permissions

Copy page [.md](https://oak.space/docs/path-permissions.md "This page as raw Markdown")

Restrict or publish subtrees of a repo with \`.oak/PERMISSIONS\`.

[Repository access](https://oak.space/docs/organizations#who-can-see-a-repository) decides who can see a repo at all. **Path permissions** change that *inside* the tree, in both directions: hide a directory from most readers, or publish one directory of a private repo to the world.

They're declared in a file in the repo — `.oak/PERMISSIONS`, shaped like `CODEOWNERS` — and always read from the tip of `main`.

## The file \#

```text
# Anything not listed is readable by anyone who can read the repo.

infra/secrets/**   @zdgeier @group/sec-team
scripts/           @group/ops
scripts/public/    *            # re-opened underneath a locked parent
vault/                          # no principals = org owners and admins only

[Open source]                   # a label, for the settings UI
cli/               @public      # published to the world
```

Each line is a **pattern** followed by the **principals** allowed to read what it matches.

| Principal | Means |
| --- | --- |
| `@alice` | The user `alice`. |
| `@group/ops` | Every member of the organization's [`ops` group](https://oak.space/docs/organizations#groups). |
| `*` | Everyone who can already read the repo. |
| `@public` | Everyone, including people with no access to the repo at all. |
| *(none)* | Organization owners and admins only. |

Owners and admins always have full access, whatever the file says.

## Matching \#

Patterns work like `.gitignore`, and **the last matching line wins**.

- `foo/` matches the directory `foo` and everything in it; `foo` matches a file *or* directory named `foo`.
- A pattern with no `/` matches at any depth — `*.pem` covers `certs/prod.pem`. A pattern containing `/` is anchored to the repo root, as is one starting with `/`.
- `*` doesn't cross `/`; `**` does.
- `#` starts a comment; `\#` is a literal `#`, and `\` a literal space.
- `[Section name]` lines just label the entries after them in the UI. They don't affect matching.

Last-match-wins is what makes re-opening read naturally: put the narrower line *after* the one that locks its parent. It also means order matters — moving a broad line below a narrow one changes what the narrow one does.

## Restricting paths \#

For someone who can read the repo, a path no line matches is readable, and lines take access away.

What a restricted path looks like to someone without access:

- **Contents are withheld; names and sizes stay visible.** Oak's trees are content-addressed and verified by the client, so the server can't hide entries from `oak clone`, `oak pull`, or a mount without breaking that verification. Restricted files arrive empty or absent, and the CLI tells you which were withheld.
- **The web UI, zip downloads, the `git clone` snapshot, the GitHub mirror, and [Sites](https://oak.space/docs/sites)** leave restricted entries out entirely.
- **Writes are refused.** A push or revert that changes a restricted path is rejected unless the pusher has access. Merging `main` into your branch with `oak pull` is fine — carrying the restricted files along unchanged isn't a write.
- If identical content also lives at a path you *can* read, you can read those bytes there.

## Publishing paths with @public \#

`@public` runs the file the other way, for people who have **no** access to the repo. For them, a path is hidden unless a line grants it to `@public`. That's how one private repo can contain a genuinely open-source subtree — and it fails safe: forget a rule, and too little is published, never too much.

Outsiders can read a published subtree:

- in the web UI (file tree, file view, zip download), and
- with stock Git: `git clone https://oak.space/\<org\>/\<repo\>.git` returns only the public paths.

They can't use `oak clone`, `oak pull`, or `oak mount` — that protocol has to send whole trees, which would reveal the names and sizes of everything private. Branches, commits, diffs, and CI also stay private. And a private repo with public paths **isn't listed** anywhere; people reach it by direct link.

## Who can change the file \#

1. The file is read from **`main`**, never from the branch being served — a branch can't grant itself access.
2. Once a repo has a `.oak/PERMISSIONS`, **only organization owners and admins** can change it. (Before one exists, anyone with write access can create it, so someone can set it up.)

## Editing it \#

**From the web:** the repo's **Settings → Path permissions** tab shows the server's own reading of the file — every entry, who it grants, any warnings, and whether enforcement is on — and has an editor for people with write access. Saving **proposes a branch** with the change rather than writing `main`, so it lands through a normal merge.

**From the CLI:** it's an ordinary file; commit it, push, and merge like any other change.

> CLI versions up to 0.105.0 skip `.oak/PERMISSIONS` when committing. If `oak status` doesn't show your edit, `oak upgrade`, or make the change from the web editor.

**Over the API:** `GET /api/{owner}/{name}/path-permissions` returns the parsed policy; `POST …/path-permissions/propose` proposes a new file as a branch. See [the API reference](https://oak.space/docs/api#path-permissions).

## When something's wrong with the file \#

Every failure leans toward denying access:

| Situation | Result |
| --- | --- |
| A pattern can't be parsed | That line is dropped (shown as a warning in Settings). |
| A user or group doesn't exist | It grants no one. |
| The file is over 64 KiB or isn't valid UTF-8 | **Everything** is locked to owners and admins. |
| There's no file | No restrictions. |

## Path permissions and CI \#

[CI runs](https://oak.space/docs/ci) check out the **whole** tree, restricted paths included — otherwise a repo with a private half couldn't build itself. To keep that safe, a run's logs and status are only visible to people who aren't restricted from anything in the repo. Workflow files are always read in full, even if `.oak/workflows/` is itself restricted.

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page