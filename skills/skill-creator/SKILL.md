---
name: skill-creator
description: Guide for creating effective skills that follow the Agent Skills standards. Use this when the user wants to create a new skill or update an existing one.
triggers:
  - "create a new skill"
  - "add a skill"
  - "how do I write a skill"
category: utility
---

# Skill Creator

This skill guides you through creating a new agent skill that is compatible with this repository's automated sync and dashboard.

## Requirements

Every skill MUST have a `SKILL.md` file in its own directory under `skills/`.

## Mandatory YAML Frontmatter

The `SKILL.md` MUST start with YAML frontmatter:

```yaml
---
name: kebab-case-name
description: A concise summary of what the skill does.
triggers:
  - "trigger phrase 1"
  - "trigger phrase 2"
category: coding -- utility -- custom
---
```

## Structure

```text
skills/<skill-name>/
  SKILL.md        # The main entry point
  scripts/        # Optional: any automation scripts (Bun, Shell, etc.)
  assets/         # Optional: images, icons, etc.
  references/     # Optional: extended documentation
```

## Best Practices

1. **Concise Description**: The description is used in the README and dashboard. Keep it under 200 characters.
2. **Clear Triggers**: Triggers help agents understand when to invoke the skill.
3. **Non-Interactive Scripts**: Ensure all scripts in `scripts/` can run without user input.
4. **Validation**: Run `bun run skills-sync` after creating a skill to verify it's correctly integrated.
