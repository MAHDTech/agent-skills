# Localhost

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/localhost)[crates.io](https://crates.io/crates/tauri-plugin-localhost)

API Reference:[](https://docs.rs/tauri-plugin-localhost)

Expose your app’s assets through a localhost server instead of the
default custom protocol.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the localhost plugin to get started.

- [Automatic](#tab-panel-6028)
- [Manual](#tab-panel-6029)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6022)
- [yarn](#tab-panel-6023)
- [pnpm](#tab-panel-6024)
- [deno](#tab-panel-6025)
- [bun](#tab-panel-6026)
- [cargo](#tab-panel-6027)

```
npm run tauri add localhost
```

```
yarn run tauri add localhost
```

```
pnpm tauri add localhost
```

```
deno task tauri add localhost
```

```
bun tauri add localhost
```

```
cargo tauri add localhost
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-localhost
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_localhost::Builder::new().build())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

## Usage

The localhost plugin is available in Rust.

```
use tauri::{webview::WebviewWindowBuilder, WebviewUrl};
pub fn run() {  let port: u16 = 9527;
  tauri::Builder::default()      .plugin(tauri_plugin_localhost::Builder::new(port).build())      .setup(move |app| {          let url = format!("http://localhost:{}", port).parse().unwrap();          WebviewWindowBuilder::new(app, "main".to_string(), WebviewUrl::External(url))              .title("Localhost Example")              .build()?;          Ok(())      })      .run(tauri::generate_context!())      .expect("error while running tauri application");}
```

src-tauri/src/lib.rs

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
