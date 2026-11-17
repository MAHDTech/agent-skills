+++
title = "antigravity"
description = "Look up Google Antigravity/AGY CLI, IDE, and SDK behavior. Use for its settings, commands, keybindings, agent permissions, hooks, or plugin integration."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Google Antigravity documentation

Answer Antigravity questions from the vendored official docs in `resources/auto/` rather than from memory. Each file is the Markdown version of one page under `https://antigravity.google/docs/`, named after its path (for example `resources/auto/docs-hooks.md`, `resources/auto/docs-cli-reference.md`, `resources/auto/docs-sdk-subagents.md`).

Use this skill to research or verify Antigravity CLI flags, slash commands, settings, rules, skills, subagents, permissions, sandboxing, hooks, MCP, plugins, or the SDK, or to configure Antigravity workspace or user settings.

## Steps

1. Find the page: list or search `resources/auto/` by topic keyword (file names mirror the docs URL paths).
2. Read the matching file and answer from its text.
3. When no local file covers the topic, fetch `https://antigravity.google/llms.txt` for the current page index and read the page's `.md` URL (append `.md` to the docs path).

## Completion criteria

- Every fact, command, parameter, or schema in the answer matches the docs text.
- Each cited fact names its source page as a `https://antigravity.google/docs/...` URL.

