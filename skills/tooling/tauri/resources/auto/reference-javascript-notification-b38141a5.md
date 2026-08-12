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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L293>

##### High

```
High: 4;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L294>

##### Low

```
Low: 2;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L292>

##### Min

```
Min: 1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L291>

##### None

```
None: 0;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L290>

------------------------------------------------------------------------

### ScheduleEvery

#### Enumeration Members

##### Day

```
Day: "day";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L165>

##### Hour

```
Hour: "hour";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L166>

##### Minute

```
Minute: "minute";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L167>

##### Month

```
Month: "month";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L162>

##### Second

```
Second: "second";
```

Not supported on iOS.

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L171>

##### TwoWeeks

```
TwoWeeks: "twoWeeks";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L163>

##### Week

```
Week: "week";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L164>

##### Year

```
Year: "year";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L161>

------------------------------------------------------------------------

### Visibility

#### Enumeration Members

##### Private

```
Private: 0;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L299>

##### Public

```
Public: 1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L300>

##### Secret

```
Secret: -1;
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L298>

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
|  `at` | `undefined` \| `object` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L175> |
|  `every` | `undefined` \| `object` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L190> |
|  `interval` | `undefined` \| `object` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L183> |

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L198>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L217>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L206>

## Interfaces

### Action

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `destructive?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L245> |
|  `foreground?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L244> |
|  `id` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L241> |
|  `input?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L246> |
|  `inputButtonTitle?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L247> |
|  `inputPlaceholder?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L248> |
|  `requiresAuthentication?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L243> |
|  `title` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L242> |

------------------------------------------------------------------------

### ActionType

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `actions` | [`Action`](https://v2.tauri.app/reference/javascript/notification/#action)\[\] | The list of associated actions | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L259> |
|  `allowInCarPlay?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L262> |
|  `customDismissAction?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L261> |
|  `hiddenPreviewsBodyPlaceholder?` | `string` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L260> |
|  `hiddenPreviewsShowSubtitle?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L264> |
|  `hiddenPreviewsShowTitle?` | `boolean` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L263> |
|  `id` | `string` | The identifier of this action type | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L255> |

------------------------------------------------------------------------

### ActiveNotification

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `actionTypeId?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L284> |
|  `attachments` | [`Attachment`](https://v2.tauri.app/reference/javascript/notification/#attachment)\[\] | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L283> |
|  `body?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L278> |
|  `data` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `string`\> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L281> |
|  `extra` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `unknown`\> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L282> |
|  `group?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L279> |
|  `groupSummary` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L280> |
|  `id` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L275> |
|  `schedule?` | [`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L285> |
|  `sound?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L286> |
|  `tag?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L276> |
|  `title?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L277> |

------------------------------------------------------------------------

### Attachment

Attachment of a notification.

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `id` | `string` | Attachment identifier. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L235> |
|  `url` | `string` | Attachment URL. Accepts the `asset` and `file` protocols. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L237> |

------------------------------------------------------------------------

### Channel

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `description?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L306> |
|  `id` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L304> |
|  `importance?` | [`Importance`](https://v2.tauri.app/reference/javascript/notification/#importance) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L311> |
|  `lightColor?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L309> |
|  `lights?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L308> |
|  `name` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L305> |
|  `sound?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L307> |
|  `vibration?` | `boolean` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L310> |
|  `visibility?` | [`Visibility`](https://v2.tauri.app/reference/javascript/notification/#visibility) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L312> |

------------------------------------------------------------------------

### Options

Options to send a notification.

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `actionTypeId?` | `string` | Defines an action type for this notification. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L62> |
|  `attachments?` | [`Attachment`](https://v2.tauri.app/reference/javascript/notification/#attachment)\[\] | Notification attachments. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L110> |
|  `autoCancel?` | `boolean` | Automatically cancel the notification when the user clicks on it. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L126> |
|  `body?` | `string` | Optional notification body. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L44> |
|  `channelId?` | `string` | Identifier of the [Channel](https://v2.tauri.app/reference/javascript/notification/#channel) that deliveres this notification. If the channel does not exist, the notification won’t fire. Make sure the channel exists with listChannels and [createChannel](https://v2.tauri.app/reference/javascript/notification/#createchannel). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L36> |
|  `extra?` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, `unknown`\> | Extra payload to store in the notification. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L114> |
|  `group?` | `string` | Identifier used to group multiple notifications. <https://developer.apple.com/documentation/usernotifications/unmutablenotificationcontent/1649872-threadidentifier> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L68> |
|  `groupSummary?` | `boolean` | Instructs the system that this notification is the summary of a group on Android. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L72> |
|  `icon?` | `string` | Notification icon. On Android the icon must be placed in the app’s `res/drawable` folder. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L96> |
|  `iconColor?` | `string` | Icon color on Android. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L106> |
|  `id?` | `number` | The notification identifier to reference this object later. Must be a 32-bit integer. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L29> |
|  `inboxLines?` | `string`\[\] | List of lines to add to the notification. Changes the notification style to inbox. Cannot be used with `largeBody`. Only supports up to 5 lines. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L90> |
|  `largeBody?` | `string` | Multiline text. Changes the notification style to big text. Cannot be used with `inboxLines`. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L54> |
|  `largeIcon?` | `string` | Notification large icon (Android). The icon must be placed in the app’s `res/drawable` folder. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L102> |
|  `number?` | `number` | Sets the number of items this notification represents on Android. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L138> |
|  `ongoing?` | `boolean` | If true, the notification cannot be dismissed by the user on Android. An application service must manage the dismissal of the notification. It is typically used to indicate a background task that is pending (e.g. a file download) or the user is engaged with (e.g. playing music). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L122> |
|  `schedule?` | [`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule) | Schedule this notification to fire on a later time or a fixed interval. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L48> |
|  `silent?` | `boolean` | Changes the notification presentation to be silent on iOS (no badge, no sound, not listed). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L130> |
|  `sound?` | `string` | The sound resource name or file path for the notification. Platform specific behavior: - On macOS: use system sounds (e.g., “Ping”, “Blow”) or sound files in the app bundle - On Linux: use XDG theme sounds (e.g., “message-new-instant”) or file paths - On Windows: use file paths to sound files (.wav format) - On Mobile: use resource names | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L82> |
|  `summary?` | `string` | Detail text for the notification with `largeBody`, `inboxLines` or `groupSummary`. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L58> |
|  `title` | `string` | Notification title. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L40> |
|  `visibility?` | [`Visibility`](https://v2.tauri.app/reference/javascript/notification/#visibility) | Notification visibility. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L134> |

------------------------------------------------------------------------

### PendingNotification

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `body?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L270> |
|  `id` | `number` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L268> |
|  `schedule` | [`Schedule`](https://v2.tauri.app/reference/javascript/notification/#schedule) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L271> |
|  `title?` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L269> |

------------------------------------------------------------------------

### ScheduleInterval

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `day?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L144> |
|  `hour?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L155> |
|  `minute?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L156> |
|  `month?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L143> |
|  `second?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L157> |
|  `weekday?` | `number` | 1 - Sunday 2 - Monday 3 - Tuesday 4 - Wednesday 5 - Thursday 6 - Friday 7 - Saturday | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L154> |
|  `year?` | `number` | \- | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L142> |

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L465>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L431>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L448>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L559>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L525>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L325>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L569>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L563>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L414>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L397>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L482>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L501>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L542>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L348>

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
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L370>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
