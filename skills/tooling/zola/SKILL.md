---
name: zola
description: Expert reference and development guidelines for the Zola static site generator. Use when the user asks to modify, build, serve, or customize Zola sites, themes, templates, shortcodes, or Zola configuration files (such as config.toml).
resources:
  - https://www.getzola.org/documentation/
---

# Zola Static Site Generator

Guidelines for managing, compiling, and configuring Zola static sites. Always refer to local Zola documentation for details on configuration schemas, template syntax (Tera), or shortcodes.

## CLI & Core Commands

To work with Zola sites, use the following commands in the workspace root or the site root:

- **Build the site:** `zola build` (or with `--root <path>` if the Zola files live in a subdirectory like `dashboard`).
- **Serve with live reload:** `zola serve` (default port is `1111`).
- **Check links:** `zola check` (validates all internal and external anchors).

## Template & Design Rules

1. **Tera Template Engine:** Zola uses the Tera templating language (similar to Jinja2/Twig). Keep syntax clean.
2. **Context & Taxonomies:** Define taxonomies (like categories or tags) in the main configuration file (`config.toml`). Access taxonomies via `get_taxonomy` or standard templates.
3. **Data Loading:** Load static configuration or structural data from files using `load_data(path="...")` (supported formats: TOML, JSON, CSV).

## Completion Criteria

The Zola task is complete when:

1. All template modifications render correctly without compilation warnings.
2. Any newly introduced structural sections or pages compile cleanly during `zola build`.
3. All internal links resolved by Zola check pass without broken anchors.
