---
name: devenv
description: Strict guidelines for using devenv for shell and dependency management. Use when a repo contains a devenv.nix or devenv.yaml file, or when running commands in a devenv environment.
resources:
  - https://secretspec.dev/llms.txt
  - https://devenv.sh/llms.txt
---

# Devenv Integration

When working in a repository that utilizes `devenv`, you must strictly adhere to the following rules for environment consistency:

- **Devenv is Mandatory:** For projects containing a `devenv.nix` and/or `devenv.yaml` file, strict use of `devenv` for dependency and shell environment management is required.
- **DO NOT Run Bare Commands:** ALL standard CLI operations MUST run via the devenv shell. Furthermore, you must **always** provide a reason for the execution via the `SECRETSPEC_REASON` environment variable to satisfy SecretSpec's audit logging for AI agents. To do this use:

```bash
SECRETSPEC_REASON="<reason>" devenv shell -- <command>
```

- **Root Execution Only:** You must ONLY run `devenv shell` commands from the root of the repository where the `devenv.nix` file is located.
  - Never run it from a sub-directory.
  - Looking for the `devenv.nix` is how you know you are in the right place.
  - Correct: `SECRETSPEC_REASON="checking code" devenv shell -- cargo check`
- **Dash-Dash is Mandatory (`--`):** NEVER use `devenv shell -c <command>`. This is an incredibly common mistake. The `-c` argument is used for a "clean" environment and using it gives unexpected results.
  - Use `SECRETSPEC_REASON="<reason>" devenv shell -- <command>` - this is the right way.

## Pre-commit Hooks and Testing

Devenv gives us the ability to run tests and linters seamlessly. The project-level hooks are run via `prek` (see the [prek](../prek/SKILL.md) skill).
- `SECRETSPEC_REASON="running tests" devenv test`: This triggers all pre-commit hooks (managed by `prek`) and other defined tests and is **mandatory** as part of testing.

**Verification Hook Run (ALL Repositories):**
You must verify that all hooks (Prettier, CSpell, etc.) pass successfully. Check if `secretspec.toml` exists in the repository root:
- **If it exists:**
  1. If `[profiles.ci]` is defined, set `SECRETSPEC_ENV="ci"`.
  2. Else if `[profiles.default]` is defined, set `SECRETSPEC_ENV="default"`.
  3. Else, use the first profile name defined under `[profiles.<profile_name>]`.
  4. Run the validation command: `SECRETSPEC_PROVIDER=env SECRETSPEC_ENV="<env>" SECRETSPEC_REASON="<context-specific-reason>" devenv shell -- prek run -a`
- **If it does not exist:**
  1. Run the validation command without environment prefix: `SECRETSPEC_REASON="<context-specific-reason>" devenv shell -- prek run -a`

If any hook fails, report the failures back.

## SecretSpec Integration

Devenv integrates with SecretSpec, a tool that manages secret resolution and auditing.
- **Audit Logging and Agent Accountability:** Coding agents are required by default to provide a human-readable reason whenever accessing secrets. This is enforced by SecretSpec's `require_reason = "agents"` policy.
- **Always Provide a Reason:** To ensure your commands never fail due to unauthorized secret access, you MUST supply a `SECRETSPEC_REASON` environment variable for **every** `devenv shell` invocation.

## MCP Servers & Skills

- **Check all available MCP servers** for additional knowledge and skills.
- Prioritize the use of the Devenv MCP server when it is available:
  - It can be accessed via `devenv mcp` to run locally.
  - Or via the remote URL: `https://mcp.devenv.sh` (Note: requires active internet connection).
- When no MCP server is available, use the local [Devenv LLMs Backup Text](resources/auto/secretspec-llms-full.txt) as a backup.

## References

- [Devenv Documentation](resources/auto/devenv-getting-started-index.md)
- [Devenv MCP Server](https://mcp.devenv.sh) (Online-Only)
- [Devenv LLMs Backup Text](resources/auto/secretspec-llms-full.txt)
