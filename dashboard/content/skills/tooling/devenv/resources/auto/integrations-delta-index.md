+++
title = "integrations-delta-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Delta

To get started using [Delta, a syntax-highlighting pager for git, diff, and grep output](https://dandavison.github.io/delta/), flip a toggle:

**devenv.nix**

```nix
{ pkgs, ... }:


{
    delta.enable = true;
}
```
