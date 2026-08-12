+++
title = "secretspec-reference-cli-441b3663"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# CLI Commands Reference

The SecretSpec CLI provides commands for managing secrets across
different providers and profiles.

## Global Options

These options are available on every command:

| Option | Description |
|----|----|
| `-f, --file <FILE>` | Path to `secretspec.toml` (default: auto-detect). Env: `SECRETSPEC_FILE` |
| `--reason <REASON>` | Reason for accessing secrets, recorded by providers that support audit logging (e.g. Proton Pass agent sessions). Takes precedence over `PROTON_PASS_AGENT_REASON`. Env: `SECRETSPEC_REASON` |

```
$ secretspec run --reason "Deploying web frontend" -- ./deploy.sh
```

Terminal window

## Commands

### init

Initialize a new `secretspec.toml` configuration file from an existing
.env file.

```
secretspec init [OPTIONS]
```

Terminal window

**Options:**

- `--from <PATH>` - Path to .env file to import from (default: `.env`)

**Example:**

```
$ secretspec init --from .env.example✓ Created secretspec.toml with 5 secrets
```

Terminal window

### config global init

Initialize user-global configuration. The explicit `global` namespace is
available in SecretSpec 0.17+; without options, the command prompts for
the provider and profile.

```
secretspec config global init [--provider <PROVIDER>] [--profile <PROFILE>] # 0.17+
```

Terminal window

SecretSpec 0.17+ accepts `--provider` and `--profile` so installations
can save both defaults without interaction. Each omitted option still
prompts; use `--profile none` to clear the saved default profile. The
corresponding `SECRETSPEC_PROVIDER` and `SECRETSPEC_PROFILE` environment
variables are also accepted. Project requirements remain in
`secretspec.toml`; the namespace makes it clear that this command writes
user-wide defaults. The legacy `secretspec config init` spelling remains
supported as a hidden alias.

**Example:**

```
$ secretspec config global init  # 0.17+? Select your preferred provider backend:> keyring: System keychain? Select your default profile:> development✓ Configuration saved to ~/.config/secretspec/config.toml
```

Terminal window

```
# SecretSpec 0.17+: save both defaults without prompting$ secretspec config global init --provider env --profile default✓ Configuration saved to ~/.config/secretspec/config.toml
```

Terminal window

### config global show

Display current user-global configuration. The explicit namespace is
available in SecretSpec 0.17+; `secretspec config show` remains a hidden
alias.

```
secretspec config global show # 0.17+
```

Terminal window

**Example:**

```
$ secretspec config global show  # 0.17+Provider: keyringProfile:  development
```

Terminal window

### config global provider add

Add a provider alias to your user-level configuration
(`~/.config/secretspec/config.toml`).

To share aliases with your team, declare them in a top-level
`[providers]` table in `secretspec.toml` instead — they take precedence
over user-level aliases on name conflict.

```
secretspec config global provider add <ALIAS> <URI> [--credential NAME=PROVIDER]... # 0.17+
```

Terminal window

**Arguments:**

- `<ALIAS>` - Short name for the provider (e.g., `prod_vault`, `shared`)
- `<URI>` - Provider URI (e.g., `onepassword://Production`, `env://`)

**Options:**

