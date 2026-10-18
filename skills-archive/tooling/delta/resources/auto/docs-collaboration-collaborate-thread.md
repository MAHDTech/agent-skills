---
title: "Collaborate in a Thread"
url: "https://delta.dev/docs/collaboration/collaborate-thread"
description: "Share a thread, join a teammate's thread, and work together in real time."
---

1.  Collaboration
2.  Collaborate in a Thread

# Collaborate in a Thread

Share a thread, join a teammate's thread, and work together in real time.

On this page

1.  [Share a thread](#share-a-thread)
    1.  [Choose who can access a thread](#choose-who-can-access-a-thread)
    2.  [Invite people by email](#invite-people-by-email)
2.  [Join a thread](#join-a-thread)
3.  [Work in a shared thread](#work-in-a-shared-thread)
    1.  [Chat with collaborators](#chat-with-collaborators)
4.  [Mention someone](#mention-someone)
5.  [Who pays for model usage](#who-pays-for-model-usage)
6.  [Where the agent runs](#where-the-agent-runs)

Share a thread to work with teammates in the same conversation. Everyone sees messages, edits, comments, and agent activity as they happen, and everyone can steer the agent.

A projectless thread has no Delta worktree or checkout. When a shared thread includes projects, Delta synchronizes their Delta worktrees through [DeltaDB](/docs/concepts/delta-and-git#what-is-deltadb). Desktop participants can mount checkouts; browser participants use the replicated files directly. See [Delta Worktrees](/docs/concepts/worktrees).

## [Share a thread](#share-a-thread)

Select the share icon in the upper-right corner, or press `Cmd+Shift+S` on macOS or `Ctrl+Shift+S` on Linux and Windows. You can also choose **File > Share Thread**, run **Share Thread** from the command palette, or right-click a thread in the thread list. Choose who can access the thread, then copy and send the link. Opening a thread to **Anyone with the link** asks for confirmation first.

Projectless threads can be shared without a Git repository. Adding or attaching a project already requires a configured Git remote, even for private threads. With several source remotes, one must be named `origin`.

If an older thread has a project without a remote, add a remote and push your branch. If Delta says the thread cannot sync or be joined, continue in a new thread after fixing the project’s remote. See [Git remote troubleshooting](/docs/troubleshooting#delta-says-my-project-needs-a-git-remote).

The link opens at your current position in the thread.

To point someone to a passage, select text in the conversation, a file, or a diff and choose **Copy Link to Selection** from the context menu. You can also link to selected comments. Press `Cmd+Option+L` on macOS or `Ctrl+Alt+L` on Linux and Windows to copy the selection link. With no selection, use **Copy Link to This Position**. The link focuses that passage or position when opened. It does not grant access; the recipient still needs access to the thread.

After sharing, you can also run **Copy Share Link** to copy a link to your current position. Press `Cmd+Option+L` on macOS or `Ctrl+Alt+L` on Linux and Windows.

In the desktop app, copying a private thread’s link shows a warning that only people with access can open it. Choose **Give Access** in the warning to open the sharing controls. Copying the link does not change access by itself.

### [Choose who can access a thread](#choose-who-can-access-a-thread)

Everyone in the thread can see the access setting and the member list and copy the link; only the thread’s owner can change who has access.

Access

Who has access

Only people invited

You and anyone who already has access

Everyone at _organization name_

Members of the selected organization

Anyone with the link

Any signed-in Delta user who has the link

> **Note:** Changing a shared thread to **Only people invited** prevents additional people from joining. People who already have access remain participants.

To choose the access level for new threads, open **[Settings](/docs/configuration/settings) > Thread > Sharing** and change **Default Visibility**. **Private** is the default; **Your organization** automatically shares new threads with everyone in your active organization.

> **Warning:** Treat an **Anyone with the link** URL as sensitive. Anyone with the URL can pass it on. After signing in, they can access the thread and the projects it references.

### [Invite people by email](#invite-people-by-email)

The thread’s owner can invite specific people by email, up to ten at a time. An invitation gives each recipient access without making the thread available to everyone in an organization or anyone with its link. The recipient must sign in with a Zed account to view the thread; they join as an editor. Invitations expire after fourteen days.

## [Join a thread](#join-a-thread)

Open the link and sign in with your Zed account. On a desktop browser, the link launches Delta, which opens the shared thread and creates isolated checkouts for any attached projects, even if you already have the repositories on your machine. You do not need to clone them first. If Delta does not open, or you do not have it installed, choose **Open on web** to read and take part in the thread in your browser. The browser uses the replicated Delta worktrees directly, without checkouts. On a phone or tablet, the link opens in the browser directly. See [Delta on the Web](/docs/collaboration/delta-on-the-web).

To join from inside Delta, choose **File > Join Thread** or run **Join Thread** from the command palette. Paste either a complete Delta join URL or its bare share ID, then choose the matching join action. Join Thread has no default keyboard shortcut.

To find a thread shared with your organization without asking for its link, browse [delta.dev/threads](https://delta.dev/threads). See [Delta on the Web](/docs/collaboration/delta-on-the-web#browse-threads).

## [Work in a shared thread](#work-in-a-shared-thread)

Everyone in a thread can send messages, steer the agent, edit files, and comment. Each participant composes their own draft, attributed to them; you can edit someone else’s draft, and sending submits every trailing draft under its own author.

The agent sees who wrote each message and comment. When someone sends, all pending drafts at the end of the thread are submitted together.

To discuss a specific part of the conversation, add a [comment](/docs/agents/comments). Anyone can comment, reply, and see the agent’s linked responses.

### [Chat with collaborators](#chat-with-collaborators)

Drafts are visible to everyone in the thread as they are typed, so you can talk to a collaborator without involving the agent. Move to the end of the thread and type. When the last draft belongs to someone else, yours starts after it, with your avatar, and the exchange reads like a chat. Drafts are part of the thread: they sync to every participant, stay if you close and reopen the thread, and anyone can edit or delete them.

Nothing reaches the agent until someone sends. Sending then submits every draft at the end of the thread, from every author, and the agent responds to all of them in one turn charged to the sender. There is no way to send a message to collaborators only.

> **Tip:** Finish the discussion before anyone sends, and delete drafts the agent does not need so it responds only to the request.

To follow a collaborator as they move between the conversation, review diffs, files, subagents, and subthreads, click their avatar and choose the **Follow** action. You can also press `Ctrl+Option+Cmd+F` on macOS or `Ctrl+Alt+Shift+F` on Linux and Windows to follow the next collaborator.

Following tracks subthreads opened in tabs or as the primary conversation, as well as zooming and panning an image.

For a separate review conversation, create a [review thread](/docs/agents/review-and-sync#review-in-a-review-thread). Anyone with access to the parent can open it, comment on the transcript or diff, and submit an approval or change request back to the parent thread, where verdicts show as badges on the reviewers’ avatars. A review thread shares whatever its parent shares: its header’s **Copy Share Link** button links straight to the review once the parent is shared.

## [Mention someone](#mention-someone)

Type `@` in a message or comment and choose a person from the suggestions. Delta notifies them in their Inbox as soon as you choose them, **before** you send the message or comment to the agent. If you type their full `@username` without choosing a suggestion, Delta waits until you send to notify them. Someone who already has the thread open does not get a separate Inbox notification.

Mentioning someone can also share a private thread with them so they can open it. Check the recipient before choosing them: removing an unsent mention does not retract a notification or invitation already sent. On desktop, open the Inbox with `Cmd+Shift+J` on macOS or `Ctrl+Shift+J` on Linux and Windows. The [browser Inbox](/docs/collaboration/delta-on-the-web#inbox-and-notifications) also shows invitations and mentions. On desktop, opening a thread marks its mentions read. In the browser, opening a notification marks that notification read. On either platform, a notification for a mention in a file or diff comment opens that comment.

## [Who pays for model usage](#who-pays-for-model-usage)

Model usage belongs to whoever sends the request, not the thread’s owner. If you use a hosted model, usage counts against your Zed plan or seat. If you connect your own API key or provider subscription, usage goes through that account.

Editing files and commenting do not consume model usage on their own.

Each participant must have access to the thread’s selected model or an available equivalent. See [Models & Providers](/docs/agents/models-and-providers#choose-a-model).

## [Where the agent runs](#where-the-agent-runs)

An agent turn runs wherever the sender is using Delta, not on the thread owner’s device or another participant’s device.

*   In the desktop app, a turn can use mounted checkouts, installed tools, environment variables, credentials, network access, and running processes.
*   In the browser, a turn works with replicated thread and project data. It has no shell or checkout and cannot access those resources on the sender’s device.

Delta syncs the conversation and recorded file changes, but it does not sync machine-local runtime state. If collaborators alternate between execution environments, the resources available to the agent can therefore change between turns.

For example, a turn on your teammate’s computer cannot access a development server, local database, application process, or credentials available only on your computer. For workflows that depend on those resources, have the participant whose machine hosts them send the dependent requests, or make the resources available to each execution environment.

Delta syncs resulting file changes to the Delta worktree. Mounted desktop checkouts receive those changes; browser participants read them directly. [Delta Worktrees](/docs/concepts/worktrees) explains how checkouts sync across machines.