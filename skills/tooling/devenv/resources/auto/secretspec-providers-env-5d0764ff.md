{% raw %}
# Environment Variable Provider

The Environment Variable provider reads secrets directly from process
environment variables. This is a **read-only** provider designed for
CI/CD compatibility and containerized environments.

## At a glance

|                 |                                                       |
|-----------------|-------------------------------------------------------|
| Provider        | `env`                                                 |
| URI             | `env://`                                              |
| Access          | Read-only                                             |
| Best for        | CI/CD, containers, and temporary overrides            |
| Authentication  | None                                                  |
| Default storage | Current process environment; values are not persisted |

## Quick start

```
# Set environment variablesexport DATABASE_URL="postgresql://localhost/mydb"export API_KEY="sk-1234567890"
# Check secrets are available$ secretspec check --provider env✓ All required secrets are configured
# Run with environment variables$ secretspec run --provider env -- npm start
```

Terminal window

## Configuration

The env provider accepts no configuration options:

```
# All these are equivalent$ secretspec check --provider env$ secretspec check --provider env:$ secretspec check --provider env://
```

Terminal window

### Project configuration

```
[providers]injected = "env"
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["injected"] }
```

secretspec.toml

## Storage model

Convention secrets read the environment variable with the same name. The
provider reads only the current process environment, never writes
variables, and does not persist values.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
reads a different variable, which is useful when your infrastructure
already exposes a value under another name: `item` is the variable name,
case-sensitive and preserved verbatim (`field` is not supported). Like
the rest of this provider, references are read-only.

```
[profiles.default]DATABASE_URL = { description = "DB", ref = { item = "POSTGRES_CONNECTION_STRING" }, providers = ["env"] }
```

## CI/CD

```
# GitHub Actions- name: Run with secrets  env:    DATABASE_URL: ${{ secrets.DATABASE_URL }}    API_KEY: ${{ secrets.API_KEY }}  run: |    secretspec run --provider env -- npm run deploy
```

## When to use

- Running in CI/CD pipelines where secrets are injected as environment
  variables
- Testing with temporary environment variables
- Working with containerized applications that use environment variables

{% endraw %}