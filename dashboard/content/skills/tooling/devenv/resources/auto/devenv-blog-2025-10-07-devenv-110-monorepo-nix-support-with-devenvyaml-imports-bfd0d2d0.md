+++
title = "devenv-blog-2025-10-07-devenv-110-monorepo-nix-support-with-devenvyaml-imports-bfd0d2d0"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# devenv 1.10: monorepo Nix support with devenv.yaml imports

Oct 7, 2025

[![Domen
Kožar](https://github.com/domenkozar.png)](https://github.com/domenkozar)

Domen Kožar

[devenv 1.10](https://github.com/cachix/devenv/releases/tag/v1.10)
brings new capabilities for structuring monorepo projects:

- [absolute/parent path
  imports](https://devenv.sh/blog/2025/10/07/devenv-110-monorepo-nix-support-with-devenvyaml-imports/#absolute--parent-path-imports)
- [git root prefixing](#git-root-prefixing)
- [devenv.yaml imports](#devenvyaml-imports)
- [devenv.local.yaml support](#devenvlocalyaml-support)
- [monorepo guide](#monorepo-guide)

## Absolute / parent path imports

Paths starting with `/` are now resolved from your git repository root,
and parent imports are also supported
([\#998](https://github.com/cachix/devenv/issues/998)).

This lets services consistently reference shared configurations:

```
imports:  - /nix/devenv.nix  - ../api/devenv.nix
```

services/worker/devenv.yaml

This is particularly handy in monorepos where projects are nested at
different depths:

```
my-monorepo/├── nix/│   └── devenv.nix       # Shared base configuration├── services/│   ├── api/│   │   └── devenv.yaml  # imports: [/nix]│   └── worker/│       └── devenv.yaml  # imports: [/nix]└── apps/    └── web/        └── devenv.yaml  # imports: [/nix]
```

All three projects reference `/nix` regardless of their location.

## Git root prefixing

The new `config.git.root` variable provides the git repository root path
for specifying working directories in tasks and processes
([\#1850](https://github.com/cachix/devenv/issues/1850),
[\#316](https://github.com/cachix/devenv/issues/316)).

```
{ config, ... }: {  tasks."db:migrate" = {    exec = "npm run migrate";    cwd = "${config.git.root}/services/api";  };
  processes.api = {    exec = "npm start";    cwd = "${config.git.root}/services/api";  };}
```

services/api/devenv.nix

Useful when reusing modules across different directories.

## devenv.yaml imports

Most upvoted feature with 75 votes
([\#14](https://github.com/cachix/devenv/issues/14)) is here!

Local imports now load and merge both `devenv.nix` and `devenv.yaml`
configurations:

```
allowUnfree: trueinputs:  nixpkgs:    url: github:NixOS/nixpkgs/nixpkgs-unstable
```

shared/devenv.yaml

```
imports:  - /shared
```

services/api/devenv.yaml

The API service inherits the `allowUnfree` setting and nixpkgs input.
Note that this merging only applies to local filesystem imports —
imports from inputs still only load Nix configurations
([\#2205](https://github.com/cachix/devenv/issues/2205)).

## devenv.local.yaml support

Just like `devenv.local.nix`, you can now use `devenv.local.yaml` for
developer-specific overrides
([\#817](https://github.com/cachix/devenv/issues/817)).

Both files are git-ignored for local overrides:

```
allowUnfree: true
```

devenv.local.yaml

## Monorepo guide

Check out the new [Monorepo Guide](https://devenv.sh/guides/monorepo/) for detailed
examples and patterns.

Join the [devenv community](https://discord.gg/naMgvexb6q) to share your
monorepo experience!

Domen

