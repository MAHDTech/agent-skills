# @tauri-apps/plugin-websocket

## Classes

### default

#### Constructors

##### new default()

```
new default(id, listeners): default
```

###### Parameters

| Parameter | Type |
|----|----|
| `id` | `number` |
| `listeners` | [`Set`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Set)\<(`arg`) =\> `void`\> |

###### Returns

[`default`](https://v2.tauri.app/reference/javascript/websocket/#default)

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L63>

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `id` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L60> |

#### Methods

##### addListener()

```
addListener(cb): () => void
```

###### Parameters

| Parameter | Type               |
|-----------|--------------------|
| `cb`      | (`arg`) =\> `void` |

###### Returns

`Function`

###### Returns

`void`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L92>

##### disconnect()

```
disconnect(): Promise<void>
```

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L119>

##### send()

```
send(message): Promise<void>
```

###### Parameters

| Parameter | Type |
|----|----|
| `message` | `string` \| `number`\[\] \| [`Message`](https://v2.tauri.app/reference/javascript/websocket/#message) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L100>

##### connect()

```
static connect(url, config?): Promise<default>
```

###### Parameters

| Parameter | Type |
|----|----|
| `url` | `string` |
| `config`? | [`ConnectionConfig`](https://v2.tauri.app/reference/javascript/websocket/#connectionconfig) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`default`](https://v2.tauri.app/reference/javascript/websocket/#default)\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L68>

## Interfaces

### CloseFrame

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `code` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L48> |
|  `reason` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L49> |

------------------------------------------------------------------------

### ConnectionConfig

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `acceptUnmaskedFrames?` | `boolean` | When set to true, the server will accept and handle unmasked frames from the client. According to the RFC 6455, the server must close the connection to the client in such cases, however it seems like there are some popular libraries that are sending unmasked frames, ignoring the RFC. By default this option is set to false, i.e. according to RFC 6455. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L35> |
|  `headers?` | `HeadersInit` | Additional connect request headers. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L39> |
|  `maxFrameSize?` | `number` \| `"none"` | The maximum size of a single incoming message frame. The string “none” means no size limit. The limit is for frame payload NOT including the frame header. The default value is 16 MiB which should be reasonably big for all normal use-cases but small enough to prevent memory eating by a malicious user. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L31> |
|  `maxMessageSize?` | `number` \| `"none"` | The maximum size of an incoming message. The string “none” means no size limit. The default value is 64 MiB which should be reasonably big for all normal use-cases but small enough to prevent memory eating by a malicious user. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L27> |
|  `maxWriteBufferSize?` | `number` | The max size of the write buffer in bytes. Setting this can provide backpressure in the case the write buffer is filling up due to write errors. The default value is unlimited. Note: The write buffer only builds up past write_buffer_size when writes to the underlying stream are failing. So the write buffer can not fill up if you are not observing write errors. Note: Should always be at least write_buffer_size + 1 message and probably a little more depending on error handling strategy. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L23> |
|  `readBufferSize?` | `number` | Read buffer capacity. The default value is 128 KiB. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L11> |
|  `writeBufferSize?` | `number` | The target minimum size of the write buffer to reach before writing the data to the underlying stream. The default value is 128 KiB. If set to 0 each message will be eagerly written to the underlying stream. It is often more optimal to allow them to buffer a little, hence the default value. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L16> |

------------------------------------------------------------------------

### MessageKind\<T, D\>

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |
| `D`            |

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `data` | `D` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L44> |
|  `type` | `T` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L43> |

## Type Aliases

### Message

```
type Message:  | MessageKind<"Text", string>  | MessageKind<"Binary", number[]>  | MessageKind<"Ping", number[]>  | MessageKind<"Pong", number[]>| MessageKind<"Close", CloseFrame | null>;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L52>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
