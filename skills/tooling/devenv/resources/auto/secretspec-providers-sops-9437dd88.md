# SOPS Provider

The `sops` provider reads and writes secrets in files encrypted with
[SOPS](https://getsops.io).

## At a glance

|                |                                                           |
|----------------|-----------------------------------------------------------|
| Provider       | `sops`                                                    |
| URI            | `sops://[PATH][?options]`                                 |
| Access         | Read and write                                            |
| Best for       | Encrypted files stored alongside a project                |
| Authentication | SOPS key configuration or SecretSpec provider credentials |
| Build feature  | `sops`                                                    |

## Quick start

After installing SOPS and configuring a creation rule or another
encryption method, write a secret to an encrypted YAML file:

```
$ secretspec set DATABASE_URL --provider sops://secrets.enc.yaml
```

Terminal window

Use the same provider to inject the secret into a command:

```
$ secretspec run --provider sops://secrets.enc.yaml -- npm start
```

Terminal window

See [Setup](#setup) if SOPS does not already know which keys to use.

## Setup

### Prerequisites

- The SOPS CLI available within the environment:

  - [Manually download a release
    binary](https://github.com/getsops/sops/releases)

  - [Use the SOPS Nix
    package](https://search.nixos.org/packages?channel=unstable&query=sops#show=sops)

  - Install with a package manager:

    ```
    # Homebrew$ brew install sops
    # Arch$ sudo pacman -S sops
    ```

    Terminal window

- The keys or credentials required by the selected SOPS encryption
  method

- Build SecretSpec with `--features sops` when the provider is not
  included by your package

For a new file, SOPS needs either encryption options in the provider URI
or a matching creation rule in `.sops.yaml`. Generate an age identity
for your own project and print its recipient:

```
$ age-keygen -o key.txt
$ age-keygen -y key.txtage1...
$ export SOPS_AGE_KEY_FILE="$PWD/key.txt"
```

Terminal window

Keep `key.txt` secret and out of version control. `SOPS_AGE_KEY_FILE`
makes the identity available for decryption. Copy the `age1...`
recipient printed by the second command into the creation rule (the
value below is a placeholder, not a usable recipient):

```
creation_rules:  - path_regex: secrets\.enc\.yaml$    age: "YOUR_AGE_RECIPIENT"
```

.sops.yaml

## Provider credentials

Secret values used to authenticate SOPS belong in a provider alias’s
`credentials` map, not in the SOPS URI.

| Credential                  | Environment fallback        | Available since |
|-----------------------------|-----------------------------|-----------------|
| `age_key`                   | `SOPS_AGE_KEY`              | 0.17+           |
| `aws_secret_access_key`     | `AWS_SECRET_ACCESS_KEY`     | 0.17+           |
| `azure_client_secret`       | `AZURE_CLIENT_SECRET`       | 0.17+           |
| `hc_vault_token`            | `VAULT_TOKEN`               | 0.17+           |
| `huawei_sdk_ak`             | `HUAWEICLOUD_SDK_AK`        | 0.17+           |
| `huawei_sdk_sk`             | `HUAWEICLOUD_SDK_SK`        | 0.17+           |
| `google_oauth_access_token` | `GOOGLE_OAUTH_ACCESS_TOKEN` | 0.17+           |

See the complete [provider credential
reference](https://secretspec.dev/reference/provider-credentials/) for all supported providers
and environment fallbacks.

For example, this alias loads an age identity from the system keyring
and passes it only to the SOPS child process. Replace
`YOUR_AGE_RECIPIENT` with the recipient printed by `age-keygen -y`
during setup:

```
[providers.sops_age]uri = "sops://secrets.enc.yaml?age_recipients=YOUR_AGE_RECIPIENT"
[providers.sops_age.credentials]age_key = "keyring"
[profiles.production.defaults]providers = ["sops_age"]
```

secretspec.toml

When a credential is not declared on the alias, SOPS can still use its
normal environment variable.

## Configuration

### URI format

```
sops://[path/to/secret][?key=value[&key=value]...]
```

- `path/to/secret` — optional absolute or relative path to the encrypted
  file; defaults to `secrets.enc.yaml`. Relative paths are resolved from
  the directory containing `secretspec.toml`, not from the shell’s
  current directory.

  - 

- `?key=value` — optional query parameter; see [Query
  parameters](#query-parameters)

- `&key=value` — additional parameters

Relative `age_key_file`, `age_ssh_private_key_file`, and `sops_config`
query values are resolved from the same manifest directory.

### Project configuration

Use an alias to keep the storage path and encryption settings in
`secretspec.toml`:

```
[providers]encrypted_file = "sops://secrets/{project}/{profile}.enc.yaml"
[profiles.default.defaults]providers = ["encrypted_file"]
```

secretspec.toml

Templated paths must contain both `{project}` and `{profile}`. Use a
single-file URI when every project and profile should share one
encrypted document.

### Query parameters

Except for the SecretSpec-specific `format` parameter, see the [SOPS
documentation](https://getsops.io/docs/#usage) for the purpose and usage
of each parameter.

#### SecretSpec

| Provider URL Query Parameter Name | Purpose |
|----|----|
| format | Overrides the extension-based file format detection. Valid values: `dotenv` `env` `ini` `json` `yaml` `yml` |

#### SOPS

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| sops_config                       | SOPS_CONFIG                        |
| sops_decryption_order             | SOPS_DECRYPTION_ORDER              |
| sops_editor                       | SOPS_EDITOR                        |
| sops_enable_local_keyservice      | SOPS_ENABLE_LOCAL_KEYSERVICE       |
| sops_keyservice                   | SOPS_KEYSERVICE                    |

#### Age

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| age_key_cmd                       | SOPS_AGE_KEY_CMD                   |
| age_key_file                      | SOPS_AGE_KEY_FILE                  |
| age_recipients                    | SOPS_AGE_RECIPIENTS                |
| age_ssh_private_key_cmd           | SOPS_AGE_SSH_PRIVATE_KEY_CMD       |
| age_ssh_private_key_file          | SOPS_AGE_SSH_PRIVATE_KEY_FILE      |

#### AWS

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| aws_access_key_id                 | AWS_ACCESS_KEY_ID                  |
| aws_profile                       | AWS_PROFILE                        |
| aws_region                        | AWS_REGION                         |
| kms_arn                           | SOPS_KMS_ARN                       |

#### GCP

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| gcp_kms_client_type               | SOPS_GCP_KMS_CLIENT_TYPE           |
| gcp_kms_endpoint                  | SOPS_GCP_KMS_ENDPOINT              |
| gcp_kms_ids                       | SOPS_GCP_KMS_IDS                   |
| gcp_kms_universe_domain           | SOPS_GCP_KMS_UNIVERSE_DOMAIN       |

#### Azure

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| azure_client_id                   | AZURE_CLIENT_ID                    |
| azure_keyvault_urls               | SOPS_AZURE_KEYVAULT_URLS           |
| azure_tenant_id                   | AZURE_TENANT_ID                    |

#### PGP

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| pgp_fp                            | SOPS_PGP_FP                        |

#### GPG

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| gpg_exec                          | SOPS_GPG_EXEC                      |

#### HashiCorp Vault/OpenBao

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| hc_vault_addr                     | VAULT_ADDR                         |
| hc_vault_allowlist                | SOPS_HC_VAULT_ALLOWLIST            |

#### Huawei Cloud

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| huawei_kms_ids                    | SOPS_HUAWEICLOUD_KMS_IDS           |
| huawei_sdk_project_id             | HUAWEICLOUD_SDK_PROJECT_ID         |

Each query option in the environment-mapping tables is exported to the
SOPS child process and overrides the same inherited environment
variable. SOPS configuration resolves in this order: `sops_config` in
the URI, a `.sops.yaml` discovered from the manifest directory or one of
its parents, then an inherited `SOPS_CONFIG`. Provider credentials
similarly override their matching secret environment fallbacks, but only
for the SOPS child process.

## Storage model

The URI shape determines the on-disk layout. This matters when editing a
file with `sops` directly and when diagnosing a value set under the
wrong profile.

For a **single YAML or JSON file**, convention-addressed secrets are
written at `[project][profile][key]`. For example,
`secretspec set API_KEY --profile production` in project `my-app`
writes:

```
my-app:  production:    API_KEY: secret-value
```

secrets.enc.yaml (decrypted view)

The equivalent selector passed to `sops set` is
`["my-app"]["production"]["API_KEY"]`. Reads also retain compatibility
with older `[profile][key]` and root `[key]` layouts, but new convention
writes use the fully namespaced path.

For a **templated YAML or JSON URI**, `{project}` and `{profile}`
already select the file, so the key is flat inside it:

```
[providers]prod_sops = "sops://secrets/{project}/{profile}.enc.yaml"
```

```
API_KEY: secret-value
```

secrets/my-app/production.enc.yaml (decrypted view)

Single-file dotenv is always a flat `API_KEY=value` document.
Single-file INI uses the selected profile as its section; templated INI
uses `[DEFAULT]` because the profile is already represented by the
filename. The following table is the write layout:

| Format      | Single file               | Templated path   |
|-------------|---------------------------|------------------|
| YAML / JSON | `[project][profile][key]` | root `[key]`     |
| dotenv      | root `[key]`              | root `[key]`     |
| INI         | `[profile][key]`          | `[DEFAULT][key]` |

In SecretSpec 0.19+, `secretspec set` and interactive `secretspec check`
print the resolved provider URI, profile, file, and selector before
prompting for or writing the value. This makes an accidentally omitted
`--profile` visible before the encrypted file changes.

### Format handling

SecretSpec asks SOPS to emit JSON when decrypting and selects the secret
from that JSON representation. When `?format=` is present, SecretSpec
passes the corresponding SOPS input type where SOPS supports one, so
filenames such as `.env.production.enc?format=dotenv` work correctly.

SOPS does not support `--input-type ini`, so `?format=ini` is accepted
only when the filename itself ends in `.ini`. Without `?format=`, the
filename must end in `.yaml`, `.yml`, `.json`, `.env`, `.dotenv`, or
`.ini`; an unrecognized extension is reported as a configuration error.

When a selected YAML or JSON node is not a string, reads return its
compact JSON representation. Writes always store the supplied secret as
a string value.

In a single dotenv file, the same key cannot hold different values for
different profiles. Use a templated path such as
`secrets/{project}/.env.{profile}.enc?format=dotenv` when profiles need
separate dotenv values.

## Use existing secrets

SOPS supports `ref = { item = "..." }` against a **single-file**
provider URI. For YAML, JSON, and dotenv, the item names a root key and
does not add the project/profile convention path. For INI, it names a
key in the `[DEFAULT]` section:

```
[providers]shared_sops = "sops://shared.enc.yaml"
[profiles.production]EXTERNAL_TOKEN = { description = "Token already managed in shared.enc.yaml", ref = { item = "existing_token" }, providers = ["shared_sops"] }
```

secretspec.toml

This YAML example reads or writes root selector `["existing_token"]`;
the INI equivalent is `["DEFAULT"]["existing_token"]`. This provider
treats the value at the selected key as the complete secret, so it
supports only `item` and rejects `field` and other extra ref
coordinates. A templated SOPS URI also rejects refs: without convention
`project`/`profile` inputs, SecretSpec cannot choose which templated
file the external item belongs to. See [Secret
References](https://secretspec.dev/concepts/references/) for the general model.

## Advanced configuration

### Pass age settings in the provider URI

The setup above uses `.sops.yaml` and `SOPS_AGE_KEY_FILE`. To keep both
settings in the provider URI instead, derive the recipient from the same
identity:

```
$ AGE_RECIPIENT="$(age-keygen -y key.txt)"
$ secretspec set DATABASE_URL --provider "sops://secrets.enc.json?age_key_file=key.txt&age_recipients=${AGE_RECIPIENT}"
```

Terminal window

## CI/CD

Make the selected SOPS key service available to the job through provider
credentials or its standard environment variables, then run SecretSpec
with the configured alias:

```
$ secretspec run --profile production --provider sops_age -- deploy
```

Terminal window

Provider credentials are exposed only to the SOPS child process. Store
their backing values in the CI platform’s secret store rather than in
the provider URI.

## Security considerations

- Commit only the SOPS-encrypted files, never decrypted copies or
  private key material.
- Keep secret authentication values in provider credentials or SOPS
  environment variables instead of URI query parameters.
- Review the resolved file and selector shown by `secretspec set` and
  interactive `secretspec check` in SecretSpec 0.19+ before confirming a
  write.
- Use templated paths when profiles must not share the same dotenv key.
