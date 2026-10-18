---
title: "Welcome to Delta"
url: "https://delta.dev/docs/getting-started"
description: "Learn your way around Delta by making and reviewing a change in your own project."
---

1.  Getting Started
2.  Welcome

# Welcome to Delta

Learn your way around Delta by making and reviewing a change in your own project.

On this page

1.  [How your first session works](#how-your-first-session-works)
2.  [Ask Delta about Delta](#ask-delta-about-delta)

Delta is a collaborative agent workspace. You describe what you want, an agent works in a checkout of your repository, and you review and pull in the results. Delta creates an isolated checkout by default, or you can use an existing local checkout. This guide covers installing Delta, signing in, and starting your first thread.

*   [Complete your first task in DeltaMake, review, and refine a useful change in one of your projects.Begin the Quick Start](/docs/getting-started/quick-start)
*   [Install DeltaDownload, install, and open Delta.Open the installation guide](/docs/installation)
*   [Learn the core conceptsSee how threads, projects, Delta worktrees, and checkouts fit together.Explore the concepts](/docs/concepts/core-concepts)

## [How your first session works](#how-your-first-session-works)

Delta’s built-in introduction teaches you how to send a message, [comment](/docs/agents/comments) on a specific passage, and open a terminal. The Quick Start then puts those controls to work:

1.  **Open a project.** Delta starts a thread with a separate copy of its files.
2.  **Make a useful change.** Choose a small task, then let the agent edit the relevant files and check its work.
3.  **Review and refine it.** [Inspect the changes](/docs/agents/review-and-sync), comment on the exact part you want changed, and review the revision.

[Core Concepts](/docs/concepts/core-concepts) explains why Delta keeps the thread, project, and separate working copy together.

## [Ask Delta about Delta](#ask-delta-about-delta)

In any thread, mention `@Delta` in your message to ask how Delta works: where a setting lives, what a shortcut is, how to share a thread. Type `@` and choose **Delta** from the menu, or type `@Delta` directly.

The mention attaches Delta’s built-in help instructions and a guide to the installed release to that one message. Everything else about the request is unchanged: it goes to the thread’s selected model through your own provider, and in a regular thread the agent keeps its usual access to your project and tools. Only the reply to that message is marked as coming from Delta; the next reply returns to the normal agent. Send `@Delta` on its own and it asks what you want to do and lists common topics.

When the bundled guide lacks an answer, `@Delta` can consult the online documentation, and if the website describes a newer release it suggests **Check for Updates**.