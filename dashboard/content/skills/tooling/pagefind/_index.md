+++
title = "pagefind"
description = "Configure, build, or troubleshoot Pagefind search for a static website. Use for indexing options, content exclusions or weighting, generated assets, and client integration."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Pagefind search indexer

Pagefind indexes a built static site and writes a search bundle beside it. Take options and attributes from the vendored docs: start at `resources/auto/llms.txt`, then read the matching page such as `resources/auto/docs-config-options-index.md`, `resources/auto/docs-config-sources-index.md`, `resources/auto/docs-indexing-index.md`, or `resources/auto/docs-weighting-index.md`. Pagefind 1.5 replaced the Default UI with the Component UI, documented in `resources/auto/llms-component-ui.txt`.

## Indexing

Run Pagefind after the site generator has written its HTML:

```bash
npx pagefind --site <build-output-dir>
```

Use the project's existing invocation (a pinned binary, `python3 -m pagefind`, or a package script) when it has one.

- Options can come from CLI flags, `PAGEFIND_*` environment variables, or a `pagefind.toml`, `pagefind.yml`, `pagefind.yaml`, or `pagefind.json` file in the directory where the command runs. Flags override environment variables, which override the file.
- The bundle is written to `<site>/pagefind/` by default (`output_subdir` changes it).

## Controlling the index

- Mark the main content with `data-pagefind-body`. Once any page has it, pages without it are not indexed, so add it to every page layout that should be searchable.
- Exclude navigation, footers, and sidebars inside indexed content with `data-pagefind-ignore`.
- Weight a region with `data-pagefind-weight="<n>"`, any number from 0.0 to 10.0 (default 1.0). The scale is quadratic: 2.0 counts about four times as much as normal text.

## Client integration

The Component UI loads `/pagefind/pagefind-component-ui.js` and `/pagefind/pagefind-component-ui.css`; the JavaScript API imports `/pagefind/pagefind.js`. Check `resources/auto/llms-component-ui.txt` for the component markup.

## Completion criteria

- The Pagefind command exits 0 and reports a non-zero number of indexed pages.
- `<site>/pagefind/` exists in the build output.
- A search for a term known to be on an indexed page returns that page, and a term that appears only in an ignored region does not.

