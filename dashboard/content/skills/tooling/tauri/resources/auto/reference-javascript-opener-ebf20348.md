+++
title = "reference-javascript-opener-ebf20348"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-opener

Open files and URLs using their default application.

## Security

This API has a scope configuration that forces you to restrict the files
and urls to be opened.

### Restricting access to the open \| `open` API

On the configuration object, `open: true` means that the open API can be
used with any URL, as the argument is validated with the
`^((mailto:\w+)|(tel:\w+)|(https?://\w+)).+` regex. You can change that
regex by changing the boolean value to a string, e.g.
`open: ^https://github.com/`.

## Functions

### openPath()

```
function openPath(path, openWith?): Promise<void>
```

Opens a path with the system’s default app, or the one specified with
openWith.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `path` | `string` | The path to open. |
| `openWith`? | `string` | The app to open the path with. If not specified, defaults to the system default application for the specified path type. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { openPath } from '@tauri-apps/plugin-opener';
// opens a file using the default program:await openPath('/path/to/file');// opens a file using `vlc` command on Windows.await openPath('C:/path/to/file', 'vlc');
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/opener/guest-js/index.ts#L71>

------------------------------------------------------------------------

### openUrl()

```
function openUrl(url, openWith?): Promise<void>
```

Opens a url with the system’s default app, or the one specified with
openWith.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `url` | `string` \| [`URL`](https://developer.mozilla.org/docs/Web/API/URL) | The URL to open. |
| `openWith`? | `string` | The app to open the URL with. If not specified, defaults to the system default application for the specified url type. On mobile, `openWith` can be provided as `inAppBrowser` to open the URL in an in-app browser. Otherwise, it will open the URL in the system default browser. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { openUrl } from '@tauri-apps/plugin-opener';
// opens the given URL on the default browser:await openUrl('https://github.com/tauri-apps/tauri');// opens the given URL using `firefox`:await openUrl('https://github.com/tauri-apps/tauri', 'firefox');
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/opener/guest-js/index.ts#L42>

------------------------------------------------------------------------

### revealItemInDir()

```
function revealItemInDir(path): Promise<void>
```

Reveal a path with the system’s default explorer.

Platform-specific:

- **Android / iOS:** Unsupported.

#### Parameters

| Parameter | Type                     | Description         |
|-----------|--------------------------|---------------------|
| `path`    | `string` \| `string`\[\] | The path to reveal. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { revealItemInDir } from '@tauri-apps/plugin-opener';await revealItemInDir('/path/to/file');await revealItemInDir([ '/path/to/file', '/path/to/another/file' ]);
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/opener/guest-js/index.ts#L96>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

