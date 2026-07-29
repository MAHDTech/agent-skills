+++
title = "plugin-geolocation-079dcc79"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Geolocation

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/geolocation)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-geolocation)[crates.io](https://crates.io/crates/tauri-plugin-geolocation)

API
Reference[](https://v2.tauri.app/reference/javascript/geolocation/)[](https://docs.rs/tauri-plugin-geolocation)

Get and track the device’s current position, including information about
altitude, heading, and speed (if available).

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the geolocation plugin to get started.

- [Automatic](#tab-panel-6360)
- [Manual](#tab-panel-6361)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6349)
- [yarn](#tab-panel-6350)
- [pnpm](#tab-panel-6351)
- [deno](#tab-panel-6352)
- [bun](#tab-panel-6353)
- [cargo](#tab-panel-6354)

```
npm run tauri add geolocation
```

```
yarn run tauri add geolocation
```

```
pnpm tauri add geolocation
```

```
deno task tauri add geolocation
```

```
bun tauri add geolocation
```

```
cargo tauri add geolocation
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-geolocation --target 'cfg(any(target_os = "android", target_os = "ios"))'
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .setup(|app| {            #[cfg(mobile)]            app.handle().plugin(tauri_plugin_geolocation::init());            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6355)
    - [yarn](#tab-panel-6356)
    - [pnpm](#tab-panel-6357)
    - [deno](#tab-panel-6358)
    - [bun](#tab-panel-6359)

    ```
    npm install @tauri-apps/plugin-geolocation
    ```

    ```
    yarn add @tauri-apps/plugin-geolocation
    ```

    ```
    pnpm add @tauri-apps/plugin-geolocation
    ```

    ```
    deno add npm:@tauri-apps/plugin-geolocation
    ```

    ```
    bun add @tauri-apps/plugin-geolocation
    ```

## Configuration

### iOS

Apple requires privacy descriptions to be specified in Info.plist for
location information, where you should describe why your app needs to
access it. Illustrated below is an example description:

```
<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0">    <dict>        <key>NSLocationWhenInUseUsageDescription</key>        <string>Required to do XY</string>    </dict></plist>
```

### Android

This plugin automatically adds the following permissions to your
`AndroidManifest.xml` file:

```
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" /><uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
```

If your app requires GPS functionality to function, you should add the
following to your `AndroidManifest.xml` file:

```
<uses-feature android:name="android.hardware.location.gps" android:required="true" />
```

The Google Play Store uses this property to decide whether it should
show the app to devices without GPS capabilities.

## Usage

The geolocation plugin is available in JavaScript.

```
import {  checkPermissions,  requestPermissions,  getCurrentPosition,  watchPosition,} from '@tauri-apps/plugin-geolocation';
let permissions = await checkPermissions();if (  permissions.location === 'prompt' ||  permissions.location === 'prompt-with-rationale') {  permissions = await requestPermissions(['location']);}
if (permissions.location === 'granted') {  const pos = await getCurrentPosition();
  await watchPosition(    { enableHighAccuracy: true, timeout: 10000, maximumAge: 0 },    (pos) => {      console.log(pos);    }  );}
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
{  "$schema": "../gen/schemas/mobile-schema.json",  "identifier": "mobile-capability",  "windows": ["main"],  "platforms": ["iOS", "android"],  "permissions": [    "core:default",    "geolocation:allow-check-permissions",    "geolocation:allow-request-permissions",    "geolocation:allow-get-current-position",    "geolocation:allow-watch-position"  ]}
```

src-tauri/capabilities/mobile.json

## [Permission Table](#permission-table)

| Identifier | Description |
|----|----|
| `geolocation:allow-check-permissions` | Enables the check_permissions command without any pre-configured scope. |
| `geolocation:deny-check-permissions` | Denies the check_permissions command without any pre-configured scope. |
| `geolocation:allow-clear-permissions` | Enables the clear_permissions command without any pre-configured scope. |
| `geolocation:deny-clear-permissions` | Denies the clear_permissions command without any pre-configured scope. |
| `geolocation:allow-clear-watch` | Enables the clear_watch command without any pre-configured scope. |
| `geolocation:deny-clear-watch` | Denies the clear_watch command without any pre-configured scope. |
| `geolocation:allow-get-current-position` | Enables the get_current_position command without any pre-configured scope. |
| `geolocation:deny-get-current-position` | Denies the get_current_position command without any pre-configured scope. |
| `geolocation:allow-request-permissions` | Enables the request_permissions command without any pre-configured scope. |
| `geolocation:deny-request-permissions` | Denies the request_permissions command without any pre-configured scope. |
| `geolocation:allow-watch-position` | Enables the watch_position command without any pre-configured scope. |
| `geolocation:deny-watch-position` | Denies the watch_position command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
