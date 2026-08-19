+++
title = "plugin-positioner-41968814"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Positioner

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/positioner)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-positioner)[crates.io](https://crates.io/crates/tauri-plugin-positioner)

API
Reference:[](https://v2.tauri.app/reference/javascript/positioner/)[](https://docs.rs/tauri-plugin-positioner)

Position your windows at well-known locations.

This plugin is a port of
[electron-positioner](https://github.com/jenslind/electron-positioner)
for Tauri.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the positioner plugin to get started.

- [Automatic](#tab-panel-6129)
- [Manual](#tab-panel-6130)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6118)
- [yarn](#tab-panel-6119)
- [pnpm](#tab-panel-6120)
- [deno](#tab-panel-6121)
- [bun](#tab-panel-6122)
- [cargo](#tab-panel-6123)

```
npm run tauri add positioner
```

```
yarn run tauri add positioner
```

```
pnpm tauri add positioner
```

```
deno task tauri add positioner
```

```
bun tauri add positioner
```

```
cargo tauri add positioner
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-positioner --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .setup(|app| {            #[cfg(desktop)]            app.handle().plugin(tauri_plugin_positioner::init());            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6124)
    - [yarn](#tab-panel-6125)
    - [pnpm](#tab-panel-6126)
    - [deno](#tab-panel-6127)
    - [bun](#tab-panel-6128)

    ```
    npm install @tauri-apps/plugin-positioner
    ```

    ```
    yarn add @tauri-apps/plugin-positioner
    ```

    ```
    pnpm add @tauri-apps/plugin-positioner
    ```

    ```
    deno add npm:@tauri-apps/plugin-positioner
    ```

    ```
    bun add @tauri-apps/plugin-positioner
    ```

Additional setup is required to get tray-relative positions to work.

1.  Add `tray-icon` feature to your `Cargo.toml` file:

    ```
    [dependencies]tauri-plugin-positioner = { version = "2.0.0", features = ["tray-icon"] }
    ```

    src-tauri/Cargo.toml

2.  Setup `on_tray_event` for positioner plugin:

    ```
    pub fn run() {  tauri::Builder::default()    // This is required to get tray-relative positions to work    .setup(|app| {        #[cfg(desktop)]        {          app.handle().plugin(tauri_plugin_positioner::init());            tauri::tray::TrayIconBuilder::new()              .on_tray_icon_event(|tray_handle, event| {                tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);              })              .build(app)?;        }      Ok(())    })    .run(tauri::generate_context!())    .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

## Usage

The plugin’s APIs are available through the JavaScript guest bindings:

```
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';// when using `"withGlobalTauri": true`, you may use// const { moveWindow, Position } = window.__TAURI__.positioner;
moveWindow(Position.TopRight);
```

You can import and use the Window trait extension directly through Rust:

```
use tauri_plugin_positioner::{WindowExt, Position};
let mut win = app.get_webview_window("main").unwrap();let _ = win.as_ref().window().move_window(Position::TopRight);
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
{  "permissions": [    ...,    "positioner:default",  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

Allows the moveWindow and handleIconState APIs

#### This default permission set includes the following:

- `allow-move-window`
- `allow-move-window-constrained`
- `allow-set-tray-icon-state`

## Permission Table

| Identifier | Description |
|----|----|
| `positioner:allow-move-window` | Enables the move_window command without any pre-configured scope. |
| `positioner:deny-move-window` | Denies the move_window command without any pre-configured scope. |
| `positioner:allow-move-window-constrained` | Enables the move_window_constrained command without any pre-configured scope. |
| `positioner:deny-move-window-constrained` | Denies the move_window_constrained command without any pre-configured scope. |
| `positioner:allow-set-tray-icon-state` | Enables the set_tray_icon_state command without any pre-configured scope. |
| `positioner:deny-set-tray-icon-state` | Denies the set_tray_icon_state command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
