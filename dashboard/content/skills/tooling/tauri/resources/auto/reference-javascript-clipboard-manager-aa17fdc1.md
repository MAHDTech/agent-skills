+++
title = "reference-javascript-clipboard-manager-aa17fdc1"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# @tauri-apps/plugin-clipboard-manager

Read and write to the system clipboard.

## Functions

### clear()

```
function clear(): Promise<void>
```

Clears the clipboard.

Platform-specific

- **Android:** Only supported on SDK 28+. For older releases we write an
  empty string to the clipboard instead.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { clear } from '@tauri-apps/plugin-clipboard-manager';await clear();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L147>

------------------------------------------------------------------------

### readImage()

```
function readImage(): Promise<Image>
```

Gets the clipboard content as Uint8Array image.

Platform-specific

- **Android / iOS:** Not supported.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`Image`\>

#### Example

```
import { readImage } from '@tauri-apps/plugin-clipboard-manager';
const clipboardImage = await readImage();const blob = new Blob([await clipboardImage.rgba()], { type: 'image' })const url = URL.createObjectURL(blob)
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L99>

------------------------------------------------------------------------

### readText()

```
function readText(): Promise<string>
```

Gets the clipboard content as plain text.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { readText } from '@tauri-apps/plugin-clipboard-manager';const clipboardText = await readText();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L46>

------------------------------------------------------------------------

### writeHtml()

```
function writeHtml(html, altText?): Promise<void>
```

- Writes HTML or fallbacks to write provided plain text to the
  clipboard.

Platform-specific

- **Android / iOS:** Not supported.

#### Parameters

| Parameter  | Type     |
|------------|----------|
| `html`     | `string` |
| `altText`? | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { writeHtml } from '@tauri-apps/plugin-clipboard-manager';await writeHtml('<h1>Tauri is awesome!</h1>', 'plaintext');// The following will write "<h1>Tauri is awesome</h1>" as plain textawait writeHtml('<h1>Tauri is awesome!</h1>', '<h1>Tauri is awesome</h1>');// we can read html data only as a string so there's just readText(), no readHtml()assert(await readText(), '<h1>Tauri is awesome!</h1>');
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L126>

------------------------------------------------------------------------

### writeImage()

```
function writeImage(image): Promise<void>
```

Writes image buffer to the clipboard.

Platform-specific

- **Android / iOS:** Not supported.

#### Parameters

| Parameter | Type |
|----|----|
| `image` | \| `string` \| `number`\[\] \| [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) \| `Image` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { writeImage } from '@tauri-apps/plugin-clipboard-manager';const buffer = [  // A red pixel  255, 0, 0, 255,
 // A green pixel  0, 255, 0, 255,];await writeImage(buffer);
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L74>

------------------------------------------------------------------------

### writeText()

```
function writeText(text, opts?): Promise<void>
```

Writes plain text to the clipboard.

#### Parameters

| Parameter     | Type     |
|---------------|----------|
| `text`        | `string` |
| `opts`?       | `object` |
| `opts.label`? | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';await writeText('Tauri is awesome!');assert(await readText(), 'Tauri is awesome!');
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L27>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
