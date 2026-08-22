+++
title = "secretspec-sdk-rust-f3fc22ff"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Rust SDK

SecretSpec provides a Rust library with type-safe access to secrets
through a derive macro. The macro reads `secretspec.toml` at compile
time and generates Rust types for its profiles and secrets.

## Quick start

Add the runtime and derive macro from the command line:

```
cargo add secretspec secretspec-derive
```

Terminal window

Alternatively, add both dependencies to your `Cargo.toml`:

```
[dependencies]secretspec = "0.20"secretspec-derive = "0.20"
```

The examples on this page are compiled as Cargo examples in
`secretspec-derive`. They generate their types from this manifest:

``` astro-code
[project]
name = "rust-sdk-example"
revision = "1.0"

[profiles.default]
DATABASE_URL = { description = "PostgreSQL connection string", required = true }
REDIS_URL = { description = "Redis connection string", required = false }
TLS_CERT = { description = "TLS certificate", required = true, as_path = true }
TLS_KEY = { description = "TLS private key", required = false, as_path = true }

[profiles.development]
DATABASE_URL = { default = "postgresql://localhost/development" }

[profiles.production]
DATABASE_URL = { required = true }
API_KEY = { description = "Production API key", required = true }

[scopes.api]
secrets = ["DATABASE_URL"]
```

`declare_secrets!` generates `SecretSpec`, `Profile`, and
`SecretSpecProfile`. The standard loader returns the union type that is
safe to use with any declared profile:

``` astro-code
secretspec_derive::declare_secrets!("secretspec.toml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolved = SecretSpec::builder()
        .with_provider("keyring://")
        .with_profile("development")
        .with_reason("start application")
        .load()?;

    println!("Database: {}", resolved.secrets.database_url);

    if let Some(redis_url) = &resolved.secrets.redis_url {
        println!("Redis: {redis_url}");
    }

    resolved.secrets.set_as_env_vars();

    println!("Profile: {}", resolved.profile);
    println!("Provider: {}", resolved.provider);

    Ok(())
}
```

Required and defaulted secrets are generated as `String`; secrets that
may be absent are `Option<String>`. Field names use Rust snake case, so
`DATABASE_URL` becomes `database_url`.

## Describing secrets in Rust (0.20+)

Starting with SecretSpec 0.20, `Spec` is the format-independent
declaration API. Build one directly in Rust when an application owns its
secret contract in code, then pass the validated specification to
`Secrets::from_spec`:

``` astro-code
use secretspec::{Profile, Secret, Secrets, Spec};

fn main() -> secretspec::Result<()> {
    let spec = Spec::builder("checkout")
        .provider("env", "env://")
        .secret(
            "DATABASE_URL",
            Secret::required("PostgreSQL connection URL").providers(["env"]),
        )
        .secret(
            "SENTRY_DSN",
            Secret::optional("Sentry error-reporting endpoint"),
        )
        .profile(
            "production",
            Profile::new().secret("SENTRY_DSN", Secret::required("Production Sentry endpoint")),
        )
        .scope("web", ["DATABASE_URL", "SENTRY_DSN"])
        .build()?;

    let mut secrets = Secrets::from_spec(spec)?;
    secrets.set_profile("production");
    secrets.set_scope("web");

    let resolved = secrets.resolve()?;
    println!("resolved profile: {}", resolved.profile);
    Ok(())
}
```

`Spec::from_toml` and `Spec::try_from(path)` produce the same type
through the same validation and compilation path. Convert from a path
for manifests with `extends`, because a TOML string has no directory
from which to resolve relative paths. A spec loaded from a file retains
that file’s directory for relative provider paths. A Rust-built
declaration resolves them from the current working directory by default;
`Secrets::from_spec_at` selects another logical base directory.

Use `schema_json(None)` to emit the value-free JSON Schema for the union
shape, or pass a profile name for that profile’s effective fields:

```
let union_schema = spec.schema_json(None)?;let production_schema = spec.schema_json(Some("production"))?;
```

Schema generation reads declarations only. It does not resolve secret
values or contact providers.

`Spec` is immutable so its declarations and compiled view cannot drift
apart. Use `to_builder()` to edit a copy, or `into_builder()` to consume
the original, then rebuild to validate the result:

```
let edited = spec    .to_builder()    .remove_secret("default", "LEGACY_TOKEN")    .add_secret(        "production",        "DEPLOY_TOKEN",        Secret::required("Production deployment token"),    )    .build()?;
```

`remove_secret` removes the declaration from that profile. Removing an
override can reveal the declaration inherited from `default`; removing
it from `default` also removes it from profiles that only inherited it.
`build()` rejects dangling scope membership, invalid compositions, empty
profiles, and other semantic errors introduced by an edit.

For a spec loaded from TOML, `secret`, `add_secret`, `replace_secret`,
and `remove_secret` preserve comments, ordering, quoting, and unrelated
syntax in the root document. Read the edited document with
`edited.preserved_text()`. Adding and then removing the same declaration
restores the original bytes, and inherited declarations are never
inlined into a child manifest. Other builder operations are semantic
edits and clear the retained text; use `to_toml()` when freshly
formatted output is acceptable. A spec constructed with
`Spec::builder()` has no original document to preserve.

The Rust-first API complements `declare_secrets!`: `Spec` describes and
resolves names dynamically, while the macro continues to generate
statically typed fields from a manifest at compile time.

