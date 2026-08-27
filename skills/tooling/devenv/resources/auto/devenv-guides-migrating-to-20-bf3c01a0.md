# Migrating to devenv 2.0

This guide covers the breaking changes in devenv 2.0 and how to update
your project.

## Native process manager is the default

devenv 2.0 replaces process-compose with a built-in Rust process
manager. If your processes work without process-compose-specific
configuration, no changes are needed — the native manager picks up
`processes.*` definitions as before.

If you depend on process-compose features or want to keep using it
during the transition:

```
{  process.manager.implementation = "process-compose";}
```

devenv.nix

The native manager supports port allocation, readiness probes, socket
activation, file watching, dependency ordering, watchdog heartbeats, and
Linux capabilities. See the [processes documentation](https://devenv.sh/processes/) for
details.

If there’s something process-compose does that the native manager
doesn’t yet cover, please [let us
know](https://github.com/cachix/devenv/issues).

### Migrating process-compose options

If you used `processes.\<name\>.process-compose` attributes, here’s how to
translate them to native equivalents.

#### Dependencies

process-compose uses `depends_on` with conditions. The native manager
uses `after` with lifecycle suffixes:

```
{  processes.api.process-compose = {    depends_on.postgres.condition = "process_healthy";    depends_on.migrations.condition = "process_completed_successfully";    depends_on.cleanup.condition = "process_completed";  };}
```

Before

```
{  processes.api.after = [    "devenv:processes:postgres"                  # waits for readiness probe (= process_healthy)    "devenv:processes:migrations@succeeded"      # waits for successful completion    "devenv:processes:cleanup@completed"          # waits for exit regardless of success  ];}
```

After

| process-compose condition | Native equivalent |
|----|----|
| `process_started` | `"devenv:processes:X@started"` |
| `process_healthy` | `"devenv:processes:X@ready"` or `"devenv:processes:X"` (default for processes; requires a `ready` probe on X) |
| `process_completed_successfully` | `"devenv:processes:X@succeeded"` |
| `process_completed` | `"devenv:processes:X@completed"` |

#### Restart policy

```
{  processes.api.process-compose = {    availability = {      restart = "on_failure";      backoff_seconds = 2;      max_restarts = 5;    };  };}
```

Before

```
{  processes.api.restart = {    on = "on_failure";  # "never" | "always" | "on_failure"    max = 5;    window = null;      # optional: sliding window in seconds for rate limiting  };}
```

After

Note: `backoff_seconds` has no native equivalent. The native manager
restarts immediately.

#### Environment variables and working directory

```
{  processes.api.process-compose = {    environment = [ "NODE_ENV=production" "PORT=3000" ];    working_dir = "/app";  };}
```

Before

```
{  processes.api = {    env = {      NODE_ENV = "production";      PORT = "3000";    };    cwd = "/app";  };}
```

After

#### Readiness probes

The `ready` option works with both managers, so if you already use it,
no changes are needed. If you used `process-compose.readiness_probe`
directly:

```
{  processes.api.process-compose = {    readiness_probe = {      exec.command = "curl -f http://localhost:8080/health";      period_seconds = 5;      failure_threshold = 3;    };  };}
```

Before

```
{  processes.api.ready = {    exec = "curl -f http://localhost:8080/health";    period = 5;    failure_threshold = 3;  };}
```

After

The native manager also supports HTTP probes and sd_notify:

```
{  # HTTP probe  processes.api.ready.http.get = { port = 8080; path = "/health"; };
  # sd_notify: process sends READY=1  processes.app.ready.notify = true;}
```

Native-only probe types

#### Liveness probes

process-compose supports `liveness_probe` separately from
`readiness_probe`. The native manager has no liveness probe — use
`watchdog` as an alternative for long-running health monitoring:

```
{  processes.api.process-compose = {    liveness_probe = {      exec.command = "check-alive";      period_seconds = 30;    };  };}
```

Before

```
{  processes.api = {    ready.notify = true;    watchdog = {      usec = 30000000;    # 30 seconds in microseconds      require_ready = true;    };  };}
```

After

The watchdog requires the process to send periodic `WATCHDOG=1`
heartbeats via `NOTIFY_SOCKET`. If your process doesn’t support
sd_notify, wrap it:

```
# In your exec script:while true; do systemd-notify WATCHDOG=1; sleep 10; done &exec myapp
```

Terminal window

#### Shutdown signal

```
{  processes.postgres.process-compose = {    shutdown.signal = 2;  # SIGINT  };}
```

Before

Set the signal on the process itself. The setting applies to both the
native manager and process-compose:

```
{  processes.postgres.shutdown.signal = 2;  # SIGINT}
```

After

!!! tip “New in version 2.2.3”

```
`processes.<name>.shutdown` was added in devenv 2.2.3.Older 2.x versions send SIGTERM; wrap the process in a script that translates the signal if you need a different one.
```

#### Elevated processes

```
{  processes.server.process-compose = {    is_elevated = true;  };}
```

Before

For specific privilege needs, use Linux capabilities instead:

```
{  processes.server.linux.capabilities = [ "net_bind_service" ];}
```

After

## git-hooks input is now optional

The `git-hooks` input is no longer included by default. If you use
`git-hooks.hooks` in your `devenv.nix`, add the input explicitly:

```
inputs:  git-hooks:    url: github:cachix/git-hooks.nix
```

devenv.yaml

If you don’t use git-hooks, no changes are needed.

The `pre-commit-hooks` to `git-hooks` alias has also been removed. If
you often switch between devenv v1.x and v2.x, add the
`pre-commit-hooks` input as well to prevent lockfile changes when
switching versions:

```
inputs:  pre-commit-hooks:    follows: git-hooks
```

## `pre-commit` renamed to `prek`

The `pre-commit` command has been replaced by `prek`, a Rust rewrite. If
you invoke `pre-commit` directly in scripts or shell commands, update
them to use `prek` instead:

```
# Beforepre-commit run --all-files
# Afterprek run --all-files
```

Terminal window

## `devenv build` returns JSON

`devenv build` now outputs JSON instead of plain store paths:

```
$ devenv build languages.rust.package{  "languages.rust.package": "/nix/store/...-rust-1.83.0"}
```

Terminal window

Update any scripts that parse the output. For example, if you previously
did:

```
store_path=$(devenv build languages.rust.package)
```

Terminal window

Use `jq` to extract the value:

```
store_path=$(devenv build languages.rust.package | jq -r '.["languages.rust.package"]')
```

Terminal window

## `devenv container` subcommand cleanup

`devenv container --copy <name>` has been removed. Use the subcommand
form instead:

```
$ devenv container copy <name>
```

Terminal window
