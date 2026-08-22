# Store

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-store)[crates.io](https://crates.io/crates/tauri-plugin-store)

API
Reference:[](https://v2.tauri.app/reference/javascript/store/)[](https://docs.rs/tauri-plugin-store)

This plugin provides a persistent key-value store. This is one of many
options to handle state in your application. See the [state management
overview](https://v2.tauri.app/develop/state-management/) for more information on additional
options.

This store will allow you to persist state to a file which can be saved
and loaded on demand including between app restarts. Note that this
process is asynchronous which will require handling it within your code.
It can be used both in the webview or within Rust.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the store plugin to get started.

- [Automatic](#tab-panel-6236)
- [Manual](#tab-panel-6237)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6225)
- [yarn](#tab-panel-6226)
- [pnpm](#tab-panel-6227)
- [deno](#tab-panel-6228)
- [bun](#tab-panel-6229)
- [cargo](#tab-panel-6230)

```
npm run tauri add store
```

```
yarn run tauri add store
```

```
pnpm tauri add store
```

```
deno task tauri add store
```

```
bun tauri add store
```

```
cargo tauri add store
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-store
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_store::Builder::new().build())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6231)
    - [yarn](#tab-panel-6232)
    - [pnpm](#tab-panel-6233)
    - [deno](#tab-panel-6234)
    - [bun](#tab-panel-6235)

    ```
    npm install @tauri-apps/plugin-store
    ```

    ```
    yarn add @tauri-apps/plugin-store
    ```

    ```
    pnpm add @tauri-apps/plugin-store
    ```

    ```
    deno add npm:@tauri-apps/plugin-store
    ```

    ```
    bun add @tauri-apps/plugin-store
    ```

## Usage

- [JavaScript](#tab-panel-6221)
- [Rust](#tab-panel-6222)

```
import { load } from '@tauri-apps/plugin-store';// when using `"withGlobalTauri": true`, you may use// const { load } = window.__TAURI__.store;
// Create a new store or load the existing one,// note that the options will be ignored if a `Store` with that path has already been createdconst store = await load('store.json', { autoSave: false });
// Set a value.await store.set('some-key', { value: 5 });
// Get a value.const val = await store.get<{ value: number }>('some-key');console.log(val); // { value: 5 }
// You can manually save the store after making changes.// Otherwise, it will save upon graceful exit// And if you set `autoSave` to a number or left empty,// it will save the changes to disk after a debounce delay, 100ms by default.await store.save();
```

```
use tauri::Wry;use tauri_plugin_store::StoreExt;use serde_json::json;
#[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_store::Builder::default().build())        .setup(|app| {            // Create a new store or load the existing one            // this also put the store in the app's resource table            // so your following `store` calls (from both Rust and JS)            // will reuse the same store.
            let store = app.store("store.json")?;
            // Note that values must be serde_json::Value instances,            // otherwise, they will not be compatible with the JavaScript bindings.            store.set("some-key", json!({ "value": 5 }));
            // Get a value from the store.            let value = store.get("some-key").expect("Failed to get value from store");            println!("{}", value); // {"value":5}
            // Remove the store from the resource table            store.close_resource();
            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
```

src-tauri/src/lib.rs

### LazyStore

There’s also a high level JavaScript API `LazyStore` which only loads
the store on first access

```
import { LazyStore } from '@tauri-apps/plugin-store';
const store = new LazyStore('settings.json');
```

## Migrating from v1 and v2 beta/rc

- [JavaScript](#tab-panel-6223)
- [Rust](#tab-panel-6224)

```
import { Store } from '@tauri-apps/plugin-store';import { LazyStore } from '@tauri-apps/plugin-store';
```

```
with_store(app.handle().clone(), stores, path, |store| {    store.insert("some-key".to_string(), json!({ "value": 5 }))?;    Ok(())});let store = app.store(path)?;store.set("some-key".to_string(), json!({ "value": 5 }));
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
{  "permissions": [    ...,    "store:default",  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set configures what kind of operations are available
from the store plugin.

#### [Granted Permissions](#granted-permissions)

All operations are enabled by default.

#### This default permission set includes the following:

- `allow-load`
- `allow-get-store`
- `allow-set`
- `allow-get`
- `allow-has`
- `allow-delete`
- `allow-clear`
- `allow-reset`
- `allow-keys`
- `allow-values`
- `allow-entries`
- `allow-length`
- `allow-reload`
- `allow-save`

## Permission Table

| Identifier | Description |
|----|----|
| `store:allow-clear` | Enables the clear command without any pre-configured scope. |
| `store:deny-clear` | Denies the clear command without any pre-configured scope. |
| `store:allow-delete` | Enables the delete command without any pre-configured scope. |
| `store:deny-delete` | Denies the delete command without any pre-configured scope. |
| `store:allow-entries` | Enables the entries command without any pre-configured scope. |
| `store:deny-entries` | Denies the entries command without any pre-configured scope. |
| `store:allow-get` | Enables the get command without any pre-configured scope. |
| `store:deny-get` | Denies the get command without any pre-configured scope. |
| `store:allow-get-store` | Enables the get_store command without any pre-configured scope. |
| `store:deny-get-store` | Denies the get_store command without any pre-configured scope. |
| `store:allow-has` | Enables the has command without any pre-configured scope. |
| `store:deny-has` | Denies the has command without any pre-configured scope. |
| `store:allow-keys` | Enables the keys command without any pre-configured scope. |
| `store:deny-keys` | Denies the keys command without any pre-configured scope. |
| `store:allow-length` | Enables the length command without any pre-configured scope. |
| `store:deny-length` | Denies the length command without any pre-configured scope. |
| `store:allow-load` | Enables the load command without any pre-configured scope. |
| `store:deny-load` | Denies the load command without any pre-configured scope. |
| `store:allow-reload` | Enables the reload command without any pre-configured scope. |
| `store:deny-reload` | Denies the reload command without any pre-configured scope. |
| `store:allow-reset` | Enables the reset command without any pre-configured scope. |
| `store:deny-reset` | Denies the reset command without any pre-configured scope. |
| `store:allow-save` | Enables the save command without any pre-configured scope. |
| `store:deny-save` | Denies the save command without any pre-configured scope. |
| `store:allow-set` | Enables the set command without any pre-configured scope. |
| `store:deny-set` | Denies the set command without any pre-configured scope. |
| `store:allow-values` | Enables the values command without any pre-configured scope. |
| `store:deny-values` | Denies the values command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
