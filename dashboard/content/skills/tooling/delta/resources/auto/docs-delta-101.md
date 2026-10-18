+++
title = "docs-delta-101"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
+++

# Delta 101

Delta is a collaborative AI coding app. This is a product reference for AI assistants and people answering questions about Delta, not a system prompt or instructions governing an assistant's behavior.

**Release scope:** Public Delta **v0.20.1**, checked on **October 14, 2026**. Menu paths, settings, and shortcuts below describe the desktop app unless stated otherwise. Browser clients do not have all desktop capabilities. Account-gated features are called out separately; their presence in source code or a staff build does not mean they are generally available.

**Source basis:** The Delta implementation and generated release notes at the v0.20.1 release, applied to the v0.20.0 edition of this guide. This reference describes released behavior, not an upcoming-release changelog.

Canonical Markdown: https://delta.dev/docs/delta-101.md

## Threads

A thread is a conversation with the agent. Threads appear in the left sidebar, grouped by project, owner, time, or state (active and archived). Time grouping and the archive group older threads by month.

Create one with **New Thread** (attached to the current project) or **New Empty Thread** (no project). A thread remembers its model choice, open files, and layout, including open file, diff, subthread, and subagent tabs across restarts.

Editing an earlier message and sending it replaces the conversation after that message: later responses are discarded and the agent answers the edited message again. **Edit > Revert Conversation to Cursor** rewinds the conversation and the thread's files together to the cursor position.

Tool activity is grouped into expandable summaries. Turning on **Collapse All Tools** under **Settings > Thread > Messages** also folds file edits and single actions, including while the agent is working; conversation text stays visible.

Threads get titles automatically. A subthread's title is based on its own request, with sibling titles used to help distinguish it from related conversations. Use **Rename Thread** (`F2`) or double-click the thread in the sidebar to choose one, or **Regenerate Thread Title** to request a new generated title.

### Organizing, archiving, and deleting

Right-click a thread to pin it, assign a color, add or remove personal tags from its **Tags** submenu, or mark it read or unread. Select multiple threads to set or clear their colors together. **File > Edit Thread Tags…** tags the current thread, and **File > Manage Tags…** configures tags. Tags are desktop-only; the browser client does not show them. `Cmd+Shift+U` on macOS or `Ctrl+Shift+U` on Linux and Windows toggles the current thread's unread state.

**File > Toggle Thread Bookmark** (`Cmd+D` on macOS, `Ctrl+D` on Linux and Windows) bookmarks a thread so it can be found again, even after archiving. **File > Show Bookmarked Threads**, the sidebar's bookmarks button, or `Cmd+Option+B` on macOS lists bookmarked threads. The Bookmarks view's filter button, shown when hovering its header, offers the **Owned by**, **Subthreads**, and **Tags** filters described below.

To bookmark a particular sent user message instead, `Cmd`-click its marker in the thread's navigation rail on macOS, or `Ctrl`-click on Linux and Windows. Repeat to remove it. Message bookmarks persist after closing the thread and are shared with everyone in it; any participant can add or remove them. They are separate from thread bookmarks.

The sidebar header menus control grouping, filtering, and sorting. Threads can be sorted manually by dragging or by creation date. Projects can be hidden through the filter menu. Its **Owned by** section shows threads owned by **Anyone** or only **Me**, its **Subthreads** section chooses whether **Plain** subthreads, **Reviews**, **Land** subthreads, and **Subagents** appear, and its **Tags** submenu filters by tag. **View > Fold All Sidebar Groups** collapses visible groups. Collapsing a section or parent thread hides every thread inside it, including the open one; a collapsed section shows a running indicator while a thread inside it is working.

**Archive Thread** (`Cmd+W` on macOS, `Ctrl+W` on Linux and Windows) hides a thread from active threads without deleting it. Archiving the open thread switches to a neighbor. To find it again, group the sidebar by **State** to reveal **Archive**, or search with **Go to Thread**. Archiving a parent thread also archives its subthreads and cleans up their worktrees; reopening the parent restores the subthreads that were active before, but not ones archived individually. **Reopen Archived Thread** (`Cmd+Shift+T` or `Ctrl+Shift+T`) reopens the most recently archived thread.

Once the server holds a full copy, Delta removes an archived thread's local history three days after it was archived or last opened. Reopening then needs a connection to download it. **Delete Locally** in the thread's context menu removes only the copy on that machine, not already-synced server copies.

## Projects, worktrees, and checkouts

A **project** is a Git repository opened in Delta. A **Delta worktree** is that project's files and recorded history for one thread. A **checkout** is the folder on a machine containing those files.

Delta normally gives each thread a managed checkout so the agent can edit without changing your original project folder. A thread can instead use an existing checkout; edits then appear in that folder directly. Threads adopting the same existing checkout share one worktree and are not isolated.

