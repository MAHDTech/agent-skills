# Usage Guide

## Using Skills

Once installed, your AI agent can use these skills automatically.

Trigger them by asking the agent to perform the task described in the skill.

Example triggers:

- "Sync my skills manifest"
- "Capture learnings from this session"
- "Create a new skill"

## Synchronizing Manifests

If you add or modify skills, run the sync script to update `README.md` and `AGENTS.md`:

````bash
bun run sync-skills
```text

This is also enforced as a pre-commit hook if you use `devenv`.

## Development

Use `devenv shell` to enter the development environment with all required tools (Bun, Zola, etc.).
````
