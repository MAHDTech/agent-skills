# Agent Skills Copilot Instructions

This repository is a multi-skill catalog for AI agent CLIs. Skills are cross-compatible with four tools: Claude Code, OpenCode, Goose, and Antigravity CLI. All of them read the same Anthropic-style `skills/<name>/SKILL.md` format, so one skill runs everywhere.

The project follows the "Agent Skills" pattern: Markdown files with YAML frontmatter that define capabilities agents can discover and use. Commands are no longer a separate concept; former commands are now skills.

## Project Layout

```text
agents/                       # Global agent instructions (AGENTS.md is auto-generated, MEMORIES.md)
crates/                       # Rust workspace (ask-cli, skills-core, skills-tui)
dashboard/                    # Zola source for the skills dashboard
  content/                    # Automatically generated from skills/
  themes/tars-dashboards/     # Custom Zola theme with Tailwind CSS v4
docs/                         # Project documentation (install.md, usage.md)
skills/                       # Skill directories grouped by category
  <category>/                 # engineering, planning, review, github, reflection, writing, authoring, game-development, tooling
    <skill-name>/
      SKILL.md                # Main skill definition
      resources/              # Optional skill-specific resources (no files directly in resources/)
        auto/                 # Downloader-owned resources fetched via ask skills download-resources
        manual/               # Hand-authored resources, scripts, and assets
  in-progress/                # Lifecycle bucket for work in progress
skills-archive/               # Retired skills (<category>/<skill-name>/)
.github/workflows/            # GitHub Actions pipelines
```

## Technology Stack

- **Runtime & CLI**: Rust toolchain via `devenv`; native `ask` CLI in `crates/ask-cli` (`ask` or `cargo run -p ask-cli --`)
- **Dashboard**: Zola (Rust-based static site generator)
- **Styling**: Tailwind CSS v4
- **Dev environment**: `devenv` (Nix). Always run commands inside the shell or via `devenv --no-tui shell -- <cmd>`
- **CI/CD**: GitHub Actions using composite actions and `devenv` for reproducibility

## Quick Commands

```bash
ask skills --action install         # Symlink the working tree into your agents (idempotent)
ask skills --action uninstall       # Remove every symlink this repo owns
ask skills sync                     # Relink AND regenerate README, agents/AGENTS.md, dashboard, skills.sh.json
ask skills lint                     # Verify skill frontmatter and structure
ask dashboard build                 # Sync content, build CSS, and generate the Zola site
ask dashboard serve                 # Build once, then serve locally with live reload
```

## Skill Standards

- Skills live at `skills/<category>/<name>/SKILL.md`. The folder `<name>` MUST match the `name` in the frontmatter.
- Names are prefix-free kebab-case. The old `brain-`, `cmd-`, and `sys-` prefixes are gone; subject scopes such as `gh-` and `git-` are kept.
- Every skill MUST have a `SKILL.md` file with YAML frontmatter. `name` and `description` are the mandatory minimum:

```yaml
---
name: kebab-case-name
description: Concisely describes what the skill does and when to use it.
---
```

Optional frontmatter fields, when needed:

- `disable-model-invocation` - prevent the model from auto-invoking the skill
- `argument-hint` - hint shown for expected arguments
- `context` - additional context to load with the skill
- `agent` - restrict the skill to a specific agent
- `metadata` - arbitrary key/value metadata

Do NOT use the legacy `custom:`, `triggers:`, `category:`, or `type:` fields. Category comes from the folder path, not frontmatter.

## Commit Convention

Conventional commits, enforced with convco: `type(scope): description`
Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
Scope examples: `skills`, `dashboard`, `ci`, `cli`
