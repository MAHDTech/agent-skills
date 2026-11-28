---
applyTo: "crates/**/*.rs,**/Cargo.toml,Cargo.lock"
---

# Rust Workspace Rules

## Crate Boundaries

- `crates/skills-core`: Core domain logic, parsing, linting, vendor sync, and catalog generation.
- `crates/skills-tui`: Interactive Ratatui terminal UI (`ask tui`).
- `crates/ask-cli`: Clap CLI entry point (`ask`).

## Lints & Safety

- Workspace lints in `Cargo.toml` enforce `unsafe_code = "deny"` and `unused_must_use = "deny"`. Never introduce `unsafe` blocks or ignore `Result` values.
- `clippy::all`, `clippy::pedantic`, `missing_debug_implementations`, `unreachable_pub`, and `clippy::doc_markdown` are enabled as warnings and checked in hooks/CI.
- Use `thiserror` for structured domain errors and `tracing` for diagnostics instead of ad-hoc `println!` logging in library code.
- Keep function complexity within `cargo-crap` thresholds (avoid functions with CRAP score above 30).

## Testing & Verification

- Keep `Cargo.lock` updated when changing dependencies; CI and hooks run Cargo with `--locked`.
- Run tests through `devenv --no-tui test` (which invokes `scripts/run-tests.sh`: `cargo nextest run --workspace --all-targets --locked` and `cargo test --workspace --doc --locked`).
