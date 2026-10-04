+++
title = "cargo-crap"
description = "Measure and gate Change Risk Anti-Patterns (CRAP) metric across Rust codebases by combining cyclomatic complexity with test coverage. Use when evaluating code maintainability, gating CI against complexity regressions, setting up cargo-llvm-cov with cargo-crap, or generating shields.io badges for Rust projects."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Cargo CRAP Metric and Quality Gates

Workflow for measuring and gating Change Risk Anti-Patterns (CRAP) scores across Rust codebases by correlating cyclomatic complexity with test coverage.

## When to Use This Skill

- Auditing Rust codebase maintainability and identifying high-risk functions before refactoring.
- Gating CI pull requests against complexity regressions and untested branching logic.
- Configuring `cargo-llvm-cov` with `cargo-crap` for automated coverage-complexity analysis.
- Setting up GitHub Code Scanning SARIF alerts or sticky pull request comments for code quality.
- Generating dynamic Shields.io status badges reflecting codebase health.

## Reference Files

Vendored documentation from upstream `cargo-crap` is available under `resources/auto/`:

- `resources/auto/minikin-cargo-crap-main-docs-explanation-crap-metric.md` - Mathematical explanation of the Savoia-Evans formula and risk curves.
- `resources/auto/minikin-cargo-crap-main-docs-guides-regression-gate.md` - Guide for implementing baseline comparison in CI pipelines.
- `resources/auto/minikin-cargo-crap-main-docs-guides-badge.md` - Specifications for Shields.io endpoint integration and color thresholds.
- `resources/auto/minikin-cargo-crap-main-docs-reference-exit-codes.md` - Process exit codes for shell scripting and automation pipelines.

## 1. The CRAP Metric and Formula

The Change Risk Anti-Patterns (CRAP) metric was originally formulated by Alberto Savoia and Bob Evans. It quantifies the risk inherent in modifying a unit of code (in Rust, a function or method) by analyzing two interacting factors:

1. **Cyclomatic Complexity `comp(m)`**: The number of linearly independent execution paths through function `m`.
2. **Test Line Coverage `cov(m)`**: The percentage of executable lines covered by tests (from `0.0` to `100.0`).

### The Savoia-Evans Formula

```text
CRAP(m) = comp(m)^2 * (1 - cov(m)/100)^3 + comp(m)
```

### Risk Dynamics

- **Full Coverage (`cov(m) = 100%`)**: The first term drops to zero, and `CRAP(m) = comp(m)`. Even complex code has a manageable CRAP score if thoroughly tested.
- **Zero Coverage (`cov(m) = 0%`)**: The formula collapses to `CRAP(m) = comp(m)^2 + comp(m)`. As complexity increases, risk grows quadratically. A function with complexity 15 and no tests yields a CRAP score of `15^2 + 15 = 240`.
- **Cubic Penalty on Uncovered Code**: The factor `(1 - cov(m)/100)^3` acts as an aggressive penalty. Minor drops in test coverage on highly branching functions cause rapid score inflation.
- **Standard Threshold**: A CRAP score of `30.0` is the standard industry threshold. Any function scoring above 30 is classified as high-risk and excessively difficult to maintain or refactor safely without defects.

## 2. Prerequisites and Installation

Standard `cargo test` is not sufficient for CRAP analysis. While `cargo test` executes test binaries and reports pass/fail outcomes, it does not instrument compiled machine code or produce line-level execution trace files.

`cargo-llvm-cov` is a mandatory coverage engine prerequisite. It leverages LLVM source-based code coverage flags (`-C instrument-coverage`) to generate standardized LCOV trace files (`lcov.info`) that `cargo-crap` ingests alongside Rust abstract syntax trees (ASTs).

### Pinning in Cargo.toml

The recommended Cargo-native pattern is pinning `cargo-crap` directly in your workspace or crate manifests:

- In root `Cargo.toml` under `[workspace.dependencies]`:

```toml
cargo-crap = "0.6.1"
```

- In crate `Cargo.toml` (such as `crates/ask-cli/Cargo.toml`) under `[dev-dependencies]`:

```toml
cargo-crap = { workspace = true, default-features = false }
```

Pinning the tool in `Cargo.toml` ensures that:

- The exact CLI tool version is tracked in `Cargo.lock` by version control.
- Automated dependency managers (such as Dependabot or Renovate) detect and propose updates automatically.
- Continuous integration pipelines (such as `tars-cloud/actions` reusable workflows) automatically detect and resolve `cargo-crap` and its exact pinned version from `Cargo.lock`.

### Installing the Tooling

For coverage trace generation, install `cargo-llvm-cov`:

```bash
cargo binstall cargo-llvm-cov || cargo install cargo-llvm-cov
```

For `cargo-crap`, use one of the following local installation workflows:

- **Pre-built binary (fastest)**:

```bash
cargo binstall cargo-crap
```

- **From source matching locked dependencies**:

```bash
cargo install --locked cargo-crap
```

## 3. Core Workflow

CRAP analysis runs in two distinct stages: generating the coverage profile, then analyzing the code with `cargo crap`.

### Step 1: Generate Coverage Trace

