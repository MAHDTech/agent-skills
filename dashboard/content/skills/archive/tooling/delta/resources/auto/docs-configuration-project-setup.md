+++
title = "docs-configuration-project-setup"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Prepare your project"
url: "https://delta.dev/docs/configuration/project-setup"
description: "Set up dependencies, share selected local files, and run development environments in isolated workspaces."
---

1.  Configuration
2.  Prepare your project

# Prepare your project

Set up dependencies, share selected local files, and run development environments in isolated workspaces.

On this page

1.  [Run setup in each checkout](#run-setup-in-each-checkout)
    1.  [Should I share node\_modules?](#should-i-share-node_modules)
2.  [Share local files with .agents/linked](#share-local-files-with-agentslinked)
    1.  [File format](#file-format)
    2.  [Does .agents/linked need to be committed?](#does-agentslinked-need-to-be-committed)
    3.  [Where linked files live](#where-linked-files-live)
3.  [Run development environments in parallel](#run-development-environments-in-parallel)
4.  [Delta-specific overrides](#delta-specific-overrides)
5.  [Custom emoji](#custom-emoji)
6.  [What’s next](#whats-next)

Automate setup for new isolated workspaces, and choose which local files they share.

To continue unfinished code changes, [return to the original thread](/docs/agents/threads#resume-work-on-a-feature) instead of creating a new workspace.

What you need

What to use

Install dependencies or recreate generated setup

[`.agents/prepare`](#run-setup-in-each-checkout)

Reuse selected local files on one machine

[`.agents/linked`](#share-local-files-with-agentslinked)

Run several development environments at once

[Separate ports and mutable resources](#run-development-environments-in-parallel)

## [Run setup in each checkout](#run-setup-in-each-checkout)

Open **Settings > Thread > .agents/prepare** and choose **Create with Delta** to have the agent write a setup script. Or create an executable `.agents/prepare` file at your repository root.

For a pnpm project on macOS or Linux, save this script as `.agents/prepare`. It requires pnpm on the machine running it and a committed lockfile:

```
#!/bin/sh
set -eu

pnpm install --frozen-lockfile
```

Make it executable from your repository root:

```
chmod +x .agents/prepare
```

Commit the script so new workspaces can use it.

Delta runs the script from the checkout root when it mounts a managed checkout, before the agent starts. It skips adopted existing checkouts and doesn’t rerun the script when reusing an already-mounted checkout.

On Windows, Delta runs preparation scripts through Git Bash. Install Git for Windows and write the script using POSIX shell syntax rather than PowerShell commands.

Keep the script non-interactive and safe to repeat when a checkout is recreated. If it fails, the thread shows the error and the checkout remains usable.

### [Should I share `node_modules`?](#should-i-share-node_modules)

Install dependencies in each checkout rather than linking `node_modules`. Branches can require different dependency versions. Installing into a shared folder changes the dependencies other workspaces use.

Let pnpm reuse its package store, but keep `node_modules` separate for each checkout. Its symlinks do not protect workspaces from changes to a shared installation.

## [Share local files with `.agents/linked`](#share-local-files-with-agentslinked)

Use `.agents/linked` to share local files or folders, such as `.env`, between managed checkouts of the same repository on one machine. Each path is a symlink to a [shared location](#where-linked-files-live), not a separate copy. Editing the shared content affects every checkout using it.

> **Warning:** Save any unique contents elsewhere before linking a path. If both the managed checkout and the shared location contain it, Delta replaces the checkout’s copy with a symlink to the shared copy. Delta does not merge their contents.

### [File format](#file-format)

`.agents/linked` is a UTF-8 text file with one repository-relative file or folder path per line.

Linked paths must be ignored. To share a local `.env` file, first add this rule to your repository’s `.gitignore` rather than relying on global Git ignore configuration:

```
/.env
```

Then create `.agents/linked` at your repository root:

```
# Shared local configuration.
.env
```

*   Use `/` as the path separator, including on Windows.
*   Blank lines and lines starting with `#` are ignored.
*   Use literal paths, not globs. Patterns such as `*.env` or `packages/*/node_modules` are not expanded in `.agents/linked`.
*   A folder entry links the whole folder; you don’t need to list its contents.

Put only paths in `.agents/linked`, never secret values. Do not commit the files it links.

### [Does `.agents/linked` need to be committed?](#does-agentslinked-need-to-be-committed)

Commit `.agents/linked` and the `.gitignore` rules for its linked files. Make sure `.agents/linked` and its parent folder aren’t ignored so new workspaces receive the configuration. This is project configuration, not a global or per-user settings file.

Delta can read an uncommitted path list once it is recorded in the current Delta worktree. But an uncommitted copy in your original project folder does not automatically apply to new isolated threads.

Committing the path list shares the configuration, not the linked contents.

### [Where linked files live](#where-linked-files-live)

If you opened the repository from your own checkout, that checkout supplies the shared files. Otherwise, Delta uses a local folder it manages on that machine.

*   If the shared location already has a listed path, Delta links to it.
*   If only the managed checkout has it, Delta copies it to the shared location and replaces the checkout’s copy with a symlink.
*   If neither location has it, listing the path does not create its contents.

If a folder needs copying to the shared location, Delta copies all its contents. Symlinks keep their original targets, so links that depend on a particular folder layout can break after the move. Prefer [setup scripts](#run-setup-in-each-checkout) for dependencies and build output.

Linked contents stay on that machine and are not recorded in thread history. Teammates need their own local files. You cannot use thread history to recover earlier versions of linked files.

## [Run development environments in parallel](#run-development-environments-in-parallel)

Isolated checkouts still use the same machine’s processes and services. Give each development server its own port. Use separate database names, containers, or other resources when one environment’s changes could affect another.

Configure these through your development tools rather than hard-coding the same values for every checkout. Don’t link configuration files that need different values in each environment.

## [Delta-specific overrides](#delta-specific-overrides)

Use `.agents/` for shared project configuration. To customize Delta without changing that shared configuration, use `.delta/`:

Shared configuration

Delta-specific configuration

`.agents/prepare`

`.delta/prepare`

`.agents/linked`

`.delta/linked`

`.agents/skills/`

`.delta/skills/`

For `prepare` and `linked`, the `.delta/` file replaces the corresponding `.agents/` file when present. Only one preparation script runs, and linked-file lists aren’t merged. Empty overrides still take precedence; invalid or failing overrides report errors rather than falling back.

Skills from both folders are available. A valid `.delta/skills/` skill overrides a shared skill with the same frontmatter `name`, regardless of folder name. Other skills remain available, and invalid definitions report load errors without hiding valid shared skills.

Commit overrides that new workspaces need, and keep secrets out of them. The **Create with Delta** action still creates `.agents/prepare`; edit overrides directly.

## [Custom emoji](#custom-emoji)

Add PNG or SVG files directly under `.delta/emoji/` in an attached project. The filename without its extension becomes the shortcode: for example, `.delta/emoji/ship.png` is available as `:ship:`. Type `:ship` in a message or comment and choose it from the picker.

Names are case-sensitive and can contain ASCII letters, digits, `_`, and `-`.

Keep the files in the project’s replicated worktree so collaborators can see them. Commit the images so new checkouts inherit them; ignored files are not shared. If several attached projects define the same name, the first project in attachment order supplies it.

## [What’s next](#whats-next)

*   [Delta Worktrees](/docs/concepts/worktrees) explains isolated and existing checkouts, and what stays on each machine.
*   [Review & Sync Changes](/docs/agents/review-and-sync) explains how to bring code changes into your repository.
*   [Terminals](/docs/agents/terminals) covers running and managing development processes in a thread.
