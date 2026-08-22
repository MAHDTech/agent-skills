+++
title = "reference-javascript-biometric-ff8295eb"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-biometric

## Enumerations

### BiometryType

#### Enumeration Members

##### FaceID

```
FaceID: 2;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L12>

##### Iris

```
Iris: 3;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L14>

##### None

```
None: 0;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L8>

##### TouchID

```
TouchID: 1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L10>

## Interfaces

### AuthOptions

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `allowDeviceCredential?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L36> |
|  `cancelTitle?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L37> |
|  `confirmationRequired?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L45> |
|  `fallbackTitle?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L40> |
|  `maxAttemps?` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L46> |
|  `subtitle?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L44> |
|  `title?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L43> |

------------------------------------------------------------------------

### Status

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `biometryType` | [`BiometryType`](https://v2.tauri.app/reference/javascript/biometric/#biometrytype) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L19> |
|  `error?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L20> |
|  `errorCode?` | \| `"appCancel"` \| `"authenticationFailed"` \| `"invalidContext"` \| `"notInteractive"` \| `"passcodeNotSet"` \| `"systemCancel"` \| `"userCancel"` \| `"userFallback"` \| `"biometryLockout"` \| `"biometryNotAvailable"` \| `"biometryNotEnrolled"` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L21> |
|  `isAvailable` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L18> |

## Functions

### authenticate()

```
function authenticate(reason, options?): Promise<void>
```

Prompts the user for authentication using the system interface (touchID,
faceID or Android Iris). Rejects if the authentication fails.

```
import { authenticate } from "@tauri-apps/plugin-biometric";await authenticate('Open your wallet');
```

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `reason` | `string` |  |
| `options`? | [`AuthOptions`](https://v2.tauri.app/reference/javascript/biometric/#authoptions) |  |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L69>

------------------------------------------------------------------------

### checkStatus()

```
function checkStatus(): Promise<Status>
```

Checks if the biometric authentication is available.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Status`](https://v2.tauri.app/reference/javascript/biometric/#status)\>

a promise resolving to an object containing all the information about
the status of the biometry.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L53>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

