# Agent Skills Repository

## Overview

This repository house AI Agent skills for use with _OpenCode_.

See the [README.md](./README.md) for an overview of the project and available skills.

## Development

- Environment: This project requires `devenv`. Always check if `devenv` is active. Run ad-hoc commands inside the devenv shell with `devenv shell -- <command>`.
- Runtime: `bun`. Use `bun` for all scripts.
- CLI Skills Tool: `bun run bin/skills.ts`. Use this to manage skills.

## Dashboard

- To build/sync: `bun run build:dashboard`.
- To serve: `bun run serve:dashboard`. (Uses Zola & Tailwind).
