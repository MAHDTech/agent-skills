+++
title = "docs-config-sources-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "pagefind"
+++

{% raw %}
> **Important: Pagefind 1.5.0 introduces the Component UI, which replaces the
> Default UI (pagefind-ui.js / PagefindUI). It includes a search modal, better
> accessibility and customization.** Full component guide:
> https://pagefind.app/llms-component-ui.txt

# Pagefind CLI configuration sources


Pagefind can be configured through CLI flags, environment variables, or configuration files. Values will be merged from all sources, with CLI flags overriding environment variables, and environment variables overriding configuration files.

## Config files

Pagefind will look for a `pagefind.toml`, `pagefind.yml`, `pagefind.yaml`, or `pagefind.json` file in the directory that you have run the command in.

```yaml
# pagefind.yml
site: public
output_subdir: pagefind
```
```bash
npx pagefind
```

## Environment variables

Pagefind will load any values via a `PAGEFIND_*` environment variable.

```bash
export PAGEFIND_OUTPUT_SUBDIR="pagefind"
PAGEFIND_SITE="public" npx pagefind
```

## CLI flags

Pagefind can be passed CLI flags directly.

```bash
npx pagefind --site public --output-subdir pagefind
```
{% endraw %}
