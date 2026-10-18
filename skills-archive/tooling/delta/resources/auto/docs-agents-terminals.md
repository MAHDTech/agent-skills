---
title: "Terminals"
url: "https://delta.dev/docs/agents/terminals"
description: "Run interactive commands and keep long-running processes active while you work."
---

1.  Using Delta
2.  Terminals

# Terminals

Run interactive commands and keep long-running processes active while you work.

On this page

1.  [Open a terminal](#open-a-terminal)
2.  [How the agent runs commands](#how-the-agent-runs-commands)
3.  [What happens when you send the message](#what-happens-when-you-send-the-message)
4.  [Run a terminal in the background](#run-a-terminal-in-the-background)
5.  [Use terminals in a shared thread](#use-terminals-in-a-shared-thread)
6.  [Manage background terminals](#manage-background-terminals)
7.  [What’s next](#whats-next)

Run commands in your thread’s checkout with an interactive terminal in the desktop app. The terminal starts in that checkout, so commands run against the same files the agent uses. [Delta Worktrees](/docs/concepts/worktrees) explains where that checkout lives.

## [Open a terminal](#open-a-terminal)

At the start of an empty line in the message composer, type `!`. Delta replaces the `!` with an interactive terminal. A `!` typed after other text remains a normal character.

You can also choose **File > Open Terminal**, press `` Ctrl+` `` , or run **Create Inline Terminal** from the command palette.

Type commands and use the terminal as you would use a terminal outside Delta. The terminal remains part of the thread. It runs your system shell; the [Default Shell setting](/docs/configuration/settings#terminal) overrides that for terminals on your machine.

For supported shell code blocks, such as `sh` or `bash`, click **Prefill Terminal** to put the code into your terminal without running it. You can also place the cursor in the code block and press `!`. Review the input and make sure it is compatible with your shell before pressing `Enter` to run it.

## [How the agent runs commands](#how-the-agent-runs-commands)

The agent’s own commands do not use your interactive terminal or the Default Shell setting. On macOS and Linux they run in `/bin/sh`. On Windows the agent has two tools: Git Bash for POSIX commands, which requires Git for Windows, and PowerShell for cmdlets, scripts, and Windows-style command-line tools.

On macOS and Linux, when `direnv` is installed and an `.envrc` applies to the checkout, Delta attempts to load it for interactive terminals and agent commands. Delta authorizes an `.envrc` that `direnv` has not yet allowed; one you have denied with `direnv deny` stays off. Because an `.envrc` runs code from the repository, review it before working in an unfamiliar project. If an `.envrc` applies but `direnv` is unavailable or its setup fails, the terminal still opens and agent commands still run without loading that environment.

The terminal process runs on your computer. Its commands use your checkout, tools, environment variables, and credentials. See [Delta Worktrees](/docs/concepts/worktrees#checkouts-a-delta-worktrees-files-on-a-machine) for details.

When a terminal script changes files, Delta shows the recorded edits as a navigable diff in the thread.

## [What happens when you send the message](#what-happens-when-you-send-the-message)

An interactive terminal starts inside your draft. Sending the message leaves a frozen snapshot in the thread and keeps the local terminal session running. Type `!` again to bring back the same session, including its running process.

Turn on **Background Mode** to keep a process running while you use another terminal in the same thread. The next `!` then opens a new terminal. The sent snapshot does not update while the process continues.

Delta temporarily retains recent output from the continuing process so the agent can inspect what happened after the display froze. See [Background terminal output](/docs/privacy-and-security/security#background-terminal-output) for details about this temporary capture.

## [Run a terminal in the background](#run-a-terminal-in-the-background)

Before you send the message, hover over or focus the terminal block. Click the terminal icon whose tooltip says **Background Mode**. You can also place the cursor inside the terminal and press `Cmd+Option+Z` on macOS or `Ctrl+Alt+Z` on Linux and Windows.

Background mode is useful when a later step depends on a process that must stay running. For example, you can:

*   Start an app server, then ask the agent to test the running application.
*   Keep a file watcher or compiler in watch mode while the agent edits files.
*   Let a long-running build continue while you discuss or review other work.

You do not need background mode for one-off commands such as `ls`, `git status`, or a test command that finishes before you send the message.

## [Use terminals in a shared thread](#use-terminals-in-a-shared-thread)

[Everyone in a shared thread sees](/docs/collaboration/collaborate-thread) a terminal block and its output as they update. The process runs on the computer where you opened it. If you opened the terminal, only you can enter commands in it. Other collaborators can watch its output.

A background process does not move or replicate when another participant sends an agent request. The next turn can interact with a development server or other local process only if its execution environment can reach that process. Seeing the terminal and its output in the shared thread does not make the process available on another machine.

Because drafts are collaborative, another participant can submit a message that contains your terminal. Sending freezes the display in that message, but your local terminal session keeps running. It remains on your computer regardless of who submits the message.

Background terminal status is shared too. Collaborators see the process in their **Background Terminals** control, and anyone in the thread can stop it.

## [Manage background terminals](#manage-background-terminals)

While a background terminal is running, a **Background Terminals** control appears beside the send controls. Open it to see the current thread’s running terminals, identified by their last submitted commands. Open a terminal’s submenu and choose **Scroll to Terminal** or **Stop Process**, or choose **Stop All** to stop every background terminal.

Terminal sessions depend on the Delta app that started them. Quitting Delta or removing the thread stops their processes.

## [What’s next](#whats-next)

*   [Prepare your project](/docs/configuration/project-setup) to install dependencies and run development environments in parallel.
*   [Review & Sync Changes](/docs/agents/review-and-sync) to bring changes from the thread’s checkout into your repository.