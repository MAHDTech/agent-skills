![](https://antigravity.google/assets/image/antigravity-cursor.png)

Google Antigravity ChangelogGoogle Antigravity Changelog

[View docs](https://antigravity.google/docs/getting-started)[Follow us on
X](https://x.com/antigravity)

Antigravity 2.0

Antigravity CLI

Antigravity SDK

Antigravity IDE

info

New versions are rolled out gradually and may take a few days to reach
all users.

Version

Description

------------------------------------------------------------------------

[2.8.1](https://antigravity.google/releases?tab=hub&version=2.8.1 "View release 2.8.1")  
August 13, 2026

### Chat Responsiveness Improvements

Bug fixes addressing message responsiveness when interacting with the
agent.

Improvements (0)

Fixes (1)

- Improved responsiveness and reduced hanging issues when sending
  messages to the agent.

Patches (0)

[2.8.0](https://antigravity.google/releases?tab=hub&version=2.8.0 "View release 2.8.0")  
August 12, 2026

### Persistent Sidebar Folders, Crash Fixes, and UI Enhancements

Antigravity improves workspace organization by persisting your sidebar
folder states. This release also ensures reliable file and artifact
previews.

Improvements (1)

- The sidebar now remembers which folders you have collapsed or
  expanded, including when you use the 'Collapse All' action, keeping
  your Projects list organized across sessions.

Fixes (6)

- Fixed an issue that prevented submitting or dismissing the feedback
  form on the welcome screen.
- Fixed frequent application crashes when opening conversations that
  contain large command outputs. The file viewer now limits the size of
  files it will display, and large changes in the history are limited to
  ensure stability.
- Fixed a styling issue where inline code in chat messages and the file
  viewer did not display in the correct colors.
- Fixed a layout bug that caused multi-line commands to display without
  line breaks in the activity history.
- Fixed an issue where the application header and top menu bar failed to
  display on Linux.
- Fixed an issue where empty section headers (such as "Terminal")
  appeared in Settings when all contained settings were disabled.

Patches (0)

[2.7.1](https://antigravity.google/releases?tab=hub&version=2.7.1 "View release 2.7.1")  
August 11, 2026

### Reliability, Performance Gains, and User Experience Polish

Antigravity 2.7.1 introduces visual side-by-side previews for image
diffs alongside comprehensive screen-reader and keyboard accessibility
enhancements. This update also delivers key performance boosts, bug
fixes, and UX refinements across chat interactions and project
navigation.

Improvements (11)

- Improved performance of the conversation sidebar, especially when
  managing a large number of projects and conversations.
- Improved the performance of the conversation sidebar, reducing lag
  when switching between conversations.
- Added the ability to delete projects from the Project Settings page.
  Deleting a project permanently removes it along with all associated
  active and archived conversations.
- Added side-by-side previews for image files (including SVGs) when
  comparing file changes.
- Added a 'Last Prompt' sorting option to the display options menu,
  allowing you to sort your conversation history by the time of your
  last message.
- Improved the slash-command menu with smarter search ranking and
  highlighting, making it easier to find and select commands as you
  type.
- Adjusted the appearance of the thumbs-up and thumbs-down feedback
  buttons on messages so they no longer look disabled.
- The conversation hover card now displays the project name next to a
  folder icon, showing where the conversation is saved.
- You can now use the keyboard to navigate to action buttons and menus
  that appear when hovering over list items (such as pinning or deleting
  conversations).
- Improved sidebar organization with new conversation grouping options.
  You can now group conversations by project or status while keeping
  standalone conversations separate, or view all conversations in a
  single combined list.
- Improved the design and organization of the display options menu in
  the projects sidebar and history pages. Additionally, menu checkboxes
  have been updated to display clearer selection states.

Fixes (20)

- Ensured that local web servers started by the agent are reachable from
  your host machine.
- Fixed a visual issue where submenu items (such as the 'Copy' options)
  were vertically misaligned when opened.
- Fixed an issue where the chat input box lost focus after switching
  conversations using keyboard shortcuts.
- Fixed an issue where scrolling inside the file path dropdown would
  also scroll the main file path bar in the background.
- Fixed an issue where line numbers were incorrectly displayed on file
  pills for images, audio, and video files in the chat.
- Fixed an issue on mobile devices where the keyboard remained open
  after sending a message.
- Fixed an issue where selected checkbox options in menus changed text
  color, ensuring consistent styling across all menu items.
- Fixed an issue where the Review tab would get stuck loading when
  trying to display binary files or files that failed to load, replacing
  the spinner with an informative message.
- Fixed an issue where SVG images without specified dimensions would
  appear blank or collapsed when opened in the media viewer.
- Fixed an issue where renaming a conversation from the header menu did
  not work on mobile web browsers.
- Fixed an issue where SVG files failed to display and showed errors;
  they now render correctly as images in the file viewer.
- Fixed an issue where folders added to the prompt displayed file icons
  instead of folder icons.
- Fixed a bug where searching for text inside a file diff would fail to
  return matches that were scrolled off-screen.
- Fixed an issue where searching for text within markdown previews
  returned no results.
- Fixed a bug in the conversation display options menu where the
  subtitle setting could appear unselected. Conversation subtitles now
  display folder and branch icons next to project and branch names for
  improved visual clarity.
- Fixed the project selector in the input box to ensure you can see and
  select all your projects when starting a new conversation.
- Fixed an issue where search matches were not highlighted when
  searching within a file.
- Fixed an issue where the highlighted option in the @-mention menu
  could scroll out of view when navigating with the arrow keys.
- Fixed an issue where conversation titles in the sidebar would
  temporarily revert to "Untitled Conversation" while updates were
  loading.
- Fixed an issue where project artifacts would fail to load if the
  conversation was not active in the sidebar.

Patches (0)

[2.6.0](https://antigravity.google/releases?tab=hub&version=2.6.0 "View release 2.6.0")  
August 7, 2026

### Faster long conversations, more reliable hooks and subagents

Conversations with long histories now open faster, improved custom hooks
and subagents behavior, and administrator policies for connected tool
servers are applied correctly — plus a broad set of fixes across chat,
comments and the file viewer.

Improvements (9)

- Improved the ask questions panel: clearer option selection, and
  continue now only activates once you've answered the current question.
- Hover details on the small icon chips in chat now appear almost
  immediately instead of after a noticeable pause.
- Images and videos in chat now open in a preview window when clicked,
  with the option to open them in a tab instead.
- Hook configurations that could never run are now rejected at load time
  with a clear error instead of being silently ignored.
- Projects in the command palette now use the same folder icon as the
  sidebar, instead of a mismatched briefcase.
- Opening long conversations is now dramatically faster.
- You can now copy an image to your clipboard from an artifact or the
  file viewer, using the menu or right-click.
- The conversation list now loads far faster for users with a large
  conversation history.
- Settings descriptions now use consistent wording throughout, and two
  typos in the shortcut and host address settings were fixed.

Fixes (21)

- Slash commands now work when a newline or tab separates the command
  from its arguments.
- Conversations started by background automations no longer disappear
  from the sidebar list; they now appear under the scheduled-tasks
  filter.
- Steps waiting on your input now show the icon for the tool involved,
  including in the permission prompt, instead of a generic bell.
- Fixed the comment popup following your cursor while dragging, and
  whole-file comments landing on the first line with no highlight.
- Agents no longer hang forever when a stop hook keeps blocking the end
  of a turn; the turn now finishes after repeated blocks.
- Conversations that had already finished no longer stay stuck showing
  as running after you quit and reopen the app.
- Fixed hovering a message revealing the Stop button on every running
  subagent card; now only the card you hover shows it.
- The Review Changes button's keyboard focus outline now wraps the
  entire button instead of only part of it.
- Hooks that call a model now stop at their configured timeout with a
  clear error, instead of stalling the agent indefinitely.
- Pasting plain multi-line text into the chat box no longer adds an
  extra blank line between each line.
- Fixed pinning and unpinning a conversation often failing on the first
  click and snapping back, forcing you to click repeatedly.
- Commands still running no longer display a nonsensical negative
  elapsed time when their start time is missing.
- Fixed administrator policies for connected tool servers being skipped
  at startup, which could let restricted servers start without their
  required settings.
- Fixed agents losing access to their built-in helpers, so research,
  browser and teamwork assistants work again.
- Fixed the wrong keyboard shortcut being displayed for toggling the
  side panel.
- Fixed the built-in browser inspection tool being wrongly blocked when
  your organization restricts which tool servers can run.
- Stopping a subagent now also stops everything it spawned, including
  nested subagents and their background tasks, instead of leaving them
  running.
- Fixed long answers and previews periodically jumping back to the top
  and losing your scroll position while you read.
- Custom hooks you define now run at the end of a turn instead of being
  skipped before they could fire.
- Fixed a crash that happened when you selected a region of an image and
  submitted a comment on it.
- Fixed a stray outline around the pending-step badge in the
  conversation list under some themes.

Patches (0)

[2.5.0](https://antigravity.google/releases?tab=hub&version=2.5.0 "View release 2.5.0")  
July 31, 2026

### Enterprise sign-in: Gemini Enterprise accounts and Workforce Identity Federation

Added enterprise sign-in support for Gemini Enterprise user accounts
(with admin controls) and Workforce Identity Federation via Advanced
SSO.

Improvements (20)

- Added support for signing in with your Gemini Enterprise user account,
  including support for organization admin controls.
- Allocated and auto-assigned license seats against GE-Standard &
  GE-Plus organization subscription tiers.
- Regionalized (US, EU, Global) model inferencing.
- Operating under Google Cloud terms of service.
- Added Workforce Identity Federation sign-in for enterprise users:
  select the Advanced SSO login option.
- Added per-model reasoning effort levels (Low, Medium, High).
- Added enterprise admin policies for browser features, MCP allowlists,
  and permission thresholds.
- Approved permissions are now remembered for the rest of the
  conversation.
- Sidecar links now render as pills that open the sidecar in the
  auxiliary pane.
- Added "Reveal in Finder/Explorer/File Manager" to the file viewer.
- Added "Collapse All Folders" and "Expand All Folders" to the command
  palette.
- Added search and keyboard navigation to the project selector.
- Added syntax highlighting for Objective-C, Swift, and Dart.
- External markdown links now reveal the destination URL on hover.
- The environment selector now remembers your last-used worktree.
- Increased voice recording limits.
- Improved @-mention file search speed and result stability.
- Reduced typing lag while the agent streams responses.
- Pinned conversations now show their workspace or project name.
- Improved mobile UI with native pickers and no keyboard popup on load.

Fixes (13)

- Fixed crashes in the file and artifact viewers, including SVG
  previews.
- Fixed a deadlock that could hang the agent while waiting for
  subagents.
- Fixed a hang when starting with an empty workspace.
- Improved performance when switching between conversations.
- MCP connections now refresh OAuth tokens instead of wiping
  credentials.
- Updated auxiliary pane tab shortcuts to avoid text navigation
  conflicts.
- Fixed OAuth issues with Salesforce and Atlassian MCP servers.
- Fixed deleting, toggling, and opening custom agents.
- Fixed queued messages sending automatically after reverting a step.
- Fixed diffs and Review Changes opening unexpectedly during background
  syncs.
- Fixed text selection drift and incorrect line numbers.
- Fixed Markdown tables and code blocks wrapping incorrectly.
- Fixed repeated notification permission prompts.

Patches (0)

[2.4.3](https://antigravity.google/releases?tab=hub&version=2.4.3 "View release 2.4.3")  
July 28, 2026

### Preview tabs, MCP timeouts, file attachments, and performance fixes

Added preview tabs, attachment support for .json, .md, and .csv files,
MCP timeouts, keyboard shortcuts, and various bug fixes and
improvements.

Improvements (17)

- Added a keyboard shortcut (Cmd+L / Ctrl+L) to quickly quote selected
  text into the chat input.
- Added support for attaching .json and .md files to the input box with
  distinct visual badges.
- Added preview (temporary) tabs to the auxiliary pane, allowing files
  to open in a temporary slot that is reused until promoted to a
  permanent tab (e.g., via double-click).
- Added a timeout for MCP server connections and tool calls to prevent
  the agent from hanging indefinitely.
- Added an 'Only Unread' filter option to the Conversation History
  dropdown.
- Added support for uploading and attaching CSV (.csv) files in chat as
  plain text.
- Added a 'Download Diagnostics' command to the command palette to
  download a diagnostics package (containing logs and agent state) for
  troubleshooting.
- Added tooltips to file links in markdown views, showing their full
  path on hover.
- Refined command approval rules to reduce redundant permission prompts
  for everyday tools.
- Improved the speed of conversation switching by optimizing trajectory
  summary list selectors.
- Improved authentication error messages to show the specific failure
  reason.
- Draft comments are now automatically saved and preserved across
  sessions.
- Improved interface responsiveness by preventing unnecessary re-renders
  when toggling sidebars or auxiliary panes.
- Improved accessibility in the Settings modal with better heading
  hierarchy and keyboard focus management.
- Updated the default settings screen name from 'Agent' to 'General' and
  updated copy for inherited settings ('Use Global' to 'Inherit
  General').
- Restored the default 5-second auto-dismiss behavior for in-app
  notifications.
- Optimized scroll performance in conversation lists and panels,
  especially during long chats.

Fixes (10)

- Fixed an issue where configuring duplicate tool names in
  customizations (such as MCP servers) caused the agent to fail to
  initialize.
- Fixed a potential crash in the image generation tool.
- Fixed a bug where terminal commands with quoted arguments (e.g.,
  arguments containing pipes or special characters) were incorrectly
  parsed, which could cause permission checks to fail or behave
  unexpectedly.
- Fixed an issue where queued agent messages could be delivered and
  displayed out of order on Windows clients.
- Fixed an issue where live-streamed subagent steps would display as
  generic text instead of rich cards.
- Fixed an issue where inline markdown diff blocks could occasionally
  render as empty boxes.
- Fixed a bug where the Settings dialog would close unexpectedly when
  clicking outside an open dropdown.
- Fixed an issue where the UI would show an infinite loading spinner
  instead of falling back to agent edits when version control features
  were unavailable.
- Added view options (grouping, sorting, filtering) and fixed search
  filtering on the Conversation History page.
- Fixed an issue where the app doesn't request microphone permission
  when enabling "Record Audio" for the first time.

Patches (0)

[2.3.1](https://antigravity.google/releases?tab=hub&version=2.3.1 "View release 2.3.1")  
July 16, 2026

### Startup stability fix

Fixes a bug where an empty or malformed configuration file caused AGY to
fail to load on startup.

Improvements (0)

Fixes (0)

Patches (1)

- Fixed a bug where an empty or malformed `config.json` configuration
  file in the `~/.gemini/config/` directory prevented AGY from loading
  on startup.

[2.3.0](https://antigravity.google/releases?tab=hub&version=2.3.0 "View release 2.3.0")  
July 13, 2026

### Queued messages, plain text files support, and stability improvements

Added support for queued messages, settings to configure message
execution, a send now option, plain text file attachments, and various
bug fixes and improvements.

Improvements (6)

- Added support for queued messages, including settings to configure
  execution behavior, a "Send Now" option, and an improved input card
  layout.
- Added support for attaching and rendering plain text (.txt) files in
  conversations and optimized comment button alignment in diff views.
- Streamlined startup performance by optimizing file watching and the
  application initialization sequence.
- Refined command permission prompting with strict literal matching,
  relaxed redirection rules, and sandbox configuration improvements.
- Enhanced reliability with automatic retries on backend overload errors
  and resolved hangs in helper agents requesting feedback.
- Updated the default UI theme to respect system preferences instead of
  defaulting to dark mode.

Fixes (5)

- Fixed find-in-page (Cmd+F) search to correctly locate and highlight
  matches in virtualized file views.
- Improved side questions (/btw) with persistence across conversation
  switches, display of the agent's thinking process, and a fix for
  background retries when closed.
- Stopped background tasks from continuing to run after a conversation
  is archived.
- Fixed an issue where diffs for files with more than 1000 lines
  rendered incorrectly.
- Fixed an issue where the agent could hang indefinitely after subagents
  finished executing.

Patches (0)

[2.2.1](https://antigravity.google/releases?tab=hub&version=2.2.1 "View release 2.2.1")  
June 25, 2026

### Antigravity Guide, audio support, search improvements, and performance fixes

New built-in Antigravity Guide skill, audio file rendering, improved
substring file search, performance optimizations, and critical bug
fixes.

Improvements (19)

- Added a new built-in Antigravity Guide skill to provide helpful
  guidance when users ask about Antigravity.
- Added syntax highlighting for C++, Python, and Protobuf files in
  markdown code and diff blocks.
- Added hover tooltips to file pills in the chat and workspace views to
  display absolute paths and clarify workspace locations.
- Improved readability of context pills by truncating long labels and
  adding hover tooltips to view the full content.
- Enabled automatic saving of refreshed OAuth tokens to the OS keyring
  to reduce authentication prompts.
- Added support for rendering and playing audio files (.mp3, .wav, .ogg,
  .m4a) within the sidebar file viewer and artifact viewer.
- Added a new 'Conversation Width' setting under Appearance settings,
  allowing you to configure the maximum width of the conversation panel
  (Default, Narrow, or Wide).
- When creating a new project, Antigravity 2.0 will now notify you if a
  project with the same folders already exists, allowing you to quickly
  navigate to it.
- Added a tooltip to show the full task summary when hovering over items
  in the Running Items panel.
- Improved the model selector dropdown to dynamically resize and prevent
  truncation of long model names.
- Updated the permissions request dialog to display a clear description
  of the action or command being run.
- Improved response time and reliability of account status checks when
  Terms of Service violations are encountered.
- Improved UI responsiveness and eliminated lag when resizing the
  sidebar layout.
- Aligned theme colors, font sizes, text size, and line height in the
  chat input area to match surrounding UI density and ensure consistent
  rendering.
- Improved usability for touch and stylus users by increasing the size
  of various UI hit targets, including buttons, pills, and toolbars.
- Adjusted the diff viewer layout to provide more horizontal space for
  code contents.
- Added a brief delay before displaying tooltips for long conversation
  titles in the sidebar.
- Optimized file watching notifications to prevent visual flickering and
  improve performance during high-frequency file writes.
- Updated icons for built-in slash commands and redesigned
  mentions/slash commands menu alignment in the chat input box.

Fixes (17)

- Fixed an issue with broken text selection anchoring when adding
  comments to files or artifacts.
- Fixed an issue where workspace context was lost when navigating to the
  history page or other views.
- Fixed performance issues, rendering loops, and lag when navigating
  folders using the file path breadcrumbs or the file tree.
- Fixed erratic behavior and lag in slash command autocomplete during
  network instability.
- Improved file search in your workspace by matching substrings instead
  of requiring exact prefix matches.
- Fixed an issue where file search within workspaces could fail with a
  'No such file or directory' error.
- Fixed an issue where allowing commands with special characters (like
  dots or environment variables) could trigger an infinite permission
  prompt loop.
- Improved command execution permission matching to support
  prefix-matching for build/test commands and correct handling of quoted
  arguments.
- Fixed issues where in-progress prompts and local history edits were
  discarded during navigation or permission checks.
- Fixed an issue where custom theme settings and color modes were not
  correctly applied on startup, including rejecting hex codes with a
  leading '#' symbol.
- Fixed startup issues and recurring User Account Control (UAC) prompts
  on Windows by registering a scheduled task and resolving access
  control errors on system PATH directories.
- Fixed crashes and synchronization issues when deleting persistent
  state across tabs.
- Improved accessibility and usability of the artifact review dialog,
  including contrast, tooltips, and error messaging.
- Fixed an issue where the agent was blocked from reading builtin
  customizations and skills due to sandbox restrictions.
- Fixed a startup race condition that could cause duplicate project
  folders to be created.
- Fixed a deadlock issue that could cause subagents to hang during
  execution.
- Fixed a potential crash when calculating token usage.

Patches (0)

[2.1.4](https://antigravity.google/releases?tab=hub&version=2.1.4 "View release 2.1.4")  
June 11, 2026

### Quota Screen Redesign and PDF Support

Quota screen redesign, PDF attachment support, new /btw slash command,
and other bug fixes

Improvements (7)

- Quota screen redesign: clearer, unambiguous view into “used” versus
  “remaining” credits in the Models tab of the Settings screen.
- Ask side questions via /btw: while in a conversation, type ‘/btw’ in
  the input and select the ‘btw’ option in the menu to write a message
  to an ephemeral, single-response agent that has the context of your
  current conversation.
- Simple conversation search functionality: cmd/ctrl+F to search for
  visible text in the conversation view.
- Support for PDF attachments to messages to Gemini models: drag and
  drop PDFs from your filesystem or add PDFs using the media option in
  the Add Context menu button in the input box.
- Breadcrumbs in file viewer pane header: quality-of-life UI to more
  easily see and navigate directories in your repo.
- Nested subagents in the overview pane: see all nested subagents
  belonging to the main conversation instead of only the subagents that
  are one level deep.
- Minor improvements to Projects UX: you can now specify a project name
  during the creation flow and sort conversations in the left sidebar by
  worktree.

Fixes (4)

- Improved LaTeX support: the agent has a better understanding and
  awareness of its ability to output LaTeX-rendered math text.
- Improved MCP server stability: increased resilience to unresponsive
  MCP servers and improved browser agent self-troubleshooting abilities
  when using the Chrome DevTools MCP server.
- MCP server schema compatibility: the mcp_config.json schema now
  accepts url in addition to serverUrl as a field.
- New entries in the sensitive paths list: .vscode and .cache are now
  recognized as sensitive paths that require explicit user confirmation
  before the agent can access them.

Patches (0)

[2.0.11](https://antigravity.google/releases?tab=hub&version=2.0.11 "View release 2.0.11")  
June 3, 2026

### Antivirus and Open IDE fixes

Startup and Open IDE button fixes

Improvements (0)

Fixes (2)

- Fixed an issue that occurs when certain antivirus products are
  installed that caused a dark blank screen on app startup.
- Fixed some bugs related to the Open IDE button.

Patches (0)

[2.0.10](https://antigravity.google/releases?tab=hub&version=2.0.10 "View release 2.0.10")  
May 28, 2026

### AGY 2.0 Bug fixes

AGY 2.0 Bug fixes

Improvements (1)

- Various reliability and usability improvements

Fixes (1)

- G1 credit bug fix

Patches (0)

[2.0.6](https://antigravity.google/releases?tab=hub&version=2.0.6 "View release 2.0.6")  
May 22, 2026

### Antigravity IDE integration

Added integration with Antigravity IDE.

Improvements (2)

- Add an install IDE button if Antigravity IDE is not installed.
- Add an open IDE button if Antigravity IDE is installed which allows
  you to open the current project in Antigravity IDE.

Fixes (0)

Patches (0)

[2.0.1](https://antigravity.google/releases?tab=hub&version=2.0.1 "View release 2.0.1")  
May 19, 2026

### AGY 2.0 Bug Fixes

Antigravity 2.0 Launch bug fixes.

Improvements (0)

Fixes (3)

- Fixed project migration issues for projects with CJK characters in
  their titles.
- Fixed an issue that caused duplicate projects to be created when
  importing from Antigravity 1.0.
- Resolved an issue where Google One credits were not being applied or
  utilized.

Patches (0)

[1.1.14](https://antigravity.google/download#antigravity-cli "View release 1.1.14")  
August 18, 2026

### Enterprise sign-in fast path, MCP OAuth metadata documents, /context scrolling, and markdown agent customizations

Fast return path for enterprise sign-in, support for OAuth client ID
metadata documents in MCP servers, terminal-sized scrollable `/context`
and `/usage` panels, unified `inheritCustomizations` configuration for
markdown agents, read-only permissions outside the workspace, and fixes
for language server crashes, hyperlink rendering, artifact approval
tracking, MCP error isolation, and bracketed paste.

Improvements (5)

- Added a faster return path for enterprise sign-in, offering returning
  users a single `Continue with ...` option naming the authentication
  method they last used.
- Added support for OAuth client ID metadata documents when connecting
  to an MCP server, allowing servers implementing this specification to
  connect without requiring a manually supplied client ID or dynamic
  client registration.
- Improved the `/context` panel, making it scrollable and dynamically
  sized to the terminal, with `/usage` sized the same way.
- Improved markdown-defined agents with a single `inheritCustomizations`
  switch that controls whether an agent adopts skills, rules, plugins,
  subagents, and MCP servers, replacing inconsistent per-kind defaults.
- Improved workspace boundary settings: accessing paths outside the
  workspace now grants read access only, with writes requiring
  authorization based on the active execution mode.

Fixes (9)

- Fixed the CLI exiting silently when its language server failed at
  startup or during execution; failures are now logged to the terminal
  and exit with a non-zero status.
- Fixed sign-in URLs and hyperlinks rendering as garbled escape
  sequences on terminals that do not support OSC 8 hyperlinks (such as
  GNU `screen`), detecting unsupported terminals and rendering plain
  text instead.
- Fixed `Enter` keystrokes being swallowed while the path-suggestion
  popup was loading, which previously sent the prompt and popped results
  open above it.
- Fixed the artifact list marking previously unreviewed, commented, or
  rejected artifacts as approved when reopening the list.
- Fixed malformed MCP server configurations causing all other servers to
  fail; invalid entries are now logged and skipped while remaining
  servers continue loading.
- Fixed built-in skills, rules, and plugins being stripped when an agent
  configured `inherit_user: false`, ensuring built-in capabilities
  remain available while personal customizations are excluded.
- Fixed conversation branching and rollbacks failing with
  `too many SQL variables` on long trajectories.
- Fixed conversation titles unexpectedly jumping to older messages after
  trajectory truncation by protecting the title checkpoint.
- Fixed multi-line pastes submitting individual prompts per line on
  terminals that do not support bracketed paste.

Patches (0)

[1.1.13](https://antigravity.google/download#antigravity-cli "View release 1.1.13")  
August 14, 2026

### GEMINI_API_KEY authentication, custom agent task management, embedded ripgrep caching, and conversation database fixes

Direct `GEMINI_API_KEY` authentication support, `/codesearch` fallback
resilience, cached artifact list scrolling, `manage_task` access for
custom declarative agents, embedded ripgrep caching with SHA-256
verification, and critical fixes for trajectory truncation, database
growth, transcript race conditions, and subagent path validation.

Improvements (10)

- Added support for `GEMINI_API_KEY`, allowing the CLI to run directly
  against the Gemini API without signing in. Set
  `modelProvider: "gemini"` in `settings.json`, export `GEMINI_API_KEY`,
  and optionally configure `GOOGLE_GEMINI_BASE_URL` for custom
  endpoints. The banner and `/help` show `Gemini API key` as the
  credential, and `/logout` explains that it originates from the
  environment.
- Improved `/codesearch` resilience by falling back to a local search
  when the `ripgrep` binary cannot be executed, such as when blocked by
  endpoint security software.
- Improved artifact list scrolling performance by caching each file's
  preview line count instead of re-reading every listed file from disk
  on every keypress.
- Improved custom agent support by making `manage_task` available to
  declarative agents that have not opted out of fundamental components,
  enabling custom agents to list and kill background tasks, with clearer
  step descriptions like `Killed task X` and `Checked task X`.
- Improved `search_web` step visualization by displaying the query while
  the search is in progress rather than only after completion.
- Improved agent behavior after backgrounding tasks by removing
  anti-polling reminder text from `manage_task` and `manage_inbox` tool
  responses to prevent unintended polling loops.
- Improved the `/resume` picker by clearly relabeling its second tab.
- Improved deleting conversations in the `/resume` picker by remapping
  the shortcut from `ctrl+delete` to `f4` for reliable cross-platform
  terminal compatibility on macOS.
- Improved the load time when opening large conversations by fetching
  step headers in batches without reading each step's full payload.
- Improved embedded `ripgrep` reliability and security by extracting
  binaries into the user cache directory instead of `/tmp`, adding
  SHA-256 integrity verification, and using atomic renaming to prevent
  concurrent execution races.

Fixes (11)

- Fixed distorted startup banner rendering in Cloud Shell by
  automatically enabling a conservative rendering mode for web
  terminals.
- Fixed direct Gemini API mode getting stuck on the sign-in screen
  instead of launching into the main UI, and resolved failures with
  custom `GOOGLE_GEMINI_BASE_URL` endpoints by omitting unsupported
  session fields.
- Fixed conversations disappearing from the `/resume` picker after
  switching tabs, and preserved the `tab` hint in the shortcut bar even
  when a tab is empty.
- Fixed trajectory truncation over-pruning conversation history by
  charging the size budget solely against reclaimable steps rather than
  protected checkpoints or already-cleared residues.
- Fixed unbounded growth of the SQLite conversation database during
  background task and subagent wakeups caused by duplicate grants and
  settings being appended to persisted rows.
- Fixed transcript corruption caused by background messages appending
  while context compaction was rewriting the log.
- Fixed an issue where a hung sandbox initialization prevented
  subsequent messages from being processed.
- Fixed path traversal vulnerability in `define_subagent` by strictly
  validating model-supplied agent names before creating directories.
- Fixed artifact list rendering bugs involving wide CJK characters and
  emoji, boundary handling when entries disappear, and viewport
  scrolling overflow.
- Fixed `Esc` key in the artifact viewer triggering duplicate review
  submission confirmations for previously completed reviews.
- Fixed missing tool call line rendering in collapsed/expanded tool
  groups for `invoke_subagent`, `send_message`, `manage_subagents`,
  `define_subagent`, and `generate_image`.

Patches (0)

[1.1.12](https://antigravity.google/download#antigravity-cli "View release 1.1.12")  
August 11, 2026

### Artifact heading outline, print-mode commands, artifact alerts/carousels, improved terminal hyperlinks, and extensive CLI fixes

Artifact outline viewer navigation, print-mode read-only slash commands,
machine-readable output for `models` and `agents`, alert callout and
carousel rendering in markdown artifacts, terminal hyperlink
improvements, and fixes for headless `--mode`, startup diagnostics,
subagent messaging, and quota reporting.

Improvements (13)

- Added a heading outline to the artifact viewer, opened with `t`, so a
  long markdown document's structure is visible at a glance and you can
  jump straight to a section instead of scrolling.
- Added non-interactive answers for more read-only slash commands in
  print mode, so `-p "/permissions"`, `/hooks`, `/help`, `/changelog`
  and `/config` each emit one tab-separated record per line — or a
  structured payload under `--output-format json` and `stream-json` —
  without starting an agent turn, spending quota or leaving a
  conversation behind, with `/help` listing exactly the commands print
  mode answers and `/changelog` printing the release notes.
- Added machine-readable output to the `models` and `agents` subcommands
  through an `--output-format` flag accepting `json` and `stream-json`,
  and moved their error messages and progress spinner off stdout so
  captured output contains only the list.
- Added a `disable-slash-command: true` flag for a skill's `SKILL.md`
  frontmatter, which hides that skill from the `/` menu and from `/name`
  resolution while leaving it discoverable and invocable by the model,
  so a large skill library no longer floods the command menu.
- Added rendering for alert callouts (`[!NOTE]`, `[!TIP]`,
  `[!IMPORTANT]`, `[!WARNING]`, `[!CAUTION]`) and for carousel blocks in
  markdown artifacts, which previously showed as raw markup.
- Improved terminal hyperlink support, enabling clickable links over SSH
  and in hyperlink-capable `tmux`, keeping word-wrapped URLs clickable
  past their first line, linking URLs that the markdown renderer had
  left as plain text including ones inside code blocks, no longer
  highlighting unrelated links together on hover, and fixing horizontal
  scrolling and text selection on lines that contain a link.
- Improved tool call headers so every native and MCP tool call carries a
  short summary of what it is doing instead of a bare name or a raw
  argument blob, with the same summary naming background tasks and
  subagents in the activity list and the matching action phrase
  describing the request in permission prompts.
- Improved headroom for large toolsets by raising the per-session limit
  on tool declarations, so heavy MCP, plugin and skill setups stop being
  rejected for having too many tools.
- Improved sign-in with Application Default Credentials by resolving the
  quota project automatically, so service-account logins work without
  extra configuration.
- Improved repository detection so a Git repository nested inside a
  larger multi-repository checkout resolves to the intended root, with
  submodules and worktrees covered.
- Improved headless `-p` runs so the agent settles a choice itself where
  it would otherwise ask, instead of stalling on a question nobody is
  there to answer.
- Improved the `schedule` tool by dropping the redundant
  `Timer Cancelled` notification when a one-shot timer with an
  early-termination condition is cancelled, and by naming both the
  condition and the exact sender whose message satisfied it in the step
  result.
- Improved file paths in tool step lines by collapsing your home
  directory to `~` instead of truncating the front of the path, so the
  interesting end of a long path stays on screen.

Fixes (13)

- Fixed `--mode` being ignored in headless `-p` runs, where a valid
  value such as `accept-edits` or `plan` was never applied and an
  unrecognized value produced no warning at all.
- Fixed startup diagnostics being swallowed into the log file instead of
  reaching the terminal, including crash notices, the `--conversation`
  not-found warning, the `--print` and `--prompt-interactive` conflict,
  conversation load errors, and the `--mode` and `--agent` warnings.
- Fixed a character disappearing from your submitted prompt where it
  wrapped at the terminal's right edge, caused by the echoed prompt
  being indented after it had already been wrapped to the full width.
- Fixed file citations in a response losing their line numbers, so a
  cited range renders as `path:10-25` again instead of just the file
  name.
- Fixed a write-in answer in a multi-select question discarding the
  boxes you had already ticked, and fixed a stale write-in being
  submitted after you went back to a predefined option.
- Fixed large background tasks crowding out the prompt by limiting each
  active item below the input box to a single line.
- Fixed corruption of `config.json` by writing user config atomically,
  so a crash or a concurrent writer can no longer leave a truncated file
  that silently breaks settings persistence.
- Fixed the CLI giving up on a slow OS keyring after one second and
  falling back to empty storage, which forced a re-login; it now waits
  five seconds, as every other keyring operation already did.
- Fixed conversation preview titles derived from the first user message
  by cutting them at the first line and capping them at 500 characters,
  so a large pasted or scripted prompt no longer produces a
  multi-megabyte stored title.
- Fixed a crash on Windows when resolving the conversation transcript
  path, by honoring the path's drive letter in the trajectory log
  artifact converter.
- Fixed a subagent going silent on its parent when its own message
  failed to send, and fixed the parent being handed an empty notice when
  a subagent went idle without a final response.
- Fixed the `read_url_content` tool leaking a connection on every call,
  which could eventually exhaust the machine's available ports.
- Fixed the status line reporting model quota that was always one fetch
  out of date, so it now matches the quota you actually have left.

Patches (0)

[1.1.11](https://antigravity.google/download#antigravity-cli "View release 1.1.11")  
August 7, 2026

### Vim modal editing mode, Vim-aware submission, print-mode slash-command enhancements, and various CLI and permission fixes

Vim modal editing mode with Normal, Insert, Visual, and Visual Line
modes, Vim-aware prompt submission, Vim editing in comment/diff views,
print-mode slash-command and non-interactive enhancements, plugin
configuration improvements, and fixes for command auto-approval, MCP
servers, and autocomplete history.

Improvements (9)

- Added a Vim editing mode, off by default and switched on from
  `/settings` under `Editor Mode`, bringing modal editing to the prompt
  with Normal, Insert, Visual and Visual Line modes, a mode badge in the
  status line and a Vim tab in `/help`, and covering the
  `h`/`j`/`k`/`l`, `w`/`b`/`e` and `W`/`B`/`E` motions, the
  `0`/`^`/`$`/`gg`/`G` boundaries, the `f`/`F`/`t`/`T` character
  searches with their `;` and `,` repeats, the `i`/`a`/`I`/`A` insert
  entries, the `d`/`c`/`y` operators alongside `x`, `D` and `C` filling
  the unnamed register for `p`, and the `iw`/`aw`/`iW`/`aW`/`ip`/`ap`
  text objects.
- Added Vim-aware submission so a prompt can be sent without leaving
  modal editing, through an `Editor Mode > Insert First` toggle that
  opens the prompt in Insert mode where a bare `Enter` submits and a
  modified key such as `shift+enter` or `ctrl+j` inserts a newline,
  `ctrl+s` and `ctrl+enter` keys that submit from Normal mode, an
  `Enter` binding available as `vim.insert.submit`, and the full set of
  `vim.*` scopes in `keybindings.json` for remapping any of it.
- Added Vim editing to the comment editors in the diff and
  artifact-detail views, whose footer hints follow the active mode
  rather than showing one fixed set of bindings, and made a collapsed
  block behave as a single unit under Vim motions.
- Added non-interactive answers for the read-only slash commands in
  print mode, so `-p "/usage"`, `/quota`, `/credits`, `/model`,
  `/effort` and `/skills` emit one tab-separated record per line — or a
  structured payload under `--output-format json` and `stream-json` —
  without starting an agent turn, spending quota, or leaving a
  conversation behind.
- Added an explicit refusal for the remaining interactive-only slash
  commands in print mode, which previously fell through as literal
  prompt text and let the model answer as though the command had run, so
  `-p "/clear"` reported the context cleared while nothing was cleared;
  each now fails with the flag or subcommand that replaces it.
- Improved plugin enable and disable so `config.json` is the only place
  enablement lives, seeded once from each plugin's manifest, which stops
  a plugin that later ships `"disabled": true` from switching itself off
  under someone who was already running it and stops a shipped-default
  change from moving every user on the next release.
- Improved the artifact detail view by wrapping long lines in code files
  instead of clipping them at the right edge.
- Improved model retries by honoring the server-supplied retry delay
  instead of the client's own backoff, so a retry after a rate limit or
  an overload waits exactly as long as the server asks.
- Improved model-loading errors so a permission failure such as a
  missing license or a missing IAM role is shown as itself, with a
  troubleshooting link, instead of a generic failure during the run.

Fixes (9)

- Fixed an allowlist entry that tokenizes to zero command words —
  `command(time)`, a comment-only entry, or an empty compound such as
  `()` — matching every command and silently auto-approving anything the
  agent ran; such an entry now matches nothing.
- Fixed commands being auto-approved while the session was in
  request-review or strict permission mode.
- Fixed admin controls being skipped for MCP servers at startup, where a
  fetch made before authentication cached "admin controls not
  applicable" and allowed every server for the next five minutes, and
  fixed the built-in Chrome DevTools MCP server being blocked outright
  by admin controls.
- Fixed MCP progress callbacks being dropped and the task log file not
  being initialized, so long-running MCP tool calls report progress
  again.
- Fixed a slash command receiving the collapsed `[Pasted text #N]`
  placeholder instead of the pasted content when its argument was pasted
  into the prompt.
- Fixed the prompt saving the typed prefix rather than the completed
  command name to history when an autocompleted slash command was
  submitted, so up-arrow recall replays what actually ran, and made an
  alias require an exact match before auto-executing so a partial alias
  fills the prompt instead of running a command you did not finish
  typing.
- Fixed an asynchronous settings refresh re-enabling the feedback survey
  partway through a print-mode run that had switched it off.
- Fixed spurious "Out of credits" errors, where an empty credits
  response was read as a balance of zero.
- Fixed the "Use AI Credits" setting being offered to accounts signed in
  through a Google Cloud project or application default credentials,
  where it does not apply.

Patches (0)

[1.1.10](https://antigravity.google/download#antigravity-cli "View release 1.1.10")  
August 3, 2026

### Gemini Enterprise & WIF authentication, read-only .git sandbox rules, hook ordering improvements, and subagent and CLI fixes

Gemini Enterprise Business sign-in, Workforce Identity Federation (WIF)
and Application Default Credentials (ADC) auth support, read-only `.git`
sandbox access, hook ordering improvements, and fixes for subagent tree
termination, `--model`/`--effort` flag resolution, and multi-select
spacebar toggling.

Improvements (7)

- Added Business sign-in for Gemini Enterprise accounts, so you can
  authenticate with a Google Cloud project under Google Cloud terms, use
  a license seat allocated or auto-assigned from your organization's
  GE-Standard or GE-Plus subscription, run inference in a chosen region,
  and have your organization's administrator controls applied to various
  features.
- Added Workforce Identity Federation sign-in for enterprise users,
  available as the `Use advanced SSO config` option on the Google Cloud
  sign-in screen, so organizations that federate identity through an
  external provider can authenticate with their own identity provider.
- Added sign-in with Application Default Credentials so you can use
  Agent Platform.
- Added a non-blocking advisory banner when the same conversation is
  already open in another CLI instance on the same machine, pointing at
  `/fork` so two sessions no longer interleave writes into one
  trajectory.
- Improved the terminal sandbox by granting read-only rather than
  writable access to a Git repository's `.git` directory, so the agent
  can inspect repository metadata without being able to rewrite it from
  inside the sandbox.
- Improved hook ordering so hooks defined in `hooks.json` run before the
  built-in termination checks, which lets `PostInvocation` hooks observe
  the final invocation of a turn and lets `Stop` hooks run at all
  instead of sitting unreachable behind the built-ins.
- Improved the `schedule` tool to accept `DurationSeconds` and
  `MaxIterations` when a model emits them as bare JSON numbers rather
  than strings, accepting integral values and rejecting non-integral
  ones with a clear error instead of failing the call.

Fixes (11)

- Fixed `--model` and `--effort` being ignored in interactive sessions
  and in headless `-p` runs, where the flags were applied after model
  configuration had already been initialized so the run silently fell
  back to the persisted or default model.
- Fixed a bare `--effort` resolving against the default model instead of
  the model you actually have selected, which could silently move you to
  a different model.
- Fixed stopping a subagent tree stopping only the conversation it was
  invoked from, while every descendant subagent and the background tasks
  they owned kept running and the CLI still reported them as killed.
- Fixed a forced-continuation deadlock where a coordinator waiting on
  active subagents or background tasks would loop injecting empty
  continue steps until it hit the invocation limit, wasting tokens and
  blocking progress.
- Fixed the spacebar not toggling an option in multi-select prompts,
  including the `ask_question` dialog and the onboarding import
  checkbox, which left `x` as the only working toggle; the hint bar now
  advertises it.
- Fixed the Left and Right arrow keys being captured to navigate the
  input box suggestion dropdown, so you can move the cursor and edit
  text again while suggestions are showing.
- Fixed the model picker's "No models available" state rendering without
  its header and footer, so it now shows the standard chrome and an
  `esc` hint to go back.
- Fixed tools that an MCP server marks to always run in the background
  executing as blocking calls that stalled the turn.
- Fixed an MCP process leak when a server connection dropped
  unexpectedly.
- Fixed the artifact viewer corrupting plain documents by horizontally
  clipping every document rather than only the diagram artifacts that
  need it.
- Fixed the sandbox not recording blocked network requests when the
  command itself exited successfully, which hid the fact that a request
  had been denied.

Patches (0)

[1.1.9](https://antigravity.google/download#antigravity-cli "View release 1.1.9")  
July 31, 2026

### Slash-command expansion in print mode, non-blocking MCP startup, session-wide permission memory, system temp directory write grants, and hook and UI stability fixes

Added slash-command and skill expansion in print mode, non-blocking
background MCP loading during interactive startup, session-scoped
permission pattern recording, system temporary directory write grants,
and stability fixes for stop/PostToolUse hooks, artifact viewer, and MCP
authentication.

Improvements (4)

- Added slash-command and skill expansion to print mode, so a headless
  run such as `-p "/my-skill review this diff"` now resolves and applies
  the skill instead of sending it as literal text, with
  `--disable-slash-commands` to opt out.
- Improved interactive startup so a slow or hanging MCP server no longer
  stalls the first agent turn, loading MCP servers in the background for
  the interactive session while headless and one-shot runs keep blocking
  so their single scripted turn still sees the full toolset.
- Improved permission grants so a pattern approved at a prompt is
  recorded for the rest of the conversation, letting later commands that
  match it run without prompting again.
- Improved the default system temporary-directory grant to cover writes
  as well as reads, so agents no longer trigger a permission prompt when
  creating or updating files there.

Fixes (7)

- Fixed stop hooks that always block hanging the agent forever; after a
  configurable number of consecutive continuations, the hook can no
  longer block and the turn ends normally.
- Fixed `PostToolUse` hooks firing on non-tool steps such as user input
  and model responses, which also caused them to ignore their configured
  matchers.
- Fixed slash commands not being recognized when separated from their
  arguments by a newline or tab, so a prompt starting with a command
  followed by a newline is now parsed as a command instead of being sent
  verbatim.
- Fixed deleting into a collapsed paste placeholder removing one
  character at a time, which left a visible fragment in the prompt while
  the full pasted content was still submitted; the block is now deleted
  atomically.
- Fixed the artifact viewer losing syntax highlighting when returning
  from the editor view, and returning to the wrong panel when exiting
  the artifact detail view.
- Fixed the headless `stream-json` `init` event advertising tools that
  are not available in your build.
- Fixed MCP servers forcing a full re-authentication after a dropped
  connection.

Patches (0)

[1.1.8](https://antigravity.google/download#antigravity-cli "View release 1.1.8")  
July 28, 2026

### Print mode structured output formats, custom JSON schema enforcement, copyOnSelect TUI setting, and compound-command permission improvements

Added structured output formats (`json`, `stream-json`) for print mode,
support for custom JSON schema validation, enriched tool and subagent
payloads, `copyOnSelect` configuration setting, and improved
compound-command permission rules.

Improvements (7)

- Print mode (`-p` / `--print`) now supports structured,
  machine-readable output via the `--output-format` flag (`text`
  (default), `json`, or `stream-json`), so headless runs in CI, eval
  harnesses, and scripts can consume the CLI's output programmatically;
  these flags are now discoverable in `--help`.
- Added the `stream-json` output format: a strongly-typed NDJSON event
  stream that emits typed `init`, `step_update`, and terminal `result`
  events with a stable, closed-vocabulary `step_type` discriminator, so
  consumers receive progress incrementally instead of waiting for the
  whole run to finish.
- Added the `--json-schema` flag to enforce a custom JSON schema on the
  structured output, accepting either an inline schema string or a path
  to a schema file; for `stream-json` the schema applies to the final
  `result` event.
- Enriched the structured stream with a `tool_info` object for each tool
  call (canonical tool name, parameters, and output) and a
  `subagent_info` payload for delegated subagents (including
  `conversation_id` and `log_uri`) so consumers can correlate child
  trajectories.
- The JSON usage object emitted by `json` and `stream-json` now reports
  token accounting including `cache_read_tokens`, so non-interactive
  consumers can attribute prompt-cache hits.
- Added a `copyOnSelect` setting (default on, toggleable in `/settings`)
  that controls whether releasing a mouse text-selection auto-copies it
  to the system clipboard in the TUI's altscreen rendering mode; disable
  it to stop the automatic copy on release — useful when the auto-copy
  is unwanted or corrupts certain payloads.
- Improved compound-command permissions so an exact chained command
  (such as `git fetch && git rebase`) can be saved as an allow-always
  rule and no longer re-prompts on the next identical run.

Fixes (0)

Patches (0)

[1.1.7](https://antigravity.google/download#antigravity-cli "View release 1.1.7")  
July 24, 2026

### Permission prompts for compound commands, and fixes for disabled plugins, MCP OAuth, and clipboard corruption

Improved permission prompts for compound shell commands, and fixed
disabled plugins running hooks, MCP OAuth issuer validation, CJK
clipboard copying on Windows, and `/btw` error on startup.

Improvements (1)

- Improved permission prompts for compound shell commands so the full
  command is shown when any part of it needs approval.

Fixes (5)

- Fixed disabled plugins still running their hooks and contributing
  other customizations, which could keep a broken hook active and break
  file-editing tools even after the plugin was turned off.
- Fixed MCP OAuth against providers that do not strictly follow the spec
  (such as Salesforce and Atlassian) by relaxing issuer validation and
  including the `refresh_token` grant.
- Fixed `/btw` failing with a "parent conversation not found" error when
  used as the very first action in a fresh session.
- Fixed clipboard corruption of CJK and other non-ASCII text when
  copying on Windows.
- Fixed print mode (`-p`) sending a prompt before the
  account-eligibility check finished.

Patches (0)

[1.1.6](https://antigravity.google/download#antigravity-cli "View release 1.1.6")  
July 24, 2026

### Markdown format custom agents, /copy index support, progressive /codesearch streaming, default temp directory access, and stability fixes

Added support for defining custom agents in Markdown format
(`agent.md`), optional index argument for `/copy`, progressive streaming
in `/codesearch`, default read access to system temporary directory, and
resolved various TUI jitter, artifact viewer, and sandbox fixes.

Improvements (7)

- Added support for defining custom agents using Markdown files
  (`agent.md`) with YAML frontmatter and H1-delimited system prompts.
  Markdown agents support `mainAgent`, `subagent`, `hidden`,
  `inheritMcp`, and `commandExecutionPolicy` frontmatter fields for
  fine-grained control over agent behavior. Dynamically defined
  subagents (via `define_subagent`) now also write Markdown format so
  they resolve correctly on external builds.
- Added an optional index argument to `/copy` so `/copy ` copies the
  n-th most recent response to the clipboard, while `/copy` and
  `/copy 1` still copy the latest.
- Improved `/codesearch` to render results progressively as they stream
  in, showing a live count while loading and letting you cancel an
  in-flight search with `Esc` instead of blocking until the whole search
  finishes.
- Improved default file access by granting read access to the system
  temporary directory out of the box, resolved correctly per platform,
  so agents no longer trigger permission prompts when reading temporary
  files.
- Improved support for markdown-based custom agents so custom agent
  management and selection behave more consistently.
- Improved customization discovery by sorting rules and discovered paths
  deterministically, preventing unstable prompt ordering and needless
  prompt-cache misses.
- Improved overall reliability and stability across the CLI with
  additional hardening and fixes for intermittent failures in background
  tasks, print mode, and interactive flows.

Fixes (11)

- Fixed switching from a custom agent back to the default agent via
  `/agents`, which previously failed silently and left the conversation
  stuck on the custom agent's persona.
- Fixed a crash when a command was blocked by sandbox permissions before
  its output was captured, and cleaned up the permission approval and
  denial messages.
- Fixed the artifact viewer emitting garbage escape bytes when cycling
  to image mode on terminals that are detected but cannot actually
  render Kitty graphics, such as iTerm2.
- Fixed the first keystroke (such as `Esc`) being dropped when opening
  the first artifact view on some non-Kitty terminals.
- Fixed conversation jitter and a stranded input box during streaming so
  transient markdown reflow no longer shifts the pending line and input
  box upward.
- Fixed print mode (`-p`) surfacing the real conversation-creation
  failure instead of a misleading "no active conversation" error.
- Fixed the message list dropping its header when rewinding or resetting
  conversation steps.
- Fixed a background auto-updater double-spawn race where two processes
  could each spawn an updater within a single update window.
- Fixed sandbox error reporting so blocked actions are recorded even
  when the network proxy is disabled.
- Fixed the screen going blank after the authentication page.
- Fixed the `ctrl+b` shortcut being hardcoded to background shell
  commands even when none were running, so a remapped `ctrl+b` is now
  respected whenever there are no running shell commands in the
  conversation.

Patches (0)

[1.1.5](https://antigravity.google/download#antigravity-cli "View release 1.1.5")  
July 21, 2026

### Reasoning effort command, model slug pinning, subagent model configuration, and TUI stability improvements

Added `/effort` command and `--effort` flag for reasoning effort
selection, stable model slugs, subagent model config, redesigned
`/model` picker, scrollable `/settings` panel, and various MCP and
background task fixes.

Improvements (8)

- Added a `/effort` command to view and change the current model's
  reasoning effort, with a left/right timeline-gauge picker and a direct
  `/effort <level>` form so you can trade latency for depth on the fly.
- Added an `--effort` flag to select a model's reasoning-effort variant
  when launching the CLI.
- Added stable, user-facing model slugs that appear in the `/model`
  picker and are accepted by `--model`, so you can pin a specific model
  reliably across sessions.
- Added a `model` option to custom agent frontmatter so an agent runs at
  a chosen model tier (such as `flash` or `pro`) when invoked as a
  subagent, defaulting to `inherit` (the parent's model).
- Redesigned the `/model` picker to group models by their base model and
  choose reasoning effort from a timeline gauge navigable with Left and
  Right, and added an effort badge to the status line for models that
  expose multiple effort variants.
- Improved the `/settings` (`/config`) panel by making it a bounded,
  scrollable list so it renders correctly in short terminals instead of
  overflowing, and stopped it from flickering when opening and closing
  dropdowns.
- Improved background-task reliability by moving long-running work onto
  a shared lifecycle with deterministic startup and shutdown and
  panic-safe launching, so a failure in one background task no longer
  disrupts the session and pending analytics are flushed on exit instead
  of dropped.
- Improved responsiveness of bursty background refreshes by coalescing
  rapid repeated triggers into a single run, cutting redundant work.

Fixes (5)

- Fixed a crash when triggering Authenticate on a remote MCP server in
  the `/mcp` panel.
- Fixed MCP tool results containing embedded resources being silently
  dropped, so text and inline media returned by MCP servers now surface
  in the conversation.
- Fixed permission checks splitting a single command into a pipeline
  when an argument contained quoted shell metacharacters (such as
  `--grep="a|b"`), which caused spurious permission prompts.
- Fixed the file-view and file-search tools failing with invalid-UTF-8
  errors when a multi-byte character was split at a truncation boundary.
- Fixed a data race when collecting customization rules by guarding the
  shared structures.

Patches (0)

[1.1.4](https://antigravity.google/download#antigravity-cli "View release 1.1.4")  
July 18, 2026

### Slash command chaining, diff scrolling fixes, and headless policy enforcement

Added slash command stacking (chaining multiple commands in a single
prompt), diff viewer scroll fixes, headless mode settings.json policy
enforcement, and subagent declaration fixes.

Improvements (2)

- Added support for stacking multiple leading slash commands in a single
  prompt, so a chain like `/plan /grill-me <prompt>` parses, activates,
  and renders every command in the order you typed them.
- Improved scrolling in the `/diff` viewer so paging through a diff no
  longer jitters or pushes the status line off the screen when lines
  wrap or comments expand.

Fixes (5)

- Fixed custom agents that declare `subagent: false` still appearing in
  the available-subagents list and being invocable as subagents.
- Fixed headless (`-p` / `--print`) runs so they now honor persisted
  `settings.json` policies, including `permissions`, file access,
  sandbox mode, auto-execution, and artifact review.
- Fixed `/btw` side-questions leaking into the conversation list as
  duplicate entries that carried the parent conversation's title.
- Fixed the prompt to honor a custom Enter binding to
  `prompt.insert_newline`, so a remapped Enter inserts a newline instead
  of submitting.
- Fixed eligibility error messages so the CLI shows the real reason
  again instead of defaulting to a generic "unknown reason".

Patches (0)

[1.1.3](https://antigravity.google/download#antigravity-cli "View release 1.1.3")  
July 16, 2026

### Codesearch command, copy-on-select, and performance optimizations

New `/codesearch` command for regex workspace searches, copy-on-select,
async skill discovery, and stability fixes.

Improvements (7)

- Added a `/codesearch` command (aliases `/cs` and `/search`) to
  interactively search code across your workspace, interpreting queries
  as regex by default with `-F`/`--literal` for exact matching and
  `f:`/`file:` globs to include or exclude paths.
- Added copy-on-select in no-flickering mode so dragging highlights text
  and releasing the mouse copies the ANSI-stripped selection to the
  clipboard, and hides the virtual scrollbar so it no longer interferes
  with copying multi-line output.
- Added an indicator at each context-compaction boundary so you can see
  where earlier compaction happened.
- Improved interactive startup latency by loading skills asynchronously
  so the CLI no longer blocks on a synchronous, filesystem-heavy
  skill-discovery pass during bring-up.
- Improved eligibility error handling by showing errors with a
  verification URL inline in the input loop instead of stacking them
  above the screen.
- Improved customization loading latency for skills, rules, agents, and
  hooks by consolidating directory walks and caching filesystem lookups
  to cut redundant I/O during discovery.
- Removed the padding spaces around inline code for tighter rendering.

Fixes (9)

- Fixed code-block corruption where `$..$` math expansion desynced from
  the Markdown parser and mangled fenced shell snippets such as
  `git fetch "$GIT_REMOTE"` by detecting fenced code blocks
  line-by-line.
- Fixed headless (`-p`) runs hanging or silently auto-approving tools
  that require a permission confirmation, so the CLI now soft-denies
  such tools and prints a stderr notice naming the allow-rule needed to
  permit them.
- Fixed outside-of-workspace file writes being incorrectly auto-approved
  in always-proceed mode.
- Fixed high CPU and unbounded render cost on large conversations in
  no-flickering mode by making index rebuilds idempotent so the
  conversation index converges instead of growing on every rebuild.
- Fixed lingering artifact comments after dismissing the artifact detail
  view and corrected no-flickering-mode row math so the status line
  renders correctly within the viewport.
- Fixed repeated sign-in prompts on Linux caused by the OS keyring: the
  CLI now bypasses the keyring when no D-Bus session bus is present
  (headless hosts and containers), skips it for an hour after a timeout,
  and uses longer keyring timeouts so a slow-but-successful credential
  read is no longer cut short and forced into a fresh login.
- Fixed MCP servers hanging the agent indefinitely when a server never
  responds by bounding connection, tool-listing, and per-tool-call
  attempts with timeouts.
- Fixed conversations breaking after certain tool calls, which
  previously corrupted the conversation history and blocked all further
  responses.
- Fixed customization rules being loaded twice when a rules directory is
  reachable through a symlink.

Patches (0)

[1.1.2](https://antigravity.google/download#antigravity-cli "View release 1.1.2")  
July 13, 2026

### Full diff preview, OAuth print mode improvements, and lag fixes

Added full-screen diffs for file creations, OAuth authorization code
pasting in print mode, and 5000+ step latency optimizations.

Improvements (3)

- Added an `f` (full diff) shortcut to the create-file tool review
  screen so new-file confirmations can open a full-screen diff view,
  matching the existing file-edit experience.
- Added support for pasting the OAuth authorization code in print mode
  (-p) via the controlling terminal (/dev/tty on POSIX and CONIN\$ on
  Windows) when stdin is consumed by a piped prompt, and made truly
  headless runs fail fast with an actionable message instead of
  blocking.
- Improved responsiveness on large conversations (5000+ steps) in no
  flickering mode by switching hot-path line-count methods to pointer
  receivers, cutting the per-frame prefix-sum cost and eliminating
  sustained 99% CPU and keystroke lag.

Fixes (6)

- Fixed print mode silently downgrading to the default model when
  --model cannot be resolved by hard-failing with a non-zero exit and
  listing the available models, while interactive sessions keep the
  fallback-with-warning behavior.
- Fixed permission checks not respecting the allowlist for nested
  command substitutions, so a command like echo "\$(dirname \$(git
  rev-parse --show-toplevel))" now runs without prompting when echo and
  git are allowlisted, instead of double-counting the nested command and
  prompting for review.
- Fixed the CLI keybindings file staying out of sync with /keybindings
  when new default bindings are introduced by persisting the injected
  defaults while preserving user overrides.
- Fixed garbled builtin tool headers such as CodeSearch(4 files
  found...) by mapping generic tool steps back to clean summaries like
  Read(/path) and CodeSearch(query).
- Fixed mcp manager failing to resolve tool schema paths in standalone
  mode and leaking MCP server subprocesses after shutdown, which
  previously caused panics and cleanup failures for custom agents
  loading MCP tools.
- Fixed a data race and copy-on-write violation when updating subagent
  states by cloning their stats before in-place mutation, preventing
  corrupted step counts and status for parallel subagents.

Patches (0)

[1.1.1](https://antigravity.google/download#antigravity-cli "View release 1.1.1")  
July 10, 2026

### Custom agents flag, in-file keyword search, and Jujutsu diff fixes

Added `--agent` flag, in-file keyword search for artifacts, recursive
subagent updates, and Jujutsu workspace diff fixes.

Improvements (6)

- Added the `--agent` flag and `agent/agents` subcommand, allowing users
  to select a custom agent at launch and list available agents.
- Added in-file keyword search (`/`) and jump navigation (`n/N`) to the
  artifact detail viewer, allowing users to find and cycle through
  matches without disrupting terminal escape sequences or image grids.
- Added support for displaying nested subagents (grandchild and deeper)
  and handling tool confirmation requests across all subagent depths by
  recursively relaying nested subtrajectory updates to the root
  conversation.
- Changed the default mode to respect write_file permissions allowlisted
  in `settings.json` under `permission.allow`, so pre-approved file
  writes no longer prompt for review.
- Changed the default name for the newly initialized project to
  `CLI Project` for clearer workspace identification.
- Improved the session exit output by placing the resume command on its
  own line, making it easier to copy and paste in terminals and tools
  like tmux.

Fixes (6)

- Fixed print mode (`--print` / `-p`) silently exiting with a success
  code and empty output when a request failed server-side, now writing
  the error to stderr and returning a non-zero exit code.
- Fixed `agy -p` hanging when run inside a shell script or subprocess by
  no longer reading stdin when a prompt is provided via a flag.
- Fixed a data race on the `/btw` cancellation function.
- Fixed interactive `/diff` viewer defects in Jujutsu (jj) workspaces by
  correctly prioritizing `.jj` over `.git` in colocated repos, fixing
  commit hash regex boundaries, and correctly highlighting active `@`
  graph nodes.
- Fixed workspace-local hooks defined in `/.agents/hooks.json` not
  loading after trusting a folder by reloading hooks whenever workspaces
  change.
- Fixed misaligned markdown tables containing file links in chat output.

Patches (0)

[1.1.0](https://antigravity.google/download#antigravity-cli "View release 1.1.0")  
July 8, 2026

### Request-review mode, execution mode cycling, and line-level diff previews

Public release of mode cycling, default request-review mode with
interactive line-level diff previews, and settings panel integration.

Improvements (8)

- Agent execution mode cycling is now publicly available: `default` -\>
  `accept-edits` -\> `plan`)
- Added `request-review` (default) mode as the default execution
  behavior: automatically pauses before file write operations to display
  an interactive, line-level diff preview (`f` shortcut) where users can
  review, accept, or reject individual code modifications before they
  are saved to disk.
- Added an `Agent Mode` option to the `/settings` panel so users can set
  and persist a default execution mode (`default`, `accept-edits`,
  `plan`) without manually editing `settings.json` or passing `--mode`
  on startup, with real-time synchronization so changes take effect
  immediately.
- Added a dedicated `"Create file"` confirmation preview for new file
  creations (`write_to_file` without overwrite): renders new content as
  an addition-only diff preview.
- Added `/plan` mode to replace legacy `/planning`, and removed `/fast`
  slash commands: consolidated and simplified execution mode switching
  around `shift+tab` mode cycling and the `/plan` mode prefix
- Improved file-edit diff preview rendering: computed accurate
  line-level diffs with context lines (`3` lines) and hunk separators,
  capped inline preview height with truncation hints, and added a
  comment confirmation prompt when exiting the diff view with unsent
  comments.
- Improved UI footer keybinding hints across all panels (such as
  `/tasks`, `/agents`, `/permissions`, and `/mcp`) by replacing
  hardcoded hint strings with centralized layout helpers that
  dynamically respect customized global and local keybinding
  configurations (`keybindings.json`).
- Improved the multiline conversation rename view in the `/resume`
  picker by dynamically adjusting input box width and padding, and
  right-aligning metadata columns (`workspace`, `steps`, `time`) on the
  top line to prevent horizontal scrolling or layout shifts during
  active renaming.

Fixes (6)

- Fixed the tool confirmation dialog to accurately check normalized file
  URIs against active workspace directories, resolving an issue where
  valid in-workspace file creations and reads were incorrectly flagged
  with an `"Reason: outside workspace"` warning.
- Fixed workspace initialization failures when launching the CLI inside
  dot-prefixed directories (such as .parent_dir/project ) by scoping
  path exclusion filters strictly to relative paths inside the workspace
  rather than rejecting dot-prefixed ancestor directories.
- Fixed the `/agents` view header displaying `agent.json` instead of
  `agent.md` when creating new subagents.
- Fixed the `/agents` panel's `"Create New Agents"` section displaying
  the wrong global configuration directory (`~/.gemini/antigravity-cli/`
  instead of `~/.gemini/config/`), ensuring users create global
  subagents in the location actively scanned during startup discovery.
- Fixed statusline shortcut hints (`? for shortcuts`) and redundant
  escape hints (`Esc to cancel`) erroneously appearing inside
  full-screen overlay panels (such as `/changelog`, `/artifact`, and
  `/settings`) by correctly tracking overlay panel states.
- Fixed inconsistent timestamp formatting in the `/tasks` panel and task
  detail views by converting agent-initiated background task timestamps
  (`time.Time`) from UTC to the local timezone.

Patches (0)

[1.0.16](https://antigravity.google/download#antigravity-cli "View release 1.0.16")  
July 2, 2026

### Streaming task logs, model generation retries, and markdown subagents

Auto-scrolling background task logs, automatic client-side generation
retries, and markdown-based dynamic subagents.

Improvements (2)

- Improved the `/tasks` detail panel to automatically scroll to the
  bottom as new background task logs stream in, and default to the
  latest output when opened while preserving scroll position if scrolled
  up manually.
- Improved model generation resilience by adding automatic client-side
  retries when encountering transient errors.

Fixes (4)

- Fixed dynamically defined subagents by transitioning definitions from
  JSON to Markdown format, fixing an issue where dynamically created
  subagents failed to invoke.
- Fixed a crash occurring when executing background tasks or terminal
  commands that produce empty outputs (such as `sleep`).
- Fixed shutdown resource leaks by integrating the shared SQLite summary
  store for background synchronization and resolving goroutine and
  database connection leaks on CLI exit.
- Fixed a permission manager hook error by safely handling empty
  decision strings returned by pre-tool hooks instead of failing with an
  "unknown pre-tool hook decision" error.

Patches (0)

[1.0.15](https://antigravity.google/download#antigravity-cli "View release 1.0.15")  
July 1, 2026

### Subagent & task status bar, editor warnings, and Windows fixes

New interactive status bar for subagents and background tasks, clipboard
image pasting improvements, and Windows fixes.

Improvements (5)

- Introduced a new interactive status indicator below the input box that
  displays active subagents and background tasks in real-time, making it
  easy to monitor and navigate parallel workflows at a glance.
- Added `ctrl+g` on the artifact view to open \$EDITOR. Also added a
  warning confirmation prompt before opening the editor in the artifact
  detail view if there are unsent comments, and ensured these comments
  are preserved upon reload if the artifact content was not modified.
- Added `alt+v` as an alternative paste shortcut on Windows to resolve
  issues where ctrl+v is intercepted by the terminal emulator, enabling
  reliable image pasting.
- Improved the `/permissions` panel to dynamically reload configurations
  from disk and prevent accidental overwrites.
- Increased the MCP connection timeout to 60 seconds to improve
  reliability for slow-starting custom MCP servers.

Fixes (4)

- Fixed a bug on Windows where print mode and other non-TUI command
  outputs were silently discarded when run in non-TTY environments (such
  as pipes or subprocesses).
- Fixed Windows editor fallback to use "edit" or "notepad" when the
  editor setting is "auto" and no editor is configured, instead of
  attempting to use "vim".
- Fixed the subagent approval TUI to dynamically render user-defined
  custom keybindings (such as alternative approval keys) instead of
  showing hardcoded defaults.
- Fixed alignment and wrapping issues in the comment editor for
  multiline comments, ensuring all lines are indented consistently.

Patches (0)

[1.0.14](https://antigravity.google/download#antigravity-cli "View release 1.0.14")  
June 30, 2026

### Indefinite goals, subagent auto-approvals, and plugin fixes

Indefinite goals support, subagent auto-approval mode for artifacts, and
directory preservation during plugin imports.

Improvements (3)

- Allowed image pasting from the clipboard in local tmux sessions.
- Removed the max limit for the `/goal` command, allowing goals to run
  indefinitely until completed or cancelled.
- Enabled "always proceeds" mode for subagents to auto approve
  artifacts, preventing them from hanging when the parent is blocked.

Fixes (4)

- Fixed plugin import logic to copy the entire plugin directory,
  preventing it from stripping non-skill directories (like `shared/`).
- Fixed an MCP configuration path mismatch in the CLI and permission
  manager to ensure reliable custom MCP server loading.
- Fixed a TUI layout race condition caused by stale input state in the
  conversation model.
- Fixed a bug where the inline viewport was not properly reset after a
  conversation rewind.

Patches (0)

[1.0.13](https://antigravity.google/download#antigravity-cli "View release 1.0.13")  
June 27, 2026

### Optimistic slash commands, strict permission matching, and multiline undo

Optimistic slash command rendering, strict non-regex permission
matching, and unified undo/redo prompt history.

Improvements (2)

- Improved command permission security by making "Always Approve" rule
  matching strict (non-regex) by default, while allowing users to
  explicitly opt-in to regex matching by prepending rules with `regex:`.
- Improved command permission usability by relaxing redirection checks,
  allowing safe commands with output redirection (e.g., `tool > file`)
  to match without requiring strict full-command approval.

Fixes (5)

- Fixed a bug where the CLI would temporarily render skill commands
  without their slash prefix during optimistic updates by deferring
  prefix stripping to the serialization boundary, ensuring the UI always
  displays exactly what the user typed.
- Fixed a redundant CLI exit message by removing the "Resume in the same
  project" hint line, leaving only the standard resume command to
  simplify exit output.
- Resolved bugs during UI transitions (such as opening subagent details
  or logging out) by introducing a unified synchronization mechanism
  that prevents key lockups and ensures overlay panels like the /help
  view are properly reset.
- Fixed a bug in the CLI prompt editor where undo and redo history
  stacks could become desynchronized during rapid mutations by
  decoupling the history state into a unified, pointer-backed structure.
- Fixed a bug where browser-related prompt sections were missing from
  the agent's prompt registry, ensuring browser-based tasks execute
  reliably.

Patches (0)

[1.0.12](https://antigravity.google/download#antigravity-cli "View release 1.0.12")  
June 24, 2026

### Project launch flags, terminal hyperlinks, and shift+n diff cycling

Added `--project` launch flags, dynamic OSC8 terminal hyperlinks,
reverse diff cycling, and permission priorities.

Improvements (5)

- Added support for `--project` and `--new-project` launch flags to
  allow users to explicitly set or create projects, and updated the
  project resolution logic to default regardless of the active
  workspace.
- Added a confirmation prompt when pressing `Esc` in comment mode with
  unsaved modifications to prevent users from accidentally discarding
  their work in review views.
- Added dynamic OSC8 terminal hyperlink support to render clickable
  links in supporting terminals, with automatic fallback stripping for
  backward compatibility.
- Introduced reverse diff cycling navigation mapped to `shift+n` in
  unified diff review mode to allow users to easily cycle backwards
  through diff blocks.
- Improved permission config merging priorities by ensuring
  project-specific configurations (located in
  `~/.gemini/config/projects/`) take precedence over global settings in
  `~/.gemini/antigravity-cli/settings.json`.

Fixes (4)

- Fixed a regression where `ctrl+o` scrollback clearing failed by
  restoring the use of cached fields rather than shared pointer
  comparisons for trajectory toggle detection.
- Fixed a rendering bug where Makefile syntax (like `$(call ...)`)
  inside code blocks was mistakenly parsed and mangled by LaTeX math
  expansion, by introducing a state machine that restricts expansion to
  prose segments.
- Fixed an enterprise network connectivity issue by restoring AES-NI
  compile-time optimizations, which prevents Deep Packet Inspection
  (DPI) firewalls from incorrectly flagging and resetting TLS
  connections.
- Fixed incorrect key strings by removing the unsupported backtab
  default binding and correcting invalid `pgdn` references to `pgdown`
  to align with Bubble Tea v2 canonical names.

Patches (0)

[1.0.11](https://antigravity.google/download#antigravity-cli "View release 1.0.11")  
June 24, 2026

### Ctrl+C interrupt flow, persistent resume cache, and AltScreen tool review

Added `ctrl+c` interrupt and exit handling, parallel `/resume` metadata
caching, and expanded AltScreen tool confirmations.

Improvements (12)

- Added `ctrl+c` as an exit and interrupt key: the first press cancels
  active agent operations (like streaming responses), and a double-press
  triggers the exit flow. Also added a dynamic exit hint in the status
  line.
- Improved `/resume` loading performance by implementing a persistent
  metadata cache and parallel loader, eliminating severe latency with
  large conversation histories and preventing background loading log
  spam.
- Added an expanded AltScreen view for tool confirmations (accessible
  via `ctrl+g`), allowing users to view and edit the full command and
  associated permissions in a dedicated full-screen view, replacing the
  inline edit (`e`) key.
- Added the `AGY_CLI_CMD_OUTPUT_PERCENTAGE` environment variable,
  allowing users to customize the maximum height of command outputs in
  the TUI as a percentage of the terminal height.
- Added strict key name validation to the keybindings system to reject
  invalid key names (like typos) and suggest canonical alternatives,
  preventing "dead keys" from being registered.
- Added a validation warning when `ctrl+c` is mapped to a non-default
  action, clarifying that the system always intercepts `ctrl+c` to
  interrupt active operations or exit, and providing instructions on how
  to resolve the warning.
- Improved command output rendering by making the output height dynamic,
  improving the readability of commands like `/keybindings`.
- Improved text rendering with ANSI-aware word wrapping at word
  boundaries and prevented URLs containing hyphens from being
  incorrectly split across lines.
- Improved the `/resume` experience: added support for pasting clipboard
  text into the search filter and rename fields, upgraded the rename
  input to a multiline editor to prevent long titles from being hidden,
  and fixed a bug where the navigation cursor could disappear.
- Improved keybinding validation warning messages to use user-facing
  names (e.g., `cli.escape`) instead of internal representation names.
- Improved startup behavior by only creating the `keybindings.json`
  configuration file when the user explicitly runs the `/keybindings`
  customization command, rather than automatically generating it on
  every startup.
- Improved keybinding error presentation by replacing the persistent
  error footer with transient error alerts, freeing up valuable terminal
  space.

Fixes (4)

- Fixed `ctrl+d` behavior to act as a forward-delete when the input
  prompt contains text, only triggering the exit flow when the prompt is
  empty.
- Fixed the `ctrl+c` exit safety valve to ensure it always works as an
  interrupt or exit key, regardless of how it is mapped in the user's
  custom keybindings configuration.
- Fixed VCS commit tree rendering to reserve the `@` marker exclusively
  for the actual current commit in the VCS history rather than the
  synthetic "Working Copy" entry, helping users easily identify the
  working copy parent.
- Fixed authentication error handling to gracefully handle unsigned-in
  states by returning an empty configuration and suppressing noisy error
  logs.

Patches (0)

[1.0.10](https://antigravity.google/download#antigravity-cli "View release 1.0.10")  
June 19, 2026

### ARM64 device support, builtin guide skill, and ASCII node graphs

Expanded ARM64 device compatibility, added builtin `antigravity_guide`
skill, and enabled ASCII Git log graphs.

Improvements (9)

- Improved compatibility with a broader set of ARM64 devices (e.g.
  raspberry pi 4b).
- Added `antigravity_guide` builtin skill to provide instant, in-context
  reference guides for the Antigravity 2.0, CLI, IDE, and SDK.
- Improved commit history navigation: scrolling now immediately loads
  and displays changed files and diffs.
- Improved Git integration by enabling ASCII node graphs
  (`git log --graph`) for visual parity with hg/jj.
- Improved commit hash matching to seamlessly resolve short (6-char) to
  long (64-char) hashes via prefix comparison.
- Added alert message type for system errors/warnings, separating them
  from standard command output.
- Added the CLI log file path to the `/help` menu for easy
  troubleshooting.
- Improved markdown rendering by upgrading `glamour` to v2.0.1 for
  cleaner headings and block padding.
- Improved authentication to automatically launch browser sign-in via
  `rundll32`.

Fixes (4)

- Fixed a bug where "ask" permissions were dropped during settings
  updates, ensuring `settings.json` preservation.
- Fixed permission engine matching bugs by escaping regex metacharacters
  (like `$` or `.`) in saved rules, preventing infinite prompt loops.
- Fixed environment flag parsing to prevent ignored disablement flags.
- Fixed bash mode argument escaping (preventing swallowed stdout) and
  defaulted shell resolution to PowerShell.

Patches (0)

[1.0.9](https://antigravity.google/download#antigravity-cli "View release 1.0.9")  
June 17, 2026

### Git submodule plugins, read-only permissions, and sandbox hardening

Automatic Git submodule resolution for plugins, read-only access for
builtin skills, and sandbox path hardening.

Improvements (4)

- Added submodule support for plugins installation. External plugin
  installation now automatically resolves and initializes Git
  submodules.
- Optimized customizations permissions: Automatically grants read-only
  access to the builtin customizations directory, eliminating redundant
  permission prompts on startup.
- Improved glamour parser error handling (like nested checkboxes inside
  list emphasis) and preventing it from crashing the TUI, falling back
  to raw text with a warning banner.
- Updated bubbletea to v2.0.7: Resolves a potential TUI panic when
  terminal input is unavailable, fixes a data race in mouse handling
  within the Cursed Renderer, and corrects mouse release behavior under
  the Kitty Keyboard protocol.

Fixes (5)

- Hardened command execution permission checks by enforcing strict
  exact-match verification for PowerShell scripts, complex shell
  redirections ( `>` , `2>&1` ), and unparseable strings to prevent
  sandbox escapes.
- Hardened sandbox execution by adding `.git` to the core list of
  dangerous paths, preventing unauthorized or destructive repository
  modifications.
- Fixed a bug where allowlisted terminal commands with quoted arguments
  (e.g., `python -c "print(1)"`) would silently fail to match at runtime
  due to flawed whitespace tokenization.
- Fixed a bug in headless print mode resumption (`--conversation`/`-c`
  `-p ...`) where the CLI would dump the entire historical conversation
  transcript instead of only printing the newly generated response.
- Fixed a CPU compatibility issue on ARM64 devices without AES hardware
  support.

Patches (0)

[1.0.8](https://antigravity.google/download#antigravity-cli "View release 1.0.8")  
June 12, 2026

### Slash command history, Models & Quota page, and TUI performance

Replay slash command history with Up arrow, redesigned Models & Quota
page, and statusline quota usage indicators.

Improvements (9)

- Added support for capturing slash command history, allowing users to
  use the up arrow to replay previously entered slash commands.
- Redesigned the "Models & Quota" page (enabled by default, replacing
  the legacy usage page) to gracefully handle disabled quota buckets by
  displaying a dimmed "Disabled" status and omitting the progress bar.
- Added display of quota usage and execution mode in the status line.
- Improved `/btw` to be more token efficient and support streaming
  responses for a smoother user experience and fixed premature
  truncation.
- Added a per-line guard against extremely long single-line pastes in
  the TUI prompt editor to prevent performance lag, replacing them with
  an expandable placeholder.
- Redesigned the `/resume` conversation picker to align the workspace
  column and added adaptive column dropping (workspace, time, steps) to
  support narrow terminals.
- Redesigned the `/tasks` list and detail views for better alignment and
  readability, placing start times on the left, right-aligning status,
  and capping the panel height.
- Improved configuration saving by propagating write failures as
  transient error flashes on the statusline.
- Improved settings inheritance by ensuring the CLI inherits the
  `use_ai_credits` setting from global user settings on startup.

Fixes (8)

- Fixed a bug where the `/hooks` command wrote configurations to
  `~/.gemini/antigravity-cli/hooks.json` instead of the shared
  `~/.gemini/config/hooks.json`, ensuring hooks remain synchronized
  between the TUI and the backend.
- Fixed a CPU compatibility issue (SIGILL on non-AES-NI CPUs),
  preventing immediate crashes on startup on older CPUs (like Intel Ivy
  Bridge) or VM environments that lack AES-NI support.
- Fixed dynamic reloading of custom skills and system slash commands,
  ensuring they are instantly discovered in autocomplete upon
  conversation switch or `/add-dir`.
- Fixed a TUI hang in the artifact view during long sessions by
  optimizing the rendering complexity of large step histories.
- Fixed an autocomplete bug where a command that is an exact prefix of
  another (e.g., `/conv` vs `/conv-switch`) would aggressively
  auto-complete and hide the suggestions menu.
- Fixed a race condition where sending a message immediately after
  denying a permission request would fail due to incomplete backend
  cleanup.
- Fixed potential OOM risks when reading large clipboard files by
  verifying file size before reading.
- Fixed Windows and Wayland-only Linux distributions clipboard image and
  file reading.

Patches (0)

[1.0.7](https://antigravity.google/download#antigravity-cli "View release 1.0.7")  
June 9, 2026

### Configurable MCP timeouts, artifact gutter line mapping, and Wayland clipboard

Configurable MCP server connection timeouts, accurate line-numbered
artifact gutters, and native Wayland clipboard support.

Improvements (6)

- Added a configurable timeout for launching MCP servers, allowing users
  to specify a custom timeout or set it to `-1` to disable the timeout
  completely.
- Revamped the artifact viewer gutter numbering and line mapping to
  accurately align terminal viewport lines with actual 1-based source
  file line numbers, including support for wrapped lines and collapsed
  Mermaid diagrams.
- Increased the maximum tool calls limit to 512 for Gemini models,
  allowing agents to perform significantly more complex, multi-step
  tasks in a single turn.
- Added support for installing plugins directly from GitHub subpaths
  (with branch resolution).
- Added native Wayland clipboard support (wl-paste) on Linux, falling
  back to `xclip` for X11 environments, and prioritized copied files
  (from file managers) over raw image data.
- Preserved unknown fields in `settings.json` during read, write, and
  merge operations, preventing settings from being silently wiped out
  when switching between different CLI versions or builds.

Fixes (8)

- Fixed a bug where the CLI could get stuck in a pending state (showing
  a transient spinner) after sending a message due to stale status
  updates.
- Fixed a bug where the wrong workspace directory was displayed in the
  header and `/help` menu when multiple workspaces were active.
- Fixed a desync bug in the agent state management where stale callbacks
  from previous runs could be used upon cache hits in new agent state.
- Fixed Windows-specific sandbox network proxy issues, resolving a hang
  during connection hijacking and correcting tunnel response protocols.
- Fixed a bug where the archival status timestamp was not correctly
  saved when archiving conversations.
- Fixed a potential stack overflow crash by introducing a non-recursive
  warning output mechanism for pre-conversation errors.
- Fixed variable resolution in plugins, ensuring gemini cli variables
  like `${extensionPath}` correctly resolves to the final installation
  directory.
- Fixed layout boundary overflow, scrolling visibility, and
  out-of-bounds scrolling bugs in the artifact detail view when inline
  comments are present.

Patches (0)

[1.0.6](https://antigravity.google/download#antigravity-cli "View release 1.0.6")  
June 6, 2026

### Path auto-completion, optimistic prompt rendering, and fuzzy matching

Path autocompletion for `/open` and `/add-dir`, optimistic chat prompt
rendering, and fuzzy slash command matching.

Improvements (5)

- Added shell-style path auto-completion for `/open` and `/add-dir`.
- Added optimistic rendering for user chat prompt submissions, injecting
  messages immediately into the viewport to eliminate perceived input
  lag.
- Added fuzzy and partial substring matching across slash commands. E.g.
  `/el` -\> shows `/help` and `/model` while previous no suggested
  completions.
- Skipped subagent conversations from `/resume`, keeping the standalone
  conversation picker focused purely on direct user initiated
  conversations.
- Added a `stack_with_default` flag to the `statusLine` configuration to
  render both the default Antigravity status line and custom status line
  output vertically stacked.

Fixes (3)

- Fixed a bug when suggestion was not triggered when `@` is typed after
  `(`. Enabled unconditional typeahead suggestions whenever `@` is typed
  without preceding whitespace, streamlining mention workflows.
- Fixed a bug where entering a prompt immediately after pressing `Esc`
  (to interrupt an active agent stream) caused the newly typed input to
  be swallowed or rejected.
- Fixed `--sandbox` flag propagation in headless print mode (`-p` /
  `--print`), ensuring sandbox isolation is correctly enforced during
  non-interactive execution.

Patches (0)

[1.0.5](https://antigravity.google/download#antigravity-cli "View release 1.0.5")  
June 3, 2026

### Model selection flags, in-CLI permissions editor, and URL MCP servers

Added `--model` launch flag, interactive `/permissions` editor, and
URL-configured MCP servers.

Improvements (9)

- Added `--model` to set model when launching CLI. Also a new `models`
  subcommand to list available models.
- Added `/permissions` command which allows to add/edit/remove
  permissions rules for each of the three configs above directly inside
  the CLI.
- Allowed opening the Artifact Review panel (shortcut `ctrl+r`) while
  answering pending questions or tool permission confirmations,
  preserving your current progress when toggling back.
- Improved statusline layout by merging active tip and artifact status
  on a single line and truncating with ellipsis on narrow terminals to
  prevent collisions.
- Improved customization support by allowing directories in the
  customization manager to be passed as workspace directories, enabling
  correct trajectory metadata population and `/add-dir` support.
- Added support for `url` in `mcp_config.json` to configure MCP servers
  directly via a URL.
- Improved `/resume` performance: optimized lazy loading of conversation
  details, filtered out empty conversations, and added support for
  scanning SQLite database files (`.db` and `.db-wal`).
- Improved autocomplete: tab completion for slash commands now resolves
  to the matched alias instead of the primary command name (e.g., `/se`
  autocompletes to `/settings` instead of `/config`).
- Integrated the permissioning system with the rest of Antigravity. CLI
  permissions now merges project level permissions, permissions from
  user settings shared with Antigravity, and permissions from the CLI
  `settings.json`.

Fixes (1)

- Fixed a bug that metadata was written in the current directory as
  opposed to `~/.gemini/antigravity-cli/cache` when running using `-p`.

Patches (0)

[1.0.4](https://antigravity.google/download#antigravity-cli "View release 1.0.4")  
June 1, 2026

### SQLite conversation store, LaTeX math rendering, and centralized project cache

SQLite conversation database format, terminal LaTeX math rendering, and
centralized project discovery cache.

Improvements (8)

- Added SQLite (.db) conversation support and will be CLI’s conversation
  format. Fixed a bug when importing SQLite conversation from
  Antigravity 2.0 to CLI.
- Added LaTeX math rendering, enabling the CLI to display beautiful
  mathematical formulas directly in the terminal viewport. Set
  `AGY_CLI_DISABLE_LATEX` environment variable to turn off LaTeX
  rendering globally if desired.
- Decoupled project discovery from local `.antigravitycli` workspace
  directories. The CLI now stores workspace-to-project mappings in a
  centralized `~/.gemini/antigravity-cli/cache/projects.json` file,
  eliminating repository clutter and speeding up project discovery to a
  single-map lookup.
- Collapses all newlines and consecutive whitespaces in conversation
  previews and titles before rendering list items, preventing visual UI
  layout breaks in the picker rows.
- Styled the separator space between the line number column and diff
  content to match the text blocks, ensuring background highlights
  stretch seamlessly across the viewport width in tool outputs and
  `/diff` details.
- Aligned the interactive `/changelog` and `agy changelog` cache paths
  to both use `antigravity-cli`, and made the caching process
  synchronous to resolve a race condition where immediate process exit
  terminated the cache write.
- Moved VCS detection out of the synchronous CLI startup path to prevent
  slow initialization.
- Parallelized the MCP server initialization sequence, preventing slow
  or hanging custom MCP servers from blocking independent, fast-starting
  servers (like local plugins) from loading on startup or configuration
  reloads.

Fixes (3)

- Resolved sporadic and permanent UI hangs caused by a stateful callback
  streamer race condition during network drops or extremely fast agent
  steps.
- Resolved inconsistent behavior where selecting skill-derived slash
  commands from autocompletion suggestions cleared the input without
  executing. Autocompleted skill commands are now correctly submitted to
  the backend.
- Resolved an issue where exclusion rules and allowlists configured in
  rules.json were silently ignored, causing the discovery engine to load
  every .md rule file unconditionally at boot.

Patches (0)

[1.0.3](https://antigravity.google/download#antigravity-cli "View release 1.0.3")  
January 1, 2026

### G1 AI credits support, /credits panel, and input prompt fixes

G1 AI credits support when standard quotas run out, in-CLI `/credits`
panel, and prompt input fixes.

Improvements (4)

- Added support for G1 credits in the Antigravity CLI. Users can now
  utilize G1 credits when their standard quota runs out. This includes a
  new `UseG1Credits` setting to enable automatic credit usage and a
  real-time display of remaining credits in the status bar.
- Added a new `/credits` panel that provides an in-CLI interface with a
  direct link to purchase additional G1 credits.
- Redesign CLI logo on Apple Terminal.
- Improved color scheme preview in settings and onboarding: added
  warnings and thought process examples to the preview, and corrected
  link styling to only underline the URL.

Fixes (6)

- Fixed an infinite loop in the prompt input. Navigating left
  (`wordLeft`) when encountering spaces at the very beginning of the
  input no longer causes an infinite hang.
- Fixed custom MCP server disabling via the TUI. Resolved a directory
  path mismatch where pressing the `[Disable]` button wrote to the
  legacy `mcp_config.json` path instead of the migrated
  `config/mcp_config.json` path, ensuring custom MCP servers can now be
  successfully disabled and unloaded.
- Fixed `$EDITOR` environment variable parsing: resolved issues where
  arguments containing `=` (e.g., `--alternate-editor=vi`) were
  incorrectly split, causing editor launch failures.
- Fixed `/diff` detail view truncation: implemented dynamic line
  wrapping based on terminal viewport width and added automatic
  tab-to-space expansion to prevent layout overflow.
- Fixed project discovery robustness: updated the CLI to skip invalid or
  broken symlinks in `.antigravitycli/` rather than failing immediately,
  allowing discovery of valid projects.
- Fixed `AskQuestion` state management: memorizes selected options,
  write-in values, and UI states when navigating back and forth
  (`KeyLeft`) between questions in multi-question dialogs.

Patches (0)

[1.0.2](https://antigravity.google/download#antigravity-cli "View release 1.0.2")  
January 1, 2026

### Account hiding flag, subagent timeouts, and TUI fixes

Added `AGY_CLI_HIDE_ACCOUNT_INFO` flag, subagent-specific interaction
timeouts, and TUI fixes.

Improvements (2)

- Added `AGY_CLI_HIDE_ACCOUNT_INFO` environment variable to hide email
  and plan tier from the header.
- Improved `/help` shortcuts tab by sorting shortcuts by keybinding key,
  adding missing keybindings (like `ctrl+r`, `ctrl+o`, `alt+j`,
  `ctrl+k`), and generalizing scrolling
  (PageUp/PageDown/GoToTop/GoToBottom) for both Commands and Shortcuts
  tabs.

Fixes (7)

- Fixed timeout overrides: restricted the default 60-second interaction
  timeout specifically to subagents, preventing the main agent from
  being unconditionally capped.
- Fixed a nil-pointer panic in Sandbox Mode: resolved a typed nil
  interface comparison when fetching URL content.
- Fixed fallback skill discovery in Standalone mode: ensures
  custom/fallback skills are successfully loaded even if the standard
  configuration directory is missing, and added automatic path
  deduplication to prevent duplicates.
- Fixed command rendering in message history: prefixed slash commands
  with a caret (`>`) in response block headers to clearly distinguish
  user-typed commands from agent outputs.
- Fixed plugin installation path mismatch: updated the `plugin`
  subcommand to install downloaded plugins directly to the shared
  configuration directory (`~/.gemini/config/`) rather than the private
  application data folder, making them instantly discoverable.
- Fixed Git short-hash support in diff selection: updated the commit
  hash recognition pattern in the /diff commit selection tree to match
  Git's standard 7-character short hashes (and up to 40-character full
  hashes).
- Fixed statusline subcommand handling and recursive loops: added
  case-insensitive subcommand parsing (help, delete, reset, enable/on,
  disable/off) to the /statusline command, providing direct control to
  toggle or revert custom statuslines and blocking recursive shell hangs
  during help queries.

Patches (0)

[1.0.1](https://antigravity.google/download#antigravity-cli "View release 1.0.1")  
January 1, 2026

### Sandbox auto-approvals, plugin discovery, and clipboard fixes

Added `proceed-in-sandbox` auto-approvals, automated plugin skill
discovery, and clipboard image reading.

Improvements (8)

- Added `proceed-in-sandbox` tool permission mode. Auto-approves
  terminal commands that run inside the secure sandbox, requesting
  manual approval only when a command attempts to bypass the sandbox.
- Integrates consumer/free-tier onboarding directly into the CLI.
- Added plugin discovery for skills and agents. Automatically scans
  installed plugin directories to make custom skills and specialized
  agents available for execution in the CLI.
- Moves the **terminal** color scheme to the top of the selection list,
  making it the default choice during onboarding and in `/settings`.
- Improved `/usage` and `/quota` commands. Forces a real-time reload of
  model configuration and remaining quotas, allowing you to see updated
  real-time consumption statistics immediately.
- Improved step rendering layout. Calculates available terminal width
  dynamically and uses middle-truncation (`/foo/.../bar`) for file path
  tools to prevent layout shifting on narrow screens.
- Improved session deletion keybinding in `/resume`. Changed the
  shortcut from `ctrl+d` to `ctrl+delete` to resolve conflicts with the
  global exit keybinding (`ctrl+d` `ctrl+d`) and preserve Emacs-style
  forward-delete in search input fields.
- Restored automatic table wrapping, preventing long cells inside
  markdown tables from being truncated.

Fixes (5)

- Fixed OAuth token persistence and authentication hangs.
- Fixed Windows log redirection and resizing issues. Resolved a critical
  bug where logs were not redirected correctly on Windows, which
  previously caused the terminal to swallow window resize events and
  shut down slowly.
- Fixed pasted text line counting. Corrected line counting for user
  inputs to ensure extremely long inputs are correctly folded into a
  `[Pasted text #X +Y lines]` placeholder to keep the viewport clean.
- Fixed onboarding stability. Resolved a race condition where a
  concurrent terminal resize event during onboarding could revert the UI
  to a blank onboarding screen.
- Resolved an issue where deleted files (represented by `+++ /dev/null`)
  had their deletion lines incorrectly merged into the previous file's
  diff.

Patches (0)

[1.0.0](https://antigravity.google/download#antigravity-cli "View release 1.0.0")  
January 1, 2026

### Initial release of Antigravity CLI

Initial public release of the Antigravity CLI.

Improvements (1)

- Initial release of the Antigravity CLI.

Fixes (0)

Patches (0)

[2.5.5](https://antigravity.google/releases?tab=ide&version=2.5.5 "View release 2.5.5")  
August 13, 2026

### Windows Media Attachments and Chat Responsiveness Improvements

Bug fixes addressing media attachment failures on Windows and improving
message responsiveness when interacting with the agent.

Improvements (0)

Fixes (2)

- Fixed an issue where the agent failed when attaching images or media
  files on Windows machines.
- Improved responsiveness and reduced hanging issues when sending
  messages to the agent.

Patches (0)

[2.5.2](https://antigravity.google/releases?tab=ide&version=2.5.2 "View release 2.5.2")  
August 12, 2026

### Queued Messages, File Attachments, and Performance Improvements

Support for queued messages, plain text, PDF, and markdown file
attachments, model thinking level selector, and major conversation
performance improvements.

Improvements (6)

- Improved the ask questions panel: clearer option selection, and
  continue now only activates once you've answered the current question.
- Added support for queued messages, including settings to configure
  execution behavior, a "Send Now" option, and an improved input card
  layout.
- Added support for attaching and rendering plain text (.txt), pdf
  (.pdf) and markdown (.md) files in conversations.
- Added a new built-in Antigravity Guide skill to provide helpful
  guidance when users ask about Antigravity.
- Improved the model selector dropdown to choose thinking levels
  per-model (Low, Medium, High).
- Updated icons for built-in slash commands and redesigned
  mentions/slash commands menu alignment in the chat input box.

Fixes (2)

- Opening long conversations is now dramatically faster.
- Reduced typing lag while the agent streams responses.

Patches (0)

[2.1.1](https://antigravity.google/releases?tab=ide&version=2.1.1 "View release 2.1.1")  
June 22, 2026

### Model quota screen and agent security fixes

New model quota screen, agent permission and security fixes, and MCP
stability improvements

Improvements (1)

- Quota screen redesign: clearer, unambiguous view into “used” versus
  “remaining” credits in the Models tab of the Settings screen.

Fixes (4)

- Fixed Agent Permission Issue: Fixed the bug where changes to the agent
  security mode were not persisted.
- Improved MCP server stability: increased resilience to unresponsive
  MCP servers and improved browser agent self-troubleshooting abilities
  when using the Chrome DevTools MCP server.
- MCP server schema compatibility: the mcp_config.json schema now
  accepts url in addition to serverUrl as a field.
- New entries in the sensitive paths list: .vscode and .cache are now
  recognized as sensitive paths that require explicit user confirmation
  before the agent can access them in strict mode.

Patches (0)

[2.0.4](https://antigravity.google/releases?tab=ide&version=2.0.4 "View release 2.0.4")  
June 2, 2026

### Enterprise Authentication Fix

Bug fixes

Improvements (0)

Fixes (1)

- Fixed the blank screen issue for Enterprise account authentication.

Patches (0)

[2.0.3](https://antigravity.google/releases?tab=ide&version=2.0.3 "View release 2.0.3")  
May 21, 2026

### Migration flow from Antigravity 1.0

Added a migration flow that allows users to import customizations from
Antigravity 1.0.

Improvements (1)

- Added a migration flow that allows users to import settings,
  extensions, and keybindings from Antigravity 1.0.

Fixes (0)

Patches (0)

[2.0.2](https://antigravity.google/releases?tab=ide&version=2.0.2 "View release 2.0.2")  
May 21, 2026

### Fix installation location

Fixed installation location when installed via Antigravity 1.0.

Improvements (0)

Fixes (1)

- Fixed installation location when installed via Antigravity 1.0.

Patches (0)

[2.0.1](https://antigravity.google/releases?tab=ide&version=2.0.1 "View release 2.0.1")  
May 19, 2026

### First Release

The first release of the Antigravity IDE.

Improvements (1)

- First release of the Antigravity IDE.

Fixes (0)

Patches (0)

[1.23.2](https://antigravity.google/releases?tab=ide&version=1.23.2 "View release 1.23.2")  
April 16, 2026

### Bug Fixes

Fixed bug that prevented MCP servers from loading and bug that prevented
accessing workspace-specific settings.

Improvements (0)

Fixes (2)

- Fixed bug that prevented MCP servers from loading
- Fixed bug that prevented accessing workspace-specific settings

Patches (0)

[1.22.2](https://antigravity.google/releases?tab=ide&version=1.22.2 "View release 1.22.2")  
April 7, 2026

### Agent Permissions

New unified [permissions system](https://antigravity.google/docs/cli/permissions) to control agent
actions.

Improvements (1)

- New agent permissions system accessible in settings.

Fixes (0)

Patches (0)

[1.21.9](https://antigravity.google/releases?tab=ide&version=1.21.9 "View release 1.21.9")  
March 30, 2026

### Onboarding Fix

Fixed bug that prevented new users from completing onboarding.

Improvements (0)

Fixes (1)

- Fixed bug that prevented new users from completing onboarding.

Patches (0)

[1.21.6](https://antigravity.google/releases?tab=ide&version=1.21.6 "View release 1.21.6")  
March 25, 2026

### Linux Sandboxing and MCP Improvements

Linux support for sandboxing, improved MCP authentication, and
deprecations in Manager.

Improvements (5)

- Linux support for sandboxing
- Simplified, condensed chat UI
- Added support for reading rules from AGENTS.md in addition to
  GEMINI.md
- One-click chat archival
- Improved sidebar design

Fixes (2)

- Improved MCP authentication
- Various layout and UX improvements in the agent manager

Patches (2)

- Deprecate follow along mode in Manager
- Deprecate playground feature in Manager

[1.20.6](https://antigravity.google/releases?tab=ide&version=1.20.6 "View release 1.20.6")  
March 17, 2026

### Fix for customizations creation

Fix for customizations creation.

Improvements (0)

Fixes (1)

- Fixed an issue where rules and workflows could not be created.

Patches (0)

[1.20.5](https://antigravity.google/releases?tab=ide&version=1.20.5 "View release 1.20.5")  
March 9, 2026

### Stability and UI improvements

Stability and UI improvements.

Improvements (3)

- Added support for reading rules from AGENTS.md in addition to
  GEMINI.md.
- Deprecated the Auto-continue setting, which is now enabled by default.
- Improved conversation load times, especially for long conversations.

Fixes (3)

- Fixed color contrast in Agent Manager terminals.
- Fixed an issue with cleaning up old SSH server instances.
- Fixed a bug in token accounting that could cause conversations to
  prematurely reach the maximum token limit.

Patches (1)

- Removed Command support.

[1.19.6](https://antigravity.google/releases?tab=ide&version=1.19.6 "View release 1.19.6")  
February 26, 2026

### Account Remediation Pathway

Introduced a formal remediation process for accounts suspended due to
Terms of Service violations. See our [official
announcement](https://x.com/antigravity/status/2027435365275967591).

Improvements (1)

- Account remediation UI for suspended users.

Fixes (0)

Patches (0)

[1.19.5](https://antigravity.google/releases?tab=ide&version=1.19.5 "View release 1.19.5")  
February 26, 2026

### Browser Fix

Stability and UI improvements.

Improvements (0)

Fixes (1)

- Fixed an issue from 1.19.4 that prevented the browser from launching.

Patches (0)

[1.19.4](https://antigravity.google/releases?tab=ide&version=1.19.4 "View release 1.19.4")  
February 25, 2026

### Stability and UI Improvements

Stability and UI improvements.

Improvements (2)

- Allow users to include screenshots in feedback reports.
- Nano Banana Pro 2 availability.

Fixes (0)

Patches (0)

[1.18.4](https://antigravity.google/releases?tab=ide&version=1.18.4 "View release 1.18.4")  
February 21, 2026

### Fix for Windows Auto-updater

Bug fixes and stability improvements.

Improvements (0)

Fixes (1)

- Fixed an issue where the Windows auto-updater fails to detect new
  releases. Please manually install the latest update if you are on
  version 1.16.5 or 1.18.3.

Patches (0)

[1.18.3](https://antigravity.google/releases?tab=ide&version=1.18.3 "View release 1.18.3")  
February 19, 2026

### Settings, Artifacts, and Stability

New settings screens for models and terminal integration, artifact
download support, and various stability and UI fixes across platforms.

Improvements (6)

- Gemini 3.1 Pro availability.
- Added Models screen to settings, providing more visibility into quota
  usage.
- Added a setting to enable or disable terminal integration.
- Support for downloading artifacts from the chat UI.
- Up/down arrow key navigation for input box history.
- Improved UI responsiveness for chat interactions, including creating
  conversations, sending messages, and reverting changes.

Fixes (3)

- Resolved an issue where external plugins could fail to load on macOS
  due to signing problems.
- Fixed certain artifact files not being recognized as artifacts and
  missing the 'Proceed' button in the chat UI on Windows.
- Fixed an issue where reverting could occasionally delete files edited
  by the agent.

Patches (0)

[1.16.5](https://antigravity.google/releases?tab=ide&version=1.16.5 "View release 1.16.5")  
February 3, 2026

### Bug Fixes

Various bug fixes and performance improvements.

Improvements (1)

- Speed up population of @-mention search results in the Agent Manager

Fixes (0)

Patches (1)

- Renamed Secure Mode to strict mode

[1.15.8](https://antigravity.google/releases?tab=ide&version=1.15.8 "View release 1.15.8")  
January 24, 2026

### Performance Improvements

Performance improvements for long conversations.

Improvements (0)

Fixes (0)

Patches (1)

- Fixes a bug with long conversations with the agent that caused
  performance issues.

[1.15.6](https://antigravity.google/releases?tab=ide&version=1.15.6 "View release 1.15.6")  
January 23, 2026

### Terminal Sandboxing

MacOS users can now execute agent terminal commands within [a
sandbox](https://antigravity.google/docs/cli/sandbox) to prevent damage to files outside the
workspace.

Improvements (1)

- Terminal commands can now be executed within a sandbox for MacOS
  users.

Fixes (0)

Patches (0)

[1.14.2](https://antigravity.google/releases?tab=ide&version=1.14.2 "View release 1.14.2")  
January 13, 2026

### Agent Skills

Introducing agent skills to Antigravity for enhanced customizability,
alongside tab model updates and new conversation settings.

Improvements (3)

- Agent Skills now available in Antigravity
- Updated tab model architecture.
- New Settings to allow disabling conversation history and knowledge.

Fixes (2)

- Resolved transparency issues across various UI components.
- Corrected overactive jump-to-bottom and autoscroll behavior in the
  chat client.

Patches (0)

[1.13.3](https://antigravity.google/releases?tab=ide&version=1.13.3 "View release 1.13.3")  
December 19, 2025

### Google Workspace Support

Higher, more frequently refreshed rate limits for Google Workspace AI
Ultra for Business subscribers.

Improvements (1)

- Higher, more frequently refreshed rate limits for Google Workspace AI
  Ultra for Business subscribers.

Fixes (0)

Patches (0)

[1.12.4](https://antigravity.google/releases?tab=ide&version=1.12.4 "View release 1.12.4")  
December 17, 2025

### Gemini 3 Flash

Support for Gemini 3 Flash in Antigravity.

Improvements (3)

- Support for Gemini 3 Flash.
- Native audio support for the agent.
- Performance improvements for Agent Manager and for long conversations
  in editor windows.

Fixes (0)

Patches (1)

- Switched default browser use model to Gemini 3 Flash.

[1.11.17](https://antigravity.google/releases?tab=ide&version=1.11.17 "View release 1.11.17")  
December 8, 2025

### Secure Mode and Security Fixes

Adding the [secure mode](https://antigravity.google/docs/cli/permissions) option, which enforces
certain settings to prevent the agent from autonomously running targeted
exploits and requires human review for all agent actions. Various
security fixes.

Improvements (1)

- Secure mode option.

Fixes (1)

- Various security fixes.

Patches (0)

[1.11.14](https://antigravity.google/releases?tab=ide&version=1.11.14 "View release 1.11.14")  
December 4, 2025

### Google One Support

Higher, more frequently refreshed rate limits for Google AI Pro and
Ultra subscribers.

Improvements (1)

- Google One integration.

Fixes (0)

Patches (0)

[1.11.9](https://antigravity.google/releases?tab=ide&version=1.11.9 "View release 1.11.9")  
November 26, 2025

### Stability and Bug Fixes

Bug fixes in the authentication flow.

Improvements (1)

- Added better error states for onboarding users during authentication
  flow.

Fixes (0)

Patches (0)

[1.11.5](https://antigravity.google/releases?tab=ide&version=1.11.5 "View release 1.11.5")  
November 20, 2025

### Nano Banana Pro

With Nano Banana Pro, our agents have gotten even better at generating
UI mockups, system diagrams, or relevant embeddable assets, all grounded
in your existing codebase and knowledge.

Improvements (1)

- Nano Banana Pro (incrementally rolling out)

Fixes (1)

- Agent can now create scratch directories if no workspaces are open.

Patches (1)

- Fixed an issue with the telemetry settings toggle on the settings
  page.

[1.11.3](https://antigravity.google/releases?tab=ide&version=1.11.3 "View release 1.11.3")  
November 18, 2025

### Launch Day Feedback

Fast hotfixes to address day one issues.

Improvements (0)

Fixes (1)

- Support for individuals with non-Latin alphabetic characters in their
  names.

Patches (1)

- Messaging to distinguish particular users hitting their user quota
  limit from all users hitting the global capacity limits.

[1.11.2](https://antigravity.google/releases?tab=ide&version=1.11.2 "View release 1.11.2")  
November 18, 2025

### Google Antigravity

The original launch of Google Antigravity, with a fully-featured
AI-powered IDE, new Agent Manager view, an integrated experience with
Chrome, broad variety of rich Artifacts, user feedback flows, knowledge
management, and much more. The vision for what development looks like in
an agent-first paradigm.

Improvements (1)

- Google Antigravity

Fixes (0)

Patches (0)

[0.1.12](https://antigravity.google/download#antigravity-sdk "View release 0.1.12")  
August 13, 2026

### Fix proto deserialization regression for ActionGenerateImage output paths

The 0.1.12 release fixes a client-side proto deserialization regression
in image generation result handling.

Improvements (0)

Fixes (1)

- **ActionGenerateImage Proto Skew**: Resolved client-side
  deserialization errors where the bundled binary emitted
  `ActionGenerateImage.output_path` while the shipped Python proto
  lacked the corresponding field, ensuring robust protobuf parsing
  across image generation steps.

Patches (0)

[0.1.11](https://antigravity.google/download#antigravity-sdk "View release 0.1.11")  
August 11, 2026

### Default Gemini 3.7 Flash upgrade, session budget limits, Vertex AI Express Mode, and autonomous behavior modes

The 0.1.11 release updates the default model to `gemini-3.7-flash`,
introduces session-level budget enforcement and turn termination stop
reasons, Vertex AI Express Mode authentication, and an autonomous agent
behavior setting. It also expands tool hook metadata, resolves string
annotation coercion for postponed evaluation, and improves MCP server
and subagent stability.

Improvements (6)

- **Default Model Upgrade to Gemini 3.7 Flash**: Upgraded the default
  inference model to `gemini-3.7-flash`.
- **Session Budget Enforcement & Stop Reasons**: Added `BudgetConfig` to
  define session-level usage limits (total tokens, turns, and cost) and
  `StopReason` enum (`BUDGET_EXCEEDED`, `TURN_LIMIT`, `USER_CANCELLED`,
  etc.) to inspect turn termination causes.
- **Vertex AI Express Mode Support**: Added native support for Express
  Mode authentication via `VertexEndpoint(api_key=...)` and
  `LocalAgentConfig(vertex=true, api_key=...)`, simplifying headless and
  non-GCP deployments.
- **Autonomous Agent Behavior Mode**: Control the agent's behavior with
  `AgentBehavior`. By default the SDK now has an
  `AgentBehavior.AUTONOMOUS` mode (previously
  `AgentBehavior.INTERACTIVE`) to streamline scripting, background, and
  headless interaction modes. Override by setting
  `CapabilitiesConfig(agent_behavior=AgentBehavior.INTERACTIVE)`.
- **Multi-Interface Hook Registration**: Enabled single-instance
  registration across multiple hook interfaces (`PreToolHook`,
  `PostToolHook`, `PreTurnHook`), allowing for cross-functional
  instrumentation without duplicate invocations.
- **PreToolArgs Metadata**: Exposed `trajectory_id` and `step_index` on
  tool hook payloads for chat thread context tracing.

Fixes (4)

- **ToolRunner String Annotation Coercion**: When using
  `from __future__ import annotations`, tool argument coercion failed on
  stringified types; resolved by resolving type annotations via
  `typing.get_type_hints` before type adaptation.
- **MCP Server Example Port Binding**: Ephemeral port race conditions in
  test and example server startup were resolved by binding directly to
  port 0.
- **Subagent Deadlock Prevention**: Handled subagent fatal errors in the
  localagent executor to prevent deadlocks when subagents terminate
  abnormally.
- **Empty Input Validation**: Added input validation to prevent SDK
  unresponsiveness on empty or whitespace-only prompts.

Patches (0)

[0.1.10](https://antigravity.google/download#antigravity-sdk "View release 0.1.10")  
August 4, 2026

### Prioritized inference tiers, real-time token usage streaming, and context-aware hook decorators

The 0.1.10 release introduces support for Gemini Prioritized Inference
service tiers, real-time token usage event streaming, stateful
context-aware hook decorators, and explicit tool call correlation IDs
across hook callbacks. It also standardizes default system instruction
merging behavior, fixes WebSocket compaction events, and expands local
Gemma model documentation.

Improvements (10)

- **Gemini Prioritized Inference Service Tier**: Configure agents to
  utilize Gemini Prioritized Inference service tiers for high-priority
  model execution with automated graceful fallback.
- **Tool Call ID Correlation in Lifecycle Hooks**: Inspect `call_id`
  attributes on tool executions, errors, and hooks to correlate
  multi-step tool invocations across lifecycle callbacks.
- **Context-Aware Hook Decorators**: Decorate hook handlers
  (`@hooks.pre_turn`, `@hooks.post_tool_call`, etc.) that optionally
  accept `HookContext` as a parameter to maintain state and share data
  across lifecycle callbacks.
- **ActionCompaction Event Emission & Hook**: Track context window
  compaction notifications over WebSockets and intercept them using
  `@hooks.on_compaction`.
- **Standardized System Instructions Strategy**: Plain string
  instructions default to appending to built-in instructions. To
  override and completely replace built-in instructions, pass
  `CustomSystemInstructions`.
- **Live Token Usage Reporting**: Introduced real-time `UsageUpdate`
  event streaming so token usage accumulates live during agent execution
  rather than delaying updates until state transitions.
- **Interactive CLI Spinner**: Updated CLI interactive loop spinner to
  list all active tool names when running concurrent tool calls (e.g.,
  `Running tools 'tool_a', 'tool_b'`).
- **Module Re-exports**: Re-exported `ReadUrlContentResult` and
  `SearchWebResult` in `connections.local` for uniform tool result
  access.
- **Local Gemma Model Documentation**: Added guides and tutorials for
  running agents locally with Gemma models using LiteRT and
  OpenAI-compatible endpoints.
- **LiteRT Token Output Limit**: Increased `max_output_tokens` default
  in LiteRT local server configuration from 8,192 to 16,384 tokens to
  prevent truncation during complex reasoning and generation tasks.

Fixes (2)

- **ActionCompaction Event Emission**: Fixed issue where compaction
  notifications were suppressed in external SDK releases, causing
  `@hooks.on_compaction` handlers and `conversation.compaction_indices`
  tracking to fail; compaction events now emit properly over WebSockets.
- **MCP Test Server Startup**: Fixed a race condition where the HTTP
  port was exposed before uvicorn server startup completed, which
  previously caused intermittent `ConnectionRefusedError` failures
  during test initialization.

Patches (0)

[0.1.9](https://antigravity.google/download#antigravity-sdk "View release 0.1.9")  
July 27, 2026

### Model-call retry/backoff configuration, LiteRT warm-up scaling, and tool execution error handling

Release 0.1.9 of the Google Antigravity Python SDK adds model-call
retry/backoff configuration, improves tool registration for custom
functions, adds missing `BuiltinTools` exports, fixes user audio payload
processing, and adds connection `DebugConfig` support along with
`ToolExecutionError` handling.

Improvements (5)

- **Model-Call Retry & Backoff Configuration**: Exposes configurable
  retry and backoff parameters in `LocalAgentConfig` for model calls.
- **LiteRT Warm-up Timeout Scaling**: Dynamically scales LiteRT engine
  warm-up timeouts based on context size and synchronizes warm-up
  cleanup.
- **Improved Tool Registration**: Enhances automatic tool registration
  and docstring parsing for custom Python functions.
- **BuiltinTools Typing Exports**: Exports `BuiltinTools` at top-level
  package and typing boundaries for subagent tool configuration.
- **Connection DebugConfig**: Adds base `DebugConfig` options for
  enhanced connection debugging and logging.

Fixes (2)

- **Audio Payload Processing**: Fixes processing for user audio payloads
  in interactive agent sessions.
- **ToolExecutionError Handling**: Introduces `ToolExecutionError` to
  SDK error types for explicit tool failure handling.

Patches (0)

[0.1.8](https://antigravity.google/download#antigravity-sdk "View release 0.1.8")  
July 21, 2026

### Gemini 3.6 Flash default model, custom subagent instructions, and Pydantic argument coercion

Release 0.1.8 of the Google Antigravity Python SDK updates the default
text model to Gemini 3.6 Flash, adds support for custom subagent
instructions, and implements automatic Pydantic argument coercion for
tool calls. It also brings defensive prompt sanitization, configurable
tool retries, and comprehensive stability improvements for local agent
execution.

Improvements (7)

- **Default Model Upgrade to Gemini 3.6 Flash**: Upgrades the default
  generative text model in the Python SDK to `gemini-3.6-flash`.
- **Pydantic TypeAdapter Tool Argument Coercion**: Automatically coerces
  stringified numbers, booleans, and nested models returned by LLMs into
  strict Python types declared in tool signatures.
- **Custom Subagent Instructions**: Adds capability to specify custom
  system instructions and allowlisted tool configurations for spawned
  subagents in multi-agent workflows.
- **Prompt Sanitization**: Strips null bytes (`\x00`) and non-printable
  control characters (`DEL`, `BEL`, `C1`) from incoming user prompts at
  the wire boundary to prevent HTTP 400 errors and terminal corruption.
- **Configurable Tool Retries**: Add `RetryConfig` definitions in local
  configuration to allow fine-tuned model and output retry policies.
- **Operating System Telemetry**: Populate client OS and version
  information in telemetry headers to improve diagnostic tracking.
- **Native Usage Accumulation**: Enable native addition (`+` and `+=`)
  operations on `UsageMetadata` instances.

Fixes (7)

- **Large Tool Output Handling**: Dynamic output truncation and removal
  of WebSocket frame limits to resolve connection termination on large
  tool responses.
- **Subagent Idle Synchronization**: Fix race conditions where multiple
  idle states could cause `receive_steps()` to hang indefinitely.
- **Custom Tool Policy Enforcement**: Ensure custom tools properly
  trigger pre-tool policy checks and cleanly report denials.
- **Media MIME Type Inference**: Raise `ValueError` instead of Pydantic
  validation failures when media MIME types cannot be inferred from file
  extensions.
- **LiteRT Log Noise**: Suppress verbose C++ LiteRT engine diagnostic
  output by default.
- **Policy Copy Idempotency**: Prevent duplicate workspace policy
  prepending during deep copies of `BaseLocalAgentConfig`.
- **Type Safety**: Enforce typed `types.Step` arguments in
  `OnCompactionHook`.

Patches (0)

[0.1.7](https://antigravity.google/download#antigravity-sdk "View release 0.1.7")  
July 14, 2026

### Multi-threaded state handling, subprocess isolation, extra high thinking, and local model MCP

Release 0.1.7 of the Google Antigravity Python SDK expands end-user
control over agent execution environments, strengthens concurrency
safety for stateful tools, and introduces deeper reasoning capabilities.
Key highlights include atomic multi-threaded state handling for tools
and hooks, customizable subprocess environment variable isolation,
support for an "extra_high" thinking severity level, and full Model
Context Protocol (MCP) and subagent support across local model backends.
This release also resolves interactive console prompt clobbering and
improves socket discovery under containerized setups.

Improvements (9)

- **Multi-threaded Hook & Tool State Handling**: Developers can now
  safely read and mutate shared context variables across multi-threaded
  tools (`asyncio.to_thread` or `ThreadPoolExecutor`) using atomic
  updates and thread locking:
- **Custom Subprocess Environment Variables**: Developers can now pass
  custom environment variables directly to isolated agent instances via
  `LocalAgentConfig`, avoiding pollution of the global parent
  environment:
- **Extra High Thinking Severity Support**: Developers can now configure
  an `"extra_high"` thinking severity level for complex reasoning tasks
  without needing to specify or override the model name:
- **MCP & Subagent Support for Local Models (LiteRT & OpenAI)**:
  Developers using local Gemma (`LiteRTAgentConfig`) or local
  OpenAI-compatible endpoints (`LocalOpenAIAgentConfig`, such as Ollama
  or LM Studio) can now directly configure subagents and register Model
  Context Protocol (MCP) servers, enabling full local multi-agent and
  MCP tool workflows.
- **Environment Hydration**: Hydrates GCP/Vertex parameters (project,
  location, routing) dynamically from standard GOOGLE_CLOUD environment
  variables when not explicitly passed during LocalAgentConfig
  initialization.
- **Default Image Generation Model Optimization**: Updated the default
  image generation model (`DEFAULT_IMAGE_GENERATION_MODEL`) to
  `"gemini-3.1-flash-lite-image"`. Previous default image models often
  took too long to run on average during standard agent execution loops;
  this lightweight model ensures dependable, high-speed image generation
  by default while remaining fully replaceable via explicit model
  configuration if higher fidelity is required.
- **Local WebSocket Connections**: Retries socket connections by
  resolving to both "localhost" and "127.0.0.1", ensuring dependable
  harness discovery under containerized setups.
- **Tool Runner Public Asyncness**: Preserves original asynchronous and
  synchronous execution interfaces of tools when accessed via
  ToolRunner.get_public_callable.
- **Config Inheritance and Gaps**: Cleaned up redundant base overrides
  in LocalAgentConfig and corrected Pydantic validation failures arising
  from None initialization variables.

Fixes (5)

- **Interactive Console Spinner Clobbering**: Fixed a bug in
  `run_interactive_loop` where background spinner animation frames
  (`⠼ Reasoning...`) continuously clobbered user confirmation prompts
  (`async_input` and `ASK_USER` policy checks) every 80ms. The active
  spinner is now explicitly cleared (`\r\033[K`) and paused when an
  interactive input prompt opens, keeping confirmation lines readable
  and resuming the spinner smoothly after input is submitted.
- **Duplicate Tool Call Events**: Fixed client rendering issues by
  filtering out custom tool events from StepUpdate payloads to prevent
  duplicate event dispatches.
- **SDK Idle Transitions**: Corrected premature agent shutdown
  situations by properly checking and clearing idling flags once a
  TrajectoryStateUpdate signals transition to running.
- **LiteRT Connection Engine**: Rectified local engine initialization
  bugs, including a missing protobuf import, a token constructor
  argument mismatch, and OpenAITool inheritance.
- **Shutdown Connection Handshake**: Added extra execution buffer to
  local connections during agent shutdown, ensuring safe persistent
  state storage actions.

Patches (0)

[0.1.6](https://antigravity.google/download#antigravity-sdk "View release 0.1.6")  
July 9, 2026

### LiteRT model configurations, dynamic GCP environment hydration, and performance updates

This release expands local execution capabilities by broadening support
for multimodal and web-connected tool workflows. Developers can now run
Gemma models locally using LiteRT or integrate with local
OpenAI-compatible APIs, natively return multimodal media from custom
tools, and leverage a more robust, streamlined lifecycle hooks framework
that is fully aligned with Model Context Protocol (MCP) workflows.

Improvements (4)

- **Local Model Connectivity**: Introduced `LiteRTAgentConfig` and
  `LiteRTConnectionStrategy` for LiteRT-LM (supporting local Gemma
  execution), `LocalOpenAIAgentConfig` and
  `LocalOpenAIConnectionStrategy` for OpenAI-compatible APIs (supporting
  Ollama and LM Studio), and a background loopback HTTP translation
  server.
- **Multimodal Tool Outputs**: Enabled custom tools to return media
  assets (`Image`, `Document`, `Audio`, `Video`) directly via a single
  tool response without needing separate follow-up turns
  (`supplemental_media`).
- **Built-in Web Fetch Tool**: Integrated the `read_url_content` tool
  end-to-end for fetching structured web content natively with the
  `ReadUrlContentResult` Pydantic model.
- **MCP String Prefix Modernization**: Decoupled tool calls and safety
  policy engines from legacy `"mcp_"` string synthesis, resolving name
  mismatch issues by utilizing explicit `server_name` attributes in tool
  evaluation.

Fixes (4)

- **Python 3.14 Compatibility**: Resolved namespace class conflicts and
  typing normalization issues in `agent.py` and `public_api_test.py`
  under Python 3.14 deferred annotations evaluation.
- **Vertex Validation Errors**: Cleared a misleading reference to API
  keys in `VertexEndpoint` validation error messages, limiting fields to
  project and location.
- **OTel Trace Warnings**: Resolved detached `contextvars` warnings and
  set-status race conditions by removing `use_span` context managers
  from Turn/Session hooks and checking span recording readiness.
- **Exception Wrapping Mapping**: Fixed `agent_middleware` integration
  check failures by making the example check for error message
  substrings instead of strict exception types.

Patches (0)

[0.1.5](https://antigravity.google/download#antigravity-sdk "View release 0.1.5")  
June 25, 2026

### Streaming tool calls, custom subagent delegates, and thread safety enhancements

This release introduces native OpenTelemetry tracing support,
declarative subagent configurations, improved type safety, and critical
robustness and compatibility updates.

Improvements (5)

- **OpenTelemetry Tracing Support**: Integrates OpenTelemetry tracing
  into the SDK to translate session, turn, step, and tool lifecycle
  events into standard GenAI-compliant semantic spans for advanced
  monitoring and performance debugging, with custom task-safe active
  span propagation for tool execution.
- **Declarative Subagent Configurations**: Added `SubagentConfig` and
  `SubagentCapabilities` in `types.py` to support constructing static
  subagents with declarative instructions and tools directly.
- **Type Safety in `AgentConfig`**: Type-annotated policies, hooks, and
  triggers parameters on `AgentConfig` and its subclasses to improve
  type safety and overall developer experience.
- **Lifecycle Hook Routing**: Shifted core orchestration of
  `OnSessionStartHook`, turn-level hooks (`PRE_TURN` and `POST_TURN`),
  and `OnSessionEndHook` to the connection layer, implementing the
  Python-side `HookRouter` for event routing.
- **Public API Cleanup**: Hid internal validation methods on media and
  error classes by prefixing them with an underscore
  (`_validate_mime_type` and `_from_pydantic` on validation errors).

Fixes (2)

- **Historical Step Absorption**: Ensured historical step absorption is
  properly drained during initialization to prevent persistence
  non-linearity issues in `Conversation`.
- **Python 3.14 Compatibility**: Resolved potential name shadowing in
  the `Conversation` class by renaming the top-level connection module
  import.

Patches (0)

[0.1.4](https://antigravity.google/download#antigravity-sdk "View release 0.1.4")  
June 18, 2026

### Async tool execution buffers, memory state persistence, and error handling improvements

This release introduces major architectural refactorings, public API
standardizations, and key new capabilities centered on centralizing
model configurations to natively support multi-model backends, exposing
a new built-in Google Web Search capability, enabling environment
variable passing for Model Context Protocol (MCP) servers, and
simplifying the agent initialization flow by removing dynamic runtime
registrations.

Improvements (10)

- **Built-in Web Search Tool**: Exposes the `SEARCH_WEB` tool directly
  within the SDK, enabling agents to leverage Google Search for grounded
  real-time information retrieval, complete with new developer examples
  (`web_tools.py`).
- **MCP Server Environment Variables**: Added support for configuring
  and passing custom environment variables to launched stdio servers via
  the new `env` field in `McpStdioServer`.
- **Base URL and HTTP Headers Support**: Out-of-the-box support for
  setting custom base URLs and HTTP headers.
- **Image Generation Aspect Ratio**: Updated the SDK model config and
  wrapper to support specifying `aspect_ratio` within the image creation
  tool configuration.
- **Centralized Multi-Model Configuration**: Replaces legacy singular
  `gemini_config` options with a unified, repeated `models` collection
  on `AgentConfig` and `LocalAgentConfig` to support multi-model
  routing, fallback strategies, and automated selection helpers.
- **Agent Session Lifecycle & API Standardization**: Improves runtime
  safety by removing dynamic post-initialization hook and trigger
  registration in favor of session creation-time declarations.
- **Top-Level Package Exports**: Exposed core SDK constructs (including
  `Content`, `Image`, `Document`, `Audio`, `Video`, `from_file`,
  `BuiltinTools`, and `SystemInstructions`) directly under the
  `google.antigravity` root module for easier access.
- **Hook Base Class Exports**: Consolidated the base hooks
  implementation by exporting `DecideHook`, `InspectHook`, and
  `TransformHook` from the hooks package root.
- **Top-Level Policy Package**: Created a new top-level policy package
  to clean up hook and workspace path validation dependencies.
- **Relocated Trigger Types**: Moved the `FileChange` model and
  `FileChangeKind` enum from `types.py` to the specialized triggers
  package.

Fixes (0)

Patches (0)

[0.1.3](https://antigravity.google/download#antigravity-sdk "View release 0.1.3")  
June 11, 2026

### Initial subagent orchestration and standard Google AI model targets

This release introduces per-server MCP timeout configurations and
improves local connection error handling.

Improvements (2)

- **Per-Server MCP Timeout**: Added configuration support to set custom
  timeouts (in seconds) for individual MCP servers
  (`BaseMcpServerConfig.timeout_seconds`).
- **Terminal Error Propagation**: The local connection now propagates
  terminal trajectory errors from the `localharness` binary as
  structured `AntigravityExecutionError` exceptions in the Python SDK
  during step collection.

Fixes (0)

Patches (0)

[0.1.2](https://antigravity.google/download#antigravity-sdk "View release 0.1.2")  
June 4, 2026

### Google Antigravity SDK release 0.1.2 updates

This release adds Windows platform support, introduces programmatic
turn-level stream cancellation, simplifies safety policy configurations
for the Model Context Protocol (MCP), and removes the deprecated MCP SSE
transport.

Improvements (3)

- **Windows Platform Support**: Native compatibility added for Windows
  x86_64 and ARM64 environments. Path and file URI resolution now
  correctly handles drive letters and directory separators under
  Windows.
- **Programmatic Turn-Level Cancellation**: Added programmatic stream
  cancellation via `ChatResponse.cancel()`. This programmatically aborts
  active generation turns directly from the client and raises
  `AntigravityCancelledError` (subclass of `asyncio.CancelledError`) to
  cleanly signal cancellation in the async flow
  (`examples/getting_started/cancellation.py`).
- **Direct MCP Safety Policy Configuration**: Overloaded `policy.allow`,
  `policy.deny`, and `policy.ask_user` to accept server configurations
  (`BaseMcpServerConfig`) directly instead of typing namespaced string
  paths. Policy evaluation follows a 9-level precedence model (Specific
  \> Prefix Wildcard \> Global Wildcard) with longest-match prefix
  validation to protect against collisions.

Fixes (0)

Patches (1)

- **Deprecation of SSE Transport**: Removed the legacy `McpSseServer`
  configuration and connection handlers in favor of standard Stdio and
  Streamable HTTP connection strategies.

[0.1.1](https://antigravity.google/download#antigravity-sdk "View release 0.1.1")  
May 29, 2026

### Google Antigravity SDK release 0.1.1 updates

This release focuses on significant enhancements to the Model Context
Protocol (MCP) integration, adds native Vertex AI authentication
support, improves robustness with better error handling, and includes
critical fixes for token usage tracking.

Improvements (4)

- **MCP Tool Filtering & Simplified Policies**: Added support for
  `enabled_tools` (allowlist) and `disabled_tools` (denylist) in server
  configurations, and overloaded safety policy helpers (`policy.allow`,
  `policy.deny`, `policy.ask_user`) to accept the MCP server
  configuration object directly.
- **Vertex AI Authentication**: Integrated native support for Vertex AI
  authentication in the Python SDK.
- **MCP Tool Prefixing & Validation**: The SDK now automatically
  namespaces and prefixes MCP tools (`mcp_{server_name}_{tool_name}`) to
  prevent name collisions when connecting multiple MCP servers. The
  `name` field is now mandatory in MCP server configurations and
  validated as a proper Python identifier.
- **Improved Error Handling**: The SDK now raises explicit, descriptive
  exceptions for terminal errors rather than failing silently.

Fixes (2)

- **Structured Output Token Tracking**: Fixed a bug where token
  `usage_metadata` was not correctly returned when structured output
  (`response.structured_output()`) was requested.
- **Type Checking Warnings**: Fixed `pytype` warnings (including
  `wrong-keyword-args` in `LocalAgentConfig`) across several modules.

Patches (0)
