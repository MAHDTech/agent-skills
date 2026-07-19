# Create a Project

One thing that makes Tauri so flexible is its ability to work with
virtually any frontend framework. We’ve created the
[`create-tauri-app`](https://github.com/tauri-apps/create-tauri-app)
utility to help you create a new Tauri project using one of the
officially maintained framework templates.

`create-tauri-app` currently includes templates for vanilla (HTML, CSS
and JavaScript without a framework), [Vue.js](https://vuejs.org),
[Svelte](https://svelte.dev), [React](https://reactjs.org/),
[SolidJS](https://www.solidjs.com/), [Angular](https://angular.io/),
[Preact](https://preactjs.com/), [Yew](https://yew.rs/),
[Leptos](https://github.com/leptos-rs/leptos), and
[Sycamore](https://sycamore.dev/). You can also find or add your own
community templates and frameworks in the [Awesome Tauri
repo](https://github.com/tauri-apps/awesome-tauri).

Alternatively, you can [add Tauri to an existing
project](#manual-setup-tauri-cli) to quickly turn your existing codebase
into a Tauri app.

## Using `create-tauri-app`

To get started using `create-tauri-app` run one of the below commands in
the folder you’d like to setup your project. If you’re not sure which
command to use we recommend the Bash command on Linux and macOS and the
PowerShell command on Windows.

- [Bash](#tab-panel-3484)
- [PowerShell](#tab-panel-3485)
- [Fish](#tab-panel-3486)
- [npm](#tab-panel-3487)
- [Yarn](#tab-panel-3488)
- [pnpm](#tab-panel-3489)
- [deno](#tab-panel-3490)
- [bun](#tab-panel-3491)
- [Cargo](#tab-panel-3492)

```
sh <(curl https://create.tauri.app/sh)
```

```
irm https://create.tauri.app/ps | iex
```

```
sh (curl -sSL https://create.tauri.app/sh | psub)
```

```
npm create tauri-app@latest
```

```
yarn create tauri-app
```

```
pnpm create tauri-app
```

```
deno run -A npm:create-tauri-app
```

```
bun create tauri-app
```

```
cargo install create-tauri-app --lockedcargo create-tauri-app
```

Follow along with the prompts to choose your project name, frontend
language, package manager, and frontend framework, and frontend
framework options if applicable.

#### Scaffold a new project

1.  Choose a name and a bundle identifier (unique-id for your app):

    ```
    ? Project name (tauri-app) ›? Identifier (com.tauri-app.app) ›
    ```

2.  Select a flavor for your frontend. First the language:

    ```
    ? Choose which language to use for your frontend ›Rust  (cargo)TypeScript / JavaScript  (pnpm, yarn, npm, bun).NET  (dotnet)
    ```

3.  Select a package manager (if there are multiple available):

    Options for **TypeScript / JavaScript**:

    ```
    ? Choose your package manager ›pnpmyarnnpmbun
    ```

4.  Select a UI Template and flavor (if there are multiple available):

    Options for **Rust**:

    ```
    ? Choose your UI template ›VanillaYewLeptosSycamore
    ```

    Options for **TypeScript / JavaScript**:

    ```
    ? Choose your UI template ›VanillaVueSvelteReactSolidAngularPreact
    ? Choose your UI flavor ›TypeScriptJavaScript
    ```

    Options for **.NET**:

    ```
    ? Choose your UI template ›Blazor  (https://dotnet.microsoft.com/en-us/apps/aspnet/web-apps/blazor/)
    ```

Once completed, the utility reports that the template has been created
and displays how to run it using the configured package manager. If it
detects missing dependencies on your system, it prints a list of
packages and prompts how to install them.

#### Start the development server

After `create-tauri-app` has completed, you can navigate into your
project’s folder, install dependencies, and then use the [Tauri
CLI](https://v2.tauri.app/reference/cli/) to start the development server:

- [npm](#tab-panel-3493)
- [yarn](#tab-panel-3494)
- [pnpm](#tab-panel-3495)
- [deno](#tab-panel-3496)
- [bun](#tab-panel-3497)
- [cargo](#tab-panel-3498)

```
cd tauri-appnpm installnpm run tauri dev
```

```
cd tauri-appyarn installyarn tauri dev
```

```
cd tauri-apppnpm installpnpm tauri dev
```

```
cd tauri-appdeno installdeno task tauri dev
```

```
cd tauri-appbun installbun tauri dev
```

```
cd tauri-appcargo install tauri-cli --version "^2.0.0" --lockedcargo tauri dev
```

You’ll now see a new window open with your app running.

**Congratulations!** You’ve made your Tauri app! 🚀

## Manual Setup (Tauri CLI)

If you already have an existing frontend or prefer to set it up
yourself, you can use the Tauri CLI to initialize the backend for your
project separately.

1.  Create a new directory for your project and initialize the frontend.
    You can use plain HTML, CSS, and JavaScript, or any framework you
    prefer such as Next.js, Nuxt, Svelte, Yew, or Leptos. You just need
    a way of serving the app in your browser. Just as an example, this
    is how you would setup a simple Vite app:

    - [npm](#tab-panel-3499)
    - [yarn](#tab-panel-3500)
    - [pnpm](#tab-panel-3501)
    - [deno](#tab-panel-3502)
    - [bun](#tab-panel-3503)

    ```
    mkdir tauri-appcd tauri-appnpm create vite@latest .
    ```

    ```
    mkdir tauri-appcd tauri-appyarn create vite .
    ```

    ```
    mkdir tauri-appcd tauri-apppnpm create vite .
    ```

    ```
    mkdir tauri-appcd tauri-appdeno run -A npm:create-vite .
    ```

    ```
    mkdir tauri-appcd tauri-appbun create vite
    ```

2.  Then, install Tauri’s CLI tool using your package manager of choice.
    If you are using `cargo` to install the Tauri CLI, you will have to
    install it globally.

    - [npm](#tab-panel-3504)
    - [yarn](#tab-panel-3505)
    - [pnpm](#tab-panel-3506)
    - [deno](#tab-panel-3507)
    - [bun](#tab-panel-3508)
    - [cargo](#tab-panel-3509)

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

    ```
    bun add -D @tauri-apps/cli@latest
    ```

    ```
    cargo install tauri-cli --version "^2.0.0" --locked
    ```

3.  Determine the URL of your frontend development server. This is the
    URL that Tauri will use to load your content. For example, if you
    are using Vite, the default URL is `http://localhost:5173`.

4.  In your project directory, initialize Tauri:

    - [npm](#tab-panel-3510)
    - [yarn](#tab-panel-3511)
    - [pnpm](#tab-panel-3512)
    - [deno](#tab-panel-3513)
    - [bun](#tab-panel-3514)
    - [cargo](#tab-panel-3515)

    ```
    npx tauri init
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

    ```
    bun tauri init
    ```

    ```
    cargo tauri init
    ```

    After running the command it will display a prompt asking you for
    different options:

    ```
    ✔ What is your app name? tauri-app✔ What should the window title be? tauri-app✔ Where are your web assets located? ..✔ What is the url of your dev server? http://localhost:5173✔ What is your frontend dev command? pnpm run dev✔ What is your frontend build command? pnpm run build
    ```

    This will create a `src-tauri` directory in your project with the
    necessary Tauri configuration files.

5.  Configure the `server.watch.ignored` option in `vite.config.ts` to
    prevent Vite from watching the `src-tauri` directory:

    ```
    import { defineConfig } from "vite";
    export default defineConfig({  server: {    watch: {      ignored: ["**/src-tauri/**"],    },  },});
    ```

    vite.config.ts

6.  Verify your Tauri app is working by running the development server:

    - [npm](#tab-panel-3516)
    - [yarn](#tab-panel-3517)
    - [pnpm](#tab-panel-3518)
    - [deno](#tab-panel-3519)
    - [bun](#tab-panel-3520)
    - [cargo](#tab-panel-3521)

    ```
    npx tauri dev
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

    ```
    bun tauri dev
    ```

    ```
    cargo tauri dev
    ```

    This command will compile the Rust code and open a window with your
    web content.

**Congratulations!** You’ve created a new Tauri project using the Tauri
CLI! 🚀

## Next Steps

- [Learn about the project layout and what each file
  does](https://v2.tauri.app/start/project-structure/)
- [Add and Configure a Frontend Framework](https://v2.tauri.app/start/frontend/)
- [Tauri Command Line Interface (CLI) Reference](https://v2.tauri.app/reference/cli/)
- [Learn how to develop your Tauri app](https://v2.tauri.app/develop/)
- [Discover additional features to extend Tauri](https://v2.tauri.app/plugin/)

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
