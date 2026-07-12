---
name: install-skills
description: Install, update, and manage agent skills from a GitHub-hosted collection with the skills.sh CLI (`npx skills add <owner>/<repo>`), across every runtime the collection targets — Claude Code, OpenCode, Goose, and Antigravity CLI. Use when adding skills to an agent, updating a stale copy, verifying an install, wiring the same collection into another runtime, or troubleshooting a skill that will not show up or that collides by name.
---

# Install Skills

One collection, many runtimes. A `skills/<name>/SKILL.md` collection is Anthropic-style, so the same source installs into Claude Code, OpenCode, Goose, and Antigravity CLI. Install and manage it with the [skills.sh](https://skills.sh) CLI.

## Install a collection

Point the CLI at a GitHub `<owner>/<repo>`:

```bash
# NPM
npx skills add MAHDTech/agent-skills

# Bun
bunx skills add MAHDTech/agent-skills
```

The installer auto-detects which agents you have (Claude Code, OpenCode, Goose, Antigravity), then lets you pick the skills and the agents to wire. Skills install by **symlink**, so later updates to the source flow through without a reinstall.

## Manage installed skills

The `add` and `update` verbs are the load-bearing ones:

```bash
npx skills add <owner>/<repo>   # install / pick more skills from a collection
npx skills update               # pull the latest for everything installed
```

Run `npx skills --help` for the full verb set (listing what is installed and removing a skill live here) before reaching for an exact flag — confirm it from the CLI's own help rather than guessing.

## Where a skill surfaces per runtime

After install, a skill lives at its runtime's skills location and the agent picks it up from there:

- **Claude Code** — `~/.claude/skills/`
- **OpenCode & Goose** — `~/.agents/skills/`
- **Antigravity CLI** — registered in `~/.gemini/config/skills.json`

Exact paths can shift per version; treat these as where to look, and confirm against the runtime's own docs if one differs.

## Verify an install

1. **Check the location** — the runtime's skills directory (above) holds a symlink for each skill you selected.
2. **Trigger it** — ask the agent to do the task the skill's description names, and confirm the skill fires. A model-invoked skill should engage on its own; a user-invoked one you name directly.

Done when the symlink exists **and** the skill actually runs.

## Troubleshoot

- **Skill not showing up** — confirm the runtime was selected during `add` (re-run `add` and pick it), then restart the agent so it re-scans its skills directory. Check the symlink resolves to a real `SKILL.md`; a broken link means the source moved or was renamed.
- **Stale copy** — run `npx skills update`. If it stays stale, the symlink may point at an old path from a rename; remove and re-`add` the skill.
- **Name collision** — two skills with the same directory name clash in one runtime's flat skills directory. Keep the one you want and remove the other; a skill's name is its directory basename, so distinct names avoid the clash.
- **Nothing detected** — `add` only wires agents it finds installed. Install the runtime first, then re-run `add`.
