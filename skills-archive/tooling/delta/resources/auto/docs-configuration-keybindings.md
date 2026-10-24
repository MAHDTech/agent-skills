---
title: "Keybindings"
url: "https://delta.dev/docs/configuration/keybindings"
description: "Keyboard shortcuts for working in Delta."
---

1.  Configuration
2.  Keybindings

# Keybindings

Keyboard shortcuts for working in Delta.

On this page

1.  [View shortcuts](#view-shortcuts)
2.  [Command palette](#command-palette)
3.  [Switch between thread and file pane](#switch-between-thread-and-file-pane)
4.  [Menu bar](#menu-bar)
5.  [Common shortcuts](#common-shortcuts)
6.  [Custom keybindings](#custom-keybindings)
7.  [What’s next](#whats-next)

Delta is keyboard-driven: every command is an _action_, and actions can be bound to keys.

## [View shortcuts](#view-shortcuts)

Press `Cmd+/` on macOS or `Ctrl+/` on Linux and Windows to open the keyboard shortcuts reference, which lists Delta’s actions grouped by area. It’s also in the menu bar as **Keymap**. The keys shown are looked up from the live keymap, so the reference always reflects the actual bindings, including platform differences in the defaults.

## [Command palette](#command-palette)

Press `Cmd+Shift+P` on macOS or `Ctrl+Shift+P` on Linux and Windows to open the command palette with the command filter applied. Press `Cmd+K` on macOS or `Ctrl+K` on Linux and Windows to search commands and threads together. The palette is also in the menu bar as **Command Palette…**. When a terminal is focused, `Cmd+K` goes to the terminal instead; use `Cmd+Shift+P`.

Press `Tab` in the command palette to switch to **Ask Delta**. Describe what you want to do, then press `Enter`. Delta answers and suggests relevant commands; select a suggestion to run it. Press `Tab` again to return to command and thread search.

## [Switch between thread and file pane](#switch-between-thread-and-file-pane)

Press `Cmd+?` on macOS or `Ctrl+?` on Linux and Windows to toggle focus between the thread and the file pane.

## [Menu bar](#menu-bar)

Actions are also discoverable through the menu bar, which shows the shortcut next to each menu item.

## [Common shortcuts](#common-shortcuts)

These shortcuts describe the desktop app. Browser shortcuts can differ or be reserved by the browser; `Cmd+D` and `Ctrl+D` do not bookmark a Delta thread on the web.

This is a curated list of common navigation and workflow shortcuts. For the complete live list, press `Cmd+/` on macOS or `Ctrl+/` on Linux and Windows.

Action

macOS

Linux and Windows

Send a message

`Cmd+Enter`

`Ctrl+Enter`

Send a message to a fork

`Cmd+Shift+Enter`

`Ctrl+Shift+Enter`

Open the primary project’s menu

`Cmd+Option+O`

`Ctrl+Alt+O`

Choose a branch

`Ctrl+Cmd+B`

`Ctrl+Alt+B`

Switch thread/file pane focus

`Cmd+?`

`Ctrl+?`

Focus the parent conversation (desktop)

`Cmd+1`

`Ctrl+1`

Find a thread

`Cmd+T`

`Ctrl+T`

Pin or unpin the current thread

`Cmd+Option+P`

`Ctrl+Alt+P`

Bookmark or unbookmark the current thread

`Cmd+D`

`Ctrl+D`

Show Bookmarked Threads

`Cmd+Option+B`

No default

View All Subthreads

`Cmd+J`

`Ctrl+J`

Return to the parent thread

`Cmd+Option+U`

`Ctrl+Alt+U`

Archive the current thread

`Cmd+W`

`Ctrl+W`

Share the current thread

`Cmd+Shift+S`

`Ctrl+Shift+S`

Copy a share link

`Cmd+Option+L`

`Ctrl+Alt+L`

Reopen a recently archived thread

`Cmd+Shift+T`

`Ctrl+Shift+T`

Show thread history

`Cmd+Y`

No default

Start a review thread

`Cmd+Shift+R`

`Ctrl+Shift+R`

Open Review Changes

`Option+Shift+D`

`Alt+Shift+D`

Collapse or expand all review files

`Shift+Tab`

`Shift+Tab`

Mark all review files seen or unseen

`Cmd+Shift+Y`

`Ctrl+Shift+Y`

Open a link or a file from tool output

`Option+Enter`

`Alt+Enter`

Open a terminal in the thread

`` Ctrl+` ``

`` Ctrl+` ``

Toggle terminal background mode

`Cmd+Option+Z`

`Ctrl+Alt+Z`

Comment on a selection

`Cmd+Option+M`

`Ctrl+Alt+M`

Toggle comment mode in a file tab

`Cmd+Option+C`

`Ctrl+Alt+C`

Open or close a delivered comment

`Cmd+Shift+M`

`Ctrl+Shift+M`

Show or hide comments in Files and Changes

`Cmd+Option+V`

`Ctrl+Alt+V`

Previous / next file tab

`Cmd+Option+Left` / `Cmd+Option+Right`

`Ctrl+Alt+Left` / `Ctrl+Alt+Right`

Follow the next collaborator

`Ctrl+Option+Cmd+F`

`Ctrl+Alt+Shift+F`

Cycle the cursor position

`Ctrl+L`

`Ctrl+L`

Open the log viewer

`Cmd+Shift+L`

`Ctrl+Shift+L`

Search the focused view

`Cmd+F`

`Ctrl+F`

Next / previous search match

`Cmd+G` / `Cmd+Shift+G`

`Ctrl+G` / `Ctrl+Shift+G`

When the file pane is focused, `Cmd+W` on macOS or `Ctrl+W` on Linux and Windows closes the file tab instead of archiving the thread. With the project or Changes panel focused, it closes that panel. In a focused subagent conversation, `Escape` stops that subagent.

Use `Ctrl+P` or `Ctrl+K` to select the previous item in menus, and `Ctrl+N` or `Ctrl+J` to select the next. Review-file shortcuts apply when the diff is focused, not while editing a comment. The return-to-parent shortcut applies when a subthread is open as primary.

`Option+Enter` on macOS or `Alt+Enter` on Linux and Windows opens the link under the cursor in the conversation regardless of **Send With Modifier**.

Context-specific gestures are documented with their workflows:

*   [Thread search and navigation](/docs/agents/threads#search-and-navigate) covers moving through a thread by whole turns and selecting code blocks.
*   [Starting a thread](/docs/agents/threads#start-a-thread) covers composer indentation.
*   [Sending to a fork](/docs/agents/threads#send-a-message-to-a-fork) covers the shortcut and modified-click gesture.
*   [Opening files from the transcript](/docs/agents/threads#open-files-from-the-transcript) covers where `Option+Enter` works.

## [Custom keybindings](#custom-keybindings)

Delta does not support rebinding keys yet. The defaults are fixed, apart from the composer’s send behavior: `Cmd+Enter` (`Ctrl+Enter` on Linux and Windows) sends by default, and turning the [Send With Modifier setting](/docs/configuration/settings) off makes `Enter` send instead. With that setting off, `Cmd+Enter` on macOS or `Ctrl+Enter` on Linux and Windows sends to a new fork.

## [What’s next](#whats-next)

*   [Settings](/docs/configuration/settings) covers the settings that change what some keys do in the composer.