+++
title = "languages-unison-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# unison

## Options

### languages.unison.enable

Whether to enable tools for Unison development.

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

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/unison.nix>

### languages.unison.package

Which package of Unison to use

*Type:* package

*Default:*

```nix
pkgs.unison-ucm
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/unison.nix>
