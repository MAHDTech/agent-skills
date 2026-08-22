+++
title = "reference-javascript-geolocation-460fa3c3"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-geolocation

## Type Aliases

### Coordinates

```
type Coordinates: object;
```

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `accuracy` | `number` | Accuracy level of the latitude and longitude coordinates in meters. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L24> |
| `altitude` | `number` \| `null` | The altitude the user is at, if available. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L33> |
| `altitudeAccuracy` | `number` \| `null` | Accuracy level of the altitude coordinate in meters, if available. Available on all iOS versions and on Android 8 and above. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L29> |
| `heading` | `number` \| `null` | The heading the user is facing, if available. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L38> |
| `latitude` | `number` | Latitude in decimal degrees. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L16> |
| `longitude` | `number` | Longitude in decimal degrees. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L20> |
| `speed` | `number` \| `null` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L34> |

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L12>

------------------------------------------------------------------------

### PermissionStatus

```
type PermissionStatus: object;
```

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `coarseLocation` | `PermissionState` | Permissions state for the coarseLoaction alias. On Android it requests/checks ACCESS_COARSE_LOCATION. On Android 12+, users can choose between Approximate location (ACCESS_COARSE_LOCATION) and Precise location (ACCESS_FINE_LOCATION). On iOS it will have the same value as the `location` alias. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L59> |
| `location` | `PermissionState` | Permission state for the location alias. On Android it requests/checks both ACCESS_COARSE_LOCATION and ACCESS_FINE_LOCATION permissions. On iOS it requests/checks location permissions. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L49> |

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L41>

------------------------------------------------------------------------

### PermissionType

```
type PermissionType: "location" | "coarseLocation";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L62>

------------------------------------------------------------------------

### Position

```
type Position: object;
```

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `coords` | [`Coordinates`](https://v2.tauri.app/reference/javascript/geolocation/#coordinates) | The GPD coordinates along with the accuracy of the data. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L72> |
| `timestamp` | `number` | Creation time for these coordinates. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L68> |

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L64>

------------------------------------------------------------------------

### PositionOptions

```
type PositionOptions: object;
```

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `enableHighAccuracy` | `boolean` | High accuracy mode (such as GPS, if available) Will be ignored on Android 12+ if users didn’t grant the ACCESS_FINE_LOCATION permission (`coarseLocation` permission). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L80> |
| `maximumAge` | `number` | The maximum age in milliseconds of a possible cached position that is acceptable to return. Default: 0 Ignored on iOS | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L92> |
| `timeout` | `number` | The maximum wait time in milliseconds for location updates. On Android the timeout gets ignored for getCurrentPosition. Ignored on iOS | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L86> |

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L75>

## Functions

### checkPermissions()

```
function checkPermissions(): Promise<PermissionStatus>
```

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PermissionStatus`](https://v2.tauri.app/reference/javascript/geolocation/#permissionstatus)\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L128>

------------------------------------------------------------------------

### clearWatch()

```
function clearWatch(channelId): Promise<void>
```

#### Parameters

| Parameter   | Type     |
|-------------|----------|
| `channelId` | `number` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L122>

------------------------------------------------------------------------

### getCurrentPosition()

```
function getCurrentPosition(options?): Promise<Position>
```

#### Parameters

| Parameter | Type |
|----|----|
| `options`? | [`PositionOptions`](https://v2.tauri.app/reference/javascript/geolocation/#positionoptions) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Position`](https://v2.tauri.app/reference/javascript/geolocation/#position)\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L114>

------------------------------------------------------------------------

### requestPermissions()

```
function requestPermissions(permissions): Promise<PermissionStatus>
```

#### Parameters

| Parameter | Type |
|----|----|
| `permissions` | `null` \| [`PermissionType`](https://v2.tauri.app/reference/javascript/geolocation/#permissiontype)\[\] |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PermissionStatus`](https://v2.tauri.app/reference/javascript/geolocation/#permissionstatus)\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L132>

------------------------------------------------------------------------

### watchPosition()

```
function watchPosition(options, cb): Promise<number>
```

#### Parameters

| Parameter | Type |
|----|----|
| `options` | [`PositionOptions`](https://v2.tauri.app/reference/javascript/geolocation/#positionoptions) |
| `cb` | (`location`, `error`?) =\> `void` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`number`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L95>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

