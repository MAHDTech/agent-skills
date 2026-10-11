+++
title = "languages-deno-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# deno

## Options

### languages.deno.enable

Whether to enable tools for Deno development.

*Type:* boolean

*Default:*

```nix
false
```

*Example:*

```nix
true
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/deno.nix>

### languages.deno.package

Which package of Deno to use.

*Type:* package

*Default:*

```nix
pkgs.deno
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/deno.nix>
