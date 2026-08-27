+++
title = "secretspec-sdk-nodejs-49b94f88"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Node.js SDK

The Node.js / TypeScript SDK (`secretspec`) is a thin wrapper over a
[napi-rs](https://napi.rs/) native addon that embeds the resolver.
Resolution happens in the Rust core, so the SDK inherits every provider
with no JS-side logic. npm installs a prebuilt addon for the host
platform: Linux x64 and arm64 (glibc, and musl for Alpine images in
0.20+), macOS on Apple silicon, and Windows x64. TypeScript declarations
ship in `index.d.ts`.

## Quick start

``` astro-code
const { SecretSpec } = require('secretspec');

const resolved = SecretSpec.builder()
  .withProvider('keyring://')
  .withProfile('production')
  .withReason('boot web app')
  .load();

console.log(resolved.provider, resolved.profile);
const db = resolved.secrets.DATABASE_URL;
console.log(db.get());   // the value, or the file path for as_path secrets
resolved.setAsEnv();     // export everything into process.env
```

A missing required secret throws `MissingRequiredError`; any other
failure throws `SecretSpecError` (with a stable `.kind`).

## Caller context (0.20+)

```
const builder = SecretSpec.builder().withCaller({  name: 'git',  version: '2.51.0',  operation: 'credential_get',  resource: 'github.com',});
```

Caller context identifies the invoking integration in audit records but
never satisfies `require_reason`. Do not put credentials or secret
values in it.

## Inline specifications (0.20+)

Use `.withInlineSpec(spec, baseDir)` (or `loadAsync`/`reportAsync`) to
resolve a strict inline-spec v1 object. `baseDir` resolves relative
provider paths; the embedded addon submits the versioned native request
directly.

## Scopes (0.17+)

Use `.withScope('api')` to resolve only a named `[scopes.api]` subset.
The selected name is available as `resolved.scope` and `report.scope`:

``` astro-code
const resolved = SecretSpec.builder().withScope('api').load();
```

## Typed access (codegen)

Generate typed interfaces with `secretspec schema` plus
[quicktype](https://quicktype.io), then convert `resolved.fieldsJson()`:

```
$ secretspec schema | quicktype -s schema --top-level SecretSpec --lang typescript -o secrets_gen.ts
```

Terminal window

``` astro-code
import { Convert } from './secrets_gen'; // typed, generated

const typed = Convert.toSecretSpec(resolved.fieldsJson());
console.log(typed.DATABASE_URL);
```

