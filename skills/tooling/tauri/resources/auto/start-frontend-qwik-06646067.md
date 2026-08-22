# Qwik

This guide will walk you through creating your Tauri app using the Qwik
web framework. Learn more about Qwik at <https://qwik.dev>.

## Checklist

- Use [SSG](https://qwik.dev/docs/guides/static-site-generation/). Tauri
  doesn’t support server-based solutions.
- Use `dist/` as `frontendDist` in `tauri.conf.json`.

## Example Configuration

1.  ##### Create a new Qwik app

    - [npm](#tab-panel-6617)
    - [yarn](#tab-panel-6618)
    - [pnpm](#tab-panel-6619)
    - [deno](#tab-panel-6620)

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

    - [npm](#tab-panel-6621)
    - [yarn](#tab-panel-6622)
    - [pnpm](#tab-panel-6623)
    - [deno](#tab-panel-6624)

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

    - [npm](#tab-panel-6625)
    - [yarn](#tab-panel-6626)
    - [pnpm](#tab-panel-6627)
    - [deno](#tab-panel-6628)

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

    - [npm](#tab-panel-6629)
    - [yarn](#tab-panel-6630)
    - [pnpm](#tab-panel-6631)
    - [deno](#tab-panel-6632)

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

    - [npm](#tab-panel-6613)
    - [yarn](#tab-panel-6614)
    - [pnpm](#tab-panel-6615)
    - [deno](#tab-panel-6616)

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

    - [npm](#tab-panel-6633)
    - [yarn](#tab-panel-6634)
    - [pnpm](#tab-panel-6635)
    - [deno](#tab-panel-6636)

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
