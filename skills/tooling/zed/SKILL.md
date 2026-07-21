---
name: zed
description: Find, search, and manage Zed editor configuration settings, keybindings, and language server configurations. Use when configuring Zed settings, creating or updating ~/.config/zed/settings.json or .zed/settings.json, editing keymap.json, or searching for Zed options and documentation.
resources:
  - https://zed.dev/llms.txt
---

# Zed Editor Configuration

Guidelines for discovering, configuring, and validating options for the Zed code editor.

Because Zed evolves rapidly, avoid assuming hardcoded key paths or deprecated schema keys. Instead, use the vendored documentation in `resources/auto/` as the single source of truth for available settings, default values, and schema structures.

## Configuration File Locations

Zed supports both global user settings and workspace-specific project settings:

- **Global User Settings:**
  - Linux/macOS: `~/.config/zed/settings.json`
  - Windows: `%APPDATA%\Zed\settings.json`
- **Project Local Settings:**
  - Workspace root: `.zed/settings.json`
- **Global Keymaps:**
  - Linux/macOS: `~/.config/zed/keymap.json`
  - Windows: `%APPDATA%\Zed\keymap.json`

## Documentation & Option Lookup Protocol

When configuring Zed settings for a task, follow this search and lookup protocol:

1. **Query Vendored Documentation:**
   Search the `resources/auto/` directory within this skill using pattern or string search tools to locate exact setting keys, section titles, and examples.
   - Example search targets: language server overrides (`lsp`), font settings (`buffer_font_size`), Vim mode (`vim_mode`), assistant/AI providers (`assistant`), or file formatting (`format_on_save`).
2. **Inspect Existing Settings:**
   Before introducing new keys, check the target `settings.json` or `.zed/settings.json` to preserve existing user choices and structure.
3. **Verify Settings Scope:**
   - Use global `settings.json` for user-wide preferences (e.g. theme, font, default keybindings).
   - Use `.zed/settings.json` for repository-specific rules (e.g. workspace formatter, language-specific tab size, project-specific LSPs).
4. **Update Offline Resources when Outdated:**
   If a newly released Zed feature or configuration key is not present in local resources, trigger a resource sync:

   ```bash
   skills --action download-resources --skill zed
   ```

## Syntax & Formatting Rules

1. **JSON / JSONC:** Zed settings files use JSONC (JSON with comments). Ensure syntax remains valid JSON or JSONC when editing.
2. **Clean Edits:** When editing settings via automated tools, avoid deleting unrelated top-level keys or inline documentation comments.

## Completion Criteria

A Zed configuration task is complete when:

1. The target setting key and value have been confirmed against the vendored docs in `resources/auto/`.
2. The modified configuration file (`settings.json`, `.zed/settings.json`, or `keymap.json`) contains valid JSON without syntax or schema errors.
3. Settings are saved in the correct scope (user-wide vs project-local).
