+++
title = "reference-javascript-os-63b39013"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-os

Provides operating system-related utility methods and properties.

## Type Aliases

### Arch

```
type Arch:  | "x86"  | "x86_64"  | "arm"  | "aarch64"  | "mips"  | "mips64"  | "powerpc"  | "powerpc64"  | "riscv64"  | "s390x"  | "sparc64";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L42>

------------------------------------------------------------------------

### Family

```
type Family: "unix" | "windows";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L97>

------------------------------------------------------------------------

### OsType

```
type OsType:  | "linux"  | "windows"  | "macos"  | "ios"  | "android";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L40>

------------------------------------------------------------------------

### Platform

```
type Platform:  | "linux"  | "macos"  | "ios"  | "freebsd"  | "dragonfly"  | "netbsd"  | "openbsd"  | "solaris"  | "android"  | "windows";
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L28>

## Functions

### arch()

```
function arch(): Arch
```

Returns the current operating system architecture. Possible values are
`'x86'`, `'x86_64'`, `'arm'`, `'aarch64'`, `'mips'`, `'mips64'`,
`'powerpc'`, `'powerpc64'`, `'riscv64'`, `'s390x'`, `'sparc64'`.

#### Returns

[`Arch`](https://v2.tauri.app/reference/javascript/os/#arch)

#### Example

```
import { arch } from '@tauri-apps/plugin-os';const archName = arch();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L138>

------------------------------------------------------------------------

### eol()

```
function eol(): string
```

Returns the operating system-specific end-of-line marker.

- `\n` on POSIX
- `\r\n` on Windows

#### Returns

`string`

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L62>

------------------------------------------------------------------------

### exeExtension()

```
function exeExtension(): string
```

Returns the file extension, if any, used for executable binaries on this
platform. Possible values are `'exe'` and `''` (empty string).

#### Returns

`string`

#### Example

```
import { exeExtension } from '@tauri-apps/plugin-os';const exeExt = exeExtension();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L152>

------------------------------------------------------------------------

### family()

```
function family(): Family
```

Returns the current operating system family. Possible values are
`'unix'`, `'windows'`.

#### Returns

[`Family`](https://v2.tauri.app/reference/javascript/os/#family)

#### Example

```
import { family } from '@tauri-apps/plugin-os';const family = family();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L109>

------------------------------------------------------------------------

### hostname()

```
function hostname(): Promise<string | null>
```

Returns the host name of the operating system.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`
\| `null`\>

#### Example

```
import { hostname } from '@tauri-apps/plugin-os';const hostname = await hostname();
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L181>

------------------------------------------------------------------------

### locale()

```
function locale(): Promise<string | null>
```

Returns a String with a `BCP-47` language tag inside. If the locale
couldn’t be obtained, `null` is returned instead.

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`string`
\| `null`\>

#### Example

```
import { locale } from '@tauri-apps/plugin-os';const locale = await locale();if (locale) {   // use the locale string here}
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L169>

------------------------------------------------------------------------

### platform()

```
function platform(): Platform
```

Returns a string describing the specific operating system in use. The
value is set at compile time. Possible values are `'linux'`, `'macos'`,
`'ios'`, `'freebsd'`, `'dragonfly'`, `'netbsd'`, `'openbsd'`,
`'solaris'`, `'android'`, `'windows'`

#### Returns

[`Platform`](https://v2.tauri.app/reference/javascript/os/#platform)

#### Example

```
import { platform } from '@tauri-apps/plugin-os';const platformName = platform();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L79>

------------------------------------------------------------------------

### type()

```
function type(): OsType
```

Returns the current operating system type. Returns `'linux'` on Linux,
`'macos'` on macOS, `'windows'` on Windows, `'ios'` on iOS and
`'android'` on Android.

#### Returns

[`OsType`](https://v2.tauri.app/reference/javascript/os/#ostype)

#### Example

```
import { type } from '@tauri-apps/plugin-os';const osType = type();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L123>

------------------------------------------------------------------------

### version()

```
function version(): string
```

Returns the current operating system version.

#### Returns

`string`

#### Example

```
import { version } from '@tauri-apps/plugin-os';const osVersion = version();
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L93>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

