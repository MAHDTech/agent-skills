---
name: nix-shell
description: Run CLI utilities and diagnostic tools in ephemeral Nix shells when a required binary is missing from PATH. Use when a command or tool is not found, when running one-off inspection or data-processing tools (such as jq, ripgrep, ffmpeg, tree, pandoc, or curl) on systems with Nix installed, or when the user asks to run something in a nix shell.
---

# Ephemeral Nix Shells

Use ephemeral Nix shells to run missing command-line utilities and diagnostic tools without polluting the host system or altering repository configurations.

## The Environment Gate

Before attempting to run tools through Nix, verify that Nix is available on the host:

```bash
command -v nix >/dev/null 2>&1
```

- **If Nix is absent:** Fail fast immediately. Do not spend tokens searching for missing binaries or alternative package managers. Report the missing tool clearly to the user.
- **If Nix is present:** Proceed with ephemeral execution as documented below.

## Scope Boundary: Ephemeral Utilities vs Project Dependencies

Distinguish between ad-hoc utilities and project dependencies:

- **Use Ephemeral Nix Shells for:**
  - One-off diagnostics, file format conversions, or system inspection (e.g., `jq`, `ripgrep`, `tree`, `file`, `hexdump`, `ffmpeg`, `pandoc`, `graphviz`, `curl`).
  - Temporary tools needed by the agent during a debugging or analysis task.
- **Do NOT Use Ephemeral Nix Shells for:**
  - Project build, test, lint, or runtime dependencies (e.g., compilers, project language runtimes, test frameworks, pre-commit hooks).
  - If a repository utilizes `devenv`, project-level tools belong in `devenv.nix` (see the devenv skill). Never use `nix shell` as a permanent bypass for repository build environments.

## Modern Command Syntax

Standardize strictly on the modern Flakes-based `nix shell` syntax:

```bash
nix shell nixpkgs#<package> -c <command> [args...]
```

> [!IMPORTANT]
> The legacy `nix-shell -p <package>` command relies on classic Nix channels (`<nixpkgs>`), which are unmaintained or absent on pure-flake systems. Always use `nix shell nixpkgs#<package>`.

### Single Command Execution

Run commands non-interactively using `-c`:

```bash
nix shell nixpkgs#jq -c jq '.version' package.json
```

```bash
nix shell nixpkgs#ripgrep -c rg "TODO" src/
```

### Multiple Packages

Provide multiple package attributes separated by spaces when multiple utilities are needed together:

```bash
nix shell nixpkgs#curl nixpkgs#jq -c bash -c "curl -s https://api.github.com/repos/MAHDTech/agent-skills | jq .stargazers_count"
```

### Compound Commands and Shell Pipelines

When running commands with pipes, redirection, or shell built-ins, pass the pipeline into `bash -c`:

```bash
nix shell nixpkgs#tree nixpkgs#gnused -c bash -c "tree -L 2 | sed 's/foo/bar/'"
```

### Persistent Sessions

One-shot execution via `-c` is the default and prevents commands from hanging. When a task requires multiple interactive steps or continuous debugging inside a persistent terminal session, invoke an explicit shell:

```bash
nix shell nixpkgs#<package> -c bash
```

Only use persistent shells within terminals configured to manage persistent state, and terminate or exit the subshell when debugging concludes.

## Resolving Package Names

When a required binary name does not match the Nixpkgs attribute name:

1. **Model Prior Knowledge:** Most standard utilities map directly or predictably (e.g., `rg` maps to `ripgrep`, `dot` maps to `graphviz`, `7z` maps to `p7zip`, `dig` maps to `dnsutils` or `bind`).
2. **Fast Lookup via `command-not-found`:** If unsure of the package name on NixOS or a system with command-not-found installed, check:

   ```bash
   command-not-found <binary>
   ```

3. **Fallback Search:** As a last resort if prior knowledge fails, query nixpkgs:

   ```bash
   nix search nixpkgs <binary> --json
   ```

   Note that `nix search` may be slow if the flake registry index is not already cached locally.

## Unfree Licenses

By default, Nix enforces pure evaluation and rejects packages with unfree licenses. If a tool fails with an unfree license error:

```bash
NIXPKGS_ALLOW_UNFREE=1 nix shell --impure nixpkgs#<package> -c <command> [args...]
```

Only pass `--impure` and `NIXPKGS_ALLOW_UNFREE=1` when an unfree package is genuinely required for the task.
