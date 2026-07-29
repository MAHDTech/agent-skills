---
name: prek
description: Run pre-commit hooks using the prek CLI tool. Differentiates between 'pre-commit' (the git hooks concept/lifecycle stage) and 'prek' (the actual binary command; pre-commit CLI is deprecated, use prek). Use when you need to run, configure, check, or troubleshoot git pre-commit hooks.
---

# Prek (Pre-commit)

`prek` is a fast Git hook manager written in Rust, designed as a drop-in replacement for the deprecated `pre-commit` CLI tool.

> [!IMPORTANT]
> **Command Line & Package Discrepancy:** The CLI binary is `prek`, NOT `pre-commit`. The `pre-commit` CLI tool is deprecated and MUST NOT be used.

## Key Differences

- **Git concept / stage:** "pre-commit" refers strictly to the Git lifecycle hook stage (e.g. `.pre-commit-config.yaml`).
- **Legacy CLI Tool (DEPRECATED):** The Python-based `pre-commit` CLI tool is deprecated. **NEVER** add `pkgs.pre-commit` or `pre-commit` package/input to `devenv.nix` or package managers.
- **Active Hook Runner:** `prek` is the CLI binary used to run and configure pre-commit hooks.

## Running Hooks

When working inside a `devenv` shell, run:

```bash
SECRETSPEC_REASON="running pre-commit hooks" devenv --no-tui shell -- prek run -a
```

If not using `devenv`, run:

```bash
prek run -a
```

Use `-a` or `--all-files` to run hooks against all files in the repository.

## Commands

- `prek run`: Run hooks against modified files.
- `prek run -a`: Run hooks against all files.
- `prek install`: Install prek Git shims into Git's hooks directory.
- `prek uninstall`: Uninstall prek Git shims.
- `prek validate-config`: Validate configuration files (`prek.toml` or `.pre-commit-config.yaml`).
- `prek cache`: Manage the prek cache (e.g., `prek cache clean`).
