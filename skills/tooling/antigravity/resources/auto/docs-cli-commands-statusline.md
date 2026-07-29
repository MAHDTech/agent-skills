- side_navigation
- Antigravity CLI
  \>
- Commands
  \>
- Status Line (/statusline)

# Status Line Command (/statusline)[link](#status-line-command-statusline)

Toggle the TUI status line or configure a custom rendering command.

## Overview[link](#overview)

The `/statusline` command allows you to quickly enable or disable the
status line at the bottom of your TUI, or configure a custom shell
command to render it dynamically, without manually editing your settings
file.

For details on how to write custom status line scripts and the JSON
state payload schema, see the conceptual **[Status Line Customization
Guide](https://antigravity.google/docs/cli/statusline)**.

## Usage[link](#usage)

Run the `/statusline` command with the following arguments to control
its behavior:

### Toggle Status Line[link](#toggle-status-line)

Type `/statusline` with no arguments to toggle the status line on and
off:

text

content_copy

```
/statusline
```

### Enable or Disable Explicitly[link](#enable-or-disable-explicitly)

You can explicitly enable or disable the status line:

- **Enable**: `/statusline on` or `/statusline enable`
- **Disable**: `/statusline off` or `/statusline disable`

bash

content_copy

```
/statusline off
```

### Configure a Custom Command[link](#configure-a-custom-command)

To route the agent state JSON payload to a custom script and render its
output in the status line, pass the command as an argument:

bash

content_copy

```
/statusline ~/.gemini/antigravity-cli/statusline.sh
```

This immediately updates your settings and starts running the script to
render the status line.

### Revert to Default[link](#revert-to-default)

To delete your custom command configuration and revert to the built-in
default status line:

bash

content_copy

```
/statusline delete
```

*(Note: `/statusline reset` is also supported).*

### Show Help[link](#show-help)

To view the quick command reference:

bash

content_copy

```
/statusline help
```

## Next steps[link](#next-steps)

- **[Status Line Guide](https://antigravity.google/docs/cli/statusline)**: Learn how to write
  custom scripts and handle the JSON payload.
- **[Window Title Command](https://antigravity.google/docs/cli/commands/title)**: Configure
  dynamic terminal window titles.
- **[CLI Reference](https://antigravity.google/docs/cli/reference)**: See all available slash
  commands.

On this Page
