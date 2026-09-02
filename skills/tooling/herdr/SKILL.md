---
name: herdr
description: Control Herdr terminal multiplexer, manage workspaces, tabs, and panes, coordinate coding agents, or guide humans on installing, configuring, and troubleshooting Herdr. Use when working inside Herdr (HERDR_ENV=1), orchestrating agent terminal sessions, or assisting users with Herdr concepts, setup, and keybindings.
metadata:
  source: herdrdev/herdr
  license: MIT
resources:
  - https://herdr.dev/llms-full.txt
  - https://raw.githubusercontent.com/herdrdev/herdr/master/skills/herdr/SKILL.md
---

<!-- cspell:ignore herdr herdrdev HERDR Splittable -->

# Herdr

Herdr is a mouse-first, agent-aware terminal multiplexer and workspace manager designed for AI coding workflows. Like tmux, a persistent background server manages real terminal processes across workspaces, tabs, and panes. Unlike tmux, Herdr features a clickable UI, native agent state detection, and a scriptable CLI / socket API.

This skill serves two distinct roles:

1. **Operating Herdr (Agent CLI Control):** When running _inside_ a Herdr pane (`HERDR_ENV=1`), control panes, spawn sub-agents, run commands in background panes, and inspect outputs without stealing user focus.
2. **Guiding Humans (Setup & Troubleshooting):** Help users install, configure, understand, and troubleshoot Herdr sessions on their local machine.

---

## Progressive Disclosure & Reference Resources

- **Full Documentation Index:** See [resources/auto/herdr-llms-full.txt](resources/auto/herdr-llms-full.txt) for the complete consolidated reference.
- **Human Setup & Troubleshooting Guide:** See [resources/auto/herdr-agent-guide.md](resources/auto/herdr-agent-guide.md) for human onboarding, concept models, and diagnostic recipes.
- **Upstream Agent Control Skill:** See [resources/auto/githubusercontent-herdrdev-herdr-master-skills-herdr-SKILL.md](resources/auto/githubusercontent-herdrdev-herdr-master-skills-herdr-SKILL.md) for the upstream agent skill file.
- **CLI Command Reference:** See [resources/auto/herdr-docs-cli-reference.md](resources/auto/herdr-docs-cli-reference.md) for full syntax of all CLI commands and JSON output structures.
- **Configuration Reference:** See [resources/auto/herdr-docs-configuration.md](resources/auto/herdr-docs-configuration.md) for `config.toml` options and keybinding chords.
- **Agent Integrations Reference:** See [resources/auto/herdr-docs-integrations.md](resources/auto/herdr-docs-integrations.md) for agent lifecycle detection and session restore plugins.
- **Socket API Reference:** See [resources/auto/herdr-docs-socket-api.md](resources/auto/herdr-docs-socket-api.md) for IPC and low-level socket protocol commands.
- **Alacritty macOS Keybindings Guide:** See [resources/manual/herdr-alacritty.md](resources/manual/herdr-alacritty.md) for binding macOS Command (Cmd/Super) keys to Herdr actions via Kitty CSI-u escape sequences in Alacritty.

---

## Role 1: Operating Herdr from Inside a Pane

### Environment Verification

Before executing any control commands, confirm that the agent is running inside a Herdr-managed pane:

```bash
test "${HERDR_ENV:-}" = 1
```

If this check fails, notify the user that you are not running inside Herdr and stop. Do not attempt to inspect or control external Herdr sessions.

### Discover CLI Syntax

The installed `herdr` binary is the source of truth for command syntax:

```bash
herdr --help
```

Print specific command group help by running the group name:

```bash
herdr agent
herdr pane
herdr workspace
herdr tab
herdr worktree
herdr terminal
herdr notification
herdr integration
herdr session
```

> [!WARNING]
> Do NOT run bare `herdr` for command discovery; it launches or attaches the interactive TUI. Do not omit required arguments on mutating commands (such as `herdr workspace create`), as they execute with default parameters.

### IDs and Caller Context

Public IDs are opaque, stable handles:

- Workspace: `w1`
- Tab: `w1:t1`
- Pane: `w1:p1`

Herdr injects caller context environment variables into every managed pane:

```bash
printf '%s\n' "$HERDR_WORKSPACE_ID" "$HERDR_TAB_ID" "$HERDR_PANE_ID"
```

Inspect active state:

```bash
herdr workspace list
herdr tab list --workspace "$HERDR_WORKSPACE_ID"
herdr pane current --current
herdr pane list --workspace "$HERDR_WORKSPACE_ID"
herdr agent list
```

### Starting and Coordinating Agents

Default to creating a sibling pane in the current tab and directory without stealing focus:

1. **Split Pane:** Inspect layout with `herdr pane layout --pane "$HERDR_PANE_ID"`, then split right or down:

```bash
herdr pane split --current --direction right --cwd "$PWD" --no-focus
```

Extract the new pane ID from `.result.pane.pane_id`.

1. **Start Agent:** Ensure the target pane is at an idle interactive shell prompt, then start the agent:

