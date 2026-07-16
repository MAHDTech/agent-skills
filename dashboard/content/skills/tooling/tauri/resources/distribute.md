+++
title = "distribute"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Distribute

Tauri provides the tooling you need to distribute your application
either to the platform app stores or as platform-specific installers.

## Building

Tauri builds your application directly from its CLI via the `build`,
`android build` and `ios build` commands.

- [npm](#tab-panel-1186)
- [yarn](#tab-panel-1187)
- [pnpm](#tab-panel-1188)
- [deno](#tab-panel-1189)
- [bun](#tab-panel-1190)
- [cargo](#tab-panel-1191)

```
npm run tauri build
```

```
yarn tauri build
```

```
pnpm tauri build
```

```
deno task tauri build
```

```
bun tauri build
```

```
cargo tauri build
```

See the [distributing](#distributing) section to learn more about the
configuration options available for each bundle and how to distribute
them to your users.

### Bundling

By default the `build` command automatically bundles your application
for the configured formats.

If you need further customization on how the platform bundles are
generated, you can split the build and bundle steps:

- [npm](#tab-panel-1192)
- [yarn](#tab-panel-1193)
- [pnpm](#tab-panel-1194)
- [deno](#tab-panel-1195)
- [bun](#tab-panel-1196)
- [cargo](#tab-panel-1197)

```
npm run tauri build -- --no-bundle# bundle for distribution outside the macOS App Storenpm run tauri bundle -- --bundles app,dmg# bundle for App Store distributionnpm run tauri bundle -- --bundles app --config src-tauri/tauri.appstore.conf.json
```

```
yarn tauri build --no-bundle# bundle for distribution outside the macOS App Storeyarn tauri bundle --bundles app,dmg# bundle for App Store distributionyarn tauri bundle --bundles app --config src-tauri/tauri.appstore.conf.json
```

```
pnpm tauri build --no-bundle# bundle for distribution outside the macOS App Storepnpm tauri bundle --bundles app,dmg# bundle for App Store distributionpnpm tauri bundle --bundles app --config src-tauri/tauri.appstore.conf.json
```

```
deno task tauri build --no-bundle# bundle for distribution outside the macOS App Storedeno task tauri bundle --bundles app,dmg# bundle for App Store distributiondeno task tauri bundle --bundles app --config src-tauri/tauri.appstore.conf.json
```

```
bun tauri build --no-bundle# bundle for distribution outside the macOS App Storebun tauri bundle --bundles app,dmg# bundle for App Store distributionbun tauri bundle --bundles app --config src-tauri/tauri.appstore.conf.json
```

```
cargo tauri build --no-bundle# bundle for distribution outside the macOS App Storecargo tauri bundle --bundles app,dmg# bundle for App Store distributioncargo tauri bundle --bundles app --config src-tauri/tauri.appstore.conf.json
```

## Versioning

Your application version can be defined in the
[`tauri.conf.json > version`](/reference/config/#version) configuration
option, which is the recommended way for managing the app version. If
that config value is not set, Tauri uses the `package > version` value
from your `src-tauri/Cargo.toml` file instead.

## Signing

Code signing enhances the security of your application by applying a
digital signature to your application’s executables and bundles,
validating your identity of the provider of your application.

Signing is required on most platforms. See the documentation for each
platform for more information.

[macOS](/distribute/sign/macos/)Code signing and notarization for macOS
apps

[Windows](/distribute/sign/windows/)Code signing Windows installers

[Linux](/distribute/sign/linux/)Code signing Linux packages

[Android](/distribute/sign/android/)Code signing for Android

[iOS](/distribute/sign/ios/)Code signing for iOS

## Distributing

Learn how to distribute your application for each platform.

### Linux

For Linux you can distribute your app using the Debian package, Snap,
AppImage, Flatpak, RPM or Arch User Repository (AUR) formats.

[AppImage](/distribute/appimage/)Distribute as an AppImage

[AUR](/distribute/aur/)Publishing To The Arch User Repository

[Debian](/distribute/debian/)Distribute as a Debian package

[RPM](/distribute/rpm/)Distribute as an RPM package

[Snapcraft](/distribute/snapcraft/)Distribute on Snapcraft.io

[Code signing](/distribute/sign/linux/)

### macOS

For macOS you can either distribute your application directly to the App
Store or ship a DMG installer as direct download. Both methods requires
code signing, and distributing outside the App Store also requires
notarization.

[App Bundle](/distribute/macos-application-bundle/)Distribute macOS apps
as an App Bundle

[App Store](/distribute/app-store/)Distribute iOS and macOS apps to the
App Store

[DMG](/distribute/dmg/)Distribute macOS apps as Apple Disk Images

[](/distribute/sign/macos/)

Code signing and notarization

### Windows

Learn how to distribute to the Microsoft Store or configure a Windows
installer.

[Microsoft Store](/distribute/microsoft-store/)Distribute Windows apps
to the Microsoft Store

[Windows Installer](/distribute/windows-installer/)Distribute installers
for Windows

[Code signing](/distribute/sign/windows/)

### Android

Distribute your Android application to Google Play.

[Google Play](/distribute/google-play/)Distribute Android apps to Google
Play

[Code signing](/distribute/sign/android/)

### iOS

Learn how to upload your application to the App Store.

[App Store](/distribute/app-store/)Distribute iOS and macOS apps to the
App Store

[Code signing](/distribute/sign/ios/)

### Cloud Services

Distribute your application to Cloud services that globally distribute
your application and support auto updates out of the box.

[CrabNebula Cloud](/distribute/crabnebula-cloud/)Distribute your app
using CrabNebula

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

