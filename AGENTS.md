# Agent Skills Repository

## Overview

This repository houses AI Agent skills that work across multiple agent tools: _Claude Code_, _OpenCode_, _Goose_, and _Antigravity CLI_. They all read the same Anthropic-style `skills/<name>/SKILL.md` format, so a single skill runs everywhere.

See the [README.md](./README.md) for an overview of the project and available skills.

## Structure

- Skills live under `skills/<category>/<name>/SKILL.md`, grouped by topic (engineering, game-development, planning, review, github, reflection, writing, authoring, tooling), with `in-progress/` and `deprecated/` lifecycle buckets.
- Names are prefix-free kebab-case; each folder name matches the skill `name` in its frontmatter.
- Additional scripts, documentation, static assets, or reference files live under a skill's `resources/` directory, split by ownership into exactly two subdirectories: `resources/auto/` (downloader-owned — (re)fetched from the skill's `resources:` frontmatter URLs by `skills --action download-resources`, safe to wipe; `clean-resources` removes only this) and `resources/manual/` (hand-authored; tooling never touches it). No files may sit directly in `resources/`; lint enforces this.
- **CRITICAL LINKING RULE:** Never use absolute `file:///` URLs referencing local paths (e.g., `file:///home/...`). Always use relative paths for links referencing files within the repository (e.g., `../../tooling/prek/SKILL.md`). This ensures paths do not leak local user directories and resolve correctly in CI and other environments.
- Consumers install with `npx skills add MAHDTech/agent-skills`. See [docs/install.md](./docs/install.md) for details. Note that skills do not need to be published to the npm registry; the installer CLI fetches them directly from this GitHub repository.

## Development

- Environment: This project requires `devenv`. Always check if `devenv` is active. Run ad-hoc commands inside the devenv shell with `devenv --no-tui shell -- <command>`.
  - ⚠️ **CRITICAL WARNING**: Always pass the `--no-tui` flag when running `devenv` commands (e.g., `devenv --no-tui shell`, `devenv --no-tui test`) in automated or AI agent environments to disable the interactive terminal interface and prevent commands from getting stuck.
  - **CRITICAL LINTING RULE:** NEVER run individual linters directly (e.g., `devenv --no-tui shell -- markdownlint .`). All linters and pre-commit hooks (managed via `prek`, see the [prek](skills/tooling/prek/SKILL.md) skill) MUST be configured and run inside `devenv.nix`.
  - **CRITICAL TESTING RULE:** ALWAYS run tests via `devenv --no-tui test` or the `run-tests` wrappers. This is the single guaranteed path.
  - If a specific linter or `prek` hook check doesn't exist, check the devenv MCP server or devenv agent docs. If you STILL don't find it, ask the user for confirmation.
- Runtime: `bun`. Use `bun` for all scripts.
- CLI Skills Tool: `bun run bin/skills/index.ts` (also `bun run skills`). Use this to lint and sync skills, and to symlink them for local development.

## Dashboard

- To build: `dashboard --action build` — syncs content, builds CSS, renders the site, indexes it (all-in-one).
- To serve: `dashboard --action serve` — builds once, then serves with live reload. (Uses Zola & Tailwind.)
- CSS only: `dashboard --action css` (escape hatch; `build` already does this).

## Behaviour

Follow the behavioural guidelines that reduce common LLM coding mistakes: think before coding (surface assumptions and tradeoffs instead of guessing), keep changes simple and surgical (minimum code, touch only what the request needs), and drive every task to a verified success criterion. They bias toward caution over speed — for trivial tasks, use judgment.

The canonical, full version lives in the `agent-guidelines` skill: [skills/tooling/agent-guidelines/SKILL.md](skills/tooling/agent-guidelines/SKILL.md). Keep that file as the source of truth; this section is only a summary.
