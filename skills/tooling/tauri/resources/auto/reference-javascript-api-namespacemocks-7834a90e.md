# mocks

## Interfaces

### MockIPCOptions

Options for `mockIPC`.

# Options

`shouldMockEvents`: If true, the `listen` and `emit` functions will be
mocked, allowing you to test event handling without a real backend.
**This will consume any events emitted with the `plugin:event` prefix.**

#### Since

2.7.0

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `shouldMockEvents?` | `boolean` | **Source**: <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L24> |

## Functions

### clearMocks()

```
function clearMocks(): void
```

Clears mocked functions/data injected by the other functions in this
module. When using a test runner that doesn’t provide a fresh window
object for each test, calling this function will reset tauri specific
properties.

# Example

```
import { mockWindows, clearMocks } from "@tauri-apps/api/mocks"
afterEach(() => {   clearMocks()})
test("mocked windows", () => {   mockWindows("main", "second", "third");
   expect(window.__TAURI_INTERNALS__).toHaveProperty("metadata")})
test("no mocked windows", () => {   expect(window.__TAURI_INTERNALS__).not.toHaveProperty("metadata")})
```

#### Returns

`void`

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L316>

------------------------------------------------------------------------

### mockConvertFileSrc()

```
function mockConvertFileSrc(osName): void
```

Mock `convertFileSrc` function

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `osName` | `string` | The operating system to mock, can be one of linux, macos, or windows |

#### Returns

`void`

#### Example

```
import { mockConvertFileSrc } from "@tauri-apps/api/mocks";import { convertFileSrc } from "@tauri-apps/api/core";
mockConvertFileSrc("windows")
const url = convertFileSrc("C:\\Users\\user\\file.txt")
```

#### Since

1.6.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L277>

------------------------------------------------------------------------

### mockIPC()

```
function mockIPC(cb, options?): void
```

Intercepts all IPC requests with the given mock handler.

This function can be used when testing tauri frontend applications or
when running the frontend in a Node.js context during static site
generation.

# Examples

Testing setup using Vitest:

```
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks"import { invoke } from "@tauri-apps/api/core"
afterEach(() => {   clearMocks()})
test("mocked command", () => { mockIPC((cmd, payload) => {  switch (cmd) {    case "add":      return (payload.a as number) + (payload.b as number);    default:      break;    } });
 expect(invoke('add', { a: 12, b: 15 })).resolves.toBe(27);})
```

The callback function can also return a Promise:

```
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks"import { invoke } from "@tauri-apps/api/core"
afterEach(() => {   clearMocks()})
test("mocked command", () => { mockIPC((cmd, payload) => {  if(cmd === "get_data") {   return fetch("https://example.com/data.json")     .then((response) => response.json())  } });
 expect(invoke('get_data')).resolves.toBe({ foo: 'bar' });})
```

`listen` can also be mocked with direct calls to the `emit` function.
This functionality is opt-in via the `shouldMockEvents` option:

```
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks"import { emit, listen } from "@tauri-apps/api/event"
afterEach(() => {   clearMocks()})
test("mocked event", () => { mockIPC(() => {}, { shouldMockEvents: true }); // enable event mocking
 const eventHandler = vi.fn(); listen('test-event', eventHandler); // typically in component setup or similar
 emit('test-event', { foo: 'bar' }); expect(eventHandler).toHaveBeenCalledWith({   event: 'test-event',   payload: { foo: 'bar' } });})
```

`emitTo` is currently **not** supported by this mock implementation.

#### Parameters

| Parameter | Type |
|----|----|
| `cb` | (`cmd`, `payload`?) =\> `unknown` |
| `options`? | [`MockIPCOptions`](https://v2.tauri.app/reference/javascript/api/namespacemocks/#mockipcoptions) |

#### Returns

`void`

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L104>

------------------------------------------------------------------------

### mockWindows()

```
function mockWindows(current, ..._additionalWindows): void
```

Mocks one or many window labels. In non-tauri context it is required to
call this function *before* using the `@tauri-apps/api/window` module.

This function only mocks the *presence* of windows, window properties
(e.g. width and height) can be mocked like regular IPC calls using the
`mockIPC` function.

# Examples

```
import { mockWindows } from "@tauri-apps/api/mocks";import { getCurrentWindow } from "@tauri-apps/api/window";
mockWindows("main", "second", "third");
const win = getCurrentWindow();
win.label // "main"
```

```
import { mockWindows } from "@tauri-apps/api/mocks";
mockWindows("main", "second", "third");
mockIPC((cmd, args) => { if (cmd === "plugin:event|emit") {   console.log('emit event', args?.event, args?.payload); }});
const { emit } = await import("@tauri-apps/api/event");await emit('loaded'); // this will cause the mocked IPC handler to log to the console.
```

#### Parameters

| Parameter | Type | Description |
|----|----|----|
| `current` | `string` | Label of window this JavaScript context is running in. |
| …`_additionalWindows` | `string`\[\] | \- |

#### Returns

`void`

#### Since

1.0.0

**Source**:
<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L248>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
