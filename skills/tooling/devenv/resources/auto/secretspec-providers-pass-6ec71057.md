# Pass Provider

The [Pass](https://www.passwordstore.org/) provider stores secrets using
the Unix password manager `pass` (password-store). Secrets are
GPG-encrypted for secure local development.

## At a glance

|                 |                                                     |
|-----------------|-----------------------------------------------------|
| Provider        | `pass`                                              |
| URI             | `pass://[folder_prefix][?store_dir=/path/to/store]` |
| Access          | Read and write                                      |
| Best for        | Local, GPG-encrypted secret storage                 |
| Authentication  | The GPG key configured for the password store       |
| Default storage | `secretspec/{project}/{profile}/{key}`              |

## Quick start

```
# Set a secret$ secretspec set DATABASE_URL --provider passEnter value for DATABASE_URL: postgresql://localhost/mydb
# Run with secrets$ secretspec run --provider pass -- npm start
```

Terminal window

## Setup

### Prerequisites

```
# Debian/Ubuntu$ sudo apt-get install pass
# Fedora$ sudo dnf install pass
# Arch$ sudo pacman -S pass
# macOS$ brew install pass
```

Terminal window

### Authentication

SecretSpec uses the GPG identity configured for the password store.
Initialize the store once if needed:

```
$ pass init <gpg-key-id>
```

Terminal window

## Configuration

### URI format

```
pass://[folder_prefix][?store_dir=/path/to/store]
```

- `folder_prefix`: Optional path prefix supporting `{project}`,
  `{profile}`, and `{key}` placeholders. Defaults to
  `secretspec/{project}/{profile}/{key}`.
- `store_dir`: Optional password store directory. When set, it is
  exported as `PASSWORD_STORE_DIR` for every `pass` invocation,
  overriding the default `~/.password-store`. The variable is scoped to
  the spawned `pass` process and does not affect secretspec’s own
  environment.

### URI examples

```
passpass://shared/{profile}/{key}pass://?store_dir=/path/to/store
```

### Project configuration

```
[providers]local = "pass://"
[profiles.default]DATABASE_URL = { description = "Database URL", providers = ["local"] }
```

secretspec.toml

## Storage model

Secrets are stored with a hierarchical path structure:
`secretspec/{project}/{profile}/{key}`

For example, with project “myapp” and profile “default”:

```
$ pass show secretspec/myapp/default/DATABASE_URLpostgresql://localhost/mydb
```

Terminal window

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names an existing store entry instead, letting you read credentials you
already keep in `pass`: `item` is the entry path (`field` is not
supported). Reads and writes target that entry in place.

```
[profiles.default]GITHUB_TOKEN = { description = "GH token", ref = { item = "github/token" }, providers = ["pass"] }
```

## Advanced configuration

### Shared secrets

By default, secrets are stored under
`secretspec/{project}/{profile}/{key}`, which isolates them per project.
To share secrets across projects, use a custom folder prefix via the
URI:

```
[defaults.providers]shared = "pass://secretspec/shared/{profile}/{key}"
```

~/.config/secretspec/config.toml

The URI supports `{project}`, `{profile}`, and `{key}` placeholders. By
omitting `{project}`, multiple projects can read and write the same pass
entry:

```
# secretspec.toml (in project-A and project-B)[profiles.default]ARTIFACTORY_USER = { description = "Artifactory user", providers = ["shared"] }
```

Both projects will resolve `ARTIFACTORY_USER` from pass entry
`secretspec/shared/default/ARTIFACTORY_USER`.