Run `cargo llvm-cov` to compile the codebase with coverage instrumentation, run the test suite, and output an LCOV trace:

```bash
# Single crate
cargo llvm-cov --lcov --output-path lcov.info

# Entire workspace
cargo llvm-cov --workspace --lcov --output-path lcov.info
```

### Step 2: Analyze with `cargo crap`

Run `cargo crap` pointing to the generated LCOV file:

```bash
# Analyze single crate
cargo crap --lcov lcov.info

# Analyze entire workspace
cargo crap --workspace --lcov lcov.info
```

### Interpreting Terminal Results

The terminal output renders a table of evaluated functions with their Cyclomatic Complexity (CC), Line Coverage percentage, and final CRAP score, accompanied by status glyphs:

- `✗` (Failing): Score exceeds the threshold (default `> 30.0`). The function requires immediate refactoring (splitting into smaller units) or additional targeted test cases.
- `▲` (Warning): Score exceeds one-third of the threshold (`> 10.0`). Indicates moderate risk; monitor closely as complexity or branch count grows.
- `✓` (Passing): Score is within acceptable limits (`<= 10.0`). The function exhibits a healthy balance of complexity and test coverage.

## 4. Configuration (`.cargo-crap.toml`)

Create a `.cargo-crap.toml` configuration file in the crate or repository root to customize thresholds, path filters, and language-specific weighting.

```toml
# Threshold above which a function is flagged as crappy
# Default is 30.0; use 15.0 for strict quality requirements
threshold = 30.0

# Weight for the try operator '?' (range: 0.0 to 1.0)
# Setting this to 0.2 prevents idiomatic error propagation from spiking complexity
try-weight = 0.2

# Show uncovered line ranges in terminal reports for failing functions
uncovered-hints = true

# Additional file patterns to exclude from analysis
exclude = [
    "src/generated/**",
    "src/proto/**",
]
```

### Excluding Files and Patterns

By default, `cargo-crap` automatically applies built-in exclusions for standard test and benchmark directories (`tests/**`, `benches/**`, `examples/**`).

- **`exclude`**: Appends additional glob patterns to the defaults (such as generated code or protocol buffers).
- **`default-excludes`**: Optional list of glob patterns that replaces the built-in defaults if specified (e.g. `default-excludes = ["benches/**", "examples/**"]`, or `default-excludes = []` to disable them), whereas `exclude` appends to the defaults.

### The `try-weight` Setting

In Rust, the question mark operator `?` is standard practice for concise error propagation. By default, every `?` introduces an implicit branching point, which increments cyclomatic complexity by 1. In error-heavy functions, this can artificially inflate complexity without representing genuine business logic risk. Adjust `try-weight` (such as `0.2` or `0.0`) to avoid penalizing idiomatic error propagation.

## 5. CI and Gating Strategies

Integrate `cargo-crap` into continuous integration pipelines to prevent quality regressions from merging into production branches.

### Reusable GitHub Actions Workflow (Recommended)

The recommended CI setup uses the official reusable workflow from `tars-cloud/actions`. It automates the complete test, coverage, baseline comparison, and reporting pipeline, separating analysis execution from quality gate evaluation.

Standard consumer caller workflow (`.github/workflows/cargo-crap.yaml`):

```yaml
name: Cargo CRAP

on:
  pull_request:
    branches:
      - trunk
      - main
  push:
    branches:
      - trunk
      - main
  workflow_dispatch: {}

permissions:
  contents: read
  actions: read
  pull-requests: write

jobs:
  cargo-crap:
    name: Cargo CRAP
    uses: tars-cloud/actions/.github/workflows/consumer-cargo-crap.yaml@v3
    with:
      baseline-branch: trunk
      coverage-tool: llvm-cov
      post-comment: true
      update-records-pr: true
    secrets:
      app-id: ${{ secrets.APP_ID }}
      app-private-key: ${{ secrets.APP_PRIVATE_KEY }}
```

#### Quality Verdict and Reporting Separation

The reusable workflow executes analysis and artifact generation before checking gating thresholds. Because quality verdicts are decoupled from publication steps, sticky PR comments, job summaries, and SARIF security uploads are always posted, even if the workflow run subsequently fails the quality gate.

#### PR Comment and Summary Severity Levels

The reusable workflow categorizes findings into three explicit severity levels for PR sticky comments and job summaries:

- 🟢 **INFO:** All functions are within the threshold (default 30) and no score regressions detected; CI passes.
- 🟠 **WARNING:** Scores regressed beyond `epsilon` (default 0.01), but all functions remain at or below the threshold; CI passes with warning annotation.
- 🔴 **ERROR:** Any function strictly exceeds the threshold; CI fails with error annotation.

#### Automated Baseline and Badge Maintenance

Enabling `update-records-pr: true` configures automated baseline management on the designated baseline branch (e.g., `trunk`). When changes land on trunk, the workflow creates or updates a dedicated bot-owned `crap/next` pull request recording `.github/crap/baseline.json` and `.github/badges/crap-badge.json`, keeping metrics synchronized without manual intervention.

### Absolute Quality Gate

Enforce an absolute ceiling across the entire codebase. If any function exceeds the threshold, the build fails:

