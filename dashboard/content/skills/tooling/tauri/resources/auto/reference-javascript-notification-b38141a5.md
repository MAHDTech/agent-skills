+++
title = "reference-javascript-notification-b38141a5"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-notification

Send toast notifications (brief auto-expiring OS window element) to your
user. Can also be used with the Notification Web API.

## Enumerations

### Importance

#### Enumeration Members

##### Default

```
Default: 3;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L294>

##### High

```
High: 4;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L295>

##### Low

```
Low: 2;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L293>

##### Min

```
Min: 1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L292>

##### None

```
None: 0;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L291>

------------------------------------------------------------------------

### ScheduleEvery

#### Enumeration Members

##### Day

```
Day: "day";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L166>

##### Hour

```
Hour: "hour";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L167>

##### Minute

```
Minute: "minute";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L168>

##### Month

```
Month: "month";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L163>

##### Second

```
Second: "second";
```

Not supported on iOS.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L172>

##### TwoWeeks

```
TwoWeeks: "twoWeeks";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L164>

##### Week

```
Week: "week";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L165>

##### Year

```
Year: "year";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L162>

------------------------------------------------------------------------

### Visibility

#### Enumeration Members

##### Private

```
Private: 0;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L300>

##### Public

```
Public: 1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L301>

##### Secret

```
Secret: -1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L299>

## Classes

### Schedule

#### Constructors

##### new Schedule()

```
new Schedule(): Schedule
```

###### Returns

[`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule)

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `at` | `undefined` \| `object` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L176> |
|  `every` | `undefined` \| `object` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L191> |
|  `interval` | `undefined` \| `object` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L184> |

#### Methods

##### at()

```
static at(   date,   repeating,   allowWhileIdle): Schedule
```

###### Parameters

| Parameter | Type | Default value |
|----|----|----|
| `date` | [`Date`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date) | `undefined` |
| `repeating` | `boolean` | `false` |
| `allowWhileIdle` | `boolean` | `false` |

###### Returns

[`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule)

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L199>

##### every()

```
static every(   kind,   count,   allowWhileIdle): Schedule
```

###### Parameters

| Parameter | Type | Default value |
|----|----|----|
| `kind` | [`ScheduleEvery`](https://v2.tauri.app/reference/javascript/notification/#scheduleevery) | `undefined` |
| `count` | `number` | `undefined` |
| `allowWhileIdle` | `boolean` | `false` |

###### Returns

[`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule)

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L218>

##### interval()

```
static interval(interval, allowWhileIdle): Schedule
```

###### Parameters

