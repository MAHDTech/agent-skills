# ADR 0002: Unified CLI Architecture and Ratatui TUI

## Context

Previously, the developer tooling was split into two top-level entrypoints: `bin/skills` and `bin/dashboard`.
Maintaining separate scripts causes CLI fragmentation, duplicated argument parsing logic, and disjointed developer workflows.
Furthermore, developers lack an interactive inspection interface to quickly search, view, lint, and manage skills in the terminal without remembering complex CLI flags.

## Decision

We will design and implement a single unified Rust binary with an integrated Terminal User Interface (TUI):

1. **Unified Binary Name:**
   Provide a unified CLI named `ask` (Agent Skills Kit), aliased or accessible as `agent-skills` and `skills` in devenv scripts.
2. **Subcommand Hierarchy (Clap Derive):**

   ```text
   ask
   ├── skills
   │   ├── lint
   │   ├── sync
   │   ├── install
   │   ├── uninstall
   │   ├── download-resources
   │   └── clean-resources
   ├── dashboard
   │   ├── build
   │   ├── serve
   │   ├── css
   │   └── lint
   └── tui (default when run interactively without subcommands)
   ```

3. **Ratatui Terminal UI:**
   - Interactive skill explorer: list by category, search by name/description, view raw frontmatter, inspect `resources/auto` vs `resources/manual`.
   - Action triggers: trigger `sync`, `lint`, `download-resources`, or `dashboard build` directly from the TUI with live progress widgets.
4. **Crate Organization:**
   - `crates/skills-core`: Core library for parsing, linting, syncing, downloading, and dashboard orchestration.
   - `crates/skills-tui`: Ratatui and Crossterm widgets, state management, and event handling.
   - `crates/skills-cli` (or `crates/agent-skills-cli`): Clap-based CLI entry point hosting both CLI commands and launching the TUI.

## Consequences

### Positive

- Single binary artifact simplifies distribution, testing, and devenv scripts.
- Rich, interactive developer experience via the Ratatui TUI.
- Clean separation between core engine logic (`skills-core`), UI widgets (`skills-tui`), and CLI dispatcher (`skills-cli`).

### Negative / Tradeoffs

- Additional crate dependencies for TUI (`ratatui`, `crossterm`).
