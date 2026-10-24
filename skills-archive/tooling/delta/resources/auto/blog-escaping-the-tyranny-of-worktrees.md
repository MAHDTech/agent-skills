---
title: "Escaping the tyranny of worktrees — Delta Blog"
url: "https://delta.dev/blog/escaping-the-tyranny-of-worktrees"
description: "Parallel agent work has outgrown Git worktrees."
---

Blog

# Escaping the tyranny of worktrees

Conrad IrwinOctober 6, 2026

As agentic work has become more common, and agents remain extremely slow, people keep reaching for Git worktrees to make progress on a bunch of things in parallel.

In Git-land, this is a somewhat reasonable compromise: Git worktrees are the main tool Git offers for checking out the same repository in multiple places. You can leave an agent running in one, review a change in another, and keep using your normal checkout without everyone overwriting the same files.

But they come with problems:

1.  Git does not know when you are finished with them, so you have to clean up the worktrees yourself or add more tooling to manage their lifecycle.
2.  You can only check out a branch in one worktree at a time.
3.  Therefore, merges and rebases become tricky. The branch you need may already be checked out somewhere else, and now you have to remember which directory owns which branch.
4.  Every tool that associates state with a directory gets another copy to manage: environment files, dependencies, build output, language servers, development servers, and databases.

The filesystem starts to fill up. Branch names accumulate. Some worktrees are useful, some are abandoned, and some contain changes that you are fairly sure mattered to… someone.

Git worktrees are fine when one person is juggling a few branches. Add a handful of agents, and suddenly keeping track of worktrees is its own job.

## [A Delta worktree is not a Git worktree](#a-delta-worktree-is-not-a-git-worktree)

Delta sidesteps these limitations with its own implementation of worktrees.

Each thread in Delta has its own [Delta worktree](/docs/concepts/worktrees), with your project’s files and recorded history in DeltaDB. A checkout is the directory where Delta materializes that worktree on a particular machine while it’s in use.

![Creating a new thread in Delta gives you an isolated Delta worktree by default.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/escaping-the-tyranny-of-worktrees/img-2-l.webp)![Creating a new thread in Delta gives you an isolated Delta worktree by default.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/escaping-the-tyranny-of-worktrees/img-2-d.webp)

Creating a new thread in Delta gives you an isolated Delta worktree by default.

The directory is temporary. The worktree is not tied to it. DeltaDB records changes continuously and replicates them between the people, agents, and machines working in a thread. When someone needs ordinary filesystem access, Delta mounts that state into a checkout. When they no longer need it, the checkout can go away.

Git still has an important role: commits are Git commits, branches are Git branches, and `origin` is still where work is published. DeltaDB does not replace Git history — it adds more detail. Live changes can be synced in real time, and we know character by character how the codebase evolved.

Each workspace has its own Git clone (though we use some tricks to make them cheap). This lets you have one agent implementing a change on a branch, another reviewing it, and another updating the documentation, all working on the same branch. Delta can merge the changes back together before committing and pushing to Git.

![The filesystem is no longer the only place where work exists in Delta.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/escaping-the-tyranny-of-worktrees/img-1-l.webp)![The filesystem is no longer the only place where work exists in Delta.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/escaping-the-tyranny-of-worktrees/img-1-d.webp)

The filesystem is no longer the only place where work exists in Delta.

## [Parallel work without branch choreography](#parallel-work-without-branch-choreography)

The immediate benefit is that starting another piece of work becomes cheap.

You do not have to decide up front whether a bug fix deserves a permanent branch and directory. You can start a thread, let an agent investigate, and keep the useful ones. The unsuccessful experiments no longer become archaeology projects in `../project-wip-7`.

You can also work with other people in real time, get their input and expertise, and let their agents work on your branch. Because Delta records edits as they happen, you do not need to branch, commit, and push every time you want to share your work in progress. Because the worktree doesn’t need to be mounted to disk, you can review and edit [in a browser](/docs/collaboration/delta-on-the-web).

