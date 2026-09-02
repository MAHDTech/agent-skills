+++
title = "plugin-file-system-8247cd8b"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# File System

[GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/fs)[npm](https://www.npmx.dev/package/@tauri-apps/plugin-fs)[crates.io](https://crates.io/crates/tauri-plugin-fs)

API
Reference:[](https://tauri.app/reference/javascript/fs/)[](https://docs.rs/tauri-plugin-fs)

Access the file system.

## Supported Platforms

*This plugin requires a Rust version of at least **1.77.2***

[TABLE]

## Setup

Install the fs plugin to get started.

- [Automatic](#tab-panel-5996)
- [Manual](#tab-panel-5997)

Use your project’s package manager to add the dependency:

- [npm](#tab-panel-5985)
- [yarn](#tab-panel-5986)
- [pnpm](#tab-panel-5987)
- [deno](#tab-panel-5988)
- [bun](#tab-panel-5989)
- [cargo](#tab-panel-5990)

```
npm run tauri add fs
```

```
yarn run tauri add fs
```

```
pnpm tauri add fs
```

```
deno task tauri add fs
```

```
bun tauri add fs
```

```
cargo tauri add fs
```

1.  Run the following command in the `src-tauri` folder to add the
    plugin to the project’s dependencies in `Cargo.toml`:

    ```
    cargo add tauri-plugin-fs
    ```

2.  Modify `lib.rs` to initialize the plugin:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {  tauri::Builder::default()    .plugin(tauri_plugin_fs::init())    .run(tauri::generate_context!())    .expect("error while running tauri application");}
    ```

    src-tauri/src/lib.rs

3.  Install the JavaScript Guest bindings using your preferred
    JavaScript package manager:

    - [npm](#tab-panel-5991)
    - [yarn](#tab-panel-5992)
    - [pnpm](#tab-panel-5993)
    - [deno](#tab-panel-5994)
    - [bun](#tab-panel-5995)

    ```
    npm install @tauri-apps/plugin-fs
    ```

    ```
    yarn add @tauri-apps/plugin-fs
    ```

    ```
    pnpm add @tauri-apps/plugin-fs
    ```

    ```
    deno add npm:@tauri-apps/plugin-fs
    ```

    ```
    bun add @tauri-apps/plugin-fs
    ```

## Configuration

### Android

When using the audio, cache, documents, downloads, picture, public or
video directories your app must have access to the external storage.

Include the following permissions to the `manifest` tag in the
`gen/android/app/src/main/AndroidManifest.xml` file:

```
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE"/><uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
```

### iOS

Apple requires app developers to specify approved reasons for API usage
to enhance user privacy.

You must create a `PrivacyInfo.xcprivacy` file in the
`src-tauri/gen/apple` folder with the required
[NSPrivacyAccessedAPICategoryFileTimestamp](https://developer.apple.com/documentation/bundleresources/privacy_manifest_files/describing_use_of_required_reason_api#4278393)
key and the
[C617.1](https://developer.apple.com/documentation/bundleresources/privacy_manifest_files/describing_use_of_required_reason_api#4278393)
recommended reason.

```
<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0">  <dict>    <key>NSPrivacyAccessedAPITypes</key>    <array>      <dict>        <key>NSPrivacyAccessedAPIType</key>        <string>NSPrivacyAccessedAPICategoryFileTimestamp</string>        <key>NSPrivacyAccessedAPITypeReasons</key>        <array>          <string>C617.1</string>        </array>      </dict>    </array>  </dict></plist>
```

## Usage

The fs plugin is available in both JavaScript and Rust.

- [JavaScript](#tab-panel-5983)
- [Rust](#tab-panel-5984)

```
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';// when using `"withGlobalTauri": true`, you may use// const { exists, BaseDirectory } = window.__TAURI__.fs;
// Check if the `$APPDATA/avatar.png` file existsawait exists('avatar.png', { baseDir: BaseDirectory.AppData });
```

```
use tauri_plugin_fs::FsExt;
#[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {  tauri::Builder::default()      .plugin(tauri_plugin_fs::init())      .setup(|app| {          // allowed the given directory          let scope = app.fs_scope();          scope.allow_directory("/path/to/directory", false);          dbg!(scope.allowed());
          Ok(())       })       .run(tauri::generate_context!())       .expect("error while running tauri application");}
```

src-tauri/src/lib.rs

## Security

This module prevents path traversal, not allowing parent directory
accessors to be used (i.e. “/usr/path/to/../file” or “../path/to/file”
paths are not allowed). Paths accessed with this API must be either
relative to one of the [base
directories](https://tauri.app/reference/javascript/api/namespacepath/#basedirectory) or
created with the [path API](https://tauri.app/reference/javascript/api/namespacepath/).

See [@tauri-apps/plugin-fs -
Security](https://tauri.app/reference/javascript/fs/#security) for more information.

## Paths

The file system plugin offers two ways of manipulating paths: the [base
directory](https://tauri.app/reference/javascript/api/namespacepath/#basedirectory) and
the [path API](https://tauri.app/reference/javascript/api/namespacepath/).

- base directory

  Every API has an options argument that lets you define a
  [baseDir](https://tauri.app/reference/javascript/api/namespacepath/#basedirectory) that
  acts as the working directory of the operation.

  ```
  import { readFile } from '@tauri-apps/plugin-fs';const contents = await readFile('avatars/tauri.png', {  baseDir: BaseDirectory.Home,});
  ```

  In the above example the ~/avatars/tauri.png file is read since we are
  using the **Home** base directory.

- path API

  Alternatively you can use the path APIs to perform path manipulations.

  ```
  import { readFile } from '@tauri-apps/plugin-fs';import * as path from '@tauri-apps/api/path';const home = await path.homeDir();const contents = await readFile(await path.join(home, 'avatars/tauri.png'));
  ```

## Files

### Create

Creates a file and returns a handle to it. If the file already exists,
it is truncated.

```
import { create, BaseDirectory } from '@tauri-apps/plugin-fs';const file = await create('foo/bar.txt', { baseDir: BaseDirectory.AppData });await file.write(new TextEncoder().encode('Hello world'));await file.close();
```

### Write

The plugin offers separate APIs for writing text and binary files for
performance.

- text files

  ```
  import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';const contents = JSON.stringify({ notifications: true });await writeTextFile('config.json', contents, {  baseDir: BaseDirectory.AppConfig,});
  ```

- binary files

  ```
  import { writeFile, BaseDirectory } from '@tauri-apps/plugin-fs';const contents = new Uint8Array(); // fill a byte arrayawait writeFile('config', contents, {  baseDir: BaseDirectory.AppConfig,});
  ```

### Open

Opens a file and returns a handle to it. With this API you have more
control over how the file should be opened (read-only mode, write-only
mode, append instead of overwrite, only create if it does not exist,
etc).

- read-only

  This is the default mode.

  ```
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';const file = await open('foo/bar.txt', {  read: true,  baseDir: BaseDirectory.AppData,});
  const stat = await file.stat();const buf = new Uint8Array(stat.size);await file.read(buf);const textContents = new TextDecoder().decode(buf);await file.close();
  ```

- write-only

  ```
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';const file = await open('foo/bar.txt', {  write: true,  baseDir: BaseDirectory.AppData,});await file.write(new TextEncoder().encode('Hello world'));await file.close();
  ```

  By default the file is truncated on any `file.write()` call. See the
  following example to learn how to append to the existing contents
  instead.

- append

  ```
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';const file = await open('foo/bar.txt', {  append: true,  baseDir: BaseDirectory.AppData,});await file.write(new TextEncoder().encode('world'));await file.close();
  ```

  Note that `{ append: true }` has the same effect as
  `{ write: true, append: true }`.

- truncate

  When the `truncate` option is set and the file already exists, it will
  be truncated to length 0.

  ```
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';const file = await open('foo/bar.txt', {  write: true,  truncate: true,  baseDir: BaseDirectory.AppData,});await file.write(new TextEncoder().encode('world'));await file.close();
  ```

  This option requires `write` to be `true`.

  You can use it along the `append` option if you want to rewrite an
  existing file using multiple `file.write()` calls.

- create

  By default the `open` API only opens existing files. To create the
  file if it does not exist, opening it if it does, set `create` to
  `true`:

  ```
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';const file = await open('foo/bar.txt', {  write: true,  create: true,  baseDir: BaseDirectory.AppData,});await file.write(new TextEncoder().encode('world'));await file.close();
  ```

  In order for the file to be created, `write` or `append` must also be
  set to `true`.

  To fail if the file already exists, see `createNew`.

- createNew

  `createNew` works similarly to `create`, but will fail if the file
  already exists.

  ```
  import { open, BaseDirectory } from '@tauri-apps/plugin-fs';const file = await open('foo/bar.txt', {  write: true,  createNew: true,  baseDir: BaseDirectory.AppData,});await file.write(new TextEncoder().encode('world'));await file.close();
  ```

  In order for the file to be created, `write` must also be set to
  `true`.

### Read

The plugin offers separate APIs for reading text and binary files for
performance.

- text files

  ```
  import { readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';const configToml = await readTextFile('config.toml', {  baseDir: BaseDirectory.AppConfig,});
  ```

  If the file is large you can stream its lines with the
  `readTextFileLines` API:

  ```
  import { readTextFileLines, BaseDirectory } from '@tauri-apps/plugin-fs';const lines = await readTextFileLines('app.logs', {  baseDir: BaseDirectory.AppLog,});for await (const line of lines) {  console.log(line);}
  ```

- binary files

  ```
  import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';const icon = await readFile('icon.png', {  baseDir: BaseDirectory.Resources,});
  ```

### Remove

Call `remove()` to delete a file. If the file does not exist, an error
is returned.

```
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';await remove('user.db', { baseDir: BaseDirectory.AppLocalData });
```

### Copy

The `copyFile` function takes the source and destination paths. Note
that you must configure each base directory separately.

```
import { copyFile, BaseDirectory } from '@tauri-apps/plugin-fs';await copyFile('user.db', 'user.db.bk', {  fromPathBaseDir: BaseDirectory.AppLocalData,  toPathBaseDir: BaseDirectory.Temp,});
```

In the above example the \<app-local-data\>/user.db file is copied to
\$TMPDIR/user.db.bk.

### Exists

Use the `exists()` function to check if a file exists:

```
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';const tokenExists = await exists('token', {  baseDir: BaseDirectory.AppLocalData,});
```

### Metadata

File metadata can be retrieved with the `stat` and the `lstat`
functions. `stat` follows symlinks (and returns an error if the actual
file it points to is not allowed by the scope) and `lstat` does not
follow symlinks, returning the information of the symlink itself.

```
import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';const metadata = await stat('app.db', {  baseDir: BaseDirectory.AppLocalData,});
```

### Rename

The `rename` function takes the source and destination paths. Note that
you must configure each base directory separately.

```
import { rename, BaseDirectory } from '@tauri-apps/plugin-fs';await rename('user.db.bk', 'user.db', {  fromPathBaseDir: BaseDirectory.AppLocalData,  toPathBaseDir: BaseDirectory.Temp,});
```

In the above example the \<app-local-data\>/user.db.bk file is renamed
to \$TMPDIR/user.db.

### Truncate

Truncates or extends the specified file to reach the specified file
length (defaults to 0).

- truncate to 0 length

```
import { truncate } from '@tauri-apps/plugin-fs';await truncate('my_file.txt', 0, { baseDir: BaseDirectory.AppLocalData });
```

- truncate to a specific length

```
import {  truncate,  readTextFile,  writeTextFile,  BaseDirectory,} from '@tauri-apps/plugin-fs';
const filePath = 'file.txt';await writeTextFile(filePath, 'Hello World', {  baseDir: BaseDirectory.AppLocalData,});await truncate(filePath, 7, {  baseDir: BaseDirectory.AppLocalData,});const data = await readTextFile(filePath, {  baseDir: BaseDirectory.AppLocalData,});console.log(data); // "Hello W"
```

## Directories

### Create

To create a directory, call the `mkdir` function:

```
import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';await mkdir('images', {  baseDir: BaseDirectory.AppLocalData,});
```

### Read

The `readDir` function recursively lists the entries of a directory:

```
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs';const entries = await readDir('users', { baseDir: BaseDirectory.AppLocalData });
```

### Remove

Call `remove()` to delete a directory. If the directory does not exist,
an error is returned.

```
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';await remove('images', { baseDir: BaseDirectory.AppLocalData });
```

If the directory is not empty, the `recursive` option must be set to
`true`:

```
import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';await remove('images', {  baseDir: BaseDirectory.AppLocalData,  recursive: true,});
```

### Exists

Use the `exists()` function to check if a directory exists:

```
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';const tokenExists = await exists('images', {  baseDir: BaseDirectory.AppLocalData,});
```

### Metadata

Directory metadata can be retrieved with the `stat` and the `lstat`
functions. `stat` follows symlinks (and returns an error if the actual
file it points to is not allowed by the scope) and `lstat` does not
follow symlinks, returning the information of the symlink itself.

```
import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';const metadata = await stat('databases', {  baseDir: BaseDirectory.AppLocalData,});
```

## Watching changes

To watch a directory or file for changes, use the `watch` or
`watchImmediate` functions.

- watch

  `watch` is debounced so it only emits events after a certain delay:

  ```
  import { watch, BaseDirectory } from '@tauri-apps/plugin-fs';await watch(  'app.log',  (event) => {    console.log('app.log event', event);  },  {    baseDir: BaseDirectory.AppLog,    delayMs: 500,  });
  ```

- watchImmediate

  `watchImmediate` immediately notifies listeners of an event:

  ```
  import { watchImmediate, BaseDirectory } from '@tauri-apps/plugin-fs';await watchImmediate(  'logs',  (event) => {    console.log('logs directory event', event);  },  {    baseDir: BaseDirectory.AppLog,    recursive: true,  });
  ```

By default watch operations on a directory are not recursive. Set the
`recursive` option to `true` to recursively watch for changes on all
sub-directories.

## Permissions

By default all potentially dangerous plugin commands and scopes are
blocked and cannot be accessed. You must modify the permissions in your
`capabilities` configuration to enable these.

See the [Capabilities Overview](https://tauri.app/security/capabilities/) for more
information and the [step by step
guide](https://tauri.app/learn/security/using-plugin-permissions/) to use plugin
permissions.

```
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": [    "fs:default",    {      "identifier": "fs:allow-exists",      "allow": [{ "path": "$APPDATA/*" }]    }  ]}
```

src-tauri/capabilities/default.json

## [Default Permission](#default-permission)

This set of permissions describes the what kind of file system access
the `fs` plugin has enabled or denied by default.

#### [Granted Permissions](#granted-permissions)

This default permission set enables read access to the application
specific directories (AppConfig, AppData, AppLocalData, AppCache,
AppLog) and all files and sub directories created in it. The location of
these directories depends on the operating system, where the application
is run.

In general these directories need to be manually created by the
application at runtime, before accessing files or folders in it is
possible.

Therefore, it is also allowed to create all of these folders via the
`mkdir` command.

#### Denied Permissions

This default permission set prevents access to critical components of
the Tauri application by default.

- On Windows the access to webview data folder `$APPLOCALDATA/EBWebView`
  is denied.
- On Linux the access to webview data paths inside `$APPLOCALDATA` are
  denied.

#### This default permission set includes the following:

- `create-app-specific-dirs`
- `read-app-specific-dirs-recursive`
- `deny-default`

## Permission Table

[TABLE]

### Scopes

This plugin permissions includes scopes for defining which paths are
allowed or explicitly rejected. For more information on scopes, see the
[Command Scopes](https://tauri.app/security/scope/).

Each `allow` or `deny` scope must include an array listing all paths
that should be allowed or denied. The scope entries are in the
`{ path: string }` format.

Scope entries can use `$<path>` variables to reference common system
paths such as the home directory, the app resources directory and the
config directory. The following table lists all common paths you can
reference:

| Path | Variable |
|----|----|
| [appConfigDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appconfigdir) | \$APPCONFIG |
| [appDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appdatadir) | \$APPDATA |
| [appLocalDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appLocaldatadir) | \$APPLOCALDATA |
| [appcacheDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appcachedir) | \$APPCACHE |
| [applogDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#applogdir) | \$APPLOG |
| [audioDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#audiodir) | \$AUDIO |
| [cacheDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#cachedir) | \$CACHE |
| [configDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#configdir) | \$CONFIG |
| [dataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#datadir) | \$DATA |
| [localDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#localdatadir) | \$LOCALDATA |
| [desktopDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#desktopdir) | \$DESKTOP |
| [documentDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#documentdir) | \$DOCUMENT |
| [downloadDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#downloaddir) | \$DOWNLOAD |
| [executableDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#executabledir) | \$EXE |
| [fontDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#fontdir) | \$FONT |
| [homeDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#homedir) | \$HOME |
| [pictureDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#picturedir) | \$PICTURE |
| [publicDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#publicdir) | \$PUBLIC |
| [runtimeDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#runtimedir) | \$RUNTIME |
| [templateDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#templatedir) | \$TEMPLATE |
| [videoDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#videodir) | \$VIDEO |
| [resourceDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#resourcedir) | \$RESOURCE |
| [tempDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#tempdir) | \$TEMP |

#### Examples

- global scope

To apply a scope to any `fs` command, use the `fs:scope` permission:

```
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": [    {      "identifier": "fs:scope",      "allow": [{ "path": "$APPDATA" }, { "path": "$APPDATA/**/*" }]    }  ]}
```

src-tauri/capabilities/default.json

To apply a scope to a specific `fs` command, use the object form of
permissions `{ "identifier": string, "allow"?: [], "deny"?: [] }`:

```
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "main-capability",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": [    {      "identifier": "fs:allow-rename",      "allow": [{ "path": "$HOME/**/*" }]    },    {      "identifier": "fs:allow-rename",      "deny": [{ "path": "$HOME/.config/**/*" }]    },    {      "identifier": "fs:allow-exists",      "allow": [{ "path": "$APPDATA/*" }]    }  ]}
```

src-tauri/capabilities/default.json

In the above example you can use the [`exists`](#exists) API using any
`$APPDATA` sub path (does not include sub-directories) and the
[`rename`](#rename)

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

