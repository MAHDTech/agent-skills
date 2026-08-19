+++
title = "secretspec-comparison-b2a822b7"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# Comparison

SecretSpec is the application-facing layer of a secrets system. It
defines what an application needs, resolves those requirements across
environments, and delivers the resulting values through its CLI and
[provider-independent SDKs](https://secretspec.dev/sdk/overview/). Providers connect that
layer to concrete value sources. Depending on the backend, they can add
secure storage, identity, access control, availability, and
provider-native operations.

This separation keeps [`secretspec.toml`](https://secretspec.dev/concepts/declarative/)
portable. A developer can use the system keyring, CI can supply
environment variables, and production can use Vault or a cloud secret
manager without changing the application’s secret contract.

```
secretspec.toml          SecretSpec                         Providerwhat the app needs  →    resolve · check · deliver    ←    provider-backed values                         route · audit                      source · access
```

## Division of responsibility

| Responsibility | SecretSpec | Providers augment SecretSpec with |
|----|----|----|
| Application secret contract | Declares names, descriptions, [requirements and defaults](https://secretspec.dev/reference/configuration/#secret-variable-options), [generated values](https://secretspec.dev/concepts/generation/), and [composed values](https://secretspec.dev/concepts/composed-secrets/) | Supply provider-backed values named by that contract |
| Environments | Defines portable [profiles](https://secretspec.dev/concepts/profiles/), [configuration inheritance](https://secretspec.dev/concepts/inheritance/), and profile-specific requirements | Add provider-native projects, vaults, paths, or environments |
| Preflight validation | [`check`](https://secretspec.dev/reference/cli/#check) validates required secrets and configuration before the application starts | Report whether a requested value exists or can be accessed |
| Provider selection | Routes each secret independently through [provider aliases and ordered fallback chains](https://secretspec.dev/concepts/providers/fallback/) | Supply concrete sources and destinations |
| Existing provider-native secrets | Uses [secret references](https://secretspec.dev/concepts/references/) to give an existing value a stable, application-facing name | Interpret provider-specific coordinates such as vault, item, field, path, or version |
| Application delivery | Resolves secrets through the [CLI](https://secretspec.dev/reference/cli/), [exports environments](https://secretspec.dev/reference/cli/#export), [starts child processes](https://secretspec.dev/reference/cli/#run), and [manages temporary files](https://secretspec.dev/reference/configuration/#as_path-option) | Supply values through provider APIs or clients |
| Application SDKs | Offers one [provider-independent resolver](https://secretspec.dev/sdk/overview/#one-resolver-thin-clients) with a shared [runtime API](https://secretspec.dev/sdk/overview/#the-runtime-api) and [typed access](https://secretspec.dev/sdk/overview/#typed-access) across supported programming languages | Vendor SDKs, when available, remain backend-specific; applications do not need to integrate them directly |
| Audit | Records [local, metadata-only access events](https://secretspec.dev/concepts/audit/) by default, including application context and optional reason | Add centralized, provider-side access records where supported and configured |
| Encryption at rest | Delegates protection of provider-backed values to the selected provider | Protect values when the backend supports encryption; dotenv and environment providers add no at-rest encryption |
| Identity and access policy | Uses the credentials available for the selected provider, including [credentials sourced from another provider](https://secretspec.dev/reference/provider-credentials/) | Enforce users, roles, service identities, policies, and sharing |
| Availability and retention | Delegates these guarantees for provider-backed values | May provide synchronization, replication, versions, backup, or retention, depending on the provider |
| Dynamic secrets and credential rotation | [Roadmap](https://github.com/cachix/secretspec/issues/11); not currently available and has no assigned target release | Provide native lifecycle features where available; use them outside SecretSpec today |

The distinction is intentional: SecretSpec provides portable application
semantics, while each provider determines how its provider-backed values
are stored, protected, and operated. Some providers, such as dotenv and
environment variables, intentionally provide fewer safeguards.
SecretSpec’s [default audit log](https://secretspec.dev/concepts/audit/) complements provider
logs by recording the project, profile, secret name, outcome, actor, and
reason seen by the application workflow. It is a size-bounded,
best-effort local log, not a replacement for central compliance records.

## Supported providers

See [Available providers](https://secretspec.dev/concepts/providers/#available-providers) for
the provider comparison, including storage backend, read and write
support, encryption at rest, and TPM-backed keys.

Providers can be mixed within one project. For example, an application
can read a shared credential from 1Password in the production profile,
read the same secret from the system keyring in the development profile,
and accept a deployment token from the environment in CI. A secret can
also define an ordered fallback chain, which tries the next provider
when an earlier provider does not return the value. SecretSpec keeps
those storage decisions outside the application’s code.

{% endraw %}
