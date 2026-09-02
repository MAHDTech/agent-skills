# Agent Skills Context & Domain Glossary

## Overview

The Agent Skills repository houses AI agent skills formatted for multi-agent runners (Claude Code, OpenCode, Goose, Antigravity CLI) alongside developer tooling for linting, synchronization, testing, resource downloads, and dashboard generation.

## Domain Terms

### Skill

An Anthropic-style capability directory containing a `SKILL.md` file with YAML frontmatter and markdown instructions, and an optional `resources/` directory.

### Skill Frontmatter

The YAML block at the beginning of a `SKILL.md` file containing metadata such as `name`, `description`, `category`, and optional `resources`.

### Skill Resources

Auxiliary files associated with a skill, strictly partitioned into:

- `resources/auto/`: Machine-managed directory populated from URLs defined in frontmatter by `ask skills download-resources`. Tooling may wipe and refresh this directory.
- `resources/manual/`: Hand-authored directory containing static references or scripts. Tooling must never modify or delete this directory.

### Skills Sync

The orchestration process that inspects all skills in the repository and synchronizes metadata into `README.md`, `agents/AGENTS.md`, `skills.sh.json`, and the dashboard content directory.

### Skills Installer

The local developer mechanism (`ask skills install/uninstall`) that symlinks skill directories into local AI agent tool directories (`~/.agents/skills/`, `~/.claude/skills/`, `~/.gemini/config/skills.json`).

### Dashboard

The static web portal built with Zola, Tailwind CSS v4, and Pagefind to browse and inspect available skills.

### Unified CLI

A single unified binary CLI entry point combining skill management (`skills`), dashboard orchestration (`dashboard`), and an interactive terminal UI (`tui`).

### Terminal UI (TUI)

An interactive terminal interface powered by Ratatui for browsing skills, inspecting frontmatter/resources, checking lint status, and triggering sync or dashboard builds.

### Rust Workspace

A single Cargo workspace containing modular crates in `crates/` replacing the legacy Bun/TypeScript scripts.

## Relationships

- A **Skill** belongs to a Category and owns zero or more **Skill Resources**.
- **Skills Sync** reads **Skills** and updates project markdown tables, manifest JSON, and **Dashboard** content.
- The **Unified CLI** provides CLI and **Terminal UI (TUI)** interfaces to drive local developer workflows and CI checks.
- The **Rust Workspace** contains `skills-core`, `skills-tui`, and the unified CLI crate.
