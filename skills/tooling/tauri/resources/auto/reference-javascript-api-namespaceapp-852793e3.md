# app

## Enumerations

### BundleType

Bundle type of the current application.

#### Enumeration Members

##### App

```
App: "app";
```

macOS app bundle

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L48>

##### AppImage

```
AppImage: "appimage";
```

Linux AppImage

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L46>

##### Deb

```
Deb: "deb";
```

Linux Debian package

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L42>

##### Msi

```
Msi: "msi";
```

Windows MSI

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L40>

##### Nsis

```
Nsis: "nsis";
```

Windows NSIS

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L38>

##### Rpm

```
Rpm: "rpm";
```

Linux RPM

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L44>

## Type Aliases

### DataStoreIdentifier

```
type DataStoreIdentifier: [number, number, number, number, number, number, number, number, number, number, number, number, number, number, number, number];
```

Identifier type used for data stores on macOS and iOS.

Represents a 128-bit identifier, commonly expressed as a 16-byte UUID.

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L14>

------------------------------------------------------------------------

### OnBackButtonPressPayload

```
type OnBackButtonPressPayload: object;
```

Payload for the onBackButtonPress event.

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `canGoBack` | `boolean` | Whether the webview canGoBack property is true. | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L260> |

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L258>

## Functions

### defaultWindowIcon()

```
function defaultWindowIcon(): Promise<Image | null>
```

Gets the default window icon.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Image`](https://v2.tauri.app/reference/javascript/api/namespaceimage/#image)
\| `null`\>

#### Example

```
import { defaultWindowIcon } from '@tauri-apps/api/app';const icon = await defaultWindowIcon();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L197>

------------------------------------------------------------------------

### fetchDataStoreIdentifiers()

```
function fetchDataStoreIdentifiers(): Promise<DataStoreIdentifier[]>
```

Fetches the data store identifiers on macOS and iOS.

See
<https://developer.apple.com/documentation/webkit/wkwebsitedatastore>
for more information.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`DataStoreIdentifier`](https://v2.tauri.app/reference/javascript/api/namespaceapp/#datastoreidentifier)\[\]\>

#### Example

```
import { fetchDataStoreIdentifiers } from '@tauri-apps/api/app';const ids = await fetchDataStoreIdentifiers();
```

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L160>

------------------------------------------------------------------------

### getBundleType()

```
function getBundleType(): Promise<BundleType>
```

Gets the application bundle type.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`BundleType`](https://v2.tauri.app/reference/javascript/api/namespaceapp/#bundletype)\>

#### Example

```
import { getBundleType } from '@tauri-apps/api/app';const type = await getBundleType();
```

#### Since

2.5.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L251>

------------------------------------------------------------------------

### getIdentifier()

```
function getIdentifier(): Promise<string>
```

Gets the application identifier.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

The application identifier as configured in `tauri.conf.json`.

#### Example

```
import { getIdentifier } from '@tauri-apps/api/app';const identifier = await getIdentifier();
```

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L112>

------------------------------------------------------------------------

### getName()

```
function getName(): Promise<string>
```

Gets the application name.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { getName } from '@tauri-apps/api/app';const appName = await getName();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L81>

------------------------------------------------------------------------

### getTauriVersion()

```
function getTauriVersion(): Promise<string>
```

Gets the Tauri framework version used by this application.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { getTauriVersion } from '@tauri-apps/api/app';const tauriVersion = await getTauriVersion();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L96>

------------------------------------------------------------------------

### getVersion()

```
function getVersion(): Promise<string>
```

Gets the application version.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

#### Example

```
import { getVersion } from '@tauri-apps/api/app';const appVersion = await getVersion();
```

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L67>

------------------------------------------------------------------------

### hide()

```
function hide(): Promise<void>
```

Hides the application on macOS.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { hide } from '@tauri-apps/api/app';await hide();
```

#### Since

1.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L143>

------------------------------------------------------------------------

### onBackButtonPress()

```
function onBackButtonPress(handler): Promise<PluginListener>
```

Listens to the backButton event on Android.

#### Parameters

| Parameter | Type                   | Description |
|-----------|------------------------|-------------|
| `handler` | (`payload`) =\> `void` |             |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PluginListener`](https://v2.tauri.app/reference/javascript/api/namespacecore/#pluginlistener)\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L267>

------------------------------------------------------------------------

### removeDataStore()

```
function removeDataStore(uuid): Promise<void>
```

Removes the data store with the given identifier.

Note that any webview using this data store should be closed before
running this API.

See
<https://developer.apple.com/documentation/webkit/wkwebsitedatastore>
for more information.

#### Parameters

| Parameter | Type |
|----|----|
| `uuid` | [`DataStoreIdentifier`](https://v2.tauri.app/reference/javascript/api/namespaceapp/#datastoreidentifier) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { fetchDataStoreIdentifiers, removeDataStore } from '@tauri-apps/api/app';for (const id of (await fetchDataStoreIdentifiers())) {  await removeDataStore(id);}
```

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L181>

------------------------------------------------------------------------

### setDockVisibility()

```
function setDockVisibility(visible): Promise<void>
```

Sets the dock visibility for the application on macOS.

#### Parameters

| Parameter | Type      | Description                                |
|-----------|-----------|--------------------------------------------|
| `visible` | `boolean` | Whether the dock should be visible or not. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { setDockVisibility } from '@tauri-apps/api/app';await setDockVisibility(false);
```

#### Since

2.5.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L236>

------------------------------------------------------------------------

### setTheme()

```
function setTheme(theme?): Promise<void>
```

Sets the application’s theme. Pass in `null` or `undefined` to follow
the system theme.

#### Parameters

| Parameter | Type |
|----|----|
| `theme`? | `null` \| [`Theme`](https://v2.tauri.app/reference/javascript/api/namespacewindow/#theme-2) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { setTheme } from '@tauri-apps/api/app';await setTheme('dark');
```

Platform-specific

- **iOS / Android:** Unsupported.

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L219>

------------------------------------------------------------------------

### show()

```
function show(): Promise<void>
```

Shows the application on macOS. This function does not automatically
focus any specific app window.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

#### Example

```
import { show } from '@tauri-apps/api/app';await show();
```

#### Since

1.2.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L128>

------------------------------------------------------------------------

### supportsMultipleWindows()

```
function supportsMultipleWindows(): Promise<boolean>
```

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L277>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
