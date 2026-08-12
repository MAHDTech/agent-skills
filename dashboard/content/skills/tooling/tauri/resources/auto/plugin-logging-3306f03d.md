+++
title = "plugin-logging-3306f03d"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Logging

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/log)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-log)[crates.io](https://crates.io/crates/tauri-plugin-log)

API
Reference[](https://v2.tauri.app/reference/javascript/log/)[](https://docs.rs/tauri-plugin-log)

Configurable logging for your Tauri app.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the log plugin to get started.

- [Automatic](#tab-panel-6424)
- [Manual](#tab-panel-6425)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6413)
- [yarn](#tab-panel-6414)
- [pnpm](#tab-panel-6415)
- [deno](#tab-panel-6416)
- [bun](#tab-panel-6417)
- [cargo](#tab-panel-6418)

```
npm run tauri add log
```

```
yarn run tauri add log
```

```
pnpm tauri add log
```

```
deno task tauri add log
```

```
bun tauri add log
```

```
cargo tauri add log
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-log
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_log::Builder::new().build())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6419)
    - [yarn](#tab-panel-6420)
    - [pnpm](#tab-panel-6421)
    - [deno](#tab-panel-6422)
    - [bun](#tab-panel-6423)

    ```
    npm install @tauri-apps/plugin-log
    ```

    ```
    yarn add @tauri-apps/plugin-log
    ```

    ```
    pnpm add @tauri-apps/plugin-log
    ```

    ```
    deno add npm:@tauri-apps/plugin-log
    ```

    ```
    bun add @tauri-apps/plugin-log
    ```

## Usage

1.  First, you need to register the plugin with Tauri.

    ```
    use tauri_plugin_log::{Target, TargetKind};
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_log::Builder::new().build())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

2.  Afterwards, all the plugin’s APIs are available through the
    JavaScript guest bindings:

    ```
    import {  warn,  debug,  trace,  info,  error,  attachConsole,  attachLogger,} from '@tauri-apps/plugin-log';// when using `"withGlobalTauri": true`, you may use// const { warn, debug, trace, info, error, attachConsole, attachLogger } = window.__TAURI__.log;
    ```

## Logging

- [JavaScript](#tab-panel-6411)
- [Rust](#tab-panel-6412)

Use one of the plugin’s `warn`, `debug`, `trace`, `info` or `error` APIs
to produce a log record from JavaScript code:

```
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
trace('Trace');info('Info');error('Error');
```

To automatically forward all `console` messages to the log plugin you
can rewrite them:

```
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
function forwardConsole(  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',  logger: (message: string) => Promise<void>) {  const original = console[fnName];  console[fnName] = (message) => {    original(message);    logger(message);  };}
forwardConsole('log', trace);forwardConsole('debug', debug);forwardConsole('info', info);forwardConsole('warn', warn);forwardConsole('error', error);
```

To create your own logs on the Rust side you can use the [`log`
crate](https://crates.io/crates/log):

```
log::error!("something bad happened!");log::info!("Tauri is awesome!");
```

Note that the [`log` crate](https://crates.io/crates/log) must be added
to your `Cargo.toml` file:

```
[dependencies]log = "0.4"
```

## Log targets

The log plugin builder has a `targets` function that lets you configure
common destination of all your application logs.

### Printing logs to the terminal

To forward all your logs to the terminal, enable the `Stdout` or
`Stderr` targets:

```
tauri_plugin_log::Builder::new()  .target(tauri_plugin_log::Target::new(    tauri_plugin_log::TargetKind::Stdout,  ))  .build()
```

This target is enabled by default.

### Logging to the webview console

To view all your Rust logs in the webview console, enable the `Webview`
target and run `attachConsole` in your frontend:

```
tauri_plugin_log::Builder::new()  .target(tauri_plugin_log::Target::new(    tauri_plugin_log::TargetKind::Webview,  ))  .build()
```

```
import { attachConsole } from '@tauri-apps/plugin-log';const detach = await attachConsole();// call detach() if you do not want to print logs to the console anymore
```

### Persisting logs

To write all logs to a file, you can use either the `LogDir` or the
`Folder` targets.

- `LogDir`:

```
tauri_plugin_log::Builder::new()  .target(tauri_plugin_log::Target::new(    tauri_plugin_log::TargetKind::LogDir {      file_name: Some("logs".to_string()),    },  ))  .build()
```

When using the LogDir target, all logs are stored in the recommended log
directory. The following table describes the location of the logs per
platform:

| Platform | Value | Example |
|----|----|----|
| Linux | `$XDG_DATA_HOME/{bundleIdentifier}/logs` or `$HOME/.local/share/{bundleIdentifier}/logs` | `/home/alice/.local/share/com.tauri.dev/logs` |
| macOS | `{homeDir}/Library/Logs/{bundleIdentifier}` | `/Users/Alice/Library/Logs/com.tauri.dev` |
| Windows | `{FOLDERID_LocalAppData}/{bundleIdentifier}/logs` | `C:\Users\Alice\AppData\Local\com.tauri.dev\logs` |

- `Folder`:

The Folder target lets you write logs to a custom location in the
filesystem.

```
tauri_plugin_log::Builder::new()  .target(tauri_plugin_log::Target::new(    tauri_plugin_log::TargetKind::Folder {      path: std::path::PathBuf::from("/path/to/logs"),      file_name: None,    },  ))  .build()
```

The default `file_name` is the application name.

#### Configuring log file behavior

By default the log file gets discarded when it reaches the maximum size.
The maximum file size can be configured via the builder’s
`max_file_size` function:

```
tauri_plugin_log::Builder::new()  .max_file_size(50_000 /* bytes */)  .build()
```

Tauri can automatically rotate your log file when it reaches the size
limit instead of discarding the previous file. This behavior can be
configured using `rotation_strategy`:

```
tauri_plugin_log::Builder::new()  .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)  .build()
```

### Filtering

By default **all** logs are processed. There are some mechanisms to
reduce the amount of logs and filter only relevant information.

### Maximum log level

To set a maximum log level, use the `level` function:

```
tauri_plugin_log::Builder::new()  .level(log::LevelFilter::Info)  .build()
```

In this example, debug and trace logs are discarded as they have a lower
level than *info*.

It is also possible to define separate maximum levels for individual
modules:

```
tauri_plugin_log::Builder::new()  .level(log::LevelFilter::Info)  // verbose logs only for the commands module  .level_for("my_crate_name::commands", log::LevelFilter::Trace)  .build()
```

Note that these APIs use the [`log`
crate](https://crates.io/crates/log), which must be added to your
`Cargo.toml` file:

```
[dependencies]log = "0.4"
```

### Target filter

A `filter` function can be defined to discard unwanted logs by checking
their metadata:

```
tauri_plugin_log::Builder::new()  // exclude logs with target `"hyper"`  .filter(|metadata| metadata.target() != "hyper")  .build()
```

### Formatting

The log plugin formats each log record as `DATE[TARGET][LEVEL] MESSAGE`.
A custom format function can be provided with `format`:

```
tauri_plugin_log::Builder::new()  .format(|out, message, record| {    out.finish(format_args!(      "[{} {}] {}",      record.level(),      record.target(),      message    ))  })  .build()
```

### Applying a different format to targets

You can specify your own log format for specific targets by using the
`format` method on `tauri_plugin_log::Target`. You may also wish to call
`clear_format` on the builder to remove the default formatter, which is
applied to all targets:

```
tauri_plugin_log::Builder::new()    .clear_format()    .targets([        tauri_plugin_log::Target::new(            tauri_plugin_log::TargetKind::Stdout        )        .format(move |out, message, record| {            // custom formatter for stdout        }),        tauri_plugin_log::Target::new(            tauri_plugin_log::TargetKind::LogDir { file_name: None }        )        .format(move |out, message, record| {            // custom formatter for log files        }),    ])    .build(),
```

#### Log dates

By default the log plugin uses the UTC timezone to format dates but you
can configure it to use the local timezone with `timezone_strategy`:

```
tauri_plugin_log::Builder::new()  .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)  .build()
```

## Permissions

By default, all plugin commands are blocked and cannot be accessed. You
must define a list of permissions in your `capabilities` configuration.

See the [Capabilities Overview](https://v2.tauri.app/security/capabilities/) for more
information and the [step by step
guide](https://v2.tauri.app/learn/security/using-plugin-permissions/) to use plugin
permissions.

```
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": ["log:default"]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

Allows the log command

#### This default permission set includes the following:

- `allow-log`

## Permission Table

| Identifier      | Description                                               |
|-----------------|-----------------------------------------------------------|
| `log:allow-log` | Enables the log command without any pre-configured scope. |
| `log:deny-log`  | Denies the log command without any pre-configured scope.  |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
