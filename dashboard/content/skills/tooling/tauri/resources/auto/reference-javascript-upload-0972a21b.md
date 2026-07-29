+++
title = "reference-javascript-upload-0972a21b"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# @tauri-apps/plugin-upload

## Enumerations

### HttpMethod

#### Enumeration Members

##### Patch

```
Patch: "PATCH";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L19>

##### Post

```
Post: "POST";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L17>

##### Put

```
Put: "PUT";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L18>

## Functions

### download()

```
function download(   url,   filePath,   progressHandler?,   headers?,body?): Promise<void>
```

#### Parameters

| Parameter | Type |
|----|----|
| `url` | `string` |
| `filePath` | `string` |
| `progressHandler`? | `ProgressHandler` |
| `headers`? | [`Map`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map)\<`string`, `string`\> |
| `body`? | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L53>

------------------------------------------------------------------------

### upload()

```
function upload(   url,   filePath,   progressHandler?,   headers?,method?): Promise<string>
```

#### Parameters

| Parameter | Type |
|----|----|
| `url` | `string` |
| `filePath` | `string` |
| `progressHandler`? | `ProgressHandler` |
| `headers`? | [`Map`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Map)\<`string`, `string`\> |
| `method`? | [`HttpMethod`](https://v2.tauri.app/reference/javascript/upload/#httpmethod) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L22>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
