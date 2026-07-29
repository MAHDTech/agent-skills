+++
title = "reference-javascript-barcode-scanner-f3bd478d"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# @tauri-apps/plugin-barcode-scanner

## Enumerations

### Format

#### Enumeration Members

##### Aztec

```
Aztec: "AZTEC";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L30>

##### Codabar

```
Codabar: "CODABAR";
```

Not supported on iOS.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L28>

##### Code128

```
Code128: "CODE_128";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L24>

##### Code39

```
Code39: "CODE_39";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L22>

##### Code93

```
Code93: "CODE_93";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L23>

##### DataMatrix

```
DataMatrix: "DATA_MATRIX";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L31>

##### EAN13

```
EAN13: "EAN_13";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L21>

##### EAN8

```
EAN8: "EAN_8";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L20>

##### GS1DataBar

```
GS1DataBar: "GS1_DATA_BAR";
```

Not supported on Android. Requires iOS 15.4+

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L36>

##### GS1DataBarExpanded

```
GS1DataBarExpanded: "GS1_DATA_BAR_EXPANDED";
```

Not supported on Android. Requires iOS 15.4+

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L44>

##### GS1DataBarLimited

```
GS1DataBarLimited: "GS1_DATA_BAR_LIMITED";
```

Not supported on Android. Requires iOS 15.4+

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L40>

##### ITF

```
ITF: "ITF";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L29>

##### PDF417

```
PDF417: "PDF_417";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L32>

##### QRCode

```
QRCode: "QR_CODE";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L14>

##### UPC_A

```
UPC_A: "UPC_A";
```

Not supported on iOS.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L18>

##### UPC_E

```
UPC_E: "UPC_E";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L19>

## Interfaces

### ScanOptions

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `cameraDirection?` | `"back"` \| `"front"` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L48> |
|  `formats?` | [`Format`](https://v2.tauri.app/reference/javascript/barcode-scanner/#format)\[\] | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L49> |
|  `windowed?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L50> |

------------------------------------------------------------------------

### Scanned

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `bounds` | `unknown` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L56> |
|  `content` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L54> |
|  `format` | [`Format`](https://v2.tauri.app/reference/javascript/barcode-scanner/#format) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L55> |

## Type Aliases

### PermissionState

```
type PermissionState: "granted" | "denied" | "prompt" | "prompt-with-rationale";
```

**Source**: undefined

## Functions

### cancel()

```
function cancel(): Promise<void>
```

Cancel the current scan process.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L70>

------------------------------------------------------------------------

### checkPermissions()

```
function checkPermissions(): Promise<PermissionState>
```

Get permission state.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`PermissionState`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L77>

------------------------------------------------------------------------

### openAppSettings()

```
function openAppSettings(): Promise<void>
```

Open application settings. Useful if permission was denied and the user
must manually enable it.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L95>

------------------------------------------------------------------------

### requestPermissions()

```
function requestPermissions(): Promise<PermissionState>
```

Request permissions to use the camera.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`PermissionState`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L86>

------------------------------------------------------------------------

### scan()

```
function scan(options?): Promise<Scanned>
```

Start scanning.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `options`? | [`ScanOptions`](https://v2.tauri.app/reference/javascript/barcode-scanner/#scanoptions) |  |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Scanned`](https://v2.tauri.app/reference/javascript/barcode-scanner/#scanned)\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L63>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
