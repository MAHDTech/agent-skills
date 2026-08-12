# @tauri-apps/plugin-deep-link

## Functions

### getCurrent()

```
function getCurrent(): Promise<string[] | null>
```

Get the current URLs that triggered the deep link. Use this on app load
to check whether your app was started via a deep link.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\[\]
\| `null`\>

#### Example

```
import { getCurrent } from '@tauri-apps/plugin-deep-link';const urls = await getCurrent();
```

Platform-specific

- **Windows / Linux:** This function reads the command line arguments
  and checks if there’s only one value, which must be an URL with scheme
  matching one of the configured values. Note that you must manually
  check the arguments when registering deep link schemes dynamically
  with \[`Self::register`\]. Additionally, the deep link might have been
  provided as a CLI argument so you should check if its format matches
  what you expect.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L25>

------------------------------------------------------------------------

### isRegistered()

```
function isRegistered(protocol): Promise<boolean>
```

Check whether the app is the default handler for the specified protocol.

#### Parameters

| Parameter  | Type     | Description                             |
|------------|----------|-----------------------------------------|
| `protocol` | `string` | The name of the protocol without `://`. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

#### Example

```
import { isRegistered } from '@tauri-apps/plugin-deep-link';await isRegistered("my-scheme");
```

Platform-specific

- **macOS / Android / iOS:** Unsupported.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L88>

------------------------------------------------------------------------

### onOpenUrl()

```
function onOpenUrl(handler): Promise<UnlistenFn>
```

Helper function for the `deep-link://new-url` event to run a function
each time the protocol is triggered while the app is running. Use
`getCurrent` on app load to check whether your app was started via a
deep link.

#### Parameters

| Parameter | Type                |
|-----------|---------------------|
| `handler` | (`urls`) =\> `void` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`UnlistenFn`\>

#### Example

```
import { onOpenUrl } from '@tauri-apps/plugin-deep-link';await onOpenUrl((urls) => { console.log(urls) });
```

Platform-specific

- **Windows / Linux:** Unsupported without the single-instance plugin.
  The OS will spawn a new app instance passing the URL as a CLI
  argument.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L109>

------------------------------------------------------------------------

### register()

```
function register(protocol): Promise<null>
```

Register the app as the default handler for the specified protocol.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `protocol` | `string` | The name of the protocol without `://`. For example, if you want your app to handle `tauri://` links, call this method with `tauri` as the protocol. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`\>

#### Example

```
import { register } from '@tauri-apps/plugin-deep-link';await register("my-scheme");
```

Platform-specific

- **macOS / Android / iOS:** Unsupported.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L46>

------------------------------------------------------------------------

### unregister()

```
function unregister(protocol): Promise<null>
```

Unregister the app as the default handler for the specified protocol.

#### Parameters

| Parameter  | Type     | Description                             |
|------------|----------|-----------------------------------------|
| `protocol` | `string` | The name of the protocol without `://`. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`\>

#### Example

```
import { unregister } from '@tauri-apps/plugin-deep-link';await unregister("my-scheme");
```

Platform-specific

- **macOS / Linux / Android / iOS:** Unsupported.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L67>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
