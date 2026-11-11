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


# Google Antigravity Documentation

Local reference documentation for official Google Antigravity developer guides. Always read these files directly to obtain accurate specifications, rules, and commands instead of relying on model pretraining memory.

## Role & Purpose

Use this skill when:

- Researching or verifying Google Antigravity specifications, command-line flags, rules, subagents, permissions, or hooks.
- Working on `plugin-agy-guards` features or configuring workspace settings.

## Execution Rules

1. **Locate the target topic**: Read the entry documentation index at `resources/auto/llms.txt` to find the exact filename in `resources/auto/` covering your query.
2. **Read target document**: Open the corresponding markdown file in `resources/auto/` to retrieve the primary source material.

## Completion Criteria

The research task is complete when:

1. The target documentation file identified from `resources/auto/llms.txt` has been read directly from `resources/auto/` to answer the query.
2. Verified facts, CLI commands, parameters, or schemas match the official text exactly.
3. Every cited fact is linked back to the user with a clickable link pointing to the specific documentation file.

