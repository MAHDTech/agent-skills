+++
title = "docs-agents-threads"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
+++

---
title: "Threads"
url: "https://delta.dev/docs/agents/threads"
description: "Start, steer, and manage agent threads."
---

1.  Using Delta
2.  Threads

# Threads

Start, steer, and manage agent threads.

On this page

1.  [Start a thread](#start-a-thread)
    1.  [Attach text and insert symbols](#attach-text-and-insert-symbols)
    2.  [Ask the agent to start or fork a thread](#ask-the-agent-to-start-or-fork-a-thread)
2.  [Send a message to a subthread](#send-a-message-to-a-subthread)
    1.  [Find and open child threads](#find-and-open-child-threads)
3.  [Resume work on a feature](#resume-work-on-a-feature)
4.  [Organize threads](#organize-threads)
    1.  [Tag and filter threads](#tag-and-filter-threads)
5.  [Send information to another agent](#send-information-to-another-agent)
6.  [Edit a sent message](#edit-a-sent-message)
7.  [Revert a conversation](#revert-a-conversation)
8.  [Mention files and folders](#mention-files-and-folders)
9.  [Open files from the transcript](#open-files-from-the-transcript)
10.  [Search and navigate](#search-and-navigate)
     1.  [Bookmark a turn](#bookmark-a-turn)
11.  [Thread lifecycle](#thread-lifecycle)
12.  [Organize projects in the sidebar](#organize-projects-in-the-sidebar)
13.  [Archive and reopen threads](#archive-and-reopen-threads)
14.  [Skills](#skills)
15.  [Subagents](#subagents)
16.  [What’s next](#whats-next)

A thread is a conversation with an agent and the basic unit of work in Delta. It can stand alone or include one or more projects. Each project it uses has a [Delta worktree](/docs/concepts/worktrees).

## [Start a thread](#start-a-thread)

Choose **File > New Empty Thread** to start without a project, or open a project to start a thread with its files. Describe what you want in plain language. In a project thread, the agent can read files, make edits, and run commands in the checkout selected for your project.

When you start a project thread in the desktop app using **Isolated Delta Worktree**, Delta automatically attempts to fetch the repository’s remote and starts from your configured worktree branch, or the repository’s default branch. You don’t need to pull changes into your primary checkout first. See [how Delta chooses your starting code](/docs/concepts/worktrees#where-new-threads-start).

Press `Tab` to indent ordinary composer text to the next two-column stop and `Shift+Tab` to outdent it. Markdown lists keep their structural indentation behavior.

### [Attach text and insert symbols](#attach-text-and-insert-symbols)

Paste a large block of text into the composer to attach it as a **Pasted text** file. The agent can read the attachment; click its link to open it in a tab.

Type `:` followed by a name in a message or comment to choose an emoji or mathematical symbol. Attached projects can also supply [custom emoji](/docs/configuration/project-setup#custom-emoji).

### [Ask the agent to start or fork a thread](#ask-the-agent-to-start-or-fork-a-thread)

In the desktop app, ask the agent to start a separate thread and describe its task. The new thread uses isolated Delta worktrees, so you can work on independent tasks in parallel. It starts with a fresh conversation and does not inherit this thread’s edits. To disable this capability, turn off **Agent Thread Creation** in **Settings > Thread > Rules**; subagent delegation remains separate.

To continue from an existing conversation, explicitly ask the agent to fork it and provide its thread ID or shared link. The fork starts from the source thread’s recorded conversation and worktree state, then advances independently. It does not merge its changes back automatically.

## [Send a message to a subthread](#send-a-message-to-a-subthread)

To work on a task in a separate conversation, write your message in a thread or an existing subthread, then press `Cmd+Shift+Enter` on macOS or `Ctrl+Shift+Enter` on Linux and Windows. Delta opens a new subthread and runs the request there, not in the parent. The sent request stays in the parent’s history.

The subthread inherits the parent’s conversation at the point it starts. That context is background for your new request, not an instruction to finish the parent’s tasks. To continue work from the parent, ask for it explicitly.

If you turned [Send With Modifier](/docs/configuration/settings#thread) off, use `Cmd+Enter` on macOS or `Ctrl+Enter` on Linux and Windows instead. You can also hold the same modifier keys while clicking the send button. These gestures work in the desktop app and on the web.

Use the same shortcut with an empty composer, or type `/subthread` and select the command, to open a blank subthread. Write your task in the new conversation. Click the subthread’s title in its tab toolbar to rename it.

In the desktop sidebar, use **Open in Tab** to open a subthread alongside its parent. Closing its tab does not stop its running work. You can also right-click a subthread to **Rename**, **Mark as Read**, or **Mark as Unread**. Press `Cmd+1` on macOS or `Ctrl+1` on Linux and Windows to return focus to the parent conversation without closing the subthread tab.

You can include a [skill](/docs/agents/skills) in the message. If the skill picker is open, the subthread shortcut accepts the highlighted skill and sends the complete draft to the new conversation. Pressing `Enter` in the picker only inserts the skill.

Subthreads you create this way do not automatically post progress, success, or blockers in the parent conversation. Ask the child agent to [send a message to the parent](#send-information-to-another-agent) when you need an update. Agent-created [subagents](/docs/agents/subagents) return their final result automatically, and Land thread results also appear in the parent.

The child’s [Delta worktrees](/docs/concepts/delta-and-git) start as copies of the parent’s. After that, the parent and child can advance independently; file changes do not sync automatically in either direction. To bring the child’s edits back, ask the parent agent to merge that subthread’s changes, as with a [review thread](/docs/agents/review-and-sync#review-in-a-review-thread). Finishing the child conversation does not merge its edits automatically. New [isolated subagents](/docs/agents/subagents#profiles) also require an explicit merge.

To give a subthread the main conversation area, focus its tab and run **Open as Primary** from the command palette. This works on desktop and web. On desktop, you can also open a subagent as primary.

New subthreads include a link back to the point where they forked from the parent.

Subthreads can contain nested subthreads. A browser turn still has the [browser’s tool limitations](/docs/collaboration/delta-on-the-web#choose-where-to-run-a-request).

### [Find and open child threads](#find-and-open-child-threads)

On desktop and web, run **View All Subthreads** or press `Cmd+J` on macOS or `Ctrl+J` on Linux and Windows to browse the parent’s children. Filter by kind, and press `Tab` to show or hide archived children.

In the desktop sidebar, click a subthread to open it as the primary conversation. Hold `Cmd` on macOS or `Ctrl` on Linux and Windows while clicking to open it in a tab instead. Double-click its name to rename it. When a subthread is primary, press `Cmd+Option+U` on macOS or `Ctrl+Alt+U` on Linux and Windows to return to its parent.

## [Resume work on a feature](#resume-work-on-a-feature)

Use **Go to Thread** to find the thread where you started the work. The search includes [archived threads](#archive-and-reopen-threads). Open the thread’s [Changes tab](/docs/agents/review-and-sync#review-the-agents-work) to see where you left off, or ask the agent for a summary.

The thread keeps the conversation and recorded file changes together, including edits you haven’t committed. Starting a new isolated thread on the same branch does not resume that work or bring over its uncommitted edits or unpushed commits. See [Why can two workspaces use the same branch name?](/docs/concepts/worktrees#checkouts-a-delta-worktrees-files-on-a-machine).

To make the thread easier to find next time, use **Rename Thread** to give it a feature or issue name. You can match the branch name, but Delta doesn’t require them to match. On desktop, you can also double-click the thread’s name in the sidebar to rename it.

Start a new isolated thread for independent work. To collaborate on the existing work, [share the original thread](/docs/collaboration/collaborate-thread).

## [Organize threads](#organize-threads)

In the desktop sidebar’s filter menu, choose **Group by** to group threads by **Project**, **Time**, **State**, or **Owner**. Under **Sort by**, choose **Creation Time**, **Last Prompted**, or **Manual**. Manual sorting is not available with Time grouping or in thread history. Time grouping and thread history group older threads by month.

Under **Subthreads**, choose which children to show: **Plain**, **Reviews**, **Land**, and **Subagents**. An expanded parent shows the six newest matching children across these kinds. Choose **Show N more** to reveal the rest.

To bookmark the current thread on desktop, press `Cmd+D` on macOS or `Ctrl+D` on Linux and Windows. Choose **Show Bookmarked Threads** from the sidebar or menu bar to find bookmarked threads, including archived ones. On macOS, you can also press `Cmd+Option+B`; Linux and Windows have no default shortcut for this view.

Right-click a thread and choose a color from its context menu to make it easier to recognize. You can also set or clear a color for several selected threads at once. Nested threads and subagents inherit their parent’s color. Colors organize your local sidebar; they are not shared with collaborators.

### [Tag and filter threads](#tag-and-filter-threads)

On desktop, right-click a thread and open **Tags**, or run **Edit Thread Tags** from the command palette. Assign tags to organize related work, then use **Tags** in the sidebar filter menu to narrow the list. The Bookmarks view also has filters for tags, ownership, and which subthreads to show.

## [Send information to another agent](#send-information-to-another-agent)

Ask the agent to send a message to another thread and `@`\-mention the destination, for example: “Send your findings to @Investigate sync errors.” From a subthread, you can ask it to send the findings to the parent.

The message appears as a **Sent Agent Message** in the source and a **Received Agent Message** in the destination. Expand the event to read it. The receiving agent handles it during its current response or starts a new response if idle. Sending a message does not merge file changes between threads.

## [Edit a sent message](#edit-a-sent-message)

You can edit one of your earlier messages in place. Sending the edit replaces the conversation after that message: later responses and follow-ups are discarded, and the agent answers the edited message fresh. Use this to rework a prompt that sent the agent in the wrong direction.

## [Revert a conversation](#revert-a-conversation)

Place the cursor at the point you want to return to, then choose **Edit > Revert Conversation to Cursor**. Delta asks for confirmation before removing later messages. It restores attached Delta worktrees that still match that point; worktrees that now use a different checkout are left unchanged.

Messages that remain in the conversation stay in sent history rather than becoming drafts.

## [Mention files and folders](#mention-files-and-folders)

Type `@` in the composer and choose **Files & Directories** to mention a file or folder from an attached project. Selecting an entry inserts a link to it. A folder mention points the agent to that folder; it does not attach all its contents.

Opening a folder link reveals it in the project panel. In a narrow browser window, widen the window to open folders.

## [Open files from the transcript](#open-files-from-the-transcript)

When the agent reads, edits, or patches a file, its output links back to the file. Place the cursor on the relevant line in a Read File, Edit File, or Apply Patch block, or in a change-guide diff, and press `Option+Enter` on macOS or `Alt+Enter` on Linux and Windows to open that location in the file pane.

The same shortcut opens a link under the cursor in the conversation, whether **Send With Modifier** is on or off.

File paths in **External Changes** reports also open with a click or this shortcut. For Markdown files, use the pencil button in the file toolbar to switch between **Show Markdown Source** and **Show Markdown Preview**.

Image files open in tabs with zoom controls. In the file pane, drag over line numbers to select whole lines. With the pane focused, use `Cmd+Option+Left` and `Cmd+Option+Right` on macOS or `Ctrl+Alt+Left` and `Ctrl+Alt+Right` on Linux and Windows to switch tabs.

When the agent reads another page of a long tool output, the page’s heading identifies the source. When the source name is linked, click it to return to the original output.

## [Search and navigate](#search-and-navigate)

Press `Cmd+F` on macOS or `Ctrl+F` on Linux and Windows to search the rendered contents of a thread. Search matches visible text across Markdown formatting, so you do not need to include Markdown punctuation in the query.

To find a recent conversation in the desktop app, open the command palette with `Cmd+K` on macOS or `Ctrl+K` on Linux and Windows. **Recent Threads** prioritizes threads you prompted most recently, not those you merely opened.

With the cursor inside a code block, press `Cmd+A` on macOS or `Ctrl+A` on Linux and Windows to select its code. Press again to expand the selection to the current message, then again to select the whole thread.

In a long thread, hold `Option` on macOS or `Alt` on Linux and Windows while scrolling to move between sent user turns and review events. Moving past the latest turn jumps to the end of the thread.

Press `Cmd+[` and `Cmd+]` on macOS, or `Alt+[` and `Alt+]` on Linux and Windows, to move between navigation stops, including final responses in long agent turns.

To pin or unpin the current thread, press `Cmd+Option+P` on macOS or `Ctrl+Alt+P` on Linux and Windows.

### [Bookmark a turn](#bookmark-a-turn)

Hold `Cmd` on macOS or `Ctrl` on Linux and Windows and click a user-turn marker in the rail at the edge of the conversation to bookmark it. Repeat the gesture to remove the bookmark. Turn bookmarks persist in the thread and are visible to everyone with access, on desktop and web.

## [Thread lifecycle](#thread-lifecycle)

Threads persist. You can:

*   **Leave and return.** A thread keeps its full history and any Delta worktrees.
*   **Run several at once.** Conversations stay separate. When threads use projects, each normally gets separate managed Delta worktrees, so their project changes do not overlap. This isolates project files; it does not sandbox the agent from the rest of the selected machine. Threads that reuse an existing local checkout are an exception; see [Delta Worktrees](/docs/concepts/worktrees#using-the-local-clone-with-delta).
*   **Switch its model.** The conversation and any Delta worktrees carry on; the new model picks up where the last one left off. See [Models & Providers](/docs/agents/models-and-providers).
*   **Abandon one.** With a managed checkout, a thread’s changes, including its commits, do not touch your project folder unless you bring them in. See [Bring changes in](/docs/agents/review-and-sync#bring-changes-in).
*   **Share one.** Invite others into the conversation. See [Collaborate in a Thread](/docs/collaboration/collaborate-thread).
*   **Open one in a browser.** Browse and read synced threads without the desktop app. See [Delta on the Web](/docs/collaboration/delta-on-the-web).
*   **Review one.** Read the agent’s work and record a verdict in a separate review thread. See [Review in a review thread](/docs/agents/review-and-sync#review-in-a-review-thread).

## [Organize projects in the sidebar](#organize-projects-in-the-sidebar)

On desktop, right-click a project header and choose **Rename Project** to give it a custom local name. Before sending a thread’s first message, drag that thread onto a project header to make that project primary.

## [Archive and reopen threads](#archive-and-reopen-threads)

Archiving removes a thread from the active thread list without deleting its conversation or Delta worktrees. Choose **File > Archive Thread**, run **Archive Thread** from the command palette, or press `Cmd+W` on macOS or `Ctrl+W` on Linux and Windows. In the thread list, `Cmd+W` archives every selected thread. When the file pane is focused, the shortcut closes its active tab; when the project or Changes panel is focused, it closes that panel. Closing an untitled thread that has never been sent deletes it locally instead of archiving it; the sidebar button is labeled **Delete Empty Thread**.

To find any archived thread, search for its title in the command palette and select it; `Cmd+T` on macOS or `Ctrl+T` on Linux and Windows opens the palette with the thread filter already applied. You can also choose **File > Reopen Archived Thread**. `Cmd+Shift+T` on macOS or `Ctrl+Shift+T` on Linux and Windows reopens a recently archived thread.

You can also archive subthreads from the sidebar and restore them from thread history without archiving their parent.

Signing in on another computer does not automatically populate its desktop thread list or sync your sidebar organization. To continue a shared thread there, [open its link](/docs/collaboration/collaborate-thread#join-a-thread). You can also [browse accessible threads on the web](/docs/collaboration/delta-on-the-web#browse-threads).

Archiving organizes your thread list without deleting the server’s copy of a synced thread. To free disk space, Delta removes the local history of an archived thread three days after you archive or last open it, once the server holds a full copy. Opening the thread downloads it again, so an old archived thread needs a connection to open. Threads that were never synced stay on your machine. See [Data Storage & Deletion](/docs/privacy-and-security/data-storage#delete-individual-threads) for the separate deletion behavior and its current limitations.

## [Skills](#skills)

Threads can draw on [skills](/docs/agents/skills): reusable instructions that teach the agent project-specific or personal workflows.

## [Subagents](#subagents)

An agent can delegate parts of a task to subagents, which appear nested under the parent in the thread sidebar. See [Subagents](/docs/agents/subagents) for when to use them, how to follow their work, and how their changes reach the parent’s Delta worktrees.

## [What’s next](#whats-next)

*   [Review & Sync Changes](/docs/agents/review-and-sync) covers how the agent’s work reaches your repository.
*   [Collaborate in a Thread](/docs/collaboration/collaborate-thread) covers sharing a thread with collaborators.
*   [Terminals](/docs/agents/terminals) covers interactive and background terminals.
*   [Models & Providers](/docs/agents/models-and-providers#context-windows-and-compaction) covers context windows and compaction.