Use **Add Project** to register a project, **Add Project to Thread** to attach an existing project, or **Open Git Project…** to register and attach a repository from disk. The repository must have a Git remote, and if it has several, one must be named `origin`; otherwise Delta does not add the project and shows an error explaining what to fix. Threads that attached such a repository before Delta required a remote cannot sync or be joined; reopening one, or trying to join it on desktop or in the browser, shows an error explaining why. A thread without projects shows an **Add Project** button in the status bar. A thread can attach multiple projects. The first is its primary project and determines its sidebar placement. Before a thread's first message is sent, dragging it onto a project header in the sidebar attaches that project if needed and makes it primary, keeping the draft.

Adding or attaching a project requires a configured Git remote, even in a private thread. Delta uses it to identify the repository. With one source remote, its name can be anything; with several, one must be named `origin`. Delta's managed-checkout `local` backlink is separate. If a repository has no remote, add one and push your branch, then retry. See [Git remote troubleshooting](/docs/troubleshooting#delta-says-my-project-needs-a-git-remote).

To tell projects apart, right-click a project's sidebar header and choose **Rename Project**. The name is a local display name on that device; Git metadata, checkout paths, and what the agent sees keep the repository's original name.

Managed checkouts live under `.delta/worktrees/` inside the repository, or in Delta's storage when joining a thread without that project. The project panel's **Reveal in Finder** on macOS or **Reveal in File Manager** on Linux and Windows locates the checkout.

**File > Open in Zed** opens the thread's checkouts in Zed and reveals the active file. The action is unavailable while Delta prepares the worktrees. Preparation failures show an error with **Retry**, rather than staying in a preparing state.

The status bar's **Project Settings** control switches the active project or branch and checks whether a branch is up to date, ahead, behind, or diverged from its configured remote branch. The project picker shortcut is `Cmd+Option+O` on macOS or `Ctrl+Alt+O` on Linux and Windows; the branch picker is `Cmd+Ctrl+B` or `Ctrl+Alt+B`.

The same control chooses where the thread works on a project: **Isolated Delta Worktree** (the default), **Existing Local Checkout**, or **Existing Git Worktree**, which lists linked Git worktrees already created with Git, an editor, or another agent. The choice locks once the thread's first message is sent. A new thread created from the current one keeps its selected checkout.

In the same menu, **Make default branch for Delta worktrees** sets the branch new Delta worktrees for that repository start from; once set, the row reads **Default for Delta worktrees** with the branch name and offers **Reset**. New Delta worktrees keep the repository's additional Git remotes, such as both a fork and its upstream.

Delta imports files Git does not ignore. Ignored `.env` files and build output stay on their original machine. Entirely untracked folders are skipped until something in them is added with `git add`.

### Repository configuration

`.agents/` holds project configuration shared with other tools. `.delta/` holds Delta-specific overrides.

- `.agents/prepare` is an executable setup script run in each new local managed checkout before the agent starts. On Windows, Delta runs it through Git Bash, so Git for Windows is required. Adopted checkouts skip it.
- `.agents/linked` lists repository-relative ignored files or folders to link into local managed checkouts.
- `.agents/setup` is for Cloud Runner setup, where that feature is available. It is not the local prepare script.
- `.agents/skills/\<name\>/SKILL.md` defines project skills.

For `prepare`, `linked`, and `setup`, Delta selects the corresponding `.delta/` override when present, otherwise the `.agents/` file. Selection is independent for each file; contents are not merged. An empty, invalid, or failing override does not fall back. **Settings > Thread > .agents/prepare** helps create the shared prepare script; overrides are edited directly.

