# image

## Classes

### Image

An RGBA Image in row-major order from top to bottom.

#### Extends

- [`Resource`](https://v2.tauri.app/reference/javascript/api/namespacecore/#resource)

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

##### rgba()

```
rgba(): Promise<Uint8Array>
```

Returns the RGBA data for this image, in row-major order from top to
bottom.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L89>

##### size()

```
size(): Promise<ImageSize>
```

Returns the size of this image.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`ImageSize`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#imagesize)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L96>

##### fromBytes()

```
static fromBytes(bytes): Promise<Image>
```

Creates a new image using the provided bytes by inferring the file
format. If the format is known, prefer \[@link Image.fromPngBytes\] or
\[@link Image.fromIcoBytes\].

Only `ico` and `png` are supported (based on activated feature flag).

Note that you need the `image-ico` or `image-png` Cargo features to use
this API. To enable it, change your Cargo.toml file:

```
[dependencies]tauri = { version = "...", features = ["...", "image-png"] }
```

###### Parameters

| Parameter | Type |
|----|----|
| `bytes` | [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L62>

##### fromPath()

```
static fromPath(path): Promise<Image>
```

Creates a new image using the provided path.

Only `ico` and `png` are supported (based on activated feature flag).

Note that you need the `image-ico` or `image-png` Cargo features to use
this API. To enable it, change your Cargo.toml file:

```
[dependencies]tauri = { version = "...", features = ["...", "image-png"] }
```

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L82>

##### new()

```
static new(   rgba,   width,height): Promise<Image>
```

Creates a new Image using RGBA data, in row-major order from top to
bottom, and with specified width and height.

###### Parameters

| Parameter | Type |
|----|----|
| `rgba` | [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) |
| `width` | `number` |
| `height` | `number` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L37>

## Interfaces

### ImageSize

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `height` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L13> |
|  `width` | `number` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L11> |

## Type Aliases

### MenuIcon

```
type MenuIcon:  | NativeIcon  | string  | Image  | Uint8Array  | ArrayBuffer  | number[];
```

A type that represents an icon that can be used in menu items.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L17>

## Functions

### transformImage()

```
function transformImage<T>(image): T
```

Transforms image from various types into a type acceptable by Rust.

See
[tauri::image::JsImage](https://docs.rs/tauri/2/tauri/image/enum.JsImage.html)
for more information. Note the API signature is not stable and might
change.

#### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

#### Parameters

| Parameter | Type |
|----|----|
| `image` | \| `null` \| `string` \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) \| [`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image) |

#### Returns

`T`

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L107>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
