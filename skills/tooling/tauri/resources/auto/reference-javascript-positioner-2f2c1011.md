# @tauri-apps/plugin-positioner

## Enumerations

### Position

Well known window positions.

#### Enumeration Members

##### BottomCenter

```
BottomCenter: 5;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L18>

##### BottomLeft

```
BottomLeft: 2;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L15>

##### BottomRight

```
BottomRight: 3;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L16>

##### Center

```
Center: 8;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L21>

##### LeftCenter

```
LeftCenter: 6;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L19>

##### RightCenter

```
RightCenter: 7;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L20>

##### TopCenter

```
TopCenter: 4;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L17>

##### TopLeft

```
TopLeft: 0;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L13>

##### TopRight

```
TopRight: 1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L14>

##### TrayBottomCenter

```
TrayBottomCenter: 14;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L27>

##### TrayBottomLeft

```
TrayBottomLeft: 10;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L23>

##### TrayBottomRight

```
TrayBottomRight: 12;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L25>

##### TrayCenter

```
TrayCenter: 13;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L26>

##### TrayLeft

```
TrayLeft: 9;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L22>

##### TrayRight

```
TrayRight: 11;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L24>

## Functions

### handleIconState()

```
function handleIconState(event): Promise<void>
```

#### Parameters

| Parameter | Type            |
|-----------|-----------------|
| `event`   | `TrayIconEvent` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L55>

------------------------------------------------------------------------

### moveWindow()

```
function moveWindow(to): Promise<void>
```

Moves the `Window` to the given
[Position](https://v2.tauri.app/reference/javascript/positioner/#position) using
`WindowExt.move_window()` All positions are relative to the **current**
screen.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `to` | [`Position`](https://v2.tauri.app/reference/javascript/positioner/#position) | The [Position](https://v2.tauri.app/reference/javascript/positioner/#position) to move to. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L36>

------------------------------------------------------------------------

### moveWindowConstrained()

```
function moveWindowConstrained(to): Promise<void>
```

Moves the `Window` to the given
[Position](https://v2.tauri.app/reference/javascript/positioner/#position) using
`WindowExt.move_window_constrained()`

This move operation constrains the window to the screen dimensions in
case of tray-icon positions.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `to` | [`Position`](https://v2.tauri.app/reference/javascript/positioner/#position) | The (tray) [Position](https://v2.tauri.app/reference/javascript/positioner/#position) to move to. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L49>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
