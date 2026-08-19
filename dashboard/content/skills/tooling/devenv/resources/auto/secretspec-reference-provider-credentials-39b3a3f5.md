+++
title = "secretspec-reference-provider-credentials-39b3a3f5"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# Provider credentials

Provider credentials let one provider load the authentication material
it needs from another SecretSpec provider. They are supported in
SecretSpec 0.15 and later.

The table below is the exhaustive reference for accepted semantic
credential names. An explicitly configured provider credential takes
precedence over its environment fallback. When more than one fallback is
listed, SecretSpec checks them from left to right.

| Provider | Credential | Environment fallback | Available since |
|----|----|----|----|
| [`aac`](https://secretspec.dev/providers/aac/) | `tenant_id` | `AZURE_TENANT_ID` | 0.20+ |
| [`aac`](https://secretspec.dev/providers/aac/) | `client_id` | `AZURE_CLIENT_ID` | 0.20+ |
| [`aac`](https://secretspec.dev/providers/aac/) | `client_secret` | `AZURE_CLIENT_SECRET` | 0.20+ |
| [`aac`](https://secretspec.dev/providers/aac/) | `connection_string` | `AZURE_APPCONFIG_CONNECTION_STRING` | 0.20+ |
| [`age`](https://secretspec.dev/providers/age/) | `identity` | `AGE_IDENTITY` | 0.17+ |
| [`akv`](https://secretspec.dev/providers/akv/) | `tenant_id` | `AZURE_TENANT_ID` | 0.15+ |
| [`akv`](https://secretspec.dev/providers/akv/) | `client_id` | `AZURE_CLIENT_ID` | 0.15+ |
| [`akv`](https://secretspec.dev/providers/akv/) | `client_secret` | `AZURE_CLIENT_SECRET` | 0.15+ |
| [`bws`](https://secretspec.dev/providers/bws/) | `access_token` | `BWS_ACCESS_TOKEN` | 0.15+ |
| [`dashlane`](https://secretspec.dev/providers/dashlane/) | `service_device_keys` | `DASHLANE_SERVICE_DEVICE_KEYS` | 0.18+ |
| [`fly`](https://secretspec.dev/providers/fly/) | `access_token` | `FLY_API_TOKEN` → `FLY_ACCESS_TOKEN` | 0.20+ |
| [`infisical`](https://secretspec.dev/providers/infisical/) | `client_id` | `INFISICAL_CLIENT_ID` | 0.16+ |
| [`infisical`](https://secretspec.dev/providers/infisical/) | `client_secret` | `INFISICAL_CLIENT_SECRET` | 0.16+ |
| [`infisical`](https://secretspec.dev/providers/infisical/) | `token` | `INFISICAL_TOKEN` | 0.16+ |
| [`kdbx`](https://secretspec.dev/providers/kdbx/) | `password` | `SECRETSPEC_KDBX_PASSWORD` | 0.17+ |
| [`keeper`](https://secretspec.dev/providers/keeper/) | `config` | `KSM_CONFIG` | 0.18+ |
| [`keeper`](https://secretspec.dev/providers/keeper/) | `token` | `KSM_TOKEN` | 0.18+ |
| [`onepassword`](https://secretspec.dev/providers/onepassword/) | `service_account_token` | `OP_SERVICE_ACCOUNT_TOKEN` | 0.15+ |
| [`openbao`](https://secretspec.dev/providers/openbao/) | `role_id` | `BAO_ROLE_ID` → `VAULT_ROLE_ID` | 0.17+ |
| [`openbao`](https://secretspec.dev/providers/openbao/) | `secret_id` | `BAO_SECRET_ID` → `VAULT_SECRET_ID` | 0.17+ |
| [`openbao`](https://secretspec.dev/providers/openbao/) | `token` | `BAO_TOKEN` → `VAULT_TOKEN` | 0.17+ |
| [`passbolt`](https://secretspec.dev/providers/passbolt/) | `private_key` | `SECRETSPEC_PASSBOLT_PRIVATE_KEY` | 0.19+ |
| [`passbolt`](https://secretspec.dev/providers/passbolt/) | `passphrase` | `SECRETSPEC_PASSBOLT_PASSPHRASE` | 0.19+ |
| [`scaleway`](https://secretspec.dev/providers/scaleway/) | `secret_key` | `SCW_SECRET_KEY` | 0.17+ |
| [`sops`](https://secretspec.dev/providers/sops/) | `age_key` | `SOPS_AGE_KEY` | 0.17+ |
| [`sops`](https://secretspec.dev/providers/sops/) | `aws_secret_access_key` | `AWS_SECRET_ACCESS_KEY` | 0.17+ |
| [`sops`](https://secretspec.dev/providers/sops/) | `azure_client_secret` | `AZURE_CLIENT_SECRET` | 0.17+ |
| [`sops`](https://secretspec.dev/providers/sops/) | `hc_vault_token` | `VAULT_TOKEN` | 0.17+ |
| [`sops`](https://secretspec.dev/providers/sops/) | `huawei_sdk_ak` | `HUAWEICLOUD_SDK_AK` | 0.17+ |
| [`sops`](https://secretspec.dev/providers/sops/) | `huawei_sdk_sk` | `HUAWEICLOUD_SDK_SK` | 0.17+ |
| [`sops`](https://secretspec.dev/providers/sops/) | `google_oauth_access_token` | `GOOGLE_OAUTH_ACCESS_TOKEN` | 0.17+ |
| [`vault`](https://secretspec.dev/providers/vault/) | `role_id` | `VAULT_ROLE_ID` | 0.15+ |
| [`vault`](https://secretspec.dev/providers/vault/) | `secret_id` | `VAULT_SECRET_ID` | 0.15+ |
| [`vault`](https://secretspec.dev/providers/vault/) | `token` | `VAULT_TOKEN` | 0.15+ |

See [Provider credentials](https://secretspec.dev/concepts/providers/#provider-credentials)
for credential source addresses, storage commands, one-hop chaining, and
runtime handling rules.

{% endraw %}
