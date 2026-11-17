+++
title = "prek"
description = "Run, configure, or troubleshoot Git pre-commit hooks with prek. Use for hook execution, installation, validation, and cache management; distinguish the hook stage from its CLI."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Prek

`prek` is a Git hook manager written in Rust and a drop-in replacement for the Python `pre-commit` CLI. It reads the same `.pre-commit-config.yaml` (or its own `prek.toml`).

Two meanings of "pre-commit" appear in hook work:

- The Git hook stage that runs before a commit is created, and the `.pre-commit-config.yaml` file that configures hooks.
- The Python `pre-commit` CLI tool.

This skill's policy: run and manage hooks with `prek`, and add `prek` (not the Python `pre-commit` package) wherever a hook runner is installed.

## Running hooks

In a devenv project, run prek through the project's devenv shell so hooks find the project's tools. Elsewhere, run it directly:

```bash
prek run           # hooks against staged files
prek run -a        # hooks against every tracked file (--all-files)
```

## Commands

- `prek install` / `prek uninstall`: add or remove the Git hook shims.
- `prek validate-config`: check `prek.toml` or `.pre-commit-config.yaml`.
- `prek cache clean`: clear cached hook environments.

Run `prek --help` or `prek <command> --help` for the flags of the installed version.

## Completion criteria

- `prek run -a` exits 0, or every failing hook is reported by its id with the error output.

