# SvelteKit

SvelteKit is a meta framework for Svelte. Learn more about SvelteKit at
<https://svelte.dev/>. This guide is accurate as of SvelteKit 2.20.4 /
Svelte 5.25.8.

## Checklist

- Use [SSG](https://svelte.dev/docs/kit/adapter-static) and
  [SPA](https://svelte.dev/docs/kit/single-page-apps) via
  `static-adapter`. Tauri doesn’t support server-based solutions.
- If using SSG **with prerendering**, be aware that `load` functions
  will not have access to tauri APIs during the build process of your
  app. Using SPA mode (without prerendering) is recommended since the
  load functions will only run in the webview with access to tauri APIs.
- Use `build/` as `frontendDist` in `tauri.conf.json`.

## Example Configuration

1.  ##### Install `@sveltejs/adapter-static`

    - [npm](#tab-panel-6992)
    - [yarn](#tab-panel-6993)
    - [pnpm](#tab-panel-6994)
    - [deno](#tab-panel-6995)

    ```
    npm install --save-dev @sveltejs/adapter-static
    ```

    ```
    yarn add -D @sveltejs/adapter-static
    ```

    ```
    pnpm add -D @sveltejs/adapter-static
    ```

    ```
    deno add -D npm:@sveltejs/adapter-static
    ```

2.  ##### Update Tauri configuration

    - [npm](#tab-panel-6988)
    - [yarn](#tab-panel-6989)
    - [pnpm](#tab-panel-6990)
    - [deno](#tab-panel-6991)

    ```
    {  "build": {    "beforeDevCommand": "npm run dev",    "beforeBuildCommand": "npm run build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "beforeDevCommand": "yarn dev",    "beforeBuildCommand": "yarn build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "beforeDevCommand": "pnpm dev",    "beforeBuildCommand": "pnpm build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
    ```

    tauri.conf.json

    ```
    {  "build": {    "beforeDevCommand": "deno task dev",    "beforeBuildCommand": "deno task build",    "devUrl": "http://localhost:5173",    "frontendDist": "../build"  }}
    ```

    tauri.conf.json

3.  ##### Update SvelteKit configuration:

    ```
    import adapter from '@sveltejs/adapter-static';import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
    /** @type {import('@sveltejs/kit').Config} */const config = {  // Consult https://svelte.dev/docs/kit/integrations#preprocessors  // for more information about preprocessors  preprocess: vitePreprocess(),
      kit: {    adapter: adapter({      fallback: 'index.html',    }),  },};
    export default config;
    ```

    svelte.config.js

4.  ##### Disable SSR

    Lastly, we need to disable SSR by adding a root `+layout.ts` file
    (or `+layout.js` if you are not using TypeScript) with these
    contents:

    ```
    export const ssr = false;
    ```

    src/routes/+layout.ts

    Note that `static-adapter` doesn’t require you to disable SSR for
    the whole app but it makes it possible to use APIs that depend on
    the global window object (like Tauri’s API) without [Client-side
    checks](https://svelte.dev/docs/kit/faq#how-do-i-use-x-with-sveltekit-how-do-i-use-a-client-side-only-library-that-depends-on-document-or-window).

    Furthermore, if you prefer Static Site Generation (SSG) over
    Single-Page Application (SPA) mode, you can change the adapter
    configurations and `+layout.ts` according to the [adapter
    docs](https://svelte.dev/docs/kit/adapter-static).

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