```bash
cargo crap --workspace --lcov lcov.info --fail-above --threshold 30
```

### Pull Request Regression Gate

An absolute gate can be difficult to adopt in large existing codebases with technical debt. Instead, use a baseline regression gate that only fails if a PR introduces new crappy functions or worsens existing scores.

- **Trunk Workflow (Main Branch)**: Generate and save `baseline.json` on trunk builds:

```bash
cargo crap --workspace --lcov lcov.info --format json --sort file --output baseline.json
```

- **Pull Request Workflow**: Fetch the trunk baseline and compare against the PR changes:

```bash
cargo crap --workspace --lcov lcov.info --baseline baseline.json --fail-regression
```

### GitHub Code Scanning (SARIF)

Export findings in SARIF format for integration into the GitHub Security Code Scanning dashboard:

```bash
cargo crap --workspace --lcov lcov.info --format sarif --output crap.sarif
```

Upload the artifact in your GitHub Actions workflow:

```yaml
- name: Upload CRAP SARIF Report
  uses: github/codeql-action/upload-sarif@v3
  if: always()
  with:
    sarif_file: crap.sarif
    category: cargo-crap
```

### Pull Request Sticky Comment

Generate a formatted Markdown summary and post it as a sticky PR comment using `actions/github-script`:

```bash
cargo crap --workspace --lcov lcov.info --format pr-comment --output crap-comment.md
```

GitHub Actions step example:

```yaml
- name: Post PR CRAP Comment
  uses: actions/github-script@v7
  if: github.event_name == 'pull_request'
  with:
    script: |
      const fs = require('fs');
      const body = fs.readFileSync('crap-comment.md', 'utf8');
      const header = '<!-- cargo-crap-comment -->';
      const fullBody = `${header}
${body}`;

      const { data: comments } = await github.rest.issues.listComments({
        owner: context.repo.owner,
        repo: context.repo.repo,
        issue_number: context.issue.number,
      });

      const botComment = comments.find(c => c.body.includes(header));
      if (botComment) {
        await github.rest.issues.updateComment({
          owner: context.repo.owner,
          repo: context.repo.repo,
          comment_id: botComment.id,
          body: fullBody,
        });
      } else {
        await github.rest.issues.createComment({
          owner: context.repo.owner,
          repo: context.repo.repo,
          issue_number: context.issue.number,
          body: fullBody,
        });
      }
```

## 6. Shields.io Badge Generation

Generate an endpoint JSON file for displaying dynamic CRAP status badges on repository readmes:

```bash
cargo crap --workspace --lcov lcov.info --format shields --output crap-badge.json
```

Publish `crap-badge.json` via the reusable workflow (which records it at `.github/badges/crap-badge.json` on trunk) or to GitHub Pages / dedicated branch, then link the badge in your `README.md`.

Official Shields.io endpoint badge URL format using URL-encoded query parameters:

```markdown
[![CRAP](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2F<owner>%2F<repo>%2Ftrunk%2F.github%2Fbadges%2Fcrap-badge.json&style=flat-square)](https://github.com/<owner>/<repo>/blob/trunk/.github/badges/crap-badge.json)
```

### Badge Status and Colors

The badge colors reflect the count of flagged functions rather than a single repository average:

- `brightgreen`: 0 functions exceed threshold (clean).
- `orange`: 1-5 functions exceed threshold (minor debt).
- `red`: 6+ functions exceed threshold (significant debt).

## 7. Traps and Operational Best Practices

### Why Not Run in Git Pre-Commit Hooks

Do not add full `cargo-crap` execution to local pre-commit hooks (such as `prek` or `.git/hooks/pre-commit`):

- **Build Cache Invalidation**: Source-based coverage (`cargo-llvm-cov`) recompiles code with `-C instrument-coverage`. This pollutes and invalidates local incremental compilation artifacts, forcing subsequent `cargo check` and `cargo test` runs to rebuild from scratch.
- **Commit Latency**: Compiling all workspace targets with coverage instrumentation and executing the full test suite can take minutes on large codebases, stalling commit velocity.
- **Recommended Practice**: Run `cargo-crap` in CI pull request checks, or provide an on-demand ad-hoc devenv shell task for developers before pushing branches.

### Path Matching and Missing Coverage Policy

When functions appear in the AST but have no matching entries in the LCOV file, use `--missing` to control behavior:

- `--missing pessimistic` (Recommended for CI): Assumes unreferenced functions have `0%` coverage. This prevents untested modules from evading analysis.
- `--missing optimistic`: Assumes unreferenced functions are fully covered or ignores them. Use only during initial rollouts or when analyzing partial workspaces.

### Process Exit Codes

For automation and custom shell scripts, `cargo crap` returns deterministic exit codes:

- `0` (Success): Analysis completed cleanly; no functions exceeded the threshold and no regressions were detected.
- `1` (Threshold Tripped): Analysis completed, but one or more functions exceeded the failure threshold (`--fail-above`) or regressed relative to baseline (`--fail-regression`).
- `2` (System Error): Analysis aborted due to invalid arguments, missing LCOV files, or syntax parsing failures.

