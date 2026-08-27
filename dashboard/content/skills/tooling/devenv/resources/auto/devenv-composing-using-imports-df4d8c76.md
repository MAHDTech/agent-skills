+++
title = "devenv-composing-using-imports-df4d8c76"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Composing using imports

You can compose environments either locally or by referencing
[inputs](https://devenv.sh/inputs/).

Imagine you’re building a typical web application, with separate
frontend and backend components that live in separate folders.

```
inputs:  nixpkgs:    url: github:cachix/devenv-nixpkgs/rolling  devenv:    url: github:cachix/devenv    flake: falseimports:- ./frontend- ./backend- devenv/examples/supported-languages- devenv/examples/scripts
```

devenv.yaml

If you enter the `frontend` directory, the environment will activate
based on what’s in the `frontend/devenv.nix` file.

If you enter the top-level project, the environment is combined with
what’s defined in `backend/devenv.nix` and `frontend/devenv.nix`. For
example, `devenv up` will start both the frontend and backend processes.

## Sharing configuration from another repository

To keep your devenv configuration in a separate repository, for example
when working on a team that doesn’t use devenv, declare it as a `path:`
input and import it:

```
inputs:  shared-config:    url: path:../shared-config/    flake: falseimports:- shared-config
```

devenv.yaml

The sibling `shared-config` repository only needs a `devenv.nix` file.
Combine this with [profiles](https://devenv.sh/profiles/) to define one shared
configuration that adapts to each project.

See [devenv.yaml reference](https://devenv.sh/reference/yaml-options/#imports) for all
supported import options.

