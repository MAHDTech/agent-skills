# Skills Dashboard

A live catalog of all available agent skills is published and hosted at <https://skills.mahdtech.com>.

It provides a searchable, browseable web interface for exploring the skill catalog, checking requirements, and discovering triggers.

## Local Development

Start the live preview from the repository root:

```bash
devenv up
```

This serves the site at <http://127.0.0.1:3000> and runs three processes:

| Process           | What it does                                                              |
| ----------------- | ------------------------------------------------------------------------- |
| `zola`            | `zola serve` with live reload for templates, content, and static files.   |
| `tailwind`        | Rebuilds the stylesheet whenever a template, script, or CSS file changes. |
| `dashboard-watch` | Regenerates dashboard content and the search index when a skill changes.  |

The search index is built from a scratch copy of the site, never `dashboard/public`, so it does not disturb the running server.

The individual commands are also available inside the `devenv` shell:

```bash
# Build the static site and Zola content
devenv --no-tui shell -- ask dashboard build

# Regenerate dashboard content and the development search index once
devenv --no-tui shell -- ask dashboard index

# Build and serve the site without devenv processes (defaults to http://localhost:3000)
devenv --no-tui shell -- ask dashboard serve

# Compile Tailwind CSS stylesheet directly
devenv --no-tui shell -- ask dashboard css

# Verify dashboard content matches skill source files
devenv --no-tui shell -- ask dashboard lint
```

## Design System

The dashboard shares its design system with the `mahdtech.com` website: the same palette, fonts (Inter and JetBrains Mono), surface classes, and component names. The theme follows the system colour scheme, and the toggle in the top right pins light or dark.

| File                                                 | Holds                                                   |
| ---------------------------------------------------- | ------------------------------------------------------- |
| `dashboard/static/styles/themes.css`                 | Light and dark palette, one `light-dark()` per token.   |
| `dashboard/static/styles/foundation.css`             | Tailwind tokens, fonts, base rules, shared surfaces.    |
| `dashboard/css/input.css`                            | Dashboard-only styles: Markdown prose, sidebar, search. |
| `dashboard/themes/tars-dashboards/templates/macros/` | Shared Tera components (`ui.*`).                        |
| `dashboard/themes/tars-dashboards/static/js/site.js` | Theme toggle, search, copy buttons, sidebar, diagrams.  |

Skills and their reference files render through one shared component, `ui.doc`. Add new page furniture as a component in `macros/ui.html` instead of repeating markup across templates.

## Validation

Run the full local CI check before pushing changes, including dependency updates:

```bash
devenv --no-tui test
```

This runs the configured hooks, all workspace tests through `scripts/run-tests.sh`, and the dashboard site test in `scripts/test-dashboard.sh`.
The site test builds the dashboard into a scratch directory and checks the rendered pages, shared chrome, assets, stylesheet, and search index. Run it alone with `dashboard-test`.
CI uses the same test wrappers with the committed Cargo lockfile.

The `Tera Templates` hook rejects quoted component arguments such as `arg="{value}"`, which render as literal text instead of failing the build. Write `arg={value}`.

Commit hooks run Cargo Check and Clippy across all workspace crates and test targets
when Rust source, Cargo manifests, lockfiles, or toolchain configuration change.
These checks compile tests; `devenv --no-tui test` executes them.

The theme uses Zola 0.23 components. Documentation is rendered as literal Markdown
so template examples such as GitHub Actions expressions remain readable.
