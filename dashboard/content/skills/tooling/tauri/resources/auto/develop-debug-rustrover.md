+++
title = "develop-debug-rustrover"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Debug in JetBrains IDEs

In this guide, we’ll be setting up JetBrains RustRover for debugging the
[Core Process of your Tauri
app](https://v2.tauri.app/concept/process-model/#the-core-process). It also mostly applies
to IntelliJ and CLion.

## Setting up a Cargo project

Depending on which frontend stack is used in a project, the project
directory may or may not be a Cargo project. By default, Tauri places
the Rust project in a subdirectory called `src-tauri`. It creates a
Cargo project in the root directory only if Rust is used for frontend
development as well.

If there’s no `Cargo.toml` file at the top level, you need to attach the
project manually. Open the Cargo tool window (in the main menu, go to
**View \| Tool Windows \| Cargo**), click **+** (**Attach Cargo
Project**) on the toolbar, and select the `src-tauri/Cargo.toml` file.

Alternatively, you could create a top-level Cargo workspace manually by
adding the following file to the project’s root directory:

```
[workspace]members = ["src-tauri"]
```

Cargo.toml

Before you proceed, make sure that your project is fully loaded. If the
Cargo tool window shows all the modules and targets of the workspace,
you’re good to go.

## Setting up Run Configurations

You will need to set up two separate Run/Debug configurations:

- one for launching the Tauri app in debugging mode,
- another one for running your frontend development server of choice.

### Tauri App

1.  In the main menu, go to **Run \| Edit Configurations**.
2.  In the **Run/Debug Configurations** dialog:

- To create a new configuration, click **+** on the toolbar and select
  **Cargo**.

![Add Run/Debug
Configuration](https://v2.tauri.app/_astro/add-cargo-config-light.Bfob59bJ_1pRmgh.webp)

With that created, we need to configure RustRover, so it instructs Cargo
to build our app without any default features. This will tell Tauri to
use your development server instead of reading assets from the disk.
Normally this flag is passed by the Tauri CLI, but since we’re
completely sidestepping that here, we need to pass the flag manually.

![Add --no-default-features
flag](https://v2.tauri.app/_astro/set-no-default-features-light.CcoA2Q8U_tmggj.webp)

Now we can optionally rename the Run/Debug Configuration to something
more memorable, in this example we called it “Run Tauri App”, but you
can name it whatever you want.

![Rename
Configuration](https://v2.tauri.app/_astro/rename-configuration-light.B4TFjRby_Z1MQo6T.webp)

### Development Server

The above configuration will use Cargo directly to build the Rust
application and attach the debugger to it. This means we completely
sidestep the Tauri CLI, so features like the `beforeDevCommand` and
`beforeBuildCommand` will **not** be executed. We need to take care of
that by running the development server manually.

To create the corresponding Run configuration, you need to check the
actual development server in use. Look for the
`src-tauri/tauri.conf.json` file and find the following line:

```
    "beforeDevCommand": "pnpm dev"
```

For `npm`, `pnpm`, or `yarn`, you could use the **npm** Run
Configuration, for example:

![NPM
Configuration](https://v2.tauri.app/_astro/npm-configuration-light.B2Rk0eaZ_Z1NO8qD.webp)

Make sure you have the correct values in the **Command**, **Scripts**,
and **Package Manager** fields.

If your development server is `trunk` for Rust-based WebAssembly
frontend frameworks, you could use the generic **Shell Script** Run
Configuration:

![Trunk Serve
Configuration](https://v2.tauri.app/_astro/trunk-configuration-light.78simsDr_1bNJJx.webp)

## Launching a Debugging Session

To launch a debugging session, you first need to run your development
server, and then start debugging the Tauri App by clicking the **Debug**
button next to the Run Configurations Switcher. RustRover will
automatically recognize breakpoints placed in any Rust file in your
project and stop on the first one hit.

![Debug Session](https://v2.tauri.app/_astro/debug-session-light.BcVudV4z_ZIN7aw.webp)

From this point, you can explore the values of your variables, step
further into the code, and check what’s going at runtime in detail.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

