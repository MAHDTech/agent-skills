+++
title = "docs-installation"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Install Delta"
url: "https://delta.dev/docs/installation"
description: "Download and install Delta, then open the Welcome to Delta screen."
---

1.  Getting Started
2.  Install Delta

# Install Delta

Download and install Delta, then open the Welcome to Delta screen.

On this page

1.  [Download Delta](#download-delta)
2.  [Install from your terminal on macOS or Linux](#install-from-your-terminal-on-macos-or-linux)
3.  [Install on macOS](#install-on-macos)
    1.  [Install the command-line tool](#install-the-command-line-tool)
4.  [Install on Linux](#install-on-linux)
    1.  [Run without installing](#run-without-installing)
5.  [Install on Windows](#install-on-windows)
6.  [Use Delta from your terminal](#use-delta-from-your-terminal)
7.  [Continue setup](#continue-setup)
8.  [Run without a window](#run-without-a-window)

Use this guide to install Delta. To set up a model and complete your first task, continue with [Quick Start](/docs/getting-started/quick-start).

Delta currently supports:

*   macOS 13 or later on Apple Silicon.
*   Linux with glibc on Intel, AMD, or ARM64 processors.
*   Windows on Intel, AMD, or ARM64 processors.

Delta is in public beta. You need a Zed account to use it. [Sign in](/docs/account/authenticate) or create an account on the [Zed dashboard](https://dashboard.zed.dev/delta-beta), then accept the Delta Early Access Agreement. You can use Delta as soon as you accept.

## [Download Delta](#download-delta)

For macOS, Linux, and Windows, open the [Delta download page](https://delta.dev/download), then choose the release for your computer:

*   **macOS:** Apple Silicon (`aarch64`).
*   **Linux:** Intel or AMD (`x86_64`), or ARM64 (`aarch64`).
*   **Windows:** Intel or AMD (`x86_64`), or ARM64 (`aarch64`). Windows downloads are signed `.exe` installers.

## [Install from your terminal on macOS or Linux](#install-from-your-terminal-on-macos-or-linux)

You can also download the installer to install Delta and its command-line tool for your user:

```
curl -fsSL https://delta.dev/install.sh -o delta-install.sh
sh delta-install.sh
```

The installer asks before downloading and installing the app. If it finds an existing stable installation, it reuses it without checking for updates. Otherwise, it downloads the current stable release. On macOS, a new installation goes in `~/Applications/Delta.app`; on Linux, it goes in `~/.local/delta.app`. Linux also requires Python 3 to read the download response.

The installer links the command-line tool at `~/.local/bin/delta` and leaves unrelated commands at that path untouched. If that folder is missing from your `PATH`, it offers to update your Bash or Zsh configuration. Open a new terminal after accepting that change. For other shells, add the folder to `PATH` yourself.

The installer does not open Delta. To open it, run the CLI executable found by the installer, even if `~/.local/bin` is not on your `PATH`:

```
"$(sh delta-install.sh --locate-cli)"
```

To find an existing stable command-line tool without installing anything, run:

```
sh delta-install.sh --locate-cli
```

This prints the executable’s absolute path when found and exits with status 1 when it is absent.

For an unattended installation, use `sh delta-install.sh --non-interactive`. This mode does not prompt or change your shell configuration. On success, it prints the command-line tool’s absolute path; use that path if `delta` is not on your PATH.

## [Install on macOS](#install-on-macos)

1.  Unzip `Delta.app.zip`.
2.  Move `Delta.app` into `/Applications`.
3.  Open **Terminal** from **Applications > Utilities**.
4.  Paste this command into Terminal, then press Return:

```
xattr -dr com.apple.quarantine /Applications/Delta.app
```

5.  Open Delta from `/Applications`.

The Terminal step prevents macOS from repeatedly asking you to move Delta to Applications. See [Troubleshooting](/docs/troubleshooting) if that happens. Delta opens to the sign-in screen.

### [Install the command-line tool](#install-the-command-line-tool)

Choose **Delta > Install CLI…**, or **Install CLI…** in the command palette, to make the `delta` command available from your terminal. This links the bundled tool to `/usr/local/bin/delta` and requests administrator permission only if needed.

If an unrelated command already exists at that path, Delta leaves it untouched. Move or rename it before retrying.

## [Install on Linux](#install-on-linux)

1.  Open a terminal in the folder containing the downloaded archive. Most browsers save it in `Downloads`; run `cd ~/Downloads` to open that folder.
2.  Make `~/.local/bin` available in the current terminal:

```
export PATH="$HOME/.local/bin:$PATH"
```

3.  Replace `\<architecture\>` below with the value in the downloaded filename: `x86_64` or `aarch64`. Then run:

```
tar -xzf delta-linux-<architecture>.tar.gz
./Delta/install.sh
delta
```

This installs Delta in `~/.local/delta.app`, registers it with your desktop environment, and opens the app. The archive includes Delta’s application libraries but still requires compatible Linux system libraries.

The installer links the command-line tool at `~/.local/bin/delta`. Add the `export PATH` line above to your shell configuration if that folder is not already on your PATH. The installer does not edit shell configuration.

If an unrelated command already exists at `~/.local/bin/delta`, the installer leaves it untouched and prints a warning. Move or rename it, then rerun `install.sh` to create the link.

### [Run without installing](#run-without-installing)

To run Delta from the extracted archive:

```
./Delta/bin/delta
```

## [Install on Windows](#install-on-windows)

Run the Windows `.exe` installer you downloaded. It installs Delta for your user by default; its **Options** page offers a per-machine installation and a custom location. The installer adds Delta to the Start menu and registers `delta://` links that open Delta. It also adds the command-line tool’s folder to PATH. Open a new terminal after installation to use the `delta` command.

Open Delta from the Start menu. Delta opens to the sign-in screen.

## [Use Delta from your terminal](#use-delta-from-your-terminal)

Run `delta` to open the desktop app:

```
delta
```

The command-line tool also lets you [check your account, sign in, and sign out](/docs/account/authenticate#sign-in-from-your-terminal).

## [Continue setup](#continue-setup)

You should now see **Welcome to Delta**. Delta checks for updates when it starts.

Follow the in-app steps to sign in and connect a model. On **Add your team**, enter an organization name and optionally invite up to three teammates, then choose **Create Organization**. Choose **Skip** to continue without creating one. Creating a team does not switch the organization supplying your hosted models.

The **Welcome to Delta** introduction is complete after you follow its final prompt and open a terminal. To repeat the introduction later, run **Restart Welcome Guide** from the command palette. Then continue with [Quick Start](/docs/getting-started/quick-start) to complete your first project task.

## [Run without a window](#run-without-a-window)

CLI commands that need Delta can start its background process without opening a window. To start, inspect, or stop that process explicitly:

```
delta daemon start
delta daemon status
delta daemon stop
```

`delta daemon stop` quits the running Delta app, including any open windows. To open a window in the same running app, run:

```
delta open
```

Closing the last window keeps Delta running by default. Change **Settings > General > Advanced > Stay Open When Window Closed** to quit when the last window closes. This setting does not stop a process that started without any windows. See [Keep Computer Awake](/docs/configuration/settings#notifications) for the sleep behavior of background work.
