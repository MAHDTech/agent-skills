# cargo-crap CI recipes

Templates that complement the upstream guides in `resources/auto/`. Check flag names against `resources/auto/minikin-cargo-crap-main-docs-reference-cli.md` before copying.

## Upstream GitHub Action

The smallest GitHub setup. It runs coverage, scores the pull request's base commit for the comparison, posts a sticky comment, and gates on regressions:

```yaml
name: CRAP

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read
  pull-requests: write

jobs:
  crap:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: minikin/cargo-crap@v1
```

Inputs, fork behaviour, and security notes are in `resources/auto/minikin-cargo-crap-main-docs-guides-github-action.md`.

## SARIF upload to code scanning

The job needs `security-events: write`. `if: always()` uploads findings even when an earlier gate step failed:

```yaml
- run: cargo llvm-cov --workspace --lcov --output-path lcov.info
- run: cargo crap --workspace --lcov lcov.info --format sarif --output crap.sarif
- uses: github/codeql-action/upload-sarif@v3
  if: always()
  with:
    sarif_file: crap.sarif
    category: cargo-crap
```

## Calling an organisation's reusable workflow

Some organisations wrap cargo-crap in a reusable workflow that also records the baseline and badge on the default branch. This caller for `tars-cloud/actions` is one example; the input names belong to that workflow, so read the target workflow's `workflow_call` inputs before adapting it:

```yaml
name: Cargo CRAP

on:
  pull_request:
    branches: [main]
  push:
    branches: [main]
  workflow_dispatch: {}

permissions:
  contents: write
  actions: read
  pull-requests: write

jobs:
  cargo-crap:
    uses: tars-cloud/actions/.github/workflows/cargo-crap.yaml@v4
    with:
      post-comment: true
```

In that workflow, `post-comment` maintains a sticky pull request comment and `publish-records` (enabled by default) publishes `baseline.json` and `crap-badge.json` directly to an orphan `crap` branch after each default-branch run using `GITHUB_TOKEN`. App credentials (`client-id` and `app-private-key`) are only needed when repository rules protect the `crap` branch against `GITHUB_TOKEN` pushes.
