# Barcode Scanner

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/barcode-scanner)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-barcode-scanner)[crates.io](https://crates.io/crates/tauri-plugin-barcode-scanner)

API
Reference[](https://v2.tauri.app/reference/javascript/barcode-scanner/)[](https://docs.rs/tauri-plugin-barcode-scanner)

Allows your mobile application to use the camera to scan QR codes,
EAN-13 and other kinds of barcodes.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the barcode-scanner plugin to get started.

- [Automatic](#tab-panel-6257)
- [Manual](#tab-panel-6258)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6246)
- [yarn](#tab-panel-6247)
- [pnpm](#tab-panel-6248)
- [deno](#tab-panel-6249)
- [bun](#tab-panel-6250)
- [cargo](#tab-panel-6251)

```
npm run tauri add barcode-scanner
```

```
yarn run tauri add barcode-scanner
```

```
pnpm tauri add barcode-scanner
```

```
deno task tauri add barcode-scanner
```

```
bun tauri add barcode-scanner
```

```
cargo tauri add barcode-scanner
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-barcode-scanner --target 'cfg(any(target_os = "android", target_os = "ios"))'
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .setup(|app| {            #[cfg(mobile)]            app.handle().plugin(tauri_plugin_barcode_scanner::init());            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6252)
    - [yarn](#tab-panel-6253)
    - [pnpm](#tab-panel-6254)
    - [deno](#tab-panel-6255)
    - [bun](#tab-panel-6256)

    ```
    npm install @tauri-apps/plugin-barcode-scanner
    ```

    ```
    yarn add @tauri-apps/plugin-barcode-scanner
    ```

    ```
    pnpm add @tauri-apps/plugin-barcode-scanner
    ```

    ```
    deno add npm:@tauri-apps/plugin-barcode-scanner
    ```

    ```
    bun add @tauri-apps/plugin-barcode-scanner
    ```

## Configuration

On iOS the barcode scanner plugin requires the
`NSCameraUsageDescription` information property list value, which should
describe why your app needs to use the camera.

In the `src-tauri/Info.ios.plist` file, add the following snippet:

```
<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0">  <dict>    <key>NSCameraUsageDescription</key>    <string>Read QR codes</string>  </dict></plist>
```

src-tauri/Info.ios.plist

## Usage

The barcode scanner plugin is available in JavaScript.

```
import { scan, Format } from '@tauri-apps/plugin-barcode-scanner';// when using `"withGlobalTauri": true`, you may use// const { scan, Format } = window.__TAURI__.barcodeScanner;
// `windowed: true` actually sets the webview to transparent// instead of opening a separate view for the camera// make sure your user interface is ready to show what is underneath with a transparent elementscan({ windowed: true, formats: [Format.QRCode] });
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
{  "$schema": "../gen/schemas/mobile-schema.json",  "identifier": "mobile-capability",  "windows": ["main"],  "platforms": ["iOS", "android"],  "permissions": ["barcode-scanner:allow-scan", "barcode-scanner:allow-cancel"]}
```

src-tauri/capabilities/mobile.json

## [Default Permission](#default-permission)

This permission set configures which barcode scanning features are by
default exposed.

#### [Granted Permissions](#granted-permissions)

It allows all barcode related features.

#### This default permission set includes the following:

- `allow-cancel`
- `allow-check-permissions`
- `allow-open-app-settings`
- `allow-request-permissions`
- `allow-scan`
- `allow-vibrate`

## Permission Table

| Identifier | Description |
|----|----|
| `barcode-scanner:allow-cancel` | Enables the cancel command without any pre-configured scope. |
| `barcode-scanner:deny-cancel` | Denies the cancel command without any pre-configured scope. |
| `barcode-scanner:allow-check-permissions` | Enables the check_permissions command without any pre-configured scope. |
| `barcode-scanner:deny-check-permissions` | Denies the check_permissions command without any pre-configured scope. |
| `barcode-scanner:allow-open-app-settings` | Enables the open_app_settings command without any pre-configured scope. |
| `barcode-scanner:deny-open-app-settings` | Denies the open_app_settings command without any pre-configured scope. |
| `barcode-scanner:allow-request-permissions` | Enables the request_permissions command without any pre-configured scope. |
| `barcode-scanner:deny-request-permissions` | Denies the request_permissions command without any pre-configured scope. |
| `barcode-scanner:allow-scan` | Enables the scan command without any pre-configured scope. |
| `barcode-scanner:deny-scan` | Denies the scan command without any pre-configured scope. |
| `barcode-scanner:allow-vibrate` | Enables the vibrate command without any pre-configured scope. |
| `barcode-scanner:deny-vibrate` | Denies the vibrate command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
