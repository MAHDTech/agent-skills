+++
title = "reference-javascript-api-namespacewebview-ebd29705"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# webview

Provides APIs to create webviews, communicate with other webviews and
manipulate the current webview.

#### Webview events

Events can be listened to using
[Webview.listen](https://v2.tauri.app/reference/javascript/api/namespacewebview/#listen):

```
import { getCurrentWebview } from "@tauri-apps/api/webview";getCurrentWebview().listen("my-webview-event", ({ event, payload }) => { });
```

## Classes

### Webview

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

#### Extended by

- [`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

#### Constructors

##### new Webview()

```
new Webview(   window,   label,   options): Webview
```

Creates a new Webview.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `window` | [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) | the window to add this webview to. |
| `label` | `string` | The unique webview label. Must be alphanumeric: `a-zA-Z-/:_`. |
| `options` | [`WebviewOptions`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webviewoptions) | \- |

###### Returns

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview)

The [Webview](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview)
instance to communicate with the webview.

###### Example

```
import { Window } from '@tauri-apps/api/window'import { Webview } from '@tauri-apps/api/webview'const appWindow = new Window('my-label')
appWindow.once('tauri://created', async function() {  const webview = new Webview(appWindow, 'my-label', {    url: 'https://github.com/tauri-apps/tauri',
    // create a webview with specific logical position and size    x: 0,    y: 0,    width: 800,    height: 600,  });
  webview.once('tauri://created', function () {    // webview successfully created  });  webview.once('tauri://error', function (e) {    // an error happened creating the webview  });});
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L194>

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `label` | `string` | The webview label. It is a unique identifier for the webview, can be used to reference it later. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L155> |
|  `listeners` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`any`\>\[\]\> | Local event listeners. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L160> |
|  `window` | [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) | The window hosting this webview. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L157> |

#### Methods

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L589>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L436>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L523>

##### listen()

```
listen<T>(event, handler): Promise<UnlistenFn>
```

Listen to an emitted event on this webview.

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
import { getCurrentWebview } from '@tauri-apps/api/webview';const unlisten = await getCurrentWebview().listen<string>('state-changed', (event) => {  console.log(`Got error: ${payload}`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L262>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L641>

##### once()

```
once<T>(event, handler): Promise<UnlistenFn>
```

Listen to an emitted event on this webview only once.

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
import { getCurrentWebview } from '@tauri-apps/api/webview';const unlisten = await getCurrent().once<null>('initialized', (event) => {  console.log(`Webview initialized!`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L297>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L572>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L506>

##### setBackgroundColor()

```
setBackgroundColor(color): Promise<void>
```

Specify the webview background color.

Platform-specific:

- **macOS / iOS**: Not implemented.
- **Windows**:
  - On Windows 7, transparency is not supported and the alpha value will
    be ignored.
  - On Windows higher than 7: translucent colors are not supported so
    any alpha value other than `0` will be replaced by `255`

###### Parameters

| Parameter | Type |
|----|----|
| `color` | `null` \| [`Color`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Since

2.1.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L607>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L490>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L471>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L453>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L415>

##### getAll()

```
static getAll(): Promise<Webview[]>
```

Gets a list of instances of `Webview` for all available webviews.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview)\[\]\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L239>

##### getByLabel()

```
static getByLabel(label): Promise<null | Webview>
```

Gets the Webview for the webview associated with the given label.

###### Parameters

| Parameter | Type     | Description        |
|-----------|----------|--------------------|
| `label`   | `string` | The webview label. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`
\| [`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview)\>

The Webview instance to communicate with the webview or null if the
webview doesn’t exist.

###### Example

```
import { Webview } from '@tauri-apps/api/webview';const mainWebview = Webview.getByLabel('main');
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L225>

##### getCurrent()

```
static getCurrent(): Webview
```

Get an instance of `Webview` for the current webview.

###### Returns

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L232>

## Interfaces

### WebviewOptions

Configuration for the webview to create.

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `acceptFirstMouse?` | `boolean` | Whether clicking an inactive webview also clicks through to the webview on macOS. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L746> |
|  `allowLinkPreview?` | `boolean` | on macOS and iOS there is a link preview on long pressing links, this is enabled by default. see <https://docs.rs/objc2-web-kit/latest/objc2_web_kit/struct.WKWebView.html#method.allowsLinkPreview> | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L850> |
|  `backgroundColor?` | [`Color`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color) | Set the window and webview background color. Platform-specific: - **macOS / iOS**: Not implemented. - **Windows**: - On Windows 7, alpha channel is ignored. - On Windows 8 and newer, if alpha channel is not `0`, it will be ignored. **Since** 2.1.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L822> |
|  `backgroundThrottling?` | [`BackgroundThrottlingPolicy`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#backgroundthrottlingpolicy) | Change the default background throttling behaviour. By default, browsers use a suspend policy that will throttle timers and even unload the whole tab (view) to free resources after roughly 5 minutes when a view became minimized or hidden. This will pause all tasks until the documents visibility state changes back from hidden to visible by bringing the view back to the foreground. \## Platform-specific - **Linux / Windows / Android**: Unsupported. Workarounds like a pending WebLock transaction might suffice. - **iOS**: Supported since version 17.0+. - **macOS**: Supported since version 14.0+. see <https://github.com/tauri-apps/tauri/issues/5250#issuecomment-2569380578> **Since** 2.3.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L841> |
|  `dataDirectory?` | `string` | Set a custom path for the webview’s data directory (localStorage, cache, etc.) **relative to \[`appDataDir()`\]/\${label}**. For security reasons, paths outside of that location can only be configured on the Rust side. Platform-specific: - **Windows**: WebViews with different values for settings like `additionalBrowserArgs`, `browserExtensionsEnabled` or `scrollBarStyle` must have different data directories. - **macOS / iOS**: Unsupported, use `dataStoreIdentifier` instead. - **Android**: Unsupported. **Since** 2.9.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L870> |
|  `dataStoreIdentifier?` | `number`\[\] | Initialize the WebView with a custom data store identifier. This can be seen as a replacement for `dataDirectory` which is unavailable in WKWebView. See <https://developer.apple.com/documentation/webkit/wkwebsitedatastore/init(foridentifier:)?language=objc> The array must contain 16 u8 numbers. Platform-specific: - **macOS / iOS**: Available on macOS \>= 14 and iOS \>= 17 - **Windows / Linux / Android**: Unsupported. **Since** 2.9.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L884> |
|  `devtools?` | `boolean` | Whether web inspector, which is usually called browser devtools, is enabled or not. Enabled by default. This API works in **debug** builds, but requires `devtools` feature flag to enable it in **release** builds. Platform-specific - macOS: This will call private functions on **macOS**. - Android: Open `chrome://inspect/#devices` in Chrome to get the devtools window. Wry’s `WebView` devtools API isn’t supported on Android. - iOS: Open Safari \> Develop \> \[Your Device Name\] \> \[Your WebView\] to get the devtools window. **Since** 2.1.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L809> |
|  `disableInputAccessoryView?` | `boolean` | Allows disabling the input accessory view on iOS. The accessory view is the view that appears above the keyboard when a text input element is focused. It usually displays a view with “Done”, “Next” buttons. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L857> |
|  `dragDropEnabled?` | `boolean` | Whether the drag and drop is enabled or not on the webview. By default it is enabled. Disabling it is required to use HTML5 drag and drop on the frontend on Windows. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L742> |
|  `focus?` | `boolean` | Whether the webview should have focus or not **Since** 2.1.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L736> |
|  `generalAutofillEnabled?` | `boolean` | Controls the WebView’s browser-level general autofill behavior. **This option does not disable password or credit card autofill.** When set to `false`, the WebView will not automatically populate general form fields using previously stored data such as addresses or contact information. If not specified, this is `true` by default. \## Platform-specific - **Windows**: Supported. WebView2’s autofill feature (called “Suggestions”) may not honor `autocomplete="off"` on input elements in some cases. - **Linux / Android / iOS / macOS**: Unsupported and performs no operation. **Since** 2.11.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L918> |
|  `height` | `number` | The initial height in logical pixels. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L724> |
|  `incognito?` | `boolean` | Whether or not the webview should be launched in incognito mode. Platform-specific - **Android:** Unsupported. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L758> |
|  `javascriptDisabled?` | `boolean` | Whether we should disable JavaScript code execution on the webview or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L845> |
|  `proxyUrl?` | `string` | The proxy URL for the WebView for all network requests. Must be either a `http://` or a `socks5://` URL. Platform-specific - **macOS**: Requires the `macos-proxy` feature flag and only compiles for macOS 14+. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L768> |
|  `scrollBarStyle?` | [`ScrollBarStyle`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#scrollbarstyle) | Specifies the native scrollbar style to use with the webview. CSS styles that modify the scrollbar are applied on top of the native appearance configured here. Defaults to `default`, which is the browser default. \## Platform-specific - **Windows**: - `fluentOverlay` requires WebView2 Runtime version 125.0.2535.41 or higher, and does nothing on older versions. - This option must be given the same value for all webviews. - **Linux / Android / iOS / macOS**: Unsupported. Only supports `Default` and performs no operation. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L899> |
|  `transparent?` | `boolean` | Whether the webview is transparent or not. Note that on `macOS` this requires the `macos-private-api` feature flag, enabled under `tauri.conf.json > app > macOSPrivateApi`. WARNING: Using private APIs on `macOS` prevents your application from being accepted to the `App Store`. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L730> |
|  `url?` | `string` | Remote URL or local file path to open. - URL such as `https://github.com/tauri-apps` is opened directly on a Tauri webview. - data: URL such as `data:text/html,<html>...` is only supported with the `webview-data-url` Cargo feature for the `tauri` dependency. - local file path or route such as `/path/to/page.html` or `/users` is appended to the application URL (the devServer URL on development, or `tauri://localhost/` and `https://tauri.localhost/` on production). | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L716> |
|  `useHttpsScheme?` | `boolean` | Sets whether the custom protocols should use `https://<scheme>.localhost` instead of the default `http://<scheme>.localhost` on Windows and Android. Defaults to `false`. \#### Note Using a `https` scheme will NOT allow mixed content when trying to fetch `http` endpoints and therefore will not match the behavior of the `<scheme>://localhost` protocols used on macOS and Linux. \#### Warning Changing this value between releases will change the IndexedDB, cookies and localstorage location and your app will not be able to access them. **Since** 2.1.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L795> |
|  `userAgent?` | `string` | The user agent for the webview. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L750> |
|  `width` | `number` | The initial width in logical pixels. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L722> |
|  `x` | `number` | The initial vertical position in logical pixels. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L718> |
|  `y` | `number` | The initial horizontal position in logical pixels. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L720> |
|  `zoomHotkeysEnabled?` | `boolean` | Whether page zooming by hotkeys is enabled Platform-specific: - **Windows**: Controls WebView2’s [`IsZoomControlEnabled`](https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/winrt/microsoft_web_webview2_core/corewebview2settings?view=webview2-winrt-1.0.2420.47#iszoomcontrolenabled) setting. - **MacOS / Linux**: Injects a polyfill that zooms in and out with `ctrl/command` + `-/=`, 20% in each step, ranging from 20% to 1000%. Requires `webview:allow-set-webview-zoom` permission - **Android / iOS**: Unsupported. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L780> |

## Type Aliases

### Color

```
type Color: [number, number, number] | [number, number, number, number] | object | string;
```

An RGBA color. Each value has minimum of 0 and maximum of 255.

It can be either a string `#ffffff`, an array of 3 or 4 elements or an
object.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2121>

------------------------------------------------------------------------

### DragDropEvent

```
type DragDropEvent: object | object | object | object;
```

The drag and drop event types.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L43>

## Functions

### getAllWebviews()

```
function getAllWebviews(): Promise<Webview[]>
```

Gets a list of instances of `Webview` for all available webviews.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview)\[\]\>

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L70>

------------------------------------------------------------------------

### getCurrentWebview()

```
function getCurrentWebview(): Webview
```

Get an instance of `Webview` for the current webview.

#### Returns

[`Webview`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#webview)

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L54>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
