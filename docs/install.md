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

2. Install the skills into your agent tools:

   ```bash
   bun run skills --action install
   ```

3. Verify the install by checking an agent's skills location (for example `~/.agents/skills/`) to confirm the symlinks were created, then trigger a skill from your agent to see it run.

`skills --action install` symlinks the working tree into each detected agent's skills location:

- `~/.agents/skills/` for OpenCode, Goose, and Antigravity (registered via `~/.gemini/config/skills.json`)
- `~/.claude/skills/` for Claude Code

It auto-detects which tools are installed and only wires those. It is idempotent: re-run it any time you add, rename, or remove a skill - it cleans up its own stale links (from renames or deletions) while leaving any skills you hand-copied for testing untouched. Because everything is symlinked, edits to a `SKILL.md` in your clone take effect immediately in every agent, with no reinstall needed.

The three commands you will use:

| Command                     | What it does                                                                                                                                    |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `skills --action install`   | Wire the skills into your machine's agent tools (idempotent)                                                                                    |
| `skills --action uninstall` | Remove every symlink this repo owns                                                                                                             |
| `skills --action sync`      | Do both of the above **and** regenerate the README, `agents/AGENTS.md`, `skills.sh.json`, and dashboard - the "make everything current" command |
