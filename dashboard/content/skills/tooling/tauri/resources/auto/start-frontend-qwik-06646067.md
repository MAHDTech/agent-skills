+++
title = "start-frontend-qwik-06646067"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Qwik

This guide will walk you through creating your Tauri app using the Qwik
web framework. Learn more about Qwik at <https://qwik.dev>.

## Checklist

- Use [SSG](https://qwik.dev/docs/guides/static-site-generation/). Tauri
  doesn’t support server-based solutions.
- Use `dist/` as `frontendDist` in `tauri.conf.json`.

## Example Configuration

1.  ##### Create a new Qwik app

    - [npm](#tab-panel-6968)
    - [yarn](#tab-panel-6969)
    - [pnpm](#tab-panel-6970)
    - [deno](#tab-panel-6971)

    ```
    npm create qwik@latestcd <PROJECT>
    ```

    ```
    yarn create qwik@latestcd <PROJECT>
    ```

    ```
    pnpm create qwik@latestcd <PROJECT>
    ```

    ```
    deno run -A npm:create-qwik@latestcd <PROJECT>
    ```

2.  ##### Install the `static adapter`

    - [npm](#tab-panel-6972)
    - [yarn](#tab-panel-6973)
    - [pnpm](#tab-panel-6974)
    - [deno](#tab-panel-6975)

    ```
    npm run qwik add static
    ```

    ```
    yarn qwik add static
    ```

    ```
    pnpm qwik add static
    ```

    ```
    deno task qwik add static
    ```

3.  ##### Add the Tauri CLI to your project

    - [npm](#tab-panel-6976)
    - [yarn](#tab-panel-6977)
    - [pnpm](#tab-panel-6978)
    - [deno](#tab-panel-6979)

    ```
    npm install -D @tauri-apps/cli@latest
    ```

    ```
    yarn add -D @tauri-apps/cli@latest
    ```

    ```
    pnpm add -D @tauri-apps/cli@latest
    ```

    ```
    deno add -D npm:@tauri-apps/cli@latest
    ```

4.  ##### Initiate a new Tauri project

    - [npm](#tab-panel-6980)
    - [yarn](#tab-panel-6981)
    - [pnpm](#tab-panel-6982)
    - [deno](#tab-panel-6983)

    ```
    npm run tauri init
    ```

    ```
    yarn tauri init
    ```

    ```
    pnpm tauri init
    ```

    ```
    deno task tauri init
    ```

5.  ##### Tauri configuration

    - [npm](#tab-panel-6964)
    - [yarn](#tab-panel-6965)
    - [pnpm](#tab-panel-6966)
    - [deno](#tab-panel-6967)

    ```
    {  "build": {    "devUrl": "http://localhost:5173"    "frontendDist": "../dist",    "beforeDevCommand": "npm run dev",    "beforeBuildCommand": "npm run build"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "devUrl": "http://localhost:5173"    "frontendDist": "../dist",    "beforeDevCommand": "yarn dev",    "beforeBuildCommand": "yarn build"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "devUrl": "http://localhost:5173"    "frontendDist": "../dist",    "beforeDevCommand": "pnpm dev",    "beforeBuildCommand": "pnpm build"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "devUrl": "http://localhost:5173"    "frontendDist": "../dist",    "beforeDevCommand": "deno task dev",    "beforeBuildCommand": "deno task build"  }}
    ```

    tauri.conf.json

6.  ##### Start your `tauri` app

    - [npm](#tab-panel-6984)
    - [yarn](#tab-panel-6985)
    - [pnpm](#tab-panel-6986)
    - [deno](#tab-panel-6987)

    ```
    npm run tauri dev
    ```

    ```
    yarn tauri dev
    ```

    ```
    pnpm tauri dev
    ```

    ```
    deno task tauri dev
    ```

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
