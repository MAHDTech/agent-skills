# Agent Skills Copilot Instructions

This repository is a multi-skill catalog for AI agentic CLIs (Gemini, Claude, OpenCode).

The project follows the "Agent Skills" pattern, using Markdown files with YAML frontmatter to define capabilities that agents can discover and use.

## Project Layout

```text
agents/                       # Global agent instructions (AGENTS.md, MEMORIES.md)
bin/                          # Core scripts (sync-skills.ts, install.ts)
dashboard/                    # Zola source for the skills dashboard
  content/                    # Automatically generated from skills/
  themes/tars-dashboards/     # Custom Zola theme with Tailwind CSS v4
docs/                         # Project documentation
scripts/                      # Utility scripts (skills-lint.ts)
skills/                       # Individual skill directories (flat structure)
  <skill-name>/
    SKILL.md                  # Main skill definition
    scripts/                  # Optional skill-specific scripts
    assets/                   # Optional skill-specific assets
.github/actions/              # Composite actions for CI/CD
.github/workflows/            # GitHub Actions pipelines
```

## Technology Stack

- **Runtime**: Bun (use `bun` not `npm`)
- **Dashboard**: Zola (Rust-based static site generator)
- **Styling**: Tailwind CSS v4
- **Dev environment**: `devenv` (Nix). Always run commands inside the shell or via `devenv shell -- <cmd>`
- **CI/CD**: GitHub Actions using composite actions and `devenv` for reproducibility.

## Quick Commands

```bash
bun run sync-skills           # Update AGENTS.md and README.md manifests
bun run lint                  # Verify skill frontmatter and structure
bun run build:dashboard       # Sync, build CSS, and generate Zola site
bun run setup                 # Interactive TUI for local installation
```

## Skill Standards

Every skill MUST have a `SKILL.md` file with mandatory YAML frontmatter:

```yaml
---
name: kebab-case-name
description: Concisely describes the skill.
triggers:
  - "example trigger phrase"
category: utility | coding | custom
---
```

## Commit Convention

Conventional commits: `type(scope): description`
Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
Scope examples: `skills`, `dashboard`, `ci`, `bin`
