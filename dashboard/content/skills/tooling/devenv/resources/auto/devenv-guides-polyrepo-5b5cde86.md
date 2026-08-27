+++
title = "devenv-guides-polyrepo-5b5cde86"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Polyrepos

When working with multiple projects across separate repositories, you
may want to compose environments or reference options — such as outputs,
packages, or other configuration — defined in one devenv project from
another.

There are two approaches:

- **[Composing with imports](#composing-with-imports)** — merge an
  entire project’s configuration (packages, services, env, etc.) into
  your environment.
- **[Referencing config across
  inputs](#referencing-config-across-inputs)** — access specific config
  from another project without merging everything.

Both examples below reference the same remote repository
(`myorg/my-service`) with the following configuration:

```
{ config, ... }: {  languages.python.enable = true;
  outputs.my-service = config.languages.python.import ./. {};
  processes.my-service.exec = "${config.outputs.my-service}/bin/my-service";}
```

my-service/devenv.nix

## Composing with imports

devenv projects compose naturally through imports. When you import
another project via an input, all of its configuration — packages,
services, outputs, env, and more — merges into your environment.

Add the remote repository as an input, then import from it:

```
inputs:  my-service:    url: github:myorg/my-serviceimports:  - my-service
```

devenv.yaml

Any configuration defined in the imported project’s `devenv.nix` merges
into your environment. For example, `my-service`’s output and process
are now available via `config`:

```
{ config, ... }: {  # my-service's output is merged into config.outputs  packages = [ config.outputs.my-service ];
  # my-service's process is also merged, so `devenv up` will start it}
```

devenv.nix

For local cross-project imports (monorepos), see the [monorepo
guide](https://devenv.sh/guides/monorepo/).

## Referencing config across inputs

When you don’t want to merge an entire environment but need access to
specific options from another project, you can reference them through
`inputs.\<name\>.devenv.config`. This is particularly useful for consuming
[outputs](https://devenv.sh/outputs/) defined in other projects.

```
inputs:  my-service:    url: github:myorg/my-service    flake: false
```

devenv.yaml

```
{ inputs, ... }: {  packages = [    inputs.my-service.devenv.config.outputs.my-service  ];
  processes.my-service.exec = "${inputs.my-service.devenv.config.outputs.my-service}/bin/my-service";}
```

devenv.nix

