+++
title = "plugin-deep-linking-ab3af9b1"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Deep Linking

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/deep-link)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-deep-link)[crates.io](https://crates.io/crates/tauri-plugin-deep-link)

API
Reference[](https://v2.tauri.app/reference/javascript/deep-link/)[](https://docs.rs/tauri-plugin-deep-link)

Set your Tauri application as the default handler for an URL.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the deep-link plugin to get started.

- [Automatic](#tab-panel-6319)
- [Manual](#tab-panel-6320)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-6308)
- [yarn](#tab-panel-6309)
- [pnpm](#tab-panel-6310)
- [deno](#tab-panel-6311)
- [bun](#tab-panel-6312)
- [cargo](#tab-panel-6313)

```
npm run tauri add deep-link
```

```
yarn run tauri add deep-link
```

```
pnpm tauri add deep-link
```

```
deno task tauri add deep-link
```

```
bun tauri add deep-link
```

```
cargo tauri add deep-link
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-deep-link@2.0.0
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_deep_link::init())        .run(tauri::generate_context!())        .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-6314)
    - [yarn](#tab-panel-6315)
    - [pnpm](#tab-panel-6316)
    - [deno](#tab-panel-6317)
    - [bun](#tab-panel-6318)

    ```
    npm install @tauri-apps/plugin-deep-link
    ```

    ```
    yarn add @tauri-apps/plugin-deep-link
    ```

    ```
    pnpm add @tauri-apps/plugin-deep-link
    ```

    ```
    deno add npm:@tauri-apps/plugin-deep-link
    ```

    ```
    bun add @tauri-apps/plugin-deep-link
    ```

## Setting up

### Android

There are two ways to open your app from links on Android:

1.  **App Links (http/https + host, verified)** For [app
    links](https://developer.android.com/training/app-links#android-app-links),
    you need a server with a `.well-known/assetlinks.json` endpoint that
    must return a text response in the given format:

```
[  {    "relation": ["delegate_permission/common.handle_all_urls"],    "target": {      "namespace": "android_app",      "package_name": "$APP_BUNDLE_ID",      "sha256_cert_fingerprints": [        $CERT_FINGERPRINT      ]    }  }]
```

.well-known/assetlinks.json

Where `$APP_BUNDLE_ID` is the value defined on
[`tauri.conf.json > identifier`](https://v2.tauri.app/reference/config/#identifier) with `-`
replaced with `_` and `$CERT_FINGERPRINT` is a list of SHA256
fingerprints of your app’s signing certificates, see [verify Android
applinks](https://developer.android.com/training/app-links/verify-android-applinks#web-assoc)
for more information.

2.  **Custom URI schemes (no host required, no verification)** For URIs
    like `myapp://...`, you can declare a custom scheme without hosting
    any files. Use the `scheme` field in the mobile configuration and
    omit the `host`.

### iOS

There are two ways to open your app from links on iOS:

1.  **Universal Links (https + host, verified)** For [universal
    links](https://developer.apple.com/documentation/xcode/allowing-apps-and-websites-to-link-to-your-content?language=objc),
    you need a server with a `.well-known/apple-app-site-association`
    endpoint that must return a JSON response in the given format:

```
{  "applinks": {    "details": [      {        "appIDs": ["$DEVELOPMENT_TEAM_ID.$APP_BUNDLE_ID"],        "components": [          {            "/": "/open/*",            "comment": "Matches any URL whose path starts with /open/"          }        ]      }    ]  }}
```

.well-known/apple-app-site-association

Where `$DEVELOPMENT_TEAM_ID` is the value defined on
`tauri.conf.json > bundle > iOS > developmentTeam` or the
`TAURI_APPLE_DEVELOPMENT_TEAM` environment variable and `$APP_BUNDLE_ID`
is the value defined on
[`tauri.conf.json > identifier`](https://v2.tauri.app/reference/config/#identifier).

To verify if your domain has been properly configured to expose the app
associations, you can run the following command, replacing `<host>` with
your actual host:

```
curl -v https://app-site-association.cdn-apple.com/a/v1/<host>
```

Terminal window

See
[applinks.details](https://developer.apple.com/documentation/bundleresources/applinks/details-swift.dictionary)
for more information.

2.  **Custom URI schemes (no host, no verification)** For URIs like
    `myapp://...`, you can declare a custom scheme under mobile
    configuration with `"appLink": false` (or omit it). The plugin
    generates the appropriate `CFBundleURLTypes` entries in your app’s
    Info.plist. No `.well-known` files or HTTPS host are needed.

### Desktop

On Linux and Windows deep links are delivered as a command line argument
to a new app process. The deep link plugin has integration with the
[single instance](https://v2.tauri.app/plugin/single-instance/) plugin if you prefer having
a unique app instance receiving the events.

- First you must add the `deep-link` feature to the single instance
  plugin:

```
[target."cfg(any(target_os = \"macos\", windows, target_os = \"linux\"))".dependencies]tauri-plugin-single-instance = { version = "2.0.0", features = ["deep-link"] }
```

src-tauri/Cargo.toml

- Then configure the single instance plugin which should always be the
  first plugin you register:

```
#[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]    {        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {          println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");          // when defining deep link schemes at runtime, you must also check `argv` here        }));    }
    builder = builder.plugin(tauri_plugin_deep_link::init());}
```

src-tauri/lib.rs

## Configuration

Under `tauri.conf.json > plugins > deep-link`, configure mobile
domains/schemes and desktop schemes you want to associate with your
application.

### Examples

**Custom scheme on mobile (no server required):**

```
{  "plugins": {    "deep-link": {      "mobile": [        {          "scheme": ["ovi"],          "appLink": false        }      ]    }  }}
```

tauri.conf.json

This registers the `ovi://*` scheme on Android and iOS.

**App Link / Universal Link (verified https + host):**

```
{  "plugins": {    "deep-link": {      "mobile": [        {          "scheme": ["https"],          "host": "your.website.com",          "pathPrefix": ["/open"],          "appLink": true        }      ]    }  }}
```

This registers `https://your.website.com/open/*` as an app/universal
link.

**Desktop custom schemes:**

```
{  "plugins": {    "deep-link": {      "desktop": {        "schemes": ["something", "my-tauri-app"]      }    }  }}
```

## Usage

The deep-link plugin is available in both JavaScript and Rust.

### Listening to Deep Links

- [JavaScript](#tab-panel-6306)
- [Rust](#tab-panel-6307)

When a deep link triggers your app while it’s running, the `onOpenUrl`
callback is called. To detect whether your app was opened via a deep
link, use `getCurrent` on app start.

```
import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';// when using `"withGlobalTauri": true`, you may use// const { getCurrent, onOpenUrl } = window.__TAURI__.deepLink;
const startUrls = await getCurrent();if (startUrls) {  // App was likely started via a deep link  // Note that getCurrent's return value will also get updated every time onOpenUrl gets triggered.}
await onOpenUrl((urls) => {  console.log('deep link:', urls);});
```

When a deep link triggers your app while it’s running, the plugin’s
`on_open_url` closure is called. To detect whether your app was opened
via a deep link, use `get_current` on app start.

```
use tauri_plugin_deep_link::DeepLinkExt;
#[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_deep_link::init())        .setup(|app| {            // Note that get_current's return value will also get updated every time on_open_url gets triggered.            let start_urls = app.deep_link().get_current()?;            if let Some(urls) = start_urls {                // app was likely started by a deep link                println!("deep link URLs: {:?}", urls);            }
            app.deep_link().on_open_url(|event| {                println!("deep link URLs: {:?}", event.urls());            });            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
```

src-tauri/src/lib.rs

### Registering Desktop Deep Links at Runtime

The [configuration](#configuration) section describes how to define
static deep link schemes for your application.

On Linux and Windows it is possible to also associate schemes with your
application at runtime via the `register` Rust function.

In the following snippet, we will register the `my-app` scheme at
runtime. After executing the app for the first time, the operating
system will open `my-app://*` URLs with our application:

```
use tauri_plugin_deep_link::DeepLinkExt;
#[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_deep_link::init())        .setup(|app| {            #[cfg(desktop)]            app.deep_link().register("my-app")?;            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
```

src-tauri/src/lib.rs

## Testing

There are some caveats to test deep links for your application.

### Desktop

Deep links are only triggered for installed applications on desktop. On
Linux and Windows you can circumvent this using the
[`register_all`](https://docs.rs/tauri-plugin-deep-link/2.0.0/tauri_plugin_deep_link/struct.DeepLink.html#method.register_all)
Rust function, which registers all configured schemes to trigger the
current executable:

```
use tauri_plugin_deep_link::DeepLinkExt;
#[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    tauri::Builder::default()        .plugin(tauri_plugin_deep_link::init())        .setup(|app| {            #[cfg(any(windows, target_os = "linux"))]            {                use tauri_plugin_deep_link::DeepLinkExt;                app.deep_link().register_all()?;            }            Ok(())        })        .run(tauri::generate_context!())        .expect("error while running tauri application");}
```

src-tauri/src/lib.rs

#### Windows

To trigger a deep link on Windows you can either open `<scheme>://url`
in the browser or run the following command in the terminal:

```
start <scheme>://url
```

Terminal window

#### Linux

To trigger a deep link on Linux you can either open `<scheme>://url` in
the browser or run `xdg-open` in the terminal:

```
xdg-open <scheme>://url
```

Terminal window

### iOS

To trigger an app link on iOS you can open the `https://<host>/path` URL
in the browser. For simulators you can leverage the `simctl` CLI to
directly open a link from the terminal:

```
xcrun simctl openurl booted https://<host>/path
```

Terminal window

### Android

To trigger an app link on Android you can open the `https://<host>/path`
URL in the browser. For emulators you can leverage the `adb` CLI to
directly open a link from the terminal:

```
adb shell am start -a android.intent.action.VIEW -d https://<host>/path <bundle-identifier>
```

Terminal window

## Permissions

By default all potentially dangerous plugin commands and scopes are
blocked and cannot be accessed. You must modify the permissions in your
`capabilities` configuration to enable these.

See the [Capabilities Overview](https://v2.tauri.app/security/capabilities/) for more
information and the [step by step
guide](https://v2.tauri.app/learn/security/using-plugin-permissions/) to use plugin
permissions.

```
{  "$schema": "../gen/schemas/mobile-schema.json",  "identifier": "mobile-capability",  "windows": ["main"],  "platforms": ["iOS", "android"],  "permissions": [    // Usually you will need core:event:default to listen to the deep-link event    "core:event:default",    "deep-link:default"  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

Allows reading the opened deep link via the get_current command

#### This default permission set includes the following:

- `allow-get-current`

## Permission Table

| Identifier | Description |
|----|----|
| `deep-link:allow-get-current` | Enables the get_current command without any pre-configured scope. |
| `deep-link:deny-get-current` | Denies the get_current command without any pre-configured scope. |
| `deep-link:allow-is-registered` | Enables the is_registered command without any pre-configured scope. |
| `deep-link:deny-is-registered` | Denies the is_registered command without any pre-configured scope. |
| `deep-link:allow-register` | Enables the register command without any pre-configured scope. |
| `deep-link:deny-register` | Denies the register command without any pre-configured scope. |
| `deep-link:allow-unregister` | Enables the unregister command without any pre-configured scope. |
| `deep-link:deny-unregister` | Denies the unregister command without any pre-configured scope. |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
