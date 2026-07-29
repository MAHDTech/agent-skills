+++
title = "reference-javascript-api-namespacepath-32079ed3"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# path

The path module provides utilities for working with file and directory
paths.

This package is also accessible with `window.__TAURI__.path` when
[`app.withGlobalTauri`](https://v2.tauri.app/reference/config/#withglobaltauri)
in `tauri.conf.json` is set to `true`.

It is recommended to allowlist only the APIs you use for optimal bundle
size and security.

## Enumerations

### BaseDirectory

#### Since

2.0.0

#### Enumeration Members

##### AppCache

```
AppCache: 16;
```

###### See

[appCacheDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appcachedir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L83>

##### AppConfig

```
AppConfig: 13;
```

###### See

[appConfigDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appconfigdir)
for more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L71>

##### AppData

```
AppData: 14;
```

###### See

[appDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#appdatadir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L75>

##### AppLocalData

```
AppLocalData: 15;
```

###### See

[appLocalDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#applocaldatadir)
for more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L79>

##### AppLog

```
AppLog: 17;
```

###### See

[appLogDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#applogdir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L87>

##### Audio

```
Audio: 1;
```

###### See

[audioDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#audiodir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L23>

##### Cache

```
Cache: 2;
```

###### See

[cacheDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#cachedir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L27>

##### Config

```
Config: 3;
```

###### See

[configDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#configdir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L31>

##### Data

```
Data: 4;
```

###### See

[dataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#datadir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L35>

##### Desktop

```
Desktop: 18;
```

###### See

[desktopDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#desktopdir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L91>

##### Document

```
Document: 6;
```

###### See

[documentDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#documentdir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L43>

##### Download

```
Download: 7;
```

###### See

[downloadDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#downloaddir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L47>

##### Executable

```
Executable: 19;
```

###### See

[executableDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#executabledir)
for more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L95>

##### Font

```
Font: 20;
```

###### See

[fontDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#fontdir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L99>

##### Home

```
Home: 21;
```

###### See

[homeDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#homedir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L103>

##### LocalData

```
LocalData: 5;
```

###### See

[localDataDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#localdatadir)
for more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L39>

##### Picture

```
Picture: 8;
```

###### See

[pictureDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#picturedir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L51>

##### Public

```
Public: 9;
```

###### See

[publicDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#publicdir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L55>

##### Resource

```
Resource: 11;
```

###### See

[resourceDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#resourcedir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L63>

##### Runtime

```
Runtime: 22;
```

###### See

[runtimeDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#runtimedir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L107>

##### Temp

```
Temp: 12;
```

###### See

[tempDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#tempdir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L67>

##### Template

```
Template: 23;
```

###### See

[templateDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#templatedir) for
more information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L111>

##### Video

```
Video: 10;
```

###### See

[videoDir](https://v2.tauri.app/reference/javascript/api/namespacepath/#videodir) for more
information.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L59>

## Functions

### appCacheDir()

```
function appCacheDir(): Promise<string>
```

Returns the path to the suggested directory for your app’s cache files.
Resolves to `${cacheDir}/${bundleIdentifier}`, where `bundleIdentifier`
is the [`identifier`](https://v2.tauri.app/reference/config/#identifier)
value configured in `tauri.conf.json`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { appCacheDir } from '@tauri-apps/api/path';const appCacheDirPath = await appCacheDir();
```

#### Since

1.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L176>

------------------------------------------------------------------------

### appConfigDir()

```
function appConfigDir(): Promise<string>
```

Returns the path to the suggested directory for your app’s config files.
Resolves to `${configDir}/${bundleIdentifier}`, where `bundleIdentifier`
is the [`identifier`](https://v2.tauri.app/reference/config/#identifier)
value configured in `tauri.conf.json`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { appConfigDir } from '@tauri-apps/api/path';const appConfigDirPath = await appConfigDir();
```

#### Since

1.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L125>

------------------------------------------------------------------------

### appDataDir()

```
function appDataDir(): Promise<string>
```

Returns the path to the suggested directory for your app’s data files.
Resolves to `${dataDir}/${bundleIdentifier}`, where `bundleIdentifier`
is the [`identifier`](https://v2.tauri.app/reference/config/#identifier)
value configured in `tauri.conf.json`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { appDataDir } from '@tauri-apps/api/path';const appDataDirPath = await appDataDir();
```

#### Since

1.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L142>

------------------------------------------------------------------------

### appLocalDataDir()

```
function appLocalDataDir(): Promise<string>
```

Returns the path to the suggested directory for your app’s local data
files. Resolves to `${localDataDir}/${bundleIdentifier}`, where
`bundleIdentifier` is the
[`identifier`](https://v2.tauri.app/reference/config/#identifier) value
configured in `tauri.conf.json`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { appLocalDataDir } from '@tauri-apps/api/path';const appLocalDataDirPath = await appLocalDataDir();
```

#### Since

1.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L159>

------------------------------------------------------------------------

### appLogDir()

```
function appLogDir(): Promise<string>
```

Returns the path to the suggested directory for your app’s log files.

Platform-specific

- **Linux:** Resolves to `${configDir}/${bundleIdentifier}/logs`.
- **macOS:** Resolves to `${homeDir}/Library/Logs/{bundleIdentifier}`
- **Windows:** Resolves to `${configDir}/${bundleIdentifier}/logs`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { appLogDir } from '@tauri-apps/api/path';const appLogDirPath = await appLogDir();
```

#### Since

1.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L604>

------------------------------------------------------------------------

### audioDir()

```
function audioDir(): Promise<string>
```

Returns the path to the user’s audio directory.

Platform-specific

- **Linux:** Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_MUSIC_DIR`.
- **macOS:** Resolves to `$HOME/Music`.
- **Windows:** Resolves to `{FOLDERID_Music}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { audioDir } from '@tauri-apps/api/path';const audioDirPath = await audioDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L198>

------------------------------------------------------------------------

### basename()

```
function basename(path, ext?): Promise<string>
```

Returns the last portion of a `path`. Trailing directory separators are
ignored.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `path` | `string` | \- |
| `ext`? | `string` | An optional file extension to be removed from the returned path. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { basename } from '@tauri-apps/api/path';const base = await basename('path/to/app.conf');assert(base === 'app.conf');
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L734>

------------------------------------------------------------------------

### cacheDir()

```
function cacheDir(): Promise<string>
```

Returns the path to the user’s cache directory.

Platform-specific

- **Linux:** Resolves to `$XDG_CACHE_HOME` or `$HOME/.cache`.
- **macOS:** Resolves to `$HOME/Library/Caches`.
- **Windows:** Resolves to `{FOLDERID_LocalAppData}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { cacheDir } from '@tauri-apps/api/path';const cacheDirPath = await cacheDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L220>

------------------------------------------------------------------------

### configDir()

```
function configDir(): Promise<string>
```

Returns the path to the user’s config directory.

Platform-specific

- **Linux:** Resolves to `$XDG_CONFIG_HOME` or `$HOME/.config`.
- **macOS:** Resolves to `$HOME/Library/Application Support`.
- **Windows:** Resolves to `{FOLDERID_RoamingAppData}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { configDir } from '@tauri-apps/api/path';const configDirPath = await configDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L242>

------------------------------------------------------------------------

### dataDir()

```
function dataDir(): Promise<string>
```

Returns the path to the user’s data directory.

Platform-specific

- **Linux:** Resolves to `$XDG_DATA_HOME` or `$HOME/.local/share`.
- **macOS:** Resolves to `$HOME/Library/Application Support`.
- **Windows:** Resolves to `{FOLDERID_RoamingAppData}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { dataDir } from '@tauri-apps/api/path';const dataDirPath = await dataDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L264>

------------------------------------------------------------------------

### delimiter()

```
function delimiter(): string
```

Returns the platform-specific path segment delimiter:

- `;` on Windows
- `:` on POSIX

#### Returns

`string`

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L644>

------------------------------------------------------------------------

### desktopDir()

```
function desktopDir(): Promise<string>
```

Returns the path to the user’s desktop directory.

Platform-specific

- **Linux:** Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_DESKTOP_DIR`.
- **macOS:** Resolves to `$HOME/Desktop`.
- **Windows:** Resolves to `{FOLDERID_Desktop}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { desktopDir } from '@tauri-apps/api/path';const desktopPath = await desktopDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L286>

------------------------------------------------------------------------

### dirname()

```
function dirname(path): Promise<string>
```

Returns the parent directory of a given `path`. Trailing directory
separators are ignored.

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { dirname } from '@tauri-apps/api/path';const dir = await dirname('/path/to/somedir/');assert(dir === '/path/to');
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L703>

------------------------------------------------------------------------

### documentDir()

```
function documentDir(): Promise<string>
```

Returns the path to the user’s document directory.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { documentDir } from '@tauri-apps/api/path';const documentDirPath = await documentDir();
```

Platform-specific

- **Linux:** Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_DOCUMENTS_DIR`.
- **macOS:** Resolves to `$HOME/Documents`.
- **Windows:** Resolves to `{FOLDERID_Documents}`.

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L308>

------------------------------------------------------------------------

### downloadDir()

```
function downloadDir(): Promise<string>
```

Returns the path to the user’s download directory.

Platform-specific

- **Linux**: Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_DOWNLOAD_DIR`.
- **macOS**: Resolves to `$HOME/Downloads`.
- **Windows**: Resolves to `{FOLDERID_Downloads}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { downloadDir } from '@tauri-apps/api/path';const downloadDirPath = await downloadDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L330>

------------------------------------------------------------------------

### executableDir()

```
function executableDir(): Promise<string>
```

Returns the path to the user’s executable directory.

Platform-specific

- **Linux:** Resolves to `$XDG_BIN_HOME/../bin` or
  `$XDG_DATA_HOME/../bin` or `$HOME/.local/bin`.
- **macOS:** Not supported.
- **Windows:** Not supported.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { executableDir } from '@tauri-apps/api/path';const executableDirPath = await executableDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L352>

------------------------------------------------------------------------

### extname()

```
function extname(path): Promise<string>
```

Returns the extension of the `path`.

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { extname } from '@tauri-apps/api/path';const ext = await extname('/path/to/file.html');assert(ext === 'html');
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L718>

------------------------------------------------------------------------

### fontDir()

```
function fontDir(): Promise<string>
```

Returns the path to the user’s font directory.

Platform-specific

- **Linux:** Resolves to `$XDG_DATA_HOME/fonts` or
  `$HOME/.local/share/fonts`.
- **macOS:** Resolves to `$HOME/Library/Fonts`.
- **Windows:** Not supported.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { fontDir } from '@tauri-apps/api/path';const fontDirPath = await fontDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L374>

------------------------------------------------------------------------

### homeDir()

```
function homeDir(): Promise<string>
```

Returns the path to the user’s home directory.

Platform-specific

- **Linux:** Resolves to `$HOME`.
- **macOS:** Resolves to `$HOME`.
- **Windows:** Resolves to `{FOLDERID_Profile}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { homeDir } from '@tauri-apps/api/path';const homeDirPath = await homeDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L396>

------------------------------------------------------------------------

### isAbsolute()

```
function isAbsolute(path): Promise<boolean>
```

Returns whether the path is absolute or not.

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

#### Example

```
import { isAbsolute } from '@tauri-apps/api/path';assert(await isAbsolute('/home/tauri'));
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L748>

------------------------------------------------------------------------

### join()

```
function join(...paths): Promise<string>
```

Joins all given `path` segments together using the platform-specific
separator as a delimiter, then normalizes the resulting path.

#### Parameters

| Parameter | Type         |
|-----------|--------------|
| …`paths`  | `string`\[\] |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { join, appDataDir } from '@tauri-apps/api/path';const appDataDirPath = await appDataDir();const path = await join(appDataDirPath, 'users', 'tauri', 'avatar.png');
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L688>

------------------------------------------------------------------------

### localDataDir()

```
function localDataDir(): Promise<string>
```

Returns the path to the user’s local data directory.

Platform-specific

- **Linux:** Resolves to `$XDG_DATA_HOME` or `$HOME/.local/share`.
- **macOS:** Resolves to `$HOME/Library/Application Support`.
- **Windows:** Resolves to `{FOLDERID_LocalAppData}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { localDataDir } from '@tauri-apps/api/path';const localDataDirPath = await localDataDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L418>

------------------------------------------------------------------------

### normalize()

```
function normalize(path): Promise<string>
```

Normalizes the given `path`, resolving `'..'` and `'.'` segments and
resolve symbolic links.

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { normalize, appDataDir } from '@tauri-apps/api/path';const appDataDirPath = await appDataDir();const path = await normalize(`${appDataDirPath}/../users/tauri/avatar.png`);
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L673>

------------------------------------------------------------------------

### pictureDir()

```
function pictureDir(): Promise<string>
```

Returns the path to the user’s picture directory.

Platform-specific

- **Linux:** Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_PICTURES_DIR`.
- **macOS:** Resolves to `$HOME/Pictures`.
- **Windows:** Resolves to `{FOLDERID_Pictures}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { pictureDir } from '@tauri-apps/api/path';const pictureDirPath = await pictureDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L440>

------------------------------------------------------------------------

### publicDir()

```
function publicDir(): Promise<string>
```

Returns the path to the user’s public directory.

Platform-specific

- **Linux:** Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_PUBLICSHARE_DIR`.
- **macOS:** Resolves to `$HOME/Public`.
- **Windows:** Resolves to `{FOLDERID_Public}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { publicDir } from '@tauri-apps/api/path';const publicDirPath = await publicDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L462>

------------------------------------------------------------------------

### resolve()

```
function resolve(...paths): Promise<string>
```

Resolves a sequence of `paths` or `path` segments into an absolute path.

#### Parameters

| Parameter | Type         |
|-----------|--------------|
| …`paths`  | `string`\[\] |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { resolve, appDataDir } from '@tauri-apps/api/path';const appDataDirPath = await appDataDir();const path = await resolve(appDataDirPath, '..', 'users', 'tauri', 'avatar.png');
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L658>

------------------------------------------------------------------------

### resolveResource()

```
function resolveResource(resourcePath): Promise<string>
```

Resolve the path to a resource file.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `resourcePath` | `string` | The path to the resource. Must follow the same syntax as defined in `tauri.conf.json > bundle > resources`, i.e. keeping subfolders and parent dir components (`../`). |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

The full path to the resource.

#### Example

```
import { resolveResource } from '@tauri-apps/api/path';const resourcePath = await resolveResource('script.sh');
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L515>

------------------------------------------------------------------------

### resourceDir()

```
function resourceDir(): Promise<string>
```

Returns the path to the application’s resource directory. To resolve a
resource path, see
[`resolveResource`](https://v2.tauri.app/reference/javascript/api/namespacepath/#resolveresource).

## Platform-specific

Although we provide the exact path where this function resolves to, this
is not a contract and things might change in the future

- **Windows:** Resolves to the directory that contains the main
  executable.
- **Linux:** When running in an AppImage, the `APPDIR` variable will be
  set to the mounted location of the app, and the resource dir will be
  `${APPDIR}/usr/lib/${exe_name}`. If not running in an AppImage, the
  path is `/usr/lib/${exe_name}`. When running the app from
  `src-tauri/target/(debug|release)/`, the path is
  `${exe_dir}/../lib/${exe_name}`.
- **macOS:** Resolves to `${exe_dir}/../Resources` (inside .app).
- **iOS:** Resolves to `${exe_dir}/assets`.
- **Android:** Currently the resources are stored in the APK as assets
  so it’s not a normal file system path, we return a special URI prefix
  `asset://localhost/` here that can be used with the [file system
  plugin](https://tauri.app/plugin/file-system/),

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { resourceDir } from '@tauri-apps/api/path';const resourceDirPath = await resourceDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L495>

------------------------------------------------------------------------

### runtimeDir()

```
function runtimeDir(): Promise<string>
```

Returns the path to the user’s runtime directory.

Platform-specific

- **Linux:** Resolves to `$XDG_RUNTIME_DIR`.
- **macOS:** Not supported.
- **Windows:** Not supported.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { runtimeDir } from '@tauri-apps/api/path';const runtimeDirPath = await runtimeDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L538>

------------------------------------------------------------------------

### sep()

```
function sep(): string
```

Returns the platform-specific path segment separator:

- `\` on Windows
- `/` on POSIX

#### Returns

`string`

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L633>

------------------------------------------------------------------------

### tempDir()

```
function tempDir(): Promise<string>
```

Returns a temporary directory.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { tempDir } from '@tauri-apps/api/path';const temp = await tempDir();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L620>

------------------------------------------------------------------------

### templateDir()

```
function templateDir(): Promise<string>
```

Returns the path to the user’s template directory.

Platform-specific

- **Linux:** Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_TEMPLATES_DIR`.
- **macOS:** Not supported.
- **Windows:** Resolves to `{FOLDERID_Templates}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { templateDir } from '@tauri-apps/api/path';const templateDirPath = await templateDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L560>

------------------------------------------------------------------------

### videoDir()

```
function videoDir(): Promise<string>
```

Returns the path to the user’s video directory.

Platform-specific

- **Linux:** Resolves to
  [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)’
  `XDG_VIDEOS_DIR`.
- **macOS:** Resolves to `$HOME/Movies`.
- **Windows:** Resolves to `{FOLDERID_Videos}`.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { videoDir } from '@tauri-apps/api/path';const videoDirPath = await videoDir();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L582>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
