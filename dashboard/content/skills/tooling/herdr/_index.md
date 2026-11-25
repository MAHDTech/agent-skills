+++
title = "herdr"
description = "Operate Herdr panes, workspaces, or agent sessions (requires HERDR_ENV=1) only when the user explicitly asks to use Herdr. Also use for Herdr setup, config, keybindings, and troubleshooting questions."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


<!-- cspell:ignore herdr herdrdev HERDR -->

# Herdr

Herdr is a terminal multiplexer for coding agents: a persistent server holds workspaces, tabs, and panes, recognizes agents running in panes, and exposes the session through the `herdr` CLI and a socket API.

Pick the role that matches the request, then read the matching upstream file before acting. The upstream files are the source of truth; the installed `herdr --help` wins when they disagree.

| Role                                                                         | Read first                                                                 |
| ---------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Control panes, tabs, workspaces, commands, or other agents from inside Herdr | `resources/auto/herdrdev-herdr-master-skills-herdr-skill.md`               |
| Help a human install, configure, learn, or troubleshoot Herdr                | `resources/auto/agent-guide.md`                                            |
| Look up CLI commands, `config.toml` keys, integrations, or the socket API    | `resources/auto/llms.txt` index, then `resources/auto/llms-full.txt`       |
| Bind macOS Command keys in Alacritty to Herdr actions                        | [resources/manual/herdr-alacritty.md](@/skills/tooling/herdr/resources/manual/herdr-alacritty.md) |

## Controlling Herdr

Run this gate before any control command:

```bash
test "${HERDR_ENV:-}" = 1
```

If it fails, tell the user you are not running inside a Herdr pane and stop; leave sessions outside your own pane untouched.

When it passes:

1. Learn syntax from `herdr --help` and from each group run without a subcommand (`herdr pane`, `herdr agent`, `herdr workspace`). Bare `herdr` attaches the TUI, and mutating commands such as `herdr workspace create` run with defaults when arguments are omitted, so never probe with them.
2. Read your own location from `$HERDR_WORKSPACE_ID`, `$HERDR_TAB_ID`, and `$HERDR_PANE_ID`.
3. Follow the upstream skill's recipes for splitting a pane, starting an agent (`herdr agent start <name> --kind <kind> --pane <pane-id> -- <agent-args...>`), prompting with `--wait`, running ordinary commands, and reading output.

Safety rules:

- Pass `--no-focus` for background work unless the user asked to switch focus.
- Target with `--current`, an explicit pane ID, or a unique agent name; read every ID from the JSON a command returns.
- Close only workspaces, tabs, panes, and sessions you created, unless the user asks otherwise.
- Stop the server (`herdr server stop`) or the main Herdr process only on an explicit user request; it kills every pane.
- Answer an agent's approval or question dialog (`blocked` state) only after asking the user.

## Guiding a human

Use the agent guide's concept model, install commands, configuration notes, and diagnosis recipes. Teach mouse interaction before keybindings, and answer with Herdr commands and `config.toml` keys rather than tmux equivalents. Before giving a keybinding, config key, or flag that the guide does not list, confirm it in `resources/auto/llms-full.txt` or `herdr --default-config`.

## Completion criteria

- Control tasks: the gate passed, every command targeted an ID or name taken from command output, and the result was read back (`herdr pane read`, `herdr agent get`, or `herdr agent read`).
- Guidance tasks: every command, key, and setting given to the user appears in the vendored docs or the installed CLI help.

