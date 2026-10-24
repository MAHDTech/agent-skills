# Elixir SDK

> Resolve SecretSpec secrets from Elixir

**New in version 0.22**

The Elixir SDK (`secretspec`, 0.22+) is a pure Elixir client for the `secretspec.resolver/1` IPC protocol. It requires a compatible `secretspec` executable (0.21+) and launches `secretspec serve` directly, without a shell, NIF, native archive, or Rust code in the Hex package.

## Install

```elixir
defp deps do
  [{:secretspec, "~> 0.22"}]
end
```

The package requires Elixir 1.19 or later. Run `mix deps.get` to fetch it. Install the [SecretSpec CLI](/quick-start/) separately and make `secretspec` available on `PATH`, or set `executable:` as described below.

## Quick start

Given a `secretspec.toml` declaring `DATABASE_URL`, resolve it from the `production` profile:

```
{:ok, %SecretSpec.Secret{value: database_url}} =
  SecretSpec.with_session(
    [manifest: "secretspec.toml", profile: "production", reason: "boot web app"],
    fn session ->
      SecretSpec.Session.get(session, "DATABASE_URL",
        representation: :value,
        purpose: %{consumer: "my_app", operation: "connect"}
      )
    end
  )

# Use database_url to configure the database connection.
is_binary(database_url)
```

`SecretSpec.with_session/2` returns the callback’s result and closes the session when the callback returns or raises. Each `get/3` resolves one declared name. Handle missing secrets and resolver errors explicitly:

```
SecretSpec.with_session([manifest: "secretspec.toml", reason: "boot web app"], fn session ->
  case SecretSpec.Session.get(session, "DATABASE_URL",
         representation: :value,
         purpose: %{consumer: "my_app", operation: "connect"}
       ) do
    {:ok, %SecretSpec.Secret{value: value}} ->
      {:ok, value}

    {:missing, true} ->
      {:error, :missing_required}

    {:missing, false} ->
      {:ok, nil}

    :undeclared ->
      {:error, :undeclared}

    {:error, %SecretSpec.Error{kind: kind}} ->
      {:error, kind}
  end
end)
```

A failed session start returns `{:error, reason}` without running the callback.

## Caller context

Pass `purpose:` on every `Session.get/3` call. It must contain non-empty `consumer` and `operation` strings; `host` and `path` are optional attribution fields:

```elixir
SecretSpec.Session.get(session, "DATABASE_URL",
  representation: :value,
  purpose: %{consumer: "my_app", operation: "connect", host: "db.example.com"}
)
```

Set the session’s `reason:` to explain why secrets are being accessed. Caller attribution does not satisfy a project’s `require_reason` policy. Keep secret values out of both the purpose metadata and the reason.

## Inline specifications

Pass `manifest: %{toml: toml, base_dir: base_dir}` to embed a TOML manifest in your application. `base_dir` resolves relative provider paths. This example uses the `null://` provider to resolve a declared default without a secret store:

```
manifest = %{
  toml: """
  [project]
  name = "inline-app"
  revision = "1.0"

  [profiles.default]
  DATABASE_URL = { description = "Database URL", default = "postgres://localhost/app" }
  """,
  base_dir: File.cwd!()
}

{:ok, %SecretSpec.Secret{value: database_url}} =
  SecretSpec.with_session(
    [manifest: manifest, provider: "null://", reason: "configure local database"],
    fn session ->
      SecretSpec.Session.get(session, "DATABASE_URL",
        representation: :value,
        purpose: %{consumer: "my_app", operation: "connect"}
      )
    end
  )

is_binary(database_url)
```

## Scopes

Pass `scope: "api"` when opening the session to limit access to a named `[scopes.api]` subset:

```toml
[project]
name = "my_app"
revision = "1.0"


[profiles.default]
DATABASE_URL = { description = "Database URL" }
API_KEY = { description = "API key" }
DEPLOY_TOKEN = { description = "Deployment token" }


[scopes.api]
secrets = ["DATABASE_URL", "API_KEY"]
```

```
{:ok, %SecretSpec.Secret{value: database_url}} =
  SecretSpec.with_session(
    [manifest: "secretspec.toml", scope: "api", reason: "boot API server"],
    fn session ->
      SecretSpec.Session.get(session, "DATABASE_URL",
        representation: :value,
        purpose: %{consumer: "api", operation: "connect"}
      )
    end
  )

is_binary(database_url)
```

Requesting `DEPLOY_TOKEN` from this session returns `:undeclared`. To select a different scope, open a new session.

## Typed access

Successful results use the `%SecretSpec.Secret{}` struct. Pattern-match on `value` for inline secrets or `path` and `lease_id` for file secrets, as shown in the quick-start and file examples. `representation: :value` or `:path` makes the expected result explicit; the default, `:auto`, follows the secret’s declaration. The Elixir SDK resolves individual names and does not expose the whole-profile field map used by other SDKs’ code-generation examples.

## Files (`as_path`)

File-shaped results include a readable `path` and a `lease_id`. Keep the session and lease alive until the file’s last consumer finishes:

```
SecretSpec.with_session([manifest: "secretspec.toml", reason: "configure TLS"], fn session ->
  {:ok, %SecretSpec.Secret{path: path, lease_id: lease_id}} =
    SecretSpec.Session.get(session, "TLS_CERT",
      representation: :path,
      purpose: %{consumer: "my_app", operation: "configure_tls"}
    )

  try do
    # Read the file while its lease is active.
    certificate = File.read!(path)
    byte_size(certificate)
  after
    :ok = SecretSpec.Session.release(session, lease_id)
  end
end)
```

`Session.release/2` releases a lease explicitly. Closing the session releases all remaining leases. Consume file paths inside the `with_session/2` callback; returning a path from that callback would outlive its lease.

## Sessions

A session fixes the manifest, profile, scope, provider override, and reason for its lifetime. For multiple requests, keep a session open and close it after the last consumer finishes:

```elixir
{:ok, session} =
  SecretSpec.Session.start_link(
    manifest: "secretspec.toml",
    profile: "production",
    provider: "keyring://",
    reason: "start application"
  )


try do
  SecretSpec.Session.get(session, "DATABASE_URL",
    representation: :value,
    timeout: 5_000,
    purpose: %{consumer: "my_app", operation: "connect"}
  )
after
  SecretSpec.Session.close(session)
end
```

`timeout:` is a per-request deadline in milliseconds, defaulting to 30,000. Requests that reach their deadline return an error with `kind: "deadline_exceeded"`. `start_link/1` links the session to the calling process; `with_session/2` also arranges cleanup when its owner exits.

## Results

* `{:ok, %SecretSpec.Secret{}}`
* `{:missing, required}`
* `:undeclared`
* `{:error, %SecretSpec.Error{}}`

`%SecretSpec.Error{}` carries `kind`, `message`, and `data`. Match on `kind` for error handling. See the [resolver protocol](/reference/resolver-protocol/) for result semantics and error categories.

## Executable discovery

The SDK finds `secretspec` through `PATH`. Point a session at a particular installation with an absolute `executable:` path:

```elixir
{:ok, session} =
  SecretSpec.Session.start_link(
    executable: "/opt/secretspec/bin/secretspec",
    manifest: "secretspec.toml"
  )
```

The SDK starts the executable with `serve` and negotiates `secretspec.resolver/1`. Include the CLI and any provider executables in your deployment environment. Whole-profile resolution and value-free reports are not exposed by the Elixir SDK; use [`secretspec check`](/reference/cli/#check) for preflight checks.