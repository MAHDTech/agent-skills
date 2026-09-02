# Azure Key Vault Provider

The [Azure Key
Vault](https://azure.microsoft.com/en-us/products/key-vault) provider
integrates with Azure for centralized secret management.

## At a glance

|  |  |
|----|----|
| Provider | `akv` |
| URI | `akv://VAULT_NAME[?auth=METHOD][&suffix=DNS_SUFFIX]` |
| Access | Read and write; secret references are read-only |
| Best for | Workloads and teams on Azure |
| Authentication | Service principal, Azure CLI, managed identity, or workload identity |
| Availability | SecretSpec 0.15+; requires the `akv` build feature |
| Default storage | `secretspec--{base32(project)}--{base32(profile)}--{base32(key)}` |

## Quick start

```
# Set a secret$ secretspec set DATABASE_URL --provider akv://myvaultEnter value for DATABASE_URL: postgresql://localhost/mydb✓ Secret 'DATABASE_URL' saved to akv (profile: default)
# Get it back$ secretspec get DATABASE_URL --provider akv://myvaultpostgresql://localhost/mydb
```

Terminal window

## Setup

### Prerequisites

- An Azure Key Vault instance
- Authenticated via a service principal, the Azure CLI (`az login`), a
  managed identity, or AKS workload identity
- Build with `--features akv`

### Authentication

Select an authentication mode with the URI’s `auth` option:

- `env` (default): service-principal provider credentials or environment
  variables, falling back to an Azure CLI session when none are set.
- `cli`: Azure CLI or Azure Developer CLI only.
- `managed_identity`: system-assigned managed identity.
- `workload_identity`: AKS workload identity federation.

## Provider credentials

| Credential      | Environment fallback  | Available since |
|-----------------|-----------------------|-----------------|
| `tenant_id`     | `AZURE_TENANT_ID`     | 0.15+           |
| `client_id`     | `AZURE_CLIENT_ID`     | 0.15+           |
| `client_secret` | `AZURE_CLIENT_SECRET` | 0.15+           |

See the complete [provider credential
reference](https://secretspec.dev/reference/provider-credentials/) for all supported providers
and environment fallbacks.

## Configuration

### URI format

```
akv://VAULT_NAME[?auth=env|cli|managed_identity|workload_identity][&suffix=DNS_SUFFIX]
```

- `VAULT_NAME`: Your Key Vault name (e.g. `myvault`), or a full DNS name
  for sovereign clouds (e.g. `myvault.vault.azure.cn`)
- `auth`: Authentication method (default: `env`)
  - `env` — a service principal from the `tenant_id`, `client_id`, and
    `client_secret` provider credentials, with
    `AZURE_TENANT_ID`/`AZURE_CLIENT_ID`/`AZURE_CLIENT_SECRET` as
    fallbacks (all three must be available together); falls back to the
    signed-in Azure CLI / Azure Developer CLI session if none are
    available. A partial set is an error rather than a silent fallback
    to a different identity.
  - `cli` — the Azure CLI / Azure Developer CLI session only
  - `managed_identity` — the VM / App Service / AKS system-assigned
    managed identity
  - `workload_identity` — AKS workload identity federation
    (`AZURE_TENANT_ID`/`AZURE_CLIENT_ID`/`AZURE_FEDERATED_TOKEN_FILE`,
    injected automatically by AKS)
- `suffix`: an explicit Key Vault DNS suffix for a bare `VAULT_NAME`,
  e.g. `akv://myvault?suffix=vault.azure.cn` for a sovereign cloud,
  instead of relying on a dotted `VAULT_NAME`

### URI examples

```
akv://myvaultakv://myvault?auth=managed_identityakv://myvault?auth=workload_identityakv://myvault?suffix=vault.azure.cn
```

### Project configuration

```
[providers]azure = "akv://myvault"
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["azure"] }
```

secretspec.toml

## Storage model

Azure Key Vault secret names may only contain ASCII letters, digits and
hyphens, and Azure compares object identifiers case-insensitively.
SecretSpec stores convention names as
`secretspec--{base32(project)}--{base32(profile)}--{base32(key)}`, using
lowercase, unpadded Base32 for each component.

This encoding is deterministic and injective: names that differ by case,
underscores versus hyphens, or leading/trailing hyphens remain distinct
even though Key Vault’s identifiers do not preserve all of those
distinctions. The encoded components contain no hyphens, so the `--`
component separators cannot be confused with component data.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names an existing secret instead: `item` is the secret name (`field` and
`version` are both rejected through SecretSpec 0.18). `field` remains
unsupported. SecretSpec 0.20+ accepts `version` as a 32-character ASCII
alphanumeric Azure Key Vault version identifier; omission reads the
latest version. References are **read-only** in this provider, and
`item` must already be a valid Azure Key Vault secret name (letters,
digits, and hyphens only) — unlike convention secrets, it is validated
but never rewritten, since silently rewriting a `ref` could point at a
different secret than the one you named.

```
[profiles.production]DATABASE_URL = {  description = "DB",  ref = { item = "database-url", version = "0123456789abcdef0123456789abcdef" }, # version: 0.20+  providers = ["akv://myvault"]}
```

## CI/CD

### Service principal

The `auth=env` mode accepts `tenant_id`, `client_id`, and
`client_secret` as [provider
credentials](https://secretspec.dev/reference/provider-credentials/). For example, the
credentials can be stored in the system keyring instead of a shell
profile:

```
[providers.azure]uri = "akv://myvault"
[providers.azure.credentials]tenant_id = "keyring"client_id = "keyring"client_secret = "keyring"
```

secretspec.toml

Store all three declared credentials, then use the alias:

```
$ secretspec config provider login azure
$ secretspec run --provider azure -- deploy
```

Terminal window

When a semantic credential is not explicitly configured, SecretSpec
falls back to its matching conventional environment variable:

```
# Set credentials$ export AZURE_TENANT_ID="..."
$ export AZURE_CLIENT_ID="..."
$ export AZURE_CLIENT_SECRET="..."
# Run command$ secretspec run --provider akv://myvault -- deploy
```

Terminal window

Across provider credentials and environment fallbacks, all three values
must be available together. A partial service principal is treated as a
configuration error rather than a silent fallback to the Azure CLI
session.

### AKS workload identity

```
# AZURE_TENANT_ID, AZURE_CLIENT_ID, and AZURE_FEDERATED_TOKEN_FILE are# injected automatically into workload-identity-enabled pods.$ secretspec run --provider akv://myvault?auth=workload_identity -- deploy
```

Terminal window
