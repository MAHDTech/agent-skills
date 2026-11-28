# Agent Skills Repository Instructions

This repository is a multi-skill catalog for AI agent CLIs (Claude Code, OpenCode, Goose, and Antigravity CLI) backed by a Rust CLI/TUI and a Zola dashboard.

## Stack

- **Language & CLI**: Rust (edition 2024, stable toolchain in `rust-toolchain.toml`), workspace crates `skills-core`, `skills-tui` (Ratatui), and `ask-cli` (Clap).
- **Environment & Hooks**: `devenv` (Nix) with `prek` via `git-hooks` in `devenv.nix`. Never use or add the deprecated standalone `pre-commit` CLI or `pkgs.pre-commit`.
- **Dashboard**: Zola (0.23+ with Tera 2 components), Tailwind CSS v4, and Pagefind search.
- **CI/CD**: GitHub Actions workflows in `.github/workflows/` validated with `actionlint` and `action-validator`.

## Architecture

```text
skills/<category>/<name>/     # Hand-authored active skills (SKILL.md, agents/openai.yaml, resources/)
skills/in-progress/<name>/    # Draft skills not yet promoted
skills/vendor/<vendor>/<name>/# Upstream vendor snapshots managed by vendor-skills.yaml (do not hand-edit)
skills-archive/<cat>/<name>/  # Retired skills kept for dashboard reference only
crates/                       # Rust workspace (skills-core, skills-tui, ask-cli)
dashboard/                    # Zola site, Tailwind CSS, and Tera theme (tars-dashboards)
  content/                    # Generated from skills/ and skills-archive/
  static/.well-known/         # Generated discovery manifests and digests
agents/                       # Global agent instructions (agents/AGENTS.md is generated)
scripts/                      # Test and tooling wrappers (run-tests.sh, test-dashboard.sh, codeql-run.sh)
```

Active skill categories: `engineering`, `game-development`, `planning`, `review`, `github`, `reflection`, `writing`, `authoring`, `tooling`, and `guards`.

## Commands

Run commands inside `devenv` and always pass `--no-tui` in non-interactive shells or automation:

```bash
devenv --no-tui test                                 # Full local CI: all prek hooks, Rust tests, dashboard test
devenv --no-tui shell -- ask skills lint             # Validate skill frontmatter, policies, and layout
devenv --no-tui shell -- ask skills sync             # Regenerate README.md, agents/AGENTS.md, skills.sh.json
devenv --no-tui shell -- ask dashboard lint --fix    # Regenerate dashboard/content/ and .well-known/ in place
devenv --no-tui shell -- ask dashboard build         # Sync content, compile CSS, build Zola site, build Pagefind index
devenv --no-tui shell -- dashboard-test              # Build dashboard into scratch dir and verify rendered output
devenv --no-tui shell -- git commit -m "<subject>"   # Run commit hooks with devenv tools on PATH
```

Stage new files before running `devenv --no-tui test`, because hooks only inspect Git-tracked files.

## Constraints (Don't)

- Do not hand-edit generated files (`README.md`, `agents/AGENTS.md`, `skills.sh.json`, `dashboard/content/`, `dashboard/static/.well-known/`, `dashboard/static/build/`, `dashboard/static/pagefind/`, `dashboard/public/`); regenerate them with `ask skills sync` or `ask dashboard lint --fix`.
- Do not edit files under `skills/vendor/` or flag vendor snapshots for local authoring conventions (punctuation, resource layout, cross-skill grouping, or `agents/openai.yaml`).
- Never run individual linters directly outside `devenv.nix` hooks, and never use standalone `pre-commit`.
- Never use em-dashes (Unicode U+2014) in custom Markdown, code comments, commit messages, or PR descriptions; use hyphens (`-`), colons, commas, or parentheses.
- Never use absolute `file:///` links to local directories inside repository files; use relative paths.
- Always specify a language tag on Markdown code fences (MD040) and surround every code fence with blank lines (MD031).
- Use Conventional Commits (`type(scope): description`) for commit messages and PR titles (`feat`, `fix`, `docs`, `refactor`, `test`, `ci`, `chore`), enforced by `convco` and `commitizen`.

## Code Review Focus

- Prioritize functional bugs, security or secret-leak risks, broken shell/CLI edge cases, and violations of repository invariants over style nits.
- Ignore generated output churn in `dashboard/content/`, `dashboard/static/.well-known/`, `README.md`, `agents/AGENTS.md`, and `skills.sh.json` except to confirm they stay in sync with source edits.
- When requesting changes to any custom skill under `skills/`, note that `dashboard/content/` and `dashboard/static/.well-known/` should be regenerated via `ask dashboard lint --fix`.
