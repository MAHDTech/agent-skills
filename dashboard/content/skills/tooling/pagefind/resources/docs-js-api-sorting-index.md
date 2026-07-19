+++
title = "docs-js-api-sorting-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "pagefind"
+++

> **Important: Pagefind 1.5.0 introduces the Component UI, which replaces the
> Default UI (pagefind-ui.js / PagefindUI). It includes a search modal, better
> accessibility and customization.** Full component guide:
> https://pagefind.app/llms-component-ui.txt

# Sorting using the Pagefind JavaScript API


Pagefind's JavaScript API supports sorting your content when searching. Doing so will override the default rankings, and will return all matching results sorted by the given attribute.

## Sorting as part of a search

If pages on your site have been tagged with [sort attributes](https://pagefind.app/docs/sorts/), a `sort` object can be provided to Pagefind when searching:

```js
const search = await pagefind.search("static", {
+    sort: {
+        date: "asc"
+    }
});
```

This object should contain one key, matching a `data-pagefind-sort` attribute, and specify either `asc` for ascending or `desc` for descending sort order.
