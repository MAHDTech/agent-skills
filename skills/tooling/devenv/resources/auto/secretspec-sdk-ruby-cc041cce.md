# Ruby SDK

The Ruby SDK (`secretspec`) is a thin client over the `libsecretspec` C
ABI, linked into a native C extension at build time. Resolution happens
in the Rust core, so the SDK inherits every provider with no Ruby-side
logic.

## Quick start

``` astro-code
require "secretspec"

resolved = Secretspec::SecretSpec.builder
                                 .with_provider("keyring://")
                                 .with_profile("production")
                                 .with_reason("boot web app")
                                 .load

puts resolved.provider, resolved.profile
db = resolved.secrets["DATABASE_URL"]
puts db.get             # the value, or the file path for as_path secrets
resolved.set_as_env!    # export everything into ENV
```

A missing required secret raises `Secretspec::MissingRequiredError`; any
other failure raises `Secretspec::Error` (with a stable `#kind`).

## Caller context

```
builder = Secretspec::SecretSpec.builder.with_caller(  Secretspec::CallerContext.new(    name: "git",    version: "2.51.0",    operation: "credential_get",    resource: "github.com"  ))
```

Caller context identifies the invoking integration in audit records but
never satisfies `require_reason`. Do not put credentials or secret
values in it.

## Inline specifications

Use `.with_inline_spec(spec, base_dir)` to resolve a strict inline-spec
v1 hash at its logical provider-path base directory. The extension links
the separate native call symbol, so an older archive cannot fall back to
a manifest search.

## Scopes

Use `.with_scope("api")` to resolve only a named `[scopes.api]` subset.
The selected name is available as `resolved.scope` and `report.scope`:

``` astro-code
resolved = Secretspec::SecretSpec.builder.with_scope("api").load
```

## Typed access (codegen)

Generate typed classes with `secretspec schema` plus
[quicktype](https://quicktype.io), then build them from
`resolved.fields`:

```
$ secretspec schema | quicktype -s schema --top-level SecretSpec --lang ruby -o secrets_gen.rb
```

Terminal window

``` astro-code
typed = SecretSpec.from_dynamic!(resolved.fields) # typed, generated
puts typed.database_url
```

## Native library

The published platform gems bundle the `libsecretspec` archive and
statically link it into the mkmf extension at install time.

### Linking with pkg-config

Install one library type with
[cargo-c](https://github.com/lu-zero/cargo-c):

```
# Use "static" (the default) or "shared"; use separate prefixes for both.$ bash libsecretspec/scripts/cinstall.sh "$PREFIX" static
```

Terminal window

Then use the same extension flag for either type:

```
$ PKG_CONFIG_PATH="$PREFIX/lib/pkgconfig" gem install secretspec -- --enable-pkg-config
```

Terminal window

A shared install in a non-system prefix also requires `PREFIX/lib` in
the platform’s runtime library search path.