- `--credential <NAME=PROVIDER>` - Declare a [provider
  credential](https://secretspec.dev/concepts/providers/#provider-credentials) and its source.
  `NAME` is semantic and provider-specific, such as `access_token` or
  `role_id`. Repeatable. Only the bare-string source form is expressible
  on the command line; add a `ref` by editing the config.

**Example:**

```
$ secretspec config global provider add prod_vault "onepassword://Production" # 0.17+✓ Provider alias 'prod_vault' added: 'onepassword://Production'
$ secretspec config global provider add bws "bws://project-uuid" --credential access_token=keyring # 0.17+✓ Provider alias 'bws' added: 'bws://project-uuid'  credentials: access_token=keyring  run 'secretspec config provider login bws' to store the credentials
```

Terminal window

### config global provider list

List all configured user-level provider aliases. Project-level aliases
declared in `secretspec.toml` are not shown by this command.

```
secretspec config global provider list # 0.17+
```

Terminal window

**Example:**

```
$ secretspec config global provider list  # 0.17+prod_vault  → onepassword://Productionshared      → onepassword://Sharedenv         → env://
```

Terminal window

### config global provider remove

Remove a provider alias from your user-level configuration. To remove a
project-level alias, edit the `[providers]` table in `secretspec.toml`
directly.

```
secretspec config global provider remove <ALIAS> # 0.17+
```

Terminal window

**Arguments:**

- `<ALIAS>` - Name of the alias to remove

**Example:**

```
$ secretspec config global provider remove prod_vault  # 0.17+✓ Provider alias 'prod_vault' removed
```

Terminal window

### config provider login

Store the [credentials](https://secretspec.dev/concepts/providers/#provider-credentials) a
provider alias declares. Prompts (hidden input) for each credential and
writes it to its source provider at the exact location resolution reads
it back from. Runs in a project, like `set` and `check`.

```
secretspec config provider login <ALIAS>
```

Terminal window

**Arguments:**

- `<ALIAS>` - Name of the alias whose credentials to store

**Example:**

```
$ secretspec config provider login bwsEnter access_token for provider 'bws' (source: keyring): ****✓ stored access_token in keyring at myproject/default/access_token
Run 'secretspec check --provider bws' to verify authentication.
```

Terminal window

A read-only source provider is rejected. An alias that declares no
credentials reports that there is nothing to store.

### check

Check if all required secrets are available, with interactive prompting
for missing secrets.

```
secretspec check [OPTIONS]
```

Terminal window

**Options:**

- `-p, --provider <PROVIDER>` - Provider backend to use
- `-P, --profile <PROFILE>` - Profile to use
- `-S, --scope <SCOPE>` - Resolve only a `[scopes]` subset of the
  profile (SecretSpec 0.17+)
- `-n, --no-prompt` - Don’t prompt for missing secrets (exit with error
  if any are missing)
- `--json` - Print a value-free resolution report as JSON instead of
  prompting
- `--explain` - Print a value-free, human-readable resolution trace
  instead of prompting

**Example:**

```
$ secretspec check --profile production✓ DATABASE_URL - Database connection string✗ API_KEY - API key for external service (required)Enter value for API_KEY (profile: production): ****✓ Secret 'API_KEY' saved to keyring (profile: production)
```

Terminal window

#### Resolution report (`--json` / `--explain`)

`--json` and `--explain` report how every declared secret resolved for
the active profile without prompting and without ever printing a secret
value. Both exit non-zero when a required secret is missing, so they
work as a CI gate.

`--explain` prints a human-readable trace:

```
$ secretspec check --profile development --explainprofile:  developmentprovider: keyring://  DATABASE_URL        ok        source keyring://  DEV_SESSION_SECRET  ok        default value  JWT_SECRET          ok        generated  SENTRY_DSN          missing   optional  STRIPE_KEY          MISSING   required
```

Terminal window

`--json` emits a versioned, machine-readable object for tooling and CI.
Each entry reports the `status` (`resolved`, `missing_required`,
`missing_optional`), whether the value came from a provider
(`source_provider`, credential-free), a generator (`generated`), or a
committed default (`default_applied`), and whether it is exposed
`as_path`. No secret values appear. The canonical JSON Schema is
committed at `schema/resolution-report.schema.json`.

```
$ secretspec check --profile production --json{  "schema_version": 1,  "provider": "keyring://",  "profile": "production",  "secrets": [    { "name": "DATABASE_URL", "status": "resolved", "required": true, "source_provider": "keyring://", "default_applied": false, "generated": false, "as_path": false },    { "name": "STRIPE_KEY", "status": "missing_required", "required": true, "default_applied": false, "generated": false, "as_path": false }  ]}
```

Terminal window

### get

Get a secret value.

```
secretspec get [OPTIONS] <NAME>
```

Terminal window

**Options:**

- `-p, --provider <PROVIDER>` - Provider backend to use
- `-P, --profile <PROFILE>` - Profile to use

**Example:**

```
$ secretspec get DATABASE_URL --profile productionpostgresql://prod.example.com/mydb
```

Terminal window

For a composed secret, `get` resolves its transitive dependencies and
prints the derived value. Available since SecretSpec 0.16.

### schema

Emit a single-root JSON Schema for the manifest’s typed shape: by
default the union `SecretSpec` (safe for any profile); with `--profile`,
that profile’s exact fields. Value-free: reads only the manifest, never
a provider.

```
secretspec schema [OPTIONS]
```

Terminal window

**Options:**

- `-P, --profile <PROFILE>` - Emit the schema for this profile’s fields
  instead of the union
- `-o, --output <FILE>` - Write to this file instead of stdout

Rather than ship a typed-accessor generator per language, feed this
schema to [quicktype](https://quicktype.io), which generates an
idiomatic type **and** deserializer for any language. Name the type with
`--top-level`. At runtime, hand the generated deserializer the flat
`{SECRET_NAME: value}` map from the SDK’s `fields()` helper:

```
$ secretspec schema | quicktype -s schema --top-level SecretSpec --lang python -o secrets_gen.py
```

Terminal window

```
from secretspec import SecretSpecfrom secrets_gen import SecretSpec as Secrets  # quicktype-generated, typed
resolved = SecretSpec.builder().with_reason("boot").load()s = Secrets.from_dict(resolved.fields())print(s.database_url)   # typed str
```

The same pattern works in every SDK: Go
`UnmarshalSecretSpec(resolved.FieldsJSON())`, TypeScript
`Convert.toSecretSpec(resolved.fieldsJson())`, Ruby
`SecretSpec.from_dynamic!(resolved.fields)`.

### set

Set a secret value.

```
secretspec set [OPTIONS] <NAME> [VALUE]
```

Terminal window

**Options:**

- `-p, --provider <PROVIDER>` - Provider backend to use
- `-P, --profile <PROFILE>` - Profile to use

**Example:**

```
$ secretspec set API_KEY sk-1234567890✓ Secret 'API_KEY' saved to keyring (profile: development)
```

Terminal window

`set` rejects composed secrets because their values are derived and
read-only. Available since SecretSpec 0.16.

### run

Run a command with secrets injected as environment variables.

```
secretspec run [OPTIONS] -- <COMMAND>
```

Terminal window

**Options:**

- `-p, --provider <PROVIDER>` - Provider backend to use
- `-P, --profile <PROFILE>` - Profile to use
- `-S, --scope <SCOPE>` - Inject only a `[scopes]` subset of the profile
  (SecretSpec 0.17+)

**Examples:**

```
# Run npm with secrets available as environment variables$ secretspec run --profile production -- npm run deploy
# Verify secrets are injected$ secretspec run -- env | grep DATABASE_URLDATABASE_URL=postgresql://localhost/mydb
# Inject only the `api` scope's secrets (SecretSpec 0.17+); secrets the# scope excludes are removed from the child even if the parent exported them$ secretspec run --scope api -- ./api-server
```

Terminal window

The `--provider` override applies to every secret, including those with
a [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field: refs are
redirected to the overriding provider just like convention secrets. This
makes it easy to point refs at fixtures during tests without editing the
manifest:

```
# Resolve every secret, refs included, from a fixtures file$ secretspec run --provider dotenv:.env.fixtures -- cargo test
```

Terminal window

### export

Resolve every secret for the active profile and write it to stdout in a
chosen format, without running a command. Unlike `run`, it never prompts
and exits non-zero when a required secret is missing, so CI can gate on
it.

```
secretspec export [OPTIONS]
```

Terminal window

Options are `-p, --provider <PROVIDER>`, `-P, --profile <PROFILE>`,
`-S, --scope <SCOPE>` (a `[scopes]` subset of the profile, SecretSpec
0.17+), and `--format <FORMAT>` (default `shell`).

Unlike [`run --scope`](#run), `export --scope` only emits the scoped
subset; it unsets nothing, because no output format can express an
unset. A shell that already holds a wider set keeps those values after a
scoped `export`, so use `run --scope` when the point is to narrow an
existing environment.

| Format | Output |
|----|----|
| `shell` | `export KEY='value'` lines, ready for `eval "$(secretspec export)"` |
| `dotenv` | `KEY="value"` lines in dotenv syntax (double-quoted, with `\`, `"`, `$`, and newline escaped) |
| `json` | a single compact JSON object mapping each secret name to its value |
| `gha` | appends `KEY=value` to the file named by `$GITHUB_ENV` and prints an `::add-mask::` command per value to stdout, so later workflow steps and third-party actions see the secrets |

```
# Load secrets into the current shell$ eval "$(secretspec export --profile production)"
# Emit JSON for another tool to consume$ secretspec export --profile production --format json{"DATABASE_URL":"postgresql://prod.example.com/mydb"}
```

Terminal window

The `gha` format targets a `secretspec export --format gha` step in a
GitHub or Forgejo Actions job: it masks the values in the runner log and
persists them to the job environment for the steps that follow.

### import

Import secrets from one provider to another.

```
secretspec import <FROM_PROVIDER>
```

Terminal window

The destination provider and profile are determined from your
configuration. Secrets that already exist in the destination provider
will not be overwritten.

**Arguments:**

- `<FROM_PROVIDER>` - Provider to import from (e.g., `env`,
  `dotenv:/path/to/.env`)

**Example:**

```
# Import from environment variables to your default provider$ secretspec import envImporting secrets from env to keyring (profile: development)...
✓ DATABASE_URL - Database connection string○ API_KEY - API key for external service (already exists in target)✗ REDIS_URL - Redis connection URL (not found in source)
Summary: 1 imported, 1 already exists, 1 not found in source
# Import from a specific .env file$ secretspec import dotenv:/home/user/old-project/.env
```

Terminal window

**Use Cases:**

- Migrate from .env files to a secure provider like keyring or
  OnePassword
- Copy secrets between different profiles or projects
- Import existing environment variables into SecretSpec management

`import` skips composed secrets because they have no stored value to
copy; their component secrets are imported normally. Available since
SecretSpec 0.16.

### cache clear (0.17+)

Delete cached provider values for one secret, or for every cached secret
in the active profile. Authoritative fallback providers are not
modified.

```
secretspec cache clear [NAME] [--profile <PROFILE>]
```

Terminal window

**Arguments and options:**

- `[NAME]` - Cached secret to clear. Omit it to clear all cached secrets
  in the profile.
- `-P, --profile <PROFILE>` - Profile whose logical cache entries are
  cleared.

The reported count is the number of entries that were actually removed,
so a profile with nothing cached reports `Cleared 0 cache entries`.
`--provider` and `SECRETSPEC_PROVIDER` are ignored: clearing always
addresses the cache of the route the manifest declares. When one cache
store cannot be cleared, the remaining secrets are still cleared and the
command then reports what failed.

```
# Force the next API_KEY read through its authoritative fallback route$ secretspec cache clear API_KEYCleared 1 cache entry
# Clear every cached secret in production$ secretspec cache clear --profile productionCleared 4 cache entries
```

Terminal window

See [Provider caching](https://secretspec.dev/concepts/providers/caching/) for configuration
and resolution behavior.

### audit

Show the local [audit log](https://secretspec.dev/concepts/audit/) of secret access.

```
secretspec audit [--project <NAME>] [--action <ACTION>] [-n <N>] [--json]
```

Terminal window

**Options:**

- `--project <NAME>` - Only show entries for this project
- `--action <ACTION>` - Only show entries for this action (`get`, `set`,
  `check`, `run`, `import`, `export`, or `cache_clear` and
  `cache_refresh` in 0.17+)
- `-n, --tail <N>` - Show only the last N entries
- `--json` - Output raw JSON Lines instead of the formatted summary

The log location is read from your user-global config (`[audit]` in
`~/.config/secretspec/config.toml`), defaulting to the per-user state
directory.

**Example:**

```
$ secretspec audit --action run -n 52026-06-04T18:06:29Z  run    found  ./deploy.sh  API_KEY,DATABASE_URL  (my-app/production)  reason: deploy  [claude-code]
# Pipe raw entries to jq$ secretspec audit --json | jq 'select(.outcome == "missing")'
```

Terminal window

## Environment Variables

| Variable              | Description                                       |
|-----------------------|---------------------------------------------------|
| `SECRETSPEC_PROFILE`  | Default profile to use                            |
| `SECRETSPEC_PROVIDER` | Default provider to use                           |
| `SECRETSPEC_FILE`     | Path to `secretspec.toml` (same as `--file`)      |
| `SECRETSPEC_REASON`   | Reason for accessing secrets (same as `--reason`) |

## Quick Start Workflow

```
# Initialize from existing .env$ secretspec init --from .env
# Set up user-global defaults (0.17+)$ secretspec config global init
# Import existing secrets (optional)$ secretspec import env  # or: secretspec import dotenv:.env.old
# Check and set missing secrets$ secretspec check
# Run your application$ secretspec run -- npm start
```

Terminal window

{% endraw %}
