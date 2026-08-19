+++
title = "secretspec-providers-kdbx-da0b1701"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

{% raw %}
# KeePass KDBX Provider

The KDBX provider reads and writes encrypted
[KeePass](https://keepass.info/) databases directly, without requiring
KeePass or KeePassXC to be installed.

## At a glance

|  |  |
|----|----|
| Provider | `kdbx` |
| URI | `kdbx:PATH[?keyfile=PATH][&prefix=TEMPLATE]` |
| Access | KDBX 3 read; KDBX 4 read and write |
| Best for | Local, portable KeePass-compatible encrypted storage |
| Authentication | Master password, key file, or both |
| Build feature | `kdbx` (0.17+) |
| Default storage | Nested groups `secretspec` → `{project}` → `{profile}`; entry titled `{key}`; field `Password` |

## Quick start

```
[providers]kdbx = {  uri = "kdbx:./secrets.kdbx",  credentials = { password = "keyring" }}
```

secretspec.toml

```
# Store the database master password in the bootstrap provider.$ secretspec config provider login kdbxEnter password for provider 'kdbx' (source: keyring): ****
# Set a secret in an existing KDBX 4 database, or create a new KDBX 4 database.$ secretspec set DATABASE_URL --provider kdbxEnter value for DATABASE_URL: postgresql://localhost/mydb✓ Secret DATABASE_URL saved to kdbx
$ secretspec get DATABASE_URL --provider kdbxpostgresql://localhost/mydb
$ secretspec run --provider kdbx -- npm start
```

Terminal window

## Setup

The provider is built into standard SecretSpec 0.17 binaries. Custom
builds must enable the `kdbx` feature.

### Authentication

Load the semantic `password` [provider
credential](https://secretspec.dev/reference/provider-credentials/) from a bootstrap provider
such as the system keyring. This keeps the KDBX master password out of
shell profiles and child-process environments:

```
[providers]kdbx = {  uri = "kdbx:./secrets.kdbx",  credentials = { password = "keyring" }}
```

secretspec.toml

Store the declared credential once:

```
$ secretspec config provider login kdbxEnter password for provider 'kdbx' (source: keyring): ****
```

Terminal window

`SECRETSPEC_KDBX_PASSWORD` is available as a fallback for environments
without a suitable bootstrap provider. Avoid it for normal interactive
use, and do not persist the master password in a shell profile.

Use `?keyfile=PATH` for a KeePass key file. When both a password and key
file are configured, both are required to unlock the database, matching
KeePass. Relative database and key-file paths resolve from the directory
containing `secretspec.toml`.

## Provider credentials

| Credential | Environment fallback       | Available since |
|------------|----------------------------|-----------------|
| `password` | `SECRETSPEC_KDBX_PASSWORD` | 0.17+           |

See the complete [provider credential
reference](https://secretspec.dev/reference/provider-credentials/) for all supported providers
and environment fallbacks.

## Configuration

### URI format

```
kdbx:PATH[?keyfile=PATH][&prefix=TEMPLATE]
```

- `PATH` is the KDBX database. Use `./` for a relative path so its
  spelling and case are preserved as a URI path.
- `keyfile` is an optional KeePass key file.
- `prefix` changes the convention entry path. It accepts `{project}`,
  `{profile}`, and `{key}` placeholders and defaults to
  `secretspec/{project}/{profile}/{key}`.

### URI examples

```
kdbx:./secrets.kdbxkdbx:/var/lib/myapp/secrets.kdbxkdbx:./secrets.kdbx?keyfile=./secrets.keykdbx:./shared.kdbx?prefix=teams/{project}/{profile}/{key}
```

### Project configuration

```
[providers]local_vault = {  uri = "kdbx:./secrets.kdbx?keyfile=./secrets.key",  credentials = { password = "keyring" }}
[profiles.default]DATABASE_URL = { description = "Database URL", providers = ["local_vault"] }
```

secretspec.toml

## Storage model

Yes: the `/` characters in the default convention address separate
nested KeePass groups. The path starts inside the database’s root group;
neither the database filename nor the root group’s display name is part
of it. SecretSpec then uses the final path component as the entry title
and stores the value in the entry’s protected `Password` field.

For this configuration:

```
[project]name = "my-app"revision = "1.0"
[profiles.default]DATABASE_URL = { description = "Database URL" }
```

secretspec.toml

the KeePass tree is:

```
Database root (its name does not matter)└── secretspec                         group    └── my-app                         group ([project].name)        └── default                    group (active profile)            └── DATABASE_URL           entry (secret key)                └── Password = <secret value>
```

### Set up an entry manually

1.  Open the database file named by the provider URI, such as
    `secrets.kdbx` for `kdbx:./secrets.kdbx`. The file itself can have
    any name.
2.  Directly below the database’s root group, create a group named
    `secretspec`.
3.  Inside it, create a group whose name exactly matches
    `[project].name` in `secretspec.toml`.
4.  Inside the project group, create a group whose name exactly matches
    the active profile, such as `default`.
5.  Inside the profile group, create an entry whose **Title** exactly
    matches the secret key, such as `DATABASE_URL`, and put the secret
    value in its **Password** field.

Do not rename the database or its root group to `secretspec`;
`secretspec` is a child group of the root. Group names and entry titles
are case-sensitive. If you want SecretSpec to write to a manually
created database, save it as KDBX 4. You can also let `secretspec set`
create the missing groups and entry automatically.

Reads open KDBX 3 and KDBX 4 databases. Writes create KDBX 4 databases
and atomically replace an existing KDBX 4 file only after the complete
encrypted replacement has been flushed. KDBX 3 databases must be
upgraded with KeePass or KeePassXC before SecretSpec can write them.

## Use existing secrets

Use [`ref`](https://secretspec.dev/reference/configuration/#secret-references) to name an
existing entry by its complete group path and title. The optional
`field` selects a standard or custom entry field; it defaults to
`Password`.

```
[profiles.production]DATABASE_PASSWORD = {  description = "Existing KeePass entry",  ref = { item = "Infrastructure/PostgreSQL", field = "Password" },  providers = ["local_vault"]}DATABASE_USERNAME = {  description = "Username from the same entry",  ref = { item = "Infrastructure/PostgreSQL", field = "UserName" },  providers = ["local_vault"]}
```

Entry and group names are matched exactly. Duplicate titles within one
group, or duplicate group names under one parent, are rejected as
ambiguous instead of selecting an arbitrary value. Empty path components
are not supported. The `Title` field is readable but not writable
because it forms part of the entry address; rename entries in KeePass or
KeePassXC.

## Security considerations and limitations

- Never place the master password in the URI. Use the `password`
  provider credential from a bootstrap provider.
  `SECRETSPEC_KDBX_PASSWORD` is a discouraged fallback for environments
  without one; reported provider URIs never contain the password.
- Keep key files separate from the KDBX database when possible.
  Possessing both removes the extra protection a key file provides.
- SecretSpec serializes KDBX operations within one process and replaces
  files atomically, but KDBX is still a local file rather than a
  multi-writer service. Avoid editing the same database simultaneously
  in SecretSpec and KeePass.
- Writing uses the `keepass` crate’s KDBX 4 writer. Back up important
  databases before first use with a new SecretSpec or `keepass` version.

{% endraw %}
