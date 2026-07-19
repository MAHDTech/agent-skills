# Leptos

Leptos is a Rust based web framework. You can read more about Leptos on
their [official website](https://leptos.dev/). This guide is accurate as
of Leptos version 0.6.

## Checklist

- Use SSG, Tauri doesn’t officially support server based solutions.
- Use `serve.ws_protocol = "ws"` so that the hot-reload websocket can
  connect properly for mobile development.
- Enable `withGlobalTauri` to ensure that Tauri APIs are available in
  the `window.__TAURI__` variable and can be imported using
  `wasm-bindgen`.

## Example Configuration

1.  ##### Update Tauri configuration

    ```
    {  "build": {    "beforeDevCommand": "trunk serve",    "devUrl": "http://localhost:1420",    "beforeBuildCommand": "trunk build",    "frontendDist": "../dist"  },  "app": {    "withGlobalTauri": true  }}
    ```

    src-tauri/tauri.conf.json

2.  ##### Update Trunk configuration

    ```
    [build]target = "./index.html"
    [watch]ignore = ["./src-tauri"]
    [serve]port = 1420open = falsews_protocol = "ws"
    ```

    Trunk.toml

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
