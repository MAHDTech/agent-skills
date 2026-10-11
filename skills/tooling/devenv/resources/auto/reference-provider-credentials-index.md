# Provider credentials

> Semantic provider credential names and their environment fallbacks

Provider credentials let one provider load the authentication material it needs from another SecretSpec provider. They are supported in SecretSpec 0.15 and later.

The table below is the exhaustive reference for accepted semantic credential names for providers compiled into SecretSpec. An explicitly configured provider credential takes precedence over its environment fallback. When more than one fallback is listed, SecretSpec checks them from left to right. A configured credential that resolves to an empty value is an error; SecretSpec never substitutes the environment fallback for it (0.21+).

External providers negotiate their URI-specific semantic names through the provider protocol in SecretSpec 0.21+. They do not appear in this compile-time catalog. Their alias may omit `credentials`; SecretSpec then uses its provider-private operating-system keyring namespace. A configured mapping is a lazy source override for the name the endpoint requests.

| Provider                                 | Credential                  | Environment fallback                 | Available since |
| ---------------------------------------- | --------------------------- | ------------------------------------ | --------------- |
| [`aac`](/providers/aac/)                 | `tenant_id`                 | `AZURE_TENANT_ID`                    | 0.20+           |
| [`aac`](/providers/aac/)                 | `client_id`                 | `AZURE_CLIENT_ID`                    | 0.20+           |
| [`aac`](/providers/aac/)                 | `client_secret`             | `AZURE_CLIENT_SECRET`                | 0.20+           |
| [`aac`](/providers/aac/)                 | `connection_string`         | `AZURE_APPCONFIG_CONNECTION_STRING`  | 0.20+           |
| [`age`](/providers/age/)                 | `identity`                  | `AGE_IDENTITY`                       | 0.17+           |
| [`akv`](/providers/akv/)                 | `tenant_id`                 | `AZURE_TENANT_ID`                    | 0.15+           |
| [`akv`](/providers/akv/)                 | `client_id`                 | `AZURE_CLIENT_ID`                    | 0.15+           |
| [`akv`](/providers/akv/)                 | `client_secret`             | `AZURE_CLIENT_SECRET`                | 0.15+           |
| [`bws`](/providers/bws/)                 | `access_token`              | `BWS_ACCESS_TOKEN`                   | 0.15+           |
| [`cloudflare`](/providers/cloudflare/)   | `api_token`                 | `CLOUDFLARE_API_TOKEN`               | 0.20+           |
| [`dashlane`](/providers/dashlane/)       | `service_device_keys`       | `DASHLANE_SERVICE_DEVICE_KEYS`       | 0.18+           |
| [`doppler`](/providers/doppler/)         | `token`                     | `DOPPLER_TOKEN`                      | 0.21+           |
| [`ejson`](/providers/ejson/)             | `private_key`               | —                                    | 0.20+           |
| [`fly`](/providers/fly/)                 | `access_token`              | `FLY_API_TOKEN` → `FLY_ACCESS_TOKEN` | 0.20+           |
| [`infisical`](/providers/infisical/)     | `client_id`                 | `INFISICAL_CLIENT_ID`                | 0.16+           |
| [`infisical`](/providers/infisical/)     | `client_secret`             | `INFISICAL_CLIENT_SECRET`            | 0.16+           |
| [`infisical`](/providers/infisical/)     | `token`                     | `INFISICAL_TOKEN`                    | 0.16+           |
| [`kdbx`](/providers/kdbx/)               | `password`                  | `SECRETSPEC_KDBX_PASSWORD`           | 0.17+           |
| [`keeper`](/providers/keeper/)           | `config`                    | `KSM_CONFIG`                         | 0.18+           |
| [`keeper`](/providers/keeper/)           | `token`                     | `KSM_TOKEN`                          | 0.18+           |
| [`onepassword`](/providers/onepassword/) | `service_account_token`     | `OP_SERVICE_ACCOUNT_TOKEN`           | 0.15+           |
| [`openbao`](/providers/openbao/)         | `role_id`                   | `BAO_ROLE_ID` → `VAULT_ROLE_ID`      | 0.17+           |
| [`openbao`](/providers/openbao/)         | `secret_id`                 | `BAO_SECRET_ID` → `VAULT_SECRET_ID`  | 0.17+           |
| [`openbao`](/providers/openbao/)         | `token`                     | `BAO_TOKEN` → `VAULT_TOKEN`          | 0.17+           |
| [`passbolt`](/providers/passbolt/)       | `private_key`               | `SECRETSPEC_PASSBOLT_PRIVATE_KEY`    | 0.19+           |
| [`passbolt`](/providers/passbolt/)       | `passphrase`                | `SECRETSPEC_PASSBOLT_PASSPHRASE`     | 0.19+           |
| [`scaleway`](/providers/scaleway/)       | `secret_key`                | `SCW_SECRET_KEY`                     | 0.17+           |
| [`sops`](/providers/sops/)               | `age_key`                   | `SOPS_AGE_KEY`                       | 0.17+           |
| [`sops`](/providers/sops/)               | `aws_secret_access_key`     | `AWS_SECRET_ACCESS_KEY`              | 0.17+           |
| [`sops`](/providers/sops/)               | `azure_client_secret`       | `AZURE_CLIENT_SECRET`                | 0.17+           |
| [`sops`](/providers/sops/)               | `hc_vault_token`            | `VAULT_TOKEN`                        | 0.17+           |
| [`sops`](/providers/sops/)               | `huawei_sdk_ak`             | `HUAWEICLOUD_SDK_AK`                 | 0.17+           |
| [`sops`](/providers/sops/)               | `huawei_sdk_sk`             | `HUAWEICLOUD_SDK_SK`                 | 0.17+           |
| [`sops`](/providers/sops/)               | `google_oauth_access_token` | `GOOGLE_OAUTH_ACCESS_TOKEN`          | 0.17+           |
| [`vault`](/providers/vault/)             | `role_id`                   | `VAULT_ROLE_ID`                      | 0.15+           |
| [`vault`](/providers/vault/)             | `secret_id`                 | `VAULT_SECRET_ID`                    | 0.15+           |
| [`vault`](/providers/vault/)             | `token`                     | `VAULT_TOKEN`                        | 0.15+           |

In 0.21+, credentials retain their exact bytes during resolution. Interfaces that require text validate UTF-8 when consuming the credential; invalid input returns an error without selecting an environment fallback. Unix CLI environments accept non-UTF-8 bytes, but cannot contain NUL bytes. A configured credential that resolves to an empty value is an error rather than a fall through to the environment.

See [Provider credentials](/concepts/providers/#provider-credentials) for credential source addresses, storage commands, one-hop chaining, and runtime handling rules.