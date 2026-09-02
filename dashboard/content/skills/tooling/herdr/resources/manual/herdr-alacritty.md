+++
title = "herdr-alacritty"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "herdr"
+++

<!-- cspell:ignore herdr herdrdev alacritty codepoint termios tcgetattr tcsetattr TCSADRAIN fileno tomllib unshifted setraw -->

# Alacritty to Herdr Command-Key Bindings

## The Core Problem

- Herdr is a terminal multiplexer running inside Alacritty (`program = "/opt/homebrew/bin/herdr"` or launched interactively).
- On macOS in legacy keyboard mode, Alacritty does not emit byte sequences for `Command+<letter>`, so a Herdr binding written as `cmd+t` never fires because Herdr never receives any input bytes.
- Herdr decodes modifiers (Alt, Super/Cmd) via the kitty keyboard protocol in the CSI-u encoding. It does not decode the legacy ESC-prefixed Alt encoding, and it does not recognize F13-style `ESC[25~` escapes as function keys in this configuration.
- While Herdr reliably decodes a plain `ctrl+<letter>` (a single control byte), binding that steals the key from the interactive shell (see Rejected Approaches).

---

## The Correct Technique: Kitty CSI-u Sequences

Configure Alacritty to emit the kitty keyboard protocol CSI-u sequence carrying the Super (Cmd) modifier, then bind the Herdr action to `cmd+<letter>`. Herdr parses the CSI-u sequence and matches the Super modifier. The CSI-u bytes are distinct from the plain control byte, keeping the shell's own `ctrl+<letter>` completely untouched without collisions.

1. In Herdr's `config.toml` `[keys]`, bind the action to `cmd+<letter>`.
2. In Alacritty's `alacritty.toml`, bind `Command+<letter>` to send the CSI-u sequence via `chars`.
3. Reload both configurations.

### CSI-u Sequence Format

- Format: `ESC [ <codepoint> ; <modifiers> u`, represented in `chars` as `"\u001b[<codepoint>;<modifiers>u"`.
- `<codepoint>` is the unshifted key's Unicode code point in decimal.
  - Lowercase letters: `a`=97 through `z`=122 (formula: `96 + alphabet_position`).
  - For example: `t` = 116, `w` = 119.
- `<modifiers>` = `1 + sum of active modifier bits`:
  - Shift = 1
  - Alt = 2
  - Ctrl = 4
  - Super (Cmd) = 8
  - Modifier combinations:
    - Cmd alone: `1 + 8 = 9`
    - Cmd + Shift: `1 + 8 + 1 = 10`
    - Cmd + Alt: `1 + 8 + 2 = 11`
    - Cmd + Ctrl: `1 + 8 + 4 = 13`
- Resulting sequences:
  - `Command + T`: `\u001b[116;9u`
  - `Command + W`: `\u001b[119;9u`

---

## Worked Example: Command+T (New Workspace) & Command+W (Close Workspace)

### 1. Herdr Configuration (`~/.config/herdr/config.toml`)

```toml
[keys]
# Command + T creates a new workspace (equivalent to the sidebar "new" button).
# Alacritty sends the kitty CSI-u sequence (\u001b[116;9u) so Herdr receives the Super modifier.
new_workspace = "cmd+t"

# Command + W closes the active workspace.
close_workspace = "cmd+w"
```

### 2. Alacritty Configuration (`~/.config/alacritty/alacritty.toml`)

```toml
[keyboard]
bindings = [
  # Command + T sends kitty CSI-u cmd+t (\u001b[116;9u); plain ctrl+t stays with the shell.
  { key = "T", mods = "Command", chars = "\u001b[116;9u" },

  # Command + W sends kitty CSI-u cmd+w (\u001b[119;9u); plain ctrl+w stays with the shell.
  { key = "W", mods = "Command", chars = "\u001b[119;9u" },
]
```

Reload both configs:
- Herdr: run `herdr server reload-config`.
- Alacritty: saves reload automatically (restart Alacritty if a `chars` change does not take effect).

