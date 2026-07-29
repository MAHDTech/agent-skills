+++
title = "secretspec-sdk-ruby-cc041cce"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# Ruby SDK

The Ruby SDK (`secretspec`) is a thin client over the `secretspec-ffi` C
ABI, statically linked into a native C extension at build time (no
runtime library to locate). Resolution happens in the Rust core, so the
SDK inherits every provider with no Ruby-side logic.

## Quick start

```
require "secretspec"
resolved = Secretspec::SecretSpec.builder                                 .with_provider("keyring://")                                 .with_profile("production")                                 .with_reason("boot web app")                                 .load
puts resolved.provider, resolved.profiledb = resolved.secrets["DATABASE_URL"]puts db.get             # the value, or the file path for as_path secretsresolved.set_as_env!    # export everything into ENV
```

A missing required secret raises `Secretspec::MissingRequiredError`; any
other failure raises `Secretspec::Error` (with a stable `#kind`).

## Scopes (0.17+)

Use `.with_scope("api")` to resolve only a named `[scopes.api]` subset.
The selected name is available as `resolved.scope` and `report.scope`:

```
resolved = Secretspec::SecretSpec.builder.with_scope("api").load
```

## Typed access (codegen)

Generate typed classes with `secretspec schema` plus
[quicktype](https://quicktype.io), then build them from
`resolved.fields`:

```
secretspec schema | quicktype -s schema --top-level SecretSpec --lang ruby -o secrets_gen.rb
```

Terminal window

```
typed = SecretSpec.from_dynamic!(resolved.fields) # typed, generatedputs typed.database_url
```

## Native library

The resolver is statically linked into a native C extension built by
mkmf, so the published platform gem is self-contained — there is no
separate `cdylib` to locate and no `SECRETSPEC_FFI_LIB` to set at
runtime.

{% endraw %}
