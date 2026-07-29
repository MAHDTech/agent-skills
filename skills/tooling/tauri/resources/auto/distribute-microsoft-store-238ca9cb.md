# Microsoft Store

Microsoft Store is the Windows app store operated by Microsoft.

This guide only covers details for distributing Windows Apps directly to
the Microsoft Store. See the [Windows Installer
guide](https://v2.tauri.app/distribute/windows-installer/) for more information on Windows
installer distribution options and configurations.

## Requirements

To publish apps on the Microsoft Store you must have a Microsoft account
and
[enroll](https://learn.microsoft.com/en-us/windows/apps/get-started/sign-up)
as a developer either as an individual or as a company.

## Changing App Icon

The Tauri CLI can generate all icons your app needs, including Microsoft
Store icons. Use the `tauri icon` command to generate app icons from a
single PNG or SVG source:

- [npm](#tab-panel-4698)
- [yarn](#tab-panel-4699)
- [pnpm](#tab-panel-4700)
- [deno](#tab-panel-4701)
- [bun](#tab-panel-4702)
- [cargo](#tab-panel-4703)

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

After you have enrolled as a developer with your Microsoft account you
need to register your app in the [Apps and
Games](https://partner.microsoft.com/en-us/dashboard/apps-and-games/overview)
page. Click `New Product`, select `EXE or MSI app` and reserve a unique
name for your app.

## Build and upload

Currently Tauri only generates [EXE and
MSI](https://v2.tauri.app/distribute/windows-installer/) installers, so you must create a
Microsoft Store application that only links to the unpacked application.
The installer linked in the Microsoft Installer must be offline, [handle
auto-updates](https://v2.tauri.app/plugin/updater/) and be [code
signed](https://v2.tauri.app/distribute/sign/windows/).

See the [official publish
documentation](https://learn.microsoft.com/en-us/windows/apps/publish/)
for more information.

### Offline Installer

The Windows installer distributed through the Microsoft Store must use
the [Offline
Installer](https://v2.tauri.app/distribute/windows-installer/#offline-installer) Webview2
installation option.

To only apply this installer configuration when bundling for Microsoft
Store, you can define a separate Tauri configuration file:

```
{  "bundle": {    "windows": {      "webviewInstallMode": {        "type": "offlineInstaller"      }    }  }}
```

"src-tauri/tauri.microsoftstore.conf.json

Then merge that config file with the main one when bundling your Tauri
app for Microsoft Store:

- [npm](#tab-panel-4704)
- [yarn](#tab-panel-4705)
- [pnpm](#tab-panel-4706)
- [deno](#tab-panel-4707)
- [bun](#tab-panel-4708)
- [cargo](#tab-panel-4709)

```
npm run tauri build -- --no-bundlenpm run tauri bundle -- --config src-tauri/tauri.microsoftstore.conf.json
```

```
yarn tauri build --no-bundleyarn tauri bundle --config src-tauri/tauri.microsoftstore.conf.json
```

```
pnpm tauri build --no-bundlepnpm tauri bundle --config src-tauri/tauri.microsoftstore.conf.json
```

```
deno task tauri build --no-bundledeno task tauri bundle --config src-tauri/tauri.microsoftstore.conf.json
```

```
bun tauri build --no-bundlebun tauri bundle --config src-tauri/tauri.microsoftstore.conf.json
```

```
cargo tauri build --no-bundlecargo tauri bundle --config src-tauri/tauri.microsoftstore.conf.json
```

This is particularly useful when setting up your CI/CD to upload your
app to the Microsoft Store while having a separate configuration for the
Windows installer you distribute outside the app store.

### Silent install

The Microsoft Store requires Win32 installers to support silent
installation. If your installer does not install silently, your
submission is rejected with an error such as:

```
10.2.9.2 Security - Package Submissions | Win32 products must install silently.
```

When you register your installer in Partner Center you must provide the
[silent install
parameters](https://learn.microsoft.com/en-us/windows/uwp/publish/msiexe/provide-package-details)
so the Store can run it unattended. Tauri’s NSIS `-setup.exe` installer
installs silently with the `/S` flag (note the uppercase `S`):

```
MyApp_x64-setup.exe /S
```

Enter `/S` as the silent install argument in the installer parameters of
your Microsoft Store product. If you distribute the MSI installer
instead, use the standard `msiexec` flag `/quiet`.

### Publisher

Your application [publisher](https://v2.tauri.app/reference/config/#publisher) name cannot
match the application product name.

If the publisher configuration value is not set, Tauri derives it from
the second part of your bundle identifier. Since the publisher name
cannot match the product name, the following configuration is invalid:

```
{  "productName": "Example",  "identifier": "com.example.app"}
```

tauri.conf.json

In this case you can define the
[publisher](https://v2.tauri.app/reference/config/#publisher) value separately to fix this
conflict:

```
{  "productName": "Example",  "identifier": "com.example.app",  "bundle": {    "publisher": "Example Inc."  }}
```

tauri.conf.json

### Upload

After building the Windows installer for Microsoft Store, you can upload
it to the distribution service of your choice and link it in your
application page in the Microsoft Store website.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
