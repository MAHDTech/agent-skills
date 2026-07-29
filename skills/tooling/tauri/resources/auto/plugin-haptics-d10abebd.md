# Haptics

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/haptics)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-haptics)[crates.io](https://crates.io/crates/tauri-plugin-haptics)

API
Reference[](https://v2.tauri.app/reference/javascript/haptics/)[](https://docs.rs/tauri-plugin-haptics)

Haptic feedback and vibrations on Android and iOS.

There are no standards/requirements for vibration support on Android, so
the feedback APIs may not work correctly on more affordable phones,
including recently released ones.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the haptics plugin to get started.

- [Automatic](#tab-panel-6388)
- [Manual](#tab-panel-6389)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6377)
- [yarn](#tab-panel-6378)
- [pnpm](#tab-panel-6379)
- [deno](#tab-panel-6380)
- [bun](#tab-panel-6381)
- [cargo](#tab-panel-6382)

```
npm run tauri add haptics
```

```
yarn run tauri add haptics
```

```
pnpm tauri add haptics
```

```
deno task tauri add haptics
```

```
bun tauri add haptics
```

```
cargo tauri add haptics
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-haptics --target 'cfg(any(target_os = "android", target_os = "ios"))'
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .setup(|app| {            #[cfg(mobile)]            app.handle().plugin(tauri_plugin_haptics::init());            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6383)
    - [yarn](#tab-panel-6384)
    - [pnpm](#tab-panel-6385)
    - [deno](#tab-panel-6386)
    - [bun](#tab-panel-6387)

    ```
    npm install @tauri-apps/plugin-haptics
    ```

    ```
    yarn add @tauri-apps/plugin-haptics
    ```

    ```
    pnpm add @tauri-apps/plugin-haptics
    ```

    ```
    deno add npm:@tauri-apps/plugin-haptics
    ```

    ```
    bun add @tauri-apps/plugin-haptics
    ```

## Usage

The haptics plugin is available in JavaScript.

```
import {  vibrate,  impactFeedback,  notificationFeedback,  selectionFeedback,} from '@tauri-apps/plugin-haptics';
await vibrate(1);await impactFeedback('medium');await notificationFeedback('warning');await selectionFeedback();
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
{  "$schema": "../gen/schemas/mobile-schema.json",  "identifier": "mobile-capability",  "windows": ["main"],  "platforms": ["iOS", "android"],  "permissions": [    "haptics:allow-impact-feedback",    "haptics:allow-notification-feedback",    "haptics:allow-selection-feedback",    "haptics:allow-vibrate"  ]}
```

src-tauri/capabilities/mobile.json

## [Permission Table](#permission-table)

| Identifier | Description |
|----|----|
| `haptics:allow-impact-feedback` | Enables the impact_feedback command without any pre-configured scope. |
| `haptics:deny-impact-feedback` | Denies the impact_feedback command without any pre-configured scope. |
| `haptics:allow-notification-feedback` | Enables the notification_feedback command without any pre-configured scope. |
| `haptics:deny-notification-feedback` | Denies the notification_feedback command without any pre-configured scope. |
| `haptics:allow-selection-feedback` | Enables the selection_feedback command without any pre-configured scope. |
| `haptics:deny-selection-feedback` | Denies the selection_feedback command without any pre-configured scope. |
| `haptics:allow-vibrate` | Enables the vibrate command without any pre-configured scope. |
| `haptics:deny-vibrate` | Denies the vibrate command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
