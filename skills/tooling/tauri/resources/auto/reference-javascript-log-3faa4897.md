# @tauri-apps/plugin-log

## Enumerations

### LogLevel

#### Enumeration Members

##### Debug

```
Debug: 2;
```

The “debug” level.

Designates lower priority information.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L26>

##### Error

```
Error: 5;
```

The “error” level.

Designates very serious errors.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L44>

##### Info

```
Info: 3;
```

The “info” level.

Designates useful information.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L32>

##### Trace

```
Trace: 1;
```

The “trace” level.

Designates very low priority, often extremely verbose, information.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L20>

##### Warn

```
Warn: 4;
```

The “warn” level.

Designates hazardous situations.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L38>

## Interfaces

### LogOptions

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `file?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L9> |
|  `keyValues?` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `undefined` \| `string`\> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L11> |
|  `line?` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L10> |

## Functions

### attachConsole()

```
function attachConsole(): Promise<UnlistenFn>
```

Attaches a listener that writes log entries to the console as they come
in.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`UnlistenFn`\>

a function to cancel the listener.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L277>

------------------------------------------------------------------------

### attachLogger()

```
function attachLogger(fn): Promise<UnlistenFn>
```

Attaches a listener for the log, and calls the passed function for each
log entry.

#### Parameters

| Parameter | Type       | Description |
|-----------|------------|-------------|
| `fn`      | `LoggerFn` |             |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`UnlistenFn`\>

a function to cancel the listener.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L256>

------------------------------------------------------------------------

### debug()

```
function debug(message, options?): Promise<void>
```

Logs a message at the debug level.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | \# Examples `import { debug } from '@tauri-apps/plugin-log'; const pos = { x: 3.234, y: -1.223 }; debug(`New position: x: {pos.x}, y: {pos.y}`);` |
| `options`? | [`LogOptions`](https://v2.tauri.app/reference/javascript/log/#logoptions) | \- |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L214>

------------------------------------------------------------------------

### error()

```
function error(message, options?): Promise<void>
```

Logs a message at the error level.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | \# Examples `import { error } from '@tauri-apps/plugin-log'; const err_info = "No connection"; const port = 22; error(`Error: \${err_info} on port \${port}`);` |
| `options`? | [`LogOptions`](https://v2.tauri.app/reference/javascript/log/#logoptions) | \- |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L148>

------------------------------------------------------------------------

### info()

```
function info(message, options?): Promise<void>
```

Logs a message at the info level.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | \# Examples `import { info } from '@tauri-apps/plugin-log'; const conn_info = { port: 40, speed: 3.20 }; info(`Connected to port {conn_info.port} at {conn_info.speed} Mb/s`);` |
| `options`? | [`LogOptions`](https://v2.tauri.app/reference/javascript/log/#logoptions) | \- |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L192>

------------------------------------------------------------------------

### trace()

```
function trace(message, options?): Promise<void>
```

Logs a message at the trace level.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | \# Examples `import { trace } from '@tauri-apps/plugin-log'; let pos = { x: 3.234, y: -1.223 }; trace(`Position is: x: {pos.x}, y: {pos.y}`);` |
| `options`? | [`LogOptions`](https://v2.tauri.app/reference/javascript/log/#logoptions) | \- |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L236>

------------------------------------------------------------------------

### warn()

```
function warn(message, options?): Promise<void>
```

Logs a message at the warn level.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | \# Examples `import { warn } from '@tauri-apps/plugin-log'; const warn_description = "Invalid Input"; warn(`Warning! {warn_description}!`);` |
| `options`? | [`LogOptions`](https://v2.tauri.app/reference/javascript/log/#logoptions) | \- |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L170>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
