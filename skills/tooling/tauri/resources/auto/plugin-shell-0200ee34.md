# Shell

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/shell)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-shell)[crates.io](https://crates.io/crates/tauri-plugin-shell)

API
Reference:[](https://v2.tauri.app/reference/javascript/shell/)[](https://docs.rs/tauri-plugin-shell)

Access the system shell. Allows you to spawn child processes.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Opener

If you’re looking for documentation for the `shell.open` API, check out
the new [Opener plugin](opener-766df9a3.md) instead.

## Setup

Install the shell plugin to get started.

- [Automatic](#tab-panel-6159)
- [Manual](#tab-panel-6160)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6148)
- [yarn](#tab-panel-6149)
- [pnpm](#tab-panel-6150)
- [deno](#tab-panel-6151)
- [bun](#tab-panel-6152)
- [cargo](#tab-panel-6153)

```
npm run tauri add shell
```

```
yarn run tauri add shell
```

```
pnpm tauri add shell
```

```
deno task tauri add shell
```

```
bun tauri add shell
```

```
cargo tauri add shell
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-shell
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_shell::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6154)
    - [yarn](#tab-panel-6155)
    - [pnpm](#tab-panel-6156)
    - [deno](#tab-panel-6157)
    - [bun](#tab-panel-6158)

    ```
    npm install @tauri-apps/plugin-shell
    ```

    ```
    yarn add @tauri-apps/plugin-shell
    ```

    ```
    pnpm add @tauri-apps/plugin-shell
    ```

    ```
    deno add npm:@tauri-apps/plugin-shell
    ```

    ```
    bun add @tauri-apps/plugin-shell
    ```

## Usage

The shell plugin is available in both JavaScript and Rust.

- [JavaScript](#tab-panel-6146)
- [Rust](#tab-panel-6147)

```
import { Command } from '@tauri-apps/plugin-shell';// when using `"withGlobalTauri": true`, you may use// const { Command } = window.__TAURI__.shell;
let result = await Command.create('exec-sh', [  '-c',  "echo 'Hello World!'",]).execute();console.log(result);
```

```
use tauri_plugin_shell::ShellExt;
let shell = app_handle.shell();let output = tauri::async_runtime::block_on(async move {    shell        .command("echo")        .args(["Hello from Rust!"])        .output()        .await        .unwrap()});if output.status.success() {    println!("Result: {:?}", String::from_utf8(output.stdout));} else {    println!("Exit with code: {}", output.status.code().unwrap());}
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
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": [    {      "identifier": "shell:allow-execute",      "allow": [        {          "name": "exec-sh",          "cmd": "sh",          "args": [            "-c",            {              "validator": "\\S+"            }          ],          "sidecar": false        }      ]    }  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set configures which shell functionality is exposed by
default.

#### [Granted Permissions](#granted-permissions)

It allows to use the `open` functionality with a reasonable scope
pre-configured. It will allow opening `http(s)://`, `tel:` and `mailto:`
links.

#### This default permission set includes the following:

- `allow-open`

## Permission Table

| Identifier | Description |
|----|----|
| `shell:allow-execute` | Enables the execute command without any pre-configured scope. |
| `shell:deny-execute` | Denies the execute command without any pre-configured scope. |
| `shell:allow-kill` | Enables the kill command without any pre-configured scope. |
| `shell:deny-kill` | Denies the kill command without any pre-configured scope. |
| `shell:allow-open` | Enables the open command without any pre-configured scope. |
| `shell:deny-open` | Denies the open command without any pre-configured scope. |
| `shell:allow-spawn` | Enables the spawn command without any pre-configured scope. |
| `shell:deny-spawn` | Denies the spawn command without any pre-configured scope. |
| `shell:allow-stdin-write` | Enables the stdin_write command without any pre-configured scope. |
| `shell:deny-stdin-write` | Denies the stdin_write command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
