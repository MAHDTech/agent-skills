+++
title = "nix-shell"
description = "Run missing ad-hoc CLI utilities in ephemeral nix shell environments when Nix is installed and the project has no devenv or Nix dev shell. Use for one-off diagnostics or conversions."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Ephemeral Nix shells

Use ephemeral Nix shells to run missing command-line utilities and diagnostic tools without polluting the host system or altering repository configurations.

## Environment gate

Before attempting to run tools through Nix, verify that Nix is available on the host:

```bash
command -v nix >/dev/null 2>&1
```

- If Nix is absent, stop and report the missing tool to the user.
- If Nix is present, continue below.

## Scope boundary

This skill covers ad-hoc tools in a project without its own Nix development environment, for example on NixOS in a repository that has no `devenv.nix`, `flake.nix` dev shell, or `shell.nix`. When the project defines such an environment, run tools through that environment's shell instead, and declare any tool the project needs permanently in its configuration.

- Use an ephemeral shell for one-off diagnostics, format conversions, or system inspection (`jq`, `ripgrep`, `tree`, `file`, `hexdump`, `ffmpeg`, `pandoc`, `graphviz`, `curl`) and other temporary tools needed during a debugging or analysis task.
- Keep project build, test, lint, and runtime dependencies (compilers, language runtimes, test frameworks, hook runners) in the project's own toolchain configuration.

## Command syntax

Use the flakes-based `nix shell` syntax:

```bash
nix shell nixpkgs#<package> -c <command> [args...]
```

The legacy `nix-shell -p <package>` depends on a `<nixpkgs>` channel, which pure-flake systems often lack; `nix shell nixpkgs#<package>` resolves through the flake registry instead.

### Single command

Run commands non-interactively using `-c`:

```bash
nix shell nixpkgs#jq -c jq '.version' package.json
```

```bash
nix shell nixpkgs#ripgrep -c rg "TODO" src/
```

### Multiple packages

Provide multiple package attributes separated by spaces when multiple utilities are needed together:

```bash
nix shell nixpkgs#curl nixpkgs#jq -c bash -c "curl -s https://api.github.com/repos/NixOS/nixpkgs | jq .stargazers_count"
```

### Pipelines

When running commands with pipes, redirection, or shell built-ins, pass the pipeline into `bash -c`:

```bash
nix shell nixpkgs#tree nixpkgs#gnused -c bash -c "tree -L 2 | sed 's/foo/bar/'"
```

### Persistent sessions

One-shot execution via `-c` is the default and prevents commands from hanging. When a task requires multiple interactive steps or continuous debugging inside a persistent terminal session, invoke an explicit shell:

```bash
nix shell nixpkgs#<package> -c bash
```

Only use persistent shells within terminals configured to manage persistent state, and terminate or exit the subshell when debugging concludes.

## Resolving package names

When a required binary name does not match the Nixpkgs attribute name:

1. Most standard utilities map directly or predictably (e.g., `rg` maps to `ripgrep`, `dot` maps to `graphviz`, `7z` maps to `p7zip`, `dig` maps to `dnsutils` or `bind`).
2. If unsure of the package name on NixOS or a system with command-not-found installed, check:

   ```bash
   command-not-found <binary>
   ```

3. As a last resort, search nixpkgs:

   ```bash
   nix search nixpkgs <binary> --json
   ```

   Note that `nix search` may be slow if the flake registry index is not already cached locally.

## Unfree licenses

By default, Nix enforces pure evaluation and rejects packages with unfree licenses. If a tool fails with an unfree license error:

```bash
NIXPKGS_ALLOW_UNFREE=1 nix shell --impure nixpkgs#<package> -c <command> [args...]
```

Only pass `--impure` and `NIXPKGS_ALLOW_UNFREE=1` when an unfree package is genuinely required for the task.

## Completion criteria

- The requested utility command exited with the expected status and produced the needed output.
- Repository files and project toolchain configurations remain untouched unless the task explicitly asked to edit them.
- Any persistent subshell opened with `nix shell ... -c bash` was exited cleanly.

