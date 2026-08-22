+++
title = "plugin-http-client-df0fddcf"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# HTTP Client

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/http)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-http)[crates.io](https://crates.io/crates/tauri-plugin-http)

API
Reference:[](https://v2.tauri.app/reference/javascript/http/)[](https://docs.rs/tauri-plugin-http)

Make HTTP requests with the http plugin.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the http plugin to get started.

- [Automatic](#tab-panel-6050)
- [Manual](#tab-panel-6051)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6039)
- [yarn](#tab-panel-6040)
- [pnpm](#tab-panel-6041)
- [deno](#tab-panel-6042)
- [bun](#tab-panel-6043)
- [cargo](#tab-panel-6044)

```
npm run tauri add http
```

```
yarn run tauri add http
```

```
pnpm tauri add http
```

```
deno task tauri add http
```

```
bun tauri add http
```

```
cargo tauri add http
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-http
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_http::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  If you’d like to make http requests in JavaScript then install the
    npm package as well:

    - [npm](#tab-panel-6045)
    - [yarn](#tab-panel-6046)
    - [pnpm](#tab-panel-6047)
    - [deno](#tab-panel-6048)
    - [bun](#tab-panel-6049)

    ```
    npm install @tauri-apps/plugin-http
    ```

    ```
    yarn add @tauri-apps/plugin-http
    ```

    ```
    pnpm add @tauri-apps/plugin-http
    ```

    ```
    deno add npm:@tauri-apps/plugin-http
    ```

    ```
    bun add @tauri-apps/plugin-http
    ```

## Usage

The HTTP plugin is available in both Rust as a
[reqwest](https://docs.rs/reqwest/) re-export and JavaScript.

### JavaScript

1.  Configure the allowed URLs

    ```
    {  "permissions": [    {      "identifier": "http:default",      "allow": [{ "url": "https://*.tauri.app" }],      "deny": [{ "url": "https://private.tauri.app" }]    }  ]}
    ```

    src-tauri/capabilities/default.json

    For more information, please see the documentation for [Permissions
    Overview](https://v2.tauri.app/security/permissions/)

2.  Send a request

    The `fetch` method tries to be as close and compliant to the
    [`fetch` Web
    API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) as
    possible.

    ```
    import { fetch } from '@tauri-apps/plugin-http';
    // Send a GET requestconst response = await fetch('http://test.tauri.app/data.json', {  method: 'GET',});console.log(response.status); // e.g. 200console.log(response.statusText); // e.g. "OK"
    ```

### Rust

In Rust you can utilize the `reqwest` crate re-exported by the plugin.
For more details refer to [reqwest docs](https://docs.rs/reqwest/).

```
use tauri_plugin_http::reqwest;
let res = reqwest::get("http://my.api.host/data.json").await;println!("{:?}", res.status()); // e.g. 200println!("{:?}", res.text().await); // e.g Ok("{ Content }")
```

## [Default Permission](#default-permission)

This permission set configures what kind of fetch operations are
available from the http plugin.

This enables all fetch operations but does not allow explicitly any
origins to be fetched. This needs to be manually configured before
usage.

#### [Granted Permissions](#granted-permissions)

All fetch operations are enabled.

#### This default permission set includes the following:

- `allow-fetch`
- `allow-fetch-cancel`
- `allow-fetch-send`
- `allow-fetch-read-body`
- `allow-fetch-cancel-body`

## Permission Table

| Identifier | Description |
|----|----|
| `http:allow-fetch` | Enables the fetch command without any pre-configured scope. |
| `http:deny-fetch` | Denies the fetch command without any pre-configured scope. |
| `http:allow-fetch-cancel` | Enables the fetch_cancel command without any pre-configured scope. |
| `http:deny-fetch-cancel` | Denies the fetch_cancel command without any pre-configured scope. |
| `http:allow-fetch-cancel-body` | Enables the fetch_cancel_body command without any pre-configured scope. |
| `http:deny-fetch-cancel-body` | Denies the fetch_cancel_body command without any pre-configured scope. |
| `http:allow-fetch-read-body` | Enables the fetch_read_body command without any pre-configured scope. |
| `http:deny-fetch-read-body` | Denies the fetch_read_body command without any pre-configured scope. |
| `http:allow-fetch-send` | Enables the fetch_send command without any pre-configured scope. |
| `http:deny-fetch-send` | Denies the fetch_send command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

