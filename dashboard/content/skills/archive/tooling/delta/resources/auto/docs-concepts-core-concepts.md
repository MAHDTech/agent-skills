+++
title = "docs-concepts-core-concepts"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Core Concepts"
url: "https://delta.dev/docs/concepts/core-concepts"
description: "What thread, project, and Delta worktree mean in Delta, and how each compares to the meaning you already know."
---

1.  How Delta Works
2.  Core Concepts

# Core Concepts

What thread, project, and Delta worktree mean in Delta, and how each compares to the meaning you already know.

On this page

1.  [Threads: where work happens](#threads-where-work-happens)
2.  [Projects: what you give the agent](#projects-what-you-give-the-agent)
3.  [Delta worktrees: where the work lives](#delta-worktrees-where-the-work-lives)
4.  [How they fit together](#how-they-fit-together)
5.  [What’s next](#whats-next)

## [Threads: where work happens](#threads-where-work-happens)

Delta organizes agent-assisted work into threads. A thread is a conversation with an agent. It can stand alone or include one or more projects. You describe what you want in plain language and steer the work as it develops.

If “thread” makes you think of Slack or a ChatGPT conversation, keep the picture of a conversation and enlarge it: when a Delta thread includes a project, it also owns working copies of the files it touches. The conversation and file changes are recorded together, in order. That combination is what makes a thread the basic unit of work in Delta:

*   **A thread persists.** Leave and come back later; the conversation and any project files are still there.
*   **Threads run in parallel.** When threads use projects, each thread works in its own isolated copy by default. Several agents can work on the same project at once without stepping on each other.
*   **Revert any thread to an earlier point.** The conversation and the thread’s project files rewind together. In a chat app, history is messages. In Delta, history can include messages and edits.
*   **A thread is shareable.** Invite collaborators, and they see the same conversation and any project files, live.

[Threads](/docs/agents/threads) covers starting, steering, and managing them.

## [Projects: what you give the agent](#projects-what-you-give-the-agent)

When you want an agent to work with files, you add a project to the thread. A project is the folder you open:

*   The folder must be the top-level folder of a Git repository with a configured remote. If it has several remotes, one must be named `origin`.
*   Delta imports every file Git tracks, plus untracked files Git does not ignore, into DeltaDB so it can record changes as they happen.

Your repository stays a normal git repository: commits, branches, and remotes work as before. Untracked files ignored by git, often build artifacts and `.env` files, stay on the machine they are on.

[Delta & Git](/docs/concepts/delta-and-git) covers what Delta records and how it connects to git.

## [Delta worktrees: where the work lives](#delta-worktrees-where-the-work-lives)

The agent makes edits, but by default not in your project folder. When a thread starts work on a project, Delta gives the thread its own Delta worktree: a copy of the project’s files whose history is recorded in DeltaDB. A thread has one Delta worktree per project it works on. Its changes stay separate from the tracked files and checked-out branch in your project folder until you bring the work in. Delta labels this default setup **Isolated Workspace**.

NOTE

Git also has a feature called worktrees: extra checkouts of one local repository. A Delta worktree is a different thing: the shared project history that Delta replicates between participants. **In these docs, “Delta worktree” refers to this shared project history.**

The agent’s file tools can work with a Delta worktree directly. On a desktop, Delta also keeps those files in a folder called a **checkout**, where terminal commands and other tools can use them. Delta keeps the two copies in sync. In the browser, the agent works with the Delta worktree directly.

[Delta Worktrees](/docs/concepts/worktrees) covers managed checkouts, adopting your own folder, and where the agent runs.

## [How they fit together](#how-they-fit-together)

A thread can stand alone. When it includes a project, the pieces connect like this:

```
  thread     a conversation with an agent
     │
     │  works with a project (the Git repository folder you open)
     ▼
  Delta worktree   the project's files and history, recorded in DeltaDB
     │
     │  when mounted on a desktop machine
     ▼
  checkout         the Delta worktree's files, as real files on disk
```

One project can be open in many threads. Each thread normally gets its own Delta worktree, and each mounted desktop client can have a checkout of it.

## [What’s next](#whats-next)

*   [Quick Start](/docs/getting-started/quick-start) walks through your first task.
*   [Threads](/docs/agents/threads) covers starting, steering, and managing threads.
*   [Delta & Git](/docs/concepts/delta-and-git) covers how DeltaDB records work and how it connects to git.
*   [Delta Worktrees](/docs/concepts/worktrees) covers where checkouts live and which machine the agent runs on.
