# tray

## Classes

### TrayIcon

Tray icon class and associated methods. This type constructor is
private, instead, you should use the static method
[`TrayIcon.new`](https://v2.tauri.app/reference/javascript/api/namespacetray/#new).

#### Warning

Unlike Rust, javascript does not have any way to run cleanup code when
an object is being removed by garbage collection, but this tray icon
will be cleaned up when the tauri app exists, however if you want to
cleanup this object early, you need to call
[`TrayIcon.close`](https://v2.tauri.app/reference/javascript/api/namespacecore/#close).

#### Example

```
import { TrayIcon } from '@tauri-apps/api/tray';const tray = await TrayIcon.new({ tooltip: 'awesome tray tooltip' });tray.set_tooltip('new tooltip');
```

#### Extends

- [`Resource`](https://v2.tauri.app/reference/javascript/api/namespacecore/#resource)

#### Properties

| Property | Modifier | Type | Description | Defined in |
|----|----|----|----|----|
|  `id` | `public` | `string` | The id associated with this tray icon. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L160> |

#### Accessors

##### rid

```
get rid(): number
```

###### Returns

`number`

###### Inherited from

[`Resource`](https://v2.tauri.app/reference/javascript/api/namespacecore/#resource).[`rid`](https://v2.tauri.app/reference/javascript/api/namespacecore/#rid)

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

###### Inherited from

[`Resource`](https://v2.tauri.app/reference/javascript/api/namespacecore/#resource).[`close`](https://v2.tauri.app/reference/javascript/api/namespacecore/#close)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L330>

##### setIcon()

```
setIcon(icon): Promise<void>
```

Sets a new tray icon. If `null` is provided, it will remove the icon.

Note that you may need the `image-ico` or `image-png` Cargo features to
use this API. To enable it, change your Cargo.toml file:

```
[dependencies]tauri = { version = "...", features = ["...", "image-png"] }
```

###### Parameters

| Parameter | Type |
|----|----|
| `icon` | \| `null` \| `string` \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) \| [`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L224>

##### setIconAsTemplate()

```
setIconAsTemplate(asTemplate): Promise<void>
```

Sets the current icon as a
[template](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc).
**macOS only**

###### Parameters

| Parameter    | Type      |
|--------------|-----------|
| `asTemplate` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L292>

##### setIconWithAsTemplate()

```
setIconWithAsTemplate(icon, asTemplate): Promise<void>
```

Sets a new tray icon and template status atomically. **macOS only**.

Note that you may need the `image-ico` or `image-png` Cargo features to
use this API. To enable it, change your Cargo.toml file:

```
[dependencies]tauri = { version = "...", features = ["...", "image-png"] }
```

###### Parameters

| Parameter | Type |
|----|----|
| `icon` | \| `null` \| `string` \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) \| [`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image) |
| `asTemplate` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L309>

##### setMenu()

```
setMenu(menu): Promise<void>
```

Sets a new tray menu.

Platform-specific:

- **Linux**: once a menu is set it cannot be removed so `null` has no
  effect

###### Parameters

| Parameter | Type |
|----|----|
| `menu` | `null` \| [`Submenu`](https://v2.tauri.app/reference/javascript/api/namespacemenu/#submenu) \| [`Menu`](https://v2.tauri.app/reference/javascript/api/namespacemenu/#menu) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L241>

##### ~~setMenuOnLeftClick()~~

```
setMenuOnLeftClick(onLeft): Promise<void>
```

Disable or enable showing the tray menu on left click.

Platform-specific:

- **Linux**: Unsupported.

###### Parameters

| Parameter | Type      |
|-----------|-----------|
| `onLeft`  | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Deprecated

use
[`TrayIcon.setShowMenuOnLeftClick`](https://v2.tauri.app/reference/javascript/api/namespacetray/#setshowmenuonleftclick)
instead.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L333>

##### setShowMenuOnLeftClick()

```
setShowMenuOnLeftClick(onLeft): Promise<void>
```

Disable or enable showing the tray menu on left click.

Platform-specific:

- **Linux**: Unsupported.

###### Parameters

| Parameter | Type      |
|-----------|-----------|
| `onLeft`  | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Since

2.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L349>

##### setTempDirPath()

```
setTempDirPath(path): Promise<void>
```

Sets the tray icon temp dir path. **Linux only**.

On Linux, we need to write the icon to the disk and usually it will be
`$XDG_RUNTIME_DIR/tray-icon` or `$TEMP/tray-icon`.

###### Parameters

| Parameter | Type               |
|-----------|--------------------|
| `path`    | `null` \| `string` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L287>

##### setTitle()

```
setTitle(title): Promise<void>
```

Sets the tooltip for this tray icon.

Platform-specific:

- **Linux:** The title will not be shown unless there is an icon as
  well. The title is useful for numerical and other frequently updated
  information. In general, it shouldn’t be shown unless a user requests
  it as it can take up a significant amount of space on the user’s
  panel. This may not be shown in all visualizations.
- **Windows:** Unsupported

###### Parameters

| Parameter | Type               |
|-----------|--------------------|
| `title`   | `null` \| `string` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L272>

##### setTooltip()

```
setTooltip(tooltip): Promise<void>
```

Sets the tooltip for this tray icon.

Platform-specific:

- **Linux:** Unsupported

###### Parameters

| Parameter | Type               |
|-----------|--------------------|
| `tooltip` | `null` \| `string` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L256>

##### setVisible()

```
setVisible(visible): Promise<void>
```

Show or hide this tray icon.

###### Parameters

| Parameter | Type      |
|-----------|-----------|
| `visible` | `boolean` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L277>

##### getById()

```
static getById(id): Promise<null | TrayIcon>
```

Gets a tray icon using the provided id.

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `id`      | `string` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`
\| [`TrayIcon`](https://v2.tauri.app/reference/javascript/api/namespacetray/#trayicon)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L168>

##### new()

```
static new(options?): Promise<TrayIcon>
```

Creates a new
[`TrayIcon`](https://v2.tauri.app/reference/javascript/api/namespacetray/#trayicon)

Platform-specific:

- **Linux:** Sometimes the icon won’t be visible unless a menu is set.
  Setting an empty
  [`Menu`](https://v2.tauri.app/reference/javascript/api/namespacemenu/#menu) is enough.

###### Parameters

| Parameter | Type |
|----|----|
| `options`? | [`TrayIconOptions`](https://v2.tauri.app/reference/javascript/api/namespacetray/#trayiconoptions) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`TrayIcon`](https://v2.tauri.app/reference/javascript/api/namespacetray/#trayicon)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L192>

##### removeById()

```
static removeById(id): Promise<void>
```

Removes a tray icon using the provided id from tauri’s internal state.

Note that this may cause the tray icon to disappear if it wasn’t cloned
somewhere else or referenced by JS.

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `id`      | `string` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L180>

## Interfaces

### TrayIconOptions

[`TrayIcon`](https://v2.tauri.app/reference/javascript/api/namespacetray/#new) creation
options

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `action?` | (`event`: [`TrayIconEvent`](https://v2.tauri.app/reference/javascript/api/namespacetray/#trayiconevent)) =\> `void` | A handler for an event on the tray icon. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L137> |
|  `icon?` | \| `string` \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) \| [`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image) | The tray icon which could be icon bytes or path to the icon file. Note that you may need the `image-ico` or `image-png` Cargo features to use this API. To enable it, change your Cargo.toml file: `[dependencies] tauri = { version = "...", features = ["...", "image-png"] }` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L89> |
|  `iconAsTemplate?` | `boolean` | Use the icon as a [template](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc). **macOS only**. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L115> |
|  `id?` | `string` | The tray icon id. If undefined, a random one will be assigned | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L76> |
|  `menu?` | [`Submenu`](https://v2.tauri.app/reference/javascript/api/namespacemenu/#submenu) \| [`Menu`](https://v2.tauri.app/reference/javascript/api/namespacemenu/#menu) | The tray icon menu | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L78> |
|  ~~`menuOnLeftClick?`~~ | `boolean` | Whether to show the tray menu on left click or not, default is `true`. Platform-specific: - **Linux**: Unsupported. **Deprecated** use [`TrayIconOptions.showMenuOnLeftClick`](https://v2.tauri.app/reference/javascript/api/namespacetray/#showmenuonleftclick) instead. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L125> |
|  `showMenuOnLeftClick?` | `boolean` | Whether to show the tray menu on left click or not, default is `true`. Platform-specific: - **Linux**: Unsupported. **Since** 2.2.0 | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L135> |
|  `tempDirPath?` | `string` | The tray icon temp dir path. **Linux only**. On Linux, we need to write the icon to the disk and usually it will be `$XDG_RUNTIME_DIR/tray-icon` or `$TEMP/tray-icon`. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L111> |
|  `title?` | `string` | The tray title Platform-specific - **Linux:** The title will not be shown unless there is an icon as well. The title is useful for numerical and other frequently updated information. In general, it shouldn’t be shown unless a user requests it as it can take up a significant amount of space on the user’s panel. This may not be shown in all visualizations. - **Windows:** Unsupported. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L104> |
|  `tooltip?` | `string` | The tray icon tooltip | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L91> |

## Type Aliases

### MouseButton

```
type MouseButton: "Left" | "Right" | "Middle";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L11>

------------------------------------------------------------------------

### MouseButtonState

```
type MouseButtonState: "Up" | "Down";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L10>

------------------------------------------------------------------------

### TrayIconClickEvent

```
type TrayIconClickEvent: object;
```

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `button` | [`MouseButton`](https://v2.tauri.app/reference/javascript/api/namespacetray/#mousebutton) | Mouse button that triggered this event. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L35> |
| `buttonState` | [`MouseButtonState`](https://v2.tauri.app/reference/javascript/api/namespacetray/#mousebuttonstate) | Mouse button state when this event was triggered. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L37> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L33>

------------------------------------------------------------------------

### TrayIconEvent

```
type TrayIconEvent:  | TrayIconEventBase<"Click"> & TrayIconClickEvent  | TrayIconEventBase<"DoubleClick"> & Omit<TrayIconClickEvent, "buttonState">  | TrayIconEventBase<"Enter">  | TrayIconEventBase<"Move">| TrayIconEventBase<"Leave">;
```

Describes a tray icon event.

Platform-specific:

- **Linux**: Unsupported. The event is not emitted even though the icon
  is shown, the icon will still show a context menu on right click.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L48>

------------------------------------------------------------------------

### TrayIconEventBase\<T\>

```
type TrayIconEventBase<T>: object;
```

#### Type Parameters

| Type Parameter |
|----|
| `T` *extends* [`TrayIconEventType`](https://v2.tauri.app/reference/javascript/api/namespacetray/#trayiconeventtype) |

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `id` | `string` | Id of the tray icon which triggered this event. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L23> |
| `position` | [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) | Physical position of the click the triggered this event. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L25> |
| `rect` | `object` | Position and size of the tray icon. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L27> |
| `rect.position` | [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) | \- | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L28> |
| `rect.size` | [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) | \- | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L29> |
| `type` | `T` | The tray icon event type | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L21> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L19>

------------------------------------------------------------------------

### TrayIconEventType

```
type TrayIconEventType:  | "Click"  | "DoubleClick"  | "Enter"  | "Move"  | "Leave";
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L12>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
