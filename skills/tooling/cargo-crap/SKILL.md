---
name: cargo-crap
description: Measure Rust change risk with cargo-crap and coverage data. Use for CRAP scores, cargo-llvm-cov integration, complexity regression gates, CI reports, or badges.
metadata:
  license: MIT
resources:
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/getting-started.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/install.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/reference/cli.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/reference/config.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/reference/exit-codes.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/explanation/crap-metric.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/guides/ci.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/guides/github-action.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/guides/regression-gate.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/guides/pr-comment.md
  - https://raw.githubusercontent.com/minikin/cargo-crap/main/docs/guides/badge.md
---

<!-- cspell:ignore crappy binstall tarpaulin -->

# Cargo CRAP

`cargo-crap` scores every Rust function with the CRAP metric, which combines cyclomatic complexity (CC) with line coverage from an LCOV file. A score is high when code is both complex and poorly tested; at 100% coverage it equals CC.

## Reference files

The vendored upstream docs in `resources/auto/` are the source of truth. Search them before quoting a flag, key, default, or colour:

- Formula and its properties: `resources/auto/minikin-cargo-crap-main-docs-explanation-crap-metric.md`.
- Every flag and the `--missing` policy: `resources/auto/minikin-cargo-crap-main-docs-reference-cli.md`.
- Every `.cargo-crap.toml` key with its range and default: `resources/auto/minikin-cargo-crap-main-docs-reference-config.md`.
- Exit codes: `resources/auto/minikin-cargo-crap-main-docs-reference-exit-codes.md`.
- CI guides, all named `resources/auto/minikin-cargo-crap-main-docs-guides-<topic>.md`: `github-action` (the upstream Action), `ci` (absolute gate and SARIF), `regression-gate`, `pr-comment`, and `badge`.
- Install options: `resources/auto/minikin-cargo-crap-main-docs-install.md`.

## Workflow

1. Install the tools if missing: `cargo binstall cargo-crap` (or `cargo install cargo-crap --locked`) and `cargo install cargo-llvm-cov`. `cargo test` alone produces no coverage data; `cargo llvm-cov` instruments the build and writes LCOV.
2. Generate coverage:

   ```bash
   cargo llvm-cov --workspace --lcov --output-path lcov.info
   ```

3. Score functions:

   ```bash
   cargo crap --workspace --lcov lcov.info
   ```

   Drop `--workspace` for a single crate; use `-p <name>` to select members.

4. Read the table: `✗` marks a score above `--threshold` (default 30), `▲` a score above a third of it, `✓` the rest. Lower a score by testing the uncovered lines (CRAP falls toward CC) or by splitting the function (CC falls). A function whose CC exceeds the threshold stays `✗` at full coverage, so only splitting fixes it. Set `uncovered-hints = true` in `.cargo-crap.toml` to see which line ranges lack tests.

## Configuration

`.cargo-crap.toml` sits at the project root or any parent; CLI flags override it, and unknown keys are rejected. Common keys:

```toml
threshold = 30.0
missing = "pessimistic"          # pessimistic | optimistic | skip
exclude = ["src/generated/**"]   # appends to the defaults: tests/**, benches/**, examples/**
uncovered-hints = true           # config only
try-weight = 1.0                 # config only; what each `?` adds to CC (0 to 100)
```

- `default-excludes` replaces the built-in exclusion list; `exclude` appends to it.
- `try-weight` below `1.0` stops idiomatic `?` error propagation from inflating CC. A baseline recorded under a different weight is not comparable.
- Keep `missing = "pessimistic"` for CI gates, so functions without coverage data score as 0% covered instead of disappearing.

## Gates and reports

- Absolute gate: `cargo crap --workspace --lcov lcov.info --fail-above` exits 1 when any function exceeds the threshold.
- Regression gate, for codebases with existing debt: record `--format json --sort file --output baseline.json` on the default branch, then run `--baseline baseline.json --fail-regression` on pull requests.
- On GitHub, the upstream Action `minikin/cargo-crap@v1` runs coverage, scores the pull request's base commit, comments, and gates in one step. Prefer it over hand-written comment scripts.
- Other outputs come from `--format`: `sarif` for code scanning, `pr-comment` for a Markdown comment body, `shields` for a badge endpoint JSON. Badge colour is brightgreen with no offenders, yellow for 1-5, and red for 6 or more.

Exit code 0 means no requested gate tripped, 1 means a gate tripped after the report was written, and 2 means the run failed (usage, input, analysis, or output error). Wrappers can branch on the code without parsing logs.

For CI workflow templates (SARIF upload, a reusable-workflow caller example), read [resources/manual/ci-recipes.md](resources/manual/ci-recipes.md).

## Where to run it

Run `cargo-crap` in CI or on demand, not in pre-commit hooks: coverage instrumentation rebuilds the workspace with `-C instrument-coverage`, which invalidates the incremental build cache and runs the full test suite on every commit.

## Completion criteria

- `cargo crap` ran against an LCOV file produced by the current code, and its exit code was checked.
- Every function reported `✗` (or regressed against the baseline) was either fixed, explicitly accepted by the user, or listed in the final report with its score, CC, and coverage.
- Any flag, config key, or default you cite matches the vendored reference files.
