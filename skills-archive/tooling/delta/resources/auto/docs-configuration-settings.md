---
title: "Settings"
url: "https://delta.dev/docs/configuration/settings"
description: "Configure Delta's appearance and behavior."
---

1.  Configuration
2.  Settings

# Settings

Configure Delta's appearance and behavior.

On this page

1.  [General](#general)
    1.  [Terminal](#terminal)
    2.  [Notifications](#notifications)
    3.  [Advanced](#advanced)
2.  [Appearance](#appearance)
3.  [Thread](#thread)
    1.  [Messages](#messages)
    2.  [Layout](#layout)
    3.  [Sharing](#sharing)
    4.  [Rules](#rules)
    5.  [.agents/prepare](#agentsprepare)
4.  [Subagents](#subagents)
    1.  [Agent profiles](#agent-profiles)
5.  [Pane](#pane)
    1.  [Files](#files)
    2.  [Changes](#changes)
6.  [LLM Providers](#llm-providers)
7.  [Cloud Runner](#cloud-runner)
8.  [Integrations](#integrations)
9.  [What’s next](#whats-next)

Open settings with `Cmd+,` on macOS or `Ctrl+,` on Linux and Windows. You can also use the **Settings…** menu item. Settings apply to the whole app, and there are no per-project settings.

Your settings follow your signed-in account across desktop and web. Delta keeps an encrypted local cache; use the Settings Editor instead of editing that cache. When an account has no saved settings, Delta imports the older `settings.json` from its configuration folder and leaves that file unchanged for older releases.

On desktop, you can also ask `@Delta` to change supported settings, such as your font size or subagent preferences. **Default Shell** and **Proxy** still need to be changed in the Settings Editor.

The names in parentheses below identify settings under `portable` or `native`. Desktop-only settings have no effect in the browser. The headings follow the pages in the Settings Editor.

Select a category to open its settings. To expand its sections in the sidebar, click its chevron or double-click the category. Select a section to jump to it. Searching settings filters these links along with the page content. With a category focused, press `Enter` or `Space` to move into its settings.

If the desktop app can’t save a change, the Settings Editor shows:

> Settings changed for this session, but Delta couldn’t save them. Check available storage and permissions, then edit a setting to retry.

Your changes remain active for the current session, but may not survive a restart.

## [General](#general)

### [Terminal](#terminal)

*   **Default Shell** (`native.default_shell`): the shell executable for interactive [terminals in threads](/docs/agents/terminals), for example `/opt/homebrew/bin/fish`. Unset by default, which uses your system shell; the field shows the detected shell as its placeholder. If the path is not executable, Delta falls back to the system shell. This setting does not affect the shell the agent uses for its own commands.

### [Notifications](#notifications)

*   **OS Notifications** (`native.os_notifications`): on by default. Delta sends a system notification when something needs your attention, such as an agent finishing while Delta is in the background. In a shared thread, completion notifications go only to the person who sent the prompt, on the device they sent it from.
*   **Agent Completion Sound** (`native.play_sound_when_agent_done`): off by default. Turn it on in **General > Notifications** to play a sound when a thread finishes or needs attention while Delta is in the background. It does not play while the Delta window is active.
*   **Keep Computer Awake** (`native.prevent_system_sleep`): on by default. Delta prevents idle system sleep while a thread is running and at least one Delta window is open. With all windows closed, your computer can sleep even if background work is still running.

### [Advanced](#advanced)

*   **Proxy** (`native.proxy`): routes Delta’s network requests through a proxy, for example `socks5h://localhost:10808`. HTTP and HTTPS `CONNECT` and SOCKS4, SOCKS4a, SOCKS5, and SOCKS5h proxies are supported, and the URL can carry credentials. Without this setting, Delta respects the standard proxy environment variables (`HTTPS_PROXY` and friends); the setting takes precedence over them, and hosts matched by `NO_PROXY` bypass both.

## [Appearance](#appearance)

Open **Settings > Appearance**.

*   **Appearance**: follow the system appearance, or always use light or dark mode.
*   **App Icon** (`portable.app_icon`): choose Dev, Trunk, Nightly, Midnight, Vermillion, or White for Delta’s interface. By default, Delta uses the icon for its release channel: White for the public release. This control is desktop-only. On macOS, changing it also updates the Dock icon immediately. Choose **Reset to Default** to restore your release channel’s icon.
*   **Dark Theme** (`portable.dark_theme`): the theme Delta uses in dark mode.
*   **Fonts**: choose independent font families and pixel sizes for **UI** (buttons, menus, and sidebars), **Prose** (conversations and rendered Markdown), and **Code** (source code, diffs, and terminal output). The size settings are `portable.ui_font_size`, `portable.prose_font_size`, and `portable.code_font_size_px`. **Font Ligatures** toggles font ligatures.
*   **Diff Colors** (`portable.diff_color_scheme`): the colors marking added and deleted lines in diffs. `green_red` is the default; `blue_orange` is a colorblind-friendly alternative. Themes supply the alternative palette, so the setting has no effect on themes that don’t declare one.
*   **Diff Markers** (`portable.diff_signs`): off by default. Turn it on to show `+` and `−` markers on added and deleted diff lines, so diffs don’t rely on color alone.

The menu bar also has a **Toggle Light/Dark Theme** action for switching appearance without opening settings.

## [Thread](#thread)

### [Messages](#messages)

*   **Send With Modifier** (`portable.send_message_with_modifier`): on by default, so sending takes `Cmd+Enter` on macOS or `Ctrl+Enter` on Linux and Windows, and `Enter` inserts a new line. Turn it off and `Enter` sends your message; right-clicking the send button also toggles it. If you were sending with `Enter` before this became the default, Delta shows a one-time tip above the send button.
*   **Cursor After Send** (`portable.cursor_after_send`): where the cursor lands after you send. `agent_response` (the default) puts it at the start of the agent’s response; `next_user_message` moves it to your next draft so you can keep typing.
*   **Collapse All Tools** (`portable.collapse_tool_activity`): off by default. Turn it on to collapse file edits and single actions too, including while the agent is working. Standard tool grouping stays on when this is off. Expand any summary to see its details.

### [Layout](#layout)

*   **Content Width** (`portable.thread_layout.width`): `default` uses Delta’s standard readable width for prose; `full` lets prose span the thread pane; `custom` uses a width you set by dragging the line-width ruler in a thread (**Toggle Line Width Ruler** in the thread’s context menu). Dragging the ruler saves that width here as the default.
*   **Constrain Wide Content** (`portable.thread_layout.constrain_wide_content`): off by default. Turn it on to keep code blocks, tool output, terminals, tables, and diagrams within the selected content width.
*   **Center Content** (`portable.thread_layout.centered`): off by default. Turn it on to center the content column in new threads.

Changing **Content Width** or **Center Content** updates the default for new threads. Per-thread layout choices are preserved. **Constrain Wide Content** also applies to existing threads.

Inline images fit the available line-wrap width while keeping their aspect ratio. Images smaller than that width stay at their natural size.

### [Sharing](#sharing)

*   **Default Visibility** (`portable.default_thread_visibility`): `private` is the default. Choose `organization` to share new threads automatically with everyone in your active organization.

### [Rules](#rules)

*   **Agent Thread Creation** (`portable.agent_thread_creation`): on by default. Turn it off to prevent agents from creating independent top-level desktop conversations. This does not disable subagent delegation.

Rules are standing instructions the agent follows without being asked. They come from three places, from broadest to most specific:

1.  **Personal rules** apply to every thread on this machine, across all your projects. The **Personal AGENTS.md** entry opens the file in Zed or lets the agent edit it for you. Delta checks `AGENT.md` before `AGENTS.md` and loads the first non-empty file it finds.
2.  **Project rules** live at the root of a repository. Delta recognizes `AGENT.md`, `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.rules`, `.cursorrules`, `.windsurfrules`, `.clinerules`, and `.github/copilot-instructions.md`, and loads the first non-empty one it finds. Project rules take precedence over personal rules when they conflict.
3.  **Folder rules** are `AGENT.md` or `AGENTS.md` files in subfolders. The agent receives them when it reads or edits files under that folder, from the outermost folder inward. More specific files override broader ones.

What you ask for in a message takes precedence over any rules file.

The personal rules folder depends on your operating system:

System

Folder

macOS

`~/.config/delta/`, then `~/Library/Application Support/delta/` as a compatibility fallback

Linux

`$XDG_CONFIG_HOME/delta/` or `~/.config/delta/`

Windows

`%APPDATA%\delta\`

`DELTA_CONFIG_DIR` overrides these locations. For reusable instructions the agent should load only for a particular kind of task, such as a release procedure, use [skills](/docs/agents/skills) instead of rules.

*   **Open in Delta Links** (`portable.pull_request_thread_links`): on by default. When the agent opens a pull request from a shared thread, it adds an “Open in Delta” link back to the thread, so reviewers can see how the change came about. The agent only adds links where the thread’s sharing scope lets recipients follow them; private threads never get one. Turn the setting off to keep thread links out of pull requests entirely.

### [.agents/prepare](#agentsprepare)

The **.agents/prepare** entry describes the setup script Delta runs in every managed checkout it creates. **Create with Delta** starts a thread in which the agent writes the script for your repository. See [Prepare your project](/docs/configuration/project-setup#run-setup-in-each-checkout) for a script example and guidance on installing dependencies per checkout.

To use a different script only in Delta, see [Delta-specific overrides](/docs/configuration/project-setup#delta-specific-overrides).

## [Subagents](#subagents)

These controls configure delegation and resource limits. See [Subagents](/docs/agents/subagents) for the workflow, built-in roles, and access to your files.

*   **Delegation** (`portable.subagent_delegation_level`): **Only When Asked** by default, so delegation requires your explicit request. **Max** lets the agent delegate when useful. **Off** removes the subagent tools. If you previously chose **Off** or **Only When Asked**, Delta keeps that choice.
*   **Max Agents Per Thread** (`portable.subagent_concurrency.maximum_per_parent`): four concurrent legacy attached subagents by default, excluding the parent agent.
*   **Max Agents Overall** (`portable.subagent_concurrency.maximum_total`): eight concurrent legacy attached subagents across the Delta process by default, excluding parent agents. Set either concurrency limit to zero to pause their work. These limits do not apply to new agent-created subthreads.
*   **Allow model overrides** (`portable.subagent_defaults.allow_parent_model_override`): on by default. Lets the parent agent choose a different model than the profile specifies.

### [Agent profiles](#agent-profiles)

Choose a model and thinking effort for each profile in **Subagents**. Provider-specific choices live in **LLM Providers > Model Preferences** and take precedence over the profile’s default model choice. See [Subagent profiles](/docs/agents/subagents#profiles) for the defaults.

Profiles are stored with your account settings. On desktop, choose **Configure with Delta** in the **Custom Profiles** row to open a conversation for creating or changing a profile. Describe its role, instructions, model, and whether it should use an isolated or shared worktree, then send the request.

Delta imports older TOML profiles once, preserving existing settings entries when they conflict. Editing those old files is no longer the configuration workflow. An import failure appears in the profiles settings and is retried on next launch.

Worker and Reviewer default to isolated worktrees; Scout defaults to shared. Changing the worktree mode changes file access, not which tools a profile can use. Profile changes apply to future subagent starts and turns. New profiles become available to the parent on its next turn.

## [Pane](#pane)

### [Files](#files)

*   **Line Numbers** (`portable.file_pane.line_numbers`) and **Soft Wrap** (`portable.file_pane.soft_wrap`): the defaults for how file tabs display code. Line numbers are on and soft wrap is off by default. Tabs follow these defaults until you toggle a setting within that tab; that choice then stays with the tab until you close it. **Soft Wrap** does not apply to the Changes tab.

### [Changes](#changes)

*   **Default Diff Base** (`portable.default_diff_base`): which comparison new threads show in the changes view. `branch` (the default) compares the Delta worktree against the recorded merge base with its upstream ref; `uncommitted` compares against its current head commit; `last_turn` compares against the Delta worktree’s state when the agent’s last turn started.

## [LLM Providers](#llm-providers)

Connect model providers with an API key or a subscription sign-in. Each connected provider shows its account details when available. **Model Preferences** lets you choose provider-specific subagent models and thinking effort. **Models Shown in Picker** controls which models appear in the model selector.

Add provider API keys in **Settings > LLM Providers**. Delta stores them in your account settings; keys from your environment are not used automatically. On desktop, **Import existing key** copies a detected key into settings. See [Models & Providers](/docs/agents/models-and-providers) for details.

## [Cloud Runner](#cloud-runner)

The **Cloud Runner** page appears in the desktop app when you have access to the [Cloud Runners](/docs/agents/cloud-runners) private beta.

*   **Preferred Runner Size**: **Medium** by default. Choose **Small**, **Medium**, or **Large** for runners you start after the change. See [The runner environment](/docs/agents/cloud-runners#the-runner-environment).
*   **Environment Variables**: add the variables your runners need, one at a time or with **Import** from a `.env` file. Values are stored in your encrypted account settings and aren’t shown again after you save them. See [Add environment variables and secrets](/docs/agents/cloud-runners#add-environment-variables-and-secrets).

## [Integrations](#integrations)

The **Integrations** page appears in the desktop app and in the browser with Cloud Runners access. Under **GitHub Account Access**, authorize your GitHub account so runners can push commits and open pull requests as you. See [Use Git and GitHub from a runner](/docs/agents/cloud-runners#use-git-and-github-from-a-runner).

## [What’s next](#whats-next)

*   [Keybindings](/docs/configuration/keybindings) covers Delta’s keyboard shortcuts.
*   [Models & Providers](/docs/agents/models-and-providers) covers model and provider configuration.
*   [MCP Servers](/docs/agents/mcp) covers connecting external tools