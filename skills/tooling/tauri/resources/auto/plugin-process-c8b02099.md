# Process

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/process)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-process)[crates.io](https://crates.io/crates/tauri-plugin-process)

API
Reference[](https://v2.tauri.app/reference/javascript/process/)[](https://docs.rs/tauri-plugin-process)

This plugin provides APIs to access the current process. To spawn child
processes, see the [shell](https://v2.tauri.app/plugin/shell/) plugin.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the plugin-process to get started.

- [Automatic](#tab-panel-6525)
- [Manual](#tab-panel-6526)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6514)
- [yarn](#tab-panel-6515)
- [pnpm](#tab-panel-6516)
- [deno](#tab-panel-6517)
- [bun](#tab-panel-6518)
- [cargo](#tab-panel-6519)

```
npm run tauri add process
```

```
yarn run tauri add process
```

```
pnpm tauri add process
```

```
deno task tauri add process
```

```
bun tauri add process
```

```
cargo tauri add process
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-process
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_process::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  If you’d like to utilize the plugin in JavaScript then install the
    npm package as well:

    - [npm](#tab-panel-6520)
    - [yarn](#tab-panel-6521)
    - [pnpm](#tab-panel-6522)
    - [deno](#tab-panel-6523)
    - [bun](#tab-panel-6524)

    ```
    npm install @tauri-apps/plugin-process
    ```

    ```
    yarn add @tauri-apps/plugin-process
    ```

    ```
    pnpm add @tauri-apps/plugin-process
    ```

    ```
    deno add npm:@tauri-apps/plugin-process
    ```

    ```
    bun add @tauri-apps/plugin-process
    ```

## Usage

The process plugin is available in both JavaScript and Rust.

- [JavaScript](#tab-panel-6512)
- [Rust](#tab-panel-6513)

```
import { exit, relaunch } from '@tauri-apps/plugin-process';// when using `"withGlobalTauri": true`, you may use// const { exit, relaunch } = window.__TAURI__.process;
// exits the app with the given status codeawait exit(0);
// restarts the appawait relaunch();
```

Note that `app` is an instance of
[`AppHandle`](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html).

```
// exits the app with the given status codeapp.exit(0);
// restarts the appapp.restart();
```

## Permissions

By default all potentially dangerous plugin commands and scopes are
blocked and cannot be accessed. You must modify the permissions in your
`capabilities` configuration to enable these.

See the [Capabilities Overview](https://v2.tauri.app/security/capabilities/) for more
information and the [step by step
guide](https://v2.tauri.app/learn/security/using-plugin-permissions/) to use plugin
permissions.

```
{  "permissions": [    ...,    "process:default",  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set configures which process features are by default
exposed.

#### [Granted Permissions](#granted-permissions)

This enables to quit via `allow-exit` and restart via `allow-restart`
the application.

#### This default permission set includes the following:

- `allow-exit`
- `allow-restart`

## Permission Table

| Identifier | Description |
|----|----|
| `process:allow-exit` | Enables the exit command without any pre-configured scope. |
| `process:deny-exit` | Denies the exit command without any pre-configured scope. |
| `process:allow-restart` | Enables the restart command without any pre-configured scope. |
| `process:deny-restart` | Denies the restart command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
