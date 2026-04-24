# Agent Skills Repository

## Overview

This repository house AI Agent skills for use with _OpenCode_.

See the [README.md](./README.md) for an overview of the project and available skills.

## Development

- Environment: This project requires `devenv`. Always check if `devenv` is active. Run ad-hoc commands inside the devenv shell with `devenv shell -- <command>`.
  - **CRITICAL LINTING RULE:** NEVER run individual linters directly (e.g., `devenv shell -- markdownlint .`). All linters and pre-commit hooks MUST be configured and run inside `devenv.nix`.
  - **CRITICAL TESTING RULE:** ALWAYS run tests via `devenv test` or the `run-tests` wrappers. This is the single guaranteed path.
  - If a specific linter or pre-commit check doesn't exist, check the devenv MCP server or devenv agent docs. If you STILL don't find it, ask the user for confirmation.
- Runtime: `bun`. Use `bun` for all scripts.
- CLI Skills Tool: `bun run bin/skills.ts`. Use this to manage skills.

## Dashboard

- To build/sync: `bun run build:dashboard`.
- To serve: `bun run serve:dashboard`. (Uses Zola & Tailwind).
