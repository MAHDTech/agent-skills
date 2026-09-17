# Installation Guide

These skills are cross-compatible with Claude Code, OpenCode, Goose, and Antigravity CLI. All four read the same Anthropic-style `skills/<name>/SKILL.md` format.

## For users

Install with the [skills.sh](https://skills.sh) CLI:

[![skills.sh](https://skills.sh/b/MAHDTech/agent-skills)](https://skills.sh/MAHDTech/agent-skills)

```bash
# NPM users
npx skills add MAHDTech/agent-skills

# Bun users
bunx skills add MAHDTech/agent-skills
```

The installer auto-detects which agents you have installed (Claude Code, OpenCode, Goose, Antigravity), then lets you pick the skills and the agents you want. Selected skills are installed by symlink, so updates flow through automatically.

To update later:

```bash
# Using npm
npx skills update

# Using Bun
bunx skills update
```

## For developers

If you are working on the skills in this repository, install them from your local working tree so you can iterate live.

1. Clone the repository:

   ```bash
   git clone https://github.com/MAHDTech/agent-skills.git
   cd agent-skills
   ```

2. Synchronize the skills into your agent tools:

   ```bash
   ask skills sync
   ```

3. Verify the install by checking an agent's skills location (for example `~/.agents/skills/`) to confirm the symlinks were created, then trigger a skill from your agent to see it run.

`ask skills sync` symlinks the working tree into each detected agent's skills location and reconciles derived documentation:

- `~/.agents/skills/` for OpenCode, Goose, and Antigravity (registered via `~/.gemini/config/skills.json`)
- `~/.claude/skills/` for Claude Code

It auto-detects which tools are installed and only wires those. It is idempotent: re-run it any time you add, rename, or remove a skill; it cleans up its own stale links (from renames or deletions) while leaving any skills you hand-copied for testing untouched. Because everything is symlinked, edits to a `SKILL.md` in your clone take effect immediately in every agent, with no reinstall needed.

The three commands you will use:

| Command                        | What it does                                                                                                                                         |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ask skills sync`              | Wire skills into agent tools, prune stale symlinks, and regenerate derived repository files (`README.md`, `agents/AGENTS.md`, and dashboard metrics) |
| `ask skills install <source>`  | Wire an individual skill directory or catalog identifier into agent tools                                                                            |
| `ask skills uninstall <skill>` | Remove an individual skill's symlinks from agent tools                                                                                               |