And when it is time to publish your work, the normal Git tools still work. You can switch branches, merge, rebase, and push from a managed checkout. Merge conflicts remain merge conflicts, though luckily agents don’t seem to mind them, but the Git worktree restriction that a branch can only be checked out once no longer gets in the way.

## [Temporary checkouts require durable setup](#temporary-checkouts-require-durable-setup)

Delta worktrees do not solve all the problems.

Working well this way requires embracing the transience of checkouts. If the development environment only works after someone has spent an afternoon carefully crafting configuration files, using an isolated checkout for everything is going to get tiring fast.

While Delta can adopt your existing local checkout when an agent needs its exact current state, this gives up isolation. It should be the exception: isolated checkouts are automatic, cheap, and safer when several agents are working at once.

To make your setup work well, you may need to do a few things:

### [Make setup repeatable](#make-setup-repeatable)

Delta runs a committed, executable [`.agents/prepare`](/docs/configuration/project-setup) script from the checkout root when it mounts a managed checkout, before the agent starts. It needs to be fast (because it’ll run on every thread creation), but it gives you a flexible way to initialize any configuration you need.

![A committed preparation script keeps temporary checkouts reproducible.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/escaping-the-tyranny-of-worktrees/img-3-l.webp)![A committed preparation script keeps temporary checkouts reproducible.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/escaping-the-tyranny-of-worktrees/img-3-d.webp)

A committed preparation script keeps temporary checkouts reproducible.

This is useful outside Delta, too. A repository that can prepare itself from a clean checkout is easier for new teammates to use and less likely to depend on state that nobody remembers creating.

You probably don’t want to install dependencies here. That tends to be too slow and many threads won’t need them. But you may also want to look into ensuring that your build cache is shared across all builds on your machine (for example by using pnpm instead of npm, if you’re not already).

If you want a config file that is Git-ignored to be shared by all threads, you can add it to [`.agents/linked`](/docs/configuration/project-setup). Delta symlinks those paths to shared locations on your machine, so all its checkouts on that machine see the same values.

### [Stop treating ports as constants](#stop-treating-ports-as-constants)

Separate checkouts do not create separate computers.

They still share the machine’s ports, processes, containers, caches, databases, and other mutable resources. If two agents both run a server hard-coded to port `3000`, one of them is going to have a bad time.

Projects that want to run several workspaces concurrently need to make these resources configurable. Development servers should accept different ports. Tests may need distinct database names. Containers may need separate project names or volumes.

## [Where did I put that agent?](#where-did-i-put-that-agent)

With Git worktrees, the directory becomes the durable object. You name it, remember it, and eventually delete it. In Delta, the thread and its recorded work are durable; the checkout is temporary.

This means you can start an investigation before deciding that it needs a branch, archive it without leaving another directory behind, and give several agents isolated checkouts without coordinating who owns which branch. Following an idea no longer means interrupting everything else that is already in progress.

Parallel agentic work should not mean managing a pile of worktrees. In Delta, isolation is automatic: press `cmd-n` and start the next thing.

You can [try Delta](/download) and read more about [how Delta and Git work together](/docs/concepts/delta-and-git).

## More from the blog

*   ### [Beyond Pass Rate: Harness Evals, and Their Blind Spots](/blog/beyond-pass-rate)
    
    ![Anant Goel](https://avatars.githubusercontent.com/u/3103248?v=4 "Anant Goel")Sep 24, 2026
    
*   ### [Please Stop Sending Me Google Docs](/blog/please-stop-sending-me-google-docs)
    
    ![Franciska Dethlefsen](https://avatars.githubusercontent.com/u/10458212?v=4 "Franciska Dethlefsen")Oct 8, 2026
    
*   ### [The Diff: September 2026](/blog/the-diff-2026-09)
    
    ![Joseph Lyons](https://avatars.githubusercontent.com/u/19867440?v=4 "Joseph Lyons")Oct 7, 2026
    

[Download Delta Now](/download)[Open Delta](/threads)