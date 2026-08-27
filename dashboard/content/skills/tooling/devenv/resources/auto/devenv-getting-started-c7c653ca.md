+++
title = "devenv-getting-started-c7c653ca"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Getting Started

## Installation

### 1. Install [Nix](https://nixos.org)

- [Linux](#tab-panel-76)
- [macOS](#tab-panel-77)
- [Windows (WSL2)](#tab-panel-78)
- [Docker](#tab-panel-79)

```
sh <(curl -L https://nixos.org/nix/install) --daemon
```

```
curl -sSfL https://artifacts.nixos.org/nix-installer | sh -s -- install
```

!!! note “Nix installer” We recommend using the above installer. It can
handle OS upgrades and has better support for Apple silicon.

If you’d like to stick with the classic installer, use:

```
sh <(curl -L https://nixos.org/nix/install)
```

**Upgrade Bash**

macOS ships with an ancient version of Bash due to licensing reasons.

We recommend installing a newer version from nixpkgs to avoid running
into evaluation errors.

=== “Nix env (newcomers)”

```
nix-env --install --attr bashInteractive -f https://github.com/NixOS/nixpkgs/tarball/nixpkgs-unstable
```

=== “Nix profiles (requires experimental flags)”

```
nix profile install nixpkgs#bashInteractive
```

```
sh <(curl -L https://nixos.org/nix/install) --no-daemon
```

```
docker run -it nixos/nix
```

### 2. Install [devenv](https://github.com/cachix/devenv)

- [Newcomers](#tab-panel-80)
- [Nix profiles (requires experimental flags)](#tab-panel-81)
- [NixOS/nix-darwin](#tab-panel-82)
- [home-manager](#tab-panel-83)

```
nix-env --install --attr devenv -f https://github.com/NixOS/nixpkgs/tarball/nixpkgs-unstable
```

```
nix profile install nixpkgs#devenv
```

```
environment.systemPackages = [  pkgs.devenv];
```

configuration.nix

```
home.packages = [  pkgs.devenv];
```

home.nix

### 3. Configure a GitHub access token (optional)

The Nix ecosystem is heavily dependent on GitHub for hosting and
distributing source code, like the source for nixpkgs. This means that
Nix will make a lot of un-authenticated requests to the GitHub API and
you may encounter rate-limiting.

To avoid being rate-limited, **we recommend providing Nix with a GitHub
access token**, which will greatly increase your API limits.

Create a new token with no extra permissions at
<https://github.com/settings/personal-access-tokens/new>. Add the token
to your `~/.config/nix/nix.conf`:

```
access-tokens = github.com=<GITHUB_TOKEN>
```

## Initial set up

Initialize a new developer environment with `devenv init`.

```
$ devenv init• Creating devenv.nix• Creating devenv.yaml• Creating .gitignore
```

Terminal window

## Commands

### Develop

- `devenv init` scaffolds a new project with `devenv.yaml`,
  `devenv.nix`, and `.gitignore`.
- `devenv shell` activates your developer environment.
- `devenv up` starts [processes](https://devenv.sh/processes/).
- `devenv processes down` stops background processes.
- `devenv tasks run <task>` runs [tasks](https://devenv.sh/tasks/).
- `devenv test` builds your developer environment and makes sure that
  all checks pass. Useful to run in your continuous integration
  environment.
- `devenv container build|copy|run` manages [containers](https://devenv.sh/containers/).

### Packages & Dependencies

- `devenv search <NAME>` searches packages matching NAME in Nixpkgs
  input.
- `devenv update` updates and pins inputs from `devenv.yaml` into
  `devenv.lock`.
- `devenv inputs add \<name\> <url>` adds an input to `devenv.yaml`.

### Inspect & Debug

- `devenv info` prints environment information.
- `devenv eval <attr>` evaluates attributes and returns JSON.
- `devenv build <attr>` builds attributes from `devenv.nix`.
- `devenv repl` launches an interactive Nix REPL for inspecting the
  environment.

### Maintenance & Tooling

- `devenv gc` [deletes unused environments](https://devenv.sh/garbage-collection/) to
  save disk space.
- `devenv lsp` starts the language server for `devenv.nix`.
- `devenv mcp` launches the MCP server for AI assistants.

## Learn more

- About `devenv.yaml` in [Inputs](https://devenv.sh/inputs/) and [Composing using
  imports](https://devenv.sh/composing-using-imports/).
- About `devenv.nix` in the **Writing devenv.nix** section, starting
  with [the basics](https://devenv.sh/basics/).

## Updating

### Update devenv CLI

- [Nix env (newcomers)](#tab-panel-84)
- [Nix profiles (requires experimental flags)](#tab-panel-85)
- [NixOS/nix-darwin/home-manager](#tab-panel-86)

```
nix-env --upgrade --attr devenv -f https://github.com/NixOS/nixpkgs/tarball/nixpkgs-unstable
```

```
nix profile upgrade devenv
```

Update nixpkgs to get the latest version of devenv.

For detailed upgrade instructions specific to your setup, please refer
to the documentation for your particular system: NixOS, nix-darwin (for
macOS), or home-manager, as applicable.

### Update project inputs

Inputs, like nixpkgs and devenv modules, are downloaded and pinned in a
`devenv.lock` lockfile.

These should be periodically updated with:

```
devenv update
```

Learn more about [Inputs](https://devenv.sh/inputs/).

## Show your support

Add a badge to your project’s README to show it’s built with devenv:

[![Built with devenv](https://devenv.sh/assets/devenv-badge.svg)](https://devenv.sh)

```
[![Built with devenv](https://devenv.sh/assets/devenv-badge.svg)](https://devenv.sh)
```

