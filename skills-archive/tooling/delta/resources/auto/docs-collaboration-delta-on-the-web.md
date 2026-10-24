---
title: "Delta on the Web"
url: "https://delta.dev/docs/collaboration/delta-on-the-web"
description: "Browse your threads and open shared threads in a browser at delta.dev/threads."
---

1.  Collaboration
2.  Delta on the Web

# Delta on the Web

Browse your threads and open shared threads in a browser at delta.dev/threads.

On this page

1.  [Browse threads](#browse-threads)
    1.  [Find a missing thread](#find-a-missing-thread)
2.  [Install Delta Web](#install-delta-web)
3.  [Inbox and notifications](#inbox-and-notifications)
4.  [Settings in the browser](#settings-in-the-browser)
5.  [Join without installing Delta](#join-without-installing-delta)
6.  [Work in a thread](#work-in-a-thread)
    1.  [Work on a computer](#work-on-a-computer)
    2.  [Work on a phone](#work-on-a-phone)
7.  [Choose where to run a request](#choose-where-to-run-a-request)
8.  [What’s next](#whats-next)

Use Delta Web to find a teammate’s thread, send a message, or review changes without installing Delta. You need a Zed account and must accept Delta’s Early Access Agreement. See [Sign In](/docs/account/authenticate) for more.

## [Browse threads](#browse-threads)

Open [delta.dev/threads](https://delta.dev/threads) and [sign in](/docs/account/authenticate) with your Zed account. Select a thread to open it in your browser.

To find your own threads or a teammate’s, open the filter menu in the header on a wider screen. On a phone, tap the floating filter button at the bottom right to open the filter sheet:

*   **Find threads you own:** under **Threads**, choose **My Threads**.
*   **Find a teammate’s thread:** choose **All Threads**. You can find threads you have joined and threads shared with everyone in the selected organization.
*   **Find a thread in another organization:** under **Organization**, choose that organization. The list starts with your default organization and shows one organization at a time.

Use **Previous** and **Next** at the bottom to browse more threads. Bookmark the page to return to the same organization, filter, and page.

### [Find a missing thread](#find-a-missing-thread)

If a thread does not appear, check the organization and choose **All Threads**. If it is still missing:

*   **Ask for the link.** A thread shared with **Anyone with the link** only appears in the list once you are a member.
*   **Ask the owner to invite you.** A private thread only appears if you have access.
*   **Open a local-only thread in the desktop app and share it.** Threads must be synced to Delta’s servers to appear on the web.

See [Collaborate in a Thread](/docs/collaboration/collaborate-thread#share-a-thread) for sharing and invitation instructions.

## [Install Delta Web](#install-delta-web)

To open Delta Web from your Home Screen, visit [delta.dev/threads](https://delta.dev/threads) and install it from your browser’s menu:

*   **Safari on iPhone:** choose **Share > Add to Home Screen**, turn on **Open as Web App**, then tap **Add**.
*   **Chrome on Android:** choose **Install and create shortcut** from the browser menu, then tap **Install**.
*   **Firefox on Android:** choose **More > Add app to Home screen** from the browser menu.

Installing it adds a Home Screen icon, not the desktop app. You still need an internet connection and a Zed account. To create top-level threads or work with local projects, [install the desktop app](/docs/installation).

## [Inbox and notifications](#inbox-and-notifications)

Open **Inbox** to see thread invitations and mentions. The Inbox and its unread indicator update automatically. On a computer, Inbox appears at the top of the left sidebar when the sidebar is open. In a thread on a phone, tap the Inbox button in the header. Opening a notification marks it read. A notification for a mention in a file or diff comment opens that comment.

For system alerts, turn on **Desktop notifications** in **Settings > General** and allow notifications when your browser asks. Alerts cover mentions and invitations while Delta Web is open. The choice applies to this site in this browser profile. If notifications are blocked, allow them in your browser’s site settings; some browsers do not support them.

## [Settings in the browser](#settings-in-the-browser)

Your account settings sync between desktop and web. Browser settings use **LLM Providers** for model connections and your Zed subscription details. Desktop-only controls, such as **App Icon**, are not available in the browser.

## [Join without installing Delta](#join-without-installing-delta)

Open the shared link and sign in with your Zed account.

*   **On a computer:** the link attempts to launch the desktop app. Choose **Open on web** to stay in your browser.
*   **On a phone or tablet:** the link opens the thread in your browser directly.

The link takes you to the sender’s position in the thread.

## [Work in a thread](#work-in-a-thread)

Send messages, [add comments](/docs/agents/comments), and [review changes](/docs/agents/review-and-sync) alongside teammates using the desktop app. You can read the thread’s files and diffs without cloning the project.

### [Work on a computer](#work-on-a-computer)

To find an action, open the command palette with `Cmd+Shift+P` on macOS or `Ctrl+Shift+P` on Linux and Windows.

To start a review, open the eye menu in the parent thread’s header and choose **Start Review**, or select an existing review. In the review, choose **Submit Review** to approve or request changes. See [review threads](/docs/agents/review-and-sync#review-in-a-review-thread) for how review edits and verdicts reach the parent.

You can also [send a message to a fork](/docs/agents/threads#send-a-message-to-a-fork) from an existing thread, including another fork. This works on desktop and web.

Run **View All Subthreads** or press `Cmd+J` on macOS or `Ctrl+J` on Linux and Windows to find the parent’s children. Filter by kind, and press `Tab` to show or hide archived children. Open a child in a tab, then use **Open as Primary** to give it the main conversation area.

Select the added/removed line counter in the bottom bar to review the thread’s changes.

To create and share a new top-level thread or manage its projects, use the desktop app, then open the thread in Delta Web.

### [Work on a phone](#work-on-a-phone)

The tab bar below the thread title keeps **Thread**, **Changes**, **Files**, and any open files or documents together. Tap **Changes** to review the diff or **Files** to browse the thread’s projects. Open a file to give it its own tab; scroll the tab bar sideways to find open tabs, and tap a tab’s close button to close it. In a subthread, the conversation tab is labeled **Subagent**, **Review**, or **Land** where applicable.

Tap the menu button at the top left of a parent thread to open the conversation’s thread list in a bottom sheet. Choose a subthread to open it, or choose **All Threads** to return to the thread browser. In a subthread, use the back arrow to return to its parent.

Tap **More** in the header for **Reviews**, **Land**, and **Search**, where available. When viewing a diff or file, this sheet also has display controls such as **Line Numbers** and **Comments** or **Comment Mode**. The controls vary with the tab you’re viewing. Use the controls below the conversation to choose a model or runner; the send button floats above them. Tap **Message Overview** to jump between turns.

To start a review, choose **Reviews > Start Review** from **More** in the parent thread. In a review, choose **Submit Review** to approve or request changes. See [review threads](/docs/agents/review-and-sync#review-in-a-review-thread) for how review edits and verdicts reach the parent.

Place the cursor on a comment or linked response to see navigation actions such as **View Reply**, **Go to Original Comment**, or **Go to Comment Target**. When the keyboard is open in the composer, use the formatting controls for **bold**, _italic_, or `inline code`. The selection toolbar lets you copy text, copy a link’s address, or copy a link to your location in the thread.

## [Choose where to run a request](#choose-where-to-run-a-request)

Choose an execution environment based on what your request needs:

*   **Work with the thread’s synced files:** send a browser turn using a Delta-hosted model. The agent can read the thread’s diffs. Browser turns do not have a disk checkout or shell, so they cannot run terminal commands.
*   **Use your computer’s tools, credentials, or running processes:** open the thread in the desktop app and run the turn on your computer. The browser cannot run commands on your device.
*   **Run commands without your computer:** if you have access to the [Cloud Runners](/docs/agents/cloud-runners) private beta, choose **Cloud** in the composer’s runner menu. The turn runs in a cloud sandbox with a shell and a checkout of the thread’s Delta worktrees.

To bring changes into your local repository, use the [desktop workflow](/docs/agents/review-and-sync#bring-changes-in).

See [Collaborate in a Thread](/docs/collaboration/collaborate-thread#where-the-agent-runs) for how execution environments differ between participants.

## [What’s next](#whats-next)

*   [Collaborate in a Thread](/docs/collaboration/collaborate-thread) covers sharing, access settings, and joining threads.
*   [Threads](/docs/agents/threads) covers steering an agent and managing threads in the desktop app.
*   [Data Storage & Deletion](/docs/privacy-and-security/data-storage) covers what Delta stores on its servers.