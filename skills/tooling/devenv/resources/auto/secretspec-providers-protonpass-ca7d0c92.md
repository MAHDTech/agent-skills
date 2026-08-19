# Proton Pass Provider

The Proton Pass provider integrates with [Proton
Pass](https://proton.me/pass) for end-to-end encrypted cloud secret
storage.

## At a glance

|  |  |
|----|----|
| Provider | `protonpass` |
| URI | `protonpass://[vault_name[/title-template]]` |
| Access | Read and write |
| Best for | End-to-end encrypted cloud storage through Proton Pass |
| Authentication | A `pass-cli` login or personal access token |
| Default storage | Note item `{project}/{profile}/{key}` in the `secretspec` vault |
| Requires | Official `pass-cli`, pinned to a version you have tested (see [compatibility](#pass-cli-compatibility)) |

## Quick start

```
# Set a secret$ secretspec set DATABASE_URL --provider protonpass://PersonalEnter value for DATABASE_URL: postgresql://localhost/mydb
# Get a secret$ secretspec get DATABASE_URL --provider protonpass://Personal
# Run with secrets$ secretspec run --provider protonpass://Personal -- npm start
```

Terminal window

## Setup

### Prerequisites

- Proton Pass CLI (`pass-cli`) - download from
  [proton.me/pass/download](https://proton.me/pass/download)
- A Proton account, signed in via `pass-cli login`
- A vault to store secrets in (e.g. `pass-cli vault create secretspec`)
- A `pass-cli` version that works with your SecretSpec release, see
  [`pass-cli` compatibility](#pass-cli-compatibility)

### Authentication

For local use, sign in interactively:

```
$ pass-cli login
```

Terminal window

For CI, use a personal access token as shown in [CI/CD](#cicd).

## `pass-cli` compatibility

Each of these `pass-cli` releases changed behaviour the provider relies
on:

| `pass-cli` | What changed | SecretSpec |
|----|----|----|
| 2.0.3 (2026-05-19) | `item list` output shape | Handled in 0.12.1+ |
| 2.1.0 (2026-05-20) | Agent sessions reject audited item operations that carry no reason | Handled in 0.12.0+, see [Agent sessions](#agent-sessions) |
| 2.2.4 (2026-07-31) | `pass-cli test` removed | Handled in 0.19+ ([\#279](https://github.com/cachix/secretspec/issues/279)) |

SecretSpec probes the session once per run before any read or write.
SecretSpec 0.18.0 and earlier probe with `pass-cli test`, so on
`pass-cli` 2.2.4 and later every Proton Pass operation fails with:

```
Provider operation failed: error: unrecognized subcommand 'test'
```

SecretSpec 0.19+ tries `pass-cli info` and falls back to
`pass-cli test`, so it works with every `pass-cli` release regardless of
which check that release carries. `info` is preferred because it runs
behind the CLI’s authentication gate and so reports whether a valid
session is present, while `test` only proved that Proton’s servers were
reachable. A `pass-cli` carrying neither is reported as incompatible
with your SecretSpec release rather than surfacing the CLI’s usage text.

On SecretSpec 0.18.0 and earlier, use `pass-cli` 2.2.3, the last release
published before `pass-cli test` was removed.

### Pinning a `pass-cli` version

Install a specific release instead of tracking the latest build, and
point SecretSpec at it with `SECRETSPEC_PROTONPASS_CLI_PATH`:

```
$ curl -Lo ~/.local/bin/pass-cli-2.2.3 \    https://github.com/protonpass/pass-cli/releases/download/2.2.3/pass-cli-linux-x86_64
$ chmod +x ~/.local/bin/pass-cli-2.2.3
$ export SECRETSPEC_PROTONPASS_CLI_PATH="$HOME/.local/bin/pass-cli-2.2.3"
```

Terminal window

Every [release](https://github.com/protonpass/pass-cli/releases)
publishes a `.sha256` file next to each binary; verify it before use.
Pin the same version in CI rather than installing the latest `pass-cli`
on each run, and treat a `pass-cli` upgrade as a change worth testing:
run `secretspec check` against the new version before rolling it out.

## Configuration

### URI format

```
protonpass://[vault_name[/title-template]]
```

- `vault_name`: Target vault (defaults to `secretspec`)
- `title-template`: Item title pattern supporting `{project}`,
  `{profile}`, `{key}` placeholders

### URI examples

```
# Default vault ("secretspec")protonpass://
# Specific vaultprotonpass://Work
# Specific vault and custom title templateprotonpass://Work/{project}/{profile}/{key}
```

### Project configuration

```
[providers]team = "protonpass://Work"
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["team"] }
```

secretspec.toml

## Storage model

Secrets are stored as note items. The vault defaults to `secretspec`,
and the item title defaults to `{project}/{profile}/{key}`. The URI can
select another vault or replace the title template.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names an existing item instead: `item` is the exact item title, whose
note is read (`field` is not supported). Reads and writes target that
item in place.

```
[profiles.production]DATABASE_URL = { description = "DB", ref = { item = "Production Database" }, providers = ["protonpass://Work"] }
```

## CI/CD

```
# Create a token$ pass-cli personal-access-token create --name ci --expiration 1y
# Authenticate in CI (store the token as a CI secret)$ pass-cli login --pat $PROTON_PASS_PAT
$ secretspec run -- deploy
```

Terminal window

## Advanced configuration

### Agent sessions

`pass-cli` 2.1.0 introduced agent sessions, which require a
`PROTON_PASS_AGENT_REASON` to be set for audited item operations
(reading, creating, and deleting items). SecretSpec sets this
automatically, so existing secrets resolve correctly under an agent
session.

The reason recorded in the Proton Pass audit log is resolved in this
order:

1.  The `--reason` flag (or `SECRETSPEC_REASON` environment variable):

    ```
    $ secretspec run --reason "Deploying app from CI" -- ./deploy.sh
    ```

    Terminal window

    When using the Rust SDK, set it for the session with `with_reason`:

    ```
    use secretspec::Secrets;
    let spec = Secrets::load()?.with_reason("Deploying app from CI");
    ```

2.  The `PROTON_PASS_AGENT_REASON` environment variable read by
    `pass-cli`:

    ```
    $ export PROTON_PASS_AGENT_REASON="Deploying app from CI"
    ```

    Terminal window

3.  A default that identifies the secretspec version (e.g.
    `secretspec/0.11.0 (https://secretspec.dev)`).

To force a meaningful reason instead of falling back to the default, use
the
[`require_reason`](https://secretspec.dev/reference/configuration/#requiring-a-reason-for-secret-access)
policy in `secretspec.toml`. It defaults to `"agents"`, so sessions
SecretSpec detects as AI agents must explain why they read a secret.
Detection is heuristic; set it to `true` to require a reason from every
SecretSpec caller. secretspec then refuses operations through SecretSpec
that do not supply an explicit reason.
