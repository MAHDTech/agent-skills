# Clipboard

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/clipboard-manager)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-clipboard-manager)[crates.io](https://crates.io/crates/tauri-plugin-clipboard-manager)

API
Reference:[](https://v2.tauri.app/reference/javascript/clipboard-manager/)[](https://docs.rs/tauri-plugin-clipboard-manager)

Read and write to the system clipboard using the clipboard plugin.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the clipboard plugin to get started.

- [Automatic](#tab-panel-5953)
- [Manual](#tab-panel-5954)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-5942)
- [yarn](#tab-panel-5943)
- [pnpm](#tab-panel-5944)
- [deno](#tab-panel-5945)
- [bun](#tab-panel-5946)
- [cargo](#tab-panel-5947)

```
npm run tauri add clipboard-manager
```

```
yarn run tauri add clipboard-manager
```

```
pnpm tauri add clipboard-manager
```

```
deno task tauri add clipboard-manager
```

```
bun tauri add clipboard-manager
```

```
cargo tauri add clipboard-manager
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-clipboard-manager
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_clipboard_manager::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  If you’d like to manage the clipboard in JavaScript then install the
    npm package as well:

    - [npm](#tab-panel-5948)
    - [yarn](#tab-panel-5949)
    - [pnpm](#tab-panel-5950)
    - [deno](#tab-panel-5951)
    - [bun](#tab-panel-5952)

    ```
    npm install @tauri-apps/plugin-clipboard-manager
    ```

    ```
    yarn add @tauri-apps/plugin-clipboard-manager
    ```

    ```
    pnpm add @tauri-apps/plugin-clipboard-manager
    ```

    ```
    deno add npm:@tauri-apps/plugin-clipboard-manager
    ```

    ```
    bun add @tauri-apps/plugin-clipboard-manager
    ```

## Usage

The clipboard plugin is available in both JavaScript and Rust.

- [JavaScript](#tab-panel-5940)
- [Rust](#tab-panel-5941)

```
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';// when using `"withGlobalTauri": true`, you may use// const { writeText, readText } = window.__TAURI__.clipboardManager;
// Write content to clipboardawait writeText('Tauri is awesome!');
// Read content from clipboardconst content = await readText();console.log(content);// Prints "Tauri is awesome!" to the console
```

```
use tauri_plugin_clipboard_manager::ClipboardExt;
app.clipboard().write_text("Tauri is awesome!".to_string()).unwrap();
// Read content from clipboardlet content = app.clipboard().read_text();println!("{:?}", content.unwrap());// Prints "Tauri is awesome!" to the terminal
```

## [Default Permission](#default-permission)

No features are enabled by default, as we believe the clipboard can be
inherently dangerous and it is application specific if read and/or write
access is needed.

Clipboard interaction needs to be explicitly enabled.

## Permission Table

| Identifier | Description |
|----|----|
| `clipboard-manager:allow-clear` | Enables the clear command without any pre-configured scope. |
| `clipboard-manager:deny-clear` | Denies the clear command without any pre-configured scope. |
| `clipboard-manager:allow-read-image` | Enables the read_image command without any pre-configured scope. |
| `clipboard-manager:deny-read-image` | Denies the read_image command without any pre-configured scope. |
| `clipboard-manager:allow-read-text` | Enables the read_text command without any pre-configured scope. |
| `clipboard-manager:deny-read-text` | Denies the read_text command without any pre-configured scope. |
| `clipboard-manager:allow-write-html` | Enables the write_html command without any pre-configured scope. |
| `clipboard-manager:deny-write-html` | Denies the write_html command without any pre-configured scope. |
| `clipboard-manager:allow-write-image` | Enables the write_image command without any pre-configured scope. |
| `clipboard-manager:deny-write-image` | Denies the write_image command without any pre-configured scope. |
| `clipboard-manager:allow-write-text` | Enables the write_text command without any pre-configured scope. |
| `clipboard-manager:deny-write-text` | Denies the write_text command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
