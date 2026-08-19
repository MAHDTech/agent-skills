# Autostart

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/autostart)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-autostart)[crates.io](https://crates.io/crates/tauri-plugin-autostart)

API
Reference:[](https://v2.tauri.app/reference/javascript/autostart/)[](https://docs.rs/tauri-plugin-autostart)

Automatically launch your application at system startup.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the autostart plugin to get started.

- [Automatic](#tab-panel-5863)
- [Manual](#tab-panel-5864)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-5852)
- [yarn](#tab-panel-5853)
- [pnpm](#tab-panel-5854)
- [deno](#tab-panel-5855)
- [bun](#tab-panel-5856)
- [cargo](#tab-panel-5857)

```
npm run tauri add autostart
```

```
yarn run tauri add autostart
```

```
pnpm tauri add autostart
```

```
deno task tauri add autostart
```

```
bun tauri add autostart
```

```
cargo tauri add autostart
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-autostart --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .setup(|app| {            #[cfg(desktop)]            app.handle().plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"]) /* arbitrary number of args to pass to your app */));            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  You can install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-5858)
    - [yarn](#tab-panel-5859)
    - [pnpm](#tab-panel-5860)
    - [deno](#tab-panel-5861)
    - [bun](#tab-panel-5862)

    ```
    npm install @tauri-apps/plugin-autostart
    ```

    ```
    yarn add @tauri-apps/plugin-autostart
    ```

    ```
    pnpm add @tauri-apps/plugin-autostart
    ```

    ```
    deno add npm:@tauri-apps/plugin-autostart
    ```

    ```
    bun add @tauri-apps/plugin-autostart
    ```

## Usage

The autostart plugin is available in both JavaScript and Rust.

- [JavaScript](#tab-panel-5850)
- [Rust](#tab-panel-5851)

```
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';// when using `"withGlobalTauri": true`, you may use// const { enable, isEnabled, disable } = window.__TAURI__.autostart;
// Enable autostartawait enable();// Check enable stateconsole.log(`registered for autostart? ${await isEnabled()}`);// Disable autostartdisable();
```

```
#[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .setup(|app| {            #[cfg(desktop)]            {                use tauri_plugin_autostart::MacosLauncher;                use tauri_plugin_autostart::ManagerExt;
                app.handle().plugin(tauri_plugin_autostart::init(                    MacosLauncher::LaunchAgent,                    Some(vec!["--flag1", "--flag2"]),                ));
                // Get the autostart manager                let autostart_manager = app.autolaunch();                // Enable autostart                let _ = autostart_manager.enable();                // Check enable state                println!("registered for autostart? {}", autostart_manager.is_enabled().unwrap());                // Disable autostart                let _ = autostart_manager.disable();            }            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
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
{  "permissions": [    ...,    "autostart:allow-enable",    "autostart:allow-disable",    "autostart:allow-is-enabled"  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set configures if your application can enable or disable
auto starting the application on boot.

#### [Granted Permissions](#granted-permissions)

It allows all to check, enable and disable the automatic start on boot.

#### This default permission set includes the following:

- `allow-enable`
- `allow-disable`
- `allow-is-enabled`

## Permission Table

| Identifier | Description |
|----|----|
| `autostart:allow-disable` | Enables the disable command without any pre-configured scope. |
| `autostart:deny-disable` | Denies the disable command without any pre-configured scope. |
| `autostart:allow-enable` | Enables the enable command without any pre-configured scope. |
| `autostart:deny-enable` | Denies the enable command without any pre-configured scope. |
| `autostart:allow-is-enabled` | Enables the is_enabled command without any pre-configured scope. |
| `autostart:deny-is-enabled` | Denies the is_enabled command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
