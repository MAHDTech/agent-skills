+++
title = "secretspec-providers-vault-7809a2b8"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Vault Provider

The [Vault](https://developer.hashicorp.com/vault) provider integrates
with HashiCorp Vault for centralized secret management using the KV
(Key-Value) secrets engine.

## At a glance

|  |  |
|----|----|
| Provider | `vault` |
| URI | `vault://[namespace@]host[:port][/mount][?options]` |
| Access | Read, write, and delete (0.17+); secret references are read-only |
| Best for | Self-managed, policy-controlled secret infrastructure |
| Authentication | Token or AppRole; JWT/OIDC (0.17+) |
| Build feature | `vault` |
| Default storage | KV path `secretspec/{project}/{profile}/{key}`, field `value` |

## Quick start

```
# With default "secret" mount$ secretspec set DATABASE_URL --provider vault://vault.example.com:8200Enter value for DATABASE_URL: postgresql://localhost/mydb✓ Secret 'DATABASE_URL' saved to vault (profile: default)
```

Terminal window

## Setup

### Prerequisites

- A running Vault server
- Authentication credentials
- KV secrets engine enabled (v1 or v2)
- Build with `--features vault`

### Token authentication

Token authentication is the default. SecretSpec reads `VAULT_TOKEN` or
`~/.vault-token`:

```
$ export VAULT_TOKEN=hvs.your-token-here
```

Terminal window

### AppRole authentication

Select AppRole with `?auth=approle`. Vault roles bind a SecretID by
default, so the usual configuration provides both environment variables:

```
$ export VAULT_ROLE_ID=your-role-id
$ export VAULT_SECRET_ID=your-secret-id
```

Terminal window

Starting with SecretSpec 0.15, these credentials can instead be read
from another provider so they do not live in a shell profile:

```
[providers.vault_approle]uri = "vault://vault.example.com:8200/secret?auth=approle"
[providers.vault_approle.credentials]role_id = { provider = "onepassword", ref = { vault = "Infra", item = "vault-approle", field = "role_id" } }secret_id = { provider = "onepassword", ref = { vault = "Infra", item = "vault-approle", field = "secret_id" } }
```

secretspec.toml

SecretSpec 0.14 supports only `VAULT_ROLE_ID` and `VAULT_SECRET_ID`.

Disabling SecretID binding removes AppRole’s usual second credential.
Keep the server default unless the workload deliberately relies on
another trust boundary, such as a tightly controlled Agent host and
network constraints.

### Custom authentication mounts

AppRole and JWT methods mounted somewhere other than their defaults can
be selected with `?auth_mount=`. The value is relative to `/v1/auth`:

```
vault://vault.example.com:8200/secret?auth=approle&auth_mount=platform-approlevault://vault.example.com:8200/secret?auth=jwt&auth_mount=ci-jwt&role=ci
```

The provider logs in at `/v1/auth/platform-approle/login` and
`/v1/auth/ci-jwt/login`, respectively. The KV mount remains the provider
URI path (`secret` in these examples).

### JWT / OIDC authentication

Select JWT with `?auth=jwt`. The provider performs the `auth/jwt/login`
exchange itself. The JWT comes from `VAULT_JWT` when set. Otherwise, in
a GitHub Actions or Forgejo job with `id-token: write`, the provider
mints one from the runner’s OIDC identity, so CI stores no static
secret.

Starting with SecretSpec 0.18, the role may be omitted when the JWT auth
mount has a `default_role`; Vault then selects that role during login.
An explicit SecretSpec role still takes precedence.

Both `role` and `audience` accept a URI query parameter or an
environment variable:

- `?role=` or `VAULT_JWT_ROLE`; optional with a server-configured
  `default_role` (0.18+)
- `?audience=` or `VAULT_JWT_AUDIENCE`, matched against the role’s
  `bound_audiences`

## Provider credentials

| Credential  | Environment fallback | Available since |
|-------------|----------------------|-----------------|
| `role_id`   | `VAULT_ROLE_ID`      | 0.15+           |
| `secret_id` | `VAULT_SECRET_ID`    | 0.15+           |
| `token`     | `VAULT_TOKEN`        | 0.15+           |

See the complete [provider credential
reference](https://secretspec.dev/reference/provider-credentials/) for all supported providers
and environment fallbacks.

## Configuration

### URI format

```
vault://[namespace@]host[:port][/mount][?key=value&...]
```

- `host[:port]`: Vault server address (falls back to `VAULT_ADDR`)
- `mount`: KV engine mount path (default: `secret`)
- `namespace@`: Optional Vault namespace (also reads `VAULT_NAMESPACE`)
- `?auth=approle`: Use AppRole authentication (default: `token`)
- `?auth=jwt` (0.17+): Use JWT/OIDC authentication; a server-configured
  `default_role` can supply the role when using SecretSpec 0.18+
- `?auth_mount=` (0.18+): Non-default AppRole or JWT mount beneath
  `/v1/auth`
- `?role=` (0.17+): Vault role for JWT auth (or `VAULT_JWT_ROLE`)
- `?audience=` (0.17+): OIDC audience (or `VAULT_JWT_AUDIENCE`)
- `?kv=1`: Use KV v1 (default: v2)
- `?tls=false`: Disable TLS for development servers

### Concurrent resolution

- One HTTP client is reused per provider instance (connection pool / h2
  reuse).
- Concurrent unique-address fetches are capped at 8 by default.
- Override the cap with `SECRETSPEC_PROVIDER_CONCURRENCY` (integer ≥ 1)
  when your Vault proxy tolerates more or less parallel load.

### URI examples

```
vault://vault.example.com:8200/secretvault://team-a@vault.example.com:8200/secretvault://vault.example.com:8200/secret?auth=approle# SecretSpec 0.18+vault://vault.example.com:8200/secret?auth=approle&auth_mount=platform-approle# SecretSpec 0.17+vault://vault.example.com:8200/secret?auth=jwt&role=ci# SecretSpec 0.18+, with default_role configured on the JWT auth mountvault://vault.example.com:8200/secret?auth=jwt
```

### Project configuration

```
[providers]vault_prod = "vault://vault.example.com:8200/secret"
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["vault_prod"] }
```

secretspec.toml

## Storage model

Each secret is stored at `secretspec/{project}/{profile}/{key}` under
the configured mount, with its value in a field named `value`.

For KV v2, `DATABASE_URL` for project `myapp` and profile `production`
is read from
`GET /v1/secret/data/secretspec/myapp/production/DATABASE_URL`.

## Provider caching

A KV v2 mount can hold a [cached provider
route’s](https://secretspec.dev/concepts/providers/caching/) entries. Vault expires them
itself: the cache’s `max_age` is written to the path’s
`delete_version_after` metadata, so a cached copy of another store’s
secret stops existing at that age even if SecretSpec never runs again.

```
[providers]slow = "onepassword://Production"shared_cache = "vault://vault.example.com:8200/secret"
myprovider = { fallback = ["slow"], cache = { provider = "shared_cache", max_age = "8h" } }
```

secretspec.toml

This needs write access to the path’s metadata as well as its data. KV
v1 has no expiry and is refused as a cache, rather than storing a copy
that would never expire.

Deleting — [`cache clear`](https://secretspec.dev/reference/cli/#cache-clear-017) and
automatic invalidation — removes the KV path’s metadata and every
version, so no soft-deleted version keeps the value recoverable. It is
confined to entries SecretSpec owns: a secret reference is never
deleted, since the path it names is managed outside SecretSpec.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names an existing KV entry: `item` is the KV path relative to the mount,
and `field` selects the field to read. `field` is required because KV
entries are maps. References are **read-only** in this provider.

```
[profiles.production]DATABASE_URL = { description = "DB", ref = { item = "myapp/config", field = "db_url" }, providers = ["vault://vault.example.com:8200/secret"] }
```

The mount is not a ref coordinate: it comes from the provider URI
(`secret` in the example). To read one secret from a different mount,
give that secret a provider entry whose URI names the mount.

## CI/CD

SecretSpec 0.16 can use AppRole to keep a user token out of the
environment by logging in from `VAULT_ROLE_ID` and `VAULT_SECRET_ID`:

```
$ export VAULT_ROLE_ID="$CI_VAULT_ROLE_ID"
$ export VAULT_SECRET_ID="$CI_VAULT_SECRET_ID"
$ secretspec export --format gha --provider "vault://vault.example.com:8200/secret?auth=approle"
```

Terminal window

SecretSpec 0.17 adds a tokenless JWT/OIDC path. Under GitHub Actions or
Forgejo Actions with `id-token: write`, the provider mints the job’s
OIDC token and logs in with a role bound to the workflow’s claims:

```
$ secretspec export --format gha --provider "vault://vault.example.com:8200/secret?auth=jwt&role=ci"
```

Terminal window

## Advanced configuration

### KV version 1

```
$ secretspec set DATABASE_URL --provider "vault://vault.example.com:8200/secret?kv=1"
```

Terminal window

### Vault namespaces

```
$ secretspec check --provider vault://team-a@vault.example.com:8200/secret
$ export VAULT_NAMESPACE=team-a
$ secretspec check --provider vault://vault.example.com:8200/secret
```

Terminal window

### Development mode

```
$ vault server -dev
$ export VAULT_TOKEN=hvs.dev-root-token
$ secretspec check --provider "vault://127.0.0.1:8200/secret?tls=false"
```

Terminal window

