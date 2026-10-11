+++
title = "services-httpbin-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# httpbin

## Options

### services.httpbin.enable

Whether to enable httpbin.

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

* <https://github.com/cachix/devenv/blob/main/src/modules/services/httpbin.nix>

### services.httpbin.bind

Addresses for httpbin to listen on.

*Type:* list of string

*Default:*

```nix
[
  "127.0.0.1:8080"
]
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/services/httpbin.nix>

### services.httpbin.extraArgs

Gunicorn CLI arguments for httpbin.

*Type:* list of string

*Default:*

```nix
[ ]
```

*Declared by:*

* <https://github.com/cachix/devenv/blob/main/src/modules/services/httpbin.nix>
