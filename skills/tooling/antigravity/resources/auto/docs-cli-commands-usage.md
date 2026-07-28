- side_navigation
- Antigravity CLI
  \>
- Commands
  \>
- Model Quotas (/usage, /quota)

# Model Quotas (/usage)[link](#model-quotas-usage)

View your active model quota usage and refresh your configuration.

## Overview[link](#overview)

Antigravity CLI provides the `/usage` command (alias `/quota`) to help
you monitor your resource consumption. When run, the command refreshes
your model configuration and quota status from the backend and opens an
interactive TUI panel.

## Viewing your usage[link](#viewing-your-usage)

To open the Model Quotas panel:

1.  Type `/usage` (or `/quota`) in the prompt box.
2.  Press `Enter`.

text

content_copy

```
/usage
```

![Quota & Credits TUI](https://antigravity.google/assets/image/docs/cli/usage-tui.png)

### Interactive Panel Features[link](#interactive-panel-features)

The panel displays:

- **Model Quotas**: A breakdown of your usage limits and remaining
  requests/tokens for each supported model (e.g., Gemini 3.5 Flash,
  Gemini 3.1 Pro).
- **Active Refresh**: The CLI automatically triggers a fresh check of
  your quotas on disk and from the backend service when you open this
  panel.

### Navigation Controls[link](#navigation-controls)

Use the following keyboard shortcuts to navigate the panel:

| Key                      | Action                                    |
|--------------------------|-------------------------------------------|
| `↑` / `↓` (or `j` / `k`) | Scroll up or down by one line.            |
| `PgUp` / `PgDn`          | Scroll up or down by one page.            |
| `g` / `G`                | Jump to the top or bottom of the list.    |
| `Esc` (or `q`)           | Close the panel and return to the prompt. |

## Next steps[link](#next-steps)

- **[CLI Reference](https://antigravity.google/docs/cli/reference)**: See all available slash
  commands and keybindings.
- **[Settings & Rendering](https://antigravity.google/docs/cli/settings)**: Configure your default
  models and credit usage preferences.

On this Page
