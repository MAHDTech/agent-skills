+++
title = "devenv-services-adminer-ee48038d"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# adminer

## Options

### services.adminer.enable

Whether to enable Adminer process.

*Type:* boolean

*Default:*

```
false
```

*Example:*

```
true
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/services/adminer.nix>

### services.adminer.package

Which package of Adminer to use.

*Type:* package

*Default:*

```
pkgs.adminer
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/services/adminer.nix>

### services.adminer.listen

Listen address for the Adminer.

*Type:* string

*Default:*

```
"127.0.0.1:8080"
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/services/adminer.nix>

