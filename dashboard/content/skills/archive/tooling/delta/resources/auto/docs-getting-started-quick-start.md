+++
title = "docs-getting-started-quick-start"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Quick Start"
url: "https://delta.dev/docs/getting-started/quick-start"
description: "Make, review, and refine a useful change in one of your projects."
---

1.  Getting Started
2.  Quick Start

# Quick Start

Make, review, and refine a useful change in one of your projects.

On this page

1.  [Prerequisites](#prerequisites)
2.  [Open your project](#open-your-project)
3.  [Make a change](#make-a-change)
4.  [Review and refine](#review-and-refine)
5.  [What you just used](#what-you-just-used)

Delta’s built-in **[Welcome to Delta](/docs/getting-started)** thread teaches the basic controls. This guide begins where that introduction ends and puts those controls to work on a real project.

By the end, you will have a small, reviewed change in a separate workspace. The change stays separate from your original project files until you decide to bring it in.

## [Prerequisites](#prerequisites)

Before you begin:

*   [Install Delta](/docs/installation), sign in, and connect a model.
*   Follow the **Welcome to Delta** introduction through its final prompt to open a terminal.
*   Choose an existing Git repository with a configured remote. Use its top-level folder, not a folder inside it. If it has several remotes, one must be named `origin`. See [Git remote setup](/docs/concepts/delta-and-git#the-two-remotes).

IMPORTANT

Adding a project stores its files on your machine and on Delta’s servers. Choose a project you are comfortable uploading. This includes every file Git tracks, along with untracked files Git does not ignore. Learn more in [Data Storage & Deletion](/docs/privacy-and-security/data-storage).

## [Open your project](#open-your-project)

1.  Click **Add Project** in the sidebar.
2.  Choose the top-level folder of the Git repository.
3.  If **Projects Sync to the Cloud** appears, review the notice and click **Continue**.

Delta opens the project in a thread automatically.

You can also choose **Open Git Project…** from the command palette to select the Git repository’s folder.

Before sending, check the project menu for the checkout and branch. A new thread can inherit your choice to work in an existing local checkout; the empty composer tells you when it does. See [Delta Worktrees](/docs/concepts/worktrees#using-the-local-clone-with-delta).

For this Quick Start, open the project menu and, under **Work In**, select **Isolated Delta Worktree**. This gives the thread a separate copy of the project’s files.

You should now see the project attached to the thread with **Isolated Delta Worktree** selected.

## [Make a change](#make-a-change)

The agent may send relevant messages and file contents to your model provider. On a desktop, it can also use tools and resources available to your operating-system account. **Isolated Delta Worktree** separates this thread’s project changes; it does not sandbox the agent. Read [AI Privacy & Telemetry](/docs/privacy-and-security/privacy) and [Agentic Safety](/docs/privacy-and-security/agentic-safety).

Send this message:

```
Fetch https://delta.dev/docs/start.md and guide me through my first Delta
project task.
```

The agent helps you choose a small, useful change and agree on how to check that it works. If you’re unsure where to begin, it can inspect your project and suggest a few options. It then explains its plan, edits the relevant files, and checks the result.

[View the instructions the agent follows](/docs/start.md).

## [Review and refine](#review-and-refine)

When the first pass is ready:

1.  Click the diff icon in the upper-right corner to open the **Changes** panel.
2.  Select the file the agent named to open a before-and-after view.
3.  Select a line or passage you want changed or explained, then start typing to add a comment.
4.  Submit the comment. You should see it attached to the selected text.
5.  Let the agent respond. If you asked for a change, it will revise the file and check the result again.

If you cannot add an inline comment, send the same feedback as a normal message. If the change already looks right, use the comment to ask the agent to explain part of it. This gives you a chance to try commenting without requesting a revision.

You are finished when the result meets the success check you agreed on and the agent has addressed your comment. The request, project files, feedback, and any revisions remain together in the thread, while the change stays in its isolated Delta worktree.

## [What you just used](#what-you-just-used)

What happened

Delta feature

Learn more

Your request, files, and feedback stayed together

A thread

[Threads](/docs/agents/threads)

The work stayed separate from your original project files

A project with its own Delta worktree

[Delta Worktrees](/docs/concepts/worktrees)

You compared the original file with the agent’s changes

Changes panel

[Review the agent’s work](/docs/agents/review-and-sync#review-the-agents-work)

You left feedback on a specific passage and the agent responded

Comments

[Comments](/docs/agents/comments)

When you are ready, [bring the change into your project](/docs/agents/review-and-sync#bring-changes-in) or [invite someone into the thread](/docs/collaboration/collaborate-thread).
