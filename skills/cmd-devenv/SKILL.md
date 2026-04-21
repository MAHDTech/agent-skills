---
name: cmd-devenv
description: Strict guidelines for using devenv for shell and dependency management
custom:
  triggers:
    - devenv
    - developer environment
    - when using devenv
    - when running commands in a devenv environment
    - when you see a devenv.nix or devenv.yaml file
---

# Devenv Integration

When working in a repository that utilizes `devenv`, you must strictly adhere to the following rules for environment consistency:

- **Devenv is Mandatory:** For projects containing a `devenv.nix` and/or `devenv.yaml` file, strict use of `devenv` for dependency and shell environment management is required.
- **DO NOT Run Bare Commands:** ALL standard CLI operations MUST run via the devenv shell. To do this use

```bash
devenv shell -- <command>
```

- **Root Execution Only:** You must ONLY run `devenv shell` commands from the root of the repository where the `devenv.nix` file is located.
  - Never run it from a sub-directory.
  - Looking for the `devenv.nix` is how you know you are in the right place.
  - Correct: `devenv shell -- cargo check`
- **Dash-Dash is Mandatory (`--`):** NEVER use `devenv shell -c <command>`. This is an incredibly common mistake. The `-c` argument is used for a "clean" environment and using it gives unexpected results.
  - Use `devenv shell -- <command>` - this is the right way.

## Pre-commit Hooks and Testing

Devenv gives us the ability to run tests and linters seamlessly:
- `devenv test`: This triggers all pre-commit hooks and other defined tests and is **mandatory** as part of testing.
- `devenv shell -- prek run -a`: As a fallback, this can also be used to just run the pre-commit hooks.

## MCP Servers & Skills

- **Check all available MCP servers** for additional knowledge and skills.
- Prioritize the use of the Devenv MCP server when it is available:
  - It can be accessed via `devenv mcp` to run locally.
  - Or via the remote URL: `https://mcp.devenv.sh`
- When no MCP server is available, use the LLMs text file at `https://devenv.sh/llms.txt` as a backup. This text file contains all the reference URLs to get the right information.

## References

- [Devenv Documentation](https://devenv.sh)
- [Devenv MCP Server](https://mcp.devenv.sh)
- [Devenv LLMs Backup Text](https://devenv.sh/llms.txt)
