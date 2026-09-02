# ADR 0001: Rewrite Developer and Dashboard Tooling in Rust

## Context

The repository currently uses Bun and TypeScript for internal developer tooling (`bin/skills` and `bin/dashboard`).
These scripts handle skill linting, metadata synchronization, local symlink installation, resource downloading, and static dashboard compilation.

Downstream consumers install skills using the external `skills.sh` CLI (`npx skills add MAHDTech/agent-skills`). `skills.sh` directly clones or inspects the git repository and parses the markdown files (`skills/**/SKILL.md`) and `skills.sh.json`. Downstream consumers do not execute any internal repository scripts.

The TypeScript scripts incur runtime dependency overhead (Node/Bun packages, tsconfig, eslint, etc.) and lack compile-time guarantees for critical tasks like markdown/frontmatter verification and atomic resource downloading.

## Decision

We will migrate all internal developer tooling from Bun/TypeScript to a unified Cargo workspace in Rust:

1. **Independent Tooling:** The repository's internal tooling is completely decoupled from downstream consumer installation. Rewriting the CLI tools in Rust will not affect `skills.sh` or end-users.
2. **Cargo Workspace:** A single Cargo workspace with `crates/` hosting:
   - `skills-core`: Shared domain models, YAML frontmatter parsing, markdown linting, sync logic, and resource downloader.
   - `skills-cli`: Binary CLI for `skills` command (`--action lint|sync|install|uninstall|download-resources|clean-resources`).
   - `dashboard-cli`: Binary CLI for `dashboard` command (`--action build|serve|css|lint`).
3. **Devenv & Toolchain:**
   - Manage the Rust toolchain via `rust-toolchain.toml` with `languages.rust.toolchainFile` in `devenv.nix`.
   - Enable `mold` linker for fast incremental compilation on Linux.
   - Configure cargo pre-commit hooks (`clippy`, `rustfmt`, `cargo-check`) via `git-hooks`.
4. **CI & Automation:**
   - Update `.github/dependabot.yml` for Cargo updates.
   - Update `.github/workflows/sec-codeql.yaml` to scan Rust code and remove JS/TS scans.
   - Update `.github/workflows/ci.yml` and `enterTest` in `devenv.nix` to run `cargo nextest` / `cargo test`.
5. **Decommission Node/Bun:**
   - Remove `package.json`, `bun.lock`, `bunfig.toml`, `tsconfig.json`, `eslint.config.mjs`, and `bin/`.

## Consequences

### Positive

- High-performance, single-binary execution with no Node/Bun runtime requirements for developers using the built binaries.
- Compile-time safety and strong type guarantees for parsing, file I/O, and resource fetching.
- Consolidated linting and formatting via standard Rust tooling (`clippy`, `rustfmt`).
- Native integration with devenv and nix development shells.

### Negative / Tradeoffs

- Contributors modifying internal tooling require a Rust toolchain (provided automatically by devenv).
- Initial compile time for the Rust workspace when built from clean state.
