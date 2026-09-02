# Persistence and remote access

Herdr keeps panes running in a background server. Your terminal client
can detach and reconnect later.

For the local, SSH, and `herdr --remote` workflows, see [How to work
with Herdr](https://herdr.dev/docs/how-to-work/).

## Detach and reattach

Detach the client with `ctrl+b q`; panes and agents keep running.
Reattach by running `herdr` again. Stop the session and its panes with
`herdr server stop`.

When Herdr starts again after a full server stop, it restores the saved
session shape. For what survives detach, server restart, screen history
replay, native agent session restore, and live handoff, see [Session
state and restore](https://herdr.dev/docs/session-state/).

## Named sessions

Use named sessions when you want independent Herdr servers.

```
herdr session listherdr session attach workherdr session attach side-projectherdr session stop workherdr session delete side-project
```

Terminal window

A named session has its own panes, tabs, workspaces, sockets, and
runtime state. It still shares the same global config file.

Use `--json` for scripts:

```
herdr session list --jsonherdr session stop work --jsonherdr session delete side-project --json
```

Terminal window

## Remote attach over SSH

Herdr supports two remote modes. [How to work with
Herdr](https://herdr.dev/docs/how-to-work/) compares them. SSH to the server and run
`herdr` there for the tmux-style path, or attach through SSH from your
local machine:

```
herdr --remote workboxherdr --remote ssh://you@server:2222
```

Terminal window

In this mode, your local Herdr is a thin client. It connects over SSH,
starts or attaches to the remote Herdr server, and streams the UI back
to your local terminal. Because the client runs locally, Herdr can
bridge local desktop features such as image clipboard paste into the
remote session by copying the image to a remote temp file and pasting
that path.

By default, `herdr --remote` uses your local Herdr keybindings for that
attach. This keeps local muscle memory even when the remote server has
different config. The local keybindings are a snapshot from attach time;
detach and reattach after editing local keybindings. Use
`--remote-keybindings server` when you want the remote server config
instead. Local custom command keybindings are not sent, because those
commands would run on the remote host.

For repeat targets, use your SSH config:

```
Host workbox  HostName server.example.com  User you  Port 2222
```

Then attach with:

```
herdr --remote workbox
```

Terminal window

Remote attach supports Linux, macOS, and Windows local clients
connecting to Linux or macOS hosts on x86_64 and aarch64. Herdr checks
the remote platform, prefers a matching `herdr` already on the remote
`PATH`, then checks common direct, Homebrew, mise, and Nix profile
install paths. If no matching binary exists, interactive runs prompt to
install one to `~/.local/bin/herdr`; non-interactive runs fail instead
of modifying the host. If `~/.local/bin` is not on the remote `PATH`,
Herdr warns after install. Windows is not supported as the remote host.

By default, `herdr --remote` runs remote setup and the bridge through a
temporary SSH config that includes your SSH config first, then adds
fallback keepalive settings. Existing user keepalive settings win. Linux
and macOS clients also use a private per-attach control socket for
connection reuse; Windows OpenSSH does not. Set
`[remote].manage_ssh_config = false` to use plain `ssh` without Herdr’s
generated config or control socket.

Remote attach uses your normal OpenSSH authentication. If the target
uses a passphrase-protected key in a non-interactive shell, script, CI
job, or mobile terminal that cannot show the passphrase prompt, load the
key into ssh-agent first:

```
ssh-addherdr --remote workbox
```

Terminal window

For any remote authentication failure, verify plain SSH access first
with `ssh workbox`, then run `herdr --remote workbox` again.

By default, remote attach uses the normal restart/stop flow if it needs
to replace or restart a running remote server. To opt into experimental
live handoff for a supported running remote server, pass `--handoff`:

```
herdr --remote workbox --handoff
```

Terminal window

If you SSH into the server first and run `herdr` there, Herdr runs
entirely on the server and cannot access your local desktop clipboard
beyond normal terminal text paste.

When your local and remote platforms match, Herdr can copy the current
local binary for direct installs. For Homebrew, mise, and Nix installs,
or when the platforms differ, it downloads the matching release asset
for the current client version from `https://herdr.dev/latest.json`.

For local builds or custom binaries, set `HERDR_REMOTE_BINARY` to a
local file path before running remote attach.

```
HERDR_REMOTE_BINARY=target/release/herdr herdr --remote workbox
```

Terminal window

## Remote named sessions

Use `--session` with `--remote` to attach to a named session on the
remote host:

```
herdr --remote workbox --session agents
```

Terminal window

## Direct terminal attach

Full Herdr attach opens the whole workspace UI. Direct attach opens one
server-owned terminal in your current terminal.

Direct terminal attach is Unix-only on Windows.

Attach by agent target:

```
herdr agent attach reviewer
```

Terminal window

Attach by terminal ID:

```
herdr terminal attach term_abc123
```

Terminal window

Direct attach streams the current rendered terminal state, then live
ANSI frames. Input goes straight to that terminal.

Detach with `ctrl+b q`. Send a literal `ctrl+b` with `ctrl+b ctrl+b`.

Only one writable direct attach client owns input and resize for a
terminal. Use `--takeover` to replace an existing owner:

```
herdr terminal attach term_abc123 --takeover
```

Terminal window

For third-party bridges that only need rendered terminal bytes, use a
read-only terminal session observer:

```
herdr terminal session observe w1:p1 --cols 120 --rows 40
```

Terminal window

It prints newline-delimited JSON `terminal.frame` records with base64
ANSI bytes, then a `terminal.closed` record when the server closes the
stream. Multiple observers can watch the same terminal without taking
input, resize, scroll, or takeover ownership.

For an interactive bridge, use a writable terminal session controller:

```
herdr terminal session control w1:p1 --takeover --cols 120 --rows 40
```

Terminal window

Control mode prints the same newline-delimited frame records and reads
newline-delimited JSON commands on stdin. `terminal.input` sends text or
base64 bytes, `terminal.resize` changes the controller viewport,
`terminal.scroll` scrolls the attached viewport, and `terminal.release`
closes the controller. Only one controller owns input and resize at a
time.

## Single-process escape hatch

Use `--no-session` to run Herdr without the background server/client
split:

```
herdr --no-session
```

Terminal window

Use `--no-session` mainly for debugging or compatibility. Persistent
session mode remains the default.
