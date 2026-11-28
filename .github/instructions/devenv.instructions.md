---
applyTo: "devenv.nix,devenv.yaml,.github/workflows/**"
---

# Devenv & CI Rules

## `devenv.nix` Style

- Use `with pkgs;` for package lists instead of repeating `pkgs.` on each entry.
- Define each top-level attribute set once using nested long-form blocks (for example `scripts = { ... };`, `git-hooks = { hooks = { ... }; };`). Never repeat dotted attribute paths (`scripts.foo.exec = ...;`), which trigger `statix` warning W20.
- Give every script declared under `scripts` a `description`.
- Format with `nixfmt` and keep `deadnix` and `statix` clean.

## Hooks & Commands

- Configure all linters and pre-commit hooks through `git-hooks` (`prek`) in `devenv.nix`. Never add `pkgs.pre-commit` or invoke the deprecated standalone `pre-commit` CLI.
- Always pass `--no-tui` to `devenv` commands in CI workflows and scripts (`devenv --no-tui test`, `devenv --no-tui shell -- ...`).
- Keep GitHub Actions workflows compliant with `actionlint` and `action-validator`.
