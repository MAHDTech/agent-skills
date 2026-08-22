+++
title = "plugin-opener-a408c918"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Opener

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/opener)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-opener)[crates.io](https://crates.io/crates/tauri-plugin-opener)

API
Reference:[](https://v2.tauri.app/reference/javascript/opener/)[](https://docs.rs/tauri-plugin-opener)

This plugin allows you to open files and URLs in a specified, or the
default, application. It also supports “revealing” files in the system’s
file explorer.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the opener plugin to get started.

- [Automatic](#tab-panel-6123)
- [Manual](#tab-panel-6124)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6112)
- [yarn](#tab-panel-6113)
- [pnpm](#tab-panel-6114)
- [deno](#tab-panel-6115)
- [bun](#tab-panel-6116)
- [cargo](#tab-panel-6117)

```
npm run tauri add opener
```

```
yarn run tauri add opener
```

```
pnpm tauri add opener
```

```
deno task tauri add opener
```

```
bun tauri add opener
```

```
cargo tauri add opener
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-opener
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_opener::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6118)
    - [yarn](#tab-panel-6119)
    - [pnpm](#tab-panel-6120)
    - [deno](#tab-panel-6121)
    - [bun](#tab-panel-6122)

    ```
    npm install @tauri-apps/plugin-opener
    ```

    ```
    yarn add @tauri-apps/plugin-opener
    ```

    ```
    pnpm add @tauri-apps/plugin-opener
    ```

    ```
    deno add npm:@tauri-apps/plugin-opener
    ```

    ```
    bun add @tauri-apps/plugin-opener
    ```

## Usage

The opener plugin is available in both JavaScript and Rust.

- [JavaScript](#tab-panel-6110)
- [Rust](#tab-panel-6111)

```
import { openPath, openUrl } from '@tauri-apps/plugin-opener';// when using `"withGlobalTauri": true`, you may use// const { openPath } = window.__TAURI__.opener;
// opens a file using the default program:await openPath('/path/to/file');// opens a file using `vlc` command on Windows:await openPath('C:/path/to/file', 'vlc');// opens a URL using the default program:await openUrl('https://tauri.app');
```

Note that `app` is an instance of `App` or
[`AppHandle`](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html).

```
use tauri_plugin_opener::OpenerExt;
// opens a file using the default program:app.opener().open_path("/path/to/file", None::<&str>);// opens a file using `vlc` command on Windows:app.opener().open_path("C:/path/to/file", Some("vlc"));// opens a URL using the default program:app.opener().open_url("https://tauri.app", None::<&str>);
```

## Permissions

By default all potentially dangerous plugin commands and scopes are
blocked and cannot be accessed. You must modify the permissions in your
`capabilities` configuration to enable these.

See the [Capabilities Overview](https://v2.tauri.app/security/capabilities/) for more
information and the [step by step
guide](https://v2.tauri.app/learn/security/using-plugin-permissions/) to use plugin
permissions.

Below are two example scope configurations. Both `path` and `url` use
the [glob pattern
syntax](https://docs.rs/glob/latest/glob/struct.Pattern.html) to define
allowed file paths and URLs.

First, an example on how to add permissions to specific paths for the
`openPath()` function:

```
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": [    {      "identifier": "opener:allow-open-path",      "allow": [        {          "path": "/path/to/file"        },        {          "path": "$APPDATA/file"        }      ]    }  ]}
```

src-tauri/capabilities/default.json

Lastly, an example on how to add permissions for the exact
`https://tauri.app` URL and all URLs on a custom protocol (must be known
to the OS) for the `openUrl()` function:

```
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": [    {      "identifier": "opener:allow-open-url",      "allow": [        {          "url": "https://tauri.app"        },        {          "url": "custom:*"        }      ]    }  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set allows opening `mailto:`, `tel:`, `https://` and
`http://` urls using their default application as well as reveal file in
directories using default file explorer

#### This default permission set includes the following:

- `allow-open-url`
- `allow-reveal-item-in-dir`
- `allow-default-urls`

## Permission Table

| Identifier | Description |
|----|----|
| `opener:allow-default-urls` | This enables opening `mailto:`, `tel:`, `https://` and `http://` urls using their default application. |
| `opener:allow-open-path` | Enables the open_path command without any pre-configured scope. |
| `opener:deny-open-path` | Denies the open_path command without any pre-configured scope. |
| `opener:allow-open-url` | Enables the open_url command without any pre-configured scope. |
| `opener:deny-open-url` | Denies the open_url command without any pre-configured scope. |
| `opener:allow-reveal-item-in-dir` | Enables the reveal_item_in_dir command without any pre-configured scope. |
| `opener:deny-reveal-item-in-dir` | Denies the reveal_item_in_dir command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

