+++
title = "providers-onepassword-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# 1Password Provider

> 1Password secrets management integration

The [1Password](https://1password.com/) provider integrates with 1Password for team-based secret management with advanced access controls.

## At a glance

|                 |                                                                             |
| --------------- | --------------------------------------------------------------------------- |
| Provider        | `onepassword`                                                               |
| URI             | `onepassword://[account@]vault`                                             |
| Access          | Read and write                                                              |
| Best for        | Team-managed secrets in 1Password vaults                                    |
| Authentication  | Desktop app integration, a service account token, or a legacy shell session |
| Default storage | Secure Note `secretspec/{project}/{profile}/{key}`                          |

## Quick start

```bash
# Set a secret
$ secretspec set DATABASE_URL --provider onepassword://Production
Enter value for DATABASE_URL: postgresql://localhost/mydb
✓ Secret 'DATABASE_URL' saved to onepassword (profile: default)


# Get a secret
$ secretspec get DATABASE_URL --provider onepassword://Production


# Run with secrets
$ secretspec run --provider onepassword://Production -- npm start
```

## Setup

### Prerequisites

* 1Password CLI (`op`)
* 1Password account

Choose one of the following authentication methods.

### Desktop app integration (recommended for local dev)

In the 1Password desktop app, open **Settings → Developer** and enable **“Integrate with 1Password CLI”**. Once enabled, `op` calls made by `secretspec` are unlocked through the desktop app via biometrics (Touch ID / Windows Hello / system password) — no shell session needed and nothing expires from under you.

Under desktop integration, `op whoami` reports `account is not signed in` even when secret access works, so `secretspec` probes auth via `op vault list` instead. It also strips any `OP_SESSION_*` environment variables from spawned `op` processes, so a stale `eval $(op signin)` session in your shell can’t shadow the desktop integration.

#### Linux note

On Linux, the desktop integration requires the `op` binary to be in the `onepassword-cli` group with the setgid bit set — the desktop app verifies the caller’s GID over its unlock socket. On NixOS this is handled automatically by `programs._1password.enable = true`. A plain `pkgs._1password-cli` install (e.g. via `nix-env` or Home Manager only) does **not** carry the setgid bit and desktop integration will fail; use the NixOS module, or fall back to a service account token for headless setups.

### Service account token

In SecretSpec 0.15 and later, you can declare the token as a [provider credential](/reference/provider-credentials/), for example to load it from your keyring:

**secretspec.toml**

```toml
[providers]
op = { uri = "onepassword://Production", credentials = { service_account_token = "keyring" } }
```

When no explicit `service_account_token` is supplied, the provider falls back to `OP_SERVICE_ACCOUNT_TOKEN`. The `onepassword+token://` scheme selects service account authentication and takes the token from one of those two sources.

> **Caution**
>
> From SecretSpec 0.19 on, the token may not be written into the URI itself (`onepassword+token://token@vault` and `onepassword+token://account:token@vault` are rejected). A URI reaches committed manifests, shell history, and CI logs, so the token belongs in a provider credential or the environment. Keep the scheme and drop the token: `onepassword+token://Production`.

### Manual signin (legacy)

Run `eval $(op signin)` to set per-shell `OP_SESSION_*` tokens. These expire after 30 minutes of inactivity; if they expire mid-session, `secretspec` falls back to desktop integration when available.

## Provider credentials

| Credential              | Environment fallback       | Available since |
| ----------------------- | -------------------------- | --------------- |
| `service_account_token` | `OP_SERVICE_ACCOUNT_TOKEN` | 0.15+           |

In 0.21+, credentials retain their exact bytes during resolution. Interfaces that require text validate UTF-8 when consuming the credential; invalid input returns an error without selecting an environment fallback. Unix CLI environments accept non-UTF-8 bytes, but cannot contain NUL bytes. A configured credential that resolves to an empty value is an error rather than a fall through to the environment.

See the complete [provider credential reference](/reference/provider-credentials/) for all supported providers and environment fallbacks.

## Configuration

### URI format

```plaintext
onepassword://[account@]vault
onepassword+token://vault
```

* `account`: Optional account shorthand
* `vault`: Target vault name (defaults to “Private”)

The `onepassword+token://` form selects service account authentication; supply the token as the `service_account_token` provider credential or through `OP_SERVICE_ACCOUNT_TOKEN`.

The URI names a vault only; item paths (e.g. `onepassword://Vault/item/field`) are rejected. To name a specific item, see [Use existing secrets](#use-existing-secrets).

### URI examples

```text
onepassword://Production
onepassword://work@DevVault
onepassword+token://Production
onepassword://
```

### Project configuration

**secretspec.toml**

```toml
[providers]
team = "onepassword://Production"


[profiles.production]
DATABASE_URL = { description = "Database URL", providers = ["team"] }
```

## Storage model

SecretSpec creates Secure Notes named `secretspec/{project}/{profile}/{key}` in the selected vault. The secret value is stored in the note’s `value` field.

## Use existing secrets

If your secrets already live in 1Password items you manage yourself, name those items with the [`ref`](/reference/configuration/#secret-references) field and route the secret at a vault with `providers`:

**secretspec.toml**

```toml
[profiles.production]
DATABASE_URL = { description = "Production DB", ref = { item = "Postgres", field = "connection-url" }, providers = ["onepassword://Infra"] }
STRIPE_API_KEY = { description = "Stripe key", ref = { item = "Stripe", field = "api key" }, providers = ["onepassword://Infra"] }
```

The coordinates translate to 1Password as follows:

* `item`: the item title or UUID. Spaces are fine.
* `field`: the field label. Without `field`, the item is read like a convention secret (its value or password field), and writes edit the `value` field.
* `vault`: overrides the URI’s default vault for this one secret, e.g. `ref = { vault = "Production", item = "infra", field = "token" }`.
* `section`: addresses a field inside a section; requires `field`.

Writes go through `op item edit`: `secretspec set` updates the referenced field in place, adding the field to the item if it is missing. Items are never created through a ref.

A ref does not pin the store. Provider resolution works as usual, so a `providers` chain can fall back to other stores, and `--provider dotenv:.env.fixtures` redirects these secrets to a fixtures file during tests.

Native reference strings from the 1Password app’s **Copy Secret Reference** (`op://vault/item/field`) are not accepted directly; pasting one into `ref` produces an error that spells out the translation:

```toml
# op://Infra/Postgres/connection-url becomes:
DATABASE_URL = { description = "Production DB", ref = { vault = "Infra", item = "Postgres", field = "connection-url" }, providers = ["onepassword://Infra"] }
```

## Advanced configuration

### Profile configuration

**secretspec.toml**

```toml
[providers]
development = "onepassword://Development"
production = "onepassword://Production"


[profiles.development.defaults]
providers = ["development"]


[profiles.production.defaults]
providers = ["production"]
```

## CI/CD

```bash
# Set token
$ export OP_SERVICE_ACCOUNT_TOKEN="ops_eyJ..."


# Run command
$ secretspec run --provider onepassword://Production -- deploy
```
