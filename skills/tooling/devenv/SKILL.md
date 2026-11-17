---
name: devenv
description: Use when a repository has devenv.nix or devenv.yaml, or when running devenv commands. Follow its shell, dependency, SecretSpec, hook, and test-runner conventions.
resources:
  - https://secretspec.dev/llms.txt
  - https://devenv.sh/llms.txt
---

<!-- cspell:ignore devenv secretspec SECRETSPEC -->

# Devenv

This skill applies when the project root contains `devenv.nix` or `devenv.yaml`. In such a project the devenv shell owns the toolchain: run the project's build, test, lint, formatter, and language tools through it, and declare any new project tool in `devenv.nix` rather than installing it on the host.

## Invoking the shell

Use one command shape for every non-interactive (agent or CI) invocation:

```bash
CI=true \
SECRETSPEC_PROVIDER=env \
SECRETSPEC_REASON="<why this command runs>" \
devenv --no-tui shell --quiet -- <command> [args...]
```

| Part                      | Reason                                                                                                                                                 |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--no-tui`                | Keeps the interactive progress interface from holding the terminal. Pass it to every devenv subcommand (`shell`, `test`, `up`, `tasks`).               |
| `--` before the command   | Separates devenv's flags from the command. `-c` is `--clean` (strip the environment), not "run a command".                                             |
| `SECRETSPEC_REASON`       | SecretSpec's default `require_reason = "agents"` policy fails secret access from coding agents until a reason is supplied; the reason is audit-logged. |
| `SECRETSPEC_PROVIDER=env` | Resolves secrets from environment variables so no provider opens an interactive authorization prompt. It overrides every per-secret provider chain.    |
| `CI=true`                 | Matches CI behaviour in `devenv.nix` setups that branch on it.                                                                                         |
| `--quiet`                 | Trims progress output.                                                                                                                                 |

Run the command from the directory that contains `devenv.nix`. When `secretspec.toml` defines several profiles and the task needs a specific one, set `SECRETSPEC_PROFILE=<profile>` (or pass `--secretspec-profile <profile>`); prefer a `ci` profile when it exists, otherwise `default`.

If a command still stops on a SecretSpec prompt, fix the environment (export the required secret variables, set the provider and profile) and re-run; never answer the prompt on the user's behalf.

## Testing and hooks

`devenv --no-tui test` builds the shell, runs every configured git hook, and runs the project's declared tests. Use it, with the variables above, as the project's verification step:

```bash
CI=true SECRETSPEC_PROVIDER=env SECRETSPEC_REASON="run project tests" \
  devenv --no-tui test
```

- Git hooks are declared under `git-hooks.hooks` in `devenv.nix`; `git-hooks.package` defaults to `pkgs.prek`, so keep `prek` as the hook runner and leave the Python `pre-commit` package out of `packages`.
- Hooks see only files Git tracks, so stage new files before relying on a green run.
- Report each failing hook or test by name with its error output.

## Changing the environment

- Add packages to `packages`, language toolchains under `languages.<name>`, scripts under `scripts`, and long-running services under `processes` or `services`. Group each option in one attribute set rather than repeating the key with dotted paths.
- After editing `devenv.nix`, re-enter the shell (`devenv --no-tui shell -- true`) to rebuild; add `--refresh-eval-cache` if a newly declared script is reported missing.
- Look up option names and packages before writing them: the devenv MCP server (`devenv mcp` locally, or `https://mcp.devenv.sh`) searches options and packages, and the vendored docs cover the rest.

## References

- devenv docs: `resources/auto/devenv-llms-full.txt` (single file, includes the CLI reference and every option) or the per-page files indexed by `resources/auto/devenv-llms.txt`.
- SecretSpec docs: `resources/auto/llms-full.txt` and the per-page files indexed by `resources/auto/llms.txt`.

## Completion criteria

- Every project command in the task ran through `devenv --no-tui shell -- <command>` (or `devenv --no-tui test`) with `SECRETSPEC_REASON` set.
- `devenv --no-tui test` exits 0, or each failure is reported by hook or test name.
