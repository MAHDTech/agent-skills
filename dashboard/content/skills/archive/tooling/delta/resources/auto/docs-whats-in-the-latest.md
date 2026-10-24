+++
title = "docs-whats-in-the-latest"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Release notes"
url: "https://delta.dev/docs/whats-in-the-latest"
description: "Notable changes in recent Delta releases, newest first."
---

1.  Getting Started
2.  Release notes

# Release notes

Notable changes in recent Delta releases, newest first.

On this page

1.  [0.21.0](#0.21.0)
2.  [0.20.1](#0.20.1)
3.  [0.20.0](#0.20.0)
4.  [0.19.2](#0.19.2)
5.  [0.19.1](#0.19.1)
6.  [0.19.0](#0.19.0)
7.  [0.18.2](#0.18.2)
8.  [0.18.1](#0.18.1)
9.  [0.18.0](#0.18.0)
10.  [0.17.1](#0.17.1)
11.  [0.17.0](#0.17.0)
12.  [0.16.1](#0.16.1)
13.  [0.16.0](#0.16.0)
14.  [0.15.0](#0.15.0)
15.  [0.14.0](#0.14.0)
16.  [0.13.0](#0.13.0)
17.  [0.12.0](#0.12.0)
18.  [0.11.0](#0.11.0)
19.  [0.10.0](#0.10.0)
20.  [0.9.1](#0.9.1)
21.  [0.9.0](#0.9.0)
22.  [0.8.0](#0.8.0)
23.  [0.7.0](#0.7.0)
24.  [0.6.2](#0.6.2)
25.  [0.6.1](#0.6.1)
26.  [0.6.0](#0.6.0)
27.  [0.4.0](#0.4.0)
28.  [0.3.1](#0.3.1)
29.  [0.3.0](#0.3.0)
30.  [0.2.0](#0.2.0)
31.  [0.1.1](#0.1.1)

## [0.21.0 Oct 21, 2026](#0.21.0)

## [✨ New this release](#0.21.0--new-this-release)

*   Shared threads can now be opened in Claude Code with the `Open in Claude Code` command, on desktop and web, when the Delta plugin for Claude Code is installed.
*   Collaborators connected through supported external tools, including Claude Code, now appear in the thread’s participant bar with an indication of the tool they’re using.
*   You can now check for Delta updates with `delta update --check` and install them on macOS and Linux with `delta update`, which shows progress while downloading and installing an update.
*   Thread images can now be opened in zoomable pane tabs and copied on macOS, Windows, and the web.
*   You can now use `Compact Now` while a local agent is running.
*   Settings links in threads open a page or highlight a control, and Settings now offers copy-link buttons and sidebar links to documentation and desktop feedback.
*   Delta now supports connecting to Figma’s MCP server.
*   `Apply Tag to Subthreads` in a thread’s right-click menu now copies its tags to existing subthreads.
*   Copy thread titles directly with `Copy Title` in the sidebar context menu or `Copy Thread Title` in the command palette.
*   Use `Collapse Child Threads` from the command palette to hide the current thread’s child rows in the sidebar.
*   On macOS, the `System Notifications` setting now shows whether Delta is allowed to show notifications and includes a `Send Test Notification` button.
*   Use `tab` and `shift-tab` to move between cells in read-only tables.

## [⚡ Better than before](#0.21.0--better-than-before)

*   Colored tags now control thread colors, and subthreads can have colors independent of their parents.
*   Editing an earlier message now warns that sending it will replace later conversation, while an in-progress response keeps running until you send.
*   On phones, Delta Web now puts conversations, changes, files, and open documents in one tab bar, with thread controls in touch-friendly bottom sheets.
*   Preparing a new thread’s worktree is now faster, especially in repositories with many files.
*   Slash commands now use action-oriented names: `/send-feedback`, `/toggle-fast`, `/change-model`, `/fork` (still searchable as `subthread`), and `/set-thinking-level`.
*   Choose the starting branch for each project’s new Delta worktrees in `Delta Worktrees` settings.
*   The `Keymap` now shows shortcuts from across Delta and lets you search by action or key combination.
*   On desktop, GitHub file links pinned to a commit can now open matching files and unchanged lines in your thread’s local projects instead of the browser.
*   Compaction errors now appear directly in the thread instead of in pop-up notifications.
*   Worktree loading is more reliable when joining shared subthreads.
*   The `ctrl-tab` thread switcher now shows running and unread indicators alongside thread colors.
*   Projects with matching default names in the sidebar now show muted owner or path prefixes to help you tell them apart, while custom names stay unchanged.
*   Closing large diffs in the desktop app is less likely to cause a noticeable pause.
*   Hovering over someone mentioned by an agent now explains that they were not notified.

## [🔧 Fixes and reliability](#0.21.0--fixes-and-reliability)

*   Threads can now recover after a connection loss interrupts recovery, without getting stuck when you discard local changes.
*   Agents no longer silently stop when they try to call an unavailable tool.
*   Long OpenAI sessions no longer stop with a “websocket connection limit reached” error.
*   Gemini models through OpenRouter, including Gemini 4 Argon, no longer fail with “Provider returned error” in threads that can edit files.
*   Claude models through OpenCode no longer fail with a `thinking.adaptive.block_binding` error when thinking is enabled.
*   Mistral Large 4 responses no longer fail when using an OpenAI-compatible provider.
*   ChatGPT subscription models no longer receive duplicate copies of their previous replies and tool calls.
*   Long-running agent turns now compact earlier work automatically when using models without built-in compaction.
*   Claude Haiku 5.5 no longer rejects conversation compaction with an invalid token threshold.
*   Signing in again to the same account no longer reloads your workspace or interrupts ongoing work.
*   Threads are now shared in the organization selected when they were created, even if you switch organizations before sharing.
*   Rejoining a thread no longer fails to load its worktree in some cases after merging changes from a subthread.
*   Switching a thread’s worktree from Isolated to Existing no longer sits on “Preparing” for minutes in repositories with many branches or tags.
*   Reviews started from a subthread can now be submitted and have their edits pulled into the original thread.
*   Agents now receive the affected file paths when merging changes from another thread introduces conflicts, instead of an unqualified success message.
*   Agents no longer report a change as landed when only its preparation is complete.
*   `Land Changes` now skips invalid landing skills when a working one is available.
*   File opens now report storage errors instead of getting stuck.
*   Reduced delays when opening threads during background cleanup.
*   You now also receive @mention notifications when the thread is open but not focused.
*   Grok 4.7 now uses its full 500k-token context window when signed in with a Grok subscription.
*   Provider setup now explains rejected API keys and lets you enter a replacement without leaving onboarding.
*   ChatGPT subscription connection errors now explain when OpenAI rejects an unsupported region instead of suggesting another sign-in attempt.
*   Thinking can now be turned off for models that allow it only at certain effort levels, with an explanation when the selected effort requires thinking.
*   Thinking output no longer repeats words or characters when using OpenAI-compatible providers such as Bifrost.
*   Summaries for custom-provider threads now use the thread’s selected model instead of the first configured model.
*   OpenCode Zen and Go now show current models, including Muse Spark Contributor models on Go.
*   OpenCode’s model picker no longer offers deprecated models or free Zen models that OpenCode blocks in Delta.
*   Context usage now updates correctly when using OpenRouter.
*   Settings no longer closes unexpectedly when startup sign-in finishes.
*   Switching recent threads with `ctrl-tab` now returns to visible subthreads instead of skipping to their parent threads.
*   On Linux and Windows, `ctrl-p` now opens `Go to File` while a thread or editor has focus.
*   `Open in Zed` now opens the file at your cursor position in Changes.
*   Comments routed into subthreads now show and link back to their original text.
*   Clicking a thread link inside Delta now jumps to its linked location even when the thread is already open.
*   Thread titles now refresh when you edit and resubmit the initial prompt.
*   Images read by the agent now consistently show an expandable preview in the thread.
*   Images in agent responses now display when referenced by a path relative to the project root.
*   Comments on new tool diffs now show the file, line number, and diff highlighting in their summaries.
*   Clicking an unloaded subthread in the sidebar now shows its loading state immediately instead of leaving the previous thread open.
*   `Remove from Sidebar` now switches away from a thread in the removed project, opening a new thread under Threads if no other thread is visible.
*   On phones, `Cancel Edit` now lets you leave an earlier-message edit after hiding the keyboard without resending the message.
*   The project tooltip now updates while you hover to show the current worktree preparation status.
*   Your comments now use the same color in thread, file, and diff views.
*   Consecutive assistant messages now appear as separate paragraphs instead of running together.
*   The composer no longer briefly jumps when sending a message containing only comments.
*   Tool calls with no output to display no longer show an expand arrow.
*   The first message in a forked thread no longer crowds against the “Forked from” row after you add or remove worktrees.

## [🗑️ Removed](#0.21.0-️-removed)

*   The `delta daemon start`, `status`, and `stop` commands are gone; Delta now starts for `delta` commands when needed and exits once they finish.
*   Opening paths or URLs with `delta \<target\>` is no longer supported; use `delta open \<target\>` instead.

## [0.20.1 Oct 14, 2026](#0.20.1)

## [🔧 Fixes and reliability](#0.20.1--fixes-and-reliability)

*   Adding a project without a Git remote now explains that Delta needs one, instead of letting the thread break after a restart.

## [0.20.0 Oct 14, 2026](#0.20.0)

## [✨ New this release](#0.20.0--new-this-release)

*   Subagents now run as regular subthreads that you can interact with, steer, and continue yourself.
*   You and your agents can now create subthreads inside other subthreads.
*   Connect MCP servers in `Settings → MCP` to let agents use tools from your connected services.
*   You can now start a review or Land changes from inside a subthread, and the review or Land covers everything in that subthread, including changes it inherited from its parent threads.
*   Delta Web now has an Inbox for thread invitations and @-mentions. The Inbox and unread indicator update automatically, and the Inbox appears in the left sidebar on wider screens, matching the native app.
*   You can now tag threads from their right-click menu or the `Edit Thread Tags` command, and filter the sidebar by tag from its filter menu.
*   Agents running in your browser can now inspect code diffs when reviewing your changes.
*   You can now send a single comment chain to a subthread by pressing the subthread shortcut (`cmd-enter`, or `cmd-shift-enter` when sending with a modifier) with the cursor in the comment.
*   Enable `Desktop notifications` in Delta Web’s General settings to receive desktop alerts for mentions and invitations while Delta Web is open.
*   You can now select Gemini 4 Argon using your Google API key, with low, medium, and high thinking levels.
*   Use `/create-skill` to get help creating reusable agent skills in Delta.
*   Choose a branch in the project menu to use as the starting point for new Delta worktrees.
*   Right-click the thread’s diff stat to filter its change counts and review by project.
*   The Bookmarks view now has filters for tags, ownership, and which subthreads to show.
*   You can now invite people to your organization from the command line with `delta org invite`, or invite teammates to your selected team organization from `Invite Members…` in the user menu, even after onboarding.
*   You can now create a team organization from the command line with `delta org create "My organization"`, or open the organization creation page with `Create Organization` in the command palette.
*   You can list your organizations, check the current selection, and switch organizations from the command line with `delta org`.
*   You can now type values directly into number fields in Settings.

## [⚡ Better than before](#0.20.0--better-than-before)

*   The `Enable Sub-agents` setting is now called `Delegation`, offers Off, Only When Asked, and Max, and is set to Only When Asked unless you had turned subagents off.
*   On desktop, API keys for model providers now sync through your Delta account.
*   New worktrees now preserve your repository’s additional Git remotes, so agents can use both your fork and upstream without extra setup.
*   Archiving and renaming a subthread now take effect immediately, even while offline.
*   Mentions of you in a thread are now marked read in your inbox when you open that thread, however you open it.
*   Clicking a notification for a mention in a file or diff comment now opens that comment.
*   The Changes panel now shows whether each file was added, modified, or deleted in the selected comparison.
*   The thread footer now shows the change count for the selected comparison, including zero changes since Last Commit or Last Turn.
*   The Changes panel now has a draggable scrollbar for long lists of changed files.
*   Expanded external-change notifications now show branch and commit transitions, with file-change counts on a separate line.
*   After the first successful use of a newly created project Land skill, Delta shows how to use it in the parent thread.
*   Threads without projects now show a labeled `Add Project` button instead of just an icon.
*   Your first thread after the welcome guide now shows the `@`, `/`, and `!` tips in its empty composer.
*   The send button tooltip now shows the `Stop Generation` shortcut while you draft during an agent response.

## [🔧 Fixes and reliability](#0.20.0--fixes-and-reliability)

*   In a shared thread, sending a message includes every comment again, so comments a collaborator left their cursor in no longer stay unsent.
*   Joining a thread whose history can’t be downloaded now shows an error with Retry instead of staying on “Joining Thread…” forever.
*   Joining a shared thread no longer asks for a GitHub username and password when your local checkout of the repository uses SSH.
*   Delta now falls back to cloud-synced settings when the settings key is missing from your system’s credential store.
*   Restarting Delta no longer reopens an archived thread after all threads have been archived.
*   Typing into an agent response after a collaborator resends an edited message now creates a comment instead of changing the response.
*   Thread sharing visibility no longer reverts to an earlier choice while an agent turn starts.
*   The main thread’s agent now knows what comments sent to a subthread refer to, including when you reply to one later.
*   In shared threads, only the person who sent the prompt is notified when the agent finishes, not everyone who has the thread open.
*   Joining a shared thread now prefers your existing local repository as the source for a new isolated checkout.
*   Running `delta` commands or another Delta build from Delta’s built-in terminal now uses that build’s own data instead of the running app’s.
*   Agents ask for confirmation before landing changes when a planning discussion leaves permission to merge unclear.
*   When the connection to the model drops, Delta no longer leaves an empty tool call in the thread.
*   Custom providers using the Anthropic API now support prompt caching, with a per-model `Prompt caching` setting.
*   OpenRouter compaction no longer fails when earlier reasoning data comes from a different backend.
*   Zed Pro trial setup now shows clearer errors and lets you try again when signup fails.
*   Subthreads now appear beneath their main thread in the web sidebar.
*   Agent links to subthreads in the current conversation now open the parent thread with the linked subthread in a tab on desktop and desktop web.
*   Returning to a thread that has a subthread tab open now puts focus back on the parent or subthread you were last using.
*   Nested threads you collapse in the sidebar now stay collapsed when you switch threads with `cmd-{` and `cmd-}`, reopen Delta, or create a new subthread under them.
*   Subthreads no longer get stuck with an outdated sidebar title after reopening.
*   Collapsed tool groups now show an activity indicator while a tool inside them is still running, even after the agent continues with other work.
*   The cursor no longer disappears when pressing `up` at the beginning of a subthread whose parent contains comments.
*   Clicking a wrapped link in an inline terminal now opens the complete URL.
*   Land results no longer show raw formatting tags when they follow the agent’s explanation.
*   Comment replies in change guides now display without internal formatting tags.
*   The `Pull Changes` button and review avatar badges now stay proportional to thread text when UI and prose font sizes differ.
*   Zoomed file panes on the web no longer show the divider from the underlying thread.
*   The first character stays visible when you replace a long thread title with shorter text.

## [🗑️ Removed](#0.20.0-️-removed)

*   Delta no longer loads `.env` files or implicitly reads model provider keys from the environment; they must be configured in settings.

## [0.19.2 Oct 9, 2026](#0.19.2)

## [🔧 Fixes and reliability](#0.19.2--fixes-and-reliability)

*   The `Send With Modifier` setting is no longer stuck on.
*   On Linux and Windows, Delta now quits when you close its last window after a normal desktop launch, so relaunching it after an update starts the new version. Windowless daemons continue running after their windows close.
*   The `Reset Prose Font` button now restores the default font even when `UI Font Family` is customized.

## [🗑️ Removed](#0.19.2-️-removed)

*   The `Stay Open When Window Closed` setting is gone; use `delta daemon start` to keep Delta running without a window.

## [0.19.1 Oct 8, 2026](#0.19.1)

## [🔧 Fixes and reliability](#0.19.1--fixes-and-reliability)

*   Sending feedback no longer incorrectly reports “Feedback submission is temporarily unavailable” after your feedback has been submitted.

## [0.19.0 Oct 7, 2026](#0.19.0)

## [✨ New this release](#0.19.0--new-this-release)

*   Mention teammates in messages and comments by typing `@` followed by their username, and they’ll be notified in their Inbox (`cmd-shift-j`).
*   @delta can now read and change many settings from within the conversation.
*   Use `Copy Link to Selection` to share selected text and comments in threads, files, and diffs; in files and diffs, `cmd-opt-l` (`ctrl-alt-l` on Windows and Linux) copies the link.
*   CLI commands can now run without opening a window, and `delta open` opens a window in the same running app.
*   The new `Stay Open When Window Closed` setting keeps Delta running after its last window closes and is enabled by default.
*   You can give projects a custom local name by right-clicking their sidebar header and choosing `Rename Project`.
*   You can now view source on Markdown files using the pencil in the toolbar.
*   Before sending the first message, you can drag a thread onto a project header in the sidebar to make that project primary.
*   Turn off `Agent Thread Creation` to prevent agents from creating top-level conversations without disabling subagents.
*   Custom provider headers now support `{thread_id}`, and session-grouping or cache-routing headers for LiteLLM, xAI, and Baseten are automatically populated by default.
*   Subthreads in the sidebar now have an Open in Tab button.

## [⚡ Better than before](#0.19.0--better-than-before)

*   Your settings now follow your account across desktop and web.
*   New threads use updated default models when available, and the model picker shows a more focused set of recommendations.
*   You can now ask the agent to merge selected worktrees from multiple threads.
*   Subagent profile configuration is now stored in settings.
*   Trial-eligible users now see the 14-day Zed Pro trial first during LLM setup and continue onboarding automatically after signup.
*   Following links to specific lines in files, attachments, or positions in a thread now briefly highlights the destination.
*   After sending a message, Delta keeps the next message box in view as the agent replies without unnecessarily scrolling your first message away.
*   In a shared thread, a comment someone is still writing waits for a later message instead of being sent half-finished.
*   Available Google AI Studio models now update without requiring a Delta release.
*   Custom OpenAI-compatible providers now accept arbitrary reasoning levels.
*   `Keep Computer Awake` now allows your computer to sleep when no Delta windows are open, even if background work is still running.
*   File paths in External Changes reports now open with a click or `alt-enter`.
*   Thread suggestions in the desktop `@` menu now show parent–child relationships, making related threads easier to find.
*   Archived threads no longer appear in the desktop `@` mention picker.
*   Recent mentions now appear closest to the cursor when the `@` picker opens above or below it.
*   The `View All Subthreads…` picker now remembers its selected filter when reopened.
*   Web model settings now appear under `LLM Providers` between `Appearance` and `Thread`, matching the desktop app.
*   Web and mobile badges now appear only when someone has unsent drafts open on more than one device, instead of on every participant and message avatar.
*   The diff file panel now shows seen files with grey (instead of red/green) diffstat.
*   The welcome thread now shows a hint for mentions, commands, and inline terminals whenever its composer is empty.
*   Message-rail previews now show thread summaries in muted text without a title, keeping human prompts more prominent.
*   The status-bar context meter no longer uses the warning color when `Automatic Compaction` is enabled.

## [🔧 Fixes and reliability](#0.19.0--fixes-and-reliability)

*   Merging the same worktree changes on two machines at the same time no longer loses, duplicates, or misplaces text when the merge has no conflicts.
*   Merging changes on multiple machines no longer leaves project files out of sync.
*   In shared threads, messages and comments sent after compacting the conversation no longer go missing from what the agent sees while other people have unsent drafts.
*   Shared threads no longer get stuck syncing after reconnecting with offline changes.
*   Opening a shared thread you already have no longer leaves it stuck loading.
*   Delta now shows an error instead of crashing when a merge prevents a thread from loading, so you can keep working in other threads.
*   Delta no longer crashes when certain tool calls change how they are displayed in a thread.
*   Some projects that failed to load after merging file changes now open correctly.
*   Files no longer get stuck opening when changes from another worktree are merged.
*   Closing a subthread’s tab no longer stops it while it is still working.
*   Agents using your own model provider now recover from temporary connection failures while Delta reconnects.
*   On Windows, saving files no longer fails with “Access is denied” when an antivirus scanner or search indexer is briefly holding the file.
*   Opening projects on Linux no longer fails with an “OS file watch limit reached” error when the personal skills directory is missing.
*   On macOS, renaming the Delta app no longer prevents updates.
*   Failed update restarts now show an error with recovery options instead of reporting `Up to Date`.
*   Archiving a parent thread now cleans up its subthreads’ worktrees, including those left behind by previously archived threads.
*   Thread searches and reads through the CLI and agent tools no longer fail with false-positive history-reset errors.
*   Subagents that have finished their work no longer appear to keep running after reopening a shared thread.
*   Quitting and reopening Delta no longer marks a thread running on a collaborator’s machine as interrupted.
*   Reasoning is now handled correctly when models using an OpenAI-compatible Responses API make parallel tool calls.
*   The web app’s `Models` settings now show your Zed account and subscription status, with the appropriate subscription-management action.
*   Messages sent to a subthread are now read-only in the parent thread, so their `Route to subthread` row no longer becomes editable text.
*   Find now uses the word at the cursor when you press `cmd-f` or `ctrl-f` without selecting text.
*   Thread content no longer jumps while you type after adding a repository.
*   `ctrl-l` now keeps the cursor at the chosen viewport position while the agent is streaming, instead of snapping back to the bottom.
*   Keyboard focus now returns to the zoomed file or diff pane when you reopen the right sidebar with `cmd-r` on macOS or `ctrl-r` on Windows and Linux.
*   You can now start editing a submitted comments-only message by typing on its `N comments` heading.
*   Sending a message to a new subthread now places the cursor according to `Cursor After Send`.
*   Adding another project to a thread now immediately shows its name and a loading indicator below the composer.
*   The agent activity cursor now remains visible after external changes and other context updates while the agent is still running.
*   Collapsed tool activity now shows a blinking cursor while the agent is streaming.
*   Markdown formatting no longer extends quotes, lists, or tables into neighboring text in certain content-update sequences.
*   Bold and italic text now renders correctly next to em dashes and other Unicode punctuation.
*   Multiline equations inside blockquotes now render without quotation markers.
*   With `Diff Markers` enabled, edit blocks now keep the markers separate from code and align the code across changed and unchanged lines.
*   Conflicting file names remain searchable in the thread after pulling review changes.
*   Change guides now avoid an error that could leave code excerpts without review formatting.
*   Agent replies to comments no longer briefly show partial closing tags while streaming.
*   Switching a Mermaid diagram from `Source` to `Preview` no longer scrolls the thread unexpectedly.
*   Following a subthread’s Forked from link now highlights the corresponding event in the parent thread.
*   Empty comment drafts in the file pane now disappear when you move the cursor out of them.
*   Diff text no longer overlaps the `+` and `-` signs when scrolling horizontally.
*   The search button no longer overlaps diff line counts in a narrow Review Changes pane.
*   Opening a subagent with the sidebar visible no longer shows a duplicate application menu button on Windows and Linux.
*   The desktop-only `App Icon` section no longer appears in web settings.

## [0.18.2 Oct 2, 2026](#0.18.2)

*   Improved Windows crash diagnostics so we can investigate reported crashes more reliably.

## [0.18.1 Oct 2, 2026](#0.18.1)

## [🔧 Fixes and reliability](#0.18.1--fixes-and-reliability)

*   Delta no longer uses excessive memory when a subagent’s changes are merged into a large project.
*   Grok subscription models no longer fail with an outdated client error.

## [0.18.0 Sep 30, 2026](#0.18.0)

## [✨ New this release](#0.18.0--new-this-release)

*   Use `View All Subthreads` (`cmd-j` on macOS or `ctrl-j` elsewhere), available on desktop and web, to search and open subthreads or attached agents from the current parent thread, filter by kind, or press `tab` to include archived subthreads.
*   Enable `Collapse All Tools` in Settings > Thread to also fold file edits and single actions into expandable summaries while keeping conversation text visible.
*   On mobile, the bottom bar now offers buttons to jump from a selected comment to its reply or original context.
*   Bookmark threads with `cmd-d` to find them again later, even after archiving, from the bookmarks button in the sidebar or `cmd-opt-b`.
*   Double-clicking a thread in the sidebar now opens its rename input.
*   Subthreads now show a one-time notice explaining that they start from an isolated copy of the parent thread and that you can ask the agent to merge their changes back.
*   Use `cmd-opt-u` on macOS or `ctrl-alt-u` on Linux and Windows to return from a primary subthread to its parent.
*   Code blocks in threads now have a Copy button when you hover over them or place the cursor inside.
*   Following a collaborator now opens the images they view and mirrors their zoom and pan.
*   Participant and message avatars now show when an author is connected from a web or mobile browser.
*   You can now copy links to selected thread text, opening collaborators directly into a focused view of the shared passage.
*   The Review header now shows how many files you have marked as seen, such as `3/17`.
*   The review’s options menu can now collapse or expand all files and mark all files as seen or unseen, also available with `shift-tab` and `cmd-shift-y` (`ctrl-shift-y` on Windows and Linux).
*   You can now use Google AI Studio Gemini models.
*   GPT-6 Sol, GPT-6 Luna, and Claude Opus 5.5 are now available with API keys.
*   Show or hide all of a provider’s current models in the picker with one click.
*   `Settings → LLM Providers → OpenRouter` now lets you limit serving providers or prioritize the highest-throughput provider.
*   Added support for OpenAI- or Anthropic-compatible model providers in the desktop app. Use by choosing `Add provider` in `Settings` → `LLM Providers`.
*   LLM Provider settings now show your ChatGPT rate limits and reset times under `ChatGPT Subscription`.
*   Threads can now run in any existing linked Git worktree and carry that checkout into new threads created from the current thread.
*   Choose `Create Organization` in the user menu to create an organization on the Zed dashboard.
*   The `delta auth` commands report and change the account the running Delta app is signed in with, including browser sign-in, credential handoff from another app, and sign-out.
*   The `delta` CLI is now available from your terminal: use `Install CLI…` from the Delta application menu on macOS, the Windows installer, or `install.sh` on Linux.

## [⚡ Better than before](#0.18.0--better-than-before)

*   The context meter now shows its color breakdown only while the context breakdown is expanded.
*   Background terminals now provide explicit actions to scroll to or stop each process.
*   Subagents in the web app’s sidebar now appear indented under their parent thread.
*   The initial request of a subthread or review thread now makes better use of the parent thread’s prompt cache.
*   Subthread rows, including `Route to subthread`, now show the subthread’s name, which opens the subthread in a tab when clicked.
*   Subthread event rows now have color-coded badges, so reviews, landings, and other subthreads are easier to tell apart.
*   Subthread tabs now open immediately and show a loading indicator until the subthread is ready.
*   Links that jump to an event in another thread, such as a subthread’s Forked from link, now scroll it to the same place as opening a subthread from the sidebar.
*   Collapsing a sidebar section or parent thread now hides every thread inside it, including the one you have open.
*   Collapsed sidebar sections now show a running indicator when a thread inside them is working.
*   Subthreads and subagents now appear together under their parent in the sidebar, newest first by when they were created, with six shown before `Show N more`.
*   The sidebar filter menu’s `Subthreads` section now shows or hides plain subthreads, reviews, land subthreads, and subagents, remembers your choice, and replaces `In-Progress Only`.
*   Message and comment timestamps now show times with AM and PM, and no longer repeat “Yesterday”.
*   Bookmarks on messages in the thread navigation rail now persist after closing the thread and are visible to everyone in it.
*   Pressing `!` in a local thread now brings back the same terminal, with whatever it was running, instead of starting a new shell.
*   Pinned threads in the sidebar now show which project they belong to.
*   Selected color-coded threads now show a thicker color bar in the sidebar, making the active thread easier to spot.
*   Older threads are now grouped by month in the archive and Time-grouped sidebar.
*   Tool call and event titles in threads now consistently use sentence case, such as “Received agent message” and “Context compacted”.
*   Clicking the sender of a received agent message now opens their thread.
*   Clicking a subthread or subagent in the sidebar now opens it as the primary thread; `cmd-click` (`ctrl-click` on Windows and Linux) opens it in a tab.
*   File changes made through terminal scripts now appear as diffs.
*   Following a collaborator now scrolls smoothly through conversations and diffs.
*   Web review tabs no longer show a separate `Review Changes` toolbar button; use the added/removed-lines counter in the bottom bar to open the diff.
*   On mobile, Review Changes now keeps its comment display controls in the main menu instead of a second overflow menu.
*   When the agent writes a change guide Delta can’t display, the agent now rewrites it automatically instead of leaving raw markup in the thread.
*   The Review and Land menus now describe what each action does, and their buttons’ tooltips show its keyboard shortcut.
*   Selecting a collapsed file in Changes now expands it in the diff view.
*   Slash command suggestions now prioritize the closest matching command or skill.
*   Sending a message that uses a skill that couldn’t be loaded now stops and explains what’s wrong with the skill file, instead of sending the message without the skill.
*   Finished subagents no longer slow Delta down or cause stutters while other agents are working.
*   Anthropic and OpenAI API billing errors now offer direct links to usage or billing pages.
*   OpenAI API-key users now get new OpenAI models, including GPT-6.1 Sol, as soon as OpenAI releases them, while older models such as GPT-5.4 and o3 are no longer listed.
*   The project button in the status bar now shows “Preparing…” until the project’s files are ready to `@` mention.
*   `Land Changes` results now appear as a card showing whether the change landed, with its target branch, commit, and CI status, in both the Land subthread and its parent thread.
*   The settings window in the web app now closes when you click outside it.
*   The account menu now uses icons and organization initials to make account actions and organization switching easier to scan.
*   Menus and pickers, including the command palette, branch picker, and model picker, now support `ctrl-j` and `ctrl-k` to select the next and previous item.
*   Markdown tables now have lighter borders, alternating row shading, and less heavy header text.
*   Settings pages now share consistent rows, dropdowns, text inputs, and spacing.
*   Checkboxes are easier to see in settings, menus, task lists, and review diffs.

## [🔧 Fixes and reliability](#0.18.0--fixes-and-reliability)

*   Typing at the beginning or end of a table cell now creates a comment on the whole cell.
*   Terminal headers no longer overlap commands when the focus control displays `shift-esc`.
*   Copying between threads in the desktop app now preserves images and pasted-text attachments.
*   Archiving a subthread now closes its tab even when that tab is not the one you are viewing.
*   The inline terminal cursor no longer briefly appears below the terminal while running commands.
*   Typing a comment on an agent message or tool call with Japanese input no longer duplicates the first character.
*   Pressing `cmd-alt-m` inside a comment now starts a reply to it.
*   Subthreads forked after a commented message no longer show extra space above their first row.
*   Delta no longer repeatedly crashes on startup after certain merge conflicts.
*   Selecting the first line of folded terminal output now keeps the cursor and highlight on the text instead of the “Show more” control.
*   Agent responses retain Markdown formatting when they resume after a prompt is sent to a subthread.
*   Agent answers stay visible instead of being hidden inside collapsed tool groups when subagent messages arrive.
*   The cursor no longer disappears when moving up from the beginning of a subthread.
*   Opening a thread in the browser no longer sometimes crashes while loading its repositories.
*   In narrow windows, opening the right sidebar, a panel, or a file now makes room for it instead of doing nothing.
*   Fixed a crash when displaying Markdown with a code block immediately following an unfinished code block inside a quote.
*   On Linux, thread links now open in Delta when the app is already running.
*   Collaborators’ names on their cursors are no longer oversized when the cursor is in a heading.
*   After you stop following someone, Delta no longer unexpectedly switches you to their commit or subthread.
*   The Share dialog now correctly shows a shared thread’s existing organization instead of the organization currently selected in Delta.
*   Following a collaborator now continues when they open a subthread, whether as a tab or as the primary thread, and back again.
*   Collaborators’ cursors no longer disappear from subthread tabs.
*   A shared thread whose offline changes were moved into a local-changes thread no longer stays stuck unsynced after the connection drops and comes back.
*   GitHub projects can now be shared when their remote address omits the `git@` prefix.
*   On Windows, joining a shared thread no longer reports unchanged files as edited or changes their line endings for everyone.
*   Review verdict badges in the thread toolbar are no longer clipped when multiple reviewers are present.
*   Typing with a phone keyboard after selecting code in Review Changes now starts a comment.
*   On the web, returning to a parent thread now restores the review conversation in its tab instead of showing changes.
*   A change guide at the start of the agent’s response no longer shows a “Could not attach change guide metadata” error.
*   URLs in diff views now open with `cmd-click` on macOS or `ctrl-click` on Windows and Linux.
*   Diff shortcuts now open the focused subthread’s changes instead of the parent thread’s changes.
*   Edits to an outdated open file no longer affect a replacement file at the same path.
*   Local subagents now work with repositories that have no Git remote, including in shared threads.
*   Agents more accurately target highlighted text when responding to file and diff comments.
*   Grok subscription accounts with Grok 4.7 can now sign in and use it in Delta.
*   OpenRouter latest-model aliases now show the model vendor’s logo instead of the OpenRouter logo when the vendor is recognized.
*   Switching models in a long conversation no longer uses the previous model for automatic compaction.
*   `Land Changes` no longer fails because an unrelated skill has invalid frontmatter.
*   Opening a project stored in WSL from the Windows app now shows a clear unsupported-location error instead of proceeding and failing later.
*   `.agents/prepare` scripts now run on Windows when Delta creates a worktree.
*   Branch details now show the commit checked out in the isolated workspace.
*   Repositories whose only Git remote is named something other than `origin` can now be shared, and their branch list refreshes against that remote.
*   New repository checkouts now use the SSH or HTTPS connection configured in a local checkout already opened in Delta.
*   Desktop Settings now warns when changes couldn’t be saved and apply only to the current session.
*   Closing a focused project or changes panel now returns focus to the file or conversation you were in before, including a zoomed file.
*   Code after a redacted secret in a file edit or change guide diff now keeps its syntax highlighting.
*   Fixed a crash that could occur when composing text with an input method.
*   Arrow keys now move the selection in the web command palette and menus.
*   New users no longer skip onboarding after signing in.
*   Copying text containing null characters on Windows no longer truncates the clipboard contents.
*   Single-line text fields now support selecting text by dragging with the mouse.

## [🗑️ Removed](#0.18.0-️-removed)

*   Subthreads no longer post progress, success, and failure updates in their parent thread; `Land Changes` results still appear there.

## [0.17.1 Sep 28, 2026](#0.17.1)

## [🔧 Fixes and reliability](#0.17.1--fixes-and-reliability)

*   Onboarding now continues after you accept Delta’s terms in your browser, without restarting the app.

## [0.17.0 Sep 23, 2026](#0.17.0)

## [✨ New this release](#0.17.0--new-this-release)

*   Type `:` followed by a name in a message or comment to find and insert emoji and mathematical symbols.
*   Use `Open as Primary` in the command palette to open the focused subthread or subagent tab as the main conversation.
*   Comments in Files and Changes can now be shown or hidden from the comment menu or with `cmd-opt-v` (`ctrl-alt-v` on Linux and Windows).
*   Threads can now be color-coded from their sidebar context menu.
*   Pasting a large block of text into a conversation now attaches it as a file the agent can read and you can open in a tab.
*   The context window popover can now show how much of the prompt is used by instructions, messages, thinking, and tools.
*   Set or clear colors for multiple selected threads from their right-click menu.
*   You can now ask the desktop agent to start separate threads with isolated workspaces and work on several tasks in parallel.
*   Repositories can now define custom thread emoji in `.delta/emoji`.
*   Turn previews now show the number of comments included with each message.
*   Thread owners now appear in the thread list, which can also group threads by owner.
*   Use `/approve` and `/request-changes` to submit a verdict in any thread.
*   Review Changes now shows syntax-highlighted symbol breadcrumbs beside the file name as you move through code.
*   You can now ask an agent to create an isolated copy of an existing thread from its thread ID or shared link.
*   Agents can now exchange messages with agents in any thread.
*   Project Settings can now check whether a thread’s branch is ahead of, behind, or diverged from its remote branch.
*   Feedback reports can now attach up to 100 KiB of the current thread’s most recent conversation context and tool output with `Include conversation context and tool output`; reasoning and images are left out.
*   Navigate settings categories and controls with the keyboard, using `enter` or `space` to move from a category into its settings.
*   Drag over line numbers in the file pane to select whole lines.
*   Image files can now be opened in zoomable workspace tabs.

## [⚡ Better than before](#0.17.0--better-than-before)

*   The context meter now shows progress toward automatic compaction and reports the percentage in its tooltip.
*   New threads now use an initial Markdown heading as their title.
*   Saved tabs and splits now appear immediately when reopening a thread, while their contents load independently.
*   Archived subthread events now use a dedicated archive badge.
*   `Routed to Subthread` rows now support cursor placement and text selection.
*   Thread context menus now show clearer bulk actions and more compact, theme-aware color tags.
*   Markdown tables now reveal pipe separators in the row you’re editing and alignment markers when you enter the header.
*   `Copy Link to This Position` is now directly below `Comment` in the thread’s right-click menu.
*   Markdown tables now reveal inline formatting markers on the row you’re editing.
*   File links in tool calls now use consistent neutral underlines.
*   Click a filename in a comment summary to open the file at the comment’s location.
*   Returning to a thread now restores focus to the conversation, pane, or panel you were using.
*   Hovering a message or comment avatar, or a rail marker, now shows when it was sent.
*   Generated thread titles now account for delivered comments and the agent’s tool activity.
*   New subthreads now show where they were forked from and link back to their parent thread.
*   Nested threads and subagents now inherit their parent thread’s sidebar color.
*   The subthread link tooltip now explains that the parent thread must be shared before its link can be copied.
*   Thread owner avatars in the sidebar now use color when the owner is confirmed active and appear dimmed otherwise.
*   Copying a private thread link now warns that only people with access can open it and offers a shortcut to update access.
*   New reviews now show their starting skill message instead of a guide preparation fold.
*   When multiple landing skills are configured, `Land Changes` lists them in its subthread and directs you to invoke one explicitly through the slash menu.
*   The `Land Changes From This Thread` menu now lists previous Land threads for quick reopening.
*   Delta Web now warns that mobile Firefox is unsupported, with an option to continue anyway.
*   Symbolic links in the project panel now have an arrow indicator that shows their target path on hover.
*   The sidebar now has a labeled `Update` button with consistent loading states.
*   Delta recognizes and redacts more GitHub, Google, Microsoft, Heroku, and Fly.io tokens, including long tokens split across streamed output.

## [🔧 Fixes and reliability](#0.17.0--fixes-and-reliability)

*   Open subthread and subagent tabs now reopen with your desktop workspace after restarting Delta.
*   Creating a subthread after a change guide no longer leaves empty blocks above the conversation.
*   New subthreads no longer show extra space above their first message.
*   Empty assistant messages now keep their own row and model icon in the thread.
*   Comment boxes no longer jump up and down while an agent writes a subagent prompt.
*   Copying a fully selected table now includes its outer Markdown pipes and adjacent spaces.
*   Markdown list markers no longer wrap onto a line by themselves.
*   `cmd-f` and the Find menu action now search subagent conversations in tabs and standalone views.
*   `cmd-w` now closes the focused project or changes panel instead of archiving the thread.
*   The cursor no longer appears one line too low while a thread is loading.
*   Subthread diff buttons and file links now reliably open the subthread as primary after returning to its parent on desktop.
*   `cmd-{` and `cmd-}` no longer jump to unexpected threads after returning from a child tab to its parent.
*   `Mark as Read` now clears unread indicators on archived threads with folded unread subthreads.
*   Folded project headers now use the thread running indicator without repeating it when the active thread belongs to that project.
*   On touch devices, the turn rail now ignores taps and shows previews only while holding or sliding along the screen edge.
*   Thread event popovers now show correctly proportioned icons.
*   Adding or removing projects from a thread no longer changes the spacing between messages.
*   Links in Markdown tables now reveal their syntax only when the cursor touches the link, rather than anywhere on the row.
*   Math formulas now render in Markdown table cells.
*   Arrow keys now move into and out of display math in the message composer without skipping to the top of the formula or jumping back into it.
*   Clicking the source link on a tool-output page now unfolds the group containing the original tool call.
*   File links copied between threads now open matching files in the current thread.
*   Zooming and panning a Mermaid diagram no longer crashes Delta, and the diagram sharpens as soon as it finishes re-rendering.
*   Git fetches after joining a shared thread now prompt for SSH passphrases and hardware-key PINs instead of failing when credentials are needed.
*   Shared threads can recover older history that previously caused synchronization errors.
*   Recovering rejected offline changes no longer crashes when older work is preserved in a separate thread.
*   Project files now become available to collaborators when an initially failed share succeeds.
*   Recovered local changes in shared threads no longer fail to sync again after receiving a collaborator’s edits.
*   Collaborators’ cursors now appear in the same frame as the text they typed, instead of briefly lagging behind it.
*   Review verdict badges now display consistently on reviewer avatars.
*   Pulling review changes in the web app now reliably shares the merged files with other devices and collaborators.
*   Clicking a subthread’s diff button now opens that subthread as the primary conversation with its diff visible on desktop and web.
*   Branch diffs now become available after creating the first commit in a local-only repository.
*   Starting a review comment with Japanese or Korean input no longer duplicates or drops the first character.
*   Review verdict summaries now describe the review’s final outcomes instead of already-resolved requests.
*   Pressing `escape` in a subagent conversation now stops that subagent.
*   Agent file edits no longer fail with spurious “path changed” errors.
*   Tabs opened from newly attached skills now show their original YAML frontmatter.
*   Agents ask you to attach a missing project to the thread before editing its checkout directly.
*   Context compaction now identifies the model performing it, including when switching to a model with a smaller context window.
*   Thinking-effort and model dropdowns now have matching heights in subagent settings.
*   Newly available ChatGPT subscription models now appear in the model picker.
*   Threads using an existing local checkout with Git LFS files no longer stall before the agent responds.
*   Opening a project after switching organizations no longer reuses that project’s identity from the previous organization.
*   Fetching and pulling Git history from a thread’s terminal no longer fails on unrelated threads’ commits.
*   The status bar no longer briefly shows an internal ID while a project’s name is loading.
*   Menus now support `ctrl-p` and `ctrl-n` to select the previous and next item.
*   Dragging a text selection past either horizontal edge of the file pane now scrolls long lines when soft wrap is off.
*   Settings categories now stay collapsed until expanded with their chevron or a double-click, rather than expanding when selected.
*   Launching Delta from an inline terminal in a Nix project no longer switches your shell to Bash.
*   Keyboard focus now returns to the previous control after closing `Send Feedback` or `Keymap` opened from the account menu.
*   Expired sign-ins no longer show a misleading “invalid response” error when Delta starts.
*   `cmd-opt-left` and `cmd-opt-right` (`ctrl-alt-left` and `ctrl-alt-right` on Windows and Linux) now switch tabs instead of threads when the file pane is focused.
*   Project panel rows now grow with the UI font size.
*   Comment buttons in files now align with the text baseline on headings and other tall lines.
*   Images copied in macOS Finder now paste into threads and Markdown files as images rather than filenames.
*   Opening Delta no longer triggers sign-in prompts from cloud credential helpers.
*   The About window can now be closed with `cmd-w` on macOS and `ctrl-w` on Linux.
*   Markdown tables in rendered files now use the configured prose font.

## [0.16.1 Sep 18, 2026](#0.16.1)

## [🔧 Fixes and reliability](#0.16.1--fixes-and-reliability)

*   Scrolling upward on iOS no longer jumps backward near collapsed Thinking blocks.
*   Sharing threads now works with more large repositories.
*   Shared threads sync more reliably when they contain large files.
*   Selecting a file tab while following someone no longer unexpectedly switches you back to their file.
*   The web app no longer intermittently crashes when its server connection drops or reconnects.
*   The web app avoids an intermittent crash when sharing cursor and scroll positions with collaborators.

## [0.16.0 Sep 16, 2026](#0.16.0)

### [⚡ Better than before](#0.16.0--better-than-before)

*   Invalid LaTeX formulas now show an error message below their source.

### [🔧 Fixes and reliability](#0.16.0--fixes-and-reliability)

*   Scrolling to the end of a thread with an unsent draft no longer jumps on iPhone.
*   Long messages sent between parent threads and subthreads now wrap instead of being cut off.
*   Unavailable Delta worktrees now show an error with `Retry` instead of remaining stuck in preparation.
*   Fixed an intermittent crash while scrolling threads in the browser.
*   TypeScript files in the file panel now retain distinct syntax highlighting inside callback functions.

## [0.15.0 Sep 16, 2026](#0.15.0)

### [✨ New this release](#0.15.0--new-this-release)

*   Open the link under your cursor with `opt-enter` on macOS or `alt-enter` on Windows and Linux, regardless of your send shortcut setting.

### [⚡ Better than before](#0.15.0--better-than-before)

*   Subthreads get more specific titles that are easier to tell apart.
*   Paged tool-output reads now display descriptive titles and links to their source.
*   Agents in subthreads focus on the new request rather than resuming unfinished parent work unless asked.

### [🔧 Fixes and reliability](#0.15.0--fixes-and-reliability)

*   Clicking a file or skill link in a subthread tab now opens that subthread as primary and shows the linked content.
*   Opening a subthread on Windows no longer crashes from running out of stack space.
*   Inline images now fit the line-wrap width and keep the cursor at their visible edge after deleting the following newline.
*   Pulling edits from a review thread is now faster.
*   Subagent work now merges successfully when repeated edits remove previously merged text.
*   Signing into a different ChatGPT or Grok account now updates the shown account and uses the new account for requests.
*   Changing the app icon now updates the macOS Dock icon immediately.

## [0.14.0 Sep 16, 2026](#0.14.0)

### [✨ New this release](#0.14.0--new-this-release)

*   The `/subthread` command now creates a blank subthread.
*   Subthreads can now be renamed and marked as read or unread from the thread sidebar.
*   You can now `@`\-mention directories to give the agent a folder to work in.
*   You can now ask Delta to inspect a subagent’s conversation from its parent thread to help diagnose stalled or failed work.
*   Choose Delta’s app logo from the new `App Icon` setting in Appearance.

### [⚡ Better than before](#0.14.0--better-than-before)

*   Subthread and subagent events now use one button that opens a tab on click and the primary thread on `cmd-click` (`ctrl-click` on Windows and Linux).
*   The `Pull Changes` button now says `Changes Pulled` after review edits are pulled.
*   New worktrees of repositories Delta clones for you use shorter paths, leaving more room for source files and build output on Windows. Worktrees of repositories opened from your own machine stay where they were.
*   Diff indicators and touch editing actions now use more balanced icons.

### [🔧 Fixes and reliability](#0.14.0--fixes-and-reliability)

*   `cmd-1` now returns to the main thread conversation from a subthread tab.
*   Notifications no longer cover toolbar controls while viewing a subthread or subagent tab.
*   Pending comments now stay attached to the draft where they were created when collaborators add other drafts.
*   Opening a subagent’s thread no longer stops the thread from syncing with a “The server could not provide a required history checkpoint” error.
*   Opening a subthread from its parent event no longer changes the parent thread’s scroll position.
*   Starting a comment with `enter` no longer inserts an unwanted initial newline.
*   Deleting a thread no longer fails because of a previous synchronization error.
*   Collapsed parent threads now show an unread indicator when a subthread is unread.
*   Created Subthread events now match other subthread events, with selectable text and consistent spacing.
*   The web app avoids a crash that could occur while connecting or reconnecting to a shared thread.
*   Opening a shared thread no longer fails with a misleading offline error when your session expires.
*   Threads with existing review subthreads can now be shared after adding a Git remote to a previously local-only repository.
*   Review creation events now show the generated review title instead of the parent thread title.
*   Pull Changes now completes for large files even when line-change counts are unavailable.
*   The agent now waits for a newly attached repository to finish preparing instead of failing its tool calls with worktree “offline” errors.
*   Reading temporary files and displaying screenshots no longer fail when the agent also specifies a worktree.
*   Terminal commands on Windows no longer hang when canceled or timed out.
*   Unsent text drafts no longer prevent the parent agent from responding when a subagent finishes.
*   OpenAI context compaction no longer fails after a minute without a response.
*   When Fable declines a request, Delta now switches to Opus 5 and shows a notification explaining the change.
*   Gemini responses no longer disappear and restart after the model finishes answering.
*   Windows repositories containing symlinks can now be opened with `core.symlinks=false`.
*   New worktrees no longer gain `~` suffixes when repository names collide, avoiding compatibility problems with tools such as Vite.
*   Settings now opens as a floating window on Linux tiling window managers.
*   Failed background connections now retry with growing intervals instead of reconnecting in a tight loop, and a history whose connection goes silent while opening recovers after 45 seconds.
*   Delta now restores its non-fullscreen window to its previous screen and macOS desktop after restarting.
*   Secrets remain redacted until Delta exits, even after you change or remove their source files.

## [0.13.0 Sep 15, 2026](#0.13.0)

### [Features](#0.13.0-features)

*   Click a subthread’s title in its tab toolbar to rename it.
*   All subthreads now appear above subagents in the sidebar.
*   Regular subthreads now receive automatically generated titles.
*   Shared plain-subthread links now open the parent at the originating prompt with the subthread in a tab on wide layouts.
*   Subthread tabs now show compact labels, with titles appearing only when multiple tabs of the same kind are open in a pane.
*   Thread event badges now use icons designed for their compact size.
*   New users can now create a team organization and invite teammates during onboarding.
*   Use `Pull Changes` on review feedback to bring the edits included when that review was submitted into the original thread.
*   Slash command autocomplete now shows descriptions for selected skills.
*   Subagents can now be opened directly from folded spawn cards.
*   ChatGPT subscription usage-limit errors now link to current limits, reset times, and available options.
*   Model-backed actions now explain how to restore model access without changing the thread when no model is available.
*   `Land Changes From This Thread` is now available from the thread title bar.
*   The folder-picker action is now named `Open Git Project…` instead of `Add Project Folder…` to clarify that it requires a Git repository.
*   Feedback reports can now include an editable model name and optional recent app logs.
*   You can now keep shared project configuration in `.agents/` and use `.delta/` for Delta-specific overrides.
*   Settings pages now provide expandable links for navigating directly to their sections.
*   More languages and Nerd Font symbols now render correctly in Delta Web.

### [Bug Fixes](#0.13.0-bug-fixes)

*   Image loading placeholders are now readable in dark mode.
*   Avatars on mobile no longer overlap messages or review blocks.
*   Subthread status tooltips in the message rail now show the subthread name and event title instead of “Loading participant…”.
*   Thread content no longer jumps while terminal progress displays redraw.
*   Plain subthreads can now be archived from the sidebar or with `cmd-w` when opened as the primary thread.
*   Retrying recovery of unsynced changes no longer fails with “thread not found.”
*   Typing below a comment summary no longer removes an extra blank line above your message.
*   File links in running editing tools now show a loading tab while waiting for a file to become available.
*   Delta Web no longer crashes when opening `Copy Options…` with the keyboard or dismissing its submenu with Escape.
*   Search highlights no longer briefly disappear when editing a matching line in a thread.
*   LaTeX formulas enclosed in `\[...\]` now render as math instead of raw text.
*   The down arrow now reaches your message composer when a collaborator’s draft ends with blank lines.
*   Reopening a thread after an interrupted upload no longer incorrectly stops synchronization.
*   Clicking multiline or wrapped diff comments now places the cursor at the clicked text.
*   Wrapped lists in change guides now keep their indentation when scrolling.
*   Review share links now update immediately when the parent thread’s access changes.
*   Starting a review of someone else’s shared thread in the browser no longer leaves an empty review after a permission error.
*   Mac keyboard shortcuts for selecting text and deleting comment text now work in web diff views.
*   Loading a project skill now reports when it is unavailable instead of running a same-named global or built-in skill.
*   Subagents now retry stalled result synchronization without losing their finished work, and stopping an idle subagent no longer interrupts completion recovery.
*   Subagent failures now show an error popup in the parent thread that names the failed subagent and explains the error.
*   Interrupted commands and their saved output now remain available to the agent after restarting Delta.
*   Agents can now edit files that an editor replaced without changing their contents.
*   New worktrees no longer drop a sole Git remote just because it is named `upstream` instead of `origin`.
*   Threads blocked by an unaccepted Xcode license now explain how to resolve it and can retry worktree setup.
*   Clicking a file’s symbol breadcrumb now opens its outline even when keyboard focus is elsewhere.
*   The `Send Feedback` form now stays open while moving the Delta window.
*   On mobile web, the text-selection toolbar now closes after tapping an action.
*   Very long lines now display a shortened preview instead of freezing the editor.
*   GitHub OAuth access tokens with the `gho_` prefix are now automatically redacted.
*   Localhost URLs without credentials, queries, or fragments are no longer incorrectly redacted as secrets.

## [0.12.0 Sep 14, 2026](#0.12.0)

### [Features](#0.12.0-features)

*   The `Constrain Wide Content` setting can now limit wide thread blocks to the selected content width while preserving usable terminal output.
*   Send any message to a new subthread on desktop or web with `cmd-shift-enter` (`ctrl-shift-enter` on Windows/Linux), or `cmd-enter` (`ctrl-enter`) when `Send With Modifier` is off.
*   Improved thread performance.
*   Inside a code block, `cmd-a` now selects its code, with repeated presses expanding to the current turn and then the whole thread.
*   Pasting and viewing very long lines in threads is now much faster.
*   Concurrent comments on the same target now appear together in one conversation.
*   You can now start reviews from the eye menu in the browser, open review threads, and submit review verdicts.
*   Review verdicts now include a control to reveal the matching event in the parent thread.
*   The review menu is now always available in parent threads and excludes archived reviews.
*   Review events now have consistent spacing in parent and review threads.
*   Review threads now show `Generating title…` until they receive a short title describing the reviewed change.
*   ChatGPT subscriptions now choose Scout and Reviewer defaults that fit each plan’s usage allowance.
*   Project and branch controls now share one menu, and new threads clearly indicate when they inherit an existing checkout.
*   Touch and hold a button to view its tooltip without activating it.
*   The project files view now has a draggable scrollbar for navigating long file lists.

### [Bug Fixes](#0.12.0-bug-fixes)

*   Dragging the sidebar closed on macOS now previews the final layout without placing the thread title under the window buttons.
*   You can now reply to file comments directly in the thread’s unsent comment summary without opening the file view.
*   File panes folded beside an open panel now remain discoverable through a narrow restoration control.
*   Replying to an unsent transcript comment in the bottom comment summary now keeps the cursor there instead of jumping to the original comment.
*   File links in shared threads now wait for files to become available instead of incorrectly reporting that their worktree is no longer attached.
*   Reverting a conversation while editing an earlier user message no longer leaves the thread uneditable.
*   The sidebar no longer jumps when you remove the open thread with `Delete Locally`.
*   The command palette’s Recent Threads section now prioritizes threads you prompted most recently.
*   Using `Discard Local Changes` no longer lets a background sync check bring those changes back.
*   The opening control on `Merge Thread` rows stays visible when long titles wrap in narrow browser windows.
*   Joining a shared thread now starts in an isolated checkout rather than your existing local checkout.
*   Review submissions no longer show an error when their optional summary cannot be generated.
*   Branch diffs now refresh after deepening or unshallowing a Git repository.
*   A crash while waiting for local edits to save during browser review creation has been fixed.
*   OpenAI requests no longer wait indefinitely when a connection stalls.
*   Model request errors now explain what failed instead of only showing “Request failed.”
*   Files deleted by Git no longer reappear as unexpected untracked files when switching branches or updating a checkout.
*   Delta no longer crashes after a failed operation loads data saved by an older version.
*   On mobile web, tapping editable text reopens the keyboard after manually dismissing it.

## [0.11.0 Sep 11, 2026](#0.11.0)

### [Features](#0.11.0-features)

*   You can now select and copy filenames and quoted code directly from file-comment summaries.
*   The mobile text popup now lets you copy a selected link’s address or a link to the current location in a shareable thread.
*   The mobile bottom bar now has an editing mode with undo, redo, and bold, italic, and inline-code formatting controls.
*   You can now ask a thread’s agent to bring in changes from its review threads and other subthreads.
*   Subthreads can now post progress, success, and failure updates directly in their parent thread.
*   New subagent views opened from the sidebar now show the latest activity.
*   Selecting Delta from the mention dropdown now inserts `@delta` in your message.
*   On mobile, text-selection controls now appear beside the selection instead of replacing the bottom bar.
*   Private thread owners can now copy a link without changing who has access.
*   Other participants no longer see your cursor while it rests in your personal empty message box.
*   Subagent profiles support a `prompt` field for adding instructions without replacing Delta’s standard system prompt.
*   Subagent profiles support `worktree = "shared"` or `"isolated"` to choose whether they share the parent’s checkout or work in a private copy.
*   The new `Skills` settings page manages global and project agent skills and opens them in Zed.
*   Use `Actions → Land Changes` to have the agent research your project and help you create a custom landing skill.
*   `UI Font Size`, `Prose Font Size`, and `Code Font Size` can now be configured independently in logical pixels.
*   Delta now prompts you to update when your local data was saved by a newer version.
*   The web app downloads about half as much code, opens faster, and uses less memory in the browser.

### [Bug Fixes](#0.11.0-bug-fixes)

*   When typewriter mode is locked, deleting a line with Backspace now retains the cursor’s viewport position.
*   Archived threads reopened from history now return to the sidebar.
*   Expanded sent agent messages now use the same indented section style as received messages.
*   Thinking blocks and tool groups no longer collapse automatically while your cursor is inside them or one of their comments.
*   On mobile web, the bottom toolbar stays above the onscreen keyboard without resizing the thread.
*   Interrupting an agent before its first response now preserves terminal formatting in legacy drafts saved by older Delta versions.
*   Agent comment replies no longer show a stray closing tag when the response continues on the same line.
*   The cursor now stays visible briefly after you click to place it.
*   Links in file-comment excerpts now open with the keyboard using the same shortcut as other links in a thread.
*   New empty agent replies to comments now show the comment they reference instead of disappearing.
*   Folding a sidebar thread now hides nested threads open in tabs, while keeping a nested thread visible when it is the primary thread.
*   Comments on deleted text no longer appear attached to replacement text in file views, while remaining available in the thread transcript.
*   Joining threads no longer fails with a “Delta token expired” error.
*   Opening a shared thread no longer starts another collaborator’s pending agent continuation on your machine after a subagent replies.
*   A shared thread whose history the server can no longer provide stops reconnecting in a loop; its connection indicator turns red and offers `Retry`, and your local changes are kept.
*   Undo now restores draft comments canceled with `escape` in diff views, including their text and cursor position.
*   Change guides no longer end with extra blank lines.
*   Delta no longer sends response-completion notifications while subagents are still working.
*   Delta no longer crashes when a thread contains a message sent to an agent whose name is not written in English.
*   Failed subagents now show their error when you open or focus their conversation.
*   OpenAI and ChatGPT requests now recover from stalled connection attempts instead of waiting indefinitely.
*   Switching hosted-model organizations no longer briefly shows an empty model list or keeps models after access is denied.
*   Delta worktrees no longer fail to prepare when a repository contains long filenames.
*   Preparing Linux worktrees is less likely to fail with an “OS file watch limit reached” error.
*   File editors on the web now keep keyboard focus when you click to place the cursor.
*   On mobile web, switching views no longer reopens the keyboard, and clipboard controls and Comment Mode preserve editing focus.
*   On mobile web, the cursor stays above the onscreen keyboard while editing threads, files, and review comments.
*   In narrow desktop browser windows, search stays focused after Enter and newly opened files and reviews accept keyboard input immediately.
*   On macOS, opening a downloaded copy of Delta now offers to restart the installed app instead of replacing it.
*   Sending feedback with a screenshot no longer fails as temporarily unavailable.

## [0.10.0 Sep 10, 2026](#0.10.0)

### [Features](#0.10.0-features)

*   Use `Restart Welcome Guide` to start a fresh copy of Delta’s interactive welcome guide.
*   Background thread notifications now preview the agent’s final response.
*   Manual compaction failures now show a `Retry` button so you can try again without losing your conversation.
*   Review threads and subagents are now easier to navigate from their parent threads and the thread sidebar.
*   The thread sidebar now stays responsive with large thread histories.
*   Hovering the `View Reply` checkmark on a replied comment now shows a directional arrow indicating that clicking jumps to the reply.
*   Sent subagent messages can now be expanded to show their message text.
*   Choose which organization supplies your Zed-hosted models from the model selector in Delta Web.
*   Onboarding now groups Zed, subscription, and API-key options into tabs and lets people choose which Zed organization provides hosted models.

### [Bug Fixes](#0.10.0-bug-fixes)

*   Subagent completions no longer change your scroll position when the assistant starts responding.
*   Switching between rendered LaTeX blocks and their source now causes less layout shift.
*   Hovering or focusing a comment now reveals only that comment’s navigation button.
*   Fixed a cause of scrolling stalls around collapsed content in long threads.
*   The cursor no longer gets stuck when navigating through read-only text in Delta Web.
*   Expanded agent-spawning cards now show the typing indicator after the task text instead of in the title.
*   A review thread’s `Copy Share Link` button now enables immediately after the parent thread’s share access changes.
*   Collaborator cursor labels are no longer clipped inside terminal and formatter cards.
*   Composer and comment avatars no longer show the same person multiple times when they contribute from multiple clients.
*   Shared threads no longer get stuck during reload when discarding certain rejected local changes.
*   Review verdicts no longer appear below agent responses that use their feedback.
*   Review event cards now accept comments on their text.
*   Review threads now clear their unread indicator when focused in a tab.
*   Subagent completions no longer appear as raw formatting text in threads.
*   The `Subagents` settings now use consistent text, control sizing, and action spacing.
*   Pressing `tab` in the command palette now opens Ask Delta instead of moving through search results.
*   Switching between Claude models no longer converts earlier thinking into ordinary message text.
*   ChatGPT subscription and OpenAI models now automatically retry interrupted connections instead of immediately stopping with “Request failed.”
*   New isolated workspaces are created under your local checkout when it is available, including after joining a shared thread.
*   Delta now ignores sockets and other special files.
*   Quitting Delta no longer crashes when background connections are still active.
*   Inline terminals now open even when direnv is unavailable or its setup fails.
*   In Delta Web, `ctrl-f` on Windows and Linux and `cmd-f` on macOS now open Delta’s search instead of moving the cursor or opening browser search.

## [0.9.1 Sep 9, 2026](#0.9.1)

*   No user-facing changes

## [0.9.0 Sep 9, 2026](#0.9.0)

### [Features](#0.9.0-features)

*   You can now pin or unpin the current thread with `cmd-opt-p` on macOS or `ctrl-alt-p` on Windows and Linux.
*   Collapsed sidebar sections and parent threads now keep the current thread’s title visible.
*   Loading threads now have a pulsing sidebar title with a tooltip explaining what is being loaded.
*   Choose `Subagents` → `In-Progress Only` in the sidebar filter menu to hide finished subagents across the sidebar.
*   Expanded threads now show the eight most recent subagents matching your filter, with a button to show the rest.
*   Threads with large comment summaries now scroll and redraw much faster.
*   `cmd-[` and `cmd-]` on macOS, or `alt-[` and `alt-]` on Windows and Linux, now navigate to final responses in long agent turns and keep the empty message editor visible.
*   Review threads can now be archived from the sidebar and restored from thread history.
*   The Review Changes toolbar now clearly separates project and comparison selectors.
*   Delta now supports subagents.
*   Customize Worker, Scout, and Reviewer subagents with model preferences and your own profiles in `Subagents` settings.
*   Agent searches across large repositories are faster.
*   Sent agent messages now use consistent tool styling, with clickable recipients that open the corresponding message.
*   LLM provider settings now show which account is connected and any additional account details available.
*   Model lists for Anthropic, Baseten, OpenRouter, OpenCode, and Grok now recover from temporary provider errors such as rate limits instead of failing immediately.
*   When ChatGPT reports when a usage limit resets, Delta waits until then before retrying.
*   You can now send feedback directly from Delta with `Send Feedback` in the Help or user menu, or `/delta-feedback` in a thread, optionally attaching a screenshot and leaving an email or Discord handle so we can follow up.

### [Bug Fixes](#0.9.0-bug-fixes)

*   URLs enclosed in angle brackets, such as `<https://example.com>`, now render as clickable links.
*   Marking threads as read or unread no longer hides their running indicators.
*   Commenting with the cursor at the end of a bold or otherwise formatted paragraph now highlights the whole paragraph instead of only its last character.
*   Copying a fully selected rendered Markdown link now preserves its surrounding link syntax, including at the end of an assistant response.
*   Your comment mode preference now applies across all threads and is remembered when you restart Delta.
*   The scrollbar now keeps moving while you scroll through a tall comment summary instead of sticking in place.
*   Collapsed sidebar sections and thread rows now keep their active nested item visible without redundant sync indicators or shifting controls, while new review threads use their parent title until a generated title is ready.
*   The streaming cursor on tool, compaction, and guide-preparation titles no longer shifts the title as work starts and finishes.
*   You can now edit and resend the user message immediately above a revert notice.
*   Your avatar no longer disappears or moves onto the following assistant response when you delete all the text while editing a past message.
*   `cmd-down` on macOS and `ctrl-end` on Windows and Linux now reach your empty composer from anywhere in another participant’s unsent draft.
*   Responses triggered by subagent messages now appear above unsent drafts instead of leaving an empty draft stranded in the conversation.
*   Undo no longer removes your message after a collaborator sends it.
*   Sharing threads now works in more cases when a GitHub remote URL contains uppercase owner or repository names.
*   Clearing a draft now keeps its avatar and typing position until someone sends, without merging neighboring messages.
*   Inserting a newline at the end of a draft no longer makes the cursor appear in the next participant’s draft.
*   Opening a review in a tab now clears its unread indicator.
*   Review threads no longer offer unusable share links before someone else can access the parent thread.
*   The `Review Changes` button now remains highlighted while its right-click menu is open.
*   The diff search box no longer intercepts clicks intended for the comparison dropdown above it.
*   `Create New Review` now creates another review while an existing review is open.
*   Agent turns that make progress now retain their automatic retry allowance after a restart.
*   A subagent finishing its work after a reconnect no longer sometimes leaves the thread asking you to discard local changes.
*   The agent’s pulsing cursor now stays visible when subagent messages arrive while it is responding.
*   ChatGPT subscription models now use their largest supported context window before automatic compaction.
*   OpenRouter model-availability errors now show the provider’s explanation with options to select another model or configure the provider.
*   Delta worktrees no longer fail to open with a “shallow boundary commit is missing” error caused by stale Git metadata.
*   Thread CI indicators now follow commits pushed from that thread instead of unrelated updates to the same remote branch.
*   Branch commit messages no longer flicker during branch refreshes.
*   Switching branches in a thread that works in your checkout no longer moves the branch you were on to the new branch’s commit.
*   Delta preserves concurrent file edits when writing to a local checkout, and reports an error rather than claiming success if the checkout keeps changing.
*   New threads now preserve the existing local checkout choice from the previous thread.
*   Branch diffs more reliably exclude upstream changes when collaborating with shallow Git repositories.
*   The Scroll Lock key now toggles typewriter mode on macOS.
*   The macOS-only `Hide Delta` and `Hide Others` commands no longer appear on Windows or Linux, preventing a Windows crash when selecting `Hide Others`.
*   Using `Open Log Viewer` while the log viewer is focused no longer crashes Delta.
*   Delta’s web app no longer stalls while loading cached images.
*   macOS Automation access requested by Delta is now correctly attributed to Delta.

### [Removed](#0.9.0-removed)

*   Older agent replies no longer show clickable reply chips; their original tag and answer text remain visible.

## [0.8.0 Sep 8, 2026](#0.8.0)

### [Features](#0.8.0-features)

*   Mobile web threads now support browser Back, Forward, and direct links when reviewing changes or viewing files.
*   Sending a message keeps your scroll position when there is already enough room to show the response.
*   The `Copy Share Link` action is now named consistently and uses `cmd-alt-l` on macOS or `ctrl-alt-l` elsewhere.
*   Messages and comments with three or more authors now show one avatar and a count instead of a crowded stack.
*   The sharing modal is now fully keyboard navigable with Tab, Shift-Tab, and Enter.
*   Review creation rows now have selectable text and keep their navigation controls aligned with the transcript.
*   Finished subagents now use less disk space by automatically cleaning up idle checkouts and restoring them when needed for follow-up work.
*   ChatGPT subscription models now respond faster during multi-step tool use.
*   CI details and the branch picker now show the commit associated with each branch.

### [Bug Fixes](#0.8.0-bug-fixes)

*   Tool output and review events following a list no longer inherit its indentation.
*   The thread navigation rail no longer skips review events when dragging or using `opt-scroll` / `alt-scroll`.
*   Reverting a conversation now keeps retained messages in sent history instead of turning them back into drafts.
*   Thread sync warnings now wrap correctly in narrow windows.
*   The thread list keeps its grouping and new-thread controls available when there are no visible threads.
*   Resizing the right sidebar no longer pushes thread and file header controls off-screen in narrow windows.
*   The line counts next to the comparison base in the Changes view now cover only the selected worktree instead of every attached worktree.
*   Review-thread events now use the same spacing as user messages, without extra blank space between them.
*   Review dropdown tooltips no longer remain visible when the menu is open.
*   Switching between OpenAI models now preserves their original thinking for the next model.
*   Cancelled GitHub Actions runs no longer appear as failed CI checks.
*   CI status details now appear below the branch and commit in the status popover.
*   Unavailable CI status now uses an interrupted-state icon instead of a disconnected-state icon.

## [0.7.0 Sep 7, 2026](#0.7.0)

### [Features](#0.7.0-features)

*   Press `alt-enter` in file tool output and change guide diffs to open the corresponding file location.
*   Text selections on touch devices now have draggable handles for adjusting the selected range.
*   Files opened from a thread now appear in a dedicated viewer on narrow browser windows.
*   The mobile thread bottom bar now adapts to text selections and comments with relevant editing and navigation actions.
*   The welcome tutorial now explains how to open its terminal, run the example command, and send the completed message.
*   Delta now limits memory used by inactive threads while keeping active work running in the background.
*   Specialized copy actions in thread context menus are now grouped under `Copy Options…` when multiple choices are available.
*   The “Jump to End” tooltip now shows your configured keyboard shortcut.
*   On mobile, you can now select code and add comments while reviewing changes.
*   Commit diffs now link to the shared Delta thread that produced the commit when one is available.
*   Middle-clicking a review diff now starts continuous autoscrolling.
*   Change guides now present the author’s intent and user-visible outcomes more clearly.
*   Review threads are named after the work under review, and a follow-up review notes what changed since the last one, such as “Retry backoff fix (added tests)”.
*   A new review thread folds the agent’s work behind a “Preparing guide…” row until the change guide starts streaming.
*   Agents on Windows can now run POSIX shell commands through Git Bash as well as PowerShell cmdlets and scripts.
*   The welcome thread now answers questions about Delta using your configured model.
*   Agents now pick up directory-specific guidance from nested `AGENT.md` and `AGENTS.md` files.
*   Agent activity spinners now match the color of the typing cursor and active terminal border.
*   Application-level settings are now grouped on a new General page.
*   The `Default Shell` setting now shows your system shell as its placeholder, with long paths shortened in the middle.

### [Bug Fixes](#0.7.0-bug-fixes)

*   The right-side drawer now remains open after restarting Delta.
*   Thread connectivity status no longer reports failures from obsolete worktrees.
*   Parent responses no longer have extra blank lines after subagent completions.
*   Text in thread comments can now be selected with mobile touch gestures.
*   Selecting text on iOS no longer also selects the entire Delta window.
*   Clicking the highlighted comment button now hides expanded comment threads without reopening the picker.
*   Threads with local changes that diverged from the server now show a single recovery warning.
*   Shared thread message popovers now show the model only on the message that submitted the agent request.
*   Threads no longer show extra blank space where an agent turn ended without content.
*   Draft lines selected with a triple-click can now be deleted or replaced by typing.
*   Slash-command and @-mention menus now stay open and preserve your selection while the agent streams a response.
*   Long-pressing text on iOS no longer intermittently selects the whole page or shows duplicate selection handles.
*   Comments now appear alongside an agent’s reply while it is still streaming, rather than waiting for the reply to finish.
*   Review thread entries no longer show extra blank lines after a compaction or another review entry.
*   The mention picker now shows the full Delta help description before a category is selected.
*   Comment navigation tooltips now stay above nearby controls and show the available `alt-enter` shortcut while editing comments.
*   Adding large files no longer causes shared-thread synchronization to exceed message size limits.
*   Shared threads with subagents can now open when they previously stayed on “Planning sync”.
*   Change guides now display excerpts when their comparison uses the worktree’s committed `HEAD`.
*   The `Previous File Tab` and `Next File Tab` shortcuts now switch away from Review tabs instead of changing threads.
*   Submitting a review verdict now marks the current choice and no longer posts duplicate approval or change-request messages.
*   Review verdict cards no longer disappear when you start typing a new message.
*   Opening a review thread while its parent is still loading no longer crashes Delta.
*   Starting a comment from an `if` condition now selects only the condition instead of the entire `if` expression.
*   New review threads no longer hide their generated change guide behind an empty pane.
*   Agent terminal commands now automatically load applicable direnv environments.
*   Failing agent turns no longer gain extra automatic retries when Delta restarts.
*   Active agents now process subagent replies without waiting for you to finish an unsent draft.
*   OpenAI requests now recover automatically when service disruptions incorrectly return 404 errors.
*   The branch diff now updates when the default branch moves, such as after landing or merging the branch, instead of only when the checked-out commit changes.
*   A conflicted `git merge` started right after a commit or reset in a mounted worktree no longer loses its merge state.
*   Delta worktrees can now be created for repositories containing branch names that differ only in capitalization.
*   Managed worktrees on Windows now support Git metadata paths longer than 260 characters.
*   Git history in agent worktrees no longer stops at commits whose parents Delta has already fetched.
*   Terminal text colors that blend into the background are now adjusted to improve readability, including PowerShell input on light themes.
*   Opening a project or thread when launching Delta now survives signing in without creating an extra empty thread.

## [0.6.2 Sep 7, 2026](#0.6.2)

### [Bug Fixes](#0.6.2-bug-fixes)

*   ChatGPT subscription models now retain the conversation context after compaction.

## [0.6.1 Sep 4, 2026](#0.6.1)

### [Bug Fixes](#0.6.1-bug-fixes)

*   Chinese and Japanese input methods on macOS no longer leave the first Romaji character in an empty thread composer.

## [0.6.0 Sep 3, 2026](#0.6.0)

*   Thread editors can now cycle the cursor between the viewport center and its top and bottom scroll margins with `ctrl-l`, `cmd-alt-z z`, or `cmd-alt-z cmd-alt-z`.
*   On touch devices, pressing and holding thread text now selects a word, and dragging extends the selection by whole-word boundaries.
*   Uploaded and joined threads now appear automatically on every device signed in to the same account, with their projects ready to use. Pinning, archiving, and deleting a thread applies on all of those devices, and renames follow the thread wherever it is open.
*   On mobile, selections in agent responses now show a button for adding a comment.
*   Comment targets and quoted selections now use rounded highlights that preserve their underline.
*   The `Open in Zed` action now opens the active worktree file at the current cursor position.
*   When typewriter mode is locked, `cmd-opt-up` and `cmd-opt-down` now use normal cursor movement instead of retaining the cursor’s viewport position.
*   When typewriter mode is locked, inserting a newline now retains the cursor’s viewport position.
*   Thread statuses now use clearer icons and show status details only when hovering the relevant icon.
*   The running terminals menu now identifies each background terminal by its command, scrolls to selected terminals, and provides clearer individual and bulk stop controls.
*   New thread histories now write far less to the local database as they are edited, reducing disk activity in heavily edited threads.
*   Project actions now distinguish between adding a project to Delta with `Add Project`, adding one to the current thread with `Add Project to Thread`, and selecting one from disk with `Add Project Folder…`.
*   Comment summaries now show the selected file text and line number, keeping the selection highlighted when long target lines are truncated.
*   The project menu now explains when Delta does not know the location of an existing local checkout and lets you locate it instead of hiding the checkout option.
*   Adding a project to a thread no longer inserts cursor-addressable blank lines into an agent response.
*   Korean input methods on macOS no longer separate or duplicate the first character typed into a new thread composer.
*   Reverting a conversation now succeeds after its project checkout changes, while leaving the current checkout unchanged and explaining that its earlier state was not restored.
*   Comments on long transcript selections now give agents the complete selected text.
*   The typing indicator now remains visible while an agent processes external changes between response rounds.
*   Math blocks in read-only messages now remain rendered when the cursor is placed at their start.
*   Rendered block math expressions now size to their content instead of filling the row.
*   Comment mode now shows a read-only cursor over file content while keeping comment text editable.
*   Comment summaries now show the first non-empty selected line when a comment starts on blank lines.
*   Large collaborative threads now continue publishing checkpoints after timeline fork metadata changes, avoiding increasingly expensive replay when opening them.
*   Closing a new thread with `cmd-w` while it was still uploading no longer freezes Delta.
*   Jumping from a comment in the thread to its target in the review diff now unfolds the file when it has been marked as reviewed, so the comment and its context are visible.
*   Clicking in a wrapped comment inside a file comment box now places the cursor at the clicked character instead of several characters early. Comment text in a narrow composer is no longer cut off at the box’s right edge.
*   Shared threads containing discarded or rejected edits now load reliably on web and mobile.
*   Thread cards no longer lose their left borders or comment accents on mobile web.
*   Dragging a word selection no longer crashes when thread content changes between frames.
*   Indent guides in tool-call diffs now align with the configured code font.
*   Empty composers in new threads no longer jump or resist scrolling.
*   Typing `!` now opens an inline terminal after clearing all text from a thread draft.
*   `Add Project` no longer crashes Delta when it opens the folder picker directly.
*   Long threads with collaborative draft history now remain responsive.
*   Following a collaborator now moves with them between the conversation, review diffs, and files.
*   “Discard Changes” now succeeds on a thread whose local changes were rejected, even while collaborators keep editing the shared thread or worktree. A discard that fails (for example, while offline) now returns the thread to the retryable discard screen instead of requiring a restart.
*   A collaborator’s cursor now follows them into a comment while they type, instead of staying stuck on the thread text near the comment box.
*   Fixed blank lines piling up below a streaming response in shared threads when a participant commented while two people had sent messages at the same time.
*   Use `/start-review` to review an agent’s work in a separate review thread: comment on the transcript or diff, ask the agent for changes there, and submit an approval or change request back to the parent thread.
*   Empty diff comparisons no longer show zero line counts and use muted text in the review toolbar.
*   Diff line counts now consistently include thousands separators.
*   Diff statistics now follow the selected `Diff Colors` accessibility setting.
*   Last-turn diffs now include the entire agent turn when context is compacted.
*   Use `/change-guide` to get a structured walkthrough of an existing code change with focused inline diffs.
*   Delta Help now explains how to move work from a managed checkout by pushing to `origin` for a shared upstream repository or to `local` for the project repository on your computer.
*   Agents can now inspect previous Delta threads without launching a separate Delta process for each command.
*   When a conversation is too large for a newly selected model and compacting it with the previous model fails, the error now offers a “Compact with ” action that retries the compaction so you can keep going.
*   Running an initial Cargo build in a thread now uses less CPU and disk I/O on Windows.
*   Agents no longer stop after tool failures that are not part of a repeated retry loop.
*   On macOS, Delta now loads personal rules from the documented `~/.config/delta` directory while continuing to recognize rules saved in the previous application support directory.
*   Delta Web now includes `Models` settings for application model roles, thinking effort, and model-picker visibility.
*   Claude Fable 5.1 now works with an Anthropic API key, including recovery from its stricter thinking-block replay rules and support for its server-side compaction.
*   ChatGPT and Grok subscriptions now sync through your Delta account, making connected models available across devices.
*   Model preference controls are now clearer and easier to read in provider settings.
*   Fast, balanced, and frontier model roles now select stronger provider-specific models and appropriate thinking efforts by default.
*   `Compact Now` now works with every model by creating a conversation summary when the provider does not offer native compaction.
*   The model picker’s note about models that can’t replay the thread’s context now stays within the window instead of being cut off at its edge.
*   The undo button in model preferences now appears after changing either the model or thinking effort.
*   Delta now preserves each file’s line endings and honors `core.autocrlf`, `core.eol`, and `.gitattributes` conversion rules, so editing a repository with CRLF files or on Windows no longer rewrites line endings or shows every file as modified in `git status`. Collaborators on different operating systems see the same file contents while each machine’s checkout keeps the endings git would produce.
*   Project checkout locations can now be selected while Git preparation is still in progress.
*   Repositories using GitHub’s SSH-over-443 configuration now register and share correctly instead of being mistaken for `ssh.github.com` repositories.
*   Added a Default Visibility setting under Thread settings to automatically share new threads with everyone in your active organization.
*   Show CI status in threads, once you configure a GitHub PAT in Settings -> Integrations
*   Added a Settings button to the web app header.
*   Added the command palette to Delta Web, available with Ctrl-Shift-P or Cmd-Shift-P.
*   Added thread content width and alignment defaults to Settings while preserving per-thread layout choices.
*   Improved the design of comment summaries in threads: replies from the agent open with a reply icon, answered follow-up exchanges fold behind a comment count, long comment excerpts fade out instead of overflowing, and stray blank lines around comment-only messages no longer appear. Also fixed selections getting stuck when extending past the last reply in a thread.
*   Improved the clarity and alignment of icons throughout the interface.
*   Improved the comment mode shortcut to Command-Option-C on macOS and Control-Alt-C on Windows and Linux.
*   Improved commit comparison labels and diff base menu consistency.
*   Improved pending comment summaries: the “N comments” header now appears in your draft message as soon as a comment has text, stays put while you type below it or delete your draft, keeps your avatar beside it, and can be reached with the cursor while remaining read-only.
*   The thread header’s Share button now displays the current visibility icon.
*   Settings and sidebar indicators now use clearer spacing and labels.
*   Fixed empty agent turns showing an agent avatar on the following human message.
*   Fixed pending comments in collaborative threads appearing in another participant’s composer and corrupting shared thread structure.
*   Fixed the command palette showing file and diff commands outside the views where they apply.
*   Fixed grouped tool calls shifting vertically while the next response is streaming.
*   Fixed excessive memory use when loading web threads with multiple repositories.
*   Fixed agent replies to file comments appearing disconnected from the comments they answer.
*   Fixed Ballerina files showing no icon in the project panel, changes panel, and command palette.
*   Fixed OpenRouter model availability errors to show the provider’s explanation and offer model selection instead of reporting a generic API endpoint failure.
*   Fixed Gemini and other models rejecting a whole request because Delta rewrote tool descriptions into a restricted schema dialect that the provider then refused. Delta now sends each tool’s complete JSON Schema to every provider, so tools that accept alternatives — such as `grep`, whose `globs` takes either one pattern or a list — are described accurately everywhere.
*   Fixed empty threads accumulating in history by deleting them locally instead of archiving when you close them from the sidebar or with the close-thread shortcut.
*   Delta now stops a turn when the agent repeats the same tool call with identical input many times without making any filesystem change, instead of letting it loop until it exhausts the context window.
*   Opening Delta again now reuses the running application instead of starting a second unusable instance.
*   Skills inside nested category folders are now available in Delta.
*   File viewers, diffs, and syntax-aware tools now load syntax highlighting correctly in Delta Web.
*   Fixed touch scrolling in Delta Web on iPhone and iPad.
*   Opening the log viewer now focuses its existing window instead of opening another copy.
*   Protocol mismatch alerts now offer to restart Delta when an update is ready to install.
*   Markdown files opened in Delta Web now render as formatted documents, matching the native app.
*   Switching a thread between incompatible models no longer causes the next message to fail when the previous model included private reasoning data.
*   Keyboard shortcut labels no longer appear when using Delta on a mobile browser.
*   Text fields no longer crash when clicked as their contents change.
*   Error text in the light theme now uses a darker color for improved contrast.
*   Delta no longer performs continuous disk writes while idle after cleaning up local data.
*   Fixed terminal command labels reserving space for hidden focus controls.
*   Fixed unnamed repositories showing blank labels instead of an available fallback name.
*   Fixed a race that could restore deleted threads or lose recent sidebar changes.
*   Fixed sharing a newly created thread before its latest changes had finished saving.
*   Fixed collaborative draft typing so one participant’s reply no longer captures another participant’s keystrokes.
*   Improved the branch diff to show the same comparison to every thread participant: the merge base against the default branch is now recorded when the worktree’s head moves and shared with all collaborators, so the branch diff works without a local checkout, including on the web.

## [0.4.0 Aug 31, 2026](#0.4.0)

*   Mermaid diagrams now render while the agent is still writing them, instead of waiting for the finished code block.
*   Fixed scroll jumps when clicking or dragging selections near code blocks.

## [0.3.1 Aug 28, 2026](#0.3.1)

*   Fixes for draft submission in shared threads, Delta worktree repositories after compaction, and scroll stability.

## [0.3.0 Aug 27, 2026](#0.3.0)

*   Thread deletion returns to the app: right-click a thread and choose **Delete Locally**. The new name says what it does; synced copies stay on Delta’s servers. See [Data Storage & Deletion](/docs/privacy-and-security/data-storage#delete-individual-threads).
*   GitHub Copilot appears as a subscription option when connecting your first model provider.

## [0.2.0 Aug 27, 2026](#0.2.0)

*   `Cmd+Enter` on macOS and `Ctrl+Enter` on Linux and Windows now send your message by default; `Enter` inserts a new line. Turn [Send With Modifier](/docs/configuration/settings) off to send with `Enter` again.
*   Two new ways to connect models: sign in with an eligible Grok or X subscription, or bring OpenCode Zen and OpenCode Go API keys. See [Models & Providers](/docs/agents/models-and-providers).
*   Set per-provider [model roles](/docs/agents/models-and-providers#model-roles): Small, Balanced, and Frontier models that Delta uses when it picks a model on its own.
*   The **Changes** tab is now the review drawer’s default panel, with **Seen** checkboxes that fold reviewed files and sync across your machines. See [Review & Sync Changes](/docs/agents/review-and-sync).
*   One shortcut for comments everywhere: `Cmd+Option+M` comments on the selection in the transcript, file tabs, and diff views; `Cmd+Shift+M` toggles delivered comments.
*   Each collaborator in a shared thread now composes their own draft, attributed to them.
*   Deleting threads moved to the CLI; the app archives instead. See [Data Storage & Deletion](/docs/privacy-and-security/data-storage#delete-individual-threads).

## [0.1.1 Aug 24, 2026](#0.1.1)

*   File tabs in Review Changes now open in comment mode: typing creates a [comment](/docs/agents/comments) instead of editing the file. A toggle in the file toolbar switches back to direct editing.
*   Search works in the diff view: `Cmd+F` finds matches across every file in the comparison, and `Cmd+G` steps through them. See [Review & Sync Changes](/docs/agents/review-and-sync).
*   Ask Delta about itself: the built-in `/delta-help` [skill](/docs/agents/skills) answers questions about using Delta.
*   The new **Default Shell** [setting](/docs/configuration/settings#terminal) picks the shell for terminals in threads.
*   Archived synced threads free their local disk space after three days; opening one downloads its history again. See [Threads](/docs/agents/threads#archive-and-reopen-threads).
*   Comment on code: the file tabs and diff views in Review Changes now support [comments](/docs/agents/comments) for everyone. Select code and press `Cmd+Shift+M`; delivered comments collapse into gutter indicators.
*   Anthropic models support **Compact Now** alongside automatic compaction. See [Models & Providers](/docs/agents/models-and-providers#context-windows-and-compaction).
*   Press `Cmd+T` to find a thread, and `Cmd+W` to archive the current one.
*   Repositories can ship an executable `.agents/prepare` script that runs in every new managed checkout. See [Delta Worktrees](/docs/concepts/worktrees).
*   New onboarding: first launch signs you in, walks through connecting a model provider, and starts a short tutorial thread.
*   Font settings split: separate families for thread text, interface, and code, plus a code font size adjustment. See [Settings](/docs/configuration/settings).
*   Bring your own OpenRouter API key: OpenRouter joins Anthropic, OpenAI, and Baseten as an API-key [model provider](/docs/agents/models-and-providers).
*   Keep a terminal running after you send: toggle **Background Mode** on an interactive terminal (`Cmd+Alt+Z` on macOS, `Ctrl+Alt+Z` on Linux and Windows) and dev servers or watchers stay up. Manage them from the status bar. See [Terminals](/docs/agents/terminals).
*   Compact a thread manually: click the context-usage indicator for **Compact Now**, or run it from the command palette. **Automatic Compaction** can be toggled per model. See [Models & Providers](/docs/agents/models-and-providers#context-windows-and-compaction).
*   The new **Last Turn** view in Review Changes shows the diff since the agent’s last turn started. It is also available as a default through the `default_diff_base` [setting](/docs/configuration/settings).
*   The agent can push directly into your checked-out branch: a clean checkout updates in place, with no branch switching. See [Review & Sync Changes](/docs/agents/review-and-sync).
*   Join a shared thread by pasting its link or share ID into the new **Join Thread** command. **Share Thread** gains a shortcut: `Cmd+Shift+S` on macOS, `Ctrl+Shift+S` on Linux and Windows.
*   Diffs are easier to read: the new **Diff Colors** setting adds a colorblind-friendly blue/orange scheme, and **Diff Markers** adds `+`/`−` signs to changed lines.
*   Typing inside someone else’s [comment](/docs/agents/comments) starts a reply instead of editing their note.
*   The **Review Changes** tab collects a thread’s diffs in one place, with a selector for the branch diff, uncommitted changes, and opened commits. See [Review & Sync Changes](/docs/agents/review-and-sync).
*   Extended thinking starts enabled in new threads. See [Models & Providers](/docs/agents/models-and-providers).
*   Shared threads resolve the model through each participant’s own providers, so collaborators don’t need matching API keys.
