+++
title = "languages-kotlin-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# kotlin

## Options

### languages.kotlin.enable

Whether to enable tools for Kotlin development.

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

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/kotlin.nix>

### languages.kotlin.lsp.enable

Whether to enable Kotlin Language Server.

*Type:* boolean

*Default:*

```nix
true
```

*Example:*

```nix
true
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/kotlin.nix>

### languages.kotlin.lsp.package

The Kotlin language server package to use.

*Type:* package

*Default:*

```nix
pkgs.kotlin-language-server
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/kotlin.nix>
