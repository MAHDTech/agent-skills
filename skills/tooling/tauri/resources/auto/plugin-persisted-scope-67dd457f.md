# Persisted Scope

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/persisted-scope)[crates.io](https://crates.io/crates/tauri-plugin-persisted-scope)

API Reference:[](https://docs.rs/tauri-plugin-persisted-scope)

Save filesystem and asset scopes and restore them when the app is
reopened.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the persisted-scope plugin to get started.

- [Automatic](#tab-panel-6116)
- [Manual](#tab-panel-6117)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6110)
- [yarn](#tab-panel-6111)
- [pnpm](#tab-panel-6112)
- [deno](#tab-panel-6113)
- [bun](#tab-panel-6114)
- [cargo](#tab-panel-6115)

```
npm run tauri add persisted-scope
```

```
yarn run tauri add persisted-scope
```

```
pnpm tauri add persisted-scope
```

```
deno task tauri add persisted-scope
```

```
bun tauri add persisted-scope
```

```
cargo tauri add persisted-scope
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-persisted-scope
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_persisted_scope::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

## Usage

After setup the plugin will automatically save and restore filesystem
and asset scopes.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
