# Develop

Now that you have [everything set up](https://v2.tauri.app/start/), you are ready to run
your application using Tauri.

If you are using a UI framework or JavaScript bundler, you likely have
access to a development server that will speed up your development
process, so if you haven’t configured your app’s dev URL and script that
starts it, you can do so via the [devUrl](https://v2.tauri.app/reference/config/#devurl) and
[beforeDevCommand](https://v2.tauri.app/reference/config/#beforedevcommand) config values:

```
{  "build": {    "devUrl": "http://localhost:3000",    "beforeDevCommand": "npm run dev"  }}
```

tauri.conf.json

Otherwise, if you are not using a UI framework or module bundler, you
can point Tauri to your frontend source code and the Tauri CLI will
start a development server for you:

```
{  "build": {    "frontendDist": "./src"  }}
```

tauri.conf.json

Note that in this example, the `src` folder must include an `index.html`
file along with any other assets loaded by your frontend.

## Developing Your Desktop Application

To develop your application for desktop, run the `tauri dev` command.

- [npm](#tab-panel-4527)
- [yarn](#tab-panel-4528)
- [pnpm](#tab-panel-4529)
- [deno](#tab-panel-4530)
- [bun](#tab-panel-4531)
- [cargo](#tab-panel-4532)

```
npm run tauri dev
```

```
yarn tauri dev
```

```
pnpm tauri dev
```

```
deno task tauri dev
```

```
bun tauri dev
```

```
cargo tauri dev
```

The first time you run this command, the Rust package manager may need
**several minutes** to download and build all the required packages.
Since they are cached, subsequent builds are much faster, as only your
code needs rebuilding.

Once Rust has finished building, the webview opens, displaying your web
app. You can make changes to your web app, and if your tooling supports
it, the webview should update automatically, just like a browser.

### Opening the Web Inspector

You can open the Web Inspector to debug your application by performing a
right-click on the webview and clicking “Inspect” or using the
`Ctrl + Shift + I` shortcut on Windows and Linux or `Cmd + Option + I`
shortcut on macOS.

## Developing Your Mobile Application

Developing for mobile is similar to how desktop development works, but
you must run `tauri android dev` or `tauri ios dev` instead:

- [npm](#tab-panel-4533)
- [yarn](#tab-panel-4534)
- [pnpm](#tab-panel-4535)
- [deno](#tab-panel-4536)
- [bun](#tab-panel-4537)
- [cargo](#tab-panel-4538)

```
npm run tauri [android|ios] dev
```

```
yarn tauri [android|ios] dev
```

```
pnpm tauri [android|ios] dev
```

```
deno task tauri [android|ios] dev
```

```
bun tauri [android|ios] dev
```

```
cargo tauri [android|ios] dev
```

The first time you run this command, the Rust package manager may need
**several minutes** to download and build all the required packages.
Since they are cached, subsequent builds are much faster, as only your
code needs rebuilding.

### Development Server

The development server on mobile works similarly to the desktop one, but
if you are trying to run on a physical iOS device, you must configure it
to listen to a particular address provided by the Tauri CLI, defined in
the `TAURI_DEV_HOST` environment variable. This address is either a
public network address (which is the default behavior) or the actual iOS
device TUN address — which is more secure, but currently needs Xcode to
connect to the device.

To use the iOS device’s address you must open Xcode before running the
dev command and ensure your device is connected via network in the
Window \> Devices and Simulators menu. Then you must run
`tauri ios dev --force-ip-prompt` to select the iOS device address (an
IPv6 address ending with **::2**).

To make your development server listen on the correct host to be
accessible by the iOS device, you must tweak its configuration to use
the `TAURI_DEV_HOST` value if it has been provided. Here is an example
configuration for Vite:

```
import { defineConfig } from 'vite';
const host = process.env.TAURI_DEV_HOST;
// https://vitejs.dev/config/export default defineConfig({  clearScreen: false,  server: {    host: host || false,    port: 1420,    strictPort: true,    hmr: host      ? {          protocol: 'ws',          host,          port: 1421,        }      : undefined,  },});
```

Check your framework’s setup guide for more information.

### Device Selection

By default, the mobile dev command tries to run your application on a
connected device, and falls back to prompting you to select a simulator
to use. To define the run target upfront, you can provide the device or
simulator name as an argument:

- [npm](#tab-panel-4539)
- [yarn](#tab-panel-4540)
- [pnpm](#tab-panel-4541)
- [deno](#tab-panel-4542)
- [bun](#tab-panel-4543)
- [cargo](#tab-panel-4544)

```
npm run tauri ios dev 'iPhone 15'
```

```
yarn tauri ios dev 'iPhone 15'
```

```
pnpm tauri ios dev 'iPhone 15'
```

```
deno task tauri ios dev 'iPhone 15'
```

```
bun tauri ios dev 'iPhone 15'
```

```
cargo tauri ios dev 'iPhone 15'
```

### Using Xcode or Android Studio

Alternatively you can choose to use Xcode or Android Studio to develop
your application. This can help you troubleshoot some development issues
by using the IDE instead of the command line tools. To open the mobile
IDE instead of running on a connected device or simulator, use the
`--open` flag:

- [npm](#tab-panel-4545)
- [yarn](#tab-panel-4546)
- [pnpm](#tab-panel-4547)
- [deno](#tab-panel-4548)
- [bun](#tab-panel-4549)
- [cargo](#tab-panel-4550)

```
npm run tauri [android|ios] dev --open
```

```
yarn tauri [android|ios] dev --open
```

```
pnpm tauri [android|ios] dev --open
```

```
deno task tauri [android|ios] dev --open
```

```
bun tauri [android|ios] dev --open
```

```
cargo tauri [android|ios] dev --open
```

### Opening the Web Inspector

- iOS

  Safari must be used to access the Web Inspector for your iOS
  application.

  Open Safari on your Mac, choose **Safari \> Settings** in the menu
  bar, click **Advanced**, then select **Show features for web
  developers**.

  If you are running on a physical device, you must enable **Web
  Inspector** in **Settings \> Safari \> Advanced**.

  After following all steps you should see a **Develop** menu in Safari,
  where you will find the connected devices and applications to inspect.
  Select your device or simulator and click on **localhost** to open the
  Safari Developer Tools window.

- Android

  The inspector is enabled by default for Android emulators, but you
  must enable it for physical devices. Connect your Android device to
  the computer, open the **Settings** app on the Android device, select
  **About**, scroll to Build Number, and tap it 7 times. This will
  enable Developer Mode for your Android device and the **Developer
  Options** settings.

  To enable application debugging on your device, you must enter the
  **Developer Options** settings, toggle on the developer options switch
  and enable **USB Debugging**.

  The Web Inspector for Android is powered by Google Chrome’s DevTools
  and can be accessed by navigating to `chrome://inspect` in the Chrome
  browser on your computer. Your device or emulator should appear in the
  remote devices list if your Android application is running, and you
  can open the developer tools by clicking **inspect** on the entry
  matching your device.

### Troubleshooting

1.  Error running build script on Xcode

Tauri hooks into the iOS Xcode project by creating a build phase that
executes the Tauri CLI to compile the Rust source as a library that is
loaded at runtime. The build phase is executed on the Xcode process
context, so it might not be able to use shell modifications such as PATH
additions, so be careful when using tools such as Node.js version
managers which may not be compatible.

2.  Network permission prompt on first iOS app execution

When you first execute `tauri ios dev`, you might see iOS prompting you
for permission to find and connect to devices on your local network.
This permission is required because, to access your development server
from an iOS device, it must be exposed on the local network. To run your
app on your device, you must click Allow and restart your application.

## Reacting to Source Code Changes

Similarly to how your webview reflects changes in real time, `tauri dev`
watches your `src-tauri` folder and its dependent crates in the
workspace for changes, so your application is automatically rebuilt and
restarted whenever you modify them.

You can disable this behavior by using the `--no-watch` flag on the
`tauri dev` command.

To ignore watching certain files, you can create `.taurignore` files
which work like regular `.gitignore` files:

```
build/src/generated/*.rsdeny.toml
```

.taurignore

`.taurignore` files are usually put in the `src-tauri` directory or
[cargo
workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html)
root folder. Currently, `tauri dev` looks for `.taurignore` files
anywhere inside the common ancestor of the watched folders and the Cargo
workspace root folder.

## Using the Browser DevTools

Tauri’s APIs only work in your app window, so once you start using them
you won’t be able to open your frontend in your system’s browser
anymore.

If you prefer using your browser’s developer tooling, you must configure
[tauri-invoke-http](https://github.com/tauri-apps/tauri-invoke-http) to
bridge Tauri API calls through a HTTP server.

## Source Control

In your project repository, you **SHOULD** commit the
`src-tauri/Cargo.lock` along with the `src-tauri/Cargo.toml` to git
because Cargo uses the lockfile to provide deterministic builds. As a
result, it is recommended that all applications check in their
`Cargo.lock`. You **SHOULD NOT** commit the `src-tauri/target` folder or
any of its contents.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
