+++
title = "start-frontend-trunk"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Trunk

Trunk is a WASM web application bundler for Rust. Learn more about Trunk
at <https://trunk-rs.github.io/trunk/>. This guide is accurate as of
Trunk 0.17.5.

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
    {  "build": {    "beforeDevCommand": "trunk serve",    "beforeBuildCommand": "trunk build",    "devUrl": "http://localhost:8080",    "frontendDist": "../dist"  },  "app": {    "withGlobalTauri": true  }}
    ```

    tauri.conf.json

2.  ##### Update Trunk configuration

    ```
    [watch]ignore = ["./src-tauri"]
    [serve]ws_protocol = "ws"
    ```

    Trunk.toml

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

