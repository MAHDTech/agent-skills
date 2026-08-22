+++
title = "reference-javascript-api-namespacewindow-be7624d9"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# window

Provides APIs to create windows, communicate with other windows and
manipulate the current window.

#### Window events

Events can be listened to using
[Window.listen](https://v2.tauri.app/reference/javascript/api/namespacewindow/#listen):

```
import { getCurrentWindow } from "@tauri-apps/api/window";getCurrentWindow().listen("my-window-event", ({ event, payload }) => { });
```

## References

### Color

Re-exports [Color](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color)

### DragDropEvent

Re-exports
[DragDropEvent](https://v2.tauri.app/reference/javascript/api/namespacewebview/#dragdropevent)

### LogicalPosition

Re-exports
[LogicalPosition](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition)

### LogicalSize

Re-exports
[LogicalSize](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize)

### PhysicalPosition

Re-exports
[PhysicalPosition](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)

### PhysicalSize

Re-exports
[PhysicalSize](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)

## Enumerations

### BackgroundThrottlingPolicy

Background throttling policy

#### Since

2.0.0

#### Enumeration Members

##### Disabled

```
Disabled: "disabled";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2133>

##### Suspend

```
Suspend: "suspend";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2135>

##### Throttle

```
Throttle: "throttle";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2134>

------------------------------------------------------------------------

### Effect

Platform-specific window effects

#### Since

2.0.0

#### Enumeration Members

##### Acrylic

```
Acrylic: "acrylic";
```

**Windows 10/11**

#### Notes

This effect has bad performance when resizing/dragging the window on
Windows 10 v1903+ and Windows 11 build 22000.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2272>

##### ~~AppearanceBased~~

```
AppearanceBased: "appearanceBased";
```

A default material appropriate for the view’s effectiveAppearance.
**macOS 10.14-**

###### Deprecated

since macOS 10.14. You should instead choose an appropriate semantic
material.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2172>

##### Blur

```
Blur: "blur";
```

**Windows 7/10/11(22H1) Only**

#### Notes

This effect has bad performance when resizing/dragging the window on
Windows 11 build 22621.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2264>

##### ContentBackground

```
ContentBackground: "contentBackground";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2244>

##### ~~Dark~~

```
Dark: "dark";
```

**macOS 10.14-**

###### Deprecated

since macOS 10.14. Use a semantic material instead.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2184>

##### FullScreenUI

```
FullScreenUI: "fullScreenUI";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2236>

##### HeaderView

```
HeaderView: "headerView";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2220>

##### HudWindow

```
HudWindow: "hudWindow";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2232>

##### ~~Light~~

```
Light: "light";
```

**macOS 10.14-**

###### Deprecated

since macOS 10.14. Use a semantic material instead.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2178>

##### ~~MediumLight~~

```
MediumLight: "mediumLight";
```

**macOS 10.14-**

###### Deprecated

since macOS 10.14. Use a semantic material instead.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2190>

##### Menu

```
Menu: "menu";
```

**macOS 10.11+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2208>

##### Mica

```
Mica: "mica";
```

**Windows 11 Only**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2256>

##### Popover

```
Popover: "popover";
```

**macOS 10.11+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2212>

##### Selection

```
Selection: "selection";
```

**macOS 10.10+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2204>

##### Sheet

```
Sheet: "sheet";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2224>

```
Sidebar: "sidebar";
```

**macOS 10.11+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2216>

##### Tabbed

```
Tabbed: "tabbed";
```

Tabbed effect that matches the system dark preference **Windows 11
Only**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2276>

##### TabbedDark

```
TabbedDark: "tabbedDark";
```

Tabbed effect with dark mode but only if dark mode is enabled on the
system **Windows 11 Only**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2280>

##### TabbedLight

```
TabbedLight: "tabbedLight";
```

Tabbed effect with light mode **Windows 11 Only**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2284>

##### Titlebar

```
Titlebar: "titlebar";
```

**macOS 10.10+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2200>

##### Tooltip

```
Tooltip: "tooltip";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2240>

##### ~~UltraDark~~

```
UltraDark: "ultraDark";
```

**macOS 10.14-**

###### Deprecated

since macOS 10.14. Use a semantic material instead.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2196>

##### UnderPageBackground

```
UnderPageBackground: "underPageBackground";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2252>

##### UnderWindowBackground

```
UnderWindowBackground: "underWindowBackground";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2248>

##### WindowBackground

```
WindowBackground: "windowBackground";
```

**macOS 10.14+**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2228>

------------------------------------------------------------------------

### EffectState

Window effect state **macOS only**

#### See

<https://developer.apple.com/documentation/appkit/nsvisualeffectview/state>

#### Since

2.0.0

#### Enumeration Members

##### Active

```
Active: "active";
```

Make window effect state always active **macOS only**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2302>

##### FollowsWindowActiveState

```
FollowsWindowActiveState: "followsWindowActiveState";
```

Make window effect state follow the window’s active state **macOS only**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2298>

##### Inactive

```
Inactive: "inactive";
```

Make window effect state always inactive **macOS only**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2306>

------------------------------------------------------------------------

### ProgressBarStatus

#### Enumeration Members

##### Error

```
Error: "error";
```

Error state. **Treated as Normal on linux**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L220>

##### Indeterminate

```
Indeterminate: "indeterminate";
```

Indeterminate state. **Treated as Normal on Linux and macOS**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L212>

##### None

```
None: "none";
```

Hide progress bar.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L204>

##### Normal

```
Normal: "normal";
```

Normal state.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L208>

##### Paused

```
Paused: "paused";
```

Paused state. **Treated as Normal on Linux**

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L216>

------------------------------------------------------------------------

### ScrollBarStyle

The scrollbar style to use in the webview.

## Platform-specific

**Windows**: This option must be given the same value for all webviews.

#### Since

2.8.0

#### Enumeration Members

##### Default

```
Default: "default";
```

The default scrollbar style for the webview.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2151>

##### FluentOverlay

```
FluentOverlay: "fluentOverlay";
```

Fluent UI style overlay scrollbars. **Windows Only**

Requires WebView2 Runtime version 125.0.2535.41 or higher, does nothing
on older versions, see
<https://learn.microsoft.com/en-us/microsoft-edge/webview2/release-notes/?tabs=dotnetcsharp#10253541>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2158>

------------------------------------------------------------------------

### UserAttentionType

Attention type to request on a window.

#### Since

1.0.0

#### Enumeration Members

##### Critical

```
Critical: 1;
```

Platform-specific

- **macOS:** Bounces the dock icon until the application is in focus.
- **Windows:** Flashes both the window and the taskbar button until the
  application is in focus.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L129>

##### Informational

```
Informational: 2;
```

Platform-specific

- **macOS:** Bounces the dock icon once.
- **Windows:** Flashes the taskbar button until the application is in
  focus.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L135>

## Classes

### CloseRequestedEvent

#### Constructors

##### new CloseRequestedEvent()

```
new CloseRequestedEvent(event): CloseRequestedEvent
```

###### Parameters

| Parameter | Type |
|----|----|
| `event` | [`Event`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventt)\<`unknown`\> |

###### Returns

[`CloseRequestedEvent`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#closerequestedevent)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L145>

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `event` | [`EventName`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventname) | Event name | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L140> |
|  `id` | `number` | Event identifier used to unlisten | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L142> |

#### Methods

##### isPreventDefault()

```
isPreventDefault(): boolean
```

###### Returns

`boolean`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L154>

##### preventDefault()

```
preventDefault(): void
```

###### Returns

`void`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L150>

------------------------------------------------------------------------

### Window

Create new window or get a handle to an existing one.

Windows are identified by a *label* a unique identifier that can be used
to reference it later. It may only contain alphanumeric characters
`a-zA-Z` plus the following special characters `-`, `/`, `:` and `_`.

#### Example

```
import { Window } from "@tauri-apps/api/window"
const appWindow = new Window('theUniqueLabel');
appWindow.once('tauri://created', function () { // window successfully created});appWindow.once('tauri://error', function (e) { // an error happened creating the window});
// emit an event to the backendawait appWindow.emit("some-event", "data");// listen to an event from the backendconst unlisten = await appWindow.listen("event-name", e => {});unlisten();
```

#### Since

2.0.0

#### Extended by

- [`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

#### Constructors

##### new Window()

```
new Window(label, options): Window
```

Creates a new Window.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `label` | `string` | The unique window label. Must be alphanumeric: `a-zA-Z-/:_`. |
| `options` | [`WindowOptions`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#windowoptions) | \- |

###### Returns

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)

The [Window](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) instance
to communicate with the window.

###### Example

```
import { Window } from '@tauri-apps/api/window';const appWindow = new Window('my-label');appWindow.once('tauri://created', function () { // window successfully created});appWindow.once('tauri://error', function (e) { // an error happened creating the window});
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L328>

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `label` | `string` | The window label. It is a unique identifier for the window, can be used to reference it later. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L306> |
|  `listeners` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, [`EventCallback`](https://v2.tauri.app/reference/javascript/api/namespaceevent/#eventcallbackt)\<`any`\>\[\]\> | Local event listeners. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L309> |

#### Methods

##### activityName()

```
activityName(): Promise<string>
```

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L877>

##### clearEffects()

```
clearEffects(): Promise<void>
```

Clear any applied effects if possible.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1265>

##### close()

```
close(): Promise<void>
```

Closes the window.

Note this emits a closeRequested event so you can intercept it. To force
window close, use
[Window.destroy](https://v2.tauri.app/reference/javascript/api/namespacewindow/#destroy).

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().close();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1186>

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
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().emit('window-loaded', { loggedIn: true, token: 'authToken' });
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L479>

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
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().emit('main', 'window-loaded', { loggedIn: true, token: 'authToken' });
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L506>

##### hide()

```
hide(): Promise<void>
```

Sets the window visibility to false.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().hide();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1168>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L796>

##### listen()

```
listen<T>(event, handler): Promise<UnlistenFn>
```

Listen to an emitted event on this window.

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
import { getCurrentWindow } from '@tauri-apps/api/window';const unlisten = await getCurrentWindow().listen<string>('state-changed', (event) => {  console.log(`Got error: ${payload}`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L417>

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
import { getCurrentWindow } from "@tauri-apps/api/webview";const unlisten = await getCurrentWindow().onDragDropEvent((event) => { if (event.payload.type === 'over') {   console.log('User hovering', event.payload.position); } else if (event.payload.type === 'drop') {   console.log('User dropped', event.payload.paths); } else {   console.log('File drop cancelled'); }});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1965>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2109>

##### once()

```
once<T>(event, handler): Promise<UnlistenFn>
```

Listen to an emitted event on this window only once.

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
import { getCurrentWindow } from '@tauri-apps/api/window';const unlisten = await getCurrentWindow().once<null>('initialized', (event) => {  console.log(`Window initialized!`);});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmountedunlisten();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L452>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L620>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L551>

##### sceneIdentifier()

```
sceneIdentifier(): Promise<string>
```

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1283>

##### setBackgroundColor()

```
setBackgroundColor(color): Promise<void>
```

Sets the window background color.

Platform-specific:

- **Windows:** alpha channel is ignored.
- **iOS / Android:** Unsupported.

###### Parameters

| Parameter | Type                                                         |
|-----------|--------------------------------------------------------------|
| `color`   | [`Color`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Since

2.1.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1635>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L950>

##### setFocus()

```
setFocus(): Promise<void>
```

Bring the window to front and focus.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().setFocus();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1476>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1775>

##### setPosition()

```
setPosition(position): Promise<void>
```

Sets the window outer position.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `position` | [`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition) \| [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) \| [`Position`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#position) | The new position, in logical or physical pixels. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';await getCurrentWindow().setPosition(new LogicalPosition(600, 500));
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1423>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1459>

##### setSize()

```
setSize(size): Promise<void>
```

Resizes the window with a new inner size.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `size` | [`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) \| [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) \| [`Size`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#size) | The logical or physical inner size. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';await getCurrentWindow().setSize(new LogicalSize(600, 500));
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1336>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1819>

##### show()

```
show(): Promise<void>
```

Sets the window visibility to true.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';await getCurrentWindow().show();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1152>

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

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1136>

##### getAll()

```
static getAll(): Promise<Window[]>
```

Gets a list of instances of `Window` for all available windows.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)\[\]\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L375>

##### getByLabel()

```
static getByLabel(label): Promise<null | Window>
```

Gets the Window associated with the given label.

###### Parameters

| Parameter | Type     | Description       |
|-----------|----------|-------------------|
| `label`   | `string` | The window label. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`
\| [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)\>

The Window instance to communicate with the window or null if the window
doesn’t exist.

###### Example

```
import { Window } from '@tauri-apps/api/window';const mainWindow = Window.getByLabel('main');
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L361>

##### getCurrent()

```
static getCurrent(): Window
```

Get an instance of `Window` for the current window.

###### Returns

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L368>

##### getFocusedWindow()

```
static getFocusedWindow(): Promise<null | Window>
```

Gets the focused window.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`
\| [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)\>

The Window instance or `undefined` if there is not any focused window.

###### Example

```
import { Window } from '@tauri-apps/api/window';const focusedWindow = Window.getFocusedWindow();
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L389>

## Interfaces

### Effects

The window effects configuration object

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `color?` | [`Color`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color) | Window effect color. Affects [Effect.Blur](https://v2.tauri.app/reference/javascript/api/namespacewindow/#blur) and [Effect.Acrylic](https://v2.tauri.app/reference/javascript/api/namespacewindow/#acrylic) only on Windows 10 v1903+. Doesn’t have any effect on Windows 7 or Windows 11. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2331> |
|  `effects` | [`Effect`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#effect)\[\] | List of Window effects to apply to the Window. Conflicting effects will apply the first one and ignore the rest. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2318> |
|  `radius?` | `number` | Window effect corner radius **macOS Only** | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2326> |
|  `state?` | [`EffectState`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#effectstate) | Window effect state **macOS Only** | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2322> |

------------------------------------------------------------------------

### Monitor

Allows you to retrieve information about a given monitor.

#### Since

1.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `name` | `null` \| `string` | Human-readable name of the monitor | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L49> |
|  `position` | [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) | the Top-left corner position of the monitor relative to the larger full screen area, in physical pixels. Note that window creation options such as `x`, `y`, `width` and `height` expect logical pixels, so convert with [`Monitor.scaleFactor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#scalefactor-1) first: `import { currentMonitor } from '@tauri-apps/api/window'; import { WebviewWindow } from '@tauri-apps/api/webviewWindow'; const monitor = await currentMonitor(); if (monitor) { const position = monitor.position.toLogical(monitor.scaleFactor); const webview = new WebviewWindow('my-label', { x: position.x, y: position.y }); }` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L75> |
|  `scaleFactor` | `number` | The scale factor that can be used to map physical pixels to logical pixels, e.g. `monitor.position.toLogical(monitor.scaleFactor)`. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L90> |
|  `size` | [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) | The monitor’s resolution in physical pixels. Use [`Monitor.scaleFactor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#scalefactor-1) to convert to logical pixels: `const logicalSize = monitor.size.toLogical(monitor.scaleFactor);` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L58> |
|  `workArea` | `object` | The monitor’s work area (the monitor area excluding taskbars and docks) in physical pixels. Use [`Monitor.scaleFactor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#scalefactor-1) to convert to logical pixels as shown in [`Monitor.position`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#position). | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L82> |
|  `workArea.position` | [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) | \- | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L83> |
|  `workArea.size` | [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) | \- | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L84> |

------------------------------------------------------------------------

### ProgressBarState

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `progress?` | `number` | The progress bar progress. This can be a value ranging from `0` to `100` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L238> |
|  `status?` | [`ProgressBarStatus`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#progressbarstatus) | The progress bar status. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L234> |

------------------------------------------------------------------------

### ScaleFactorChanged

The payload for the `scaleChange` event.

#### Since

1.0.2

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `scaleFactor` | `number` | The new window scale factor. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L113> |
|  `size` | [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) | The new window size | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L115> |

------------------------------------------------------------------------

### WindowOptions

Configuration for the window to create.

#### Since

1.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `activityName?` | `string` | The name of the Android activity to create for this window. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2568> |
|  `allowLinkPreview?` | `boolean` | on macOS and iOS there is a link preview on long pressing links, this is enabled by default. see <https://docs.rs/objc2-web-kit/latest/objc2_web_kit/struct.WKWebView.html#method.allowsLinkPreview> | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2542> |
|  `alwaysOnBottom?` | `boolean` | Whether the window should always be below other windows. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2408> |
|  `alwaysOnTop?` | `boolean` | Whether the window should always be on top of other windows or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2406> |
|  `backgroundColor?` | [`Color`](https://v2.tauri.app/reference/javascript/api/namespacewebview/#color) | Set the window background color. Platform-specific: - **Android / iOS:** Unsupported. - **Windows**: alpha channel is ignored. **Since** 2.1.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2519> |
|  `backgroundThrottling?` | [`BackgroundThrottlingPolicy`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#backgroundthrottlingpolicy) | Change the default background throttling behaviour. \## Platform-specific - **Linux / Windows / Android**: Unsupported. Workarounds like a pending WebLock transaction might suffice. - **iOS**: Supported since version 17.0+. - **macOS**: Supported since version 14.0+. see <https://github.com/tauri-apps/tauri/issues/5250#issuecomment-2569380578> **Since** 2.3.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2533> |
|  `center?` | `boolean` | Show window in the center of the screen.. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2349> |
|  `closable?` | `boolean` | Whether the window’s native close button is enabled or not. Defaults to `true`. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2474> |
|  `contentProtected?` | `boolean` | Prevents the window contents from being captured by other apps. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2410> |
|  `createdByActivityName?` | `string` | The name of the Android activity that is creating this webview window. This is important to determine which stack the activity will belong to. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2574> |
|  `decorations?` | `boolean` | Whether the window should have borders and bars or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2404> |
|  `disableInputAccessoryView?` | `boolean` | Allows disabling the input accessory view on iOS. The accessory view is the view that appears above the keyboard when a text input element is focused. It usually displays a view with “Done”, “Next” buttons. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2549> |
|  `focus?` | `boolean` | Whether the window will be initially focused or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2388> |
|  `focusable?` | `boolean` | Whether the window can be focused or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2390> |
|  `fullscreen?` | `boolean` | Whether the window is in fullscreen mode or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2386> |
|  `height?` | `number` | The initial height in logical pixels. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2357> |
|  `hiddenTitle?` | `boolean` | If `true`, sets the window title to be hidden on macOS. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2455> |
|  `javascriptDisabled?` | `boolean` | Whether we should disable JavaScript code execution on the webview or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2537> |
|  `maxHeight?` | `number` | The maximum height in logical pixels. Only applies if `maxWidth` is also set. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2365> |
|  `maxWidth?` | `number` | The maximum width in logical pixels. Only applies if `maxHeight` is also set. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2363> |
|  `maximizable?` | `boolean` | Whether the window’s native maximize button is enabled or not. Defaults to `true`. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2466> |
|  `maximized?` | `boolean` | Whether the window should be maximized upon creation or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2400> |
|  `minHeight?` | `number` | The minimum height in logical pixels. Only applies if `minWidth` is also set. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2361> |
|  `minWidth?` | `number` | The minimum width in logical pixels. Only applies if `minHeight` is also set. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2359> |
|  `minimizable?` | `boolean` | Whether the window’s native minimize button is enabled or not. Defaults to `true`. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2470> |
|  `noRedirectionBitmap?` | `boolean` | This sets `WS_EX_NOREDIRECTIONBITMAP`. This can avoid the white flash that may appear before the webview content is rendered when using a transparent window. **Windows only**. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2419> |
|  `parent?` | `string` \| [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) \| [`WebviewWindow`](https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/#webviewwindow) | Sets a parent to the window to be created. Can be either a [`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window) or a label of the window. Platform-specific - **Windows**: This sets the passed parent as an owner window to the window to be created. From [MSDN owned windows docs](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-features#owned-windows): - An owned window is always above its owner in the z-order. - The system automatically destroys an owned window when its owner is destroyed. - An owned window is hidden when its owner is minimized. - **Linux**: This makes the new window transient for parent, see <https://docs.gtk.org/gtk3/method.Window.set_transient_for.html> - **macOS**: This adds the window as a child of parent, see <https://developer.apple.com/documentation/appkit/nswindow/1419152-addchildwindow?language=objc> | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2488> |
|  `preventOverflow?` | `boolean` \| `PreventOverflowMargin` | Prevent the window from overflowing the working area (e.g. monitor size - taskbar size) on creation, which means the window size will be limited to `monitor size - taskbar size` Can either be set to `true` or to a PreventOverflowMargin object to set an additional margin that should be considered to determine the working area (in this case the window size will be limited to `monitor size - taskbar size - margin`) **NOTE**: The overflow check is only performed on window creation, resizes can still overflow Platform-specific - **iOS / Android:** Unsupported. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2380> |
|  `requestedBySceneIdentifier?` | `string` | Sets the identifier of the UIScene that is requesting the creation of this new scene, establishing a relationship between the two scenes. By default the system uses the foreground scene. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2581> |
|  `resizable?` | `boolean` | Whether the window is resizable or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2382> |
|  `scrollBarStyle?` | [`ScrollBarStyle`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#scrollbarstyle) | Specifies the native scrollbar style to use with the webview. CSS styles that modify the scrollbar are applied on top of the native appearance configured here. Defaults to `default`, which is the browser default. \## Platform-specific - **Windows**: - `fluentOverlay` requires WebView2 Runtime version 125.0.2535.41 or higher, and does nothing on older versions. - This option must be given the same value for all webviews. - **Linux / Android / iOS / macOS**: Unsupported. Only supports `Default` and performs no operation. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2564> |
|  `shadow?` | `boolean` | Whether or not the window has shadow. Platform-specific - **Windows:** - `false` has no effect on decorated window, shadows are always ON. - `true` will make undecorated window have a 1px white border, and on Windows 11, it will have a rounded corners. - **Linux:** Unsupported. **Since** 2.0.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2433> |
|  `skipTaskbar?` | `boolean` | Whether or not the window icon should be added to the taskbar. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2412> |
|  `tabbingIdentifier?` | `string` | Defines the window [tabbing identifier](https://developer.apple.com/documentation/appkit/nswindow/1644704-tabbingidentifier) on macOS. Windows with the same tabbing identifier will be grouped together. If the tabbing identifier is not set, automatic tabbing will be disabled. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2462> |
|  `theme?` | [`Theme`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#theme-2) | The initial window theme. Defaults to the system theme. Only implemented on Windows and macOS 10.14+. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2439> |
|  `title?` | `string` | Window title. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2384> |
|  `titleBarStyle?` | [`TitleBarStyle`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#titlebarstyle-1) | The style of the macOS title bar. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2443> |
|  `trafficLightPosition?` | [`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition) | The position of the window controls on macOS. Requires `titleBarStyle: 'overlay'` and `decorations: true`. **Since** 2.4.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2451> |
|  `transparent?` | `boolean` | Whether the window is transparent or not. Note that on `macOS` this requires the `macos-private-api` feature flag, enabled under `tauri.conf.json > app > macOSPrivateApi`. WARNING: Using private APIs on `macOS` prevents your application from being accepted to the `App Store`. On Windows, using `noRedirectionBitmap` can help avoid a white flash when creating a transparent window. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2398> |
|  `visible?` | `boolean` | Whether the window should be immediately visible upon creation or not. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2402> |
|  `visibleOnAllWorkspaces?` | `boolean` | Whether the window should be visible on all workspaces or virtual desktops. Platform-specific - **Windows / iOS / Android:** Unsupported. **Since** 2.0.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2497> |
|  `width?` | `number` | The initial width in logical pixels. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2355> |
|  `windowEffects?` | [`Effects`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#effects) | Window effects. Requires the window to be transparent. Platform-specific: - **Windows**: If using decorations or shadows, you may want to try this workaround <https://github.com/tauri-apps/tao/issues/72#issuecomment-975607891> - **Linux**: Unsupported | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2508> |
|  `x?` | `number` | The initial vertical position in logical pixels. Only applies if `y` is also set. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2351> |
|  `y?` | `number` | The initial horizontal position in logical pixels. Only applies if `x` is also set. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2353> |

------------------------------------------------------------------------

### WindowSizeConstraints

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `maxHeight?` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L227> |
|  `maxWidth?` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L226> |
|  `minHeight?` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L225> |
|  `minWidth?` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L224> |

## Type Aliases

### CursorIcon

```
type CursorIcon:  | "default"  | "crosshair"  | "hand"  | "arrow"  | "move"  | "text"  | "wait"  | "help"  | "progress"  | "notAllowed"  | "contextMenu"  | "cell"  | "verticalText"  | "alias"  | "copy"  | "noDrop"  | "grab"  | "grabbing"  | "allScroll"  | "zoomIn"  | "zoomOut"  | "eResize"  | "nResize"  | "neResize"  | "nwResize"  | "sResize"  | "seResize"  | "swResize"  | "wResize"  | "ewResize"  | "nsResize"  | "neswResize"  | "nwseResize"  | "colResize"  | "rowResize";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L159>

------------------------------------------------------------------------

### Theme

```
type Theme: "light" | "dark";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L93>

------------------------------------------------------------------------

### TitleBarStyle

```
type TitleBarStyle: "visible" | "transparent" | "overlay";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L94>

## Functions

### availableMonitors()

```
function availableMonitors(): Promise<Monitor[]>
```

Returns the list of all the monitors available on the system.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Monitor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#monitor)\[\]\>

#### Example

```
import { availableMonitors } from '@tauri-apps/api/window';const monitors = await availableMonitors();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2660>

------------------------------------------------------------------------

### currentMonitor()

```
function currentMonitor(): Promise<Monitor | null>
```

Returns the monitor on which the window currently resides. Returns
`null` if current monitor can’t be detected.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Monitor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#monitor)
\| `null`\>

#### Example

```
import { currentMonitor } from '@tauri-apps/api/window';const monitor = await currentMonitor();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2610>

------------------------------------------------------------------------

### cursorPosition()

```
function cursorPosition(): Promise<PhysicalPosition>
```

Get the cursor position relative to the top-left hand corner of the
desktop.

Note that the top-left hand corner of the desktop is not necessarily the
same as the screen. If the user uses a desktop with multiple monitors,
the top-left hand corner of the desktop is the top-left hand corner of
the main monitor on Windows and macOS or the top-left of the leftmost
monitor on X11.

The coordinates can be negative if the top-left hand corner of the
window is outside of the visible screen region.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2676>

------------------------------------------------------------------------

### getAllWindows()

```
function getAllWindows(): Promise<Window[]>
```

Gets a list of instances of `Window` for all available windows.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)\[\]\>

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L258>

------------------------------------------------------------------------

### getCurrentWindow()

```
function getCurrentWindow(): Window
```

Get an instance of `Window` for the current window.

#### Returns

[`Window`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#window)

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L246>

------------------------------------------------------------------------

### monitorFromPoint()

```
function monitorFromPoint(x, y): Promise<Monitor | null>
```

Returns the monitor that contains the given point. Returns `null` if
can’t find any.

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `x`       | `number` |
| `y`       | `number` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Monitor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#monitor)
\| `null`\>

#### Example

```
import { monitorFromPoint } from '@tauri-apps/api/window';const monitor = await monitorFromPoint(100.0, 200.0);
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2643>

------------------------------------------------------------------------

### primaryMonitor()

```
function primaryMonitor(): Promise<Monitor | null>
```

Returns the primary monitor of the system. Returns `null` if it can’t
identify any monitor as a primary one.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Monitor`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#monitor)
\| `null`\>

#### Example

```
import { primaryMonitor } from '@tauri-apps/api/window';const monitor = await primaryMonitor();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2627>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

