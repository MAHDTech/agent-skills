+++
title = "secretspec-providers-sops-9437dd88"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# SOPS: Secrets OPerationS

The SOPS provider integrates with [SOPS](https://getsops.io) to enable
reading and writing of secrets encrypted at rest in local files.

## Prerequisites

- The SOPS CLI available within the environment:
  - [Manually download a release
    binary](https://github.com/getsops/sops/releases)

  - [Use the SOPS Nix
    package](https://search.nixos.org/packages?channel=unstable&query=sops#show=sops)

  - Install with a package manager:

    ```
      # Homebrew  $ brew install sops
      # Arch  $ sudo pacman -S sops
    ```

    Terminal window

## Configuration

### URI Format

```
sops://path/to/secret[?key=value[&key=value]...]
```

- `path/to/secret` — required path to the encrypted file (absolute or
  relative)

  - 

- `?key=value` — optional query parameter, refer to the [Query
  Parameters section](#query-parameters) for available parameters.

- `&key=value` — additional parameters

### Query Parameters

Excepting the SecretSpec-specific `format` parameter, refer to [the SOPS
documentation](https://getsops.io/docs/#usage) for the purpose and usage
of each parameter.

#### SecretSpec

| Provider URL Query Parameter Name | Purpose |
|----|----|
| format | Overrides the extension-based file format detection. Valid values: `dotenv` `env` `ini` `json` `yaml` |

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

#### Hashicorp Vault/OpenBao

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| hc_vault_addr                     | VAULT_ADDR                         |
| hc_vault_allowlist                | SOPS_HC_VAULT_ALLOWLIST            |

#### Huawei Cloud

| Provider URL Query Parameter Name | Corresponding Environment Variable |
|-----------------------------------|------------------------------------|
| huawei_kms_ids                    | SOPS_HUAWEICLOUD_KMS_IDS           |
| huawei_sdk_project_id             | HUAWEICLOUD_SDK_PROJECT_ID         |

### Provider credentials

Secret values used to authenticate SOPS belong in a provider alias’s
`credentials` map, not in the SOPS URI. The SOPS provider accepts these
semantic credential names:

| Credential                  | SOPS environment fallback   |
|-----------------------------|-----------------------------|
| `age_key`                   | `SOPS_AGE_KEY`              |
| `aws_secret_access_key`     | `AWS_SECRET_ACCESS_KEY`     |
| `azure_client_secret`       | `AZURE_CLIENT_SECRET`       |
| `google_oauth_access_token` | `GOOGLE_OAUTH_ACCESS_TOKEN` |
| `hc_vault_token`            | `VAULT_TOKEN`               |
| `huawei_sdk_ak`             | `HUAWEICLOUD_SDK_AK`        |
| `huawei_sdk_sk`             | `HUAWEICLOUD_SDK_SK`        |

For example, this alias loads an age identity from the system keyring
and passes it only to the SOPS child process:

```
[providers.sops_age]uri = "sops://secrets.enc.yaml?age_recipients=age1example"
[providers.sops_age.credentials]age_key = "keyring"
[profiles.production.defaults]providers = ["sops_age"]
```

secretspec.toml

When a credential is not declared on the alias, SOPS can still use its
normal environment variable. See [Provider
credentials](https://secretspec.dev/concepts/providers/#provider-credentials) for convention
and explicit credential-source addresses.

### Format handling

SecretSpec asks SOPS to emit JSON when decrypting, then uses one lookup
path for YAML, JSON, dotenv, and INI files. When `?format=` is present,
SecretSpec also passes the corresponding SOPS input type so filenames
such as `.env.production.enc?format=dotenv` work correctly.

SOPS does not support `--input-type ini`, so `?format=ini` is accepted
only when the filename itself ends in `.ini`. Without `?format=`, the
filename must end in `.yaml`, `.yml`, `.json`, `.env`, `.dotenv`, or
`.ini`; an unrecognized extension is reported as a configuration error.

YAML and JSON single-file layouts can nest values by project and
profile. INI uses profile sections, while dotenv is always flat; in a
single dotenv file, the same key therefore cannot hold different values
for different profiles. Use a templated path such as
`secrets/{project}/.env.{profile}.enc?format=dotenv` when profiles need
separate dotenv values.

## Usage

### Set a secret with age

```
$ secretspec set DATABASE_URL --provider 'sops://secrets.enc.json?age_key_file=key.txt&age_recipients=age1jpa8rf5qmrg6pw444fcgpkaxg8x4neueszrexzagdjpunjlgeyzq304w34'
```

Terminal window

Or, if `keys.txt` exists in a place discoverable by SOPS and
`.sops.yaml` exists next to the project’s `secretspec.toml`, configure a
creation rule:

```
creation_rules:  - path_regex: secrets.enc.json$    age: "age1jpa8rf5qmrg6pw444fcgpkaxg8x4neueszrexzagdjpunjlgeyzq304w34"
```

.sops.yaml

then:

```
$ secretspec set DATABASE_URL --provider sops://secrets.enc.json
```

Terminal window

{% endraw %}
