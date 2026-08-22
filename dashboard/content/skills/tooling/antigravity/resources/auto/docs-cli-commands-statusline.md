+++
title = "docs-cli-commands-statusline"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "antigravity"
+++

Markdownkeyboard_arrow_down

content_copyCopy Markdown

open_in_newView Markdown

# Status Line Command (/statusline)

Toggle the TUI status line or configure a custom rendering command.

## Overview

The `/statusline` command allows you to quickly enable or disable the
status line at the bottom of your TUI, or configure a custom shell
command to render it dynamically, without manually editing your settings
file.

For details on how to write custom status line scripts and the JSON
state payload schema, see the conceptual **[Status Line Customization
Guide](https://antigravity.google/docs/cli/statusline)**.

## Usage

Run the `/statusline` command with the following arguments to control
its behavior:

### Toggle Status Line

Type `/statusline` with no arguments to toggle the status line on and
off:

``` astro-code
/statusline
```

### Enable or Disable Explicitly

You can explicitly enable or disable the status line:

- **Enable**: `/statusline on` or `/statusline enable`
- **Disable**: `/statusline off` or `/statusline disable`

``` astro-code
/statusline off
```

### Configure a Custom Command

To route the agent state JSON payload to a custom script and render its
output in the status line, pass the command as an argument:

``` astro-code
/statusline ~/.gemini/antigravity-cli/statusline.sh
```

This immediately updates your settings and starts running the script to
render the status line.

### Revert to Default

To delete your custom command configuration and revert to the built-in
default status line:

``` astro-code
/statusline delete
```

*(Note: `/statusline reset` is also supported).*

### Show Help

To view the quick command reference:

``` astro-code
/statusline help
```

## Next steps

- **[Status Line Guide](https://antigravity.google/docs/cli/statusline)**: Learn how to write
  custom scripts and handle the JSON payload.
- **[Window Title Command](https://antigravity.google/docs/cli/commands/title)**: Configure
  dynamic terminal window titles.
- **[CLI Reference](https://antigravity.google/docs/cli/reference)**: See all available slash
  commands.

