# Command Line Interface

The Tauri command line interface (CLI) is the way to interact with Tauri
throughout the development lifecycle.

You can add the Tauri CLI to your current project using your package
manager of choice:

- [npm](#tab-panel-6289)
- [yarn](#tab-panel-6290)
- [pnpm](#tab-panel-6291)
- [deno](#tab-panel-6292)
- [cargo](#tab-panel-6293)

```
npm install --save-dev @tauri-apps/cli@latest
```

```
yarn add -D @tauri-apps/cli@latest
```

```
pnpm add -D @tauri-apps/cli@latest
```

```
deno add -D npm:@tauri-apps/cli@latest
```

```
cargo install tauri-cli --version "^2.0.0" --locked
```

## List of Commands

| Command | Description |
|----|----|
| [`init`](#init) | Initialize a Tauri project in an existing directory |
| [`dev`](#dev) | Run your app in development mode |
| [`build`](#build) | Build your app in release mode and generate bundles and installers |
| [`bundle`](#bundle) | Generate bundles and installers for your app (already built by `tauri build`) |
| [`android`](#android) | Android commands |
| [`android init`](#android-init) | Initialize Android target in the project |
| [`android dev`](#android-dev) | Run your app in development mode on Android |
| [`android build`](#android-build) | Build your app in release mode for Android and generate APKs and AABs |
| [`android run`](#android-run) | Run your app in production mode on Android |
| [`ios`](#ios) | iOS commands |
| [`ios init`](#ios-init) | Initialize iOS target in the project |
| [`ios dev`](#ios-dev) | Run your app in development mode on iOS |
| [`ios build`](#ios-build) | Build your app in release mode for iOS and generate IPAs |
| [`ios run`](#ios-run) | Run your app in production mode on iOS |
| [`migrate`](#migrate) | Migrate from v1 to v2 |
| [`info`](#info) | Show a concise list of information about the environment, Rust, Node.js and their versions as well as a few relevant project configurations |
| [`add`](#add) | Add a tauri plugin to the project |
| [`remove`](#remove) | Remove a tauri plugin from the project |
| [`plugin`](#plugin) | Manage or create Tauri plugins |
| [`plugin new`](#plugin-new) | Initializes a new Tauri plugin project |
| [`plugin init`](#plugin-init) | Initialize a Tauri plugin project on an existing directory |
| [`plugin android`](#plugin-android) | Manage the Android project for a Tauri plugin |
| [`plugin ios`](#plugin-ios) | Manage the iOS project for a Tauri plugin |
| [`plugin android init`](#plugin-android-init) | Initializes the Android project for an existing Tauri plugin |
| [`plugin ios init`](#plugin-ios-init) | Initializes the iOS project for an existing Tauri plugin |
| [`icon`](#icon) | Generate various icons for all major platforms |
| [`signer`](#signer) | Generate signing keys for Tauri updater or sign files |
| [`signer sign`](#signer-sign) | Sign a file |
| [`signer generate`](#signer-generate) | Generate a new signing key to sign files |
| [`completions`](#completions) | Generate Tauri CLI shell completions for Bash, Zsh, PowerShell or Fish |
| [`permission`](#permission) | Manage or create permissions for your app or plugin |
| [`permission new`](#permission-new) | Create a new permission file |
| [`permission add`](#permission-add) | Add a permission to capabilities |
| [`permission rm`](#permission-rm) | Remove a permission file, and its reference from any capability |
| [`permission ls`](#permission-ls) | List permissions available to your application |
| [`capability`](#capability) | Manage or create capabilities for your app |
| [`capability new`](#capability-new) | Create a new permission file |
| [`inspect`](#inspect) | Inspect values used by Tauri |
| [`inspect wix-upgrade-code`](#inspect-wix-upgrade-code) | Print the default Upgrade Code used by MSI installer derived from productName |

### `init`

- [npm](#tab-panel-6294)
- [yarn](#tab-panel-6295)
- [pnpm](#tab-panel-6296)
- [deno](#tab-panel-6297)
- [bun](#tab-panel-6298)
- [cargo](#tab-panel-6299)

```
npm run tauri init
```

```
yarn tauri init
```

```
pnpm tauri init
```

```
deno task tauri init
```

```
bun tauri init
```

```
cargo tauri init
```

```
Initialize a Tauri project in an existing directory
Usage: tauri init [OPTIONS]
Options:      --ci          Skip prompting for values [env: CI=true]  -v, --verbose...          Enables verbose logging  -f, --force          Force init to overwrite the src-tauri folder  -l, --log          Enables logging  -d, --directory <DIRECTORY>          Set target directory for init [default: /opt/build/repo/packages/cli-generator]  -t, --tauri-path <TAURI_PATH>          Path of the Tauri project to use (relative to the cwd)  -A, --app-name <APP_NAME>          Name of your Tauri application  -W, --window-title <WINDOW_TITLE>          Window title of your Tauri application  -D, --frontend-dist <FRONTEND_DIST>          Web assets location, relative to <project-dir>/src-tauri  -P, --dev-url <DEV_URL>          Url of your dev server      --before-dev-command <BEFORE_DEV_COMMAND>          A shell command to run before `tauri dev` kicks in      --before-build-command <BEFORE_BUILD_COMMAND>          A shell command to run before `tauri build` kicks in  -h, --help          Print help  -V, --version          Print version
```

### `dev`

- [npm](#tab-panel-6300)
- [yarn](#tab-panel-6301)
- [pnpm](#tab-panel-6302)
- [deno](#tab-panel-6303)
- [bun](#tab-panel-6304)
- [cargo](#tab-panel-6305)

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

```
Run your app in development mode with hot-reloading for the Rust code. It makes use of the `build.devUrl` property from your `tauri.conf.json` file. It also runs your `build.beforeDevCommand` which usually starts your frontend devServer.
Usage: tauri dev [OPTIONS] [ARGS]...
Arguments:  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. Arguments after a second `--` are passed to the application e.g. `tauri dev -- [runnerArgs] -- [appArgs]`
Options:  -r, --runner <RUNNER>          Binary to use to run the application
  -v, --verbose...          Enables verbose logging
  -t, --target <TARGET>          Target triple to build against
  -f, --features [<FEATURES>...]          List of cargo features to activate
  -e, --exit-on-panic          Exit on panic
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --release          Run the code in release mode
      --no-dev-server-wait          Skip waiting for the frontend dev server to start before building the tauri application
          [env: TAURI_CLI_NO_DEV_SERVER_WAIT=]
      --no-watch          Disable the file watcher
      --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>          Additional paths to watch for changes
      --no-dev-server          Disable the built-in dev server for static files
      --port <PORT>          Specify port for the built-in dev server for static files. Defaults to 1430
          [env: TAURI_CLI_PORT=]
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

### `build`

- [npm](#tab-panel-6306)
- [yarn](#tab-panel-6307)
- [pnpm](#tab-panel-6308)
- [deno](#tab-panel-6309)
- [bun](#tab-panel-6310)
- [cargo](#tab-panel-6311)

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

```
Build your app in release mode and generate bundles and installers. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`. This will also run `build.beforeBundleCommand` before generating the bundles and installers of your app.
Usage: tauri build [OPTIONS] [ARGS]...
Arguments:  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments
Options:  -r, --runner <RUNNER>          Binary to use to build the application, defaults to `cargo`
  -v, --verbose...          Enables verbose logging
  -d, --debug          Builds with the debug flag
  -t, --target <TARGET>          Target triple to build against.
          It must be one of the values outputted by `$rustc --print target-list` or `universal-apple-darwin` for an universal macOS application.
          Note that compiling an universal macOS application requires both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets to be installed.
  -f, --features [<FEATURES>...]          Space or comma separated list of features to activate
  -b, --bundles [<BUNDLES>...]          Space or comma separated list of bundles to package
          [possible values: deb, rpm, appimage]
      --no-bundle          Skip the bundling step even if `bundle > active` is `true` in tauri config
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --ci          Skip prompting for values
          [env: CI=true]
      --skip-stapling          Whether to wait for notarization to finish and `staple` the ticket onto the app.
          Gatekeeper will look for stapled tickets to tell whether your app was notarized without reaching out to Apple's servers which is helpful in offline environments.
          Enabling this option will also result in `tauri build` not waiting for notarization to finish which is helpful for the very first time your app is notarized as this can take multiple hours. On subsequent runs, it's recommended to disable this setting again.
      --ignore-version-mismatches          Do not error out if a version mismatch is detected on a Tauri package.
          Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.
      --no-sign          Skip code signing when bundling the app
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

### `bundle`

- [npm](#tab-panel-6312)
- [yarn](#tab-panel-6313)
- [pnpm](#tab-panel-6314)
- [deno](#tab-panel-6315)
- [bun](#tab-panel-6316)
- [cargo](#tab-panel-6317)

```
npm run tauri bundle
```

```
yarn tauri bundle
```

```
pnpm tauri bundle
```

```
deno task tauri bundle
```

```
bun tauri bundle
```

```
cargo tauri bundle
```

```
Generate bundles and installers for your app (already built by `tauri build`). This run `build.beforeBundleCommand` before generating the bundles and installers of your app.
Usage: tauri bundle [OPTIONS]
Options:  -d, --debug          Builds with the debug flag
  -v, --verbose...          Enables verbose logging
  -b, --bundles [<BUNDLES>...]          Space or comma separated list of bundles to package
          [possible values: deb, rpm, appimage]
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
  -f, --features [<FEATURES>...]          Space or comma separated list of features, should be the same features passed to `tauri build` if any
  -t, --target <TARGET>          Target triple to build against.
          It must be one of the values outputted by `$rustc --print target-list` or `universal-apple-darwin` for an universal macOS application.
          Note that compiling an universal macOS application requires both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets to be installed.
      --ci          Skip prompting for values
          [env: CI=true]
      --skip-stapling          Whether to wait for notarization to finish and `staple` the ticket onto the app.
          Gatekeeper will look for stapled tickets to tell whether your app was notarized without reaching out to Apple's servers which is helpful in offline environments.
          Enabling this option will also result in `tauri build` not waiting for notarization to finish which is helpful for the very first time your app is notarized as this can take multiple hours. On subsequent runs, it's recommended to disable this setting again.
      --no-sign          Skip code signing during the build or bundling process.
          Useful for local development and CI environments where signing certificates or environment variables are not available or not needed.
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

### `android`

- [npm](#tab-panel-6318)
- [yarn](#tab-panel-6319)
- [pnpm](#tab-panel-6320)
- [deno](#tab-panel-6321)
- [bun](#tab-panel-6322)
- [cargo](#tab-panel-6323)

```
npm run tauri android
```

```
yarn tauri android
```

```
pnpm tauri android
```

```
deno task tauri android
```

```
bun tauri android
```

```
cargo tauri android
```

```
Android commands
Usage: tauri android [OPTIONS] <COMMAND>
Commands:  init   Initialize Android target in the project  dev    Run your app in development mode on Android  build  Build your app in release mode for Android and generate APKs and AABs  run    Run your app in production mode on Android  help   Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `android init`

- [npm](#tab-panel-6324)
- [yarn](#tab-panel-6325)
- [pnpm](#tab-panel-6326)
- [deno](#tab-panel-6327)
- [bun](#tab-panel-6328)
- [cargo](#tab-panel-6329)

```
npm run tauri android init
```

```
yarn tauri android init
```

```
pnpm tauri android init
```

```
deno task tauri android init
```

```
bun tauri android init
```

```
cargo tauri android init
```

```
Initialize Android target in the project
Usage: tauri android init [OPTIONS]
Options:      --ci          Skip prompting for values
          [env: CI=true]
  -v, --verbose...          Enables verbose logging
      --skip-targets-install          Skips installing rust toolchains via rustup
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `android dev`

- [npm](#tab-panel-6330)
- [yarn](#tab-panel-6331)
- [pnpm](#tab-panel-6332)
- [deno](#tab-panel-6333)
- [bun](#tab-panel-6334)
- [cargo](#tab-panel-6335)

```
npm run tauri android dev
```

```
yarn tauri android dev
```

```
pnpm tauri android dev
```

```
deno task tauri android dev
```

```
bun tauri android dev
```

```
cargo tauri android dev
```

```
Run your app in development mode on Android with hot-reloading for the Rust code. It makes use of the `build.devUrl` property from your `tauri.conf.json` file. It also runs your `build.beforeDevCommand` which usually starts your frontend devServer.
Usage: tauri android dev [OPTIONS] [DEVICE] [-- <ARGS>...]
Arguments:  [DEVICE]          Runs on the given device name
  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android dev -- [runnerArgs]`
Options:  -f, --features [<FEATURES>...]          List of cargo features to activate
  -v, --verbose...          Enables verbose logging
  -e, --exit-on-panic          Exit on panic
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --release          Run the code in release mode
      --no-dev-server-wait          Skip waiting for the frontend dev server to start before building the tauri application
          [env: TAURI_CLI_NO_DEV_SERVER_WAIT=]
      --no-watch          Disable the file watcher
      --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>          Additional paths to watch for changes
  -o, --open          Open Android Studio instead of trying to run on a connected device
      --force-ip-prompt          Force prompting for an IP to use to connect to the dev server on mobile
      --host [<HOST>]          Use the public network address for the development server. If an actual address it provided, it is used instead of prompting to pick one.
          On Windows we use the public network address by default.
          This option is particularly useful along the `--open` flag when you intend on running on a physical device.
          This replaces the devUrl configuration value to match the public network address host, it is your responsibility to set up your development server to listen on this address by using 0.0.0.0 as host for instance.
          When this is set or when running on an iOS device the CLI sets the `TAURI_DEV_HOST` environment variable so you can check this on your framework's configuration to expose the development server on the public network address.
          [default: <none>]
      --no-dev-server          Disable the built-in dev server for static files
      --port <PORT>          Specify port for the built-in dev server for static files. Defaults to 1430
          [env: TAURI_CLI_PORT=]
      --root-certificate-path <ROOT_CERTIFICATE_PATH>          Path to the certificate file used by your dev server. Required for mobile dev when using HTTPS
          [env: TAURI_DEV_ROOT_CERTIFICATE_PATH=]
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `android build`

- [npm](#tab-panel-6336)
- [yarn](#tab-panel-6337)
- [pnpm](#tab-panel-6338)
- [deno](#tab-panel-6339)
- [bun](#tab-panel-6340)
- [cargo](#tab-panel-6341)

```
npm run tauri android build
```

```
yarn tauri android build
```

```
pnpm tauri android build
```

```
deno task tauri android build
```

```
bun tauri android build
```

```
cargo tauri android build
```

```
Build your app in release mode for Android and generate APKs and AABs. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.
Usage: tauri android build [OPTIONS] [-- <ARGS>...]
Arguments:  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android build -- [runnerArgs]`
Options:  -d, --debug          Builds with the debug flag
  -v, --verbose...          Enables verbose logging
  -t, --target [<TARGETS>...]          Which targets to build (all by default)
          [possible values: aarch64, armv7, i686, x86_64]
  -f, --features [<FEATURES>...]          List of cargo features to activate
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --split-per-abi          Whether to split the APKs and AABs per ABIs
      --apk          Build APKs
      --aab          Build AABs
  -o, --open          Open Android Studio
      --ci          Skip prompting for values
          [env: CI=true]
      --ignore-version-mismatches          Do not error out if a version mismatch is detected on a Tauri package.
          Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `android run`

- [npm](#tab-panel-6342)
- [yarn](#tab-panel-6343)
- [pnpm](#tab-panel-6344)
- [deno](#tab-panel-6345)
- [bun](#tab-panel-6346)
- [cargo](#tab-panel-6347)

```
npm run tauri android run
```

```
yarn tauri android run
```

```
pnpm tauri android run
```

```
deno task tauri android run
```

```
bun tauri android run
```

```
cargo tauri android run
```

```
Run your app in production mode on Android. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.
Usage: tauri android run [OPTIONS] [DEVICE] [-- <ARGS>...]
Arguments:  [DEVICE]          Runs on the given device name
  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android build -- [runnerArgs]`
Options:  -r, --release          Run the app in release mode
  -v, --verbose...          Enables verbose logging
  -f, --features [<FEATURES>...]          List of cargo features to activate
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --no-watch          Disable the file watcher
      --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>          Additional paths to watch for changes
  -o, --open          Open Android Studio
      --ignore-version-mismatches          Do not error out if a version mismatch is detected on a Tauri package.
          Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

### `ios`

*All iOS commands are only available on macOS hosts.*

- [npm](#tab-panel-6348)
- [yarn](#tab-panel-6349)
- [pnpm](#tab-panel-6350)
- [deno](#tab-panel-6351)
- [bun](#tab-panel-6352)
- [cargo](#tab-panel-6353)

```
npm run tauri ios
```

```
yarn tauri ios
```

```
pnpm tauri ios
```

```
deno task tauri ios
```

```
bun tauri ios
```

```
cargo tauri ios
```

```
iOS commands
Usage: tauri ios [OPTIONS] <COMMAND>
Commands:  init   Initialize iOS target in the project  dev    Run your app in development mode on iOS  build  Build your app in release mode for iOS and generate IPAs  run    Run your app in production mode on iOS  help   Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `ios init`

*All iOS commands are only available on macOS hosts.*

- [npm](#tab-panel-6354)
- [yarn](#tab-panel-6355)
- [pnpm](#tab-panel-6356)
- [deno](#tab-panel-6357)
- [bun](#tab-panel-6358)
- [cargo](#tab-panel-6359)

```
npm run tauri ios init
```

```
yarn tauri ios init
```

```
pnpm tauri ios init
```

```
deno task tauri ios init
```

```
bun tauri ios init
```

```
cargo tauri ios init
```

```
Initialize iOS target in the project
Usage: tauri ios init [OPTIONS]
Options:      --ci          Skip prompting for values
          [env: CI=]
  -v, --verbose...          Enables verbose logging
  -r, --reinstall-deps          Reinstall dependencies
      --skip-targets-install          Skips installing rust toolchains via rustup
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `ios dev`

*All iOS commands are only available on macOS hosts.*

- [npm](#tab-panel-6360)
- [yarn](#tab-panel-6361)
- [pnpm](#tab-panel-6362)
- [deno](#tab-panel-6363)
- [bun](#tab-panel-6364)
- [cargo](#tab-panel-6365)

```
npm run tauri ios dev
```

```
yarn tauri ios dev
```

```
pnpm tauri ios dev
```

```
deno task tauri ios dev
```

```
bun tauri ios dev
```

```
cargo tauri ios dev
```

```
Run your app in development mode on iOS with hot-reloading for the Rust code.It makes use of the `build.devUrl` property from your `tauri.conf.json` file.It also runs your `build.beforeDevCommand` which usually starts your frontend devServer.
When connected to a physical iOS device, the public network address must be used instead of `localhost`for the devUrl property. Tauri makes that change automatically, but your dev server might needa different configuration to listen on the public address. You can check the `TAURI_DEV_HOST`environment variable to determine whether the public network should be used or not.
Usage: tauri ios dev [OPTIONS] [DEVICE] [-- <ARGS>...]
Arguments:  [DEVICE]          Runs on the given device name
  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri ios dev -- [runnerArgs]`
Options:  -f, --features [<FEATURES>...]          List of cargo features to activate
  -v, --verbose...          Enables verbose logging
  -e, --exit-on-panic          Exit on panic
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --release          Run the code in release mode
      --no-dev-server-wait          Skip waiting for the frontend dev server to start before building the tauri application
          [env: TAURI_CLI_NO_DEV_SERVER_WAIT=]
      --no-watch          Disable the file watcher
      --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>          Additional paths to watch for changes
  -o, --open          Open Xcode instead of trying to run on a connected device
      --force-ip-prompt          Force prompting for an IP to use to connect to the dev server on mobile
      --host [<HOST>]          Use the public network address for the development server. If an actual address it provided, it is used instead of prompting to pick one.
          This option is particularly useful along the `--open` flag when you intend on running on a physical device.
          This replaces the devUrl configuration value to match the public network address host, it is your responsibility to set up your development server to listen on this address by using 0.0.0.0 as host for instance.
          When this is set or when running on an iOS device the CLI sets the `TAURI_DEV_HOST` environment variable so you can check this on your framework's configuration to expose the development server on the public network address.
          [default: <none>]
      --no-dev-server          Disable the built-in dev server for static files
      --port <PORT>          Specify port for the built-in dev server for static files. Defaults to 1430
          [env: TAURI_CLI_PORT=]
      --root-certificate-path <ROOT_CERTIFICATE_PATH>          Path to the certificate file used by your dev server. Required for mobile dev when using HTTPS
          [env: TAURI_DEV_ROOT_CERTIFICATE_PATH=]
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `ios build`

*All iOS commands are only available on macOS hosts.*

- [npm](#tab-panel-6366)
- [yarn](#tab-panel-6367)
- [pnpm](#tab-panel-6368)
- [deno](#tab-panel-6369)
- [bun](#tab-panel-6370)
- [cargo](#tab-panel-6371)

```
npm run tauri ios build
```

```
yarn tauri ios build
```

```
pnpm tauri ios build
```

```
deno task tauri ios build
```

```
bun tauri ios build
```

```
cargo tauri ios build
```

```
Build your app in release mode for iOS and generate IPAs. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.
Usage: tauri ios build [OPTIONS] [-- <ARGS>...]
Arguments:  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri ios build -- [runnerArgs]`
Options:  -d, --debug          Builds with the debug flag
  -v, --verbose...          Enables verbose logging
  -t, --target [<TARGETS>...]          Which targets to build
          [default: aarch64]          [possible values: aarch64, aarch64-sim, x86_64]
  -f, --features [<FEATURES>...]          List of cargo features to activate
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --build-number <BUILD_NUMBER>          Build number to append to the app version
  -o, --open          Open Xcode
      --ci          Skip prompting for values
          [env: CI=]
      --export-method <EXPORT_METHOD>          Describes how Xcode should export the archive.
          Use this to create a package ready for the App Store (app-store-connect option) or TestFlight (release-testing option).
          [possible values: app-store-connect, release-testing, debugging]
      --ignore-version-mismatches          Do not error out if a version mismatch is detected on a Tauri package.
          Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `ios run`

*All iOS commands are only available on macOS hosts.*

- [npm](#tab-panel-6372)
- [yarn](#tab-panel-6373)
- [pnpm](#tab-panel-6374)
- [deno](#tab-panel-6375)
- [bun](#tab-panel-6376)
- [cargo](#tab-panel-6377)

```
npm run tauri ios run
```

```
yarn tauri ios run
```

```
pnpm tauri ios run
```

```
deno task tauri ios run
```

```
bun tauri ios run
```

```
cargo tauri ios run
```

```
Run your app in production mode on iOS. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.
Usage: tauri ios run [OPTIONS] [DEVICE] [-- <ARGS>...]
Arguments:  [DEVICE]          Runs on the given device name
  [ARGS]...          Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android build -- [runnerArgs]`
Options:  -r, --release          Run the app in release mode
  -v, --verbose...          Enables verbose logging
  -f, --features [<FEATURES>...]          List of cargo features to activate
  -c, --config <CONFIG>          JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file
          Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.
          Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.
      --no-watch          Disable the file watcher
      --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>          Additional paths to watch for changes
  -o, --open          Open Xcode
      --ignore-version-mismatches          Do not error out if a version mismatch is detected on a Tauri package.
          Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

### `migrate`

- [npm](#tab-panel-6378)
- [yarn](#tab-panel-6379)
- [pnpm](#tab-panel-6380)
- [deno](#tab-panel-6381)
- [bun](#tab-panel-6382)
- [cargo](#tab-panel-6383)

```
npm run tauri migrate
```

```
yarn tauri migrate
```

```
pnpm tauri migrate
```

```
deno task tauri migrate
```

```
bun tauri migrate
```

```
cargo tauri migrate
```

```
Migrate from v1 to v2
Usage: tauri migrate [OPTIONS]
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

### `info`

- [npm](#tab-panel-6384)
- [yarn](#tab-panel-6385)
- [pnpm](#tab-panel-6386)
- [deno](#tab-panel-6387)
- [bun](#tab-panel-6388)
- [cargo](#tab-panel-6389)

```
npm run tauri info
```

```
yarn tauri info
```

```
pnpm tauri info
```

```
deno task tauri info
```

```
bun tauri info
```

```
cargo tauri info
```

```
Show a concise list of information about the environment, Rust, Node.js and their versions as well as a few relevant project configurations
Usage: tauri info [OPTIONS]
Options:      --interactive  Interactive mode to apply automatic fixes  -v, --verbose...   Enables verbose logging  -h, --help         Print help  -V, --version      Print version
```

### `add`

- [npm](#tab-panel-6390)
- [yarn](#tab-panel-6391)
- [pnpm](#tab-panel-6392)
- [deno](#tab-panel-6393)
- [bun](#tab-panel-6394)
- [cargo](#tab-panel-6395)

```
npm run tauri add
```

```
yarn tauri add
```

```
pnpm tauri add
```

```
deno task tauri add
```

```
bun tauri add
```

```
cargo tauri add
```

```
Add a tauri plugin to the project
Usage: tauri add [OPTIONS] <PLUGIN>
Arguments:  <PLUGIN>  The plugin to add
Options:  -t, --tag <TAG>        Git tag to use  -v, --verbose...       Enables verbose logging  -r, --rev <REV>        Git rev to use  -b, --branch <BRANCH>  Git branch to use      --no-fmt           Don't format code with rustfmt  -h, --help             Print help  -V, --version          Print version
```

### `remove`

- [npm](#tab-panel-6396)
- [yarn](#tab-panel-6397)
- [pnpm](#tab-panel-6398)
- [deno](#tab-panel-6399)
- [bun](#tab-panel-6400)
- [cargo](#tab-panel-6401)

```
npm run tauri remove
```

```
yarn tauri remove
```

```
pnpm tauri remove
```

```
deno task tauri remove
```

```
bun tauri remove
```

```
cargo tauri remove
```

```
Remove a tauri plugin from the project
Usage: tauri remove [OPTIONS] <PLUGIN>
Arguments:  <PLUGIN>  The plugin to remove
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

### `plugin`

- [npm](#tab-panel-6402)
- [yarn](#tab-panel-6403)
- [pnpm](#tab-panel-6404)
- [deno](#tab-panel-6405)
- [bun](#tab-panel-6406)
- [cargo](#tab-panel-6407)

```
npm run tauri plugin
```

```
yarn tauri plugin
```

```
pnpm tauri plugin
```

```
deno task tauri plugin
```

```
bun tauri plugin
```

```
cargo tauri plugin
```

```
Manage or create Tauri plugins
Usage: tauri plugin [OPTIONS] <COMMAND>
Commands:  new      Initializes a new Tauri plugin project  init     Initialize a Tauri plugin project on an existing directory  android  Manage the Android project for a Tauri plugin  ios      Manage the iOS project for a Tauri plugin  help     Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `plugin new`

- [npm](#tab-panel-6408)
- [yarn](#tab-panel-6409)
- [pnpm](#tab-panel-6410)
- [deno](#tab-panel-6411)
- [bun](#tab-panel-6412)
- [cargo](#tab-panel-6413)

```
npm run tauri plugin new
```

```
yarn tauri plugin new
```

```
pnpm tauri plugin new
```

```
deno task tauri plugin new
```

```
bun tauri plugin new
```

```
cargo tauri plugin new
```

```
Initializes a new Tauri plugin project
Usage: tauri plugin new [OPTIONS] <PLUGIN_NAME>
Arguments:  <PLUGIN_NAME>          Name of your Tauri plugin
Options:      --no-api          Initializes a Tauri plugin without the TypeScript API
  -v, --verbose...          Enables verbose logging
      --no-example          Initialize without an example project
  -d, --directory <DIRECTORY>          Set target directory for init
  -a, --author <AUTHOR>          Author name
      --android          Whether to initialize an Android project for the plugin
      --ios          Whether to initialize an iOS project for the plugin
      --mobile          Whether to initialize Android and iOS projects for the plugin
      --ios-framework <IOS_FRAMEWORK>          Type of framework to use for the iOS project
          [default: spm]
          Possible values:          - spm:   Swift Package Manager project          - xcode: Xcode project
      --github-workflows          Generate github workflows
  -t, --tauri-path <TAURI_PATH>          Path of the Tauri project to use (relative to the cwd)
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `plugin init`

- [npm](#tab-panel-6414)
- [yarn](#tab-panel-6415)
- [pnpm](#tab-panel-6416)
- [deno](#tab-panel-6417)
- [bun](#tab-panel-6418)
- [cargo](#tab-panel-6419)

```
npm run tauri plugin init
```

```
yarn tauri plugin init
```

```
pnpm tauri plugin init
```

```
deno task tauri plugin init
```

```
bun tauri plugin init
```

```
cargo tauri plugin init
```

```
Initialize a Tauri plugin project on an existing directory
Usage: tauri plugin init [OPTIONS] [PLUGIN_NAME]
Arguments:  [PLUGIN_NAME]          Name of your Tauri plugin. If not specified, it will be inferred from the current directory
Options:      --no-api          Initializes a Tauri plugin without the TypeScript API
  -v, --verbose...          Enables verbose logging
      --no-example          Initialize without an example project
  -d, --directory <DIRECTORY>          Set target directory for init
          [default: /opt/build/repo/packages/cli-generator]
  -a, --author <AUTHOR>          Author name
      --android          Whether to initialize an Android project for the plugin
      --ios          Whether to initialize an iOS project for the plugin
      --mobile          Whether to initialize Android and iOS projects for the plugin
      --ios-framework <IOS_FRAMEWORK>          Type of framework to use for the iOS project
          [default: spm]
          Possible values:          - spm:   Swift Package Manager project          - xcode: Xcode project
      --github-workflows          Generate github workflows
  -t, --tauri-path <TAURI_PATH>          Path of the Tauri project to use (relative to the cwd)
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `plugin android`

- [npm](#tab-panel-6420)
- [yarn](#tab-panel-6421)
- [pnpm](#tab-panel-6422)
- [deno](#tab-panel-6423)
- [bun](#tab-panel-6424)
- [cargo](#tab-panel-6425)

```
npm run tauri plugin android
```

```
yarn tauri plugin android
```

```
pnpm tauri plugin android
```

```
deno task tauri plugin android
```

```
bun tauri plugin android
```

```
cargo tauri plugin android
```

```
Manage the Android project for a Tauri plugin
Usage: tauri plugin android [OPTIONS] <COMMAND>
Commands:  init  Initializes the Android project for an existing Tauri plugin  help  Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

##### `plugin android init`

- [npm](#tab-panel-6426)
- [yarn](#tab-panel-6427)
- [pnpm](#tab-panel-6428)
- [deno](#tab-panel-6429)
- [bun](#tab-panel-6430)
- [cargo](#tab-panel-6431)

```
npm run tauri plugin android init
```

```
yarn tauri plugin android init
```

```
pnpm tauri plugin android init
```

```
deno task tauri plugin android init
```

```
bun tauri plugin android init
```

```
cargo tauri plugin android init
```

```
Initializes the Android project for an existing Tauri plugin
Usage: tauri plugin android init [OPTIONS] [PLUGIN_NAME]
Arguments:  [PLUGIN_NAME]  Name of your Tauri plugin. Must match the current plugin's name. If not specified, it will be inferred from the current directory
Options:  -o, --out-dir <OUT_DIR>  The output directory [default: /opt/build/repo/packages/cli-generator]  -v, --verbose...         Enables verbose logging  -h, --help               Print help  -V, --version            Print version
```

#### `plugin ios`

- [npm](#tab-panel-6432)
- [yarn](#tab-panel-6433)
- [pnpm](#tab-panel-6434)
- [deno](#tab-panel-6435)
- [bun](#tab-panel-6436)
- [cargo](#tab-panel-6437)

```
npm run tauri plugin ios
```

```
yarn tauri plugin ios
```

```
pnpm tauri plugin ios
```

```
deno task tauri plugin ios
```

```
bun tauri plugin ios
```

```
cargo tauri plugin ios
```

```
Manage the iOS project for a Tauri plugin
Usage: tauri plugin ios [OPTIONS] <COMMAND>
Commands:  init  Initializes the iOS project for an existing Tauri plugin  help  Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

##### `plugin ios init`

- [npm](#tab-panel-6438)
- [yarn](#tab-panel-6439)
- [pnpm](#tab-panel-6440)
- [deno](#tab-panel-6441)
- [bun](#tab-panel-6442)
- [cargo](#tab-panel-6443)

```
npm run tauri plugin ios init
```

```
yarn tauri plugin ios init
```

```
pnpm tauri plugin ios init
```

```
deno task tauri plugin ios init
```

```
bun tauri plugin ios init
```

```
cargo tauri plugin ios init
```

```
Initializes the iOS project for an existing Tauri plugin
Usage: tauri plugin ios init [OPTIONS] [PLUGIN_NAME]
Arguments:  [PLUGIN_NAME]          Name of your Tauri plugin. Must match the current plugin's name. If not specified, it will be inferred from the current directory
Options:  -o, --out-dir <OUT_DIR>          The output directory
          [default: /opt/build/repo/packages/cli-generator]
  -v, --verbose...          Enables verbose logging
      --ios-framework <IOS_FRAMEWORK>          Type of framework to use for the iOS project
          [default: spm]
          Possible values:          - spm:   Swift Package Manager project          - xcode: Xcode project
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

### `icon`

- [npm](#tab-panel-6444)
- [yarn](#tab-panel-6445)
- [pnpm](#tab-panel-6446)
- [deno](#tab-panel-6447)
- [bun](#tab-panel-6448)
- [cargo](#tab-panel-6449)

```
npm run tauri icon
```

```
yarn tauri icon
```

```
pnpm tauri icon
```

```
deno task tauri icon
```

```
bun tauri icon
```

```
cargo tauri icon
```

```
Generate various icons for all major platforms
Usage: tauri icon [OPTIONS] [INPUT]
Arguments:  [INPUT]          Path to the source icon (squared PNG or SVG file with transparency) or a manifest file.
          The manifest file is a JSON file with the following structure: { "default": "app-icon.png", "bg_color": "#fff", "android_bg": "app-icon-bg.png", "android_fg": "app-icon-fg.png", "android_fg_scale": 85, "android_monochrome": "app-icon-monochrome.png" }
          All file paths defined in the manifest JSON are relative to the manifest file path.
          Only the `default` manifest property is required.
          The `bg_color` manifest value overwrites the `--ios-color` option if set.
          [default: ./app-icon.png]
Options:  -o, --output <OUTPUT>          Output directory. Default: 'icons' directory next to the tauri.conf.json file
  -v, --verbose...          Enables verbose logging
  -p, --png <PNG>          Custom PNG icon sizes to generate. When set, the default icons are not generated
      --ios-color <IOS_COLOR>          The background color of the iOS icon - string as defined in the W3C's CSS Color Module Level 4 <https://www.w3.org/TR/css-color-4/>
          [default: #fff]
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

### `signer`

- [npm](#tab-panel-6450)
- [yarn](#tab-panel-6451)
- [pnpm](#tab-panel-6452)
- [deno](#tab-panel-6453)
- [bun](#tab-panel-6454)
- [cargo](#tab-panel-6455)

```
npm run tauri signer
```

```
yarn tauri signer
```

```
pnpm tauri signer
```

```
deno task tauri signer
```

```
bun tauri signer
```

```
cargo tauri signer
```

```
Generate signing keys for Tauri updater or sign files
Usage: tauri signer [OPTIONS] <COMMAND>
Commands:  sign      Sign a file  generate  Generate a new signing key to sign files  help      Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `signer sign`

- [npm](#tab-panel-6456)
- [yarn](#tab-panel-6457)
- [pnpm](#tab-panel-6458)
- [deno](#tab-panel-6459)
- [bun](#tab-panel-6460)
- [cargo](#tab-panel-6461)

```
npm run tauri signer sign
```

```
yarn tauri signer sign
```

```
pnpm tauri signer sign
```

```
deno task tauri signer sign
```

```
bun tauri signer sign
```

```
cargo tauri signer sign
```

```
Sign a file
Usage: tauri signer sign [OPTIONS] <FILE>
Arguments:  <FILE>  Sign the specified file
Options:  -k, --private-key <PRIVATE_KEY>          Load the private key from a string [env: TAURI_SIGNING_PRIVATE_KEY=]  -v, --verbose...          Enables verbose logging  -f, --private-key-path <PRIVATE_KEY_PATH>          Load the private key from a file [env: TAURI_SIGNING_PRIVATE_KEY_PATH=]  -p, --password <PASSWORD>          Set private key password when signing [env: TAURI_SIGNING_PRIVATE_KEY_PASSWORD=]  -h, --help          Print help  -V, --version          Print version
```

#### `signer generate`

- [npm](#tab-panel-6462)
- [yarn](#tab-panel-6463)
- [pnpm](#tab-panel-6464)
- [deno](#tab-panel-6465)
- [bun](#tab-panel-6466)
- [cargo](#tab-panel-6467)

```
npm run tauri signer generate
```

```
yarn tauri signer generate
```

```
pnpm tauri signer generate
```

```
deno task tauri signer generate
```

```
bun tauri signer generate
```

```
cargo tauri signer generate
```

```
Generate a new signing key to sign files
Usage: tauri signer generate [OPTIONS]
Options:  -p, --password <PASSWORD>      Set private key password when signing  -v, --verbose...               Enables verbose logging  -w, --write-keys <WRITE_KEYS>  Write private key to a file  -f, --force                    Overwrite private key even if it exists on the specified path      --ci                       Skip prompting for values [env: CI=true]  -h, --help                     Print help  -V, --version                  Print version
```

### `completions`

- [npm](#tab-panel-6468)
- [yarn](#tab-panel-6469)
- [pnpm](#tab-panel-6470)
- [deno](#tab-panel-6471)
- [bun](#tab-panel-6472)
- [cargo](#tab-panel-6473)

```
npm run tauri completions
```

```
yarn tauri completions
```

```
pnpm tauri completions
```

```
deno task tauri completions
```

```
bun tauri completions
```

```
cargo tauri completions
```

```
Generate Tauri CLI shell completions for Bash, Zsh, PowerShell or Fish
Usage: tauri completions [OPTIONS] --shell <SHELL>
Options:  -s, --shell <SHELL>    Shell to generate a completion script for. [possible values: bash, elvish, fish, powershell, zsh]  -v, --verbose...       Enables verbose logging  -o, --output <OUTPUT>  Output file for the shell completions. By default the completions are printed to stdout  -h, --help             Print help  -V, --version          Print version
```

### `permission`

- [npm](#tab-panel-6474)
- [yarn](#tab-panel-6475)
- [pnpm](#tab-panel-6476)
- [deno](#tab-panel-6477)
- [bun](#tab-panel-6478)
- [cargo](#tab-panel-6479)

```
npm run tauri permission
```

```
yarn tauri permission
```

```
pnpm tauri permission
```

```
deno task tauri permission
```

```
bun tauri permission
```

```
cargo tauri permission
```

```
Manage or create permissions for your app or plugin
Usage: tauri permission [OPTIONS] <COMMAND>
Commands:  new   Create a new permission file  add   Add a permission to capabilities  rm    Remove a permission file, and its reference from any capability  ls    List permissions available to your application  help  Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `permission new`

- [npm](#tab-panel-6480)
- [yarn](#tab-panel-6481)
- [pnpm](#tab-panel-6482)
- [deno](#tab-panel-6483)
- [bun](#tab-panel-6484)
- [cargo](#tab-panel-6485)

```
npm run tauri permission new
```

```
yarn tauri permission new
```

```
pnpm tauri permission new
```

```
deno task tauri permission new
```

```
bun tauri permission new
```

```
cargo tauri permission new
```

```
Create a new permission file
Usage: tauri permission new [OPTIONS] [IDENTIFIER]
Arguments:  [IDENTIFIER]  Permission identifier
Options:      --description <DESCRIPTION>  Permission description  -v, --verbose...                 Enables verbose logging  -a, --allow <ALLOW>              List of commands to allow  -d, --deny <DENY>                List of commands to deny      --format <FORMAT>            Output file format [default: json] [possible values: json, toml]  -o, --out <OUT>                  The output file  -h, --help                       Print help  -V, --version                    Print version
```

#### `permission add`

- [npm](#tab-panel-6486)
- [yarn](#tab-panel-6487)
- [pnpm](#tab-panel-6488)
- [deno](#tab-panel-6489)
- [bun](#tab-panel-6490)
- [cargo](#tab-panel-6491)

```
npm run tauri permission add
```

```
yarn tauri permission add
```

```
pnpm tauri permission add
```

```
deno task tauri permission add
```

```
bun tauri permission add
```

```
cargo tauri permission add
```

```
Add a permission to capabilities
Usage: tauri permission add [OPTIONS] <IDENTIFIER> [CAPABILITY]
Arguments:  <IDENTIFIER>  Permission to add  [CAPABILITY]  Capability to add the permission to
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `permission rm`

- [npm](#tab-panel-6492)
- [yarn](#tab-panel-6493)
- [pnpm](#tab-panel-6494)
- [deno](#tab-panel-6495)
- [bun](#tab-panel-6496)
- [cargo](#tab-panel-6497)

```
npm run tauri permission rm
```

```
yarn tauri permission rm
```

```
pnpm tauri permission rm
```

```
deno task tauri permission rm
```

```
bun tauri permission rm
```

```
cargo tauri permission rm
```

```
Remove a permission file, and its reference from any capability
Usage: tauri permission rm [OPTIONS] <IDENTIFIER>
Arguments:  <IDENTIFIER>          Permission to remove.
          To remove all permissions for a given plugin, provide `<plugin-name>:*`
Options:  -v, --verbose...          Enables verbose logging
  -h, --help          Print help (see a summary with '-h')
  -V, --version          Print version
```

#### `permission ls`

- [npm](#tab-panel-6498)
- [yarn](#tab-panel-6499)
- [pnpm](#tab-panel-6500)
- [deno](#tab-panel-6501)
- [bun](#tab-panel-6502)
- [cargo](#tab-panel-6503)

```
npm run tauri permission ls
```

```
yarn tauri permission ls
```

```
pnpm tauri permission ls
```

```
deno task tauri permission ls
```

```
bun tauri permission ls
```

```
cargo tauri permission ls
```

```
List permissions available to your application
Usage: tauri permission ls [OPTIONS] [PLUGIN]
Arguments:  [PLUGIN]  Name of the plugin to list permissions
Options:  -f, --filter <FILTER>  Permission identifier filter  -v, --verbose...       Enables verbose logging  -h, --help             Print help  -V, --version          Print version
```

### `capability`

- [npm](#tab-panel-6504)
- [yarn](#tab-panel-6505)
- [pnpm](#tab-panel-6506)
- [deno](#tab-panel-6507)
- [bun](#tab-panel-6508)
- [cargo](#tab-panel-6509)

```
npm run tauri capability
```

```
yarn tauri capability
```

```
pnpm tauri capability
```

```
deno task tauri capability
```

```
bun tauri capability
```

```
cargo tauri capability
```

```
Manage or create capabilities for your app
Usage: tauri capability [OPTIONS] <COMMAND>
Commands:  new   Create a new permission file  help  Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `capability new`

- [npm](#tab-panel-6510)
- [yarn](#tab-panel-6511)
- [pnpm](#tab-panel-6512)
- [deno](#tab-panel-6513)
- [bun](#tab-panel-6514)
- [cargo](#tab-panel-6515)

```
npm run tauri capability new
```

```
yarn tauri capability new
```

```
pnpm tauri capability new
```

```
deno task tauri capability new
```

```
bun tauri capability new
```

```
cargo tauri capability new
```

```
Create a new permission file
Usage: tauri capability new [OPTIONS] [IDENTIFIER]
Arguments:  [IDENTIFIER]  Capability identifier
Options:      --description <DESCRIPTION>  Capability description  -v, --verbose...                 Enables verbose logging      --windows <WINDOWS>          Capability windows      --permission <PERMISSION>    Capability permissions      --format <FORMAT>            Output file format [default: json] [possible values: json, toml]  -o, --out <OUT>                  The output file  -h, --help                       Print help  -V, --version                    Print version
```

### `inspect`

- [npm](#tab-panel-6516)
- [yarn](#tab-panel-6517)
- [pnpm](#tab-panel-6518)
- [deno](#tab-panel-6519)
- [bun](#tab-panel-6520)
- [cargo](#tab-panel-6521)

```
npm run tauri inspect
```

```
yarn tauri inspect
```

```
pnpm tauri inspect
```

```
deno task tauri inspect
```

```
bun tauri inspect
```

```
cargo tauri inspect
```

```
Inspect values used by Tauri
Usage: tauri inspect [OPTIONS] <COMMAND>
Commands:  wix-upgrade-code  Print the default Upgrade Code used by MSI installer derived from productName  help              Print this message or the help of the given subcommand(s)
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

#### `inspect wix-upgrade-code`

- [npm](#tab-panel-6522)
- [yarn](#tab-panel-6523)
- [pnpm](#tab-panel-6524)
- [deno](#tab-panel-6525)
- [bun](#tab-panel-6526)
- [cargo](#tab-panel-6527)

```
npm run tauri inspect wix-upgrade-code
```

```
yarn tauri inspect wix-upgrade-code
```

```
pnpm tauri inspect wix-upgrade-code
```

```
deno task tauri inspect wix-upgrade-code
```

```
bun tauri inspect wix-upgrade-code
```

```
cargo tauri inspect wix-upgrade-code
```

```
Print the default Upgrade Code used by MSI installer derived from productName
Usage: tauri inspect wix-upgrade-code [OPTIONS]
Options:  -v, --verbose...  Enables verbose logging  -h, --help        Print help  -V, --version     Print version
```

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
