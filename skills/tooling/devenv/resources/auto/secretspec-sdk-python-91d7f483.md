# Python SDK

The Python SDK (`secretspec`) is a thin client over a pyo3 extension
that calls `secretspec::resolve_json` directly. Resolution (providers,
chains, profiles, generation, `as_path`) happens in the Rust core, so
the SDK inherits every provider with no Python-side logic.

## Quick start

``` astro-code
from secretspec import SecretSpec

resolved = (
    SecretSpec.builder()
    .with_provider("keyring://")
    .with_profile("production")
    .with_reason("boot web app")
    .load()
)

print(resolved.provider, resolved.profile)
db = resolved.secrets["DATABASE_URL"]
print(db.get)              # the value, or the file path for as_path secrets
resolved.set_as_env()      # export everything into os.environ
```

A missing required secret raises `MissingRequiredError`; any other
failure raises `SecretSpecError` (with a stable `.kind`).

## Caller context (0.20+)

```
from secretspec import CallerContext, SecretSpec
builder = SecretSpec.builder().with_caller(CallerContext(    name="git",    version="2.51.0",    operation="credential_get",    resource="github.com",))
```

Caller context identifies the invoking integration in audit records but
never satisfies `require_reason`. Do not put credentials or secret
values in it.

## Scopes (0.17+)

Use `.with_scope("api")` to resolve only a named `[scopes.api]` subset.
The selected name is available as `resolved.scope` and `report.scope`:

``` astro-code
resolved = SecretSpec.builder().with_scope("api").load()
```

## Typed access (codegen)

Generate typed classes with `secretspec schema` plus
[quicktype](https://quicktype.io), then build them from
`resolved.fields()`:

```
$ secretspec schema | quicktype -s schema --top-level SecretSpec --lang python -o secrets_gen.py
```

Terminal window

``` astro-code
from secrets_gen import SecretSpec as Secrets  # typed

typed = Secrets.from_dict(resolved.fields())
print(typed.database_url)  # typed str
```

## Native library

The resolver is statically linked into a pyo3 extension
(`secretspec._native`, built from the `secretspec-py-native` crate)
using pyo3’s `abi3-py39` feature, so the published `cp39-abi3` wheel is
self-contained — there is no separate `cdylib` to locate and no runtime
dlopen.
