# Processes

Devenv’s built-in native process manager is the default and recommended
way to run your development stack. It provides supervision, socket
activation, file watching, readiness checks, and dependency management
without additional configuration.

## Basic Example

```
{ pkgs, ... }:
{  processes = {    silly-example.exec = "while true; do echo hello && sleep 1; done";    ping.exec = "ping localhost";    server = {      exec = "python -m http.server";      cwd = "./public";    };  };}
```

devenv.nix

To start the processes, run:

```
$ devenv up
```

Terminal window

To stop processes started in the background:

```
$ devenv down
```

Terminal window

With the native process manager, wait for all processes to become ready
(useful in CI):

```
$ devenv processes wait --timeout 120
```

Terminal window

The default timeout is 120 seconds.

## Attaching to running processes

This section describes the native process manager. External managers can
run in the background when they advertise that capability, but devenv
cannot attach its own live view or issue individual process-control
commands to them.

When native-managed processes are already running in the background
(started with `devenv up -d`), a second `devenv up` attaches to them
instead of failing. It starts any processes that are enabled but not
currently running, honoring their `after`/`before` dependencies, and
streams a live view of process status and logs. Press Ctrl-C to detach,
leaving the processes running.

An attaching `devenv up` reports which processes it scheduled and which
were already running, and exits nonzero when nothing could be started.

You can also pass a subset of processes to start:

```
$ devenv up -d            # start everything in the background$ devenv processes stop api$ devenv up api           # attach and bring api back up
```

Terminal window

A bare `devenv up` starts only processes with `start.enable = true`;
explicitly named processes always start, even when their `start.enable`
is `false`. The same applies to `devenv processes start <name>`, which
uses the same dependency-aware launch path: if a dependency is not
running, the process waits for it instead of starting without it. When
no process manager is running yet, `devenv processes start <name>`
starts one in the background launching only the named process, like
`devenv up -d <name>`.

The attached client is a non-interactive live view: stdin is not
connected to the processes, and Ctrl-C detaches while leaving them
running (the TUI restart/stop keybindings still work).

To attach a live view without starting anything (native process manager
only):

```
$ devenv processes attach
```

Terminal window

## Dependencies

Processes can depend on other processes and tasks using `after` and
`before`:

```
{  processes = {    database.exec = "postgres";
    api = {      exec = "myapi";      after = [ "devenv:processes:database" ];  # wait for database to be ready    };  };}
```

devenv.nix

Dependency suffixes control when a dependency is considered satisfied.

For **process** dependencies:

- `@started` — wait for the process to begin execution
- `@ready` (default) — wait for the readiness probe to pass
- `@completed` — wait for the process to finish, regardless of exit code
  (soft dependency, does not propagate failure)

For **task** dependencies:

- `@started` — wait for the task to begin execution
- `@succeeded` (default) — wait for the task to exit with code 0
- `@completed` — wait for the task to finish, regardless of exit code
  (soft dependency, does not propagate failure)

