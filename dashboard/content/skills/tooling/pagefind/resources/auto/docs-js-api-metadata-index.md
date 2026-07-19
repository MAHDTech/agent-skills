+++
title = "docs-js-api-metadata-index"
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

# Getting metadata with the Pagefind JavaScript API


Pagefind's JavaScript API returns the metadata of your pages automatically alongside your search result data.

## Getting metadata from a search result

```js
const pagefind = await import("/pagefind/pagefind.js");
const search = await pagefind.search("static");
+const oneResult = await search.results[0].data();
```

Here, `oneResult` will contain:

```js
{
  /* ... other result keys ... */
  "url": "/url-of-the-page/",
  "excerpt": "A small snippet of the <mark>static</mark> content, from the &lt;body&gt; of the page.",
  "plain_excerpt": "A small snippet of the static content, from the &lt;body&gt; of the page.",
~  "meta": {
~    "title": "The title from the first h1 element on the page",
~    "image": "/weka.png",
~    "my-custom-key": "My custom metadata content",
~  }
}
```
