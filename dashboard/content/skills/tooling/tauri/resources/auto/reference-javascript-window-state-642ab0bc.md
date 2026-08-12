+++
title = "reference-javascript-window-state-642ab0bc"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# @tauri-apps/plugin-window-state

## Enumerations

### StateFlags

#### Enumeration Members

##### ALL

```
ALL: 63;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L15>

##### DECORATIONS

```
DECORATIONS: 16;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L13>

##### FULLSCREEN

```
FULLSCREEN: 32;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L14>

##### MAXIMIZED

```
MAXIMIZED: 4;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L11>

##### POSITION

```
POSITION: 2;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L10>

##### SIZE

```
SIZE: 1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L9>

##### VISIBLE

```
VISIBLE: 8;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L12>

## Functions

### filename()

```
function filename(): Promise<string>
```

Get the name of the file used to store window state.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L44>

------------------------------------------------------------------------

### restoreState()

```
function restoreState(label, flags?): Promise<void>
```

Restore the state for the specified window from disk.

#### Parameters

| Parameter | Type                                                           |
|-----------|----------------------------------------------------------------|
| `label`   | `string`                                                       |
| `flags`?  | [`StateFlags`](https://v2.tauri.app/reference/javascript/window-state/#stateflags) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L28>

------------------------------------------------------------------------

### restoreStateCurrent()

```
function restoreStateCurrent(flags?): Promise<void>
```

Restore the state for the current window from disk.

#### Parameters

| Parameter | Type                                                           |
|-----------|----------------------------------------------------------------|
| `flags`?  | [`StateFlags`](https://v2.tauri.app/reference/javascript/window-state/#stateflags) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L38>

------------------------------------------------------------------------

### saveWindowState()

```
function saveWindowState(flags?): Promise<void>
```

Save the state of all open windows to disk.

#### Parameters

| Parameter | Type                                                           |
|-----------|----------------------------------------------------------------|
| `flags`?  | [`StateFlags`](https://v2.tauri.app/reference/javascript/window-state/#stateflags) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L21>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
