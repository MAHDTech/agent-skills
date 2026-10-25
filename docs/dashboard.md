# Skills Dashboard

A live catalog of all available agent skills is published and hosted at <https://skills.mahdtech.com>.

It provides a searchable, browseable web interface for exploring the skill catalog, checking requirements, and discovering triggers.

## Local Development

You can build and serve the dashboard locally to preview changes or run the site offline.

All commands must be run within the `devenv` shell environment:

```bash
# Build the static site and Zola content
devenv --no-tui shell -- ask dashboard build

# Build and serve the site with live reloading (defaults to http://localhost:3000)
devenv --no-tui shell -- ask dashboard serve

# Compile Tailwind CSS stylesheet directly
devenv --no-tui shell -- ask dashboard css

# Verify dashboard content matches skill source files
devenv --no-tui shell -- ask dashboard lint
```

## Validation

Run the full local CI check before pushing changes, including dependency updates:

```bash
devenv --no-tui test
```

This runs the configured hooks and all workspace tests through `scripts/run-tests.sh`.
The CLI integration tests build the dashboard, including its templates and Pagefind index.
CI uses the same test wrapper with the committed Cargo lockfile.

Commit hooks run Cargo Check and Clippy across all workspace crates and test targets
when Rust source, Cargo manifests, lockfiles, or toolchain configuration change.
These checks compile tests; `devenv --no-tui test` executes them.

The theme uses Zola 0.23 components. Documentation is rendered as literal Markdown
so template examples such as GitHub Actions expressions remain readable.
