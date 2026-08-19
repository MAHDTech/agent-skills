+++
title = "plugin-upload-a0a43e3c"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Upload

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/upload)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-upload)[crates.io](https://crates.io/crates/tauri-plugin-upload)

API
Reference:[](https://v2.tauri.app/reference/javascript/upload/)[](https://docs.rs/tauri-plugin-upload)

Upload files from disk to a remote server over HTTP. Download files from
a remote HTTP server to disk.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

- [Automatic](#tab-panel-6261)
- [Manual](#tab-panel-6262)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6250)
- [yarn](#tab-panel-6251)
- [pnpm](#tab-panel-6252)
- [deno](#tab-panel-6253)
- [bun](#tab-panel-6254)
- [cargo](#tab-panel-6255)

```
npm run tauri add upload
```

```
yarn run tauri add upload
```

```
pnpm tauri add upload
```

```
deno task tauri add upload
```

```
bun tauri add upload
```

```
cargo tauri add upload
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-upload
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {  tauri::Builder::default()    .plugin(tauri_plugin_upload::init())      .run(tauri::generate_context!())      .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6256)
    - [yarn](#tab-panel-6257)
    - [pnpm](#tab-panel-6258)
    - [deno](#tab-panel-6259)
    - [bun](#tab-panel-6260)

    ```
    npm install @tauri-apps/plugin-upload
    ```

    ```
    yarn add @tauri-apps/plugin-upload
    ```

    ```
    pnpm add @tauri-apps/plugin-upload
    ```

    ```
    deno add npm:@tauri-apps/plugin-upload
    ```

    ```
    bun add @tauri-apps/plugin-upload
    ```

## Usage

Once you’ve completed the registration and setup process for the plugin,
you can access all of its APIs through the JavaScript guest bindings.

Here’s an example of how you can use the plugin to upload and download
files:

```
import { upload } from '@tauri-apps/plugin-upload';// when using `"withGlobalTauri": true`, you may use// const { upload } = window.__TAURI__.upload;
upload(  'https://example.com/file-upload',  './path/to/my/file.txt',  ({ progress, total }) =>    console.log(`Uploaded ${progress} of ${total} bytes`), // a callback that will be called with the upload progress  { 'Content-Type': 'text/plain' } // optional headers to send with the request);
```

```
import { download } from '@tauri-apps/plugin-upload';// when using `"withGlobalTauri": true`, you may use// const { download } = window.__TAURI__.upload;
download(  'https://example.com/file-download-link',  './path/to/save/my/file.txt',  ({ progress, total }) =>    console.log(`Downloaded ${progress} of ${total} bytes`), // a callback that will be called with the download progress  { 'Content-Type': 'text/plain' } // optional headers to send with the request);
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
{  "permissions": [    ...,    "upload:default",  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set configures what kind of operations are available
from the upload plugin.

#### [Granted Permissions](#granted-permissions)

All operations are enabled by default.

#### This default permission set includes the following:

- `allow-upload`
- `allow-download`

## Permission Table

| Identifier | Description |
|----|----|
| `upload:allow-download` | Enables the download command without any pre-configured scope. |
| `upload:deny-download` | Denies the download command without any pre-configured scope. |
| `upload:allow-upload` | Enables the upload command without any pre-configured scope. |
| `upload:deny-upload` | Denies the upload command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
