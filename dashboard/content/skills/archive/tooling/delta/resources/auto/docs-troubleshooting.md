+++
title = "docs-troubleshooting"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Troubleshooting"
url: "https://delta.dev/docs/troubleshooting"
description: "Solutions to common problems with installing and running Delta."
---

1.  Getting Started
2.  Troubleshooting

# Troubleshooting

Solutions to common problems with installing and running Delta.

On this page

1.  [Delta keeps asking to move to Applications](#delta-keeps-asking-to-move-to-applications)
2.  [Delta won’t open a workspace](#delta-wont-open-a-workspace)
3.  [Delta says my project needs a Git remote](#delta-says-my-project-needs-a-git-remote)
    1.  [Several remotes, but none named origin](#several-remotes-but-none-named-origin)
    2.  [An older thread cannot sync or be joined](#an-older-thread-cannot-sync-or-be-joined)
4.  [Delta says WSL projects aren’t supported](#delta-says-wsl-projects-arent-supported)
5.  [Delta asks me to accept the Xcode license](#delta-asks-me-to-accept-the-xcode-license)
6.  [Delta says a worktree’s history is unavailable](#delta-says-a-worktrees-history-is-unavailable)
7.  [The agent can’t reach a model](#the-agent-cant-reach-a-model)
8.  [I opened the same branch, but my earlier changes are missing](#i-opened-the-same-branch-but-my-earlier-changes-are-missing)
9.  [The agent committed, but my repository doesn’t have the changes](#the-agent-committed-but-my-repository-doesnt-have-the-changes)
10.  [A thread says it has diverged from the server](#a-thread-says-it-has-diverged-from-the-server)
11.  [Delta opens the existing app instead of a second instance](#delta-opens-the-existing-app-instead-of-a-second-instance)
12.  [Delta reports a protocol version mismatch](#delta-reports-a-protocol-version-mismatch)
13.  [Delta says my local data needs a newer version](#delta-says-my-local-data-needs-a-newer-version)
14.  [A LaTeX formula shows an error](#a-latex-formula-shows-an-error)
15.  [Find Delta’s logs](#find-deltas-logs)
16.  [Getting help](#getting-help)

## [Delta keeps asking to move to Applications](#delta-keeps-asking-to-move-to-applications)

macOS puts a quarantine flag on downloaded apps. Without clearing it, macOS runs Delta from a randomized read-only location (App Translocation) instead of `/Applications`, which makes Delta prompt you to move it on every launch. Clear the flag after installing:

```
xattr -dr com.apple.quarantine /Applications/Delta.app
```

## [Delta won’t open a workspace](#delta-wont-open-a-workspace)

Delta checks in with Zed’s servers at launch: you need to be signed in, have accepted the Early Access Agreement, and be online for Delta to open a workspace. If Delta blocks at launch, check your network connection and any [proxy settings](/docs/configuration/settings), then sign in again from the Delta menu.

## [Delta says my project needs a Git remote](#delta-says-my-project-needs-a-git-remote)

Delta requires a configured Git remote before you add or attach a project, including in private threads. It uses the remote to identify the repository. A repository created with `git init` alone is not enough.

Run this in your original project folder to check its remotes:

```
git remote -v
```

If it has no remote, create a repository on your Git host. Replace `<repository-url>` below with its SSH or HTTPS URL, then add the remote and push your current branch:

```
git remote add origin <repository-url>
git push -u origin HEAD
```

Then try adding or attaching the project again.

### [Several remotes, but none named `origin`](#several-remotes-but-none-named-origin)

Delta uses the sole source remote regardless of its name. If you have several, one must be named `origin` so Delta knows which identifies the project. Replace `<chosen-remote>` with the name Delta should use:

```
git remote rename <chosen-remote> origin
```

Retry in Delta after renaming it. Delta’s managed-checkout `local` backlink is separate from the source remote; don’t rename that backlink.

### [An older thread cannot sync or be joined](#an-older-thread-cannot-sync-or-be-joined)

If Delta says the thread uses a project with no Git remote, add a remote and push your branch, then continue in a new thread. Preserve any outstanding changes before switching threads. Adding a remote does not recover missing recorded history.

If you’re joining someone else’s thread and Delta says a project was never registered, ask the owner to check its remote and share again, or continue in a new thread. That message can also mean registration failed; it does not confirm that the owner’s repository has no remote.

See [Delta & Git](/docs/concepts/delta-and-git#the-two-remotes) for the roles of the source remote and the `local` backlink.

## [Delta says WSL projects aren’t supported](#delta-says-wsl-projects-arent-supported)

The Windows app cannot open repositories through WSL shares such as `\\wsl.localhost\Ubuntu\home\you\project` or `\\wsl$\Ubuntu\home\you\project`. Clone or copy the repository to a Windows drive, then open that folder in Delta.

## [Delta asks me to accept the Xcode license](#delta-asks-me-to-accept-the-xcode-license)

If Delta shows **Xcode License Agreement Required** on macOS, choose **Open Xcode**, review and accept Apple’s agreement, then return to Delta and choose **Retry**. Delta needs Git to prepare the thread’s worktree, and the pending license agreement prevents Git from running.

## [Delta says a worktree’s history is unavailable](#delta-says-a-worktrees-history-is-unavailable)

In the desktop app, **Failed to prepare the … Delta worktree** can mean Delta cannot load the project’s recorded worktree history. The conversation remains available, but Delta cannot prepare that project’s checkout, and **Open in Zed** is unavailable.

Choose **Retry** once the history is available. Retrying does not recover history that is still missing.

If you need to reattach the project instead, first recover or back up any changes that aren’t available elsewhere. Removing and reattaching keeps the conversation, but does not restore the removed worktree’s recorded state. Open the project’s menu, choose **Remove Project**, then use **Add Project to Thread** to attach it again.

## [The agent can’t reach a model](#the-agent-cant-reach-a-model)

Start with the error shown in the thread. It can identify a connection problem, a provider response error, or an access issue rather than only saying the request failed.

For Delta-hosted models, check the selected organization as well as your account. If access is blocked by a subscription, your role, or the organization’s hosted-model settings, the thread explains the restriction and offers the relevant account or organization action.

If your ChatGPT subscription reaches its usage limit, follow the error’s link to [ChatGPT usage settings](https://chatgpt.com/codex/settings/usage) to check your limits, reset times, and available options.

For an API-key provider, open **Settings > LLM Providers** and check that its key is configured. Add or replace the key there if it was revoked or rotated. Keys stored outside Delta do not update provider settings automatically. On desktop, **Import existing key** appears beneath the key field when Delta detects one it can copy into settings. See [Models & Providers](/docs/agents/models-and-providers).

## [I opened the same branch, but my earlier changes are missing](#i-opened-the-same-branch-but-my-earlier-changes-are-missing)

Use **Go to Thread** to find the thread where you made the changes, then open its **Changes** tab. The search includes archived threads. See [Resume work on a feature](/docs/agents/threads#resume-work-on-a-feature).

Each managed checkout has its own Git repository. Opening the same branch name in a new isolated thread does not bring over the original thread’s uncommitted edits or unpushed commits. Continue in the original thread, or review and commit its outstanding changes, then [transfer the commits](/docs/agents/review-and-sync#bring-changes-in).

## [The agent committed, but my repository doesn’t have the changes](#the-agent-committed-but-my-repository-doesnt-have-the-changes)

By default, a thread works in a Delta-managed checkout: a separate folder under `.delta/worktrees/` with its own [git repository](/docs/concepts/delta-and-git). When the agent commits, the commit lands in that repository, not in your project folder. Files it created look missing from your checkout, and `git log` there shows nothing new, until the commits are pushed to your repository.

The managed checkout has a `local` remote that points at your repository. Push the branch to it, either by asking the agent or from the thread’s terminal (type `!` at the start of an empty line in the composer):

```
git push local <branch-name>
```

If `<branch-name>` is the branch your repository has checked out and your working tree is clean, the push updates it in place. Otherwise, run `git switch <branch-name>` in your repository. To have the agent push after every commit without being asked, add that instruction to your [rules](/docs/configuration/settings#rules). See [Bring changes in](/docs/agents/review-and-sync#bring-changes-in) for the full workflow.

To check which checkout a thread uses, open the project menu and look at the **Work In** section: **Isolated Delta Worktree** is a managed checkout, **Existing Local Checkout** is your own project folder, and **Existing Git Worktree** is a linked Git worktree you selected. **Reveal in Finder** (**Reveal in File Manager** on Linux and Windows) shows the folder on disk.

## [A thread says it has diverged from the server](#a-thread-says-it-has-diverged-from-the-server)

If Delta cannot sync some of a thread’s local changes, for example after losing connectivity, it keeps them rather than dropping them. The thread shows a footer reading **This thread has diverged from the server**, and what you see is a recovery copy named `<thread name> (local changes)` that holds your unsynced work. Choose one of the footer’s actions:

*   **Keep Local Changes in a New Thread** turns the recovery copy into a normal thread of its own. The original thread returns to the version on the server.
*   **Discard Local Changes** deletes the recovery copy and returns you to the original thread, restored to the server version.

Until you choose, the recovery copy stays out of the sidebar and its Delta worktrees are not checked out. If Delta cannot load the thread far enough to create the recovery copy on its own, it offers **Fork & Restore**, which does the same thing, or **Discard Changes**.

## [Delta opens the existing app instead of a second instance](#delta-opens-the-existing-app-instead-of-a-second-instance)

Delta runs one app instance at a time. Opening Delta again sends the request to the running app instead of starting a second instance.

## [Delta reports a protocol version mismatch](#delta-reports-a-protocol-version-mismatch)

When Delta has already downloaded an update, the protocol mismatch alert offers **Restart to Update**. Otherwise, choose **Check for Updates** from the alert.

## [Delta says my local data needs a newer version](#delta-says-my-local-data-needs-a-newer-version)

If Delta shows **Update Delta to open your local data**, choose **Check for Updates** in the alert and update the app. The local data was saved by a newer version of Delta and has not been changed by the failed attempt to open it.

## [A LaTeX formula shows an error](#a-latex-formula-shows-an-error)

If a LaTeX formula fails to render, Delta keeps its source visible and shows the error below it. Use the error to correct the formula, or ask the agent to revise it. This applies to both inline and display formulas.

## [Find Delta’s logs](#find-deltas-logs)

Choose **Delta > Open Log Viewer**, or press `Cmd+Shift+L` on macOS or `Ctrl+Shift+L` on Linux and Windows. Opening it again focuses the existing log viewer.

Delta stores `delta.log` and the previous session’s `delta.log.old` here:

System

Folder

macOS

`~/Library/Application Support/delta/`

Linux

`~/.local/share/delta/`

Windows

`%APPDATA%\delta\`

## [Getting help](#getting-help)

If you hit something not covered here, choose **Send Feedback** in the Help or user menu, or type `/send-feedback` in a thread. In the desktop feedback form, you can enter or correct the model name, attach a screenshot, and select **Include recent logs** to attach logs from the current and previous app sessions, when available. These logs can cover multiple threads and contain file paths, repository names, and other sensitive information. Logs are not included unless you select that option. You can also leave an email address or Discord handle for follow-up.

For freezes, select **Include performance data** in the desktop feedback form to attach diagnostic information about recent hangs. Performance data is included only when you select this option.

To include the current thread, select **Include conversation context and tool output**. This attaches up to 100 KiB of recent messages, system instructions, and tool activity. Reasoning and images are excluded. The option is unavailable when the feedback form has no conversation to attach.

You can also email [delta-beta@zed.dev](mailto:delta-beta@zed.dev). Include your operating system, Delta version, the steps that caused the problem, what you expected, and what happened instead. Remove API keys and other secrets from logs or screenshots before you send them.
