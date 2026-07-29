+++
title = "plugin-window-state-22846c22"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Window State

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/window-state)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-window-state)[crates.io](https://crates.io/crates/tauri-plugin-window-state)

API
Reference[](https://v2.tauri.app/reference/javascript/window-state/)[](https://docs.rs/tauri-plugin-window-state)

Save window positions and sizes and restore them when the app is
reopened.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the window-state plugin to get started.

- [Automatic](#tab-panel-6668)
- [Manual](#tab-panel-6669)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6657)
- [yarn](#tab-panel-6658)
- [pnpm](#tab-panel-6659)
- [deno](#tab-panel-6660)
- [bun](#tab-panel-6661)
- [cargo](#tab-panel-6662)

```
npm run tauri add window-state
```

```
yarn run tauri add window-state
```

```
pnpm tauri add window-state
```

```
deno task tauri add window-state
```

```
bun tauri add window-state
```

```
cargo tauri add window-state
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-window-state --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .setup(|app| {            #[cfg(desktop)]            app.handle().plugin(tauri_plugin_window_state::Builder::default().build());            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6663)
    - [yarn](#tab-panel-6664)
    - [pnpm](#tab-panel-6665)
    - [deno](#tab-panel-6666)
    - [bun](#tab-panel-6667)

    ```
    npm install @tauri-apps/plugin-window-state
    ```

    ```
    yarn add @tauri-apps/plugin-window-state
    ```

    ```
    pnpm add @tauri-apps/plugin-window-state
    ```

    ```
    deno add npm:@tauri-apps/plugin-window-state
    ```

    ```
    bun add @tauri-apps/plugin-window-state
    ```

## Usage

After adding the window-state plugin, all windows will remember their
state when the app is being closed and will restore to their previous
state on the next launch.

You can also access the window-state plugin in both JavaScript and Rust.

### JavaScript

You can use `saveWindowState` to manually save the window state:

```
import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';// when using `"withGlobalTauri": true`, you may use// const { saveWindowState, StateFlags } = window.__TAURI__.windowState;
saveWindowState(StateFlags.ALL);
```

Similarly you can manually restore a window’s state from disk:

```
import {  restoreStateCurrent,  StateFlags,} from '@tauri-apps/plugin-window-state';// when using `"withGlobalTauri": true`, you may use// const { restoreStateCurrent, StateFlags } = window.__TAURI__.windowState;
restoreStateCurrent(StateFlags.ALL);
```

### Rust

You can use the `save_window_state()` method exposed by the
`AppHandleExt` trait:

```
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
// `tauri::AppHandle` now has the following additional methodapp.save_window_state(StateFlags::all()); // will save the state of all open windows to disk
```

Similarly you can manually restore a window’s state from disk using the
`restore_state()` method exposed by the `WindowExt` trait:

```
use tauri_plugin_window_state::{WindowExt, StateFlags};
// all `Window` types now have the following additional methodwindow.restore_state(StateFlags::all()); // will restore the window's state from disk
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
{  "permissions": [    ...,    "window-state:default",  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set configures what kind of operations are available
from the window state plugin.

#### [Granted Permissions](#granted-permissions)

All operations are enabled by default.

#### This default permission set includes the following:

- `allow-filename`
- `allow-restore-state`
- `allow-save-window-state`

## Permission Table

| Identifier | Description |
|----|----|
| `window-state:allow-filename` | Enables the filename command without any pre-configured scope. |
| `window-state:deny-filename` | Denies the filename command without any pre-configured scope. |
| `window-state:allow-restore-state` | Enables the restore_state command without any pre-configured scope. |
| `window-state:deny-restore-state` | Denies the restore_state command without any pre-configured scope. |
| `window-state:allow-save-window-state` | Enables the save_window_state command without any pre-configured scope. |
| `window-state:deny-save-window-state` | Denies the save_window_state command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
