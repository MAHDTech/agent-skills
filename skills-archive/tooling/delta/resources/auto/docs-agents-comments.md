---
title: "Comments"
url: "https://delta.dev/docs/agents/comments"
description: "Attach comments to any part of a thread and get linked replies."
---

1.  Using Delta
2.  Comments

# Comments

Attach comments to any part of a thread and get linked replies.

On this page

1.  [Add a comment](#add-a-comment)
2.  [Comment on files and diffs](#comment-on-files-and-diffs)
3.  [When comments reach the agent](#when-comments-reach-the-agent)
    1.  [Send one comment chain to a subthread](#send-one-comment-chain-to-a-subthread)
4.  [How the agent responds](#how-the-agent-responds)
5.  [When to use comments](#when-to-use-comments)
6.  [What’s next](#whats-next)

Comment on a thread the way you’d comment on a document: select a span of text in your own message or the agent’s, and attach a note to it. The comment stays with the text it’s about. Comments also work on code, in the file tabs and diff views of the Changes tab.

## [Add a comment](#add-a-comment)

There are two ways to start a comment on the agent’s output:

1.  Click in the output or select a passage, then start typing. Delta creates a draft comment at the cursor or around the selection instead of changing the agent’s response.
2.  Select a passage, then choose **Edit > Comment on Selection**. You can also press `Cmd+Option+M` on macOS or `Ctrl+Alt+M` on Linux and Windows.

Comments support replies, so a comment can grow into a small review discussion between you and the agent or among [collaborators](/docs/collaboration/collaborate-thread) in a shared thread. Typing in a draft comment edits it. After a comment is delivered, typing inside it starts a reply, including when the delivered comment is your own.

## [Comment on files and diffs](#comment-on-files-and-diffs)

To leave feedback on the code itself rather than the conversation, comment in the **Changes** tab. In both file tabs and diff views, select code and start typing: comment mode is on by default, so typing creates a comment rather than editing the file. You can also press `Cmd+Option+M` on macOS or `Ctrl+Alt+M` on Linux and Windows, the same shortcut as in the transcript, or right-click the selection and choose **Comment**.

To edit a file directly instead, click the speech-bubble toggle in the file toolbar: filled means typing creates comments, outlined means typing edits the file. You can also press `Cmd+Option+C` on macOS or `Ctrl+Alt+C` on Linux and Windows while the file tab is focused. Your choice applies across threads and is remembered when you restart Delta.

Sending a comment follows the same keys as sending a message: `Cmd+Enter` on macOS or `Ctrl+Enter` on Linux and Windows by default, or `Enter` if you turned the [Send With Modifier setting](/docs/configuration/settings) off. Sending from a file or diff comment delivers it without returning to the composer. Once delivered, these comments collapse into indicators in the gutter; click an indicator, or press `Cmd+Shift+M` on macOS or `Ctrl+Shift+M` on Linux and Windows, to reopen the discussion.

In a file tab, right-click the comment-mode button and toggle **Show Comments**. In Changes, open **Display Options** and choose **Hide Comments** or **Show Comments**. On a phone or tablet, toggle **Comments** in **Display Options**.

You can also press `Cmd+Option+V` on macOS or `Ctrl+Alt+V` on Linux and Windows while the file or diff tab is focused. Visibility applies to that tab; it does not change your comment-mode preference.

## [When comments reach the agent](#when-comments-reach-the-agent)

Comments you write are pending until you next submit: they are delivered together with your message, and the agent addresses them in that reply. You can also submit comments on their own, without a message to go with them.

You can reply directly in a pending comment summary. Click inside a teammate’s comment and type to start a reply without returning to the original passage. Typing in your own pending comment edits that draft.

### [Send one comment chain to a subthread](#send-one-comment-chain-to-a-subthread)

With the cursor in an unsent comment, press `Cmd+Shift+Enter` on macOS or `Ctrl+Shift+Enter` on Linux and Windows to send that comment chain to a new subthread. If **Send With Modifier** is off, use `Cmd+Enter` or `Ctrl+Enter` instead. This lets you handle that feedback separately from the main conversation.

## [How the agent responds](#how-the-agent-responds)

The agent treats comments as feedback aimed at the exact passage it’s attached to: a question to answer, a problem to fix, or a review note to act on. When the agent replies, its answer is linked back to your comment, so you can jump between the annotation and the response. With the cursor in a comment’s summary, press `Option+Enter` on macOS or `Alt+Enter` on Linux and Windows to jump to the comment itself.

Click a filename in a comment summary to open the file at the comment’s location. You can still select and copy the quoted excerpt or filename without opening it.

Comment replies appear beside the agent’s response while it is still streaming.

## [When to use comments](#when-to-use-comments)

Follow-up messages work too. Use a comment when feedback is about a specific place:

*   Reviewing a plan the agent proposed, point by point.
*   Flagging one line in a code block without quoting it.
*   Leaving several independent notes on one long response, each of which gets its own linked answer.

## [What’s next](#whats-next)

*   [Review & Sync Changes](/docs/agents/review-and-sync) covers reviewing the agent’s edits.
*   [Collaborate in a Thread](/docs/collaboration/collaborate-thread) covers comment discussions with collaborators in shared threads.