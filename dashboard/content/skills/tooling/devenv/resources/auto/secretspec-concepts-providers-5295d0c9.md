+++
title = "secretspec-concepts-providers-5295d0c9"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# Providers

A provider is a storage backend from which SecretSpec reads secrets and,
when supported, writes them. Providers let one `secretspec.toml`
describe the secrets an application needs without requiring every
environment to use the same secret store.

For example, a developer can use the system keyring, CI can supply
environment variables, and production can use a shared password manager
or cloud secret manager. The secret definitions stay the same; only
their provider configuration changes.

## Provider specifications

Anywhere SecretSpec accepts a provider, you can use one of three forms:

- A provider name, such as `keyring` or `env`.
- A provider URI, such as `dotenv://.env.local` or
  `onepassword://Production`. The URI configures a particular instance
  of the provider.
- A provider alias, such as `prod_vault`, defined in project or user
  configuration.

Aliases are useful when a URI is shared by several secrets or should
have a meaningful, store-independent name.

```
[providers]prod_vault = "onepassword://Production"
[profiles.production]DATABASE_URL = { description = "Production database", providers = ["prod_vault"] }
```

secretspec.toml

## Available providers

| Provider | Storage backend | Read | Write | Encrypted at rest | TPM-backed keys |
|----|----|----|----|----|----|
| [keyring](https://secretspec.dev/providers/keyring/) | [macOS Keychain](https://support.apple.com/guide/security/keychain-data-protection-secb0694df1a/web), [Windows Credential Manager](https://learn.microsoft.com/windows/win32/secauthn/credentials-management), or [Linux Secret Service](https://gnome.pages.gitlab.gnome.org/libsecret/) | ✓ | ✓ | ✓ | — |
| [kdbx](https://secretspec.dev/providers/kdbx/) (0.17+) | KeePass KDBX file (requires the `kdbx` build feature) | ✓ | KDBX 4 | ✓ | — |
| [dotenv](https://secretspec.dev/providers/dotenv/) | A `.env` file | ✓ | ✓ | ✗ | — |
| [env](https://secretspec.dev/providers/env/) | Current process environment | ✓ | ✗ | ✗ | — |
| [systemd-credential](https://secretspec.dev/providers/systemd-credential/) (0.17+) | Credentials passed to the current systemd service | ✓ | ✗ | Depends on the unit’s credential source | [Via systemd-creds](https://www.freedesktop.org/software/systemd/man/latest/systemd-creds.html) |
| [pass](https://secretspec.dev/providers/pass/) | Unix `pass` password store | ✓ | ✓ | ✓ | [Via GnuPG](https://gnupg.org/blog/20210315-using-tpm-with-gnupg-2.3.html) |
| [gopass](https://secretspec.dev/providers/gopass/) (0.15+) | `gopass` password store (git-synced, GPG-encrypted) | ✓ | ✓ | ✓ | [Via GnuPG](https://gnupg.org/blog/20210315-using-tpm-with-gnupg-2.3.html) |
| [protonpass](https://secretspec.dev/providers/protonpass/) | Proton Pass | ✓ | ✓ | ✓ | — |
| [onepassword](https://secretspec.dev/providers/onepassword/) | 1Password | ✓ | ✓ | ✓ | — |
| [lastpass](https://secretspec.dev/providers/lastpass/) | LastPass | ✓ | ✓ | ✓ | — |
| [dashlane](https://secretspec.dev/providers/dashlane/) (0.18+) | Dashlane, through the `dcli` CLI | ✓ | ✗ | ✓ | — |
| [gcsm](https://secretspec.dev/providers/gcsm/) | Google Cloud Secret Manager (requires the `gcsm` build feature) | ✓ | ✓ | ✓ | — |
| [awssm](https://secretspec.dev/providers/awssm/) | AWS Secrets Manager (requires the `awssm` build feature) | ✓ | ✓ | ✓ | — |
| [scaleway](https://secretspec.dev/providers/scaleway/) (0.17+) | Scaleway Secret Manager (requires the `scaleway` build feature) | ✓ | ✓ | ✓ | — |
| [vault](https://secretspec.dev/providers/vault/) | HashiCorp Vault (requires the `vault` build feature) | ✓ | ✓ | ✓ | — |
| [openbao](https://secretspec.dev/providers/openbao/) (0.17+) | OpenBao (requires the `openbao` build feature; 0.16 uses `openbao://` through `vault`) | ✓ | ✓ | ✓ | — |
| [bws](https://secretspec.dev/providers/bws/) | Bitwarden Secrets Manager (official `bws` CLI in SecretSpec 0.17+; requires the `bws` build feature) | ✓ | ✓ | ✓ | — |
| [akv](https://secretspec.dev/providers/akv/) | Azure Key Vault (requires the `akv` build feature) | ✓ | ✓ | ✓ | — |
| [infisical](https://secretspec.dev/providers/infisical/) (0.16+) | Infisical (requires the `infisical` build feature) | ✓ | ✓ | ✓ | — |
| [age](https://secretspec.dev/providers/age/) (0.17+) | An age-encrypted file (requires the `age` build feature) | ✓ | ✓ | ✓ | — |
| [sops](https://secretspec.dev/providers/sops/) (0.17+) | SOPS-encrypted files (requires the `sops` build feature and SOPS CLI) | ✓ | ✓ | ✓ | Depends on the configured SOPS key service |

“TPM-backed keys” means the local key used by the provider can be
protected by a [TPM
2.0](https://trustedcomputinggroup.org/resource/tpm-library-specification/)
through the provider path SecretSpec uses. Pass and Gopass inherit this
capability from GnuPG when its encryption key is moved to the TPM.
systemd credentials inherit it from
[systemd-creds](https://www.freedesktop.org/software/systemd/man/latest/systemd-creds.html),
which seals encrypted credentials to the TPM2 by default when the host
has one. [libsecret has an optional TPM2-enabled file
backend](https://gnome.pages.gitlab.gnome.org/libsecret/libsecret-tpm2.html),
but SecretSpec’s Linux keyring transport uses the Secret Service D-Bus
API rather than that file backend. macOS Keychain uses Apple’s Secure
Enclave rather than a TPM, and [Windows Vault credentials are not
protected by Credential
Guard](https://learn.microsoft.com/windows/security/identity-protection/credential-guard/how-it-works).
An em dash means SecretSpec has no documented TPM integration for that
provider; it does not describe other hardware security used internally
by the provider service. Each provider page starts with a minimal
working example, then covers setup, project configuration, storage
conventions, existing provider-native secrets, and CI/CD where
applicable.

## Configure the default provider

Run the interactive configuration command to select the user-global
provider SecretSpec uses when a secret has no provider-specific
configuration. SecretSpec 0.17+ provides an explicit `global` namespace;
the legacy spelling without it remains supported:

```
$ secretspec config global init # 0.17+
```

Terminal window

SecretSpec 0.17+ can persist the provider and profile non-interactively:

```
$ secretspec config global init --provider env --profile default
```

Terminal window

The resulting user configuration contains a default provider:

```
[defaults]provider = "keyring"profile = "development" # Optional default profile
```

~/.config/secretspec/config.toml

Use `--provider` for a one-off override, or `SECRETSPEC_PROVIDER` for
commands in the current shell or CI job:

```
# Route every secret in this command to a project .env file.$ secretspec run --provider dotenv -- npm start
# Route every secret in subsequent commands to existing environment variables.$ export SECRETSPEC_PROVIDER=env$ secretspec check
```

Terminal window

A provider URI can configure the selected backend more precisely:

```
# Select a specific 1Password vault.$ secretspec run --provider "onepassword://Development" -- npm start
# Select a specific dotenv file.$ secretspec run --provider "dotenv:/home/user/work/.env" -- npm test
```

Terminal window

## Configure provider aliases

Provider aliases can be declared at either project or user scope:

- Define project aliases in the top-level `[providers]` table in
  `secretspec.toml`. Commit these aliases so team members and CI use the
  same mapping.
- Define user aliases in `[defaults.providers]` in
  `~/.config/secretspec/config.toml`. Use these for personal mappings
  that should apply across projects.

If both scopes define the same alias, the project alias takes
precedence.

```
[providers]prod_vault = "onepassword://Production"shared_vault = "onepassword://Shared"local = "keyring://"
[profiles.production]DATABASE_URL = { description = "Production database", providers = ["prod_vault", "local"] }SENTRY_DSN = { description = "Error reporting", providers = ["shared_vault", "local"] }
```

secretspec.toml

Provider lists may combine aliases, provider names, and inline provider
URIs:

```
[profiles.production]DATABASE_URL = { description = "Production database", providers = ["onepassword://Production", "keyring"] }
```

secretspec.toml

Use the CLI to manage user-level aliases:

```
# SecretSpec 0.17+$ secretspec config global provider add prod_vault "onepassword://Production"$ secretspec config global provider list$ secretspec config global provider remove prod_vault
```

Terminal window

These commands modify only `~/.config/secretspec/config.toml`. Edit the
top-level `[providers]` table directly to change project aliases.

## Provider credentials

Some providers need credentials before they can retrieve secrets.
Examples include an access token for Bitwarden Secrets Manager, a Vault
token or AppRole credentials, a 1Password service account token, and
Azure service-principal credentials.

An alias can load these credentials from another provider. This avoids
storing long-lived provider credentials in a shell profile or CI
variable when a secure store is available.

### Use the convention address

In an alias’s `credentials` table, map each semantic credential name to
the provider that stores it:

```
[providers]keyring = "keyring://"
# Read the access token from keyring before connecting to Bitwarden.bws = { uri = "bws://a9230ec4-5507-4870-b8b5-b3f500587e4c", credentials = { access_token = "keyring" } }
```

secretspec.toml

A string value such as `"keyring"` is a provider specification.
SecretSpec reads the credential from that provider at the conventional
`{project}/{profile}/{credential}` address for the active project and
profile.

### Use an explicit address

Use a table with `provider` and `ref` when the credential already exists
at a specific provider-native address:

```
[providers.vault_prod]uri = "vault://secret/myapp?auth=approle"credentials = {  role_id = { provider = "onepassword", ref = { vault = "Infra", item = "vault-approle", field = "role_id" } },  secret_id = { provider = "onepassword", ref = { vault = "Infra", item = "vault-approle", field = "secret_id" } }}
```

secretspec.toml

The `ref` table uses the same provider-native coordinates as a secret
[`ref`](https://secretspec.dev/reference/configuration/#secret-references).

### Store provider credentials

Use `config provider login` to prompt for every credential declared by
an alias and write it to the configured source:

```
$ secretspec config provider login bwsEnter access_token for provider 'bws' (source: keyring): ****✓ stored access_token in keyring at smoke/default/access_token
```

Terminal window

You can also create a user-level alias with a convention-address
credential source from the CLI:

```
$ secretspec config global provider add bws "bws://project-uuid" --credential access_token=keyring # 0.17+$ secretspec config provider login bws
```

Terminal window

Provider credentials follow these rules:

- **Configured credentials are authoritative.** When an alias declares a
  credential, SecretSpec reads its configured source. Providers may
  still use their conventional environment variables when no explicit
  credential is supplied.
- **Credentials remain internal.** SecretSpec passes a retrieved
  credential to the destination provider in memory. It does not export
  the credential or include it in the environment of a process started
  by `secretspec run`.
- **Credential chains are one hop.** A source provider cannot require
  provider credentials of its own. SecretSpec validates this before
  accessing the provider, preventing dependency cycles.
- **Convention addresses are profile-specific.** A string source uses
  the active project and profile. Use a `ref` source when multiple
  projects or profiles should share one provider credential.
- **Names are provider-specific.** Bitwarden accepts `access_token`;
  Vault accepts `token`, `role_id`, and `secret_id`; 1Password accepts
  `service_account_token`; Azure Key Vault accepts `tenant_id`,
  `client_id`, and `client_secret`; SOPS (0.17+) accepts `age_key`,
  `aws_secret_access_key`, `azure_client_secret`,
  `google_oauth_access_token`, `hc_vault_token`, `huawei_sdk_ak`, and
  `huawei_sdk_sk`. Unsupported names are rejected before any source is
  read.

## Next steps

- Learn how [Provider fallback](https://secretspec.dev/concepts/providers/fallback/) selects
  and orders sources.
- Cache slow remote routes with [Provider
  caching](https://secretspec.dev/concepts/providers/caching/) (0.17+).
- Review the URI and authentication details for an individual provider
  in the [Providers](https://secretspec.dev/providers/keyring/) section.
- Learn how [Profiles](https://secretspec.dev/concepts/profiles/) apply provider defaults to
  an environment.
- Learn how [Secret references](https://secretspec.dev/concepts/references/) separate provider
  selection from provider-native addresses.

{% endraw %}
