+++
title = "devenv-services-eb5f62b8"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Overview

# Services

Services are a higher-level abstraction over [processes](https://devenv.sh/processes/).
While processes provide low-level control for running any command,
services offer pre-configured interfaces for existing software like
databases.

Here’s an example starting PostgreSQL with a few extensions:

```
{ pkgs, ... }:
{  services.postgres = {    enable = true;    package = pkgs.postgresql_15;    initialDatabases = [{ name = "mydb"; }];    extensions = extensions: [      extensions.postgis      extensions.timescaledb    ];    settings.shared_preload_libraries = "timescaledb";    initialScript = "CREATE EXTENSION IF NOT EXISTS timescaledb;";  };}
```

devenv.nix

Services start like processes with `devenv up`:

```
$ devenv upStarting processes ...
```

Terminal window

Service states are persisted to directories in `$DEVENV_STATE`. When you
adjust options like the above used `initialScript`, you will have to
delete the service’s directory for changes to take effect on next
`devenv up`.

## Services in the background

Services start in the foreground by default. If you want to start
services up in the background, you can pass the `-d` flag:

```
$ devenv up -d
```

Terminal window