---

## Rejected Approaches (Why Naive Setups Fail)

- **`cmd+t` in Herdr without Alacritty bindings:** Alacritty emits no bytes for Cmd+letter in legacy mode, so Herdr never receives an event.
- **Control-byte bridging (`Command+T` emits `\u0014` / Ctrl+T, Herdr binds `ctrl+t`):** Herdr intercepts that control byte globally, preventing the underlying shell from receiving standard shortcuts (such as `ctrl+w` to delete a word, `ctrl+u` to clear a line, or `ctrl+a` to jump to beginning). CSI-u avoids this by using unambiguous escape sequences.
- **F13 escape sequences (`ESC[25~`) or ESC-prefixed Alt (`\u001b\u0014`):** Herdr does not decode these legacy terminal encodings for keybindings.

---

## Configuration File Locations

- **Alacritty:** `~/.config/alacritty/alacritty.toml` under `[keyboard].bindings`.
- **Herdr:** `~/.config/herdr/config.toml` (Linux/macOS) or `%APPDATA%\herdr\config.toml` (Windows) under `[keys]`.

---

## Reloading and Validating

- **Herdr:** `herdr server reload-config` applies keybinding changes immediately without restarting panes.
- **Alacritty:** Live-reloads on file save. If changes do not reflect, restart the Alacritty application.
- **Validation Caveat:** `herdr server reload-config` returns `"status":"applied"` even if a key syntax cannot be decoded by the terminal. Verify the binding with a physical keypress.

---

## Discovering Herdr Action Names

- Action names live in `~/.config/herdr/config.toml` under `[keys]`.
- Inspect the authoritative list of key names and defaults from the installed binary:

```bash
herdr --default-config
```

Look under `[keys]` for actions such as `new_workspace`, `close_workspace`, `new_tab`, `split_vertical`, `split_horizontal`, `zoom`, etc.

---

## Diagnosing Key Sequences with a Byte Dumper

To inspect what bytes Alacritty sends when a key combination is pressed inside a Herdr pane:

1. Run the raw-mode terminal reader inside a Herdr pane:

```bash
python3 -c 'import sys,os,tty,termios
fd=sys.stdin.fileno(); old=termios.tcgetattr(fd)
try:
    tty.setraw(fd)
    while True:
        b=os.read(fd,128)
        if b in (b"q", b"\x03"): break
        sys.stdout.write("bytes: "+" ".join(f"{c:02x}" for c in b)+"\r\n"); sys.stdout.flush()
finally:
    termios.tcsetattr(fd,termios.TCSADRAIN,old)'
```

2. Press the key combination:
   - If Herdr bound the key, Herdr consumes it and no bytes appear in the dumper.
   - If unbound or unhandled, the raw hex bytes printed show what sequence Alacritty emitted (e.g. `1b 5b 31 31 36 3b 39 75` for `\u001b[116;9u`).

3. Inspect Herdr binary symbols and logging:

```bash
strings -n 4 "$(readlink -f /opt/homebrew/bin/herdr)" | grep -iE "kitty|SUPER|modifier|HERDR_LOG|input/terminal"
```

To enable debug logging for new servers: `HERDR_LOG=herdr=debug`.

---

## Tips and Caveats

- **Avoid Outer Alacritty Tab Actions:** Do not bind Alacritty's native `action = "CreateNewTab"` or `CreateNewWindow` to Cmd shortcuts intended for Herdr, as Alacritty will intercept the shortcut and create native OS windows instead of Herdr workspaces.
- **Valid TOML Escapes:** Write CSI-u sequences using TOML `\u001b` unicode escape sequences (e.g. `chars = "\u001b[116;9u"`).
- **Confirmation Prompts:** Certain Herdr commands (such as `close_workspace`) may display an interactive confirmation prompt by default. Check `herdr --default-config` for confirmation options if immediate execution without prompts is desired.

