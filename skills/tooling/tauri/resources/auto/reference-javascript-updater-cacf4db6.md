# @tauri-apps/plugin-updater

## Classes

### Update

#### Extends

- `Resource`

#### Constructors

##### new Update()

```
new Update(metadata): Update
```

###### Parameters

| Parameter  | Type             |
|------------|------------------|
| `metadata` | `UpdateMetadata` |

###### Returns

[`Update`](https://v2.tauri.app/reference/javascript/updater/#update)

###### Overrides

`Resource.constructor`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L77>

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  ~~`available`~~ | `boolean` | **Deprecated** This is always true, check if the return value is `null` instead when using [`check`](https://v2.tauri.app/reference/javascript/updater/#check) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L69> |
|  `body?` | `string` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L73> |
|  `currentVersion` | `string` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L70> |
|  `date?` | `string` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L72> |
|  `rawJson` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `unknown`\> | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L74> |
|  `version` | `string` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L71> |

#### Accessors

##### rid

```
get rid(): number
```

###### Returns

`number`

###### Inherited from

`Resource.rid`

**Source**: undefined

#### Methods

##### close()

```
close(): Promise<void>
```

Destroys and cleans up this resource from memory. **You should not call
any method on this object anymore and should drop any reference to it.**

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Overrides

`Resource.close`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L152>

##### download()

```
download(onEvent?, options?): Promise<void>
```

Download the updater package. Call
[`install`](https://v2.tauri.app/reference/javascript/updater/#install) later to install it

###### Parameters

| Parameter | Type |
|----|----|
| `onEvent`? | (`progress`) =\> `void` |
| `options`? | [`DownloadOptions`](https://v2.tauri.app/reference/javascript/updater/#downloadoptions) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L88>

##### downloadAndInstall()

```
downloadAndInstall(onEvent?, options?): Promise<void>
```

Downloads the updater package and installs it

## Platform-specific:

- **Windows:** This function exits the app after launching the updater
  installer successfully
- **macOS / Linux:** You need to relaunch the app to run the newly
  install version

###### Parameters

| Parameter | Type |
|----|----|
| `onEvent`? | (`progress`) =\> `void` |
| `options`? | [`DownloadOptions`](https://v2.tauri.app/reference/javascript/updater/#downloadoptions) & `InstallOptions` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L136>

##### install()

```
install(options?): Promise<void>
```

Install downloaded updater package. Must be called after
[`download`](https://v2.tauri.app/reference/javascript/updater/#download).

## Platform-specific:

- **Windows:** This function exits the app after launching the updater
  installer successfully
- **macOS / Linux:** You need to relaunch the app to run the newly
  install version

###### Parameters

| Parameter  | Type             |
|------------|------------------|
| `options`? | `InstallOptions` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L113>

## Interfaces

### CheckOptions

Options used when checking for updates

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `allowDowngrades?` | `boolean` | Allow downgrades to previous versions by not checking if the current version is greater than the available version. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L28> |
|  `headers?` | `HeadersInit` | Request headers | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L12> |
|  `proxy?` | `string` | A proxy url to be used when checking and downloading updates. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L20> |
|  `target?` | `string` | Target identifier for the running application. This is sent to the backend. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L24> |
|  `timeout?` | `number` | Timeout in milliseconds | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L16> |

------------------------------------------------------------------------

### DownloadOptions

Options used when downloading an update

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `headers?` | `HeadersInit` | Request headers | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L36> |
|  `timeout?` | `number` | Timeout in milliseconds | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L40> |

## Type Aliases

### DownloadEvent

```
type DownloadEvent: object | object | object;
```

Updater download event

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L61>

## Functions

### check()

```
function check(options?): Promise<Update | null>
```

Check for updates, resolves to `null` if no updates are available

#### Parameters

| Parameter  | Type                                                          |
|------------|---------------------------------------------------------------|
| `options`? | [`CheckOptions`](https://v2.tauri.app/reference/javascript/updater/#checkoptions) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Update`](https://v2.tauri.app/reference/javascript/updater/#update)
\| `null`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L159>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
