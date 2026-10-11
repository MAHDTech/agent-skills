+++
title = "recipes-containers-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Containers

## Container patterns

### Exclude packages from a container

**devenv.nix**

```nix
{ pkgs, lib, config, ... }: {
  packages = [
    pkgs.git
  ] ++ lib.optionals (!config.container.isBuilding) [
    pkgs.haskell-language-server
  ];
}
```
