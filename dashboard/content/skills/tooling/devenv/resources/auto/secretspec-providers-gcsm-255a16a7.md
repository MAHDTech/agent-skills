+++
title = "secretspec-providers-gcsm-255a16a7"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# Google Cloud Secret Manager Provider

The Google Cloud Secret Manager provider integrates with GCP for
centralized secret management.

## At a glance

|                 |                                                 |
|-----------------|-------------------------------------------------|
| Provider        | `gcsm`                                          |
| URI             | `gcsm://PROJECT_ID`                             |
| Access          | Read and write; secret references are read-only |
| Best for        | Workloads and teams on Google Cloud             |
| Authentication  | Google Application Default Credentials          |
| Build feature   | `gcsm`                                          |
| Default storage | `secretspec-{project}-{profile}-{key}`          |

## Quick start

```
# Set a secret$ secretspec set DATABASE_URL --provider gcsm://my-gcp-projectEnter value for DATABASE_URL: postgresql://localhost/mydb✓ Secret 'DATABASE_URL' saved to gcsm (profile: default)
# Run with secrets$ secretspec run --provider gcsm://my-gcp-project -- npm start
```

Terminal window

## Setup

### Prerequisites

- Google Cloud CLI (`gcloud`)
- GCP project with Secret Manager API enabled
- Build with `--features gcsm`

### Authentication

Google Cloud Secret Manager uses Application Default Credentials. For
local development:

```
$ gcloud auth application-default login
```

Terminal window

In Google Cloud runtimes, Application Default Credentials use the
attached service account automatically.

## Configuration

### URI format

```
gcsm://PROJECT_ID
```

- `PROJECT_ID`: Your GCP project ID

### URI examples

```
gcsm://my-gcp-project
```

### Project configuration

```
[providers]google = "gcsm://my-gcp-project"
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["google"] }
```

secretspec.toml

## Storage model

Secrets are stored as `secretspec-{project}-{profile}-{key}`. For
example, project `myapp`, profile `production`, and key `DATABASE_URL`
map to `secretspec-myapp-production-DATABASE_URL`.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names an existing secret instead: `item` is the secret id, and the
optional `version` pins a version (defaults to latest; `field` is not
supported). References are **read-only** in this provider.

```
[profiles.production]DATABASE_URL = { description = "DB", ref = { item = "database-url" }, providers = ["gcsm://my-gcp-project"] }SIGNING_KEY = { description = "Key", ref = { item = "signing-key", version = "3" }, providers = ["gcsm://my-gcp-project"] }
```

## CI/CD

```
# Set credentials$ export GOOGLE_APPLICATION_CREDENTIALS="/path/to/key.json"
# Run command$ secretspec run --provider gcsm://my-gcp-project -- deploy
```

Terminal window

{% endraw %}
