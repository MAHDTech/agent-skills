+++
title = "reference-javascript-process-3c314b5a"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-process

Perform operations on the current process.

## Functions

### exit()

```
function exit(code): Promise<void>
```

Exits immediately with the given `exitCode`.

#### Parameters

| Parameter | Type     | Default value | Description           |
|-----------|----------|---------------|-----------------------|
| `code`    | `number` | `0`           | The exit code to use. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { exit } from '@tauri-apps/plugin-process';await exit(1);
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/process/guest-js/index.ts#L25>

------------------------------------------------------------------------

### relaunch()

```
function relaunch(): Promise<void>
```

Exits the current instance of the app then relaunches it.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { relaunch } from '@tauri-apps/plugin-process';await relaunch();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/process/guest-js/index.ts#L41>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