See [Dependency states](https://devenv.sh/tasks/#dependency-states) for the full
semantics, and [Execution modes](https://devenv.sh/tasks/#execution-modes) for how
`devenv up` and `devenv tasks run` decide which dependencies to
schedule.

## Using Pre-built Services

Devenv provides many pre-configured services with proper process
management. See the [Services documentation](https://devenv.sh/services/) for available
services like:

- [PostgreSQL](https://devenv.sh/services/postgres/)
- [Redis](https://devenv.sh/services/redis/)
- [MySQL](https://devenv.sh/services/mysql/)
- [MongoDB](https://devenv.sh/services/mongodb/)
- [Elasticsearch](https://devenv.sh/services/elasticsearch/)

These services come with sensible defaults, health checks, and proper
initialization scripts.

## Restart Policies

Control how processes restart when they exit:

- `on_failure` (default) - restart only on non-zero exit
- `always` - restart on any exit
- `never` - never restart

```
{  processes.worker = {    exec = "worker --queue jobs";    restart = {      on = "always";      max = 10;  # null for unlimited (default: 5)    };  };}
```

devenv.nix

## Shutdown

## Ready Probes

Ready probes let the process manager detect when a process is ready to
serve. This is used by `after` dependencies to know when a dependency is
available.

### Exec probe

Run a shell command to check readiness. Exit code 0 means ready:

```
{  processes.database = {    exec = "postgres -D $PGDATA";    ready = {      exec = "pg_isready -d template1";    };  };}
```

devenv.nix

### HTTP probe

Poll an HTTP endpoint for readiness:

```
{  processes.api = {    exec = "myserver";    ready = {      http.get = {        port = 8080;        path = "/health";        # host = "127.0.0.1";  # default        # scheme = "http";     # default      };    };  };}
```

devenv.nix

### Notify probe

Use systemd-style readiness notification. Your process should send
`READY=1` to the socket path in `$NOTIFY_SOCKET`:

```
{  processes.database = {    exec = "postgres";    ready.notify = true;  };
  processes.api = {    exec = "myapi";    after = [ "devenv:processes:database" ];  # waits for READY=1  };}
```

devenv.nix

### Probe timing options

All probe types support these timing options:

```
{  processes.api = {    exec = "myserver";    ready = {      http.get = { port = 8080; path = "/health"; };      initial_delay = 2;    # seconds before first probe (default: 0)      period = 10;           # seconds between probes (default: 10)      probe_timeout = 1;           # seconds before probe times out (default: 1)      success_threshold = 1; # consecutive successes needed (default: 1)      failure_threshold = 3; # consecutive failures before unhealthy (default: 3)      # timeout = ; Overall deadline in seconds for the process to become ready. null = no deadline.    };  };}
```

devenv.nix

When `listen` sockets or allocated `ports` are configured and no
explicit probe is set, a TCP connectivity check is used automatically.

## File Watching

Automatically restart processes when files change:

```
{  processes.backend = {    exec = "cargo run";    watch = {      paths = [ ./src ];      extensions = [ "rs" "toml" ];      ignore = [ "target" "*.log" ];    };  };}
```

devenv.nix

This works for both long-running processes and one-shot commands. A
long-running process (such as `cargo run`) is restarted on each change.
A one-shot command that exits immediately is re-run on each change — the
watcher stays active after the command exits.

```
{  # Prints a line every time a file in ./src changes.  processes.on-change = {    exec = "echo 'a file in ./src changed'";    watch = {      paths = [ ./src ];    };  };}
```

devenv.nix

## Socket Activation

Socket activation allows the process manager to bind sockets before
starting your process. This enables zero-downtime restarts and lazy
process startup.

```
{  processes.api = {    exec = "myserver";    listen = [      {        name = "http";        kind = "tcp";        address = "127.0.0.1:8080";      }      {        name = "admin";        kind = "unix_stream";        path = "$DEVENV_STATE/admin.sock";      }    ];  };}
```

devenv.nix

Your process receives these environment variables:

- `LISTEN_FDS` - number of passed file descriptors
- `LISTEN_PID` - PID that should accept the sockets
- `LISTEN_FDNAMES` - colon-separated socket names

File descriptors start at 3 (after stdin, stdout, stderr). This is
compatible with systemd socket activation.

## Watchdog

Enable systemd-compatible watchdog monitoring. Your process must
periodically send `WATCHDOG=1` to the notify socket, or it will be
killed and restarted:

```
{  processes.api = {    exec = "myserver";    ready.notify = true;    watchdog = {      usec = 30000000;      # 30 seconds      require_ready = true;  # only enforce after READY=1 (default)    };  };}
```

devenv.nix

## Git Integration

Processes can reference the git repository root path using
`${config.git.root}`, useful in monorepo environments:

```
{ config, ... }:
{  processes.frontend = {    exec = "npm run dev";    cwd = "${config.git.root}/frontend";  };
  processes.backend = {    exec = "cargo run";    cwd = "${config.git.root}/backend";  };}
```

devenv.nix

Processes are automatically available as tasks, allowing you to define
pre and post hooks. See the [Processes as
tasks](https://devenv.sh/tasks/#processes-as-tasks) section for details.

## Automatic port allocation

Devenv can automatically allocate free ports for your processes,
preventing conflicts when a port is already in use or when running
multiple devenv projects simultaneously.

Define ports using `ports.\<name\>.allocate` with a base port number.
Devenv will find a free port starting from that base, incrementing until
one is available:

```
{ config, ... }:
{  processes.server = {    ports.http.allocate = 8080;    ports.admin.allocate = 9000;    exec = ''      echo "HTTP server on port ${toString config.processes.server.ports.http.value}"      echo "Admin panel on port ${toString config.processes.server.ports.admin.value}"      python -m http.server ${toString config.processes.server.ports.http.value}    '';  };}
```

devenv.nix

The resolved port is available via
`config.processes.\<name\>.ports.\<port\>.value`. If port 8080 is already in
use, devenv will automatically try 8081, 8082, and so on until it finds
an available port.

Devenv holds the allocated ports during configuration evaluation to
prevent race conditions, then releases them just before starting the
processes so your application can bind to them.

This is particularly useful for:

- **Running multiple projects**: Each project gets its own ports without
  manual coordination
- **CI environments**: Tests can run in parallel without port conflicts
- **Shared development machines**: Multiple developers can run the same
  project simultaneously

### Strict port mode

If you want devenv to fail when a port is already in use instead of
automatically finding the next available port, you can set the default
in `devenv.yaml`:

```
strict_ports: true
```

Or override it for a single run with CLI flags:

```
$ devenv up --strict-ports$ devenv up --no-strict-ports
```

Terminal window

The CLI flags take precedence over the config value.

This is useful when you need deterministic port assignments and want to
be notified of conflicts rather than having them silently resolved. When
a port conflict is detected in strict mode, devenv will show an error
message including which process is currently using the port.

## Alternative process managers

The native manager is the best starting point and supports devenv’s
complete process feature set. If you have an existing workflow that
depends on a specific external manager, you can switch implementations:

- [process-compose](https://devenv.sh/supported-process-managers/process-compose/) -
  Feature-rich external process manager with TUI
- [overmind](https://devenv.sh/supported-process-managers/overmind/) - Procfile-based
  with tmux integration
- [honcho](https://devenv.sh/supported-process-managers/honcho/) - Python Foreman port
- [hivemind](https://devenv.sh/supported-process-managers/hivemind/) - Simple Procfile
  manager
- [mprocs](https://devenv.sh/supported-process-managers/mprocs/) - TUI process manager

To switch:

```
{  process.manager.implementation = "process-compose";}
```

devenv.nix

Selecting a manager does not imply that it supports every process
command. Each manager declares the lifecycle capabilities that devenv
may use, and the CLI rejects unsupported operations before starting the
manager.

| Manager | Background start | devenv attach | Wait ready | Individual control | Cold-start subset |
|----|----|----|----|----|----|
| native | Yes | Yes | Yes | Yes | Yes |
| process-compose | Yes | No | No | No | Yes |
| overmind | Yes | No | No | No | Yes |
| honcho | Yes | No | No | No | Yes |
| hivemind | Yes | No | No | No | No |
| mprocs | No | No | No | No | No |

The columns mean:

- **Background start** (`background_start`): `devenv up -d` can return
  while the manager and its processes remain running.
- **devenv attach** (`devenv_attach`): `devenv processes attach`, and
  devenv’s live attach behavior when `devenv up` finds a running
  manager.
- **Wait ready** (`wait_ready`): `devenv processes wait` can query
  readiness through that manager.
- **Individual control** (`individual_control`):
  `devenv processes start`, `stop`, and `restart` can control an
  existing manager by process name.
- **Cold-start subset** (`cold_start_subset`): a new manager can be
  started with selected names, for example `devenv up -d api worker`.

### Manager adapters

Capabilities answer which user-visible operations are available. Runtime
adapters separately describe how devenv hosts and stops each manager:

| Manager         | Terminal adapter | Stop adapter    | Client adapter |
|-----------------|------------------|-----------------|----------------|
| native          | `none`           | `native-api`    | `native-api`   |
| process-compose | `none`           | `process-scope` | `none`         |
| overmind        | `none`           | `command`       | `none`         |
| honcho          | `none`           | `process-scope` | `none`         |
| hivemind        | `none`           | `process-scope` | `none`         |
| mprocs          | `controlling`    | `process-scope` | `none`         |

The terminal adapters mean:

- `none`: the manager has no continuing controlling-terminal
  requirement.
- `controlling`: the manager must remain connected to a controlling
  terminal while it runs.

The stop adapters mean:

- `native-api`: devenv requests shutdown through its native manager
  control protocol.
- `command`: devenv invokes a manager-specific stop command, then
  performs final process-scope cleanup.
- `process-scope`: devenv terminates the recorded operating-system
  process scope directly and verifies that the manager and its
  descendants have exited.

The client adapter names the protocol used for attach, readiness, and
individual process control. `native-api` uses devenv’s native manager
socket; `none` means those capabilities must remain disabled. A future
external client protocol can be added as a new adapter without
conflating its transport with the operations it implements.

`devenv down` is supported for every manager that supports background
start. The stop adapter describes how that shutdown is performed; it is
not itself an optional operation capability.

mprocs currently requires a controlling terminal, so it is supported by
foreground `devenv up` but `devenv up -d` rejects it before spawning
anything. Background mprocs support would require a persistent
devenv-owned PTY that stays alive, drains output, and participates in
shutdown and recovery. Merely changing `background_start` to `true`
would not be sufficient.

Capabilities and adapters are internal implementation data rather than
additional public Nix options. When a newer CLI is used with older
devenv Nix modules that do not declare them, the CLI uses embedded
compatibility declarations for the known managers above. Unknown
managers receive no optional capabilities implicitly.

See [Alternative process managers](https://devenv.sh/supported-process-managers/) for the
tradeoffs and manager-specific options.
