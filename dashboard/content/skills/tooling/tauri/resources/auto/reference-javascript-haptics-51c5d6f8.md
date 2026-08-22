+++
title = "reference-javascript-haptics-51c5d6f8"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-haptics

## Type Aliases

### ImpactFeedbackStyle

```
type ImpactFeedbackStyle:  | "light"  | "medium"  | "heavy"  | "soft"  | "rigid";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/bindings.ts#L76>

------------------------------------------------------------------------

### NotificationFeedbackType

```
type NotificationFeedbackType: "success" | "warning" | "error";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/bindings.ts#L82>

## Functions

### impactFeedback()

```
function impactFeedback(style): Promise<Result<null, never>>
```

#### Parameters

| Parameter | Type |
|----|----|
| `style` | [`ImpactFeedbackStyle`](https://v2.tauri.app/reference/javascript/haptics/#impactfeedbackstyle) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`Result`\<`null`,
`never`\>\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L11>

------------------------------------------------------------------------

### notificationFeedback()

```
function notificationFeedback(type): Promise<Result<null, never>>
```

#### Parameters

| Parameter | Type |
|----|----|
| `type` | [`NotificationFeedbackType`](https://v2.tauri.app/reference/javascript/haptics/#notificationfeedbacktype) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`Result`\<`null`,
`never`\>\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L12>

------------------------------------------------------------------------

### selectionFeedback()

```
function selectionFeedback(): Promise<Result<null, never>>
```

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`Result`\<`null`,
`never`\>\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L13>

------------------------------------------------------------------------

### vibrate()

```
function vibrate(duration): Promise<Result<null, never>>
```

#### Parameters

| Parameter  | Type     |
|------------|----------|
| `duration` | `number` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`Result`\<`null`,
`never`\>\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L10>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