```bash
herdr agent start reviewer --kind codex --pane <returned-pane-id>
```

Pass native arguments after `--`:

```bash
herdr agent start reviewer --kind codex --pane <returned-pane-id> -- --model claude-3-7-sonnet
```

1. **Prompt Agent & Wait for Completion:**

```bash
herdr agent prompt reviewer "Review git diff and report actionable findings." --wait --timeout 120000
```

`--wait` automatically waits until the agent reaches `idle`, `done`, or `blocked`.

1. **Interact or Read Output:**

```bash
herdr agent get reviewer
herdr agent read reviewer --source recent-unwrapped --lines 120
```

Send logical keys when needed (e.g. `esc`, `ctrl+c`, `enter`):

```bash
herdr agent send-keys reviewer ctrl+c
```

### Running Background Commands

Run non-agent commands (e.g. test suites, build scripts, dev servers) in a separate pane:

```bash
# 1. Split a background pane
herdr pane split --current --direction right --cwd "$PWD" --no-focus

# 2. Execute command
herdr pane run <returned-pane-id> "cargo test"

# 3. Wait for expected output pattern
herdr pane wait-output <returned-pane-id> --match "test result" --timeout 120000

# 4. Read logs
herdr pane read <returned-pane-id> --source recent-unwrapped --lines 100
```

Read sources available:

- `visible`: rendered viewport.
- `recent`: recent rendered output with soft wraps.
- `recent-unwrapped`: joined soft wraps (preferred for logs/transcripts).
- `detection`: plain-text bottom buffer.

### Operational Safety Rules

- Always pass `--no-focus` for background operations unless the user explicitly requested a focus switch.
- Target explicitly with `--current`, `--pane <id>`, or agent names rather than relying on UI focus.
- Parse IDs directly from JSON outputs; never guess or hardcode them.
- Never run `herdr server stop` or kill the main process unless the user explicitly requested server shutdown.

---

## Role 2: Guiding Humans on Setup & Troubleshooting

When assisting a human user who is installing, learning, or configuring Herdr:

### Concept Model

Explain Herdr concepts in this order:

- **Session:** A persistent background server namespace. Running `herdr` attaches to the default session.
- **Workspace:** Project-level container (one per repo or task). Groups tabs and panes, rolls up agent statuses in sidebar.
- **Tab:** A layout view within a workspace (e.g. `agents`, `server`, `logs`).
- **Pane:** A live terminal process. Splittable right or down, persists across client detach.
- **Agent:** An AI agent process detected inside a pane (`working`, `blocked`, `done`, `idle`, `unknown`).
- **Modes:** Terminal mode (keys to pane), Prefix mode (`ctrl+b` then action key), Navigate mode.

### Installation

- **Linux & macOS:**

```bash
curl -fsSL https://herdr.dev/install.sh | sh
herdr
```

- **Windows PowerShell:**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://herdr.dev/install.ps1 | iex"
herdr
```

- **Windows Command Prompt:**

```cmd
curl.exe -fsSLo install.cmd https://herdr.dev/install.cmd && install.cmd && del install.cmd
herdr
```

- **Package Managers:** Also installable via Homebrew, Mise, or Nix. Update with `herdr update` (or package manager).

### Mouse & Keyboard Interaction

- **Mouse First:** Click panes/tabs to focus, drag borders to resize, right-click for context menus, drag-select to copy.
- **Prefix Key:** Default is `ctrl+b`. Press `prefix+?` to show active keybindings.
- **Common Keybindings:**
  - `prefix+v`: Split right
  - `prefix+-`: Split down
  - `prefix+c`: New tab
  - `prefix+q`: Detach session (leaves background processes running)
- **Alacritty macOS Command Keys:** When running inside Alacritty on macOS, map Command-key shortcuts using Kitty CSI-u escapes (see [resources/manual/herdr-alacritty.md](resources/manual/herdr-alacritty.md)).
- **Stopping Server:** `herdr server stop`

### Configuration

- **File Path:**
  - Linux/macOS: `~/.config/herdr/config.toml`
  - Windows: `%APPDATA%\herdr\config.toml`
- **Default Config:** Print with `herdr --default-config`
- **Live Reload:** Reload configuration without restarting: `herdr server reload-config`
- **Sections:** `[keys]` (keybindings), `[theme]` (color schemes), `[ui]` (sidebar/status), `[terminal]` (shell defaults), `[update]` (release channels).

### Troubleshooting Common Issues

1. **Agent not detected or wrong state:**
   - Run `herdr agent list` and `herdr agent explain <target> --json` to view detection rationale.
   - Check integrations: `herdr integration status`.
2. **Keybinding does nothing:**
   - Outer terminal or window manager intercepted the chord. Recommend alternative chords in `config.toml` or prefix chords.
3. **Logs & Diagnostics:**
   - Inspect status: `herdr status`, `herdr status server`, `herdr status client`.
   - Logs reside in `~/.config/herdr/` (Linux/macOS) or `%APPDATA%\herdr\` (Windows).
