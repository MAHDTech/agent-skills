+++
title = "secretspec-providers-scaleway-6ae6ba0e"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Scaleway Secret Manager Provider

The Scaleway Secret Manager provider stores secrets in [Scaleway Secret
Manager](https://www.scaleway.com/en/secret-manager/) through its
`v1beta1` REST API.

## At a glance

|  |  |
|----|----|
| Provider | `scaleway` |
| URI | `scaleway://[REGION][?project_id=UUID&path=/folder]` |
| Access | Read and write; secret references are read-only |
| Best for | Workloads and teams on Scaleway |
| Authentication | API secret key (`X-Auth-Token`) |
| Build feature | `scaleway` |
| Default storage | folder `[{base}/]secretspec/{project}/{profile}`, name `{key}` |

## Quick start

```
$ export SCW_SECRET_KEY=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
$ export SCW_DEFAULT_PROJECT_ID=11111111-2222-3333-4444-555555555555
# Set a secret$ secretspec set DATABASE_URL --provider scaleway://fr-parEnter value for DATABASE_URL: postgresql://localhost/mydb✓ Secret 'DATABASE_URL' saved to scaleway (profile: default)
# Run with secrets$ secretspec run --provider scaleway://fr-par -- npm start
```

Terminal window

## Setup

### Prerequisites

- A Scaleway account with Secret Manager enabled
- An API key (access key + secret key) with Secret Manager permissions
- Build with `--features scaleway`

### Authentication

The provider authenticates with a Scaleway API **secret key**, sent in
the `X-Auth-Token` header. It is read from, in order:

1.  The `secret_key` provider credential
2.  The `SCW_SECRET_KEY` environment variable

The target project is read from `?project_id=` in the URI, falling back
to `SCW_DEFAULT_PROJECT_ID`. The region is the URI host, falling back to
`SCW_DEFAULT_REGION`, and finally `fr-par`. Secret Manager is available
in the `fr-par`, `nl-ams`, and `pl-waw` regions.

## Provider credentials

| Credential   | Environment fallback | Available since |
|--------------|----------------------|-----------------|
| `secret_key` | `SCW_SECRET_KEY`     | 0.17+           |

See the complete [provider credential
reference](https://secretspec.dev/reference/provider-credentials/) for all supported providers
and environment fallbacks.

## Configuration

### URI format

```
scaleway://[REGION][?project_id=UUID][&path=/folder]
```

- `REGION`: Scaleway region (e.g. `fr-par`). If omitted,
  `SCW_DEFAULT_REGION` is used, then `fr-par`.
- `project_id`: Target project UUID. If omitted,
  `SCW_DEFAULT_PROJECT_ID` is used.
- `path`: Optional base folder prepended to the convention hierarchy.
  Defaults to `/` (root).

### URI examples

```
scaleway://fr-parscaleway://nl-ams?project_id=11111111-2222-3333-4444-555555555555scaleway://fr-par?project_id=11111111-2222-3333-4444-555555555555&path=/myteamscaleway://
```

### Project configuration

The region and project usually vary per environment, so they are a
natural fit for a checked-in [provider alias](https://secretspec.dev/reference/configuration/)
in `secretspec.toml` (the secret key stays in the environment, never the
URI):

```
[providers]scw = "scaleway://fr-par?project_id=11111111-2222-3333-4444-555555555555"
```

```
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["scw"] }
```

secretspec.toml

## Storage model

Scaleway secret names may not contain `/` — that character separates
folders — so the SecretSpec convention lives in the folder hierarchy
rather than the name. A secret is stored in the folder
`[{base}/]secretspec/{project}/{profile}` with the key as its name.

For example, `DATABASE_URL` in project `myapp` and profile `production`
is stored at folder `/secretspec/myapp/production` with name
`DATABASE_URL`. With `?path=/myteam`, the folder becomes
`/myteam/secretspec/myapp/production`.

Each write appends a new secret version; reads return the latest enabled
version.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names an existing Scaleway secret instead. `item` is the secret’s
absolute path (folder + name, e.g. `/prod/db-url`); the optional `field`
selects one key of a JSON (`key_value`) secret, and the optional
`version` pins a revision number (default: latest enabled). References
are **read-only** in this provider.

```
[profiles.production]# Whole secret value, latest enabled revisionDATABASE_URL = { description = "DB", ref = { item = "/prod/database-url" }, providers = ["scaleway://fr-par"] }# One key of a JSON secret, pinned to revision 3DB_PASSWORD = { description = "DB pw", ref = { item = "/prod/db-credentials", field = "password", version = "3" }, providers = ["scaleway://fr-par"] }
```

## CI/CD

```
$ export SCW_SECRET_KEY=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
$ export SCW_DEFAULT_PROJECT_ID=11111111-2222-3333-4444-555555555555
$ export SCW_DEFAULT_REGION=fr-par
$ secretspec run --provider scaleway://fr-par -- deploy
```

Terminal window

