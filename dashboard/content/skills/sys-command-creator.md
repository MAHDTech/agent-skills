+++
title = "sys-command-creator"
description = "Guide for creating effective commands that follow the Agent Commands standards. Use this when the user wants to create a new command or update an existing one."
date = 2026-06-01
[extra]
triggers = ["create a new command","add a command","how do I write a command"]
mermaid = false
is_command = false
+++


# Command Creator

This skill guides you through creating a new agent command that is compatible with this repository's automated sync and dashboard.

## Naming Conventions

Commands must follow a strict `<tool/domain>-<verb>-<noun[s]>.md` naming convention using kebab-case to group related items alphabetically and improve discoverability.

- **`tool`**: Use if the command is a wrapper for a specific CLI tool (e.g., `gh-create-issue.md`, `git-resolve-conflicts.md`).
- **`domain`**: Use if the command is a conceptual workflow without a primary backing CLI (e.g., `code-sculpt.md`, `pr-build-context.md`, `plan-store.md`).

They must reside directly under the `commands/` directory.

The value in the `name:` YAML frontmatter field MUST exactly match the base filename (without `.md`).

## Requirements

Every command MUST be a single Markdown file located in the `commands/` directory.

## Mandatory YAML Frontmatter

The command file MUST start with YAML frontmatter containing `name`, `description`, and a `custom` block with at least `type: command`. Additional fields like `context` or `agent` can be added under `custom` if needed.

```yaml
---
name: your-command-name
description: A concise summary of what the command does.
custom:
  type: command
---
```

## Structure

```text
commands/
  your-command-name.md
```

## Best Practices

1. **Clear Instructions**: Detail the steps the agent should follow when the command is invoked. Provide numbered instructions or a clear sequence of operations.
2. **Context Gathering**: Include explicit steps for the agent to gather necessary context (e.g., branch diffs, specific files, tool outputs) before executing the main task.
3. **Structured Execution**: Tell the agent exactly how to perform the work, constraints to follow, and exactly how the output should be formatted.

