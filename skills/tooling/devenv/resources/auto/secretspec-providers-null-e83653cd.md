# Null Provider

The [null](https://man7.org/linux/man-pages/man4/null.4.html) provider
always reports that a value is missing. SecretSpec can then use the
declaration’s committed `default`, generate a fresh value, or—in
SecretSpec 0.19+—ask the operator during `run` when `prompt = true`.
This is useful for non-sensitive environment configuration and values
that should exist for only one invocation or resolution.

## At a glance

|  |  |
|----|----|
| Provider | `null` (0.19+) |
| URI | `null://` |
| Access | Always returns missing; ordinary writes are rejected |
| Best for | Team-shared defaults, ephemeral generated values, and operator-supplied run values (0.19+) |
| Storage | None |

## Quick start

Route committed defaults to `null`:

```
[profiles.default]SPRING_PROFILES_ACTIVE = { description = "Spring application profile", default = "local", providers = ["null"] }
[profiles.staging]SPRING_PROFILES_ACTIVE = { default = "staging" }
```

secretspec.toml

```
$ secretspec run --profile staging -- mvn spring-boot:run
```

Terminal window

This keeps the application mode aligned with the SecretSpec profile and
its secrets. The same pattern works for values such as `LOCAL_PORT`.

## Ephemeral generation

Route a generated secret to `null` when each materializing resolution
should receive a fresh value without storing it in a provider:

```
[profiles.default]SESSION_SECRET = { description = "Per-run session secret", type = "base64", generate = { bytes = 32 }, providers = ["null"] }
```

secretspec.toml

`secretspec run` generates `SESSION_SECRET` once for the resolved
environment and gives that value to the child process. A later `run`,
`get`, `check`, or SDK value-carrying resolution generates a new value.
Value-free reports mark the secret as generated without minting it.

## Ephemeral operator input

Combine `prompt = true` with `null` when the value must always come from
the operator and must never be stored:

```
[profiles.default]DEPLOY_PASSWORD = { description = "One-time deployment password", required = true, prompt = true, providers = ["null"] }
```

secretspec.toml

`secretspec run -- ./deploy` reads the value through a hidden
controlling terminal prompt, without consuming the child’s stdin. The
answer is present in the child environment for that invocation and is
then discarded. It is never passed to `null.set()` or written to a
cache. A noninteractive run fails before the child starts; other
commands and SDK resolution do not prompt.

## How it works

SecretSpec normally asks the selected provider before using a default or
generating a missing secret. `null` cannot read or store values: reads
always report a missing value, and every ordinary write is rejected. The
missing read lets SecretSpec use the committed default or generator
without provider I/O.

The provider has no options, credentials, feature flag, or persistent
state. Use it on declarations with defaults, enabled generation, or
`prompt = true` (0.19+). Here `prompt` chooses operator input while
`null` chooses ephemeral handling; with a writable provider the same
prompted answer would be saved. Required declarations with none of those
remain missing, and explicit writes are rejected.
