# Cloudflare Secrets Store provider

The [Cloudflare](https://www.cloudflare.com/) provider publishes
declared values to an account-level [Cloudflare Secrets
Store](https://developers.cloudflare.com/secrets-store/) through the
Cloudflare REST API.

## At a glance

|  |  |
|----|----|
| Provider | `cloudflare` (0.20+) |
| URI | `cloudflare://STORE_ID[?account_id=ACCOUNT_ID][&OPTIONS]` |
| Access | Write, delete, and discover names; plaintext values cannot be read back |
| Best for | Publishing secrets to Workers and other Cloudflare services from a separate source of truth |
| Authentication | API token or credentials from `wrangler auth token --json` |
| Availability | SecretSpec 0.20+; included in official and default builds (`cloudflare` feature for custom minimal builds) |
| Default storage | Account secret named `{key}` in the selected store |

## Quick start

Find the account ID and Secrets Store ID in the Cloudflare dashboard or
with Wrangler, then authenticate and configure an alias:

```
$ wrangler login$ wrangler secrets-store store list --remote
```

Terminal window

```
[providers]cloudflare_prod = "cloudflare://0123456789abcdef0123456789abcdef?account_id=abcdef0123456789abcdef0123456789&auth=wrangler"
[profiles.production]DATABASE_URL = { description = "Production database URL" }
```

secretspec.toml

```
# Publish or replace the account secret$ secretspec set DATABASE_URL --profile production --provider cloudflare_prod
# Remove it$ secretspec delete DATABASE_URL --profile production --provider cloudflare_prod
```

Terminal window

Cloudflare never returns plaintext through its management API, so
`secretspec get`, `check`, and `run` cannot resolve a value from this
provider. Keep the authoritative value in a readable provider and select
`cloudflare_prod` explicitly when publishing it.

## Setup

### Prerequisites

- SecretSpec 0.20 or newer
- A Cloudflare account with a Secrets Store
- Account **Secrets Store Write** permission for publishing and deletion
- The account ID and Secrets Store ID

The official SecretSpec CLI includes this provider. Custom minimal Rust
builds enable it with `--features cloudflare`.

### Wrangler authentication

With `auth=wrangler`, SecretSpec runs:

```
$ wrangler auth token --json
```

Terminal window

Wrangler can return an API token, a refreshed OAuth token from
`wrangler login`, or legacy API-key/email credentials. SecretSpec uses
the returned credential only in HTTPS request headers. It never passes
the account secret value to Wrangler.

Wrangler supports named authentication profiles:

```
cloudflare://STORE_ID?account_id=ACCOUNT_ID&auth=wrangler&wrangler_profile=production
```

If the executable has another name or location, set
`SECRETSPEC_WRANGLER_PATH`. SecretSpec never invokes `npx`
automatically.

### Authentication with provider credentials

For CI or a machine identity, declare the `api_token` [provider
credential](https://secretspec.dev/reference/provider-credentials/):

```
[providers]bootstrap = "keyring://"
[providers.cloudflare_prod]uri = "cloudflare://0123456789abcdef0123456789abcdef?account_id=abcdef0123456789abcdef0123456789&auth=token"credentials = { api_token = "bootstrap" }
```

secretspec.toml

Store it once:

```
$ secretspec config provider login cloudflare_prodEnter api_token for provider 'cloudflare_prod' (source: bootstrap): ****
```

Terminal window

Use a scoped user or account API token with **Secrets Store Write** on
only the required account. Do not use the full-access Global API Key for
new setups.

### Environment fallback

`CLOUDFLARE_API_TOKEN` supplies the `api_token` credential when no
explicit provider credential exists. `CLOUDFLARE_ACCOUNT_ID` supplies
the account ID when the URI omits `account_id`.

The default `auth=auto` uses the provider credential or
`CLOUDFLARE_API_TOKEN` first, then falls back to Wrangler. Use
`auth=token` to require a token or `auth=wrangler` to require Wrangler
credentials.

## Provider credentials

| Credential  | Environment fallback   | Available since |
|-------------|------------------------|-----------------|
| `api_token` | `CLOUDFLARE_API_TOKEN` | 0.20+           |

See the complete [provider credential
reference](https://secretspec.dev/reference/provider-credentials/) for all supported providers
and environment fallbacks.

## Configuration

### URI format

```
cloudflare://STORE_ID[?account_id=ACCOUNT_ID][&scopes=LIST][&auth=MODE][&wrangler_profile=NAME]
```

- `STORE_ID` is required and selects the account-level Secrets Store.
- `account_id` selects the Cloudflare account and falls back to
  `CLOUDFLARE_ACCOUNT_ID`.
- `scopes` is a comma-separated list applied when a secret is created or
  replaced. It defaults to `workers`. Supported values are `workers`,
  `ai_gateway`, `dex`, `access`, `containers`, and `websearch`.
- `auth` is `auto` (default), `token`, or `wrangler`.
- `wrangler_profile` selects a named Wrangler auth profile and requires
  `auth=wrangler`.

### URI examples

```
cloudflare://STORE_ID?account_id=ACCOUNT_IDcloudflare://STORE_ID?account_id=ACCOUNT_ID&auth=tokencloudflare://STORE_ID?account_id=ACCOUNT_ID&auth=wranglercloudflare://STORE_ID?account_id=ACCOUNT_ID&scopes=workers,containerscloudflare://STORE_ID?account_id=ACCOUNT_ID&auth=wrangler&wrangler_profile=production
```

## Storage model

The provider maps a declaration key directly to an account-secret name.
For example, `DATABASE_URL` maps to:

```
account: ACCOUNT_IDstore:   STORE_IDsecret:  DATABASE_URL
```

Project and profile names are not added to the secret name. The store
selected by the provider alias supplies isolation. Use a different alias
and store when two profiles must hold different values for the same key.

Cloudflare Workers can bind that account secret to any binding name; the
SecretSpec key does not need to match the Worker’s binding variable.

## Use existing secrets

A [`ref`](https://secretspec.dev/reference/configuration/#secret-references) changes the
Cloudflare secret name updated or deleted by SecretSpec:

```
[profiles.production]DATABASE_URL = {  description = "Production database URL",  ref = { item = "PRIMARY_DATABASE_URL" }}
```

secretspec.toml

The reference remains write-only: it can select an existing name but
cannot retrieve its plaintext value.

## Discover secret names

Cloudflare’s list API exposes names, IDs, scopes, comments, and status
without returning values. SecretSpec uses that metadata for declaration
discovery:

```
$ secretspec init \    --from 'cloudflare://STORE_ID?account_id=ACCOUNT_ID&auth=wrangler' \    --project my-app --profile production
```

Terminal window

The generated manifest contains required declarations for active or
pending secret names, not defaults or values.

## CI/CD

Use a short-lived or account-owned token scoped to Secrets Store Write:

```
- run: secretspec set DATABASE_URL --profile production --provider cloudflare_prod  env:    CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
```

The account ID and store ID are attribution, not credentials, and can
stay in the checked-in provider URI.

## Security considerations and limitations

- Cloudflare’s management API accepts values for creation and
  replacement but never returns them. Plaintext access exists only
  inside a Cloudflare service with a Secrets Store binding. This
  provider therefore cannot support `get`, `check`, `run`, fallback
  reads, generation-on-miss, prompting-on-miss, or value comparisons.
- Secret values are serialized directly into an HTTPS request body. They
  do not appear in the provider URI, command arguments, Wrangler input,
  or SecretSpec diagnostics.
- HTTP redirects are rejected so credentials and secret-bearing request
  bodies remain confined to Cloudflare’s API origin.
- `secretspec set` lists metadata to resolve an existing name to the
  secret ID, then creates or patches it. `secretspec delete` uses the
  same metadata lookup and remains idempotent when the name is absent.
- A replacement applies the `scopes` configured in the provider URI.
  Review those scopes because changing them affects which Cloudflare
  services may bind the secret.
- Secret values cannot exceed Cloudflare’s 65,536-byte limit.
- Cloudflare Secrets Store is currently a beta service; API behavior and
  scope availability may change upstream.