## Profile-specific types

Use `load_profile()` when code should receive the exact shape of the
selected profile. It returns a `SecretSpecProfile` enum whose variants
contain that profile’s effective fields, including fields inherited from
`[profiles.default]`:

``` astro-code
secretspec_derive::declare_secrets!("secretspec.toml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolved = SecretSpec::builder()
        .with_provider("keyring://")
        .with_profile(Profile::Production)
        .with_reason("start production application")
        .load_profile()?;

    match resolved.secrets {
        SecretSpecProfile::Production {
            database_url,
            api_key,
            ..
        } => {
            println!("Database: {database_url}");
            println!("API key loaded: {} bytes", api_key.len());
        }
        _ => unreachable!("the production profile was selected"),
    }

    Ok(())
}
```

## Scopes (0.17+)

A [scope](https://secretspec.dev/concepts/scopes/) resolves only a named subset of a profile.
Scopes are available through the untyped `Secrets` API:

``` astro-code
use secretspec::Secrets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut spec = Secrets::load()?;
    spec.set_scope("api");

    let resolved = spec.resolve()?;
    assert_eq!(resolved.scope.as_deref(), Some("api"));

    Ok(())
}
```

`resolve()` and `report()` both return the active scope. The untyped API
also honors `SECRETSPEC_SCOPE` when no scope is selected explicitly.

Typed loaders generated by `declare_secrets!` deliberately do not
support scopes. A generated struct has a field for every declared
secret, so hiding one would leave that field unfillable.
`SecretSpec::builder()` therefore has no `with_scope`, and typed
`load()` and `load_profile()` always resolve the full profile. Use a
separate manifest or the untyped API when a component needs a narrowed
set.

## Resolving one secret (0.19+)

`resolve()` answers whether the whole profile can be satisfied, so a
single missing required secret fails it and returns nothing. When a
component needs one secret, `resolve_named()` reads only that secret and
the inputs it composes from, and reports the outcomes separately:

``` astro-code
use secretspec::{NamedResolution, Secrets};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Resolving one secret reads only that secret and its composition inputs,
    // so an unrelated missing required secret cannot fail the call.
    let spec = Secrets::load()?.with_default_reason("cache warmup");

    match spec.resolve_named("REDIS_URL")? {
        NamedResolution::Resolved(secret) => {
            // Exactly one of `value` and `path` is set; `path` for `as_path`.
            println!("resolved from {:?}", secret.source);
        }
        // Declared, but nothing provided it. `required` says whether a
        // whole-profile resolve would treat that as an error.
        NamedResolution::Missing { required } => {
            println!("no value (required: {required})");
        }
        // Not declared in this profile, or hidden by the active scope.
        NamedResolution::Undeclared => println!("not on this profile's surface"),
    }

    Ok(())
}
```

`NamedResolution::Undeclared` covers both a name the profile does not
declare and one the active [scope](https://secretspec.dev/concepts/scopes/) hides, since
neither is on the surface this session resolves. Provider and
configuration failures stay `Err` rather than turning into a missing
value, and whole-profile presence constraints (`at_least_one`,
`exactly_one`) are not evaluated for a single-secret read.

`with_default_reason()` (also 0.19+) supplies a reason only when the
caller has not already set one through `with_reason()` or
`SECRETSPEC_REASON`, so a wrapper can describe itself without
overwriting the more specific reason it was given. When a wrapper only
needs to identify the software integration, use the separate caller
context below; unlike a default reason, it cannot satisfy
`require_reason`.

## Caller context (0.20+)

Software integrations can record what invoked SecretSpec without
replacing the user-supplied access reason:

```
use secretspec::{CallerContext, Secrets};
let spec = Secrets::load()?.with_caller(    CallerContext::new("git")        .with_version("2.51.0")        .with_operation("credential_get")        .with_resource("github.com"),);
```

Generated builders expose the same `with_caller()` method. Caller
context is caller-asserted audit metadata, not an authenticated
identity, and never satisfies `require_reason`. Do not put credentials
or secret values in it.

## Interactive prompting (0.20+)

`Secrets::ensure_secrets` prompts for and stores any missing required
secret when stdin is a real terminal. Generated builders opt into the
same behavior with `prompt_missing()`:

``` astro-code
secretspec_derive::declare_secrets!("secretspec.toml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolved = SecretSpec::builder()
        .with_provider("keyring://")
        .with_profile("development")
        .with_reason("start application")
        .prompt_missing(true)
        .load()?;

    println!("Database: {}", resolved.secrets.database_url);

    Ok(())
}
```

Left unset (the default), a missing required secret still fails fast
with `RequiredSecretMissing`, exactly as without `prompt_missing()`.

## Secrets as file paths

Secrets declared with `as_path = true` are generated as `PathBuf`
instead of `String`. Optional file-shaped secrets use `Option<PathBuf>`:

``` astro-code
secretspec_derive::declare_secrets!("secretspec.toml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolved = SecretSpec::builder()
        .with_provider("keyring://")
        .with_reason("configure TLS")
        .load()?;

    let certificate: &std::path::PathBuf = &resolved.secrets.tls_cert;
    println!("Certificate: {}", certificate.display());

    if let Some(private_key) = &resolved.secrets.tls_key {
        println!("Private key: {}", private_key.display());
    }

    // The materialized files remain valid until `resolved` is dropped.
    Ok(())
}
```

