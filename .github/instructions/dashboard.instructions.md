---
applyTo: "dashboard/**"
---

# Dashboard & Theme Rules

## Generated vs Hand-Authored Files

- Hand-authored theme and styling live in `dashboard/config.toml`, `dashboard/css/input.css`, `dashboard/static/styles/`, and `dashboard/themes/tars-dashboards/`.
- Do not hand-edit generated paths (`dashboard/content/`, `dashboard/public/`, `dashboard/static/build/`, `dashboard/static/pagefind/`, `dashboard/static/.well-known/`). Regenerate via `devenv --no-tui shell -- ask dashboard lint --fix` or `ask dashboard build`.

## Templates & Design System

- Zola 0.23+ uses Tera 2 components. Always pass component arguments with bare braces (`arg={value}`), never quoted (`arg="{value}"`), which renders literal text and fails the `tera-templates` hook.
- Keep design tokens, classes, and component names aligned with `mahdtech.com` (`themes.css`, `foundation.css`, `input.css`).
- Put reusable page markup into `ui.*` components in `dashboard/themes/tars-dashboards/templates/macros/ui.html` instead of duplicating HTML across templates.

## Verification

- Verify dashboard changes with `devenv --no-tui shell -- dashboard-test` or the full `devenv --no-tui test` suite.
