---
name: prek
description: Run pre-commit hooks using the prek CLI tool. Differentiates between 'pre-commit' (the git hooks concept/lifecycle stage) and 'prek' (the actual binary command). Use when you need to run, configure, check, or troubleshoot git pre-commit hooks.
---

# Prek (Pre-commit)

`prek` is a fast Git hook manager written in Rust, designed as a drop-in alternative to `pre-commit`.

> [!IMPORTANT]
> **Command Line Discrepancy:** The actual binary name is `prek`, NOT `pre-commit`. Always use `prek` to invoke the CLI.

## Key Differences

- **Git concept / hooks:** "pre-commit" refers to the Git lifecycle hook stage.
- **The tool:** `prek` is the command-line interface used to run and configure these hooks.

## Running Hooks

When working inside a `devenv` shell, run:

```bash
SECRETSPEC_REASON="running pre-commit hooks" devenv shell -- prek run -a
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
