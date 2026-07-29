# dpi

## Classes

### LogicalPosition

A position represented in logical pixels. For an explanation of what
logical pixels are, see description of
[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize).

#### Since

2.0.0

#### Constructors

##### new LogicalPosition()

```
new LogicalPosition(x, y): LogicalPosition
```

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `x`       | `number` |
| `y`       | `number` |

###### Returns

[`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L219>

##### new LogicalPosition()

```
new LogicalPosition(object): LogicalPosition
```

###### Parameters

| Parameter          | Type     |
|--------------------|----------|
| `object`           | `object` |
| `object.Logical`   | `object` |
| `object.Logical.x` | `number` |
| `object.Logical.y` | `number` |

###### Returns

[`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L220>

##### new LogicalPosition()

```
new LogicalPosition(object): LogicalPosition
```

###### Parameters

| Parameter  | Type     |
|------------|----------|
| `object`   | `object` |
| `object.x` | `number` |
| `object.y` | `number` |

###### Returns

[`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L221>

#### Properties

| Property | Modifier | Type | Default value | Defined in |
|----|----|----|----|----|
|  `type` | `readonly` | `"Logical"` | `'Logical'` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L215> |
|  `x` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L216> |
|  `y` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L217> |

#### Methods

##### \_\_TAURI_TO_IPC_KEY\_\_()

```
__TAURI_TO_IPC_KEY__(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `x` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L263> |
| `y` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L264> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L261>

##### toJSON()

```
toJSON(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `x` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L263> |
| `y` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L264> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L268>

##### toPhysical()

```
toPhysical(scaleFactor): PhysicalPosition
```

Converts the logical position to a physical one.

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)

###### Example

```
import { LogicalPosition } from '@tauri-apps/api/dpi';import { getCurrentWindow } from '@tauri-apps/api/window';
const appWindow = getCurrentWindow();const factor = await appWindow.scaleFactor();const position = new LogicalPosition(400, 500);const physical = position.toPhysical(factor);
```

###### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L257>

------------------------------------------------------------------------

### LogicalSize

A size represented in logical pixels. Logical pixels are scaled
according to the window’s DPI scale. Most browser APIs (i.e.
`MouseEvent`’s `clientX`) will return logical pixels.

For logical-pixel-based position, see
[`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition).

#### Since

2.0.0

#### Constructors

##### new LogicalSize()

```
new LogicalSize(width, height): LogicalSize
```

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `width`   | `number` |
| `height`  | `number` |

###### Returns

[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L21>

##### new LogicalSize()

```
new LogicalSize(object): LogicalSize
```

###### Parameters

| Parameter               | Type     |
|-------------------------|----------|
| `object`                | `object` |
| `object.Logical`        | `object` |
| `object.Logical.height` | `number` |
| `object.Logical.width`  | `number` |

###### Returns

[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L22>

##### new LogicalSize()

```
new LogicalSize(object): LogicalSize
```

###### Parameters

| Parameter       | Type     |
|-----------------|----------|
| `object`        | `object` |
| `object.height` | `number` |
| `object.width`  | `number` |

###### Returns

[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L23>

#### Properties

| Property | Modifier | Type | Default value | Defined in |
|----|----|----|----|----|
|  `height` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L19> |
|  `type` | `readonly` | `"Logical"` | `'Logical'` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L17> |
|  `width` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L18> |

#### Methods

##### \_\_TAURI_TO_IPC_KEY\_\_()

```
__TAURI_TO_IPC_KEY__(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `height` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L66> |
| `width` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L65> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L63>

##### toJSON()

```
toJSON(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `height` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L66> |
| `width` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L65> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L70>

##### toPhysical()

```
toPhysical(scaleFactor): PhysicalSize
```

Converts the logical size to a physical one.

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)

###### Example

```
import { LogicalSize } from '@tauri-apps/api/dpi';import { getCurrentWindow } from '@tauri-apps/api/window';
const appWindow = getCurrentWindow();const factor = await appWindow.scaleFactor();const size = new LogicalSize(400, 500);const physical = size.toPhysical(factor);
```

###### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L59>

------------------------------------------------------------------------

### PhysicalPosition

A position represented in physical pixels.

For an explanation of what physical pixels are, see description of
[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize).

#### Since

2.0.0

#### Constructors

##### new PhysicalPosition()

```
new PhysicalPosition(x, y): PhysicalPosition
```

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `x`       | `number` |
| `y`       | `number` |

###### Returns

[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L286>

##### new PhysicalPosition()

```
new PhysicalPosition(object): PhysicalPosition
```

###### Parameters

| Parameter           | Type     |
|---------------------|----------|
| `object`            | `object` |
| `object.Physical`   | `object` |
| `object.Physical.x` | `number` |
| `object.Physical.y` | `number` |

###### Returns

[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L287>

##### new PhysicalPosition()

```
new PhysicalPosition(object): PhysicalPosition
```

###### Parameters

| Parameter  | Type     |
|------------|----------|
| `object`   | `object` |
| `object.x` | `number` |
| `object.y` | `number` |

###### Returns

[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L288>

#### Properties

| Property | Modifier | Type | Default value | Defined in |
|----|----|----|----|----|
|  `type` | `readonly` | `"Physical"` | `'Physical'` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L282> |
|  `x` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L283> |
|  `y` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L284> |

#### Methods

##### \_\_TAURI_TO_IPC_KEY\_\_()

```
__TAURI_TO_IPC_KEY__(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `x` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L330> |
| `y` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L331> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L328>

##### toJSON()

```
toJSON(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `x` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L330> |
| `y` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L331> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L335>

##### toLogical()

```
toLogical(scaleFactor): LogicalPosition
```

Converts the physical position to a logical one.

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition)

###### Example

```
import { PhysicalPosition } from '@tauri-apps/api/dpi';import { getCurrentWindow } from '@tauri-apps/api/window';
const appWindow = getCurrentWindow();const factor = await appWindow.scaleFactor();const position = new PhysicalPosition(400, 500);const physical = position.toLogical(factor);
```

###### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L324>

------------------------------------------------------------------------

### PhysicalSize

A size represented in physical pixels.

Physical pixels represent actual screen pixels, and are DPI-independent.
For high-DPI windows, this means that any point in the window on the
screen will have a different position in logical pixels
[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize).

For physical-pixel-based position, see
[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition).

#### Since

2.0.0

#### Constructors

##### new PhysicalSize()

```
new PhysicalSize(width, height): PhysicalSize
```

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `width`   | `number` |
| `height`  | `number` |

###### Returns

[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L92>

##### new PhysicalSize()

```
new PhysicalSize(object): PhysicalSize
```

###### Parameters

| Parameter                | Type     |
|--------------------------|----------|
| `object`                 | `object` |
| `object.Physical`        | `object` |
| `object.Physical.height` | `number` |
| `object.Physical.width`  | `number` |

###### Returns

[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L93>

##### new PhysicalSize()

```
new PhysicalSize(object): PhysicalSize
```

###### Parameters

| Parameter       | Type     |
|-----------------|----------|
| `object`        | `object` |
| `object.height` | `number` |
| `object.width`  | `number` |

###### Returns

[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L94>

#### Properties

| Property | Modifier | Type | Default value | Defined in |
|----|----|----|----|----|
|  `height` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L90> |
|  `type` | `readonly` | `"Physical"` | `'Physical'` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L88> |
|  `width` | `public` | `number` | `undefined` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L89> |

#### Methods

##### \_\_TAURI_TO_IPC_KEY\_\_()

```
__TAURI_TO_IPC_KEY__(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `height` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L133> |
| `width` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L132> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L130>

##### toJSON()

```
toJSON(): object
```

###### Returns

`object`

| Name | Type | Defined in |
|----|----|----|
| `height` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L133> |
| `width` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L132> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L137>

##### toLogical()

```
toLogical(scaleFactor): LogicalSize
```

Converts the physical size to a logical one.

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize)

###### Example

```
import { getCurrentWindow } from '@tauri-apps/api/window';const appWindow = getCurrentWindow();const factor = await appWindow.scaleFactor();const size = await appWindow.innerSize(); // PhysicalSizeconst logical = size.toLogical(factor);
```

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L126>

------------------------------------------------------------------------

### Position

A position represented either in physical or in logical pixels.

This type is basically a union type of
[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) and
[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)
but comes in handy when using `tauri::Position` in Rust as an argument
to a command, as this class automatically serializes into a valid format
so it can be deserialized correctly into `tauri::Position`

So instead of

```
import { invoke } from '@tauri-apps/api/core';import { LogicalPosition, PhysicalPosition } from '@tauri-apps/api/dpi';
const position: LogicalPosition | PhysicalPosition = someFunction(); // where someFunction returns either LogicalPosition or PhysicalPositionconst validPosition = position instanceof LogicalPosition  ? { Logical: { x: position.x, y: position.y } }  : { Physical: { x: position.x, y: position.y } }await invoke("do_something_with_position", { position: validPosition });
```

You can just use
[`Position`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#position)

```
import { invoke } from '@tauri-apps/api/core';import { LogicalPosition, PhysicalPosition, Position } from '@tauri-apps/api/dpi';
const position: LogicalPosition | PhysicalPosition = someFunction(); // where someFunction returns either LogicalPosition or PhysicalPositionconst validPosition = new Position(position);await invoke("do_something_with_position", { position: validPosition });
```

#### Since

2.1.0

#### Constructors

##### new Position()

```
new Position(position): Position
```

###### Parameters

| Parameter | Type |
|----|----|
| `position` | [`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition) \| [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) |

###### Returns

[`Position`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#position)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L375>

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `position` | [`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition) \| [`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition) | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L373> |

#### Methods

##### \_\_TAURI_TO_IPC_KEY\_\_()

```
__TAURI_TO_IPC_KEY__(): object
```

###### Returns

`object`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L391>

##### toJSON()

```
toJSON(): object
```

###### Returns

`object`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L400>

##### toLogical()

```
toLogical(scaleFactor): LogicalPosition
```

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`LogicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L379>

##### toPhysical()

```
toPhysical(scaleFactor): PhysicalPosition
```

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`PhysicalPosition`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalposition)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L385>

------------------------------------------------------------------------

### Size

A size represented either in physical or in logical pixels.

This type is basically a union type of
[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) and
[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)
but comes in handy when using `tauri::Size` in Rust as an argument to a
command, as this class automatically serializes into a valid format so
it can be deserialized correctly into `tauri::Size`

So instead of

```
import { invoke } from '@tauri-apps/api/core';import { LogicalSize, PhysicalSize } from '@tauri-apps/api/dpi';
const size: LogicalSize | PhysicalSize = someFunction(); // where someFunction returns either LogicalSize or PhysicalSizeconst validSize = size instanceof LogicalSize  ? { Logical: { width: size.width, height: size.height } }  : { Physical: { width: size.width, height: size.height } }await invoke("do_something_with_size", { size: validSize });
```

You can just use [`Size`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#size)

```
import { invoke } from '@tauri-apps/api/core';import { LogicalSize, PhysicalSize, Size } from '@tauri-apps/api/dpi';
const size: LogicalSize | PhysicalSize = someFunction(); // where someFunction returns either LogicalSize or PhysicalSizeconst validSize = new Size(size);await invoke("do_something_with_size", { size: validSize });
```

#### Since

2.1.0

#### Constructors

##### new Size()

```
new Size(size): Size
```

###### Parameters

| Parameter | Type |
|----|----|
| `size` | [`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) \| [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) |

###### Returns

[`Size`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#size)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L177>

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `size` | [`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize) \| [`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize) | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L175> |

#### Methods

##### \_\_TAURI_TO_IPC_KEY\_\_()

```
__TAURI_TO_IPC_KEY__(): object
```

###### Returns

`object`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L193>

##### toJSON()

```
toJSON(): object
```

###### Returns

`object`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L202>

##### toLogical()

```
toLogical(scaleFactor): LogicalSize
```

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`LogicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#logicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L181>

##### toPhysical()

```
toPhysical(scaleFactor): PhysicalSize
```

###### Parameters

| Parameter     | Type     |
|---------------|----------|
| `scaleFactor` | `number` |

###### Returns

[`PhysicalSize`](https://v2.tauri.app/reference/javascript/api/namespacedpi/#physicalsize)

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L187>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
