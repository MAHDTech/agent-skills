+++
title = "reference-javascript-store-2c5805fa"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-store

## Classes

### LazyStore

A lazy loaded key-value store persisted by the backend layer.

#### Implements

- `IStore`

#### Constructors

##### new LazyStore()

```
new LazyStore(path, options?): LazyStore
```

Note that the options are not applied if someone else already created
the store

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `path` | `string` | Path to save the store in `app_data_dir` |
| `options`? | [`StoreOptions`](https://v2.tauri.app/reference/javascript/store/#storeoptions) | Store configuration options |

###### Returns

[`LazyStore`](https://v2.tauri.app/reference/javascript/store/#lazystore)

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L104>

#### Methods

##### clear()

```
clear(): Promise<void>
```

Clears the store, removing all key-value pairs.

Note: To clear the storage and reset it to its `default` value, use
[`reset`](https://v2.tauri.app/reference/javascript/store/#reset) instead.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.clear`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L132>

##### close()

```
close(): Promise<void>
```

Close the store and cleans up this resource from memory. **You should
not call any method on this object anymore and should drop any reference
to it.**

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.close`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L177>

##### delete()

```
delete(key): Promise<boolean>
```

Removes a key-value pair from the store.

###### Parameters

| Parameter | Type     | Description |
|-----------|----------|-------------|
| `key`     | `string` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

###### Implementation of

`IStore.delete`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L128>

##### entries()

```
entries<T>(): Promise<[string, T][]>
```

Returns a list of all entries in the store.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<\[`string`,
`T`\]\[\]\>

###### Implementation of

`IStore.entries`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L148>

##### get()

```
get<T>(key): Promise<undefined | T>
```

Returns the value for the given `key` or `undefined` if the key does not
exist.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type     | Description |
|-----------|----------|-------------|
| `key`     | `string` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`undefined`
\| `T`\>

###### Implementation of

`IStore.get`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L120>

##### has()

```
has(key): Promise<boolean>
```

Returns `true` if the given `key` exists in the store.

###### Parameters

| Parameter | Type     | Description |
|-----------|----------|-------------|
| `key`     | `string` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

###### Implementation of

`IStore.has`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L124>

##### init()

```
init(): Promise<void>
```

Init/load the store if it’s not loaded already

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L112>

##### keys()

```
keys(): Promise<string[]>
```

Returns a list of all keys in the store.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\[\]\>

###### Implementation of

`IStore.keys`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L140>

##### length()

```
length(): Promise<number>
```

Returns the number of key-value pairs in the store.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`number`\>

###### Implementation of

`IStore.length`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L152>

##### onChange()

```
onChange<T>(cb): Promise<UnlistenFn>
```

Listen to changes on the store.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type                        | Description |
|-----------|-----------------------------|-------------|
| `cb`      | (`key`, `value`) =\> `void` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`UnlistenFn`\>

A promise resolving to a function to unlisten to the event.

###### Since

2.0.0

###### Implementation of

`IStore.onChange`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L171>

##### onKeyChange()

```
onKeyChange<T>(key, cb): Promise<UnlistenFn>
```

Listen to changes on a store key.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type                 | Description |
|-----------|----------------------|-------------|
| `key`     | `string`             |             |
| `cb`      | (`value`) =\> `void` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`UnlistenFn`\>

A promise resolving to a function to unlisten to the event.

###### Since

2.0.0

###### Implementation of

`IStore.onKeyChange`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L164>

##### reload()

```
reload(options?): Promise<void>
```

Attempts to load the on-disk state at the store’s `path` into memory.

This method is useful if the on-disk state was edited by the user and
you want to synchronize the changes.

Note:

- This method loads the data and merges it with the current store, this
  behavior will be changed to resetting to default first and then
  merging with the on-disk state in v3, to fully match the store with
  the on-disk state, set
  [`ignoreDefaults`](https://v2.tauri.app/reference/javascript/store/#reloadoptions) to
  `true`
- This method does not emit change events.

###### Parameters

| Parameter  | Type                                                          |
|------------|---------------------------------------------------------------|
| `options`? | [`ReloadOptions`](https://v2.tauri.app/reference/javascript/store/#reloadoptions) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.reload`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L156>

##### reset()

```
reset(): Promise<void>
```

Resets the store to its `default` value.

If no default value has been set, this method behaves identical to
[`clear`](https://v2.tauri.app/reference/javascript/store/#clear).

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.reset`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L136>

##### save()

```
save(): Promise<void>
```

Saves the store to disk at the store’s `path`.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.save`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L160>

##### set()

```
set(key, value): Promise<void>
```

Inserts a key-value pair into the store.

###### Parameters

| Parameter | Type      | Description |
|-----------|-----------|-------------|
| `key`     | `string`  |             |
| `value`   | `unknown` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.set`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L116>

##### values()

```
values<T>(): Promise<T[]>
```

Returns a list of all values in the store.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`T`\[\]\>

###### Implementation of

`IStore.values`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L144>

------------------------------------------------------------------------

### Store

A key-value store persisted by the backend layer.

#### Extends

- `Resource`

#### Implements

- `IStore`

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

##### clear()

```
clear(): Promise<void>
```

Clears the store, removing all key-value pairs.

Note: To clear the storage and reset it to its `default` value, use
[`reset`](https://v2.tauri.app/reference/javascript/store/#reset-1) instead.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.clear`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L267>

##### close()

```
close(): Promise<void>
```

Destroys and cleans up this resource from memory. **You should not call
any method on this object anymore and should drop any reference to it.**

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.close`

###### Inherited from

`Resource.close`

**Source**: undefined

##### delete()

```
delete(key): Promise<boolean>
```

Removes a key-value pair from the store.

###### Parameters

| Parameter | Type     | Description |
|-----------|----------|-------------|
| `key`     | `string` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

###### Implementation of

`IStore.delete`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L260>

##### entries()

```
entries<T>(): Promise<[string, T][]>
```

Returns a list of all entries in the store.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<\[`string`,
`T`\]\[\]\>

###### Implementation of

`IStore.entries`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L283>

##### get()

```
get<T>(key): Promise<undefined | T>
```

Returns the value for the given `key` or `undefined` if the key does not
exist.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type     | Description |
|-----------|----------|-------------|
| `key`     | `string` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`undefined`
\| `T`\>

###### Implementation of

`IStore.get`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L245>

##### has()

```
has(key): Promise<boolean>
```

Returns `true` if the given `key` exists in the store.

###### Parameters

| Parameter | Type     | Description |
|-----------|----------|-------------|
| `key`     | `string` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

###### Implementation of

`IStore.has`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L253>

##### keys()

```
keys(): Promise<string[]>
```

Returns a list of all keys in the store.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`\[\]\>

###### Implementation of

`IStore.keys`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L275>

##### length()

```
length(): Promise<number>
```

Returns the number of key-value pairs in the store.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`number`\>

###### Implementation of

`IStore.length`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L287>

##### onChange()

```
onChange<T>(cb): Promise<UnlistenFn>
```

Listen to changes on the store.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type                        | Description |
|-----------|-----------------------------|-------------|
| `cb`      | (`key`, `value`) =\> `void` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`UnlistenFn`\>

A promise resolving to a function to unlisten to the event.

###### Since

2.0.0

###### Implementation of

`IStore.onChange`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L310>

##### onKeyChange()

```
onKeyChange<T>(key, cb): Promise<UnlistenFn>
```

Listen to changes on a store key.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter | Type                 | Description |
|-----------|----------------------|-------------|
| `key`     | `string`             |             |
| `cb`      | (`value`) =\> `void` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`UnlistenFn`\>

A promise resolving to a function to unlisten to the event.

###### Since

2.0.0

###### Implementation of

`IStore.onKeyChange`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L299>

##### reload()

```
reload(options?): Promise<void>
```

Attempts to load the on-disk state at the store’s `path` into memory.

This method is useful if the on-disk state was edited by the user and
you want to synchronize the changes.

Note:

- This method loads the data and merges it with the current store, this
  behavior will be changed to resetting to default first and then
  merging with the on-disk state in v3, to fully match the store with
  the on-disk state, set
  [`ignoreDefaults`](https://v2.tauri.app/reference/javascript/store/#reloadoptions) to
  `true`
- This method does not emit change events.

###### Parameters

| Parameter  | Type                                                          |
|------------|---------------------------------------------------------------|
| `options`? | [`ReloadOptions`](https://v2.tauri.app/reference/javascript/store/#reloadoptions) |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.reload`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L291>

##### reset()

```
reset(): Promise<void>
```

Resets the store to its `default` value.

If no default value has been set, this method behaves identical to
[`clear`](https://v2.tauri.app/reference/javascript/store/#clear-1).

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.reset`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L271>

##### save()

```
save(): Promise<void>
```

Saves the store to disk at the store’s `path`.

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.save`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L295>

##### set()

```
set(key, value): Promise<void>
```

Inserts a key-value pair into the store.

###### Parameters

| Parameter | Type      | Description |
|-----------|-----------|-------------|
| `key`     | `string`  |             |
| `value`   | `unknown` |             |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

###### Implementation of

`IStore.set`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L237>

##### values()

```
values<T>(): Promise<T[]>
```

Returns a list of all values in the store.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`T`\[\]\>

###### Implementation of

`IStore.values`

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L279>

##### get()

```
static get(path): Promise<null | Store>
```

Gets an already loaded store.

If the store is not loaded, returns `null`. In this case you must
[load](https://v2.tauri.app/reference/javascript/store/#load) it.

This function is more useful when you already know the store is loaded
and just need to access its instance. Prefer
[Store.load](https://v2.tauri.app/reference/javascript/store/#load) otherwise.

###### Parameters

| Parameter | Type     | Description        |
|-----------|----------|--------------------|
| `path`    | `string` | Path of the store. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`null`
\| [`Store`](https://v2.tauri.app/reference/javascript/store/#store)\>

###### Example

```
import { Store } from '@tauri-apps/api/store';let store = await Store.get('store.json');if (!store) {  store = await Store.load('store.json');}
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L231>

##### load()

```
static load(path, options?): Promise<Store>
```

Create a new Store or load the existing store with the path.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `path` | `string` | Path to save the store in `app_data_dir` |
| `options`? | [`StoreOptions`](https://v2.tauri.app/reference/javascript/store/#storeoptions) | Store configuration options |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Store`](https://v2.tauri.app/reference/javascript/store/#store)\>

###### Example

```
import { Store } from '@tauri-apps/api/store';const store = await Store.load('store.json');
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L204>

## Type Aliases

### ReloadOptions

```
type ReloadOptions: object;
```

Options to IStore.reload a IStore

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `ignoreDefaults` | `boolean` | To fully match the store with the on-disk state, ignoring defaults | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L461> |

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L457>

------------------------------------------------------------------------

### StoreOptions

```
type StoreOptions: object;
```

Options to create a store

#### Type declaration

| Name | Type | Description | Defined in |
|----|----|----|----|
| `autoSave` | `boolean` \| `number` | Auto save on modification with debounce duration in milliseconds, it’s 100ms by default, pass in `false` to disable it | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L28> |
| `createNew` | `boolean` | Force create a new store with default values even if it already exists. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L40> |
| `defaults` | `object` | Default value of the store | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L24> |
| `deserializeFnName` | `string` | Name of a deserialize function registered in the rust side plugin builder | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L36> |
| `overrideDefaults` | `boolean` | When creating the store, override the store with the on-disk state if it exists, ignoring defaults | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L44> |
| `serializeFnName` | `string` | Name of a serialize function registered in the rust side plugin builder | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L32> |

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L20>

## Functions

### getStore()

```
function getStore(path): Promise<Store | null>
```

Gets an already loaded store.

If the store is not loaded, returns `null`. In this case you must
[load](https://v2.tauri.app/reference/javascript/store/#load) it.

This function is more useful when you already know the store is loaded
and just need to access its instance. Prefer
[Store.load](https://v2.tauri.app/reference/javascript/store/#load) otherwise.

#### Parameters

| Parameter | Type     | Description        |
|-----------|----------|--------------------|
| `path`    | `string` | Path of the store. |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Store`](https://v2.tauri.app/reference/javascript/store/#store)
\| `null`\>

#### Example

```
import { getStore } from '@tauri-apps/api/store';const store = await getStore('store.json');
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L82>

------------------------------------------------------------------------

### load()

```
function load(path, options?): Promise<Store>
```

Create a new Store or load the existing store with the path.

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `path` | `string` | Path to save the store in `app_data_dir` |
| `options`? | [`StoreOptions`](https://v2.tauri.app/reference/javascript/store/#storeoptions) | Store configuration options |

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Store`](https://v2.tauri.app/reference/javascript/store/#store)\>

#### Example

```
import { Store } from '@tauri-apps/api/store';const store = await Store.load('store.json');
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L59>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

