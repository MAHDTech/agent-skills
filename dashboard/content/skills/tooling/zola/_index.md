+++
title = "zola"
description = "Build, serve, or customize an existing Zola site. Use for Tera templates and components, taxonomies, zola.toml options, theme changes, or Zola build and link checks."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Zola static site generator

Take configuration keys, template functions, and Tera syntax from the vendored docs in `resources/auto/`: the Zola docs (`resources/auto/getzola-zola-master-docs-content-documentation-*.md`) and the Tera v1 to v2 migration guide (`resources/auto/keats-tera-master-migration.md`). For the full Tera 2 template language reference, read `https://github.com/Keats/tera/blob/master/docs/content/_index.md` online.

## Version check

Run `zola --version` first. Zola 0.23 moved to Tera 2 and removed shortcodes: content files are now Tera templates, and reusable snippets are Tera components (`{% component name(args) %}...{% endcomponent %}`, called as `{{ <name arg="value" /> }}`). Sites on older versions use shortcodes and Tera 1; read their syntax from the docs of the installed version. The config file is `zola.toml`; Zola falls back to `config.toml` when `zola.toml` is absent.

## Commands

Run from the site root, or pass `--root <path>` before the subcommand (for example, `zola --root <path> build`):

- `zola build` renders the site to `public/`.
- `zola serve` serves with live reload on port 1111.
- `zola check` builds without writing output and fetches every external link found in Markdown content (links in templates are not checked); `--skip-external-links` skips the fetches. Broken internal links (`@/...`) fail any build by default, controlled by `[link_checker] internal_level`.

## Templates

- Define taxonomies in `zola.toml` and read them in templates with `get_taxonomy` or `get_taxonomy_url`.
- `load_data(path=...)` reads TOML, JSON, CSV, BibTeX, YAML, and XML; any other file type loads as plain text. It also accepts a URL or a literal string.
- Exclude Tera templates from HTML formatters such as Prettier: they rewrite `{% %}` blocks and component arguments into invalid Tera.
- Literal `{{` or `{%` in content now fails compilation. Wrap such lines in `{% raw %}...{% endraw %}`, or list content that needs no templating in `skip_content_templating`.

## Completion criteria

- `zola build` (or `zola check`) exits 0 with no warnings introduced by the change.
- New sections and pages appear in the build output, and internal links resolve.

