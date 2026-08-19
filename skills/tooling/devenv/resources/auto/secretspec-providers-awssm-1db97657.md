# AWS Secrets Manager Provider

The AWS Secrets Manager provider integrates with AWS for centralized
secret management.

## At a glance

|                 |                                                 |
|-----------------|-------------------------------------------------|
| Provider        | `awssm`                                         |
| URI             | `awssm://[AWS_PROFILE@]REGION[?options]`        |
| Access          | Read and write; secret references are read-only |
| Best for        | Workloads and teams on AWS                      |
| Authentication  | Standard AWS SDK credential chain               |
| Build feature   | `awssm`                                         |
| Default storage | `[prefix/]secretspec/{project}/{profile}/{key}` |

## Quick start

```
# Set a secret$ secretspec set DATABASE_URL --provider awssm://us-east-1Enter value for DATABASE_URL: postgresql://localhost/mydb✓ Secret 'DATABASE_URL' saved to awssm (profile: default)
# Run with secrets$ secretspec run --provider awssm://us-east-1 -- npm start
```

Terminal window

## Setup

### Prerequisites

- AWS account with Secrets Manager access
- AWS credentials configured (CLI, environment variables, IAM roles, or
  SSO)
- Build with `--features awssm`

### Authentication

AWS Secrets Manager uses the standard AWS SDK credential chain:

1.  Environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
2.  Shared credentials file (`~/.aws/credentials`)
3.  AWS SSO (`aws sso login`)
4.  IAM roles (EC2 instance profiles, ECS task roles, Lambda execution
    roles)

### Required IAM permissions

For identities used only to read secrets, such as those running
`secretspec get`, `secretspec check`, or `secretspec run`, use a
read-only policy. Replace the example region and account ID with your
own:

```
{  "Version": "2012-10-17",  "Statement": [    {      "Sid": "SecretspecBatchFetch",      "Effect": "Allow",      "Action": "secretsmanager:BatchGetSecretValue",      "Resource": "*",      "Condition": {        "StringEquals": {          "aws:RequestedRegion": "us-east-1"        }      }    },    {      "Sid": "SecretspecRead",      "Effect": "Allow",      "Action": "secretsmanager:GetSecretValue",      "Resource": "arn:aws:secretsmanager:us-east-1:123456789012:secret:secretspec/*"    }  ]}
```

Identities that run `secretspec set` also need this statement in the
policy’s `Statement` array:

```
{  "Sid": "SecretspecWrite",  "Effect": "Allow",  "Action": [    "secretsmanager:CreateSecret",    "secretsmanager:PutSecretValue"  ],  "Resource": "arn:aws:secretsmanager:us-east-1:123456789012:secret:secretspec/*"}
```

If you use a prefix such as `?prefix=myteam`, adjust the secret ARN in
the read and write statements:

```
arn:aws:secretsmanager:us-east-1:123456789012:secret:myteam/secretspec/*
```

## Configuration

### URI format

```
awssm://[AWS_PROFILE@]REGION[?prefix=PREFIX][&kms_key_id=KEY][&tag.NAME=VALUE...]
```

- `REGION`: AWS region (e.g., `us-east-1`). If omitted, the SDK default
  region chain is used.
- `AWS_PROFILE`: Optional AWS profile from `~/.aws/credentials`. If
  omitted, the SDK default credential chain is used.
- `PREFIX`: Optional root prefix prepended to all secret names. Useful
  when IAM policies scope access by prefix (e.g., only allow
  `myteam/*`).
- `kms_key_id`: Optional KMS key (id, ARN, or `alias/...`) used to
  encrypt secrets that secretspec creates.
- `tag.NAME=VALUE`: Optional tags applied to secrets that secretspec
  creates. Repeat for multiple tags.

`kms_key_id` and `tag.NAME=VALUE` are applied **only when secretspec
creates a secret** (`CreateSecret`); updating a value (`PutSecretValue`)
accepts neither, and a pre-existing secret keeps the key and tags it was
created with. This supports AWS “tag-on-create” guardrails, where an SCP
or IAM condition denies `CreateSecret` unless required
`aws:RequestTag/*` tags (and often a customer-managed key) are present
in the same call.

### URI examples

```
awssm://us-east-1awssm://production@us-east-1awssm://us-east-1?prefix=myteamawssm://prod@us-east-1?kms_key_id=alias/my-key&tag.team=platform&tag.env=prodawssm
```

### Project configuration

Because guardrail tags and keys usually vary per environment, they are a
natural fit for a checked-in [provider alias](https://secretspec.dev/reference/configuration/)
in `secretspec.toml`:

```
[providers]prod = "awssm://prod@us-east-1?kms_key_id=alias/my-key&tag.team=platform&tag.env=prod"
```

Route secrets through the alias in project configuration:

```
[profiles.production]DATABASE_URL = { description = "Database URL", providers = ["prod"] }
```

secretspec.toml

## Storage model

Secrets are stored as `[prefix/]secretspec/{project}/{profile}/{key}`.

For example, `DATABASE_URL` in project `myapp` and profile `production`
is stored as `secretspec/myapp/production/DATABASE_URL`. With
`?prefix=myteam`, it becomes
`myteam/secretspec/myapp/production/DATABASE_URL`.

## Use existing secrets

A secret’s [`ref`](https://secretspec.dev/reference/configuration/#secret-references) field
names an existing secret instead: `item` is the secret name (or ARN),
and the optional `field` selects one key of a JSON secret value. Without
`field`, the whole secret string is returned. References are
**read-only** in this provider.

```
[profiles.production]# Whole secret valueDATABASE_URL = { description = "DB", ref = { item = "prod/database-url" }, providers = ["awssm://us-east-1"] }# One key of a JSON secret valueDB_PASSWORD = { description = "DB pw", ref = { item = "prod/db-credentials", field = "password" }, providers = ["awssm://us-east-1"] }
```

## CI/CD

```
# Using environment variables$ export AWS_ACCESS_KEY_ID=AKIA...
$ export AWS_SECRET_ACCESS_KEY=...
$ export AWS_DEFAULT_REGION=us-east-1
# Run command$ secretspec run --provider awssm://us-east-1 -- deploy
# Or with IAM roles (no credentials needed)$ secretspec run --provider awssm://us-east-1 -- deploy
```

Terminal window
