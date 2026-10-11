# SOPS Provider

> Store secrets in SOPS-encrypted files

**New in version 0.17**

The [`sops`](https://getsops.io) provider reads and writes secrets in files encrypted with SOPS.

## At a glance

|                |                                                           |
| -------------- | --------------------------------------------------------- |
| Provider       | `sops`                                                    |
| URI            | `sops://[PATH][?options]`                                 |
| Access         | Read and write                                            |
| Best for       | Encrypted files stored alongside a project                |
| Authentication | SOPS key configuration or SecretSpec provider credentials |
| Build feature  | `sops`                                                    |

## Quick start

After installing SOPS and configuring a creation rule or another encryption method, write a secret to an encrypted YAML file:

```bash
$ secretspec set DATABASE_URL --provider sops://secrets.enc.yaml
```

Use the same provider to inject the secret into a command:

```bash
$ secretspec run --provider sops://secrets.enc.yaml -- npm start
```

See [Setup](#setup) if SOPS does not already know which keys to use.

## Setup

### Prerequisites

* The SOPS CLI available within the environment:

  * [Manually download a release binary](https://github.com/getsops/sops/releases)

  * [Use the SOPS Nix package](https://search.nixos.org/packages?channel=unstable\&query=sops#show=sops)

  * Install with a package manager:

    ```bash
    # Homebrew
    $ brew install sops


    # Arch
    $ sudo pacman -S sops
    ```

* The keys or credentials required by the selected SOPS encryption method

* Build SecretSpec with `--features sops` when the provider is not included by your package

For a new file, SOPS needs either encryption options in the provider URI or a matching creation rule in `.sops.yaml`. Generate an age identity for your own project and print its recipient:

```bash
$ age-keygen -o key.txt


$ age-keygen -y key.txt
age1...


$ export SOPS_AGE_KEY_FILE="$PWD/key.txt"
```

Keep `key.txt` secret and out of version control. `SOPS_AGE_KEY_FILE` makes the identity available for decryption. Copy the `age1...` recipient printed by the second command into the creation rule (the value below is a placeholder, not a usable recipient):

**.sops.yaml**

```yaml
creation_rules:
  - path_regex: secrets\.enc\.yaml$
    age: "YOUR_AGE_RECIPIENT"
```

## Provider credentials

**Changed in version 0.21**

Explicit provider credentials preserve non-UTF-8 bytes in the SOPS subprocess environment on Unix. NUL bytes are rejected before SOPS starts; other platforms require UTF-8 credentials.

Secret values used to authenticate SOPS belong in a provider alias’s `credentials` map, not in the SOPS URI.

| Credential                  | Environment fallback        | Available since |
| --------------------------- | --------------------------- | --------------- |
| `age_key`                   | `SOPS_AGE_KEY`              | 0.17+           |
| `aws_secret_access_key`     | `AWS_SECRET_ACCESS_KEY`     | 0.17+           |
| `azure_client_secret`       | `AZURE_CLIENT_SECRET`       | 0.17+           |
| `hc_vault_token`            | `VAULT_TOKEN`               | 0.17+           |
| `huawei_sdk_ak`             | `HUAWEICLOUD_SDK_AK`        | 0.17+           |
| `huawei_sdk_sk`             | `HUAWEICLOUD_SDK_SK`        | 0.17+           |
| `google_oauth_access_token` | `GOOGLE_OAUTH_ACCESS_TOKEN` | 0.17+           |

In 0.21+, credentials retain their exact bytes during resolution. Interfaces that require text validate UTF-8 when consuming the credential; invalid input returns an error without selecting an environment fallback. Unix CLI environments accept non-UTF-8 bytes, but cannot contain NUL bytes. A configured credential that resolves to an empty value is an error rather than a fall through to the environment.

See the complete [provider credential reference](/reference/provider-credentials/) for all supported providers and environment fallbacks.

For example, this alias loads an age identity from the system keyring and passes it only to the SOPS child process. Replace `YOUR_AGE_RECIPIENT` with the recipient printed by `age-keygen -y` during setup:

**secretspec.toml**

```toml
[providers.sops_age]
uri = "sops://secrets.enc.yaml?age_recipients=YOUR_AGE_RECIPIENT"


[providers.sops_age.credentials]
age_key = "keyring"


[profiles.production.defaults]
providers = ["sops_age"]
```

When a credential is not declared on the alias, SOPS can still use its normal environment variable.

## Configuration

### URI format

```text
sops://[path/to/secret][?key=value[&key=value]...]
```

* `path/to/secret` — optional absolute or relative path to the encrypted file; defaults to `secrets.enc.yaml`. Relative paths are resolved from the directory containing `secretspec.toml`, not from the shell’s current directory.

  * > **Secrets in multiple files?**
    >
    > To represent more than one file, use a templated path. This supports separate SOPS-encrypted files per profile or a hierarchical directory structure.
    >
    > A templated path must include both `{project}` and `{profile}` placeholders. Examples:
    >
    > * `sops://secrets-dir/{project}.{profile}.enc.json`
    > * `sops://secrets-dir/{project}/{profile}.enc.yaml`

* `?key=value` — optional query parameter; see [Query parameters](#query-parameters)

* `&key=value` — additional parameters

Relative `age_key_file`, `age_ssh_private_key_file`, and `sops_config` query values are resolved from the same manifest directory.

### Project configuration

Use an alias to keep the storage path and encryption settings in `secretspec.toml`:

**secretspec.toml**

```toml
[providers]
encrypted_file = "sops://secrets/{project}/{profile}.enc.yaml"


[profiles.default.defaults]
providers = ["encrypted_file"]
```

Templated paths must contain both `{project}` and `{profile}`. Use a single-file URI when every project and profile should share one encrypted document.

### Query parameters

Except for the SecretSpec-specific `format` parameter, see the [SOPS documentation](https://getsops.io/docs/#usage) for the purpose and usage of each parameter.

#### SecretSpec

| Provider URL Query Parameter Name | Purpose                                                                                                     |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| format                            | Overrides the extension-based file format detection. Valid values: `dotenv` `env` `ini` `json` `yaml` `yml` |

#### SOPS

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| sops\_config                      | SOPS\_CONFIG                       |
| sops\_decryption\_order           | SOPS\_DECRYPTION\_ORDER            |
| sops\_editor                      | SOPS\_EDITOR                       |
| sops\_enable\_local\_keyservice   | SOPS\_ENABLE\_LOCAL\_KEYSERVICE    |
| sops\_keyservice                  | SOPS\_KEYSERVICE                   |

#### Age

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| age\_key\_cmd                     | SOPS\_AGE\_KEY\_CMD                |
| age\_key\_file                    | SOPS\_AGE\_KEY\_FILE               |
| age\_recipients                   | SOPS\_AGE\_RECIPIENTS              |
| age\_ssh\_private\_key\_cmd       | SOPS\_AGE\_SSH\_PRIVATE\_KEY\_CMD  |
| age\_ssh\_private\_key\_file      | SOPS\_AGE\_SSH\_PRIVATE\_KEY\_FILE |

#### AWS

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| aws\_access\_key\_id              | AWS\_ACCESS\_KEY\_ID               |
| aws\_profile                      | AWS\_PROFILE                       |
| aws\_region                       | AWS\_REGION                        |
| kms\_arn                          | SOPS\_KMS\_ARN                     |

#### GCP

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| gcp\_kms\_client\_type            | SOPS\_GCP\_KMS\_CLIENT\_TYPE       |
| gcp\_kms\_endpoint                | SOPS\_GCP\_KMS\_ENDPOINT           |
| gcp\_kms\_ids                     | SOPS\_GCP\_KMS\_IDS                |
| gcp\_kms\_universe\_domain        | SOPS\_GCP\_KMS\_UNIVERSE\_DOMAIN   |

#### Azure

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| azure\_client\_id                 | AZURE\_CLIENT\_ID                  |
| azure\_keyvault\_urls             | SOPS\_AZURE\_KEYVAULT\_URLS        |
| azure\_tenant\_id                 | AZURE\_TENANT\_ID                  |

#### PGP

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| pgp\_fp                           | SOPS\_PGP\_FP                      |

#### GPG

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| gpg\_exec                         | SOPS\_GPG\_EXEC                    |

#### HashiCorp Vault/OpenBao

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| hc\_vault\_addr                   | VAULT\_ADDR                        |
| hc\_vault\_allowlist              | SOPS\_HC\_VAULT\_ALLOWLIST         |

#### Huawei Cloud

| Provider URL Query Parameter Name | Corresponding Environment Variable |
| --------------------------------- | ---------------------------------- |
| huawei\_kms\_ids                  | SOPS\_HUAWEICLOUD\_KMS\_IDS        |
| huawei\_sdk\_project\_id          | HUAWEICLOUD\_SDK\_PROJECT\_ID      |

Each query option in the environment-mapping tables is exported to the SOPS child process and overrides the same inherited environment variable. SOPS configuration resolves in this order: `sops_config` in the URI, a `.sops.yaml` discovered from the manifest directory or one of its parents, then an inherited `SOPS_CONFIG`. Provider credentials similarly override their matching secret environment fallbacks, but only for the SOPS child process.

## Storage model

The URI shape determines the on-disk layout. This matters when editing a file with `sops` directly and when diagnosing a value set under the wrong profile.

For a **single YAML or JSON file**, convention-addressed secrets are written at `[project][profile][key]`. For example, `secretspec set API_KEY --profile production` in project `my-app` writes:

**secrets.enc.yaml (decrypted view)**

```yaml
my-app:
  production:
    API_KEY: secret-value
```

The equivalent selector passed to `sops set` is `["my-app"]["production"]["API_KEY"]`. Reads also retain compatibility with older `[profile][key]` and root `[key]` layouts, but new convention writes use the fully namespaced path.

For a **templated YAML or JSON URI**, `{project}` and `{profile}` already select the file, so the key is flat inside it:

```toml
[providers]
prod_sops = "sops://secrets/{project}/{profile}.enc.yaml"
```

**secrets/my-app/production.enc.yaml (decrypted view)**

```yaml
API_KEY: secret-value
```

Single-file dotenv is always a flat `API_KEY=value` document. Single-file INI uses the selected profile as its section; templated INI uses `[DEFAULT]` because the profile is already represented by the filename. The following table is the write layout:

| Format      | Single file               | Templated path   |
| ----------- | ------------------------- | ---------------- |
| YAML / JSON | `[project][profile][key]` | root `[key]`     |
| dotenv      | root `[key]`              | root `[key]`     |
| INI         | `[profile][key]`          | `[DEFAULT][key]` |

In SecretSpec 0.19+, `secretspec set` and interactive `secretspec check` print the resolved provider URI, profile, file, and selector before prompting for or writing the value. This makes an accidentally omitted `--profile` visible before the encrypted file changes.

### Format handling

SecretSpec asks SOPS to emit JSON when decrypting and selects the secret from that JSON representation. When `?format=` is present, SecretSpec passes the corresponding SOPS input type where SOPS supports one, so filenames such as `.env.production.enc?format=dotenv` work correctly.

SOPS does not support `--input-type ini`, so `?format=ini` is accepted only when the filename itself ends in `.ini`. Without `?format=`, the filename must end in `.yaml`, `.yml`, `.json`, `.env`, `.dotenv`, or `.ini`; an unrecognized extension is reported as a configuration error.

When a selected YAML or JSON node is not a string, reads return its compact JSON representation. Writes always store the supplied secret as a string value.

In a single dotenv file, the same key cannot hold different values for different profiles. Use a templated path such as `secrets/{project}/.env.{profile}.enc?format=dotenv` when profiles need separate dotenv values.

## Use existing secrets

SOPS supports `ref = { item = "..." }` against a **single-file** provider URI. For YAML, JSON, and dotenv, the item names a root key and does not add the project/profile convention path. For INI, it names a key in the `[DEFAULT]` section:

**secretspec.toml**

```toml
[providers]
shared_sops = "sops://shared.enc.yaml"


[profiles.production]
EXTERNAL_TOKEN = { description = "Token already managed in shared.enc.yaml", ref = { item = "existing_token" }, providers = ["shared_sops"] }
```

This YAML example reads or writes root selector `["existing_token"]`; the INI equivalent is `["DEFAULT"]["existing_token"]`. This provider treats the value at the selected key as the complete secret, so it supports only `item` and rejects `field` and other extra ref coordinates. A templated SOPS URI also rejects refs: without convention `project`/`profile` inputs, SecretSpec cannot choose which templated file the external item belongs to. See [Secret References](/concepts/references/) for the general model.

## Advanced configuration

### Pass age settings in the provider URI

The setup above uses `.sops.yaml` and `SOPS_AGE_KEY_FILE`. To keep both settings in the provider URI instead, derive the recipient from the same identity:

```bash
$ AGE_RECIPIENT="$(age-keygen -y key.txt)"


$ secretspec set DATABASE_URL --provider "sops://secrets.enc.json?age_key_file=key.txt&age_recipients=${AGE_RECIPIENT}"
```

## CI/CD

Make the selected SOPS key service available to the job through provider credentials or its standard environment variables, then run SecretSpec with the configured alias:

```bash
$ secretspec run --profile production --provider sops_age -- deploy
```

Provider credentials are exposed only to the SOPS child process. Store their backing values in the CI platform’s secret store rather than in the provider URI.

## Security considerations

* Commit only the SOPS-encrypted files, never decrypted copies or private key material.
* Keep secret authentication values in provider credentials or SOPS environment variables instead of URI query parameters.
* Review the resolved file and selector shown by `secretspec set` and interactive `secretspec check` in SecretSpec 0.19+ before confirming a write.
* Use templated paths when profiles must not share the same dotenv key.