Skills use different override rules, described under [Skills](#skills). Configuration needed in fresh checkouts belongs in the repository, but secrets and machine-local state do not.

### Bringing changes back

An adopted checkout already contains the agent's edits, so no transfer is needed.

For a managed checkout, the agent can commit changes and push the branch to a shared upstream remote, often named `origin`, for review and merge. Delta also provides a `local` remote pointing to the original project repository, so pushing a branch there makes it available without a fetch.

Pushing to `local` can target the branch checked out in that repository: a clean working tree updates in place, while Git rejects a push that would overwrite uncommitted edits. On a teammate's machine, `local` points to Delta's own clone rather than the teammate's project folder. The shared upstream and the `local` backlink serve different purposes.

## Execution environments and availability

Ordinary desktop turns run locally. Terminal commands, tools, files, credentials, and environment variables belong to the machine executing the turn.

**Cloud Runner is staff-only in v0.20.1**, not a generally available desktop feature. For accounts with access, a runner selector beside the send controls chooses **Local** or **Cloud** per turn. Cloud execution uses Delta-hosted models and shares the thread if it is not already shared. A cloud machine cannot read files or credentials that exist only on your computer.

**Cross-device account thread sync and connectivity indicators are also staff-gated in this release.** This reference does not describe them as standard public UI. Shared-thread collaboration is separate from the gated cross-device account thread-sync feature.

The browser can open shared threads, including review threads. Agents running in the browser can inspect the thread's code diffs when reviewing changes. Desktop shell access, local checkout management, native menus, and locally stored configuration are not promises about the browser client.

## Sharing and collaboration

**Share Thread** (`Cmd+Shift+S` on macOS, `Ctrl+Shift+S` on Linux and Windows), **File > Share Thread**, or the upper-right share icon opens sharing controls.

The owner chooses **Only people invited**, **Everyone in your organization**, or **Anyone with the link**. Only the owner can change access. The share popover copies the link and can invite up to ten people by email at a time; invitations expire after fourteen days.

**File > Join Thread**, also available in the command palette, accepts a join link or bare share ID. Delta creates checkouts for the shared worktrees without a manual clone, preferring an existing local checkout of the repository as the source and fetching from the same remote it uses, so a checkout that uses SSH is not asked for a GitHub username and password. If the thread's history cannot be downloaded, Delta shows an error with **Retry** instead of waiting indefinitely. If a repository fetch needs credentials, a modal identifies the repository and prompts for them, including SSH key passphrases and hardware-key PINs.

Each participant has an attributed draft. Sending submits all trailing drafts under their respective authors, and model usage is charged to the person who sends. Collaborators see the same conversation and can contribute while work continues.

To mention a teammate in a message or comment, type `@` followed by their username; the `@` menu's **Users** section, or a `user:` query, lists people. Mentioned people are notified in their **Inbox**, opened with the sidebar's inbox button or `Cmd+Shift+J` on macOS (`Ctrl+Shift+J` on Linux and Windows). The Inbox lists mentions and thread invitations, filters between **Unread** and **All**, and offers **Mark All As Read**. Delta also shows a system notification for mentions and invitations unless **Settings > General > Notifications > Mention and Invitation Notifications** is off. Clicking a notification for a mention in a file or diff comment opens that comment. On desktop, opening a thread however you reach it marks your mentions in it as read.

The browser client also has an Inbox, opened with `Alt+Shift+J` or the inbox button in the sidebar header on wider screens (the conversation header on narrow screens). It and its unread indicator update automatically. In the browser, a mention is marked read when opened from the Inbox or a notification, not when its thread is opened another way. Turning on **Settings > General > Notifications > Desktop notifications** in the browser shows desktop alerts for mentions and invitations while Delta Web is open.

Threads are private until shared. Sharing requires sign-in; projectless threads do not need a Git repository. Attached projects require Git remotes when added, not only when shared. If an older thread uses a project with no remote and Delta says it cannot sync or be joined, add a remote and continue in a new thread. A joining client may instead report that a project was never registered, which can also mean its registration failed; ask the owner to check the remote and share again, or continue in a new thread. A subthread relies on its parent's sharing: **Copy Share Link** stays disabled until the parent is shared.

Right-click text in a thread, a comment, a file, or a diff and choose **Copy Link to Selection**, or **Copy Link to This Position** without a selection, to share a link that opens collaborators directly on that location. In files and diffs, `Cmd+Option+L` on macOS or `Ctrl+Alt+L` on Linux and Windows copies the same link. A linked range stays highlighted until dismissed with **Exit Focus** or `Escape`, without changing the recipient's own selection. Following a link to a line in a file, an attachment, or a position in a thread briefly highlights the destination.

**View > Follow Next Collaborator** cycles through collaborators and follows their location across thread views, including into and out of subthreads, whether opened as tabs or as the primary thread. Following also opens images they view and mirrors their zoom and pan. Its shortcut is `Ctrl+Option+Cmd+F` on macOS or `Ctrl+Alt+Shift+F` on Linux and Windows.

## Sending messages and subthreads

By default, `Cmd+Enter` on macOS or `Ctrl+Enter` on Linux and Windows sends a message; `Enter` inserts a newline. Turn off **Send With Modifier** under **Settings > Thread > Messages**, or from the send button's context menu, to make `Enter` send.

Sending another message during generation stops the current response and immediately sends the new message. **Cancel Generation** stops output without sending.

In ordinary composer text, `Tab` indents to the next two-column stop and `Shift+Tab` outdents. Typing `:` followed by a name offers emoji and mathematical symbols outside code, math, and link targets.

Repositories can add PNG or SVG images directly under `.delta/emoji/` for custom `:name:` emoji. Names are case-sensitive and may contain ASCII letters, digits, `_`, and `-`. The first attached worktree wins duplicate names. Committed images are available to fresh and joined checkouts.

### Sending to a subthread

With **Send With Modifier** on, `Cmd+Shift+Enter` on macOS or `Ctrl+Shift+Enter` on Linux and Windows sends the message to a new subthread, also available as **File > Send Message to Subthread**. The subthread starts from an isolated copy of the parent's context and changes, and the agent can be asked to merge its changes back into the parent at any time; a one-time notice in new subthreads explains this. The agent can also merge selected worktrees from several related threads at once: a top-level thread, its subthreads, and their forks. Independently created threads cannot be merged this way. With the setting off, use `Cmd+Enter` or `Ctrl+Enter`. The same modifiers on the send button have the same effect; an unmodified click sends in the current thread.

With the cursor in a comment, in the composer or in a file or diff, the same subthread shortcut sends only that comment chain to a new subthread instead of the whole submission.

This works with ordinary messages and skill invocations, from a top-level thread or from inside a subthread, so subthreads can be nested; an attached subagent from an earlier release cannot create one. On desktop it uses local execution; in the browser it uses the browser execution runtime and requires a signed-in session and a loaded shared parent thread.

The submitted message remains read-only in the parent with a subthread badge and a **Route to subthread** row naming the child; clicking the name opens the child in a tab, which shows a loading indicator until the subthread is ready. Subthread event rows carry color-coded badges that distinguish reviews, landings, and other subthreads. Subthreads you create do not post progress, success, or failure updates to the parent; **Land Changes** results are the exception. A subthread an agent creates reports its final message to its immediate parent's agent when it finishes. The task runs in the child without replacing a newer parent draft or interrupting an existing parent response. The child's link remains available even if starting its task fails.

Opening a file or skill link from a subthread tab makes that child the primary thread, so the linked content comes from the child's version rather than the parent's. `Cmd+Option+U` on macOS or `Ctrl+Alt+U` on Linux and Windows returns from a primary subthread to its parent.

**File > View All Subthreads…** (`Cmd+J` on macOS, `Ctrl+J` on Linux and Windows) searches and opens the direct subthreads and subagents of the current parent, showing each one's state with the sidebar's icons. `Cmd+1` through `Cmd+5` (`Ctrl` on Linux and Windows) filter by all, reviews, land subthreads, plain subthreads, or subagents, and `Tab` shows or hides archived subthreads. The picker remembers its selected filter when reopened. It lists one level only: from inside a subthread it shows that subthread's siblings, and nested descendants do not appear. The browser client also offers **View All Subthreads**.

## Models and context

The composer shows the thread's selected model. Clicking it opens a searchable provider/model picker. Each thread remembers its choice.

**Settings > LLM Providers** connects ChatGPT, GitHub Copilot, or Grok subscriptions, or API keys for Anthropic, OpenAI, Google AI Studio, OpenRouter, OpenCode Zen, and OpenCode Go. Add or update API keys in settings; Delta does not use environment keys automatically. On desktop, **Import existing key** copies a detected key into settings once, and provider API keys sync through your Delta account. In the desktop app, **Add Provider** on that page configures an OpenAI- or Anthropic-compatible provider. In the browser, **LLM Providers** sits between **Appearance** and **Thread** and covers Delta-hosted model preferences along with your Zed account, subscription status, and subscription-management action.

For a compatible provider, enter its **Name**, base **API URL**, **API mode**, and an **API key** if required. Use the base URL, not a request endpoint such as `/chat/completions`; Delta adds the endpoint. Recognized names, such as **Ollama**, **LM Studio**, or **DeepSeek**, fill an empty URL and select the API mode. When adding a provider with a recognized URL, a paste button beside **API key** offers the matching environment variable's value if Delta detects one. This copies the value into the form rather than keeping a live environment reference.

**Custom headers** sets extra headers for the provider's requests. `{thread_id}` in a header value is replaced with the requesting thread's ID, and headers using it are omitted when there is no thread, such as during model discovery. For LiteLLM, xAI, and Baseten, Delta prefills an editable session header with `{thread_id}`. A model's **Thinking options** field takes comma-separated option names; OpenAI-compatible providers accept any unique names, and the first option is the default. For providers using the Anthropic API, each model also has a **Prompt caching** setting.

For local Ollama, start Ollama on your machine and enter **Ollama** to fill `http://localhost:11434/v1`. Delta attempts to discover models automatically and fills context limits and thinking options when Ollama reports them, including the allocated context size for running models. Check the discovered values, or use **Add Model** to enter them manually if discovery cannot supply them, then click **Save Provider**. These are desktop-local settings and services, not a promise of availability in the browser, on another machine, or in Cloud Runner.

Each provider's **Model Preferences** has a model-visibility switch labelled **All Shown**, **Some Hidden**, or **All Hidden**. Use it to show or hide all of that provider's current models in the model picker. This does not set visibility for future models.

The **ChatGPT Subscription** section shows the account's rate limits and reset times. The **OpenRouter** section's **Allowed Providers** limits which OpenRouter providers can serve requests, and **Provider Order** can prioritize **Highest throughput**.

The model selector offers **Thinking**, **Thinking Effort**, and fast mode where supported. Thinking choices are remembered per model in the thread. In a shared thread, a model unavailable through your providers is replaced by an equivalent choice or marked unavailable until another is selected.

### Compaction

Compaction summarizes older conversation content so a long thread can continue within the model's context window.

Click the circular context-usage indicator beside the send controls to open **Context Window**, inspect usage, toggle **Automatic Compaction**, and set **Compact at**. With automatic compaction enabled on a supporting model, the indicator fills toward that threshold and reports usage until compaction. Otherwise it reports usage against the model's context window.

Support depends on the provider and model. Some support automatic but not manual compaction. **Compact Now** in the command palette requests manual compaction and explains when the current model, platform, or thread state does not allow it.

## Subagents and profiles

A subagent is a subthread an agent creates to delegate a task. It runs as a regular subthread with its own transcript and status, so you can open it, send it messages to steer it, and continue it yourself. Subagents created by earlier releases remain attached agents with their own subagent view. Every subthread an agent creates is a subagent. Subagents can create their own subagents, so they can be nested, unless their profile disallows it; **Delegation** set to **Off** stops agents from creating them.

Subagents and subthreads appear together beneath their parent in the sidebar, at every nesting level, newest first, with six shown before **Show N more**. Clicking one opens it as the primary thread; `Cmd`-click on macOS or `Ctrl`-click on Linux and Windows opens it in a tab. Subthread rows also show an **Open in Tab** button on hover.

With a running subagent focused, `Escape` stops that subagent once dismissible UI, such as search or a selection, has been dismissed. If one is open, press `Escape` to dismiss it first. This works in a tab or as the primary thread and does not stop its parent or siblings.

Agents can send messages to other agent conversations to pass along findings or coordinate work. In a received agent-message row, click the sender's name to open its thread. On desktop, agents can also create independent top-level threads; turning off **Settings > Thread > Rules > Agent Thread Creation** prevents that without affecting subagents or threads you create yourself.

Delta ships three profiles:

- **Worker** implements work with full tool access.
- **Scout** gathers information from worktrees or the web.
- **Reviewer** checks work already present and reports problems.

Scout and Reviewer have terminal and web access but no file-editing tools. Lack of editing tools is not a sandbox: terminal commands can still have side effects.

Profiles choose `isolated` worktrees or the parent's `shared` live worktrees. Worker, Reviewer, and custom profiles default to isolated copies. As with other subthreads, isolated edits are not merged automatically and must be merged into the parent explicitly; only attached subagents from earlier releases merge them on successful completion. Scout defaults to shared worktrees. Shared commands and edits affect the parent immediately; stopping or failing does not undo them. Existing children retain the attachments chosen when they spawned.

**Settings > Subagents** controls **Delegation**, how readily agents hand work to subagents: **Off**, **Only When Asked** (the default), or **Max**, where higher levels use more tokens and time. It also sets the **Max Agents Per Thread** and **Max Agents Overall** limits, profiles, and **Allow model overrides**. Each profile row chooses a model and thinking effort, with **Reset to Default**. **Provider Overrides** links to **LLM Providers**, where each provider's **Model Preferences** can select a profile model for threads using that provider. Provider-specific choices win over the profile's own model, then built-in defaults: Worker uses the parent's model, Scout the provider's Scout model, and Reviewer its frontier model. In the browser, this page shows profiles without model controls or the custom-profile and provider-override rows.

### Custom profiles

Profiles are stored in Delta's settings, so they follow your account along with your other settings. There is no form for adding a profile: **Configure with Delta** in the **Custom Profiles** row opens a new thread with an unsent prompt asking @Delta to update your subagent profiles, and the desktop agent then edits them through its settings tool.

Profile TOML files from earlier releases, in `\<Delta config directory\>/profiles/`, are imported once and left unchanged. If any of them is invalid, none are imported and Settings explains why; fixing the files and restarting Delta retries the import.

A profile using a built-in ID extends that profile. A custom profile requires a `description` so the parent knows when to use it. `worktree` accepts `shared` or `isolated`. `prompt` adds instructions to the base prompt, and `system_prompt` replaces the base. `model` uses a `provider/model-id` selector, `thinking` sets the effort for that model, and `overrides` choose a model and effort per provider.

Tool capabilities are not configurable in profiles. Built-in profiles retain their fixed capabilities even with a replaced prompt: Worker subagents can create further subthreads, while Scout and Reviewer cannot. Custom profiles have ordinary parent tool access, including creating subthreads. Attached subagents from earlier releases cannot spawn anything further.

If the stored profiles are invalid, Delta falls back to the built-in profiles and Settings shows the error. An unavailable pinned model causes spawning to fail rather than silently using a different model. This reference does not promise custom profiles for Cloud Runner turns.

## Skills

A skill is reusable agent guidance in a `SKILL.md` file with `name` and `description` frontmatter. Project skills live in `.agents/skills/\<name\>/SKILL.md`; personal skills live in `~/.agents/skills/\<name\>/SKILL.md`.

A valid project skill in `.delta/skills` overrides an `.agents/skills` skill with the same frontmatter `name`, regardless of directory name. Other names remain available from both locations. Invalid definitions report load errors rather than hiding valid skills. Sending a message that invokes a skill that failed to load is refused with the file and the reason, rather than sending without the skill. Project skills win over personal skills with the same name. `.delta/skills` is project-only. The built-in `/create-skill` helps write or update a skill in the project or in `~/.agents/skills`.

Typing `/` in the composer lists invocable skills and built-in commands, including `/approve` and `/request-changes`. `user-invocable: false` hides a skill from this menu; `disable-model-invocation: true` means only a person can trigger it. `@Delta` asks about Delta itself. On desktop, when you ask, `@Delta` and other agents that can edit files can read and change many settings from the conversation. **Default Shell**, **Proxy**, and **Node Executable** remain for you to change in Settings.

Skills run in the current conversation. To make one eligible for the **Land Changes** button, its frontmatter includes:

```yaml
metadata:
  delta-action: land
```

Only lowercase `land` is supported. The field identifies eligibility, not an execution location; a skill's name or path alone does not make it eligible.

**Land Changes** (`Cmd+L` on macOS, `Alt+L` on Linux and Windows) opens a fresh Land subthread and leaves a clickable event in the parent without changing the parent's input or generation. It works from a top-level thread or from a subthread at any depth, including a subagent, and Land from a subthread covers everything in it, including changes inherited from its parent threads; it is unavailable inside Review and Land subthreads and attached subagents from earlier releases. With one eligible skill, the child runs it. With several, the agent lists them and asks for an explicit slash-menu invocation. There is no default remembered selection or Land chooser. With none, a setup skill offers to configure one; after the newly created project Land skill first lands successfully, Delta shows how to use it in the parent thread. Selecting a skill applies it in the existing Land subthread rather than creating another child. A skill file that fails to load stops Land only when it is, or might be, a Land skill.

The result appears as a card in both the Land subthread and its immediate parent, showing whether the change landed, with its target branch, commit, and CI status.

## Files, comments, and review

The right sidebar contains the file pane (files touched by the agent or opened by you) and the project panel (the worktree file tree). **Toggle Right Sidebar** and **Toggle Project Panel** control them. Editors support **Find**, **Find Next**, and **Find Previous**. Opening Find without a selection searches for the word at the cursor.

Markdown files open rendered. The pencil button after search in the file toolbar, labelled **Show Markdown Source** or **Show Markdown Preview**, switches that tab between source and the rendered view without losing edits or undo history.

Open an image file to view it in a zoomable workspace tab rather than as text. Use **Zoom In**, **Zoom Out**, or **Fit to View**, and drag the image to pan. In text files, drag the file-pane line numbers to select whole lines.

### Comments

Click or select agent output and type to draft a comment, or use **Edit > Comment on Selection** (`Cmd+Option+M` on macOS, `Ctrl+Alt+M` on Linux and Windows). With the cursor inside an existing comment, the same shortcut starts a reply.

In files and diffs, select code and type in Comment Mode, or right-click and choose **Comment**. The file toolbar's speech-bubble toggle switches between **Comment Mode On** (typing comments) and **Comment Mode Off** (typing edits the file).

`Cmd+Shift+M` or `Ctrl+Shift+M` reopens a collapsed gutter comment. **Toggle Delivered Comments** in the comment menu hides or shows submitted comments in Files and Changes; its shortcut is `Cmd+Option+V` or `Ctrl+Alt+V`.

Comments stay pending until the next message is sent, or can be sent on their own. In a shared thread, a send delivers everyone's pending comments, including ones a collaborator still has their cursor in. Collaborators and the agent can see and reply to them.

### Reviewing changes

**File > Review Changes** or the **Review Changes** action opens the current thread's changes (`Option+Shift+D` on macOS, `Alt+Shift+D` on Linux and Windows).

The Changes selector chooses the branch diff (labelled with its base, such as `origin/main`), **Last Commit** (uncommitted changes), **Last Turn** (since the last agent turn began), or an opened commit. **Settings > Pane > Default Diff Base** chooses the default for new threads.

The status bar's diff stat shows the line counts for the selected comparison, including zero changes since **Last Commit** or **Last Turn**; click it to review changes or right-click it to change the comparison. With more than one project attached, the same menu chooses **All Projects** or a single project to filter the counts and the review.

As you select code in **Review Changes**, symbol breadcrumbs beside the file name show its enclosing symbols when syntax information is available.

**Toggle Changes Panel Focus** focuses the changed-file list, which marks each file as added, modified, or deleted in the selected comparison and has a draggable scrollbar for long lists. Each file's **Seen** checkbox folds it until it changes again (`Cmd+Option+Y` or `Ctrl+Alt+Y`), and the Review header counts files marked as seen, such as `3/17`. The review's options menu offers **Collapse All Files** or **Expand All Files** (`Shift+Tab`) and **Mark All as Seen** or **Mark All as Unseen** (`Cmd+Shift+Y` on macOS, `Ctrl+Shift+Y` on Linux and Windows) for the files currently shown. `Cmd+[` and `Cmd+]` on macOS, or `Alt+[` and `Alt+]` on Linux and Windows, move between files.

**Create Review Thread**, `/review`, or `Cmd+Shift+R` on macOS (`Ctrl+Shift+R` elsewhere) opens a separate review conversation, from a top-level thread or from any subthread other than a Review or Land subthread; a review started in a subthread covers everything in it, including changes inherited from its parent threads. Comments and revision requests there do not interrupt the parent. `/approve` or `/request-changes` submits a verdict shown in the parent. Review threads can be shared and opened in the browser.

When an agent opens a pull request from a shared thread, an **Open in Delta** link can let reviewers open the conversation. **Settings > Thread > Rules > Open in Delta Links** controls this behavior.

## Terminal

**File > Open Terminal** or **Create Inline Terminal** opens an interactive shell in the thread's worktree. `Ctrl+Backtick` with the thread focused, or `!` at the beginning of an empty composer line, also opens one.

Shell fenced code blocks labelled `sh`, `bash`, `zsh`, or `ksh` (also `shell` or `shellscript`) offer **Prefill Terminal** beside **Copy Code**. Clicking it puts the code into an inline terminal without executing it. Typing `!` with the caret inside a supported block in a sent message or agent output also prefills it; this shortcut does not apply inside an editable draft. Check the command and press `Enter` to run it. Outside supported blocks and the empty-line shortcut, `!` is ordinary text.

**Enter Terminal** (`Enter`) focuses a selected terminal; **Leave Terminal** (`Escape`) leaves it when the shell is not consuming Escape. While focused, keystrokes go to the shell rather than the message composer.

In a local thread, `!` reuses one terminal per thread. Sending a message leaves a snapshot of the terminal in the sent message and keeps its process running, and the next `!` brings back the same session. The terminal's **Background Mode** icon or `Cmd+Option+Z` on macOS (`Ctrl+Alt+Z` elsewhere) moves it to the background so it keeps running separately, and the next `!` starts a new terminal. **Background Terminals** in the status bar lists running processes, with **Scroll to Terminal** and **Stop Process** for each and **Stop All**.

**Settings > General > Terminal > Default Shell** overrides the shell for local interactive terminals. An empty value uses the system shell.

File edits made through terminal scripts can appear as navigable diffs in the conversation's external-change reports. When a checkout's branch or commit changes, the expanded report shows the transition, with file-change counts on a separate line. Click a file path in the report, or press `Option+Enter` on macOS or `Alt+Enter` on Linux and Windows with the cursor on it, to open the affected file; deleted paths are not links. Delta uses nearby file-editing commands as a display hint, not proof of who made the change. Existing import exclusions still apply: ignored files and entirely untracked folders are not included. Binary files, lockfiles, and unreadable or oversized content can be reported by path only; large reports may omit diff detail.

## Navigation, shortcuts, and export

**Command Palette** opens with `Cmd+Shift+P` or `Cmd+K` on macOS, `Ctrl+Shift+P` or `Ctrl+K` on Linux and Windows. With a terminal focused, `Cmd+K` goes to the shell; the Shift+P shortcut still opens the palette.

The palette searches commands and threads. `Tab` switches to **Ask Delta** for natural-language command suggestions, and `Tab` again returns to search. `Ctrl+P` and `Ctrl+N`, or `Ctrl+K` and `Ctrl+J`, move through menus and pickers alongside the arrow keys.

`Option+Enter` on macOS or `Alt+Enter` on Linux and Windows opens the link under the cursor in a draft or agent output, with either send setting.

- **Go to Thread**: `Cmd+T` or `Ctrl+T`, including archived threads.
- **Previous Thread** / **Next Thread**: cycle in sidebar order.
- **Most Recent Thread**: `Ctrl+Tab` and `Ctrl+Shift+Tab` cycle recently viewed threads.
- **Toggle Sidebar**: collapse or reveal the thread list.
- **Focus Thread Editor**: focus the composer.
- **Toggle Thread/File Pane Focus**: `Cmd+?` or `Ctrl+?`.
- **View > Zoom In**, **Zoom Out**, **Reset Zoom**: adjust interface scale.
- **Help > Keymap** / **View Keymap**: `Cmd+/` or `Ctrl+/`, showing live bindings.

The Edit menu offers **Copy Thread Content** (`Cmd+Option+C` or `Ctrl+Alt+C`), **Copy Thread as JSON**, **Copy This Agent Response**, and **Copy Share Link** (`Cmd+Option+L` or `Ctrl+Alt+L`). Copy Thread Content gives readable conversation text; JSON gives structured replay data. Code blocks in threads show a **Copy Code** button on hover or while the cursor is inside them.

## Settings and account

**Settings** (`Cmd+,` on macOS, `Ctrl+,` on Linux and Windows) has searchable categories and keyboard navigation. Clicking a category selects it; its chevron, a double-click, or a search match expands its sections.

Public pages include **General**, **Appearance**, **Thread**, **Subagents**, **Pane**, and **LLM Providers**, plus **Skills** and **MCP** in the desktop app. They cover updates and terminal settings; themes, diff colors, and UI/prose/code fonts; message layout, sharing, and rules; delegation; skills; and file-pane preferences. **Settings > MCP** connects MCP servers so agents can use tools from your connected services; the browser has no MCP page. Number fields accept typed values. Additional pages and controls can be account-gated.

Application settings belong to the signed-in account. They sync between desktop and web and are cached on each device in encrypted form, so they are not meant to be edited by hand. An account without saved settings imports the older `settings.json` from Delta's config directory (relocated by `DELTA_CONFIG_DIR`), which is left unchanged for older releases. Settings are app-wide, unlike the status bar's Project Settings control for an attached checkout. If the settings key is missing from the system's credential store, Delta falls back to the cloud-synced settings.

**View > Toggle Light/Dark Theme** changes appearance without opening Settings. **Settings > Thread > Personal AGENTS.md** edits global rules: Delta loads the first non-empty of `~/.config/delta/AGENT.md` and `~/.config/delta/AGENTS.md`.

**Settings > Appearance > App Icon > Change Build Icon** overrides the icon for the current build. This section is desktop-only. On macOS, the Dock icon changes immediately.

On macOS, **Delta > Sign In** opens browser sign-in with a Zed (GitHub) account and saves the session in the system keychain; **Delta > Sign Out** signs out. Opening a workspace requires being online and signed in. **Check for Updates** and **About Delta** are in the same menu. **Create Organization**, in the user menu and the command palette, opens the Zed dashboard to create an organization. **Invite Members…** in the user menu invites people to the selected team organization; it is disabled when a personal organization is selected.

The `delta` command-line tool works with the Delta app from a terminal. On macOS, **Delta > Install CLI…** makes it available in terminals; the Windows installer adds it to PATH for new terminals; on Linux, `install.sh` installs it as `delta` and prints PATH instructions when needed. `delta auth status`, `delta auth login`, and `delta auth logout` report or change the account the app is signed in with. `delta org list`, `delta org current`, and `delta org switch` list your organizations, show the current selection, and switch it. `delta org create "My organization"` creates a team organization without selecting it, and `delta org invite` invites people by email to the current organization, or to another with `--org`. Running `delta` commands or another Delta build from Delta's built-in terminal uses that build's own data rather than the running app's.

When Delta is not already running, most CLI commands start it without a window, and that windowless app keeps running after the command finishes. Bare `delta`, `delta PATH_OR_URL`, and `delta open` instead start Delta with a window, or show a window in the app that is already running. `delta daemon status` reports whether the app is running, `delta daemon start` starts it without a window, and `delta daemon stop` shuts it down, including its windows. If Delta is already running after being started with a window, `delta daemon start` fails; quit Delta, then run it again.

How Delta was started decides whether it outlives its windows; there is no setting for this. An app started without a window, by `delta daemon start` or another CLI command, keeps running after windows opened in it close, until `delta daemon stop`. An app started with a window quits when its last window closes on Linux and Windows, so relaunching it after an update starts the new version; on macOS it stays running until quit. Closing a window can still stop that window's local agent work. **Settings > General > Notifications > Keep Computer Awake** prevents sleep only while a thread is running and a Delta window is open. This setting does not appear in the browser.

**What's New** shows release changes and **Restart Welcome Guide** starts the introduction again. They are in the macOS Help menu and the command palette.

## Privacy and troubleshooting

Delta replaces known secret values from environment variables, `.env` and mise files, and home-folder credential files with `[REDACTED]` before thread content is stored, synced, or sent to a model. Matching is exact-value based, not a guarantee that every secret will be detected.

**Send Feedback** or `/delta-feedback` opens the feedback form; on macOS it is also **Help > Send Feedback**. Attachments are opt-in, including a screenshot, recent logs, performance data, and conversation context/tool output. Conversation context is limited to 100 KiB of recent messages, system instructions, and tool activity, excluding reasoning and images.

**Open Log Viewer** (`Cmd+Shift+L` on macOS, `Ctrl+Shift+L` on Linux and Windows) shows logs. Files are also stored as `delta.log` and `delta.log.old` in Delta's data folder. Logs and screenshots may contain sensitive information and need review before sharing.

## Further documentation

- [Delta documentation](https://delta.dev/docs)
- [Documentation index for AI tools](https://delta.dev/docs/llms.txt)
- [Quick-start runbook](https://delta.dev/docs/start.md)


