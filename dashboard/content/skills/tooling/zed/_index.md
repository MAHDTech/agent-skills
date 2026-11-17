+++
title = "zed"
description = "Configure or troubleshoot Zed editor settings, keybindings, and language servers. Use to verify Zed option names and update user or project settings in the correct scope."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Zed editor configuration

Zed's settings change between releases, so take every key, default, and schema from the vendored docs in `resources/auto/`, not from memory. The complete settings reference is `resources/auto/docs-reference-all-settings.md`.

## Configuration files

- User settings: `~/.config/zed/settings.json` (Linux and macOS), `%APPDATA%\Zed\settings.json` (Windows).
- Project settings: `.zed/settings.json` in the project root.
- Keymaps: `~/.config/zed/keymap.json` (Linux and macOS), `%APPDATA%\Zed\keymap.json` (Windows).

## Steps

1. Search `resources/auto/` for the exact key and an example, such as `lsp` (language server overrides), `buffer_font_size`, `vim_mode`, `agent` (AI agent and providers), `format_on_save`, or `file_types`.
2. If the local docs lack a recently added key, read the current page from `https://zed.dev/docs/llms.txt`, which indexes a Markdown copy of every docs page.
3. Read the target settings file and keep its existing keys and comments.
4. Choose the scope: user settings for personal preferences (theme, fonts, keybindings); `.zed/settings.json` for repository rules (formatter, per-language tab size, project language servers).
5. Write the change as JSONC (JSON with comments).

## Language-specific settings

- `file_types` maps a language name to extensions, file names, or globs. Path globs follow the default entries' form with a leading `**/` (for example `"**/templates/**/*.html"`); bare entries such as `"c"` match an extension or name.
- Override formatting per language under `languages`, for example `"languages": { "HTML": { "format_on_save": "off" } }`, when a formatter breaks template syntax such as Tera or Jinja inside `.html` files.

## Completion criteria

- The key and value match `resources/auto/` or the live docs page.
- The edited `settings.json`, `.zed/settings.json`, or `keymap.json` parses as JSONC.
- The setting sits in the scope the user asked for (user-wide or project).

