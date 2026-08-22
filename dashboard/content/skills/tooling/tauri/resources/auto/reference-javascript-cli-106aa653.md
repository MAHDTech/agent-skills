+++
title = "reference-javascript-cli-106aa653"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# @tauri-apps/plugin-cli

Parse arguments from your Command Line Interface.

## Interfaces

### ArgMatch

#### Since

2.0.0

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `occurrences` | `number` | Number of occurrences | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L26> |
|  `value` | `null` \| `string` \| `boolean` \| `string`\[\] | string if takes value boolean if flag string\[\] or null if takes multiple values | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L22> |

------------------------------------------------------------------------

### CliMatches

#### Since

2.0.0

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `args` | [`Record`](https://www.typescriptlang.org/docs/handbook/utility-types.html#recordkeys-type)\<`string`, [`ArgMatch`](https://v2.tauri.app/reference/javascript/cli/#argmatch)\> | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L41> |
|  `subcommand` | `null` \| [`SubcommandMatch`](https://v2.tauri.app/reference/javascript/cli/#subcommandmatch) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L42> |

------------------------------------------------------------------------

### SubcommandMatch

#### Since

2.0.0

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `matches` | [`CliMatches`](https://v2.tauri.app/reference/javascript/cli/#climatches) | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L34> |
|  `name` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L33> |

## Functions

### getMatches()

```
function getMatches(): Promise<CliMatches>
```

Parse the arguments provided to the current process and get the matches
using the configuration defined
[`tauri.cli`](https://tauri.app/v1/api/config/#tauriconfig.cli) in
`tauri.conf.json`

#### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`CliMatches`](https://v2.tauri.app/reference/javascript/cli/#climatches)\>

#### Example

```
import { getMatches } from '@tauri-apps/plugin-cli';const matches = await getMatches();if (matches.subcommand?.name === 'run') {  // `./your-app run $ARGS` was executed  const args = matches.subcommand?.matches.args  if ('debug' in args) {    // `./your-app run --debug` was executed  }} else {  const args = matches.args  // `./your-app $ARGS` was executed}
```

#### Since

2.0.0

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L66>

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

