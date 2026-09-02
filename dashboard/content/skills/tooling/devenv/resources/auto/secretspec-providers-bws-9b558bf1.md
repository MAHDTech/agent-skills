+++
title = "secretspec-providers-bws-9b558bf1"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Bitwarden Secrets Manager Provider

The [Bitwarden Secrets
Manager](https://bitwarden.com/products/secrets-manager/) (BWS) provider
integrates with Bitwarden for centralized, end-to-end encrypted secret
management. SecretSpec 0.17 and later invoke the separately installed
official `bws` CLI instead of linking the Bitwarden SDK.

## At a glance

|  |  |
|----|----|
| Provider | `bws` |
| URI | `bws://[SERVER_BASE@]PROJECT_UUID` |
| Access | Read and write |
| Best for | Machine and CI/CD secrets managed in Bitwarden |
| Authentication | Machine-account access token; official `bws` CLI in SecretSpec 0.17+ |
| Build feature | `bws` |
| Default storage | Flat key names in the selected BWS project |

## Quick start

```
# Set a secret$ secretspec set DATABASE_URL --provider bws://a9230ec4-5507-4870-b8b5-b3f500587e4cEnter value for DATABASE_URL: postgresql://localhost/mydb✓ Secret 'DATABASE_URL' saved to bws (profile: default)
# Run with secrets$ secretspec run --provider bws://a9230ec4-5507-4870-b8b5-b3f500587e4c -- npm start
```

Terminal window

## Setup

### Prerequisites

- Bitwarden Secrets Manager subscription
- SecretSpec 0.17+: official [`bws`
  CLI](https://bitwarden.com/help/secrets-manager-cli/) 0.3.0 or later
  installed and available on `PATH`
- Machine account access token (`BWS_ACCESS_TOKEN` environment variable)
- Build with `--features bws`

Set `SECRETSPEC_BWS_CLI_PATH` to the executable path if `bws` is not on
`PATH` (SecretSpec 0.17+).

### Authentication

Generate a machine account access token from the Bitwarden Secrets
Manager web interface.

In SecretSpec 0.15 and later, you can declare the access token as a
[provider credential](https://secretspec.dev/reference/provider-credentials/), for example to
store it in your system keyring so it never lives in a shell profile:

```
[providers]bitwarden = { uri = "bws://a9230ec4-5507-4870-b8b5-b3f500587e4c", credentials = { access_token = "keyring" } }
```

secretspec.toml

When no explicit `access_token` credential is supplied, the provider
falls back to `BWS_ACCESS_TOKEN`:

```
$ export BWS_ACCESS_TOKEN="0.your-access-token..."
```

Terminal window

SecretSpec 0.14 supports only the environment-variable form.

## Provider credentials

| Credential     | Environment fallback | Available since |
|----------------|----------------------|-----------------|
| `access_token` | `BWS_ACCESS_TOKEN`   | 0.15+           |

See the complete [provider credential
reference](https://secretspec.dev/reference/provider-credentials/) for all supported providers
and environment fallbacks.

## Configuration

### URI format

```
bws://[SERVER_BASE@]PROJECT_UUID
```

- `PROJECT_UUID`: Your Bitwarden Secrets Manager project UUID
- `SERVER_BASE` (optional): Hostname of the Bitwarden instance for EU
  cloud or self hosted deployments. Defaults to `vault.bitwarden.com`
  (US cloud) when omitted.

In SecretSpec 0.17 and later, `SERVER_BASE` is passed to each CLI
invocation as `--server-url https://SERVER_BASE`, overriding the CLI’s
saved server configuration. Use the web vault hostname here, for example
`vault.bitwarden.eu` for the EU cloud. Only a bare hostname is supported
(no scheme prefix or custom port). SecretSpec 0.16 and earlier
configured the same derived `/identity` and `/api` endpoints directly
through the SDK.

### URI examples

```
bws://a9230ec4-5507-4870-b8b5-b3f500587e4cbws://vault.bitwarden.eu@a9230ec4-5507-4870-b8b5-b3f500587e4cbws://bw.example.com@a9230ec4-5507-4870-b8b5-b3f500587e4c
```

### Project configuration

```
[providers]bitwarden = { uri = "bws://a9230ec4-5507-4870-b8b5-b3f500587e4c", credentials = { access_token = "keyring" } }
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["bitwarden"] }
```

secretspec.toml

## Storage model

SecretSpec uses flat key names matching the secret key directly, such as
`DATABASE_URL`. The BWS project UUID provides namespace isolation, so
use separate BWS projects when applications or environments need
separate values.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names a different key instead: `item` is the BWS key name (`field` is
not supported). Reads and writes target that key in place.

```
[profiles.production]DATABASE_URL = { description = "DB", ref = { item = "prod-db-connection" }, providers = ["bws://a9230ec4-5507-4870-b8b5-b3f500587e4c"] }
```

## CI/CD

```
# Set access token (from CI secrets)$ export BWS_ACCESS_TOKEN="$BWS_TOKEN"
# Run command$ secretspec run --provider bws://a9230ec4-5507-4870-b8b5-b3f500587e4c -- deploy
```

Terminal window

## Security considerations

SecretSpec passes the access token to `bws` through `BWS_ACCESS_TOKEN`,
not the CLI’s `--access-token` argument. The official CLI requires new
or updated secret values as command-line arguments, however, so during
`secretspec set` the value may briefly be visible to process-inspection
tools available to the same user. This applies to the CLI-backed
provider in SecretSpec 0.17 and later.

