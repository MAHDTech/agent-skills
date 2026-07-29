+++
title = "reference-javascript-api-namespaceevent-85e9251b"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# event

The event system allows you to emit events to the backend and listen to
events from it.

This package is also accessible with `window.__TAURI__.event` when
[`app.withGlobalTauri`](https://v2.tauri.app/reference/config/#withglobaltauri)
in `tauri.conf.json` is set to `true`.

## Enumerations

### TauriEvent

#### Since

1.1.0

#### Enumeration Members

##### DRAG_DROP

```
DRAG_DROP: "tauri://drag-drop";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L73>

##### DRAG_ENTER

```
DRAG_ENTER: "tauri://drag-enter";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L71>

##### DRAG_LEAVE

```
DRAG_LEAVE: "tauri://drag-leave";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L74>

##### DRAG_OVER

```
DRAG_OVER: "tauri://drag-over";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L72>

##### WEBVIEW_CREATED

```
WEBVIEW_CREATED: "tauri://webview-created";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L70>

##### WINDOW_BLUR

```
WINDOW_BLUR: "tauri://blur";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L64>

##### WINDOW_CLOSE_REQUESTED

```
WINDOW_CLOSE_REQUESTED: "tauri://close-requested";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L61>

##### WINDOW_CREATED

```
WINDOW_CREATED: "tauri://window-created";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L67>

##### WINDOW_DESTROYED

```
WINDOW_DESTROYED: "tauri://destroyed";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L62>

##### WINDOW_FOCUS

```
WINDOW_FOCUS: "tauri://focus";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L63>

##### WINDOW_MOVED

```
WINDOW_MOVED: "tauri://move";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L60>

##### WINDOW_RESIZED

```
WINDOW_RESIZED: "tauri://resize";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L59>

##### WINDOW_RESUMED

```
WINDOW_RESUMED: "tauri://resumed";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L69>

##### WINDOW_SCALE_FACTOR_CHANGED

```
WINDOW_SCALE_FACTOR_CHANGED: "tauri://scale-change";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L65>

##### WINDOW_SUSPENDED

```
WINDOW_SUSPENDED: "tauri://suspended";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L68>

##### WINDOW_THEME_CHANGED

```
WINDOW_THEME_CHANGED: "tauri://theme-changed";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L66>

## Interfaces

### Event\<T\>

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `event` | [`EventName`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventname) | Event name | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L32> |
|  `id` | `number` | Event identifier used to unlisten | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L34> |
|  `payload` | `T` | Event payload | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L36> |

------------------------------------------------------------------------

### Options

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `target?` | `string` \| [`EventTarget`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget) | The event target to listen to, defaults to `{ kind: 'Any' }`, see [EventTarget](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget). If a string is provided, EventTarget.AnyLabel is used. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L52> |

## Type Aliases

### EventCallback()\<T\>

```
type EventCallback<T>: (event) => void;
```

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type |
|----|----|
| `event` | [`Event`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventt)\<`T`\> |

#### Returns

`void`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L39>

------------------------------------------------------------------------

### EventName

```
type EventName: `${TauriEvent}` | string & Record<never, never>;
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L44>

------------------------------------------------------------------------

### EventTarget

```
type EventTarget:  | object  | object  | object  | object  | object  | object;
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L22>

------------------------------------------------------------------------

### UnlistenFn()

```
type UnlistenFn: () => void;
```

#### Returns

`void`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L42>

## Functions

### emit()

```
function emit<T>(event, payload?): Promise<void>
```

Emits an event to all
[targets](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget).

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `event` | `string` | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `payload`? | `T` | Event payload. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { emit } from '@tauri-apps/api/event';await emit('frontend-loaded', { loggedIn: true, token: 'authToken' });
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L188>

------------------------------------------------------------------------

### emitTo()

```
function emitTo<T>(   target,   event,payload?): Promise<void>
```

Emits an event to all
[targets](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget)
matching the given target.

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `target` | `string` \| [`EventTarget`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget) | Label of the target Window/Webview/WebviewWindow or raw [EventTarget](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget) object. |
| `event` | `string` | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `payload`? | `T` | Event payload. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { emitTo } from '@tauri-apps/api/event';await emitTo('main', 'frontend-loaded', { loggedIn: true, token: 'authToken' });
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L210>

------------------------------------------------------------------------

### listen()

```
function listen<T>(   event,   handler,options?): Promise<UnlistenFn>
```

Listen to an emitted event to any
[target](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget).

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `event` | [`EventName`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventname) | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`T`\> | Event handler callback. |
| `options`? | [`Options`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#options) | Event listening options. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

#### Example

```
import { listen } from '@tauri-apps/api/event';const unlisten = await listen<string>('error', (event) => {  console.log(`Got error, payload: ${event.payload}`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L115>

------------------------------------------------------------------------

### once()

```
function once<T>(   event,   handler,options?): Promise<UnlistenFn>
```

Listens once to an emitted event to any
[target](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget).

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `event` | [`EventName`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventname) | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`T`\> | Event handler callback. |
| `options`? | [`Options`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#options) | Event listening options. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

#### Example

```
import { once } from '@tauri-apps/api/event';interface LoadedPayload {  loggedIn: boolean,  token: string}const unlisten = await once<LoadedPayload>('loaded', (event) => {  console.log(`App is loaded, loggedIn: ${event.payload.loggedIn}, token: ${event.payload.token}`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L159>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
