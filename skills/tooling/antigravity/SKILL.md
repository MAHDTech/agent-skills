---
name: antigravity
description: Provides a comprehensive guide, quick reference, and sitemap for Google Antigravity (AGY), including the Antigravity CLI (agy), Antigravity 2.0, Antigravity IDE, Python SDK, slash commands, keybindings, and customizations (skills, rules, MCP, sidecars). Activate this skill when the user asks questions about how to use, configure, or customize Antigravity, AGY, the agy CLI, the Antigravity IDE, or Antigravity 2.0.
resources:
  - https://antigravity.google/llms.txt
---

# Google Antigravity Documentation

This skill provides a local knowledge repository of the official Google Antigravity developer documentation. Always read these files directly to obtain accurate specifications, rules, and commands instead of relying on model pretraining memory.

## Role & Purpose

Use this skill when:

- Researching or verifying Google Antigravity specifications, command-line flags, rules, subagents, permissions, or hooks.
- Working on `antigravity-plugin-tars` features or configuring workspace settings.

## Execution Rules

1. **Locate the target topic**: Read the entry documentation file at [resources/docs-home.md](resources/docs-home.md) using `view_file` to find the exact filename and relative path of the documentation covering your query.
2. **Read target document**: Open the corresponding markdown file in `resources/` to retrieve the primary source material.

## Completion Criteria

The research task is complete when:

1. The target documentation file identified from [resources/docs-home.md](resources/docs-home.md) has been read directly via `view_file` to answer the query.
2. Verified facts, CLI commands, parameters, or schemas match the official text exactly.
3. Every cited fact is linked back to the user with a clickable link pointing to the specific documentation file.
