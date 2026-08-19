+++
title = "devenv"
description = "Strict guidelines for using devenv for shell and dependency management. Use when a repo contains a devenv.nix or devenv.yaml file, or when running commands in a devenv environment."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Devenv Integration

When working in a repository that utilizes `devenv`, you must strictly adhere to the following rules for environment consistency:

- **Devenv is Mandatory:** For projects containing a `devenv.nix` and/or `devenv.yaml` file, strict use of `devenv` for dependency and shell environment management is required.
- **DO NOT Run Bare Commands:** ALL standard CLI operations MUST run via the devenv shell. Furthermore, you must **always** provide a reason for the execution via the `SECRETSPEC_REASON` environment variable to satisfy SecretSpec's audit logging for AI agents. To do this use:

```bash
SECRETSPEC_REASON="<reason>" devenv --no-tui shell -- <command>
```

- **NO TUI is Mandatory (`--no-tui`):** ALWAYS include the `--no-tui` flag when running `devenv` commands (e.g. `devenv --no-tui shell ...`, `devenv --no-tui test`) to disable the interactive terminal interface so AI Agents do not get stuck!
- **Root Execution Only:** You must ONLY run `devenv shell` commands from the root of the repository where the `devenv.nix` file is located.
  - Never run it from a sub-directory.
  - Looking for the `devenv.nix` is how you know you are in the right place.
  - Correct: `SECRETSPEC_REASON="checking code" devenv --no-tui shell -- cargo check`
- **Dash-Dash is Mandatory (`--`):** NEVER use `devenv shell -c <command>`. This is an incredibly common mistake. The `-c` argument is used for a "clean" environment and using it gives unexpected results.
  - Use `SECRETSPEC_REASON="<reason>" devenv --no-tui shell -- <command>` - this is the right way.

## Non-Interactive Agent and CI Invocations

Bare `devenv shell` (or `devenv --no-tui shell` without a non-interactive secrets provider) can **hang or fail on an authorization prompt** in agent sessions, headless CI, and backlog gates. That looks like a red test gate when the suite never started.

For **every** agent-driven or CI-driven `devenv` invocation, use this shape:

```bash
CI=true \
SECRETSPEC_PROVIDER=env \
SECRETSPEC_REASON="<reason>" \
devenv --no-tui shell --quiet -- \
  <command>
```

| Variable / flag | Why |
| --------------- | --- |
| `CI=true` | Many `devenv.nix` setups change install/auto-deps behaviour under CI; agents should match CI. |
| `SECRETSPEC_PROVIDER=env` | Resolve secrets from the environment only - **no interactive provider prompt**. |
| `SECRETSPEC_REASON=…` | Required for SecretSpec agent audit logging. |
| `--no-tui` | Never open the interactive TUI (hangs agents). |
| `--quiet` | Prefer under CI/agents to reduce noise; still always use `--no-tui`. |

**Profile selection** when `secretspec.toml` exists (same rules as hook runs below): set `SECRETSPEC_ENV` to `ci` if `[profiles.ci]` exists, else `default`, else the first defined profile.

**Never** run bare `devenv shell` without `--no-tui` in an agent context. **Never** dismiss or ignore a secretspec authorization prompt - fix the env (`SECRETSPEC_PROVIDER=env` + required vars) instead.

Other skills (for example the tars-backlog pipeline) must **not** restate these flags. If a project has `devenv.nix` or `devenv.yaml`, follow **this** skill when building any command that enters the devenv shell. Backlog prepare freezes the resulting opaque command strings into `.tars/run.env`; implementers only execute those strings.

## Pre-commit Hooks and Testing

> [!CAUTION]
> **PRE-COMMIT VS PREK IN DEVENV.NIX:**
> - **Terminology:** "pre-commit" refers strictly to the Git lifecycle hook stage (e.g. `.pre-commit-config.yaml`).
> - **CLI Tool Deprecation:** The standalone `pre-commit` Python CLI tool and package are **DEPRECATED** and replaced by `prek`.
> - **DO NOT add `pkgs.pre-commit` or `pre-commit` package/input to `devenv.nix`.** If a hook runner package is needed, use `pkgs.prek` or `git-hooks.git-hooks`.
> - ALWAYS use `prek` to run or manage pre-commit hooks (e.g., `prek run -a`).

Devenv gives us the ability to run tests and linters seamlessly. The project-level hooks are run via `prek` (see the [prek](@/skills/tooling/prek/_index.md) skill).
- `SECRETSPEC_REASON="running tests" devenv --no-tui test`: This triggers all pre-commit hooks (managed by `prek`) and other defined tests and is **mandatory** as part of testing. Prefer the non-interactive prefix from **Non-Interactive Agent and CI Invocations** when an agent or gate runs this.

**Verification Hook Run (ALL Repositories):**
You must verify that all hooks (Prettier, CSpell, etc.) pass successfully. Check if `secretspec.toml` exists in the repository root:
- **If it exists:**
  1. If `[profiles.ci]` is defined, set `SECRETSPEC_ENV="ci"`.
  2. Else if `[profiles.default]` is defined, set `SECRETSPEC_ENV="default"`.
  3. Else, use the first profile name defined under `[profiles.<profile_name>]`.
  4. Run the validation command with the non-interactive prefix:

     ```bash
     CI=true SECRETSPEC_PROVIDER=env SECRETSPEC_ENV="<env>" \
       SECRETSPEC_REASON="<context-specific-reason>" \
       devenv --no-tui shell --quiet -- prek run -a
     ```

- **If it does not exist:**
  1. Run:

     ```bash
     CI=true SECRETSPEC_REASON="<context-specific-reason>" \
       devenv --no-tui shell --quiet -- prek run -a
     ```

If any hook fails, report the failures back.

## SecretSpec Integration

Devenv integrates with SecretSpec, a tool that manages secret resolution and auditing.
- **Audit Logging and Agent Accountability:** Coding agents are required by default to provide a human-readable reason whenever accessing secrets. This is enforced by SecretSpec's `require_reason = "agents"` policy.
- **Always Provide a Reason:** To ensure your commands never fail due to unauthorized secret access, you MUST supply a `SECRETSPEC_REASON` environment variable for **every** `devenv --no-tui shell` invocation.
- **Non-Interactive Provider:** In agent, CI, and backlog-gate contexts, always set `SECRETSPEC_PROVIDER=env` so SecretSpec does not open an interactive authorization UI. Interactive providers are for humans at a terminal only.

## MCP Servers & Skills

- **Check all available MCP servers** for additional knowledge and skills.
- Prioritize the use of the Devenv MCP server when it is available:
  - It can be accessed via `devenv mcp` to run locally.
  - Or via the remote URL: `https://mcp.devenv.sh` (Note: requires active internet connection).

## References

- [Devenv Documentation](@/skills/tooling/devenv/resources/auto/devenv-getting-started-index.md)
- [Devenv MCP Server](https://mcp.devenv.sh) (Online-Only)