| Parameter | Type | Default value |
|----|----|----|
| `interval` | [`ScheduleInterval`](https://v2.tauri.app/reference/javascript/notification/#scheduleinterval) | `undefined` |
| `allowWhileIdle` | `boolean` | `false` |

###### Returns

[`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule)

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L207>

## Interfaces

### Action

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `destructive?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L246> |
|  `foreground?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L245> |
|  `id` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L242> |
|  `input?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L247> |
|  `inputButtonTitle?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L248> |
|  `inputPlaceholder?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L249> |
|  `requiresAuthentication?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L244> |
|  `title` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L243> |

------------------------------------------------------------------------

### ActionType

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `actions` | [`Action`](https://v2.tauri.app/reference/javascript/notification/#action)\[\] | The list of associated actions | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L260> |
|  `allowInCarPlay?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L263> |
|  `customDismissAction?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L262> |
|  `hiddenPreviewsBodyPlaceholder?` | `string` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L261> |
|  `hiddenPreviewsShowSubtitle?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L265> |
|  `hiddenPreviewsShowTitle?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L264> |
|  `id` | `string` | The identifier of this action type | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L256> |

------------------------------------------------------------------------

### ActiveNotification

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `actionTypeId?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L285> |
|  `attachments` | [`Attachment`](https://v2.tauri.app/reference/javascript/notification/#attachment)\[\] | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L284> |
|  `body?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L279> |
|  `data` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `string`\> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L282> |
|  `extra` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `unknown`\> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L283> |
|  `group?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L280> |
|  `groupSummary` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L281> |
|  `id` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L276> |
|  `schedule?` | [`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L286> |
|  `sound?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L287> |
|  `tag?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L277> |
|  `title?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L278> |

------------------------------------------------------------------------

### Attachment

Attachment of a notification.

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `id` | `string` | Attachment identifier. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L236> |
|  `url` | `string` | Attachment URL. Accepts the `asset` and `file` protocols. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L238> |

------------------------------------------------------------------------

### Channel

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `description?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L307> |
|  `id` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L305> |
|  `importance?` | [`Importance`](https://v2.tauri.app/reference/javascript/notification/#importance) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L312> |
|  `lightColor?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L310> |
|  `lights?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L309> |
|  `name` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L306> |
|  `sound?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L308> |
|  `vibration?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L311> |
|  `visibility?` | [`Visibility`](https://v2.tauri.app/reference/javascript/notification/#visibility) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L313> |

------------------------------------------------------------------------

### Options

Options to send a notification.

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `actionTypeId?` | `string` | Defines an action type for this notification. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L62> |
|  `attachments?` | [`Attachment`](https://v2.tauri.app/reference/javascript/notification/#attachment)\[\] | Notification attachments. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L111> |
|  `autoCancel?` | `boolean` | Automatically cancel the notification when the user clicks on it. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L127> |
|  `body?` | `string` | Optional notification body. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L44> |
|  `channelId?` | `string` | Identifier of the [Channel](https://v2.tauri.app/reference/javascript/notification/#channel) that deliveres this notification. If the channel does not exist, the notification won’t fire. Make sure the channel exists with listChannels and [createChannel](https://v2.tauri.app/reference/javascript/notification/#createchannel). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L36> |
|  `extra?` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `unknown`\> | Extra payload to store in the notification. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L115> |
|  `group?` | `string` | Identifier used to group multiple notifications. <https://developer.apple.com/documentation/usernotifications/unmutablenotificationcontent/1649872-threadidentifier> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L68> |
|  `groupSummary?` | `boolean` | Instructs the system that this notification is the summary of a group on Android. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L72> |
|  `icon?` | `string` | Notification icon. On Android the icon must be placed in the app’s `res/drawable` folder. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L97> |
|  `iconColor?` | `string` | Icon color on Android. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L107> |
|  `id?` | `number` | The notification identifier to reference this object later. Must be a 32-bit integer. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L29> |
|  `inboxLines?` | `string`\[\] | List of lines to add to the notification. Changes the notification style to inbox. Cannot be used with `largeBody`. Only supports up to 5 lines. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L91> |
|  `largeBody?` | `string` | Multiline text. Changes the notification style to big text. Cannot be used with `inboxLines`. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L54> |
|  `largeIcon?` | `string` | Notification large icon (Android). The icon must be placed in the app’s `res/drawable` folder. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L103> |
|  `number?` | `number` | Sets the number of items this notification represents on Android. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L139> |
|  `ongoing?` | `boolean` | If true, the notification cannot be dismissed by the user on Android. An application service must manage the dismissal of the notification. It is typically used to indicate a background task that is pending (e.g. a file download) or the user is engaged with (e.g. playing music). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L123> |
|  `schedule?` | [`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule) | Schedule this notification to fire on a later time or a fixed interval. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L48> |
|  `silent?` | `boolean` | Changes the notification presentation to be silent on iOS (no badge, no sound, not listed). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L131> |
|  `sound?` | `string` | The sound resource name or file path for the notification. \## Platform-specific behavior: - On macOS: use system sounds (e.g., “Ping”, “Blow”) or sound files in the app bundle - On Linux: use XDG theme sounds (e.g., “message-new-instant”) or file paths - On Windows: use file paths to sound files (.wav format) - On Mobile: use resource names | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L83> |
|  `summary?` | `string` | Detail text for the notification with `largeBody`, `inboxLines` or `groupSummary`. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L58> |
|  `title` | `string` | Notification title. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L40> |
|  `visibility?` | [`Visibility`](https://v2.tauri.app/reference/javascript/notification/#visibility) | Notification visibility. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L135> |

------------------------------------------------------------------------

### PendingNotification

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `body?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L271> |
|  `id` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L269> |
|  `schedule` | [`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L272> |
|  `title?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L270> |

------------------------------------------------------------------------

### ScheduleInterval

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `day?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L145> |
|  `hour?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L156> |
|  `minute?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L157> |
|  `month?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L144> |
|  `second?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L158> |
|  `weekday?` | `number` | 1 - Sunday 2 - Monday 3 - Tuesday 4 - Wednesday 5 - Thursday 6 - Friday 7 - Saturday | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L155> |
|  `year?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L143> |

## Type Aliases

### PermissionState

```
type PermissionState: "granted" | "denied" | "prompt" | "prompt-with-rationale";
```

**Source**: undefined

## Functions

### active()

```
function active(): Promise<ActiveNotification[]>
```

Retrieves the list of active notifications.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`ActiveNotification`](https://v2.tauri.app/reference/javascript/notification/#activenotification)\[\]\>

A promise resolving to the list of active notifications.

#### Example

```
import { active } from '@tauri-apps/plugin-notification';const activeNotifications = await active();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L466>

------------------------------------------------------------------------

### cancel()

```
function cancel(notifications): Promise<void>
```

Cancels the pending notifications with the given list of identifiers.

#### Parameters

| Parameter       | Type         |
|-----------------|--------------|
| `notifications` | `number`\[\] |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { cancel } from '@tauri-apps/plugin-notification';await cancel([-34234, 23432, 4311]);
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L432>

------------------------------------------------------------------------

### cancelAll()

```
function cancelAll(): Promise<void>
```

Cancels all pending notifications.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { cancelAll } from '@tauri-apps/plugin-notification';await cancelAll();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L449>

------------------------------------------------------------------------

### channels()

```
function channels(): Promise<Channel[]>
```

Retrieves the list of notification channels.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Channel`](https://v2.tauri.app/reference/javascript/notification/#channel)\[\]\>

A promise resolving to the list of notification channels.

#### Example

```
import { channels } from '@tauri-apps/plugin-notification';const notificationChannels = await channels();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L560>

------------------------------------------------------------------------

### createChannel()

```
function createChannel(channel): Promise<void>
```

Creates a notification channel.

#### Parameters

| Parameter | Type                                                     |
|-----------|----------------------------------------------------------|
| `channel` | [`Channel`](https://v2.tauri.app/reference/javascript/notification/#channel) |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { createChannel, Importance, Visibility } from '@tauri-apps/plugin-notification';await createChannel({  id: 'new-messages',  name: 'New Messages',  lights: true,  vibration: true,  importance: Importance.Default,  visibility: Visibility.Private});
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L526>

------------------------------------------------------------------------

### isPermissionGranted()

```
function isPermissionGranted(): Promise<boolean>
```

Checks if the permission to send notifications is granted.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

#### Example

```
import { isPermissionGranted } from '@tauri-apps/plugin-notification';const permissionGranted = await isPermissionGranted();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L326>

------------------------------------------------------------------------

### onAction()

```
function onAction(cb): Promise<PluginListener>
```

#### Parameters

| Parameter | Type                        |
|-----------|-----------------------------|
| `cb`      | (`notification`) =\> `void` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`PluginListener`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L570>

------------------------------------------------------------------------

### onNotificationReceived()

```
function onNotificationReceived(cb): Promise<PluginListener>
```

#### Parameters

| Parameter | Type                        |
|-----------|-----------------------------|
| `cb`      | (`notification`) =\> `void` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`PluginListener`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L564>

------------------------------------------------------------------------

### pending()

```
function pending(): Promise<PendingNotification[]>
```

Retrieves the list of pending notifications.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`PendingNotification`](https://v2.tauri.app/reference/javascript/notification/#pendingnotification)\[\]\>

A promise resolving to the list of pending notifications.

#### Example

```
import { pending } from '@tauri-apps/plugin-notification';const pendingNotifications = await pending();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L415>

------------------------------------------------------------------------

### registerActionTypes()

```
function registerActionTypes(types): Promise<void>
```

Register actions that are performed when the user clicks on the
notification.

#### Parameters

| Parameter | Type |
|----|----|
| `types` | [`ActionType`](https://v2.tauri.app/reference/javascript/notification/#actiontype)\[\] |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { registerActionTypes } from '@tauri-apps/plugin-notification';await registerActionTypes([{  id: 'tauri',  actions: [{    id: 'my-action',    title: 'Settings'  }]}])
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L398>

------------------------------------------------------------------------

### removeActive()

```
function removeActive(notifications): Promise<void>
```

Removes the active notifications with the given list of identifiers.

#### Parameters

| Parameter       | Type         |
|-----------------|--------------|
| `notifications` | `object`\[\] |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { cancel } from '@tauri-apps/plugin-notification';await cancel([-34234, 23432, 4311])
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L483>

------------------------------------------------------------------------

### removeAllActive()

```
function removeAllActive(): Promise<void>
```

Removes all active notifications.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { removeAllActive } from '@tauri-apps/plugin-notification';await removeAllActive()
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L502>

------------------------------------------------------------------------

### removeChannel()

```
function removeChannel(id): Promise<void>
```

Removes the channel with the given identifier.

#### Parameters

| Parameter | Type     |
|-----------|----------|
| `id`      | `string` |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

#### Example

```
import { removeChannel } from '@tauri-apps/plugin-notification';await removeChannel();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L543>

------------------------------------------------------------------------

### requestPermission()

```
function requestPermission(): Promise<NotificationPermission>
```

Requests the permission to send notifications.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`NotificationPermission`\>

A promise resolving to whether the user granted the permission or not.

#### Example

```
import { isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';let permissionGranted = await isPermissionGranted();if (!permissionGranted) {  const permission = await requestPermission();  permissionGranted = permission === 'granted';}
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L349>

------------------------------------------------------------------------

### sendNotification()

```
function sendNotification(options): void
```

Sends a notification to the user.

#### Parameters

| Parameter | Type |
|----|----|
| `options` | `string` \| [`Options`](https://v2.tauri.app/reference/javascript/notification/#options) |

#### Returns

`void`

#### Example

```
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';let permissionGranted = await isPermissionGranted();if (!permissionGranted) {  const permission = await requestPermission();  permissionGranted = permission === 'granted';}if (permissionGranted) {  sendNotification('Tauri is awesome!');  sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' });}
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L371>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

