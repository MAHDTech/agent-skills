# Nuxt

Nuxt is a meta framework for Vue. Learn more about Nuxt at
<https://nuxt.com>. This guide is accurate as of Nuxt 4.2.

## Checklist

- Use SSG by setting `ssr: false`. Tauri doesn’t support server based
  solutions.
- Use default `../dist` as `frontendDist` in `tauri.conf.json`.
- Compile using `nuxi build`.
- (Optional): Disable telemetry by setting `telemetry: false` in
  `nuxt.config.ts`.

## Example Configuration

1.  ##### Update Tauri configuration

    - [npm](#tab-panel-3526)
    - [yarn](#tab-panel-3527)
    - [pnpm](#tab-panel-3528)
    - [deno](#tab-panel-3529)

    ```
    {  "build": {    "beforeDevCommand": "npm run dev",    "beforeBuildCommand": "npm run generate",    "devUrl": "http://localhost:3000",    "frontendDist": "../dist"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "beforeDevCommand": "yarn dev",    "beforeBuildCommand": "yarn generate",    "devUrl": "http://localhost:3000",    "frontendDist": "../dist"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "beforeDevCommand": "pnpm dev",    "beforeBuildCommand": "pnpm generate",    "devUrl": "http://localhost:3000",    "frontendDist": "../dist"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "beforeDevCommand": "deno task dev",    "beforeBuildCommand": "deno task generate",    "devUrl": "http://localhost:3000",    "frontendDist": "../dist"  }}
    ```

    tauri.conf.json

2.  ##### Update Nuxt configuration

    ```
    export default defineNuxtConfig({  compatibilityDate: '2025-05-15',  // (optional) Enable the Nuxt devtools  devtools: { enabled: true },  // Enable SSG  ssr: false,  // Enables the development server to be discoverable by other devices when running on iOS physical devices  devServer: {    host: '0',  },  vite: {    // Better support for Tauri CLI output    clearScreen: false,    // Enable environment variables    // Additional environment variables can be found at    // https://v2.tauri.app/reference/environment-variables/    envPrefix: ['VITE_', 'TAURI_'],    server: {      // Tauri requires a consistent port      strictPort: true,    },  },  // Avoids error [unhandledRejection] EMFILE: too many open files, watch  ignore: ['**/src-tauri/**'],});
    ```

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
