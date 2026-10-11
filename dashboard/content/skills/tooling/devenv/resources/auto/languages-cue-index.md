+++
title = "languages-cue-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# cue

## Options

### languages.cue.enable

Whether to enable tools for Cue development.

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

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/cue.nix>

### languages.cue.package

The CUE package to use.

*Type:* package

*Default:*

```nix
pkgs.cue
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/cue.nix>

### languages.cue.lsp.enable

Whether to enable CUE Language Server.

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

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/cue.nix>

### languages.cue.lsp.package

The CUE language server package to use.

*Type:* package

*Default:*

```nix
pkgs.cuelsp
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/languages/cue.nix>
