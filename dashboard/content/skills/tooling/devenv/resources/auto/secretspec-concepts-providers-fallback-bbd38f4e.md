+++
title = "secretspec-concepts-providers-fallback-bbd38f4e"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# Provider fallback

Secrets may live in different stores across environments or during a
migration. An ordered provider route lets SecretSpec read from the first
store that has a value while keeping one store as the write target.

## Provider selection order

SecretSpec selects each secret’s route in this order:

1.  The `--provider` command-line option.
2.  The `SECRETSPEC_PROVIDER` environment variable.
3.  The secret’s effective `providers` list after profile inheritance
    and `[profiles.\<name\>.defaults]` are applied.
4.  The default provider in the user configuration.

`--provider` and `SECRETSPEC_PROVIDER` replace the configured route for
every secret. Without an override or effective `providers` list,
SecretSpec uses the user-level default.

## Ordered fallback routes

Provider lists accept aliases, provider names, and inline URIs. Reads
try them from left to right:

```
[providers]prod_vault = "onepassword://Production"local = "keyring://"
[profiles.production.defaults]providers = ["prod_vault", "local"]
[profiles.production]# Uses the profile default: prod_vault, then local.DATABASE_URL = { description = "Production database" }
# Overrides the profile default and reads only from the environment.DEPLOY_TOKEN = { description = "Deployment token", providers = ["env"] }
```

secretspec.toml

Reads stop at the first value. Writes and generated values go only to
the first provider in the effective list (`prod_vault` above).

Later entries are resolved, constructed, and contacted only when needed.
If a reached provider cannot be resolved, constructed, or read,
SecretSpec warns and continues. If every reached provider fails, the
operation returns a provider error rather than reporting the secret as
absent.

## Cached routes

SecretSpec 0.17+ can place a local cache before an ordered route. See
[Provider caching](https://secretspec.dev/concepts/providers/caching/) for configuration and
freshness rules.

## Next steps

- Learn how to [configure provider
  aliases](https://secretspec.dev/concepts/providers/#configure-provider-aliases).
- Learn how [Profiles](https://secretspec.dev/concepts/profiles/) apply provider defaults.

{% endraw %}
