+++
title = "reference-javascript-api-namespacewebviewwindow-55018470"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# webviewWindow

## References

### Color

Re-exports [Color](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color)

### DragDropEvent

Re-exports
[DragDropEvent](https://v2.tauri.app/reference/javascript/api/namespacewebview/#dragdropevent)

## Classes

### WebviewWindow

Create new webview or get a handle to an existing one.

Webviews are identified by a *label* a unique identifier that can be
used to reference it later. It may only contain alphanumeric characters
`a-zA-Z` plus the following special characters `-`, `/`, `:` and `_`.

#### Example

```
import { Window } from "@tauri-apps/api/window"import { Webview } from "@tauri-apps/api/webview"
const appWindow = new Window('uniqueLabel');
appWindow.once('tauri://created', async function () {  // `new Webview` Should be called after the window is successfully created,  // or webview may not be attached to the window since window is not created yet.
  // loading embedded asset:  const webview = new Webview(appWindow, 'theUniqueLabel', {    url: 'path/to/page.html',
    // create a webview with specific logical position and size    x: 0,    y: 0,    width: 800,    height: 600,  });  // alternatively, load a remote URL:  const webview = new Webview(appWindow, 'theUniqueLabel', {    url: 'https://github.com/tauri-apps/tauri',
    // create a webview with specific logical position and size    x: 0,    y: 0,    width: 800,    height: 600,  });
  webview.once('tauri://created', function () {    // webview successfully created  });  webview.once('tauri://error', function (e) {    // an error happened creating the webview  });
  // emit an event to the backend  await webview.emit("some-event", "data");  // listen to an event from the backend  const unlisten = await webview.listen("event-name", e => { });  unlisten();});
```

#### Since

2.0.0

#### Extends

- [`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)

#### Constructors

##### new WebviewWindow()

```
new WebviewWindow(label, options): WebviewWindow
```

Creates a new
[Window](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) hosting a
[Webview](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `label` | `string` | The unique webview label. Must be alphanumeric: `a-zA-Z-/:_`. |
| `options` | [`Omit`](https://www.typescriptlang.org/docs/handbook/utility-types.html#omittype-keys)\<[`WebviewOptions`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webviewoptions), `"x"` \| `"y"` \| `"width"` \| `"height"`\> & [`WindowOptions`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#windowoptions) | \- |

###### Returns

[`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

The
[WebviewWindow](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)
instance to communicate with the window and webview.

###### Example

```
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'const webview = new WebviewWindow('my-label', {  url: 'https://github.com/tauri-apps/tauri'});webview.once('tauri://created', function () { // webview successfully created});webview.once('tauri://error', function (e) { // an error happened creating the webview});
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`constructor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#constructors-1)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L75>

#### Properties

| Property | Type | Description | Inherited from | Defined in |
|----|----|----|----|----|
|  `label` | `string` | The webview label. It is a unique identifier for the webview, can be used to reference it later. | [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`label`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#label) | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L51> |
|  `listeners` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`any`\>\[\]\> | Local event listeners. | [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`listeners`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#listeners) | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L54> |
|  `window` | [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) | The window hosting this webview. | [`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`window`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#window) | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L157> |

#### Methods

##### activityName()

```
activityName(): Promise<string>
```

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`activityName`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#activityname)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L853>

##### center()

```
center(): Promise<void>
```

Centers the window.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().center();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`center`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#center)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L877>

##### clearAllBrowsingData()

```
clearAllBrowsingData(): Promise<void>
```

Clears all browsing data for this webview.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().clearAllBrowsingData();
```

###### Inherited from

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`clearAllBrowsingData`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#clearallbrowsingdata)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L589>

##### clearEffects()

```
clearEffects(): Promise<void>
```

Clear any applied effects if possible.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`clearEffects`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#cleareffects)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1265>

##### close()

```
close(): Promise<void>
```

Closes the webview.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().close();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`close`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#close)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L436>

##### destroy()

```
destroy(): Promise<void>
```

Destroys the window. Behaves like
[Window.close](https://v2.tauri.app/reference/javascript/api/namespacewindow/#close) but
forces the window close instead of emitting a closeRequested event.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().destroy();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`destroy`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#destroy)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1202>

##### emit()

```
emit<T>(event, payload?): Promise<void>
```

Emits an event to all
[targets](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget).

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `event` | `string` | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `payload`? | `T` | Event payload. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().emit('webview-loaded', { loggedIn: true, token: 'authToken' });
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`emit`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#emit)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L325>

##### emitTo()

```
emitTo<T>(   target,   event,payload?): Promise<void>
```

Emits an event to all
[targets](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget)
matching the given target.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `target` | `string` \| [`EventTarget`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget) | Label of the target Window/Webview/WebviewWindow or raw [EventTarget](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventtarget) object. |
| `event` | `string` | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `payload`? | `T` | Event payload. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().emitTo('main', 'webview-loaded', { loggedIn: true, token: 'authToken' });
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`emitTo`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#emitto)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L353>

##### hide()

```
hide(): Promise<void>
```

Hide the webview.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().hide();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`hide`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#hide)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L523>

##### innerPosition()

```
innerPosition(): Promise<PhysicalPosition>
```

The position of the top-left hand corner of the window’s client area
relative to the top-left hand corner of the desktop.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)\>

The window’s inner position.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const position = await getCurrentWindow().innerPosition();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`innerPosition`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#innerposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L567>

##### innerSize()

```
innerSize(): Promise<PhysicalSize>
```

The physical size of the window’s client area. The client area is the
content of the window, excluding the title bar and borders.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)\>

The window’s inner size.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const size = await getCurrentWindow().innerSize();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`innerSize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#innersize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L600>

##### isAlwaysOnTop()

```
isAlwaysOnTop(): Promise<boolean>
```

Whether the window is configured to be always on top of other windows or
not.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window is visible or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const alwaysOnTop = await getCurrentWindow().isAlwaysOnTop();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isAlwaysOnTop`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isalwaysontop)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L847>

##### isClosable()

```
isClosable(): Promise<boolean>
```

Gets the window’s native close button state.

Platform-specific

- **iOS / Android:** Unsupported.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window’s native close button is enabled or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const closable = await getCurrentWindow().isClosable();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isClosable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isclosable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L780>

##### isDecorated()

```
isDecorated(): Promise<boolean>
```

Gets the window’s current decorated state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window is decorated or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const decorated = await getCurrentWindow().isDecorated();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isDecorated`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isdecorated)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L701>

##### isEnabled()

```
isEnabled(): Promise<boolean>
```

Whether the window is enabled or disabled.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setEnabled(false);
```

###### Since

2.0.0

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isEnabled`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isenabled)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L969>

##### isFocused()

```
isFocused(): Promise<boolean>
```

Gets the window’s current focus state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window is focused or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const focused = await getCurrentWindow().isFocused();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isFocused`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isfocused)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L685>

##### isFullscreen()

```
isFullscreen(): Promise<boolean>
```

Gets the window’s current fullscreen state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window is in fullscreen mode or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const fullscreen = await getCurrentWindow().isFullscreen();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isFullscreen`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isfullscreen)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L639>

##### isMaximizable()

```
isMaximizable(): Promise<boolean>
```

Gets the window’s native maximize button state.

Platform-specific

- **Linux / iOS / Android:** Unsupported.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window’s native maximize button is enabled or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const maximizable = await getCurrentWindow().isMaximizable();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isMaximizable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#ismaximizable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L738>

##### isMaximized()

```
isMaximized(): Promise<boolean>
```

Gets the window’s current maximized state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window is maximized or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const maximized = await getCurrentWindow().isMaximized();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isMaximized`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#ismaximized)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L669>

##### isMinimizable()

```
isMinimizable(): Promise<boolean>
```

Gets the window’s native minimize button state.

Platform-specific

- **Linux / iOS / Android:** Unsupported.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window’s native minimize button is enabled or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const minimizable = await getCurrentWindow().isMinimizable();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isMinimizable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isminimizable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L759>

##### isMinimized()

```
isMinimized(): Promise<boolean>
```

Gets the window’s current minimized state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const minimized = await getCurrentWindow().isMinimized();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isMinimized`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isminimized)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L653>

##### isResizable()

```
isResizable(): Promise<boolean>
```

Gets the window’s current resizable state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window is resizable or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const resizable = await getCurrentWindow().isResizable();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isResizable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isresizable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L717>

##### isVisible()

```
isVisible(): Promise<boolean>
```

Gets the window’s current visible state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

Whether the window is visible or not.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const visible = await getCurrentWindow().isVisible();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`isVisible`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#isvisible)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L796>

##### listen()

```
listen<T>(event, handler): Promise<UnlistenFn>
```

Listen to an emitted event on this webview window.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `event` | [`EventName`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventname) | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`T`\> | Event handler. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';const unlisten = await WebviewWindow.getCurrent().listen<string>('state-changed', (event) => {  console.log(`Got error: ${payload}`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`listen`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#listen)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L155>

##### maximize()

```
maximize(): Promise<void>
```

Maximizes the window.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().maximize();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`maximize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#maximize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1072>

##### minimize()

```
minimize(): Promise<void>
```

Minimizes the window.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().minimize();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`minimize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#minimize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1120>

##### onCloseRequested()

```
onCloseRequested(handler): Promise<UnlistenFn>
```

Listen to window close requested. Emitted when the user requests to
closes the window.

###### Parameters

| Parameter | Type |
|----|----|
| `handler` | (`event`) =\> `void` \| [`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\> |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { getCurrentWindow } from "@tauri-apps/api/window";import { confirm } from '@tauri-apps/api/dialog';const unlisten = await getCurrentWindow().onCloseRequested(async (event) => {  const confirmed = await confirm('Are you sure?');  if (!confirmed) {    // user did not confirm closing the window; let's prevent it    event.preventDefault();  }});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`onCloseRequested`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#oncloserequested)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1927>

##### onDragDropEvent()

```
onDragDropEvent(handler): Promise<UnlistenFn>
```

Listen to a file drop event. The listener is triggered when the user
hovers the selected files on the webview, drops the files or cancels the
operation.

###### Parameters

| Parameter | Type |
|----|----|
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<[`DragDropEvent`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#dragdropevent)\> |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { getCurrentWebview } from "@tauri-apps/api/webview";const unlisten = await getCurrentWebview().onDragDropEvent((event) => { if (event.payload.type === 'over') {   console.log('User hovering', event.payload.position); } else if (event.payload.type === 'drop') {   console.log('User dropped', event.payload.paths); } else {   console.log('File drop cancelled'); }});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

When the debugger panel is open, the drop position of this event may be
inaccurate due to a known limitation. To retrieve the correct drop
position, please detach the debugger.

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`onDragDropEvent`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#ondragdropevent)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L641>

##### onFocusChanged()

```
onFocusChanged(handler): Promise<UnlistenFn>
```

Listen to window focus change.

###### Parameters

| Parameter | Type |
|----|----|
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`boolean`\> |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { getCurrentWindow } from "@tauri-apps/api/window";const unlisten = await getCurrentWindow().onFocusChanged(({ payload: focused }) => { console.log('Focus changed, window is focused? ' + focused);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`onFocusChanged`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#onfocuschanged)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2043>

##### onMoved()

```
onMoved(handler): Promise<UnlistenFn>
```

Listen to window move.

###### Parameters

| Parameter | Type |
|----|----|
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)\> |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { getCurrentWindow } from "@tauri-apps/api/window";const unlisten = await getCurrentWindow().onMoved(({ payload: position }) => { console.log('Window moved', position);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`onMoved`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#onmoved)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1898>

##### onResized()

```
onResized(handler): Promise<UnlistenFn>
```

Listen to window resize.

###### Parameters

| Parameter | Type |
|----|----|
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)\> |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { getCurrentWindow } from "@tauri-apps/api/window";const unlisten = await getCurrentWindow().onResized(({ payload: size }) => { console.log('Window resized', size);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`onResized`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#onresized)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1874>

##### onScaleChanged()

```
onScaleChanged(handler): Promise<UnlistenFn>
```

Listen to window scale change. Emitted when the window’s scale factor
has changed. The following user actions can cause DPI changes:

- Changing the display’s resolution.
- Changing the display’s scale factor (e.g. in Control Panel on
  Windows).
- Moving the window to a display with a different scale factor.

###### Parameters

| Parameter | Type |
|----|----|
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<[`ScaleFactorChanged`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#scalefactorchanged)\> |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { getCurrentWindow } from "@tauri-apps/api/window";const unlisten = await getCurrentWindow().onScaleChanged(({ payload }) => { console.log('Scale changed', payload.scaleFactor, payload.size);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`onScaleChanged`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#onscalechanged)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2083>

##### onThemeChanged()

```
onThemeChanged(handler): Promise<UnlistenFn>
```

Listen to the system theme change.

###### Parameters

| Parameter | Type |
|----|----|
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<[`Theme`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#theme-2)\> |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { getCurrentWindow } from "@tauri-apps/api/window";const unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => { console.log('New theme: ' + theme);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`onThemeChanged`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#onthemechanged)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2109>

##### once()

```
once<T>(event, handler): Promise<UnlistenFn>
```

Listen to an emitted event on this webview window only once.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `event` | [`EventName`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventname) | Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`. |
| `handler` | [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`T`\> | Event handler. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`UnlistenFn`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that
removing the listener is required if your listener goes out of scope
e.g. the component is unmounted.

###### Example

```
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';const unlisten = await WebviewWindow.getCurrent().once<null>('initialized', (event) => {  console.log(`Webview initialized!`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`once`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#once)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L190>

##### outerPosition()

```
outerPosition(): Promise<PhysicalPosition>
```

The position of the top-left hand corner of the window relative to the
top-left hand corner of the desktop.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)\>

The window’s outer position.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const position = await getCurrentWindow().outerPosition();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`outerPosition`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#outerposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L583>

##### outerSize()

```
outerSize(): Promise<PhysicalSize>
```

The physical size of the entire window. These dimensions include the
title bar and borders. If you don’t want that (and you usually don’t),
use inner_size instead.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)\>

The window’s outer size.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const size = await getCurrentWindow().outerSize();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`outerSize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#outersize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L620>

##### position()

```
position(): Promise<PhysicalPosition>
```

The position of the top-left hand corner of the webview’s client area
relative to the top-left hand corner of the desktop.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)\>

The webview’s position.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';const position = await getCurrentWebview().position();
```

###### Inherited from

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`position`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#position)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L398>

##### reparent()

```
reparent(window): Promise<void>
```

Moves this webview to the given label.

###### Parameters

| Parameter | Type |
|----|----|
| `window` | `string` \| [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) \| [`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().reparent('other-window');
```

###### Inherited from

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`reparent`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#reparent)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L572>

##### requestUserAttention()

```
requestUserAttention(requestType): Promise<void>
```

Requests user attention to the window, this has no effect if the
application is already focused. How requesting for user attention
manifests is platform dependent, see `UserAttentionType` for details.

Providing `null` will unset the request for user attention. Unsetting
the request for user attention might not be done automatically by the WM
when the window receives input.

Platform-specific

- **macOS:** `null` has no effect.
- **Linux:** Urgency levels have the same effect.

###### Parameters

| Parameter | Type |
|----|----|
| `requestType` | `null` \| [`UserAttentionType`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#userattentiontype) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().requestUserAttention();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`requestUserAttention`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#requestuserattention)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L903>

##### scaleFactor()

```
scaleFactor(): Promise<number>
```

The scale factor that can be used to map physical pixels to logical
pixels.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`number`\>

The window’s monitor scale factor.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const factor = await getCurrentWindow().scaleFactor();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`scaleFactor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#scalefactor)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L551>

##### sceneIdentifier()

```
sceneIdentifier(): Promise<string>
```

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`sceneIdentifier`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#sceneidentifier)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L859>

##### setAlwaysOnBottom()

```
setAlwaysOnBottom(alwaysOnBottom): Promise<void>
```

Whether the window should always be below other windows.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `alwaysOnBottom` | `boolean` | Whether the window should always be below other windows or not. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setAlwaysOnBottom(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setAlwaysOnBottom`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setalwaysonbottom)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1301>

##### setAlwaysOnTop()

```
setAlwaysOnTop(alwaysOnTop): Promise<void>
```

Whether the window should always be on top of other windows.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `alwaysOnTop` | `boolean` | Whether the window should always be on top of other windows or not. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setAlwaysOnTop(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setAlwaysOnTop`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setalwaysontop)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1283>

##### setAutoResize()

```
setAutoResize(autoResize): Promise<void>
```

Sets whether the webview should automatically grow and shrink its size
and position when the parent window resizes.

###### Parameters

| Parameter    | Type      |
|--------------|-----------|
| `autoResize` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().setAutoResize(true);
```

###### Inherited from

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`setAutoResize`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#setautoresize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L506>

##### setBackgroundColor()

```
setBackgroundColor(color): Promise<void>
```

Set the window and webview background color.

Platform-specific:

- **Android / iOS:** Unsupported for the window layer.
- **macOS / iOS**: Not implemented for the webview layer.
- **Windows**:
  - alpha channel is ignored for the window layer.
  - On Windows 7, alpha channel is ignored for the webview layer.
  - On Windows 8 and newer, if alpha channel is not `0`, it will be
    ignored.

###### Parameters

| Parameter | Type                                                         |
|-----------|--------------------------------------------------------------|
| `color`   | [`Color`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Since

2.1.0

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setBackgroundColor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setbackgroundcolor)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L222>

##### setBadgeCount()

```
setBadgeCount(count?): Promise<void>
```

Sets the badge count. It is app wide and not specific to this window.

Platform-specific

- **Windows**: Unsupported. Use @{linkcode Window.setOverlayIcon}
  instead.

###### Parameters

| Parameter | Type     | Description                                           |
|-----------|----------|-------------------------------------------------------|
| `count`?  | `number` | The badge count. Use `undefined` to remove the badge. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setBadgeCount(5);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setBadgeCount`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setbadgecount)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1727>

##### setBadgeLabel()

```
setBadgeLabel(label?): Promise<void>
```

Sets the badge cont **macOS only**.

###### Parameters

| Parameter | Type     | Description                                           |
|-----------|----------|-------------------------------------------------------|
| `label`?  | `string` | The badge label. Use `undefined` to remove the badge. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setBadgeLabel("Hello");
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setBadgeLabel`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setbadgelabel)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1746>

##### setClosable()

```
setClosable(closable): Promise<void>
```

Sets whether the window’s native close button is enabled or not.

Platform-specific

- **Linux:** GTK+ will do its best to convince the window manager not to
  show a close button. Depending on the system, this function may not
  have any effect when called on a window that is already visible
- **iOS / Android:** Unsupported.

###### Parameters

| Parameter  | Type      |
|------------|-----------|
| `closable` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setClosable(false);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setClosable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setclosable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1037>

##### setContentProtected()

```
setContentProtected(protected_): Promise<void>
```

Prevents the window contents from being captured by other apps.

###### Parameters

| Parameter    | Type      |
|--------------|-----------|
| `protected_` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setContentProtected(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setContentProtected`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setcontentprotected)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1318>

##### setCursorGrab()

```
setCursorGrab(grab): Promise<void>
```

Grabs the cursor, preventing it from leaving the window.

There’s no guarantee that the cursor will be hidden. You should hide it
by yourself if you want so.

Platform-specific

- **Linux:** Unsupported.
- **macOS:** This locks the cursor in a fixed location, which looks
  visually awkward.

###### Parameters

| Parameter | Type      | Description                                            |
|-----------|-----------|--------------------------------------------------------|
| `grab`    | `boolean` | `true` to grab the cursor icon, `false` to release it. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setCursorGrab(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setCursorGrab`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setcursorgrab)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1574>

##### setCursorIcon()

```
setCursorIcon(icon): Promise<void>
```

Modifies the cursor icon of the window.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `icon` | [`CursorIcon`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#cursoricon) | The new cursor icon. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setCursorIcon('help');
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setCursorIcon`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setcursoricon)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1616>

##### setCursorPosition()

```
setCursorPosition(position): Promise<void>
```

Changes the position of the cursor in window coordinates.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `position` | [`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition) \| [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) \| [`Position`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#position) | The new cursor position. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';await getCurrentWindow().setCursorPosition(new LogicalPosition(600, 300));
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setCursorPosition`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setcursorposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1650>

##### setCursorVisible()

```
setCursorVisible(visible): Promise<void>
```

Modifies the cursor’s visibility.

Platform-specific

- **Windows:** The cursor is only hidden within the confines of the
  window.
- **macOS:** The cursor is hidden as long as the window has input focus,
  even if the cursor is outside of the window.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `visible` | `boolean` | If `false`, this will hide the cursor. If `true`, this will show the cursor. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setCursorVisible(false);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setCursorVisible`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setcursorvisible)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1598>

##### setDecorations()

```
setDecorations(decorations): Promise<void>
```

Whether the window should have borders and bars.

###### Parameters

| Parameter     | Type      | Description                                      |
|---------------|-----------|--------------------------------------------------|
| `decorations` | `boolean` | Whether the window should have borders and bars. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setDecorations(false);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setDecorations`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setdecorations)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1219>

##### setEffects()

```
setEffects(effects): Promise<void>
```

Set window effects.

###### Parameters

| Parameter | Type                                                            |
|-----------|-----------------------------------------------------------------|
| `effects` | [`Effects`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#effects) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setEffects`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#seteffects)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1255>

##### setEnabled()

```
setEnabled(enabled): Promise<void>
```

Enable or disable the window.

###### Parameters

| Parameter | Type      |
|-----------|-----------|
| `enabled` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setEnabled(false);
```

###### Since

2.0.0

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setEnabled`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setenabled)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L950>

##### setFocus()

```
setFocus(): Promise<void>
```

Bring the webview to front and focus.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().setFocus();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setFocus`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setfocus)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L490>

##### setFocusable()

```
setFocusable(focusable): Promise<void>
```

Sets whether the window can be focused.

Platform-specific

- **macOS**: If the window is already focused, it is not possible to
  unfocus it after calling `set_focusable(false)`. In this case, you
  might consider calling
  [Window.setFocus](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setfocus)
  but it will move the window to the back i.e. at the bottom in terms of
  z-order.

###### Parameters

| Parameter   | Type      | Description                        |
|-------------|-----------|------------------------------------|
| `focusable` | `boolean` | Whether the window can be focused. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setFocusable(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setFocusable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setfocusable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1499>

##### setFullscreen()

```
setFullscreen(fullscreen): Promise<void>
```

Sets the window fullscreen state.

###### Parameters

| Parameter    | Type      | Description                                        |
|--------------|-----------|----------------------------------------------------|
| `fullscreen` | `boolean` | Whether the window should go to fullscreen or not. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setFullscreen(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setFullscreen`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setfullscreen)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1443>

##### setIcon()

```
setIcon(icon): Promise<void>
```

Sets the window icon.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `icon` | \| `string` \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) \| [`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image) | Icon bytes or path to the icon file. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setIcon('/tauri/awesome.png');
```

Note that you may need the `image-ico` or `image-png` Cargo features to
use this API. To enable it, change your Cargo.toml file:

```
[dependencies]tauri = { version = "...", features = ["...", "image-png"] }
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setIcon`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#seticon)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1524>

##### setIgnoreCursorEvents()

```
setIgnoreCursorEvents(ignore): Promise<void>
```

Changes the cursor events behavior.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `ignore` | `boolean` | `true` to ignore the cursor events; `false` to process them as usual. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setIgnoreCursorEvents(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setIgnoreCursorEvents`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setignorecursorevents)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1671>

##### setMaxSize()

```
setMaxSize(size): Promise<void>
```

Sets the window maximum inner size. If the `size` argument is undefined,
the constraint is unset.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `size` | \| `undefined` \| `null` \| [`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) \| [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) \| [`Size`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#size) | The logical or physical inner size, or `null` to unset the constraint. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';await getCurrentWindow().setMaxSize(new LogicalSize(600, 500));
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setMaxSize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setmaxsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1374>

##### setMaximizable()

```
setMaximizable(maximizable): Promise<void>
```

Sets whether the window’s native maximize button is enabled or not. If
resizable is set to false, this setting is ignored.

Platform-specific

- **macOS:** Disables the “zoom” button in the window titlebar, which is
  also used to enter fullscreen mode.
- **Linux / iOS / Android:** Unsupported.

###### Parameters

| Parameter     | Type      |
|---------------|-----------|
| `maximizable` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setMaximizable(false);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setMaximizable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setmaximizable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L992>

##### setMinSize()

```
setMinSize(size): Promise<void>
```

Sets the window minimum inner size. If the `size` argument is not
provided, the constraint is unset.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `size` | \| `undefined` \| `null` \| [`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) \| [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) \| [`Size`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#size) | The logical or physical inner size, or `null` to unset the constraint. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window';await getCurrentWindow().setMinSize(new PhysicalSize(600, 500));
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setMinSize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setminsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1354>

##### setMinimizable()

```
setMinimizable(minimizable): Promise<void>
```

Sets whether the window’s native minimize button is enabled or not.

Platform-specific

- **Linux / iOS / Android:** Unsupported.

###### Parameters

| Parameter     | Type      |
|---------------|-----------|
| `minimizable` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setMinimizable(false);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setMinimizable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setminimizable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1014>

##### setOverlayIcon()

```
setOverlayIcon(icon?): Promise<void>
```

Sets the overlay icon. **Windows only** The overlay icon can be set for
every window.

Note that you may need the `image-ico` or `image-png` Cargo features to
use this API. To enable it, change your Cargo.toml file:

```
[dependencies]tauri = { version = "...", features = ["...", "image-png"] }
```

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `icon`? | \| `string` \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) \| [`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image) | Icon bytes or path to the icon file. Use `undefined` to remove the overlay icon. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setOverlayIcon("/tauri/awesome.png");
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setOverlayIcon`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setoverlayicon)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1775>

##### setPosition()

```
setPosition(position): Promise<void>
```

Sets the webview position.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `position` | [`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition) \| [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) \| [`Position`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#position) | The new position, in logical or physical pixels. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrent, LogicalPosition } from '@tauri-apps/api/webview';await getCurrentWebview().setPosition(new LogicalPosition(600, 500));
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setPosition`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L471>

##### setProgressBar()

```
setProgressBar(state): Promise<void>
```

Sets the taskbar progress state.

Platform-specific

- **Linux / macOS**: Progress bar is app-wide and not specific to this
  window.
- **Linux**: Only supported desktop environments with `libunity` (e.g.
  GNOME).

###### Parameters

| Parameter | Type |
|----|----|
| `state` | [`ProgressBarState`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#progressbarstate) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow, ProgressBarStatus } from '@tauri-apps/api/window';await getCurrentWindow().setProgressBar({  status: ProgressBarStatus.Normal,  progress: 50,});
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setProgressBar`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setprogressbar)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1803>

##### setResizable()

```
setResizable(resizable): Promise<void>
```

Updates the window resizable flag.

###### Parameters

| Parameter   | Type      |
|-------------|-----------|
| `resizable` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setResizable(false);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setResizable`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setresizable)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L931>

##### setShadow()

```
setShadow(enable): Promise<void>
```

Whether or not the window should have shadow.

Platform-specific

- **Windows:**
  - `false` has no effect on decorated window, shadows are always ON.
  - `true` will make undecorated window have a 1px white border, and on
    Windows 11, it will have a rounded corners.
- **Linux:** Unsupported.

###### Parameters

| Parameter | Type      |
|-----------|-----------|
| `enable`  | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setShadow(false);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setShadow`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setshadow)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1245>

##### setSimpleFullscreen()

```
setSimpleFullscreen(fullscreen): Promise<void>
```

On macOS, Toggles a fullscreen mode that doesn’t require a new macOS
space. Returns a boolean indicating whether the transition was
successful (this won’t work if the window was already in the native
fullscreen). This is how fullscreen used to work on macOS in versions
before Lion. And allows the user to have a fullscreen window without
using another space or taking control over the entire monitor.

On other platforms, this is the same as
[Window.setFullscreen](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setfullscreen).

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `fullscreen` | `boolean` | Whether the window should go to simple fullscreen or not. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setSimpleFullscreen`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setsimplefullscreen)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1459>

##### setSize()

```
setSize(size): Promise<void>
```

Resizes the webview.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `size` | [`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) \| [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) \| [`Size`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#size) | The logical or physical size. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrent, LogicalSize } from '@tauri-apps/api/webview';await getCurrentWebview().setSize(new LogicalSize(600, 500));
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setSize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L453>

##### setSizeConstraints()

```
setSizeConstraints(constraints): Promise<void>
```

Sets the window inner size constraints.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `constraints` | `undefined` \| `null` \| [`WindowSizeConstraints`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#windowsizeconstraints) | The logical or physical inner size, or `null` to unset the constraint. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setSizeConstraints({ minWidth: 300 });
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setSizeConstraints`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setsizeconstraints)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1394>

##### setSkipTaskbar()

```
setSkipTaskbar(skip): Promise<void>
```

Whether the window icon should be hidden from the taskbar or not.

Platform-specific

- **macOS:** Unsupported.

###### Parameters

| Parameter | Type      | Description                                 |
|-----------|-----------|---------------------------------------------|
| `skip`    | `boolean` | true to hide window icon, false to show it. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setSkipTaskbar(true);
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setSkipTaskbar`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setskiptaskbar)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1548>

##### setTheme()

```
setTheme(theme?): Promise<void>
```

Set window theme, pass in `null` or `undefined` to follow system theme

Platform-specific

- **Linux / macOS**: Theme is app-wide and not specific to this window.
- **iOS / Android:** Unsupported.

###### Parameters

| Parameter | Type |
|----|----|
| `theme`? | `null` \| [`Theme`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#theme-2) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Since

2.0.0

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setTheme`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#settheme)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1848>

##### setTitle()

```
setTitle(title): Promise<void>
```

Sets the window title.

###### Parameters

| Parameter | Type     | Description   |
|-----------|----------|---------------|
| `title`   | `string` | The new title |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setTitle('Tauri');
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setTitle`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#settitle)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1055>

##### setTitleBarStyle()

```
setTitleBarStyle(style): Promise<void>
```

Sets the title bar style. **macOS only**.

###### Parameters

| Parameter | Type |
|----|----|
| `style` | [`TitleBarStyle`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#titlebarstyle-1) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Since

2.0.0

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setTitleBarStyle`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#settitlebarstyle)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1831>

##### setVisibleOnAllWorkspaces()

```
setVisibleOnAllWorkspaces(visible): Promise<void>
```

Sets whether the window should be visible on all workspaces or virtual
desktops.

Platform-specific

- **Windows / iOS / Android:** Unsupported.

###### Parameters

| Parameter | Type      |
|-----------|-----------|
| `visible` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Since

2.0.0

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`setVisibleOnAllWorkspaces`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#setvisibleonallworkspaces)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1819>

##### setZoom()

```
setZoom(scaleFactor): Promise<void>
```

Set webview zoom level.

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().setZoom(1.5);
```

###### Inherited from

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`setZoom`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#setzoom)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L555>

##### show()

```
show(): Promise<void>
```

Show the webview.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';await getCurrentWebview().show();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`show`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#show)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L539>

##### size()

```
size(): Promise<PhysicalSize>
```

The physical size of the webview’s client area. The client area is the
content of the webview, excluding the title bar and borders.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)\>

The webview’s size.

###### Example

```
import { getCurrentWebview } from '@tauri-apps/api/webview';const size = await getCurrentWebview().size();
```

###### Inherited from

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview).[`size`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#size)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L415>

##### startDragging()

```
startDragging(): Promise<void>
```

Starts dragging the window.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().startDragging();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`startDragging`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#startdragging)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1688>

##### startResizeDragging()

```
startResizeDragging(direction): Promise<void>
```

Starts resize-dragging the window.

###### Parameters

| Parameter   | Type              |
|-------------|-------------------|
| `direction` | `ResizeDirection` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().startResizeDragging();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`startResizeDragging`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#startresizedragging)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1704>

##### theme()

```
theme(): Promise<null | Theme>
```

Gets the window’s current theme.

Platform-specific

- **macOS:** Theme was introduced on macOS 10.14. Returns `light` on
  macOS 10.13 and below.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`
\| [`Theme`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#theme-2)\>

The window theme.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const theme = await getCurrentWindow().theme();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`theme`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#theme)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L831>

##### title()

```
title(): Promise<string>
```

Gets the window’s current title.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const title = await getCurrentWindow().title();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`title`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#title)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L810>

##### toggleMaximize()

```
toggleMaximize(): Promise<void>
```

Toggles the window maximized state.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().toggleMaximize();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`toggleMaximize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#togglemaximize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1104>

##### unmaximize()

```
unmaximize(): Promise<void>
```

Unmaximizes the window.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().unmaximize();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`unmaximize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#unmaximize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1088>

##### unminimize()

```
unminimize(): Promise<void>
```

Unminimizes the window.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().unminimize();
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`unminimize`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#unminimize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1136>

##### getAll()

```
static getAll(): Promise<WebviewWindow[]>
```

Gets a list of instances of `Webview` for all available webviews.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)\[\]\>

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`getAll`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#getall)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L132>

##### getByLabel()

```
static getByLabel(label): Promise<null | WebviewWindow>
```

Gets the Webview for the webview associated with the given label.

###### Parameters

| Parameter | Type     | Description        |
|-----------|----------|--------------------|
| `label`   | `string` | The webview label. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`
\|
[`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)\>

The Webview instance to communicate with the webview or null if the
webview doesn’t exist.

###### Example

```
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';const mainWebview = WebviewWindow.getByLabel('main');
```

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`getByLabel`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#getbylabel)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L112>

##### getCurrent()

```
static getCurrent(): WebviewWindow
```

Get an instance of `Webview` for the current webview.

###### Returns

[`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

###### Inherited from

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window).[`getCurrent`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#getcurrent)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L125>

## Functions

### getAllWebviewWindows()

```
function getAllWebviewWindows(): Promise<WebviewWindow[]>
```

Gets a list of instances of `Webview` for all available webview windows.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)\[\]\>

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L34>

------------------------------------------------------------------------

### getCurrentWebviewWindow()

```
function getCurrentWebviewWindow(): WebviewWindow
```

Get an instance of `Webview` for the current webview window.

#### Returns

[`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L23>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
