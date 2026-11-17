# Keyboard Shortcuts

Press `Ctrl+.` (or `Ctrl+X` on Windows and in terminals without the Kitty keyboard protocol) to open this list inside the TUI; entries that do not apply in the current context are dimmed.

The tables give each key's base action; see [While a turn is running](#while-a-turn-is-running) for what changes mid-turn, and [Terminal differences](#terminal-differences) for chords that differ by terminal.

## Essentials

| Keys | Action |
| ---- | ------ |
| `Enter` | Send the prompt |
| `Tab` | Move focus between prompt and scrollback |
| `Esc` | Close a menu, overlay, or search |
| `Esc Esc` | Clear the prompt, or open rewind when it is empty |
| `Ctrl+C` | Cancel the running turn. When the prompt is focused and has text, the first press clears the text instead |
| `Shift+Tab` | Cycle mode (Normal / Plan / Auto when available / Always-approve) |
| `Ctrl+P`, or `?` outside the prompt | Command palette |
| `Ctrl+.` / `Ctrl+X` | Keyboard shortcuts |
| `F2` or `Ctrl+,` | Settings |
| `Ctrl+Q` / `Ctrl+D` | Quit (press twice) |

## While a turn is running

* `Enter` queues the prompt. With `follow_up_behavior` set to `steer` instead of the default `queue` (under `[ui]` in `config.toml`, or Follow-up behavior in `/settings`):
  * The turn picks up the prompt at its next tool or model step.
  * While the turn waits on a subagent or a background command and nothing else is queued, `Enter` acts as send now instead.
* Send now (`Ctrl+Enter` or `Ctrl+I`; see [Terminal differences](#terminal-differences)) cancels the turn and sends the prompt.
  * With send now or `Enter`, a built-in slash command does not cancel the turn: most run right away, and those the agent runs, such as `/compact`, are queued. Send now on `/goal`, `/deep-research`, a skill such as `/create-workflow`, or a saved workflow run as `/\<name\>` cancels the turn like any other prompt.
  * If the prompt is empty and a follow-up is queued, `Enter` or send now cancels the turn and sends the top one. If the top one is `/compact`, `/flush`, `/dream`, `/loop` or another follow-up that can't be sent now, the turn keeps running and that follow-up waits for it to end. With nothing queued, neither cancels the turn.
  * During `/goal`, the prompt or follow-up steers the goal instead of cancelling the turn.
* `Esc` never cancels the turn; it shows a hint to press `Ctrl+C`. `Esc Esc` does not clear the prompt or open rewind.

## Input

| Keys | Action |
| ---- | ------ |
| `Ctrl+Enter` or `Ctrl+I` | Send now, while a turn is running |
| `Shift+Enter` | Newline — or send, in multiline mode (`Alt+Enter` where `Shift+Enter` is unsupported) |
| `Ctrl+M` | Toggle multiline input |
| `Ctrl+S` or `Alt+S` | Stash the prompt, or restore the stash on an empty prompt |
| `↑` | On an empty prompt, focus the prompt queue if it has entries, or browse prompt history if it has none. Search the history with `/history` |
| `!` | Shell mode, on an empty prompt |

## Scrollback

Focus the scrollback with `Tab`, then navigate. Letter keys, with or without `Shift`, require vim mode (`/vim-mode`, or `vim_mode = true` under `[ui]` in `config.toml`); `Ctrl` chords and the arrow-key equivalents always work.

| Keys | Action |
| ---- | ------ |
| `j` / `↓`, `k` / `↑` | Select next / previous entry |
| `Shift+L` / `Shift+→`, `Shift+H` / `Shift+←` | Next / previous turn |
| `Shift+J`, `Shift+K` | Next / previous turn, counted from the top of the view rather than the selection |
| `g`, `Shift+G` | Go to top / bottom |
| `Ctrl+U`, `Ctrl+D` | Scroll half page up / down |
| `Page Up`, `Page Down` | Scroll one page up / down |
| `h` / `←`, `l` / `→` | Collapse / expand the selected entry |
| `e`, `Shift+E` | Expand or collapse one entry / all entries |
| `Ctrl+E` | Toggle all thinking blocks |
| `r` | Toggle raw markdown |
| `y`, `Shift+Y` | Copy content / copy command or path |
| `Enter` or `Ctrl+F` | Open the selected block in the fullscreen viewer |
| `/` | Search scrollback (vim mode) |
| `x` | Kill the selected background task |

## Panels and session

| Keys | Action |
| ---- | ------ |
| `Ctrl+T` | Toggle the [todo pane](/build/features/sessions#todos) (agent screen) |
| `Ctrl+B` | Send the running command to the background |
| `Ctrl+;` or `Ctrl+'` | Toggle prompt queue |
| `Ctrl+R` | Open sessions (same as `/resume`). When the scrollback is focused and `[ui] mouse_reporting_toggle` is on, it toggles mouse capture instead |
| `Ctrl+L` | Open extensions |
| `Ctrl+G` | Toggle the [tasks pane](/build/features/background-tasks) |
| `Ctrl+O` | Toggle always-approve |
| `Ctrl+N` | New session (press twice) |
| `Ctrl+M` | Pick model, when the prompt is not focused |
| `Ctrl+\` | Open the [Agent Dashboard](/build/features/dashboard) |

## Terminal differences

* VS Code-family terminals (VS Code, Cursor, Windsurf, Zed):
  * Quit is `Ctrl+D` only.
  * Send now is `Ctrl+L`.
  * `Ctrl+L` does not open extensions; use `/plugins`.
  * Half-page down is `Shift+D`, in vim mode only.
  * On a local Mac, `Ctrl+;` and `Ctrl+'` often don't arrive, so `Ctrl+4` toggles the prompt queue. `Ctrl+\` arrives as `Ctrl+4` and toggles it as well, so open the Agent Dashboard with `/dashboard`.
  * Use `Alt+Enter` for newlines.
* Apple Terminal:
  * Send now is `Ctrl+O` in the prompt and the prompt queue, since the terminal cannot send `Ctrl+Enter` or `Ctrl+I` as distinct keys.
  * `Ctrl+O` toggles always-approve in other panes, and in the prompt when there is nothing to send now: no turn is running, or the prompt and the prompt queue are both empty. In the prompt queue with no turn running, it does nothing.
* WezTerm needs `enable_kitty_keyboard = true` for `Ctrl+Enter` and `Shift+Enter`.

See [Terminal Support](/build/cli/terminal-support) for fixes and diagnostics.

