+++
title = "pagefind"
description = "Use when configuring, indexing, or troubleshooting Pagefind static search for websites. Covers config options, CLI flags, and client integrations."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Pagefind Search Indexer

Guidelines for indexing static websites and configuring search capabilities using Pagefind. Always consult local Pagefind documentation for full options reference.

## CLI & Core Indexing Commands

Run Pagefind post-build to index the output HTML content:

- **Default Indexing:** `npx pagefind --site <build_output_dir>` (indexes all HTML pages in the output directory, e.g., `dashboard/public`).
- **Config file:** Pagefind can load parameters from a `pagefind.yml` or `pagefind.json` file placed in the project root.

## Client Integration Rules

1. **JS/CSS Assets:** Pagefind generates static search assets (compiled index and search UI scripts) under the `_pagefind` directory in the site root (e.g. `_pagefind/pagefind.js`, `_pagefind/pagefind.css`).
2. **Excluding Elements:** To keep irrelevant layout elements (such as footers, navbars, sidebars) from polluting search index scores, add `data-pagefind-ignore` attribute to their HTML elements.
3. **Weighting Content:** To highlight specific regions (e.g., titles, primary paragraphs), use `data-pagefind-weight="<value>"` (value from 1 to 10) on elements.

## Completion Criteria

The indexing task is complete when:

1. `pagefind` completes building the search database successfully without throwing errors.
2. The index folder `_pagefind/` exists in the public output directory containing the generated search chunks and compiled index configs.
3. Relevant search query tests return expected matches.

