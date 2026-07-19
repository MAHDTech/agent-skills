+++
title = "devenv-composing-using-imports-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Composing using imports

You can compose environments either locally or by referencing [inputs](https://devenv.sh/inputs/index.md).

Imagine you're building a typical web application, with separate frontend and backend components that live in separate folders.

devenv.yaml

```
inputs:
  nixpkgs:
    url: github:cachix/devenv-nixpkgs/rolling
  devenv:
    url: github:cachix/devenv
    flake: false
imports:
- ./frontend
- ./backend
- devenv/examples/supported-languages
- devenv/examples/scripts
```

If you enter the `frontend` directory, the environment will activate based on what's in the `frontend/devenv.nix` file.

If you enter the top-level project, the environment is combined with what's defined in `backend/devenv.nix` and `frontend/devenv.nix`. For example, `devenv up` will start both the frontend and backend processes.

Added in 1.10

Composing `devenv.yaml` files is now supported for local files (relative and absolute paths). Remote inputs are not yet supported for `devenv.yaml` imports.

See [devenv.yaml reference](https://devenv.sh/reference/yaml-options/#imports) for all supported import options.
