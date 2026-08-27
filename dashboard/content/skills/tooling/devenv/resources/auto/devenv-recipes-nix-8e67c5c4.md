+++
title = "devenv-recipes-nix-8e67c5c4"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Nix

## Nix patterns

### Getting a recent version of a package from `nixpkgs-unstable`

By default, new devenv projects are configured to use a fork of
`nixpkgs` called
[`devenv-nixpkgs/rolling`](https://github.com/cachix/devenv-nixpkgs).

`devenv-nixpkgs/rolling` is tested against devenv’s test suite and
receives monthly updates, as well as interim stability patches that
affect devenv’s most popular services and integrations.

For some packages that are updated frequently, you may want to use a
more recent version from `nixpkgs-unstable`.

```
inputs:  nixpkgs:    url: github:cachix/devenv-nixpkgs/rolling  nixpkgs-unstable:    url: github:NixOS/nixpkgs/nixpkgs-unstable
```

devenv.yaml

```
{ pkgs, inputs, ... }:let  pkgs-unstable = import inputs.nixpkgs-unstable { system = pkgs.stdenv.system; };in{  packages = [    pkgs-unstable.elmPackages.elm-test-rs  ];}
```

devenv.nix

### How do I contribute a package or a fix to `nixpkgs`?

For temporary fixes, we recommend using either [overlays](https://devenv.sh/overlays/) or
a [different nixpkgs
input](#getting-a-recent-version-of-a-package-from-nixpkgs-unstable).

You can also consider contributing your changes back to
[`nixpkgs`](https://github.com/NixOS/nixpkgs). Follow the [nixpkgs
contributing
guide](https://github.com/NixOS/nixpkgs/blob/master/pkgs/README.md) to
get started.

Once you’ve forked and cloned `nixpkgs`, test your changes with devenv:

```
inputs:  nixpkgs:    url: github:username/nixpkgs/branch    # Or a local path to nixpkgs    # url: path:/path/to/local/nixpkgs/clone
```

### Add a directory to `$PATH`

This example adds Elixir install scripts to `~/.mix/escripts`:

```
{ ... }:
{  languages.elixir.enable = true;
  enterShell = ''    export PATH="$HOME/.mix/escripts:$PATH"  '';}
```

devenv.nix

### Skip the C compiler toolchain

If your project doesn’t need a C compiler toolchain, set
[`stdenv`](https://devenv.sh/reference/options/#stdenv) to `pkgs.stdenvNoCC` to drop it
from the shell. This saves a few hundred MB of storage and shell startup
time:

```
{ pkgs, ... }: {  stdenv = pkgs.stdenvNoCC;}
```

devenv.nix

This is equivalent to using nixpkgs’ `mkShellNoCC` instead of `mkShell`.

### Escape Nix curly braces inside shell scripts

```
{ pkgs, ... }: {  scripts.myscript.exec = ''    foobar=1    echo ''${foobar}  '';}
```

devenv.nix

