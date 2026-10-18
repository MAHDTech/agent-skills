+++
title = "docs-concepts-worktrees"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Delta Worktrees"
url: "https://delta.dev/docs/concepts/worktrees"
description: "Where the agent runs, and how files are managed."
---

1.  How Delta Works
2.  Delta Worktrees

# Delta Worktrees

Where the agent runs, and how files are managed.

On this page

1.  [Checkouts: a Delta worktree’s files on a machine](#checkouts-a-delta-worktrees-files-on-a-machine)
2.  [Repositories: how we connect Delta to Git](#repositories-how-we-connect-delta-to-git)
3.  [Where new threads start](#where-new-threads-start)
4.  [Configuring your checkouts](#configuring-your-checkouts)
5.  [Using the local clone with Delta](#using-the-local-clone-with-delta)

Delta takes care of managing checkouts and worktrees on your behalf.

Each thread is paired with one (or more) DeltaDB worktrees that track changes to your files. These worktrees exist inside DeltaDB, which lets agents read and edit files from the web. Commonly, for programming tasks, you need the contents of your worktrees synced to disk, and DeltaDB uses checkouts for this.

## [Checkouts: a Delta worktree’s files on a machine](#checkouts-a-delta-worktrees-files-on-a-machine)

Each participant in a thread gets their own checkout of a worktree on their own machine. When you send a message, the agent runs on your machine using your checkout. When a collaborator sends a message, the agent runs on their machine using their checkout. DeltaDB takes care of ensuring that all changes are synced between the two in real time.

These checkouts are managed automatically. When you archive a thread in Delta, we remove its managed checkout from your disk, and we re-create it on demand when you open an old thread, as long as its recorded worktree history is available. Existing checkouts you choose to work in are not removed. See [worktree recovery](/docs/troubleshooting#delta-says-a-worktrees-history-is-unavailable) if Delta cannot prepare the checkout.

> **Note:** Delta does not use git worktrees for its managed checkouts, despite the similar name, to avoid limitations around how many threads can work on one branch.

## [Repositories: how we connect Delta to Git](#repositories-how-we-connect-delta-to-git)

There are two ways to connect a git repository to Delta:

*   You can open a folder containing a git repository from your local disk.
*   You can join a shared thread that gives you access to a git repository.

Your repository needs a configured Git remote before you add or attach it to a thread, not only when sharing. With one source remote, its name can be anything. With several, one must be named `origin`. Delta’s `local` backlink does not count as a source remote. See [Git remote setup](/docs/concepts/delta-and-git#the-two-remotes).

When we create a checkout of a worktree, we configure [git remotes](/docs/concepts/delta-and-git) that the agent can push to. For checkouts created from your own project folder, the source remote keeps its original name, often `origin`. Additional Git remotes are preserved too, so agents can use both your fork and upstream without reconfiguring them. If you have opened a folder on your local disk with the same git remote, then we also configure the `local` remote to point to that folder; so you can ask the agent to push to it if needed.

The location of the managed checkouts on disk depends on whether or not you have a local clone of the repository connected to Delta. If you do, the checkouts will be nested inside a `.delta` directory in that clone; if you do not then they are in a fully-managed directory. To discover the location of a checkout, click on the project name at the bottom of the window.

## [Where new threads start](#where-new-threads-start)

Your **primary checkout** is the local repository folder you opened in Delta. An **isolated workspace** is a separate checkout managed by Delta for a thread.

When you create a new isolated thread in the desktop app, Delta attempts to fetch the repository’s remote and uses its default branch as the starting point, unless you configured another branch for new worktrees. The branch does not have to be named `main`, and your primary checkout does not need to be on it.

To use another branch, open a project’s menu while its checkout is on that branch and choose **Make default branch for Delta worktrees**. Future worktrees start from it when available. The menu shows **Default for Delta worktrees:** with the chosen branch; clear that choice to return to the repository default.

This fetch retrieves commits without pulling changes into your primary checkout’s current branch or updating its working files. If fetching fails, Delta can start from locally available commits, so the new thread may not have the latest remote changes.

If you choose to [work in your existing local checkout](#using-the-local-clone-with-delta) or an existing linked Git worktree instead, the thread uses that checkout’s current branch and files.

## [Configuring your checkouts](#configuring-your-checkouts)

When Delta creates a new checkout of a worktree it also runs the script at `.agents/prepare` (or `.delta/prepare` if one exists). This script should configure the new checkout so that the agent can get to work.

The script is run frequently (every time you create a new thread, or a worktree is re-created) and so it should be fast. Common things to do are:

*   Configuring builders and package managers to use a shared cache
*   Linking git or gerrit hooks
*   Initializing configuration files

> **Warning:** You should avoid doing anything expensive in `.agents/prepare` like running a download or a build step, otherwise Delta will feel very slow to use.

For the common case where all the script needs to do is create symlinks, and you have an existing local clone of the repository, we also support a manifest file at `.agents/linked` (or `.delta/linked`). Files listed in this manifest are symlinked to a shared location so that changes in one checkout are visible in all the others.

These files must be ignored by Git, which also excludes them from change-tracking in DeltaDB.

For example if you use a `.env` file for local configuration you might do:

```
# .agents/linked
.env

# .gitignore
.env
```

See also [Prepare your project](/docs/configuration/project-setup).

## [Using the local clone with Delta](#using-the-local-clone-with-delta)

Although we strongly recommend letting Delta manage worktrees for you, we do allow a limited escape hatch. When you start a new thread in the desktop app, you can open the project menu and choose **Existing Local Checkout**. This will make the agent work in the local checkout on your machine instead of an isolated worktree.

> **Warning:** Threads that use the same existing checkout are not isolated. Edits made in either thread appear in the shared Delta worktree, and agents running at the same time can modify the same files.

> **Note:** The project’s **Work In** menu also lets you choose **Existing Git Worktree** to work in a linked git worktree you already created, or **Isolated Delta Worktree** for the default managed checkout.

Choose the checkout before the agent uses that project; afterward, the location is locked. A new thread created from the current thread keeps its selected checkout.
