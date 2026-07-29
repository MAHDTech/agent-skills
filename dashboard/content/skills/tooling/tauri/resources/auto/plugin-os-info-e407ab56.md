+++
title = "plugin-os-info-e407ab56"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# OS Information

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/os)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-os)[crates.io](https://crates.io/crates/tauri-plugin-os)

API
Reference[](https://v2.tauri.app/reference/javascript/os/)[](https://docs.rs/tauri-plugin-os)

Read information about the operating system using the OS Information
plugin.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the OS Information plugin to get started.

- [Automatic](#tab-panel-6489)
- [Manual](#tab-panel-6490)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6478)
- [yarn](#tab-panel-6479)
- [pnpm](#tab-panel-6480)
- [deno](#tab-panel-6481)
- [bun](#tab-panel-6482)
- [cargo](#tab-panel-6483)

```
npm run tauri add os
```

```
yarn run tauri add os
```

```
pnpm tauri add os
```

```
deno task tauri add os
```

```
bun tauri add os
```

```
cargo tauri add os
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-os
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_os::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  If you’d like to use in JavaScript then install the npm package as
    well:

    - [npm](#tab-panel-6484)
    - [yarn](#tab-panel-6485)
    - [pnpm](#tab-panel-6486)
    - [deno](#tab-panel-6487)
    - [bun](#tab-panel-6488)

    ```
    npm install @tauri-apps/plugin-os
    ```

    ```
    yarn add @tauri-apps/plugin-os
    ```

    ```
    pnpm add @tauri-apps/plugin-os
    ```

    ```
    deno add npm:@tauri-apps/plugin-os
    ```

    ```
    bun add @tauri-apps/plugin-os
    ```

## Usage

With this plugin you can query multiple information from current
operational system. See all available functions in the [JavaScript
API](https://v2.tauri.app/reference/javascript/os/) or [Rust
API](https://docs.rs/tauri-plugin-os/) references.

#### Example: OS Platform

`platform` returns a string describing the specific operating system in
use. The value is set at compile time. Possible values are `linux`,
`macos`, `ios`, `freebsd`, `dragonfly`, `netbsd`, `openbsd`, `solaris`,
`android`, `windows`.

- [JavaScript](#tab-panel-6476)
- [Rust](#tab-panel-6477)

```
import { platform } from '@tauri-apps/plugin-os';// when using `"withGlobalTauri": true`, you may use// const { platform } = window.__TAURI__.os;
const currentPlatform = platform();console.log(currentPlatform);// Prints "windows" to the console
```

```
let platform = tauri_plugin_os::platform();println!("Platform: {}", platform);// Prints "windows" to the terminal
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
{  "permissions": [    ...,    "os:default"  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This permission set configures which operating system information are
available to gather from the frontend.

#### [Granted Permissions](#granted-permissions)

All information except the host name are available.

#### This default permission set includes the following:

- `allow-arch`
- `allow-exe-extension`
- `allow-family`
- `allow-locale`
- `allow-os-type`
- `allow-platform`
- `allow-version`

## Permission Table

| Identifier | Description |
|----|----|
| `os:allow-arch` | Enables the arch command without any pre-configured scope. |
| `os:deny-arch` | Denies the arch command without any pre-configured scope. |
| `os:allow-exe-extension` | Enables the exe_extension command without any pre-configured scope. |
| `os:deny-exe-extension` | Denies the exe_extension command without any pre-configured scope. |
| `os:allow-family` | Enables the family command without any pre-configured scope. |
| `os:deny-family` | Denies the family command without any pre-configured scope. |
| `os:allow-hostname` | Enables the hostname command without any pre-configured scope. |
| `os:deny-hostname` | Denies the hostname command without any pre-configured scope. |
| `os:allow-locale` | Enables the locale command without any pre-configured scope. |
| `os:deny-locale` | Denies the locale command without any pre-configured scope. |
| `os:allow-os-type` | Enables the os_type command without any pre-configured scope. |
| `os:deny-os-type` | Denies the os_type command without any pre-configured scope. |
| `os:allow-platform` | Enables the platform command without any pre-configured scope. |
| `os:deny-platform` | Denies the platform command without any pre-configured scope. |
| `os:allow-version` | Enables the version command without any pre-configured scope. |
| `os:deny-version` | Denies the version command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
