# core

Invoke your custom commands.

This package is also accessible with `window.__TAURI__.core` when
[`app.withGlobalTauri`](https://v2.tauri.app/reference/config/#withglobaltauri)
in `tauri.conf.json` is set to `true`.

## Classes

### Channel\<T\>

#### Type Parameters

| Type Parameter | Default type |
|----------------|--------------|
| `T`            | `unknown`    |

#### Constructors

##### new Channel()

```
new Channel<T>(onmessage?): Channel<T>
```

###### Parameters

| Parameter    | Type                    |
|--------------|-------------------------|
| `onmessage`? | (`response`) =\> `void` |

###### Returns

[`Channel`](https://v2.tauri.app/reference/javascript/api/namespacecore/#channelt)\<`T`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L87>

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `id` | `number` | The callback id returned from [`transformCallback`](https://v2.tauri.app/reference/javascript/api/namespacecore/#transformcallback) | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L79> |

#### Accessors

##### onmessage

```
get onmessage(): (response) => void
```

```
set onmessage(handler): void
```

###### Parameters

| Parameter | Type                    |
|-----------|-------------------------|
| `handler` | (`response`) =\> `void` |

###### Returns

`Function`

###### Parameters

| Parameter  | Type |
|------------|------|
| `response` | `T`  |

###### Returns

`void`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L142>

#### Methods

##### \_\_TAURI_TO_IPC_KEY\_\_()

```
__TAURI_TO_IPC_KEY__(): string
```

###### Returns

`string`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L146>

##### toJSON()

```
toJSON(): string
```

###### Returns

`string`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L150>

------------------------------------------------------------------------

### PluginListener

#### Constructors

##### new PluginListener()

```
new PluginListener(   plugin,   event,   channelId): PluginListener
```

###### Parameters

| Parameter   | Type     |
|-------------|----------|
| `plugin`    | `string` |
| `event`     | `string` |
| `channelId` | `number` |

###### Returns

[`PluginListener`](https://v2.tauri.app/reference/javascript/api/namespacecore/#pluginlistener)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L161>

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `channelId` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L159> |
|  `event` | `string` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L158> |
|  `plugin` | `string` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L157> |

#### Methods

##### unregister()

```
unregister(): Promise<void>
```

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L167>

------------------------------------------------------------------------

### Resource

A rust-backed resource stored through `tauri::Manager::resources_table`
API.

The resource lives in the main process and does not exist in the
Javascript world, and thus will not be cleaned up automatically except
on application exit. If you want to clean it up early, call
[`Resource.close`](https://v2.tauri.app/reference/javascript/api/namespacecore/#close)

#### Example

```
import { Resource, invoke } from '@tauri-apps/api/core';export class DatabaseHandle extends Resource {  static async open(path: string): Promise<DatabaseHandle> {    const rid: number = await invoke('open_db', { path });    return new DatabaseHandle(rid);  }
  async execute(sql: string): Promise<void> {    await invoke('execute_sql', { rid: this.rid, sql });  }}
```

#### Extended by

- [`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image)
- [`TrayIcon`](https://v2.tauri.app/reference/javascript/api/namespacetray/#trayicon)

#### Constructors

##### new Resource()

```
new Resource(rid): Resource
```

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `rid`     | `number` |

###### Returns

[`Resource`](https://v2.tauri.app/reference/javascript/api/namespacecore/#resource)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L322>

#### Accessors

##### rid

```
get rid(): number
```

###### Returns

`number`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L318>

#### Methods

##### close()

```
close(): Promise<void>
```

Destroys and cleans up this resource from memory. **You should not call
any method on this object anymore and should drop any reference to it.**

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L330>

## Interfaces

### InvokeOptions

#### Since

2.0.0

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `headers` | `HeadersInit` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L233> |

## Type Aliases

### InvokeArgs

```
type InvokeArgs: Record<string, unknown> | number[] | ArrayBuffer | Uint8Array;
```

Command arguments.

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L227>

------------------------------------------------------------------------

### PermissionState

```
type PermissionState: "granted" | "denied" | "prompt" | "prompt-with-rationale";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L202>

## Variables

### SERIALIZE_TO_IPC_FN

```
const SERIALIZE_TO_IPC_FN: "__TAURI_TO_IPC_KEY__" = '__TAURI_TO_IPC_KEY__';
```

A key to be used to implement a special function on your types that
define how your type should be serialized when passing across the IPC.

#### Example

Given a type in Rust that looks like this

```
#derive(serde::Serialize, serde::Deserialize)enum UserId {  String(String),  Number(u32),}
```

`UserId::String("id")` would be serialized into `{ String: "id" }` and
so we need to pass the same structure back to Rust

```
import { SERIALIZE_TO_IPC_FN } from "@tauri-apps/api/core"
class UserIdString {  id  constructor(id) {    this.id = id  }
  [SERIALIZE_TO_IPC_FN {    return { String: this.id }  }}
class UserIdNumber {  id  constructor(id) {    this.id = id  }
  SERIALIZE_TO_IPC_FN {    return { Number: this.id }  }}
type UserId = UserIdString | UserIdNumber
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L59>

## Functions

### addPluginListener()

```
function addPluginListener<T>(   plugin,   event,cb): Promise<PluginListener>
```

Adds a listener to a plugin event.

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type                   |
|-----------|------------------------|
| `plugin`  | `string`               |
| `event`   | `string`               |
| `cb`      | (`payload`) =\> `void` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PluginListener`](https://v2.tauri.app/reference/javascript/api/namespacecore/#pluginlistener)\>

The listener object to stop listening to the events.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L182>

------------------------------------------------------------------------

### checkPermissions()

```
function checkPermissions<T>(plugin): Promise<T>
```

Get permission state for a plugin.

This should be used by plugin authors to wrap their actual
implementation.

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `plugin`  | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`T`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L209>

------------------------------------------------------------------------

### convertFileSrc()

```
function convertFileSrc(filePath, protocol): string
```

Convert a device file path to an URL that can be loaded by the webview.
Note that `asset:` and `http://asset.localhost` must be added to
[`app.security.csp`](https://v2.tauri.app/reference/config/#csp-1) in
`tauri.conf.json`. Example CSP value:
`"csp": "default-src 'self' ipc: http://ipc.localhost; img-src 'self' asset: http://asset.localhost"`
to use the asset protocol on image sources.

Additionally, `"enable" : "true"` must be added to
[`app.security.assetProtocol`](https://v2.tauri.app/reference/config/#assetprotocolconfig)
in `tauri.conf.json` and its access scope must be defined on the `scope`
array on the same `assetProtocol` object.

#### Parameters

| Parameter | Type | Default value | Description |
|----|----|----|----|
| `filePath` | `string` | `undefined` | The file path. |
| `protocol` | `string` | `'asset'` | The protocol to use. Defaults to `asset`. You only need to set this when using a custom protocol. |

#### Returns

`string`

the URL that can be used as source on the webview.

#### Example

```
import { appDataDir, join } from '@tauri-apps/api/path';import { convertFileSrc } from '@tauri-apps/api/core';const appDataDirPath = await appDataDir();const filePath = await join(appDataDirPath, 'assets/video.mp4');const assetUrl = convertFileSrc(filePath);
const video = document.getElementById('my-video');const source = document.createElement('source');source.type = 'video/mp4';source.src = assetUrl;video.appendChild(source);video.load();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L289>

------------------------------------------------------------------------

### invoke()

```
function invoke<T>(   cmd,   args,options?): Promise<T>
```

Sends a message to the backend.

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `cmd` | `string` | The command name. |
| `args` | [`InvokeArgs`](https://v2.tauri.app/reference/javascript/api/namespacecore/#invokeargs) | The optional arguments to pass to the command. |
| `options`? | [`InvokeOptions`](https://v2.tauri.app/reference/javascript/api/namespacecore/#invokeoptions) | The request options. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`T`\>

A promise resolving or rejecting to the backend response.

#### Example

```
import { invoke } from '@tauri-apps/api/core';await invoke('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L251>

------------------------------------------------------------------------

### isTauri()

```
function isTauri(): boolean
```

#### Returns

`boolean`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L337>

------------------------------------------------------------------------

### requestPermissions()

```
function requestPermissions<T>(plugin): Promise<T>
```

Request permissions.

This should be used by plugin authors to wrap their actual
implementation.

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `plugin`  | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`T`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L218>

------------------------------------------------------------------------

### transformCallback()

```
function transformCallback<T>(callback?, once?): number
```

Stores the callback in a known location, and returns an identifier that
can be passed to the backend. The backend uses the identifier to
`eval()` the callback.

#### Type Parameters

| Type Parameter | Default type |
|----------------|--------------|
| `T`            | `unknown`    |

#### Parameters

| Parameter   | Type                    | Default value |
|-------------|-------------------------|---------------|
| `callback`? | (`response`) =\> `void` | `undefined`   |
| `once`?     | `boolean`               | `false`       |

#### Returns

`number`

An unique identifier associated with the callback function.

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L69>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
