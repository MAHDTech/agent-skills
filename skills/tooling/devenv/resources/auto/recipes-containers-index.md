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