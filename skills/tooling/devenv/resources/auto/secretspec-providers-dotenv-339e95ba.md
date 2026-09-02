# Dotenv Provider

The [Dotenv](https://github.com/cachix/dotenv-ng) provider stores
secrets in local `.env` files for development setups and compatibility
with existing tools.

## At a glance

|  |  |
|----|----|
| Provider | `dotenv` |
| URI | `dotenv[:path]` |
| Access | Read and write |
| Best for | Local development and compatibility with `.env`-based tools |
| Authentication | None |
| Default storage | `.env` next to `secretspec.toml` (plain text) |

## Quick start

```
# Initialize from existing .env$ secretspec init --from .env
# Set a secret$ secretspec set DATABASE_URL --provider dotenvEnter value for DATABASE_URL: postgresql://localhost/mydb
# Run with secrets$ secretspec run --provider dotenv -- npm start
```

Terminal window

## Configuration

### URI format

```
# Default (.env next to secretspec.toml)dotenv
# Custom pathsdotenv:.env.localdotenv:config/.envdotenv:/absolute/path/.env
# Home-relative path (0.18+)dotenv:~/.config/my-project/.env
```

Starting in SecretSpec 0.18, a leading `~` path component expands to the
current user’s home directory.

### Environment variable

```
$ export SECRETSPEC_PROVIDER=dotenv:.env.local
```

Terminal window

### Project configuration

```
[providers]local = "dotenv:.env.local"
[profiles.default]DATABASE_URL = { description = "Database URL", providers = ["local"] }
```

secretspec.toml

## Storage model

Dotenv uses standard `KEY=VALUE` pairs:

```
DATABASE_URL=postgresql://localhost/mydbAPI_KEY=sk-1234567890DEBUG=true  # Comments supported
# Multi-line values must be quotedPRIVATE_KEY="-----BEGIN RSA PRIVATE KEY-----MIIEpAIBAAKCAQEA...-----END RSA PRIVATE KEY-----"
```

.env

The file itself provides the namespace. Project and profile names are
not included in keys; use a different file when environments need
separate values:

```
$ secretspec run --provider dotenv:.env.production -- node server.js
```

Terminal window

## Use existing secrets

By default each secret reads the key named after it. A secret’s
[`ref`](https://secretspec.dev/reference/configuration/#secret-references) field reads a key
stored under a different name: `item` is the `.env` key (`field` is not
supported). Reads and writes target that key in place; the secret’s own
name is ignored.

```
[profiles.default]DATABASE_URL = { description = "DB", ref = { item = "POSTGRES_URL" }, providers = ["dotenv://.env.shared"] }
```

## Security considerations
