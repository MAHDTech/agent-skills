+++
title = "plugin-websocket-8129096d"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Websocket

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/websocket)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-websocket)[crates.io](https://crates.io/crates/tauri-plugin-websocket)

API
Reference[](https://v2.tauri.app/reference/javascript/websocket/)[](https://docs.rs/tauri-plugin-websocket)

Open a WebSocket connection using a Rust client in JavaScript.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the websocket plugin to get started.

- [Automatic](#tab-panel-6655)
- [Manual](#tab-panel-6656)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6644)
- [yarn](#tab-panel-6645)
- [pnpm](#tab-panel-6646)
- [deno](#tab-panel-6647)
- [bun](#tab-panel-6648)
- [cargo](#tab-panel-6649)

```
npm run tauri add websocket
```

```
yarn run tauri add websocket
```

```
pnpm tauri add websocket
```

```
deno task tauri add websocket
```

```
bun tauri add websocket
```

```
cargo tauri add websocket
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-websocket
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_websocket::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6650)
    - [yarn](#tab-panel-6651)
    - [pnpm](#tab-panel-6652)
    - [deno](#tab-panel-6653)
    - [bun](#tab-panel-6654)

    ```
    npm install @tauri-apps/plugin-websocket
    ```

    ```
    yarn add @tauri-apps/plugin-websocket
    ```

    ```
    pnpm add @tauri-apps/plugin-websocket
    ```

    ```
    deno add npm:@tauri-apps/plugin-websocket
    ```

    ```
    bun add @tauri-apps/plugin-websocket
    ```

## Usage

The websocket plugin is available in JavaScript.

```
import WebSocket from '@tauri-apps/plugin-websocket';// when using `"withGlobalTauri": true`, you may use// const WebSocket = window.__TAURI__.websocket;
const ws = await WebSocket.connect('ws://127.0.0.1:8080');
const removeListener = ws.addListener((msg) => {  console.log('Received Message:', msg);});
await ws.send('Hello World!');
// optionally remove the listenerremoveListener();
await ws.disconnect();
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
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": ["websocket:default"]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

Allows connecting and sending data to a WebSocket server

#### This default permission set includes the following:

- `allow-connect`
- `allow-send`

## Permission Table

| Identifier | Description |
|----|----|
| `websocket:allow-connect` | Enables the connect command without any pre-configured scope. |
| `websocket:deny-connect` | Denies the connect command without any pre-configured scope. |
| `websocket:allow-send` | Enables the send command without any pre-configured scope. |
| `websocket:deny-send` | Denies the send command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
