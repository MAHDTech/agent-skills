# Announcing the Tauri Mobile Alpha Release

Dec 9, 2022

![Lucas Nogueira](https://v2.tauri.app/authors/lucasfernog.jpeg)

Lucas Nogueira

Tauri Co-Founder

![Tauri 2.0 Launch Hero Image](https://v2.tauri.app/_astro/header.DJC8YrJ3_rv4kd.webp)

Tauri mobile is here! The first alpha release 2.0.0-alpha.0 has been
published.

## Updating dependencies

Make sure to update both NPM and Cargo dependencies to the 2.0.0-alpha.0
release. You can update the dependencies with:

- [npm](#tab-panel-4398)
- [yarn](#tab-panel-4399)
- [pnpm](#tab-panel-4400)
- [cargo](#tab-panel-4401)

```
npm install @tauri-apps/cli@next @tauri-apps/api@next
```

```
yarn upgrade @tauri-apps/cli@next @tauri-apps/api@next
```

```
pnpm update @tauri-apps/cli@next @tauri-apps/api@next
```

```
cargo add tauri@2.0.0-alpha.0cargo add tauri-build@2.0.0-alpha.0 --buildcargo install tauri-cli --version "^2.0.0-alpha" --locked
```

## Preview

You can adapt your existing desktop application to run on mobile or
start a fresh project. Tauri runs on the connected device or starts an
emulator if available.

![iOS Preview](https://v2.tauri.app/_astro/ios-preview.au3ri0xF_1nrnxq.webp)![Android
Preview](https://v2.tauri.app/_astro/android-preview.nQXuMXya_Z2uNsjm.webp)

------------------------------------------------------------------------

## Getting started

Read the complete guide on the [`next` documentation
website](https://v2.tauri.app).

## Known issues

- TLS support has been moved behind a Cargo feature until we figure out
  how to cross compile OpenSSL on Windows.
- Currently running on a device is not supported when using Xcode 14.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
