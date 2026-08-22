+++
title = "reference-javascript-dialog-ab35f0f7"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-dialog

## Interfaces

### ConfirmDialogOptions

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `cancelLabel?` | `string` | The label of the cancel button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L293> |
|  `kind?` | `"info"` \| `"warning"` \| `"error"` | The kind of the dialog. Defaults to `info`. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L289> |
|  `okLabel?` | `string` | The label of the confirm button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L291> |
|  `title?` | `string` | The title of the dialog. Defaults to the app name. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L287> |

------------------------------------------------------------------------

### DialogFilter

Extension filters for the file dialog.

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `extensions` | `string`\[\] | Extensions to filter, without a `.` prefix. **Note:** Mobile platforms have different APIs for filtering that may not support extensions. iOS: Extensions are supported in the document picker, but not in the media picker. Android: Extensions are not supported. For these platforms, MIME types are the primary way to filter files, as opposed to extensions. This means the string values here labeled as `extensions` may also be a MIME type. This property name of `extensions` is being kept for backwards compatibility, but this may be revisited to specify the difference between extension or MIME type filtering. **Example** `extensions: ['svg', 'png']` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L32> |
|  `name` | `string` | Filter name. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L14> |

------------------------------------------------------------------------

### MessageDialogOptions

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `buttons?` | [`MessageDialogButtons`](https://v2.tauri.app/reference/javascript/dialog/#messagedialogbuttons) | The buttons of the dialog. **Example** `// Use system default buttons texts await message('Hello World!', { buttons: 'Ok' }) await message('Hello World!', { buttons: 'OkCancel' }) // Or with custom button texts await message('Hello World!', { buttons: { ok: 'Yes!' } }) await message('Take on the task?', { buttons: { ok: 'Accept', cancel: 'Cancel' } }) await message('Show the file content?', { buttons: { yes: 'Show content', no: 'Show in folder', cancel: 'Cancel' } })` **Since** 2.4.0 | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L259> |
|  `kind?` | `"info"` \| `"warning"` \| `"error"` | The kind of the dialog. Defaults to `info`. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L230> |
|  ~~`okLabel?`~~ | `string` | The label of the Ok button. **Deprecated** Use [`MessageDialogOptions.buttons`](https://v2.tauri.app/reference/javascript/dialog/#buttons) instead. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L236> |
|  `title?` | `string` | The title of the dialog. Defaults to the app name. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L228> |

------------------------------------------------------------------------

### OpenDialogOptions

Options for the open dialog.

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `canCreateDirectories?` | `boolean` | Whether to allow creating directories in the dialog. Enabled by default. **macOS Only** | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L71> |
|  `defaultPath?` | `string` | Initial directory or file path. If it’s a directory path, the dialog interface will change to that folder. If it’s not an existing directory, the file name will be set to the dialog’s file name input and the dialog will be set to the parent folder. On mobile the file name is always used on the dialog’s file name input. If not provided, Android uses `(invalid).txt` as default file name. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L60> |
|  `directory?` | `boolean` | Whether the dialog is a directory selection or not. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L64> |
|  `fileAccessMode?` | [`FileAccessMode`](https://v2.tauri.app/reference/javascript/dialog/#fileaccessmode-1) | The file access mode of the dialog. If not provided, `copy` is used, which matches the behavior of the [`open`](https://v2.tauri.app/reference/javascript/dialog/#open) method before the introduction of this option. **Usage** If a file is opened with [`: 'copy'`](https://v2.tauri.app/reference/javascript/dialog/#fileaccessmode), it will be copied to the app’s sandbox. This means the file can be read, edited, deleted, copied, or any other operation without any issues, since the file now belongs to the app. This also means that the caller has responsibility of deleting the file if this file is not meant to be retained in the app sandbox. If a file is opened with [`: 'scoped'`](https://v2.tauri.app/reference/javascript/dialog/#fileaccessmode), the file will remain in its original location and security-scoped access will be automatically managed by the system. **Note** This is specifically meant for document pickers on iOS or MacOS, in conjunction with [security scoped resources](https://developer.apple.com/documentation/foundation/nsurl/startaccessingsecurityscopedresource()). Why only document pickers, and not image or video pickers? The image and video pickers on iOS behave differently from the document pickers, and return [NSItemProvider](https://developer.apple.com/documentation/foundation/nsitemprovider) objects instead of file URLs. These are meant to be ephemeral (only available within the callback of the picker), and are not accessible outside of the callback. So for image and video pickers, the only way to access the file is to copy it to the app’s sandbox, and this is the URL that is returned from this API. This means there is no provision for using `scoped` mode with image or video pickers. If an image or video picker is used, `copy` is always used. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L103> |
|  `filters?` | [`DialogFilter`](https://v2.tauri.app/reference/javascript/dialog/#dialogfilter)\[\] | The filters of the dialog. On mobile platforms, if either: A) the [`pickerMode`](https://v2.tauri.app/reference/javascript/dialog/#pickermode) is set to `media`, `image`, or `video` – or – B) the filters include **only** either image or video mime types, the media picker will be displayed. Otherwise, the document picker will be displayed. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L51> |
|  `multiple?` | `boolean` | Whether the dialog allows multiple selection or not. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L62> |
|  `pickerMode?` | [`PickerMode`](https://v2.tauri.app/reference/javascript/dialog/#pickermode-1) | The preferred mode of the dialog. This is meant for mobile platforms (iOS and Android) which have distinct file and media pickers. If not provided, the dialog will automatically choose the best mode based on the MIME types or extensions of the [`filters`](https://v2.tauri.app/reference/javascript/dialog/#filters). On desktop, this option is ignored. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L78> |
|  `recursive?` | `boolean` | If `directory` is true, indicates that it will be read recursively later. Defines whether subdirectories will be allowed on the scope or not. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L69> |
|  `title?` | `string` | The title of the dialog window (desktop only). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L42> |

------------------------------------------------------------------------

### SaveDialogOptions

Options for the save dialog.

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `canCreateDirectories?` | `boolean` | Whether to allow creating directories in the dialog. Enabled by default. **macOS Only** | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L126> |
|  `defaultPath?` | `string` | Initial directory or file path. If it’s a directory path, the dialog interface will change to that folder. If it’s not an existing directory, the file name will be set to the dialog’s file name input and the dialog will be set to the parent folder. On mobile the file name is always used on the dialog’s file name input. If not provided, Android uses `(invalid).txt` as default file name. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L124> |
|  `filters?` | [`DialogFilter`](https://v2.tauri.app/reference/javascript/dialog/#dialogfilter)\[\] | The filters of the dialog. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L115> |
|  `title?` | `string` | The title of the dialog window (desktop only). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L113> |

## Type Aliases

### FileAccessMode

```
type FileAccessMode: "copy" | "scoped";
```

The file access mode of the dialog.

- `copy`: copy/move the picked file to the app sandbox; no scoped access
  required.
- `scoped`: keep file in place; security-scoped access is automatically
  managed.

**Note:** This option is only supported on iOS 14 and above. This
parameter is ignored on iOS 13 and below.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L147>

------------------------------------------------------------------------

### MessageDialogButtons

```
type MessageDialogButtons: MessageDialogDefaultButtons | MessageDialogCustomButtons;
```

The buttons of a message dialog.

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L219>

------------------------------------------------------------------------

### MessageDialogButtonsOk

```
type MessageDialogButtonsOk: object & BanExcept<"ok">;
```

The Ok button of a message dialog.

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `ok` | `string` | The Ok button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L201> |

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L199>

------------------------------------------------------------------------

### MessageDialogButtonsOkCancel

```
type MessageDialogButtonsOkCancel: object & BanExcept<"ok" | "cancel">;
```

The Ok and Cancel buttons of a message dialog.

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `cancel` | `string` | The Cancel button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L191> |
| `ok` | `string` | The Ok button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L189> |

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L187>

------------------------------------------------------------------------

### MessageDialogButtonsYesNoCancel

```
type MessageDialogButtonsYesNoCancel: object & BanExcept<"yes" | "no" | "cancel">;
```

The Yes, No and Cancel buttons of a message dialog.

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `cancel` | `string` | The Cancel button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L179> |
| `no` | `string` | The No button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L177> |
| `yes` | `string` | The Yes button. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L175> |

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L173>

------------------------------------------------------------------------

### MessageDialogCustomButtons

```
type MessageDialogCustomButtons: MessageDialogButtonsYesNoCancel | MessageDialogButtonsOkCancel | MessageDialogButtonsOk;
```

Custom buttons for a message dialog.

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L209>

------------------------------------------------------------------------

### MessageDialogDefaultButtons

```
type MessageDialogDefaultButtons: "Ok" | "OkCancel" | "YesNo" | "YesNoCancel";
```

Default buttons for a message dialog.

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L154>

------------------------------------------------------------------------

### MessageDialogResult

```
type MessageDialogResult:  | "Yes"  | "No"  | "Ok"  | "Cancel"  | string & object;
```

The result of a message dialog.

The result is a string if the dialog has custom buttons, otherwise it is
one of the default buttons.

#### Since

2.4.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L406>

------------------------------------------------------------------------

### OpenDialogReturn\<T\>

```
type OpenDialogReturn<T>: T["directory"] extends true ? T["multiple"] extends true ? string[] | null : string | null : T["multiple"] extends true ? string[] | null : string | null;
```

#### Type Parameters

| Type Parameter |
|----|
| `T` *extends* [`OpenDialogOptions`](https://v2.tauri.app/reference/javascript/dialog/#opendialogoptions) |

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L296>

------------------------------------------------------------------------

### PickerMode

```
type PickerMode: "document" | "media" | "image" | "video";
```

The preferred mode of the dialog. This is meant for mobile platforms
(iOS and Android) which have distinct file and media pickers. On
desktop, this option is ignored. If not provided, the dialog will
automatically choose the best mode based on the MIME types or extensions
of the filters.

**Note:** This option is only supported on iOS 14 and above. This
parameter is ignored on iOS 13 and below.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L137>

## Functions

### ask()

```
function ask(message, options?): Promise<boolean>
```

Shows a question dialog with `Yes` and `No` buttons.

Convenient wrapper for
`await message('msg', { buttons: 'YesNo' }) === 'Yes'`

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | The message to show. |
| `options`? | `string` \| [`ConfirmDialogOptions`](https://v2.tauri.app/reference/javascript/dialog/#confirmdialogoptions) | The dialog’s options. If a string, it represents the dialog title. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

A promise resolving to a boolean indicating whether `Yes` was clicked or
not.

#### Example

```
import { ask } from '@tauri-apps/plugin-dialog';const yes = await ask('Are you sure?', 'Tauri');const yes2 = await ask('This action cannot be reverted. Are you sure?', { title: 'Tauri', kind: 'warning' });
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L467>

------------------------------------------------------------------------

### confirm()

```
function confirm(message, options?): Promise<boolean>
```

Shows a question dialog with `Ok` and `Cancel` buttons.

Convenient wrapper for
`await message('msg', { buttons: 'OkCancel' }) === 'Ok'`

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | The message to show. |
| `options`? | `string` \| [`ConfirmDialogOptions`](https://v2.tauri.app/reference/javascript/dialog/#confirmdialogoptions) | The dialog’s options. If a string, it represents the dialog title. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

A promise resolving to a boolean indicating whether `Ok` was clicked or
not.

#### Example

```
import { confirm } from '@tauri-apps/plugin-dialog';const confirmed = await confirm('Are you sure?', 'Tauri');const confirmed2 = await confirm('This action cannot be reverted. Are you sure?', { title: 'Tauri', kind: 'warning' });
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L504>

------------------------------------------------------------------------

### message()

```
function message(message, options?): Promise<MessageDialogResult>
```

Shows a message dialog with an `Ok` button.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `message` | `string` | The message to show. |
| `options`? | `string` \| [`MessageDialogOptions`](https://v2.tauri.app/reference/javascript/dialog/#messagedialogoptions) | The dialog’s options. If a string, it represents the dialog title. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`MessageDialogResult`](https://v2.tauri.app/reference/javascript/dialog/#messagedialogresult)\>

A promise indicating the success or failure of the operation.

#### Example

```
import { message } from '@tauri-apps/plugin-dialog';await message('Tauri is awesome', 'Tauri');await message('File not found', { title: 'Tauri', kind: 'error' });
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L437>

------------------------------------------------------------------------

### open()

```
function open<T>(options): Promise<OpenDialogReturn<T>>
```

Open a file/directory selection dialog.

The selected paths are added to the filesystem and asset protocol
scopes. When security is more important than the easy of use of this
API, prefer writing a dedicated command instead.

Note that the scope change is not persisted, so the values are cleared
when the application is restarted. You can save it to the filesystem
using
[tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).

#### Type Parameters

| Type Parameter |
|----|
| `T` *extends* [`OpenDialogOptions`](https://v2.tauri.app/reference/javascript/dialog/#opendialogoptions) |

#### Parameters

| Parameter | Type |
|-----------|------|
| `options` | `T`  |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`OpenDialogReturn`](https://v2.tauri.app/reference/javascript/dialog/#opendialogreturnt)\<`T`\>\>

A promise resolving to the selected path(s)

#### Examples

```
import { open } from '@tauri-apps/plugin-dialog';// Open a selection dialog for image filesconst selected = await open({  multiple: true,  filters: [{    name: 'Image',    extensions: ['png', 'jpeg']  }]});if (Array.isArray(selected)) {  // user selected multiple files} else if (selected === null) {  // user cancelled the selection} else {  // user selected a single file}
```

```
import { open } from '@tauri-apps/plugin-dialog';import { appDir } from '@tauri-apps/api/path';// Open a selection dialog for directoriesconst selected = await open({  directory: true,  multiple: true,  defaultPath: await appDir(),});if (Array.isArray(selected)) {  // user selected multiple directories} else if (selected === null) {  // user cancelled the selection} else {  // user selected a single directory}
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L356>

------------------------------------------------------------------------

### save()

```
function save(options): Promise<string | null>
```

Open a file/directory save dialog.

The selected path is added to the filesystem and asset protocol scopes.
When security is more important than the easy of use of this API, prefer
writing a dedicated command instead.

Note that the scope change is not persisted, so the values are cleared
when the application is restarted. You can save it to the filesystem
using
[tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).

#### Parameters

| Parameter | Type |
|----|----|
| `options` | [`SaveDialogOptions`](https://v2.tauri.app/reference/javascript/dialog/#savedialogoptions) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`
\| `null`\>

A promise resolving to the selected path.

#### Example

```
import { save } from '@tauri-apps/plugin-dialog';const filePath = await save({  filters: [{    name: 'Image',    extensions: ['png', 'jpeg']  }]});
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L390>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

