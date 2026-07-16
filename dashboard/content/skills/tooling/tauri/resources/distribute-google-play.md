+++
title = "distribute-google-play"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Google Play

Google Play is the Android app distribution service maintained by
Google.

This guide covers the requirements for publishing your Android app on
Google Play.

## Requirements

To distribute Android apps in the Play Store you must create a [Play
Console](https://play.google.com/console/developers) developer account.

Additionally, you must setup [code signing](/distribute/sign/android/).

See the [release
checklist](https://play.google.com/console/about/guides/releasewithconfidence/)
for more information.

## Changing App Icon

After running `tauri android init` to setup the Android Studio project,
you can use the `tauri icon` command to update the app icons.

- [npm](#tab-panel-1228)
- [yarn](#tab-panel-1229)
- [pnpm](#tab-panel-1230)
- [deno](#tab-panel-1231)
- [bun](#tab-panel-1232)
- [cargo](#tab-panel-1233)

```
npm run tauri icon /path/to/app-icon.png
```

```
yarn tauri icon /path/to/app-icon.png
```

```
pnpm tauri icon /path/to/app-icon.png
```

```
deno task tauri icon /path/to/app-icon.png
```

```
bun tauri icon /path/to/app-icon.png
```

```
cargo tauri icon /path/to/app-icon.png
```

## Setting up

Once you’ve created a Play Console developer account, you need to
register your app on the Google [Play
Console](https://play.google.com/console/developers) website. It will
guide you through all the required forms and setup tasks.

## Build

You can build an Android App Bundle (AAB) to upload to Google Play by
running the following command:

- [npm](#tab-panel-1234)
- [yarn](#tab-panel-1235)
- [pnpm](#tab-panel-1236)
- [deno](#tab-panel-1237)
- [bun](#tab-panel-1238)
- [cargo](#tab-panel-1239)

```
npm run tauri android build -- --aab
```

```
yarn tauri android build --aab
```

```
pnpm tauri android build --aab
```

```
deno task tauri android build --aab
```

```
bun tauri android build --aab
```

```
cargo tauri android build --aab
```

Tauri derives the version code from the value defined in
[`tauri.conf.json > version`](/reference/config/#version)
(`versionCode = major*1000000 + minor*1000 + patch`). You can set a
custom version code in the
\[`tauri.conf.json > bundle > android > versionCode`\] configuration if
you need a different version code scheme e.g. sequential codes:

```
{  "bundle": {    "android": {      "versionCode": 100    }  }}
```

tauri.conf.json

### Build APKs

The AAB format is the recommended bundle file to upload to Google Play,
but it is also possible to generate APKs that can be used for testing or
distribution outside the store. To compile APKs for your app you can use
the `--apk` argument:

- [npm](#tab-panel-1240)
- [yarn](#tab-panel-1241)
- [pnpm](#tab-panel-1242)
- [deno](#tab-panel-1243)
- [bun](#tab-panel-1244)
- [cargo](#tab-panel-1245)

```
npm run tauri android build -- --apk
```

```
yarn tauri android build --apk
```

```
pnpm tauri android build --apk
```

```
deno task tauri android build --apk
```

```
bun tauri android build --apk
```

```
cargo tauri android build --apk
```

### Architecture selection

By default Tauri builds your app for all supported architectures
(aarch64, armv7, i686 and x86_64). To only compile for a subset of
targets, you can use the `--target` argument:

- [npm](#tab-panel-1246)
- [yarn](#tab-panel-1247)
- [pnpm](#tab-panel-1248)
- [deno](#tab-panel-1249)
- [bun](#tab-panel-1250)
- [cargo](#tab-panel-1251)

```
npm run tauri android build -- --aab --target aarch64 --target armv7
```

```
yarn tauri android build --aab --target aarch64 --target armv7
```

```
pnpm tauri android build --aab --target aarch64 --target armv7
```

```
deno task tauri android build --aab --target aarch64 --target armv7
```

```
bun tauri android build --aab --target aarch64 --target armv7
```

```
cargo tauri android build --aab --target aarch64 --target armv7
```

### Separate bundles per architecture

By default the generated AAB and APK is universal, containing all
supported targets. To generate individual bundles per target, use the
`--split-per-abi` argument.

- [npm](#tab-panel-1252)
- [yarn](#tab-panel-1253)
- [pnpm](#tab-panel-1254)
- [deno](#tab-panel-1255)
- [bun](#tab-panel-1256)
- [cargo](#tab-panel-1257)

```
npm run tauri android build -- --apk --split-per-abi
```

```
yarn tauri android build --apk --split-per-abi
```

```
pnpm tauri android build --apk --split-per-abi
```

```
deno task tauri android build --apk --split-per-abi
```

```
bun tauri android build --apk --split-per-abi
```

```
cargo tauri android build --apk --split-per-abi
```

### Changing the minimum supported Android version

The minimum supported Android version for Tauri apps is Android 7.0
(codename Nougat, SDK 24).

There are some techniques to use newer Android APIs while still
supporting older systems. See the [Android
documentation](https://developer.android.com/training/basics/supporting-devices/platforms#version-codes)
for more information.

If your app must execute on a newer Android version, you can configure
\[`tauri.conf.json > bundle > android > minSdkVersion`\]:

```
{  "bundle": {    "android": {      "minSdkVersion": 28    }  }}
```

tauri.conf.json

## Upload

After building your app and generating the Android App Bundle file,
which can be found in
`gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab`,
you can now create a new release and upload it in the Google Play
Console.

The first upload must be made manually in the website so it can verify
your app signature and bundle identifier. Tauri currently does not offer
a way to automate the process of creating Android releases, which must
leverage the [Google Play Developer
API](https://developers.google.com/android-publisher/api-ref/rest), but
it is a work in progress.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

