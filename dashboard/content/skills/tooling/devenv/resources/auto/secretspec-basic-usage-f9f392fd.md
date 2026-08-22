+++
title = "secretspec-basic-usage-f9f392fd"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Basic Usage

Once your project has a `secretspec.toml` file and you have selected a
default provider, most day-to-day work uses a small set of commands.

## Check required secrets

Check that every required secret can be resolved. Missing values are
shown without printing any secrets, and SecretSpec offers to set them
interactively:

```
$ secretspec check
```

Terminal window

Use `secretspec check --no-prompt` in CI or other non-interactive
environments. It exits with an error when a required secret is missing.

## Store or replace a value

Set a secret without putting its value in your shell history:

```
$ secretspec set API_KEYEnter value for API_KEY (profile: development): ********✓ Secret 'API_KEY' saved to keyring (profile: development)
```

Terminal window

Running `set` again replaces the stored value. The secret must already
be declared in `secretspec.toml`.

## Read one value

Resolve and print a single secret:

```
$ secretspec get DATABASE_URLpostgresql://localhost/myapp
```

Terminal window

## Run your application

Start a command with the resolved secrets available as environment
variables:

```
$ secretspec run -- npm start
```

Terminal window

The `--` separates SecretSpec’s options from the command you want to
run. SecretSpec stops before starting the command if a required secret
is missing.

## Add a declaration (0.18+)

Declare a new secret without editing `secretspec.toml` by hand, then
store its value:

```
$ secretspec add API_KEY --description "API access token"$ secretspec set API_KEY
```

Terminal window

`add` changes only the declaration. It never asks for or stores the
secret value.

## Delete stored values (0.18+)

Remove a stored value from its provider:

```
$ secretspec delete API_KEY
```

Terminal window

This leaves the declaration in `secretspec.toml`, so the project still
records that it expects `API_KEY`. See the [CLI
reference](https://secretspec.dev/reference/cli/#delete-018) for deleting multiple values or
using `--all`.

## Use another profile or provider

Your configured defaults apply automatically. Override them for one
command with `--profile` or `--provider`:

```
$ secretspec check --profile production$ secretspec run --provider dotenv://.env.test -- npm test
```

Terminal window

These options do not change your saved preferences.

## Next steps

- See every option in the [CLI command reference](https://secretspec.dev/reference/cli/)
- Learn how [profiles](https://secretspec.dev/concepts/profiles/) separate environments
- Explore available [providers](https://secretspec.dev/concepts/providers/)

