+++
title = "docs-multisite-index"
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

# Searching multiple sites


Pagefind can be configured to search across multiple sites, merging results and filters into a single response. Multisite search configuration happens entirely in the browser, by pointing one Pagefind instance at multiple search bundles.

The following examples reflect Pagefind running on a website at `blog.example.com` that wants to include pages from `docs.example.com` in the search results. _(Note that in a matching setup across domains, you will need to configure CORS headers. See [Cross origin indexes](#cross-origin-indexes) below.)_

## Searching additional sites from the Component UI

When configuring the Component UI, include a `mergeIndex` option with an array of additional indexes to merge into the main index. The URL should be the path to a pagefind bundle folder.

```js
// Running on blog.example.com

const { configureInstance } = window.PagefindComponents;

+configureInstance("default", {
+    mergeIndex: [{
+        bundlePath: "https://docs.example.com/pagefind"
+    }]
+});
```

Pagefind options can be passed to the additional indexes alongside the `bundlePath`:

```js
// Running on blog.example.com

const { configureInstance } = window.PagefindComponents;

configureInstance("default", {
+    // ... options for the blog.example.com index
    mergeIndex: [{
        bundlePath: "https://docs.example.com/pagefind",
+        // ... options for the docs.example.com index
    }]
});
```

## Searching additional sites from the Pagefind JS API

Using an initialized instance of Pagefind, await the `mergeIndex` function to add an additional index. The URL should be the path to a pagefind bundle folder.

```js
// Running on blog.example.com

const pagefind = await import("/pagefind/pagefind.js");
+await pagefind.mergeIndex("https://docs.example.com/pagefind");
```

Pagefind options can be passed to the additional index as a second argument:

```js
// Running on blog.example.com

const pagefind = await import("/pagefind/pagefind.js");
+await pagefind.options({/* ... options for the blog.example.com index */});
await pagefind.mergeIndex(
  "https://docs.example.com/pagefind",
+  {/* ... options for the docs.example.com index */}
);
```

## Changing the weighting of individual indexes

When searching across multiple sites you may want to rank each index higher or lower than the others. This can be achieved by passing an `indexWeight` option for each index:

```js
// Component UI:
const { configureInstance } = window.PagefindComponents;

configureInstance("default", {
+    indexWeight: 2,
    mergeIndex: [{
        bundlePath: "https://docs.example.com/pagefind",
+        indexWeight: 0.5
    }]
});

// JS API:
const pagefind = await import("/pagefind/pagefind.js");
+await pagefind.options({ indexWeight: 2 });
await pagefind.mergeIndex("https://docs.example.com/pagefind", {
+    indexWeight: 0.5
});
```

## Filtering results by index

When searching across multiple sites you may want to filter to each index, without having to tag every page on each site with the filter. This can be achieved with the `mergeFilter` option on each index:

```js
// Component UI:
const { configureInstance } = window.PagefindComponents;

configureInstance("default", {
+    mergeFilter: {
+        resource: "Blog"
+    },
    mergeIndex: [{
        bundlePath: "https://docs.example.com/pagefind",
+        mergeFilter: {
+            resource: "Documentation"
+        }
    }]
});

// JS API:
const pagefind = await import("/pagefind/pagefind.js");
+await pagefind.options({ mergeFilter: { resource: "Blog" } });
await pagefind.mergeIndex("https://docs.example.com/pagefind", {
+    mergeFilter: {
+        resource: "Documentation"
+    }
});
```

## Merging a specific language index

Pagefind will attempt to grab a matching language when merging an index, falling back to the default language for that index. You can change this behavior by passing a `language` option:

```js
// Component UI:
const { configureInstance } = window.PagefindComponents;

configureInstance("default", {
    mergeIndex: [{
        bundlePath: "https://docs.example.com/pagefind",
+        language: "pt-br"
    }]
});

// JS API:
const pagefind = await import("/pagefind/pagefind.js");
await pagefind.mergeIndex("https://docs.example.com/pagefind", {
+    language: "pt-br"
});
```

## Notes

### Cross origin indexes

Due to index merging happening in the browser, your additional search indexes must be configured with [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) headers if they span separate domains. For example, to configure these headers in a CloudCannon `routing.json` file:

```json
{
  "headers": [
    {
      "match": "/pagefind/.*",
      "headers": [
        {
          "name": "Access-Control-Allow-Origin",
          "value": "*"
        }
      ]
    }
  ]
}
```

### Merging multiple languages

Merged indexes will be searched using the WebAssembly module from your main instance. This means that merging an index from another language will use the language support from your main Pagefind instance.
{% endraw %}
