# Modes and Commands

The TUI has pager-local slash commands, plus a smaller set provided by `xai-grok-shell`. User-invocable skills also appear as slash commands.

In the TUI, `Shift+Tab` cycles session modes. For the full key reference, see [Keyboard Shortcuts](https://docs.x.ai/build/keyboard-shortcuts).

## Modes

### Plan

Plan mode is for planning first. When it is active, edits to the session plan file are auto-approved while writes to other files still require your approval.

Use it when you want Grok to sketch the approach before it starts making changes. Enter it with `/plan [description]` and view the current plan with `/view-plan`.

Plan mode keeps the working plan visible in the TUI.

It can also stop to ask a clarifying question before edits.

### Always-approve

Always-approve skips permission prompts for tool calls.

You can start in this mode with:

```bash customLanguage="bash"
grok --always-approve
```

You can also toggle it from the TUI with `/always-approve`.

### Permission mode in config.toml

Set the default permission behavior in `~/.grok/config.toml`:

```text
[ui]
permission_mode = "always-approve"
```

Use `permission_mode = "ask"` for prompts on each tool call, or `permission_mode = "always-approve"` to skip them. The default is `ask`. The legacy keys `approval_mode` and `yolo = true` are still accepted but `permission_mode` takes precedence.

Put this in `~/.grok/config.toml`, not project-scoped `.grok/config.toml`.

For the full set of `config.toml` options, see [Settings](https://docs.x.ai/build/settings).

## Core TUI commands

The command palette groups session, context, model, and tool actions.

Use `/context` to check current context usage.

| Command | What it does |
| ------- | ------------ |
| `/quit` (alias `/exit`) | Quit the application |
| `/help` | Browse commands and keyboard shortcuts |
| `/home` | Return to the welcome screen |
| `/new` (alias `/clear`) | Start a new session |
| `/resume` | Resume a previous session |
| `/sessions` | Switch, rename, or close active sessions |
| `/fork` | Branch the current session into a peer agent |
| `/rename <title>` (alias `/title`) | Rename the current session |
| `/share` | Share the current session via URL |
| `/session-info` | Show session info |
| `/context` | View context usage |
| `/compact [context]` | Compact conversation history |
| `/rewind` | Rewind to a previous turn |
| `/export` | Export the conversation to a file or clipboard |
| `/copy [N]` | Copy the last (or Nth-latest) response to the clipboard |
| `/find` | Search the conversation scrollback |
| `/transcript` | View the full transcript in your pager (`$PAGER`) |
| `/model <name>` (alias `/m`) | Switch the active model |
| `/effort` | Set reasoning effort for the current model |
| `/always-approve` | Toggle always-approve mode |
| `/plan [description]` | Enter plan mode |
| `/view-plan` | View the current plan |
| `/btw <question>` | Ask a side question without interrupting |
| `/loop [interval] <prompt>` | Run a prompt on a recurring interval — see [Background Tasks](https://docs.x.ai/build/features/background-tasks) |
| `/imagine <prompt>` | Generate an image from a text description |
| `/imagine-video <prompt>` | Generate a video from a text description |
| `/tasks` | List background tasks, subagents, and scheduled tasks |
| `/queue` | List the prompts queued behind the running turn |
| `/dashboard` | Open the [Agent Dashboard](https://docs.x.ai/build/features/dashboard) |
| `/settings` (alias `/config`) | Open the settings modal |
| `/theme [name]` (alias `/t`) | Switch the color theme |
| `/compact-mode` | Toggle denser UI layout |
| `/multiline` (alias `/ml`) | Toggle multiline input |
| `/vim-mode` | Toggle vim-style scrollback keybindings |
| `/timestamps` | Toggle message timestamps |
| `/terminal-setup` | Check terminal and clipboard setup |
| `/config-agents` (alias `/agents`) | Manage agent definitions |
| `/personas` | Manage personas |
| `/remember <note>` | Save a memory note |
| `/import-claude` | Open the Claude settings import modal |
| `/feedback [text]` | Send feedback about the current session |
| `/release-notes` (alias `/changelog`) | View release notes for the current version |
| `/usage` | View credit usage or manage billing |
| `/privacy` | Show or toggle privacy and data-retention status |
| `/login`, `/logout` | Sign in, or sign out of the current account |
| `/hooks` | Open the unified extensions modal at the Hooks tab |
| `/plugins` | Open the unified extensions modal at the Plugins tab |
| `/marketplace` | Open the unified extensions modal at the Marketplace tab |
| `/skills` | Open the unified extensions modal at the Skills tab |
| `/mcps` | Open the unified extensions modal at the MCP tab |

`/hooks`, `/plugins`, `/marketplace`, `/skills`, and `/mcps` all open the same extensions modal — they just pre-select a tab. A few commands appear only when their feature is available (for example `/imagine` and `/loop`).

## Shell-provided commands

| Command | What it does |
| ------- | ------------ |
| `/flush` | Flush conversation memory to disk now |
| `/memory` (alias `/mem`) | Browse, view, and manage your memories |
| `/dream` | Run memory consolidation |

These appear when cross-session memory is enabled.

## Skills as commands

Any user-invocable skill can also appear as a slash command, for example `/<skill-name>`.

If names collide, use the qualified form, such as `/local:commit`.