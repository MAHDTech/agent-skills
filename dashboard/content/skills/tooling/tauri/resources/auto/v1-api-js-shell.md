+++
title = "v1-api-js-shell"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

On this page

# shell

Access the system shell. Allows you to spawn child processes and manage
files and URLs using their default application.

This package is also accessible with `window.__TAURI__.shell` when
[`build.withGlobalTauri`](https://tauri.app/v1/api/config/#buildconfig.withglobaltauri)
in `tauri.conf.json` is set to `true`.

The APIs must be added to
[`tauri.allowlist.shell`](https://tauri.app/v1/api/config/#allowlistconfig.shell)
in `tauri.conf.json`:

``` prism-code
{
  "tauri": {
    "allowlist": {
      "shell": {
        "all": true, // enable all shell APIs
        "execute": true, // enable process spawn APIs
        "sidecar": true, // enable spawning sidecars
        "open": true // enable opening files/URLs using the default program
      }
    }
  }
}
```

It is recommended to allowlist only the APIs you use for optimal bundle
size and security.

## Security

This API has a scope configuration that forces you to restrict the
programs and arguments that can be used.

### Restricting access to the [`open`](https://tauri.app/v1/api/js/shell#open) API

On the allowlist, `open: true` means that the
[open](https://tauri.app/v1/api/js/shell#open) API can be used with any URL, as the
argument is validated with the
`^((mailto:\w+)|(tel:\w+)|(https?://\w+)).+` regex. You can change that
regex by changing the boolean value to a string, e.g.
`open: ^https://github.com/`.

### Restricting access to the [`Command`](https://tauri.app/v1/api/js/shell#command) APIs

The `shell` allowlist object has a `scope` field that defines an array
of CLIs that can be used. Each CLI is a configuration object
`{ name: string, cmd: string, sidecar?: bool, args?: boolean | Arg[] }`.

- `name`: the unique identifier of the command, passed to the [Command
  constructor](https://tauri.app/v1/api/js/shell#constructor). If it's a sidecar, this
  must be the value defined on
  `tauri.conf.json > tauri > bundle > externalBin`.
- `cmd`: the program that is executed on this configuration. If it's a
  sidecar, this value is ignored.
- `sidecar`: whether the object configures a sidecar or a system
  program.
- `args`: the arguments that can be passed to the program. By default no
  arguments are allowed.
  - `true` means that any argument list is allowed.
  - `false` means that no arguments are allowed.
  - otherwise an array can be configured. Each item is either a string
    representing the fixed argument value or a `{ validator: string }`
    that defines a regex validating the argument value.

#### Example scope configuration

CLI: `git commit -m "the commit message"`

Configuration:

``` prism-code
{
  "scope": [
    {
      "name": "run-git-commit",
      "cmd": "git",
      "args": ["commit", "-m", { "validator": "\\S+" }]
    }
  ]
}
```

Usage:

``` prism-code
import { Command } from '@tauri-apps/api/shell'
new Command('run-git-commit', ['commit', '-m', 'the commit message'])
```

Trying to execute any API with a program not configured on the scope
results in a promise rejection due to denied access.

## Classes

### `Child`

**Since**: 1.1.0

#### Constructors

##### `constructor`

> **new Child**(`pid`: `number`): [`Child`](https://tauri.app/v1/api/js/shell#child)

**Parameters**

| Name  | Type     |
|:------|:---------|
| `pid` | `number` |

**Defined in:**
[shell.ts:293](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L293)

#### Properties

##### `pid`

> **pid**: `number`

The child process `pid`.

**Defined in:**
[shell.ts:291](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L291)

#### Methods

##### `kill`

> **kill**():
> [`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

Kills the child process.

**Returns:**
[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

##### `write`

> **write**(`data`: `string` \|
> [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)):
> [`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

Writes `data` to the `stdin`.

**Example**

``` prism-code
import { Command } from '@tauri-apps/api/shell';
const command = new Command('node');
const child = await command.spawn();
await child.write('message');
await child.write([0, 1, 2, 3, 4, 5]);
```

**Parameters**

| Name | Type | Description |
|:---|:---|:---|
| `data` | `string` \| [`Uint8Array`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | The message to write, either a string or a byte array. |

**Returns:**
[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

A promise indicating the success or failure of the operation.

### `Command`

The entry point for spawning child processes. It emits the `close` and
`error` events.

**Example**

``` prism-code
import { Command } from '@tauri-apps/api/shell';
const command = new Command('node');
command.on('close', data => {
  console.log(`command finished with code ${data.code} and signal ${data.signal}`)
});
command.on('error', error => console.error(`command error: "${error}"`));
command.stdout.on('data', line => console.log(`command stdout: "${line}"`));
command.stderr.on('data', line => console.log(`command stderr: "${line}"`));

const child = await command.spawn();
console.log('pid:', child.pid);
```

**Since**: 1.1.0

**Hierarchy**

- [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`"close"` \|
  `"error"`\>
  - **Command**

#### Constructors

##### `constructor`

> **new Command**(`program`: `string`, `args?`: `string` \|
> `string`\[\], `options?`:
> [`SpawnOptions`](https://tauri.app/v1/api/js/shell#spawnoptions)):
> [`Command`](https://tauri.app/v1/api/js/shell#command)

Creates a new `Command` instance.

**Parameters**

[TABLE]

**Overrides:**
[EventEmitter](https://tauri.app/v1/api/js/shell#eventemitter).[constructor](https://tauri.app/v1/api/js/shell#constructor)

**Defined in:**
[shell.ts:381](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L381)

#### Properties

##### `stderr`

> `Readonly` **stderr**:
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`"data"`\>

Event emitter for the `stderr`. Emits the `data` event.

**Defined in:**
[shell.ts:371](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L371)

##### `stdout`

> `Readonly` **stdout**:
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`"data"`\>

Event emitter for the `stdout`. Emits the `data` event.

**Defined in:**
[shell.ts:369](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L369)

#### Methods

##### `addListener`

> **addListener**(`eventName`: `"error"` \| `"close"`, `listener`:
> `fn`): [`Command`](https://tauri.app/v1/api/js/shell#command)

Alias for `emitter.on(eventName, listener)`.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `"error"` \| `"close"`            |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `execute`

> **execute**():
> [`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`ChildProcess`](https://tauri.app/v1/api/js/shell#childprocess)\>

Executes the command as a child process, waiting for it to finish and
collecting all of its output.

**Example**

``` prism-code
import { Command } from '@tauri-apps/api/shell';
const output = await new Command('echo', 'message').execute();
assert(output.code === 0);
assert(output.signal === null);
assert(output.stdout === 'message');
assert(output.stderr === '');
```

**Returns:**
[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`ChildProcess`](https://tauri.app/v1/api/js/shell#childprocess)\>

A promise resolving to the child process output.

##### `listenerCount`

> **listenerCount**(`eventName`: `"error"` \| `"close"`): `number`

Returns the number of listeners listening to the event named
`eventName`.

**Since**: 1.1.0

**Parameters**

| Name        | Type                   |
|:------------|:-----------------------|
| `eventName` | `"error"` \| `"close"` |

**Returns:** `number`

##### `off`

> **off**(`eventName`: `"error"` \| `"close"`, `listener`: `fn`):
> [`Command`](https://tauri.app/v1/api/js/shell#command)

Removes the all specified listener from the listener array for the event
eventName Returns a reference to the `EventEmitter`, so that calls can
be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `"error"` \| `"close"`            |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `on`

> **on**(`eventName`: `"error"` \| `"close"`, `listener`: `fn`):
> [`Command`](https://tauri.app/v1/api/js/shell#command)

Adds the `listener` function to the end of the listeners array for the
event named `eventName`. No checks are made to see if the `listener` has
already been added. Multiple calls passing the same combination of
`eventName`and `listener` will result in the `listener` being added, and
called, multiple times.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.0.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `"error"` \| `"close"`            |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `once`

> **once**(`eventName`: `"error"` \| `"close"`, `listener`: `fn`):
> [`Command`](https://tauri.app/v1/api/js/shell#command)

Adds a **one-time**`listener` function for the event named `eventName`.
The next time `eventName` is triggered, this listener is removed and
then invoked.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `"error"` \| `"close"`            |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `prependListener`

> **prependListener**(`eventName`: `"error"` \| `"close"`, `listener`:
> `fn`): [`Command`](https://tauri.app/v1/api/js/shell#command)

Adds the `listener` function to the *beginning* of the listeners array
for the event named `eventName`. No checks are made to see if the
`listener` has already been added. Multiple calls passing the same
combination of `eventName`and `listener` will result in the `listener`
being added, and called, multiple times.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `"error"` \| `"close"`            |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `prependOnceListener`

> **prependOnceListener**(`eventName`: `"error"` \| `"close"`,
> `listener`: `fn`): [`Command`](https://tauri.app/v1/api/js/shell#command)

Adds a **one-time**`listener` function for the event named `eventName`
to the*beginning* of the listeners array. The next time `eventName` is
triggered, this listener is removed, and then invoked.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `"error"` \| `"close"`            |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `removeAllListeners`

> **removeAllListeners**(`event?`: `"error"` \| `"close"`):
> [`Command`](https://tauri.app/v1/api/js/shell#command)

Removes all listeners, or those of the specified eventName.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name     | Type                   |
|:---------|:-----------------------|
| `event?` | `"error"` \| `"close"` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `removeListener`

> **removeListener**(`eventName`: `"error"` \| `"close"`, `listener`:
> `fn`): [`Command`](https://tauri.app/v1/api/js/shell#command)

Alias for `emitter.off(eventName, listener)`.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `"error"` \| `"close"`            |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

##### `spawn`

> **spawn**():
> [`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Child`](https://tauri.app/v1/api/js/shell#child)\>

Executes the command as a child process, returning a handle to it.

**Returns:**
[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`Child`](https://tauri.app/v1/api/js/shell#child)\>

A promise resolving to the child process handle.

##### `sidecar`

> `Static` **sidecar**(`program`: `string`, `args?`: `string` \|
> `string`\[\], `options?`:
> [`SpawnOptions`](https://tauri.app/v1/api/js/shell#spawnoptions)):
> [`Command`](https://tauri.app/v1/api/js/shell#command)

Creates a command to execute the given sidecar program.

**Example**

``` prism-code
import { Command } from '@tauri-apps/api/shell';
const command = Command.sidecar('my-sidecar');
const output = await command.execute();
```

**Parameters**

[TABLE]

**Returns:** [`Command`](https://tauri.app/v1/api/js/shell#command)

### `EventEmitter<E>`

**Since**: 1.0.0

**Type parameters**

- `E` *extends* `string`

**Hierarchy**

- **EventEmitter**
  - [`Command`](https://tauri.app/v1/api/js/shell#command)

#### Constructors

##### `constructor`

> **new EventEmitter**\<`E`\>():
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

**Type parameters**

- `E` *extends* `string`

#### Methods

##### `addListener`

> **addListener**(`eventName`: `E`, `listener`: `fn`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Alias for `emitter.on(eventName, listener)`.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `E`                               |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

##### `listenerCount`

> **listenerCount**(`eventName`: `E`): `number`

Returns the number of listeners listening to the event named
`eventName`.

**Since**: 1.1.0

**Parameters**

| Name        | Type |
|:------------|:-----|
| `eventName` | `E`  |

**Returns:** `number`

##### `off`

> **off**(`eventName`: `E`, `listener`: `fn`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Removes the all specified listener from the listener array for the event
eventName Returns a reference to the `EventEmitter`, so that calls can
be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `E`                               |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

##### `on`

> **on**(`eventName`: `E`, `listener`: `fn`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Adds the `listener` function to the end of the listeners array for the
event named `eventName`. No checks are made to see if the `listener` has
already been added. Multiple calls passing the same combination of
`eventName`and `listener` will result in the `listener` being added, and
called, multiple times.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.0.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `E`                               |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

##### `once`

> **once**(`eventName`: `E`, `listener`: `fn`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Adds a **one-time**`listener` function for the event named `eventName`.
The next time `eventName` is triggered, this listener is removed and
then invoked.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `E`                               |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

##### `prependListener`

> **prependListener**(`eventName`: `E`, `listener`: `fn`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Adds the `listener` function to the *beginning* of the listeners array
for the event named `eventName`. No checks are made to see if the
`listener` has already been added. Multiple calls passing the same
combination of `eventName`and `listener` will result in the `listener`
being added, and called, multiple times.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `E`                               |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

##### `prependOnceListener`

> **prependOnceListener**(`eventName`: `E`, `listener`: `fn`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Adds a **one-time**`listener` function for the event named `eventName`
to the*beginning* of the listeners array. The next time `eventName` is
triggered, this listener is removed, and then invoked.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `E`                               |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

##### `removeAllListeners`

> **removeAllListeners**(`event?`: `E`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Removes all listeners, or those of the specified eventName.

Returns a reference to the `EventEmitter`, so that calls can be chained.

**Since**: 1.1.0

**Parameters**

| Name     | Type |
|:---------|:-----|
| `event?` | `E`  |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

##### `removeListener`

> **removeListener**(`eventName`: `E`, `listener`: `fn`):
> [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

Alias for `emitter.off(eventName, listener)`.

**Since**: 1.1.0

**Parameters**

| Name        | Type                              |
|:------------|:----------------------------------|
| `eventName` | `E`                               |
| `listener`  | (...`args`: `any`\[\]) =\> `void` |

**Returns:** [`EventEmitter`](https://tauri.app/v1/api/js/shell#eventemitter)\<`E`\>

## Interfaces

### `ChildProcess`

**Since**: 1.0.0

#### Properties

##### `code`

> **code**: `null` \| `number`

Exit code of the process. `null` if the process was terminated by a
signal on Unix.

**Defined in:**
[shell.ts:109](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L109)

##### `signal`

> **signal**: `null` \| `number`

If the process was terminated by a signal, represents that signal.

**Defined in:**
[shell.ts:111](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L111)

##### `stderr`

> **stderr**: `string`

The data that the process wrote to `stderr`.

**Defined in:**
[shell.ts:115](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L115)

##### `stdout`

> **stdout**: `string`

The data that the process wrote to `stdout`.

**Defined in:**
[shell.ts:113](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L113)

### `SpawnOptions`

**Since**: 1.0.0

#### Properties

##### `cwd`

> `Optional` **cwd**: `string`

Current working directory.

**Defined in:**
[shell.ts:88](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L88)

##### `encoding`

> `Optional` **encoding**: `string`

Character encoding for stdout/stderr

**Since**: 1.1.0

**Defined in:**
[shell.ts:96](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L96)

##### `env`

> `Optional` **env**:
> [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`,
> `string`\>

Environment variables. set to `null` to clear the process env.

**Defined in:**
[shell.ts:90](https://github.com/tauri-apps/tauri/blob/7082094/tooling/api/src/shell.ts#L90)

## Functions

### `open`

> **open**(`path`: `string`, `openWith?`: `string`):
> [`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

Opens a path or URL with the system's default app, or the one specified
with `openWith`.

The `openWith` value must be one of `firefox`, `google chrome`,
`chromium` `safari`, `open`, `start`, `xdg-open`, `gio`, `gnome-open`,
`kde-open` or `wslview`.

**Example**

``` prism-code
import { open } from '@tauri-apps/api/shell';
// opens the given URL on the default browser:
await open('https://github.com/tauri-apps/tauri');
// opens the given URL using `firefox`:
await open('https://github.com/tauri-apps/tauri', 'firefox');
// opens a file using the default program:
await open('/path/to/file');
```

**Since**: 1.0.0

**Parameters**

[TABLE]

**Returns:**
[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`void`\>

