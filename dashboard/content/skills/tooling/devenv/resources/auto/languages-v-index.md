+++
title = "languages-v-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# v

## Options

### languages.v.enable

Whether to enable tools for V development.

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

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/v.nix>

### languages.v.package

The V package to use.

*Type:* package

*Default:*

```nix
pkgs.vlang
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/v.nix>
