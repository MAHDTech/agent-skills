# Usage Guide

## Using skills

Once installed, your AI agent can use these skills automatically. Trigger them by asking the agent to perform the task a skill describes.

Example triggers:

- "Create a new skill"
- "Store this plan for later"
- "Resolve the conflicts on this branch"

## Discovering what is available

Run the `/skill-router` skill to browse the catalog and find the right skill for the task at hand. It surfaces the installed skills and points you to the one that fits.

## Category structure

Skills are organised by topic under `skills/<category>/<name>/SKILL.md`. Categories include `engineering`, `planning`, `review`, `github`, `reflection`, `writing`, `authoring`, `game-development`, and `tooling`, plus `in-progress/` and `deprecated/` lifecycle buckets. The category is the folder, not a frontmatter field.

## Linting and syncing

If you add or change a skill, use the CLI to validate and regenerate the derived files. Run it inside `devenv` so the toolchain is consistent:

```bash
# Validate frontmatter and folder structure
devenv --no-tui shell -- skills --action lint

# Regenerate the derived files from skills/
devenv --no-tui shell -- skills --action sync
```

The `sync` action regenerates `README.md`, `agents/AGENTS.md`, the dashboard content, and `skills.sh.json`. Syncing is also enforced as a pre-commit hook when you use `devenv`, so these stay in step with the skills.

## Development

Use `devenv --no-tui shell` to enter the development environment with all required tools (Bun, Zola, and the rest) on your PATH.
