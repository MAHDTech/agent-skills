+++
title = "secretspec-concepts-declarative-bfd496bf"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Declarative Configuration

SecretSpec uses `secretspec.toml` to declare what secrets your
application needs, separating requirements from storage mechanisms for
portability across environments.

## Basic Structure

```
[project]name = "my-app"revision = "1.0"extends = ["../shared/common"]  # Optional: inherit from other configs
[profiles.default]DATABASE_URL = { description = "PostgreSQL connection string", required = true }API_KEY = { description = "External API key", required = true }SESSION_SECRET = { description = "Session signing secret", required = true, type = "password", generate = true }
```

## Secret Declarations

Each secret is declared with configuration options:

```
SECRET_NAME = {  description = "Human-readable explanation",  # Required: shown in prompts  required = true,                            # Optional: defaults to true  default = "value"                           # Optional: fallback if not set}
```

**Options:**

- `description`: Explains the secret’s purpose (required in the
  `default` profile; profile overrides inherit it when omitted)
- `required`: Whether the secret must be provided (default: `true`)
- `default`: Fallback value for optional secrets
- `composed` (0.16+): Derive a read-only value from other declared
  secrets (see [Composed Secrets](https://secretspec.dev/concepts/composed-secrets/) for the
  strict template and dependency semantics)
- `type`: Secret type for auto-generation (`password`, `hex`, `base64`,
  `uuid`, `command`)
- `generate`: Enable auto-generation when the secret is missing (`true`
  or a table with options)
- `prompt` (0.19+): Securely ask for a missing value during
  `secretspec run` and let the selected provider decide whether to save
  the answer

## Related Concepts

- [Configuration Inheritance](https://secretspec.dev/concepts/inheritance/) lets projects
  share common secret definitions via the `extends` field
- [Secret Generation](https://secretspec.dev/concepts/generation/) auto-creates passwords,
  tokens, and keys when secrets are missing
- [Run prompts
  (0.19+)](https://secretspec.dev/reference/configuration/#prompt-on-missing-during-run-019)
  provision stored secrets on first use, or remain invocation-only with
  `null`
- [Composed Secrets (0.16+)](https://secretspec.dev/concepts/composed-secrets/) derive values
  from other declared secrets without dotenv or shell expansion

## Best Practices

1.  **Descriptive names**: Use `STRIPE_API_KEY` instead of generic
    `API_KEY`
2.  **Clear descriptions**: Help developers understand each secret’s
    purpose
3.  **Sensible defaults**: Provide development defaults, require
    production values
4.  **Modular inheritance**: Create reusable base configurations for
    common patterns

## Complete Example

```
[project]name = "web-api"revision = "1.0"extends = ["../shared/base", "../shared/auth"]
[profiles.default]# Inherits DATABASE_URL, INTERNAL_API_KEY from base# Inherits JWT_SECRET, SESSION_SECRET from auth# Service-specific additions:STRIPE_API_KEY = { description = "Stripe payment API", required = true }REDIS_URL = { description = "Redis cache connection", required = true }PORT = { description = "Server port", required = false, default = "3000" }
```

